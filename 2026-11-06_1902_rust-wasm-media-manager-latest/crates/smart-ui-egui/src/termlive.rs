//! Renders the real shell (pty + vt100) inside the Terminal app: cursor, selection, scrollback, clipboard, resize.
use crate::app::*;
use crate::appkit::Frame;
use crate::chrome::*;
use crate::overlays::mi;
use crate::paint::*;
use crate::pty::{input_bytes, Live};
use crate::terminal::ansi_rgb;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, FontId, Key, Rect, Sense, Ui};
use smart_core::Location;

impl SmartApp {
    /// Spawns the shell lazily (needs the egui context for repaint wake-ups). Returns true when a live shell owns the session.
    pub(crate) fn term_live_active(&mut self, ctx: &egui::Context, si: usize, fr: &Frame) -> bool {
        let font = self.shell.term.font;
        let charw = (font * 0.6).max(6.0);
        let lh = (font * 1.35).round();
        let cols = (((fr.stage.width() - 40.0) / charw) as u16).max(40);
        let rows = (((fr.stage.height() - 60.0) / lh) as u16).max(10);
        let s = &mut self.shell.term.sessions[si];
        if s.smart {
            return false;
        }
        if s.live.is_none() && !s.live_tried {
            s.live_tried = true;
            let cwd = if s.cwd.fs == "file" { Some(std::path::PathBuf::from(&s.cwd.path)) } else { None };
            match Live::spawn(ctx.clone(), cwd.as_deref(), cols, rows) {
                Ok(l) => s.live = Some(l),
                Err(e) => {
                    s.live_err = Some(e);
                    s.smart = true;
                }
            }
        }
        s.live.is_some()
    }

    pub(crate) fn draw_term_live(&mut self, ui: &mut Ui, fr: &Frame, si: usize) {
        let th = self.th.clone();
        let ctx = ui.ctx().clone();
        let p = ui.painter().clone();
        let font_px = self.shell.term.font;
        let mono = FontId::monospace(font_px);
        let charw = self.tw(&p, "M", &mono);
        let lh = (font_px * 1.35).round();
        let rect = fr.stage;
        let hint_h = 28.0;
        let surf = Rect::from_min_max(pos2(rect.left() + 6.0, rect.top() + 2.0), pos2(rect.right() - 4.0, rect.bottom() - hint_h - 2.0));
        well(&p, &th, surf, 16.0);
        let inner = surf.shrink2(vec2(14.0, 10.0));
        let (cols, rows) = (((inner.width() / charw) as u16).max(20), ((inner.height() / lh) as u16).max(4));
        let kb = fr.active && self.shell.settings.is_none() && !self.palette.open && self.shell.appmenu.is_none();
        let resp = self.hit(ui, inner, ("tm-live", si), Sense::click_and_drag());
        let time = self.time;
        let cell_at = |pos: egui::Pos2| -> (u16, u16) { ((((pos.y - inner.top()) / lh).floor().max(0.0) as u16).min(rows - 1), (((pos.x - inner.left()) / charw).floor().max(0.0) as u16).min(cols - 1)) };
        let events: Vec<egui::Event> = if kb { ctx.input(|i| i.events.clone()) } else { vec![] };
        let (wheel, hovering) = (ctx.input(|i| i.smooth_scroll_delta.y), resp.hovered());
        let mut copy_text: Option<String> = None;
        let mut respawn = false;
        let mut cwd_update: Option<std::path::PathBuf> = None;
        {
            let s = &mut self.shell.term.sessions[si];
            let Some(live) = s.live.as_mut() else { return };
            live.resize(cols, rows);
            let alive = live.is_alive();
            let (app_cursor, bracketed, alt) = {
                let g = live.parser.lock().unwrap_or_else(|e| e.into_inner());
                let sc = g.screen();
                (sc.application_cursor(), sc.bracketed_paste(), sc.alternate_screen())
            };
            if alive {
                // keys the app itself owns: new/close session, palette, zoom
                let evs: Vec<egui::Event> = events
                    .into_iter()
                    .filter(|e| match e {
                        egui::Event::Key { key, modifiers, pressed: true, .. } => !(modifiers.ctrl && !modifiers.shift && matches!(key, Key::T | Key::W | Key::K | Key::Comma | Key::Equals | Key::Plus | Key::Minus)),
                        _ => true,
                    })
                    .collect();
                for e in &evs {
                    if let egui::Event::Key { key: Key::C, pressed: true, modifiers, .. } = e {
                        if modifiers.ctrl && modifiers.shift {
                            if let Some((a, b)) = live.sel {
                                let g = live.parser.lock().unwrap_or_else(|e| e.into_inner());
                                let (a, b) = if a <= b { (a, b) } else { (b, a) };
                                copy_text = Some(g.screen().contents_between(a.0, a.1, b.0, b.1 + 1));
                            }
                        }
                    }
                }
                let bytes = input_bytes(&evs, app_cursor, bracketed);
                if !bytes.is_empty() {
                    live.write(&bytes);
                    live.scroll = 0;
                    live.sel = None;
                }
                // wheel: scrollback on the primary screen, arrows on the alternate screen (less, vim, man)
                if hovering && wheel != 0.0 {
                    if alt {
                        let n = ((wheel / lh).abs().ceil() as usize).clamp(1, 6);
                        let seq: &[u8] = if wheel > 0.0 { if app_cursor { b"\x1bOA" } else { b"\x1b[A" } } else if app_cursor { b"\x1bOB" } else { b"\x1b[B" };
                        live.write(&seq.repeat(n));
                    } else {
                        let d = (wheel / lh * 1.6).round() as i64;
                        live.scroll = (live.scroll as i64 + d).max(0) as usize;
                    }
                }
                if kb && ctx.input(|i| i.key_pressed(Key::PageUp) && i.modifiers.shift) {
                    live.scroll += rows as usize / 2;
                }
                if kb && ctx.input(|i| i.key_pressed(Key::PageDown) && i.modifiers.shift) {
                    live.scroll = live.scroll.saturating_sub(rows as usize / 2);
                }
                // selection
                if let Some(pp) = resp.interact_pointer_pos() {
                    if resp.drag_started() {
                        let c = cell_at(pp);
                        live.sel = Some((c, c));
                    } else if resp.dragged() {
                        if let Some((a, _)) = live.sel {
                            live.sel = Some((a, cell_at(pp)));
                        }
                    }
                }
                if resp.clicked() && !resp.dragged() {
                    live.sel = None;
                }
                if resp.double_clicked() {
                    if let Some(pp) = resp.interact_pointer_pos() {
                        let (r, c0) = cell_at(pp);
                        let g = live.parser.lock().unwrap_or_else(|e| e.into_inner());
                        let line: Vec<char> = g.screen().contents_between(r, 0, r, cols).chars().collect();
                        let word = |ch: char| !(ch.is_whitespace() || "()[]{}<>\"'`,;|&".contains(ch));
                        let (mut a, mut b) = (c0 as usize, c0 as usize);
                        if a < line.len() && word(line[a]) {
                            while a > 0 && word(line[a - 1]) {
                                a -= 1;
                            }
                            while b + 1 < line.len() && word(line[b + 1]) {
                                b += 1;
                            }
                            live.sel = Some(((r, a as u16), (r, b as u16)));
                        }
                    }
                }
                if time - (self.shell.term.cursor as f64 * 0.0) > 0.0 {
                    // keep the working directory chip in step with `cd`
                    if let Some(d) = live.cwd() {
                        cwd_update = Some(d);
                    }
                }
            } else if kb && events.iter().any(|e| matches!(e, egui::Event::Key { key: Key::Enter, pressed: true, .. })) {
                respawn = true;
            }
        }
        if let Some(d) = cwd_update {
            let s = &mut self.shell.term.sessions[si];
            if s.cwd.fs != "file" || s.cwd.path != d.to_string_lossy() {
                s.cwd = Location::new("file", d.to_string_lossy().to_string());
            }
        }
        if respawn {
            let s = &mut self.shell.term.sessions[si];
            s.live = None;
            s.live_tried = false;
            return;
        }
        if let Some(t) = copy_text {
            if !t.trim().is_empty() {
                self.acts.push(Act::CopyText(t));
            }
        }
        // draw the screen
        let active = fr.active;
        let blink_on = (time * 1.8).fract() < 0.6 || !self.shell.term.blink;
        let s = &mut self.shell.term.sessions[si];
        let Some(live) = s.live.as_mut() else { return };
        let sel = live.sel.map(|(a, b)| if a <= b { (a, b) } else { (b, a) });
        let alive = live.is_alive();
        let mut scrolled = 0usize;
        {
            let mut g = live.parser.lock().unwrap_or_else(|e| e.into_inner());
            g.set_scrollback(live.scroll);
            scrolled = scrolled.max(g.screen().scrollback());
            live.scroll = scrolled;
            let scr = g.screen();
            let pp = p.with_clip_rect(inner.expand(4.0));
            let (rws, cls) = scr.size();
            for r in 0..rws.min(rows) {
                let mut c = 0u16;
                while c < cls.min(cols) {
                    let Some(cell) = scr.cell(r, c) else { break };
                    let key = (cell.fgcolor(), cell.bgcolor(), cell.bold(), cell.inverse(), cell.underline(), cell.italic());
                    let mut text = String::new();
                    let mut e = c;
                    while e < cls.min(cols) {
                        let Some(x) = scr.cell(r, e) else { break };
                        if (x.fgcolor(), x.bgcolor(), x.bold(), x.inverse(), x.underline(), x.italic()) != key {
                            break;
                        }
                        if !x.is_wide_continuation() {
                            text.push_str(&if x.has_contents() { x.contents() } else { " ".into() });
                        }
                        e += 1;
                    }
                    let (mut fg, mut bg) = (ansi_rgb(key.0, key.2), ansi_rgb(key.1, false));
                    if key.3 {
                        let surface = if th.dark { hex("#10151f") } else { Color32::WHITE };
                        let (f2, b2) = (bg.unwrap_or(surface), fg.unwrap_or(th.ink1));
                        fg = Some(f2);
                        bg = Some(b2);
                    }
                    let x0 = inner.left() + c as f32 * charw;
                    let y0 = inner.top() + r as f32 * lh;
                    if let Some(b) = bg {
                        pp.rect_filled(Rect::from_min_size(pos2(x0, y0), vec2((e - c) as f32 * charw, lh)), cr(1.0), b);
                    }
                    if !text.trim().is_empty() {
                        let col = fg.unwrap_or(th.ink1);
                        pp.text(pos2(x0, y0 + lh / 2.0), Align2::LEFT_CENTER, &text, mono.clone(), col);
                        if key.4 {
                            pp.line_segment([pos2(x0, y0 + lh - 2.0), pos2(x0 + (e - c) as f32 * charw, y0 + lh - 2.0)], egui::Stroke::new(1.0, col));
                        }
                    }
                    c = e.max(c + 1);
                }
                // selection highlight
                if let Some((a, b)) = sel {
                    if r >= a.0 && r <= b.0 {
                        let c0 = if r == a.0 { a.1 } else { 0 };
                        let c1 = if r == b.0 { b.1 } else { cols - 1 };
                        pp.rect_filled(Rect::from_min_max(pos2(inner.left() + c0 as f32 * charw, inner.top() + r as f32 * lh), pos2(inner.left() + (c1 + 1) as f32 * charw, inner.top() + (r + 1) as f32 * lh)), cr(2.0), alpha(th.accent, 0.35));
                    }
                }
            }
            if live.scroll == 0 && !scr.hide_cursor() && alive {
                let (cr_, cc) = scr.cursor_position();
                let r = Rect::from_min_size(pos2(inner.left() + cc as f32 * charw, inner.top() + cr_ as f32 * lh), vec2(charw, lh));
                if active && blink_on {
                    pp.rect_filled(r, cr(2.0), alpha(th.accent, 0.62));
                } else if !active {
                    pp.rect_stroke(r.shrink(0.5), cr(2.0), egui::Stroke::new(1.2, alpha(th.accent, 0.7)), egui::StrokeKind::Inside);
                }
            }
            g.set_scrollback(0);
        }
        if live.scroll > 0 {
            let b = Rect::from_min_size(pos2(inner.right() - 150.0, inner.top() + 4.0), vec2(140.0, 24.0));
            grad_fill(&p, b, 12.0, alpha(th.accent, 0.9), alpha(th.accent_press, 0.9));
            p.text(b.center(), Align2::CENTER_CENTER, format!("\u{2191} {} lines back", live.scroll), f_semi(10.5), th.on_accent());
        }
        if !alive {
            let b = Rect::from_center_size(inner.center(), vec2(360.0, 46.0));
            elevate(&p, &th, b, 16.0, 3);
            grad_fill(&p, b, 16.0, th.panel_top, th.panel_bottom);
            p.text(b.center(), Align2::CENTER_CENTER, "The shell exited. Press Enter to start a new one.", f_semi(T_BODY), th.ink1);
        }
        // hint bar
        let hb = Rect::from_min_max(pos2(rect.left() + 18.0, rect.bottom() - hint_h), pos2(rect.right() - 18.0, rect.bottom() - 4.0));
        let shell = live.shell.rsplit('/').next().unwrap_or("sh").to_string();
        let msg = format!("{shell}  \u{b7}  real shell on a pty  \u{b7}  Ctrl+Shift+C copy  \u{b7}  Ctrl+Shift+V paste  \u{b7}  wheel or Shift+PgUp scrollback  \u{b7}  {cols}\u{d7}{rows}");
        p.text(pos2(hb.left(), hb.center().y), Align2::LEFT_CENTER, msg, f_medium(10.5), th.ink4);
        if resp.secondary_clicked() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                self.open_app_menu(
                    pos,
                    vec![
                        mi("copy", "Copy", Act::App("term.live_copy".into())).k("Ctrl+Shift+C"),
                        mi("clipboard", "Paste", Act::App("term.live_paste".into())).k("Ctrl+Shift+V"),
                        mi("trash-bin-trash", "Clear", Act::App("term.live_clear".into())).k("Ctrl+L").gap(),
                        mi("stars-minimalistic", "Switch to Smart blocks", Act::App("term.mode".into())),
                        mi("settings-minimalistic", "Terminal settings\u{2026}", Act::App("settings".into())).gap(),
                    ],
                );
            }
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }

    pub(crate) fn term_live_cmd(&mut self, name: &str) {
        let si = self.shell.term.active.min(self.shell.term.sessions.len().saturating_sub(1));
        let Some(s) = self.shell.term.sessions.get_mut(si) else { return };
        match name {
            "live_copy" => {
                if let Some(live) = s.live.as_mut() {
                    if let Some((a, b)) = live.sel {
                        let (a, b) = if a <= b { (a, b) } else { (b, a) };
                        let t = live.parser.lock().unwrap_or_else(|e| e.into_inner()).screen().contents_between(a.0, a.1, b.0, b.1 + 1);
                        self.acts.push(Act::CopyText(t));
                    }
                }
            }
            "live_paste" => self.acts.push(Act::App("term.paste_request".into())),
            "live_clear" => {
                if let Some(live) = s.live.as_mut() {
                    live.write(b"\x0c");
                }
            }
            "mode" => {
                s.smart = !s.smart;
                if !s.smart && s.live.is_none() {
                    s.live_tried = false;
                }
            }
            _ => {}
        }
    }
}
