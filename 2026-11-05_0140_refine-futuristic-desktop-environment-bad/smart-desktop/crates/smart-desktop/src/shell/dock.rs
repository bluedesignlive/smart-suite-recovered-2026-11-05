//! The dock: spring magnification, live window previews on hover, an AI orb, and the Void (trash) —
//! a glass vortex that fills with embers as you throw things in, instead of a copy of a bin.
use super::*;
use crate::widgets::arc_points;
use egui::{Painter, Shape, Stroke};

#[derive(Clone, Copy, PartialEq)]
enum Di {
    App(AppId),
    Sep,
    Ai,
    Settings,
    Trash,
}

impl SmartApp {
    fn draw_void(&self, p: &Painter, r: Rect, hover: bool, over: bool, n: usize, flash: f32) {
        let th = &self.th;
        let t = self.time as f32;
        let rad = r.width() * 0.30;
        shadow(p, r, rad, 5.0, 12.0, -3.0, alpha(th.shadow, if th.dark { 0.55 } else { 0.30 }));
        let (g0, g1) = if th.dark { (hex("#27324f"), hex("#070a12")) } else { (hex("#e6ecf8"), hex("#8794b4")) };
        grad_fill(p, r, rad, g0, g1);
        rim(p, r, rad, 1.4, Color32::from_white_alpha(if th.dark { 110 } else { 190 }), true, r.width() * 0.7);
        rim(p, r, rad, 1.0, Color32::from_black_alpha(70), false, 8.0);
        let ctr = r.center() + vec2(0.0, 1.0);
        let rr = r.width() * 0.31;
        let lit = if over { 1.0 } else if hover { 0.7 } else { 0.45 };
        let hue = if n > 0 || over { hex("#ff9a52") } else { hex("#6ee7ff") };
        for k in 0..3 {
            let sp = t * (1.2 + k as f32 * 0.55) * if over { 2.6 } else { 1.0 } + k as f32 * 2.1;
            let pts = arc_points(ctr, rr * (0.50 + 0.24 * k as f32), sp, sp + 2.3 - k as f32 * 0.45);
            p.add(Shape::line(pts, Stroke::new(1.5 + k as f32 * 0.3, alpha(hue, (0.35 + 0.4 * lit) * (1.0 - k as f32 * 0.18)))));
        }
        let glow = (if over { 1.0 } else if n > 0 { 0.6 } else { 0.25 }) + flash;
        p.circle_filled(ctr, rr * (0.46 + 0.1 * flash), alpha(hue, 0.18 * glow));
        p.circle_filled(ctr, rr * 0.24, alpha(hue, 0.65 * glow.min(1.0)));
        p.circle_filled(ctr - vec2(rr * 0.07, rr * 0.08), rr * 0.07, Color32::from_white_alpha((160.0 * glow.min(1.0)) as u8));
        // embers rising for every few items thrown in
        for i in 0..n.min(6) {
            let ph = (t * 0.5 + i as f32 * 0.37).fract();
            let x = ctr.x + (i as f32 * 2.3 + t * 0.4).sin() * rr * 0.55;
            p.circle_filled(pos2(x, ctr.y + rr * 0.6 - ph * rr * 1.5), 1.6 * (1.0 - ph), alpha(hue, 0.9 * (1.0 - ph)));
        }
        if n > 0 {
            let b = Rect::from_center_size(pos2(r.right() - 4.0, r.top() + 5.0), vec2(if n > 9 { 20.0 } else { 16.0 }, 16.0));
            grad_fill(p, b, 8.0, hex("#ff9a52"), hex("#e0572a"));
            rim(p, b, 8.0, 1.0, Color32::from_white_alpha(140), true, 8.0);
            p.text(b.center(), Align2::CENTER_CENTER, n.to_string(), f_mono_b(9.5), Color32::WHITE);
        }
    }

    fn mini_window(&self, p: &Painter, id: AppId, r: Rect) {
        let th = &self.th;
        let t = self.time as f32;
        shadow(p, r, 6.0, 3.0, 8.0, -2.0, alpha(th.shadow, 0.35));
        let dark_app = id == AppId::Terminal || id == AppId::Capture;
        let (a, b) = if dark_app { (hex("#161d2e"), hex("#0a0e18")) } else { (th.card_top, th.card_bottom) };
        let (a, b) = if dark_app { (a, b) } else { (mix(a, Color32::WHITE, 0.5), mix(b, Color32::WHITE, 0.3)) };
        grad_fill(p, r, 6.0, a, b);
        let tb = Rect::from_min_size(r.min, vec2(r.width(), 9.0));
        p.rect_filled(tb, cr(6.0), alpha(th.ink3, 0.18));
        for (i, col) in [hex("#ff7c72"), hex("#ffd264"), hex("#6fdc8c")].iter().enumerate() {
            p.circle_filled(pos2(r.left() + 7.0 + i as f32 * 6.0, r.top() + 4.5), 1.8, *col);
        }
        let body = Rect::from_min_max(pos2(r.left() + 4.0, r.top() + 13.0), pos2(r.right() - 4.0, r.bottom() - 4.0));
        let side = Rect::from_min_size(body.min, vec2(body.width() * 0.22, body.height()));
        if id != AppId::Terminal {
            p.rect_filled(side, cr(3.0), alpha(th.accent, 0.12));
            for i in 0..4 {
                p.rect_filled(Rect::from_min_size(side.min + vec2(3.0, 4.0 + i as f32 * 7.0), vec2(side.width() - 6.0, 3.5)), cr(1.5), alpha(th.ink3, 0.3));
            }
        }
        let main = Rect::from_min_max(pos2(side.right() + 4.0, body.top()), body.max);
        match id {
            AppId::Terminal => {
                let lines: Vec<String> = self.shell.term.history.iter().rev().take(5).cloned().collect();
                for (i, l) in lines.iter().rev().enumerate() {
                    p.text(pos2(body.left() + 2.0, body.top() + 6.0 + i as f32 * 10.0), Align2::LEFT_CENTER, format!("\u{276f} {}", l), f_mono(7.5), hex("#8fe0b8"));
                }
                let y = body.top() + 6.0 + lines.len() as f32 * 10.0;
                if (t * 2.0) as i32 % 2 == 0 {
                    p.rect_filled(Rect::from_min_size(pos2(body.left() + 2.0, y - 4.0), vec2(5.0, 8.0)), cr(1.0), hex("#6fd9a8"));
                }
            }
            AppId::Photos => {
                let cols = [hex("#ff9fb8"), hex("#6e9bff"), hex("#59c79a"), hex("#e5b05a"), hex("#8f7cf5"), hex("#ff8a6b")];
                for i in 0..6 {
                    let (cx, cy) = (i % 3, i / 3);
                    let w = (main.width() - 8.0) / 3.0;
                    let h = (main.height() - 4.0) / 2.0;
                    let c = Rect::from_min_size(main.min + vec2(cx as f32 * (w + 4.0), cy as f32 * (h + 4.0)), vec2(w, h));
                    grad_fill(p, c, 3.0, cols[i], mix(cols[i], Color32::BLACK, 0.35));
                }
            }
            AppId::Capture => {
                let c = main.center();
                p.circle_filled(c, 9.0, alpha(hex("#ff5a5a"), 0.25 + 0.2 * (t * 3.0).sin().abs()));
                p.circle_filled(c, 5.0, hex("#ff5a5a"));
            }
            _ => {
                for i in 0..5 {
                    let w = main.width() * (0.9 - 0.12 * (i % 3) as f32);
                    p.rect_filled(Rect::from_min_size(main.min + vec2(0.0, i as f32 * 9.0), vec2(w, 5.0)), cr(2.0), alpha(th.ink3, 0.28));
                }
            }
        }
        let sx = r.left() + ((t * 0.5).fract() * 1.6 - 0.3) * r.width();
        let sheen = Rect::from_min_max(pos2(sx.max(r.left()), r.top()), pos2((sx + 22.0).min(r.right()), r.bottom()));
        if sheen.width() > 1.0 {
            grad_fill(p, sheen, 0.0, Color32::from_white_alpha(0), Color32::from_white_alpha(22));
        }
        rim(p, r, 6.0, 1.0, Color32::from_white_alpha(70), true, 12.0);
    }

    /// Hover preview card. Returns true when the pointer is on it (so it stays open while you move onto it).
    fn dock_preview(&mut self, ui: &mut Ui, id: AppId, cx: f32, bottom: f32, screen: Rect) -> bool {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let a = spring_to(&ctx, ("dprev", id), 1.0, 300.0, 26.0).clamp(0.0, 1.2);
        let (cw, ch) = (252.0, 168.0);
        let card = Rect::from_center_size(pos2(cx.clamp(screen.left() + cw / 2.0 + 10.0, screen.right() - cw / 2.0 - 10.0), bottom - 14.0 - ch / 2.0 + (1.0 - a.min(1.0)) * 14.0), vec2(cw, ch));
        let p = ui.painter().clone();
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((a * 2.5).clamp(0.0, 1.0));
        let p2 = cui.painter().clone();
        let _ = p;
        elevate(&p2, &th, card, 20.0, 5);
        grad_fill(&p2, card, 20.0, th.panel_top, th.panel_bottom);
        rim(&p2, card, 20.0, 1.4, th.rim, true, 40.0);
        let (c0, c1) = match id {
            AppId::Photos => (hex("#ff9fb8"), hex("#ff7a59")),
            AppId::Terminal => (hex("#2a3550"), hex("#0d1220")),
            AppId::Capture => (hex("#ff7a7a"), hex("#d83b4b")),
            AppId::Settings => (hex("#9aa6bd"), hex("#6b778f")),
            AppId::Studio => (hex("#8f7cf5"), hex("#5b4fd6")),
            AppId::Files => th.accent_grad(),
        };
        let ic = Rect::from_min_size(card.min + vec2(12.0, 10.0), vec2(20.0, 20.0));
        grad_fill(&p2, ic, 6.0, c0, c1);
        self.ico(&p2, id.icon(), Ic::Duo, ic.center(), 12.0, Color32::WHITE);
        p2.text(pos2(ic.right() + 8.0, ic.center().y), Align2::LEFT_CENTER, id.name(), f_bold(12.0), th.ink1);
        let min = self.app_win(id).map(|w| w.min).unwrap_or(self.win_min);
        let wr = self.win_rect_of(id, self.shell.screen).or_else(|| self.app_win(id).map(|w| w.rect));
        let chip = if min { "Minimized".to_string() } else if self.focused_app() == id { "Active".into() } else { "Open".into() };
        let cc = if min { th.warn } else { th.good };
        let chw = self.tw(&p2, &chip, &f_semi(9.0)) + 14.0;
        let chr = Rect::from_center_size(pos2(card.right() - 12.0 - chw / 2.0, ic.center().y), vec2(chw, 16.0));
        grad_fill(&p2, chr, 8.0, alpha(cc, 0.22), alpha(cc, 0.10));
        p2.text(chr.center(), Align2::CENTER_CENTER, chip, f_semi(9.0), cc);
        let area = Rect::from_min_max(card.min + vec2(12.0, 38.0), card.max - vec2(12.0, 12.0));
        let asp = wr.map(|r| r.width() / r.height().max(1.0)).unwrap_or(1.4).clamp(0.8, 2.2);
        let (mw, mh) = if area.width() / area.height() > asp { (area.height() * asp, area.height()) } else { (area.width(), area.width() / asp) };
        let mini = Rect::from_center_size(area.center(), vec2(mw, mh));
        self.mini_window(&p2, id, mini);
        if let Some(r) = wr {
            p2.text(pos2(card.right() - 14.0, card.bottom() - 4.0), Align2::RIGHT_BOTTOM, format!("{}\u{d7}{}", r.width() as i32, r.height() as i32), f_mono(8.0), th.ink4);
        }
        let resp = cui.interact(card, Id::new(("dprevc", id)), Sense::click());
        if resp.clicked() {
            self.open_app(id);
        }
        resp.hovered()
    }

    pub(crate) fn draw_dock(&mut self, ui: &mut Ui, screen: Rect, ai_t: f32) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let p = ui.painter().clone();
        let items = [Di::App(AppId::Files), Di::App(AppId::Photos), Di::App(AppId::Terminal), Di::App(AppId::Capture), Di::Sep, Di::Ai, Di::Settings, Di::Sep, Di::Trash];
        let base = self.shell.dock.size;
        let magk = 0.62 * self.shell.dock.mag;
        let (gap, pad, sepw) = (8.0f32, 12.0f32, 10.0f32);
        let n = items.len();
        let w0: f32 = items.iter().map(|i| if matches!(i, Di::Sep) { sepw } else { base }).sum::<f32>() + gap * (n as f32 - 1.0) + pad * 2.0;
        let left0 = screen.center().x - w0 / 2.0;
        let ptr = ctx.input(|i| i.pointer.hover_pos());
        let pill_h = base + 16.0;
        let want_hide = self.shell.dock.autohide && !ptr.is_some_and(|pp| pp.y > screen.bottom() - 74.0) && self.shell.drag_win.is_none() && self.shell.dock_prev.is_none();
        let hide = spring_to(&ctx, "dock-hide", if want_hide { 1.0 } else { 0.0 }, 200.0, 26.0).clamp(0.0, 1.0);
        let slide = hide * (pill_h + 30.0) + ai_t * (pill_h + 50.0);
        let zone = Rect::from_min_max(pos2(left0 - 30.0, screen.bottom() - DOCK_H - 20.0), pos2(left0 + w0 + 30.0, screen.bottom()));
        let in_zone = ptr.is_some_and(|pp| zone.contains(pp)) && slide < 4.0;
        let mut cx0 = left0 + pad;
        let mut scales = vec![];
        for (i, it) in items.iter().enumerate() {
            let w = if matches!(it, Di::Sep) { sepw } else { base };
            let c0 = cx0 + w / 2.0;
            let tgt = match (it, ptr) {
                (Di::Sep, _) => 1.0,
                (_, Some(pp)) if in_zone => 1.0 + magk * (-((pp.x - c0) / (base * 1.35)).powi(2)).exp(),
                _ => 1.0,
            };
            scales.push(spring_to(&ctx, ("dock-s", i), tgt, 320.0, 25.0));
            cx0 += w + gap;
        }
        let total: f32 = items.iter().zip(&scales).map(|(it, s)| if matches!(it, Di::Sep) { sepw } else { base * s }).sum::<f32>() + gap * (n as f32 - 1.0) + pad * 2.0;
        let left = screen.center().x - total / 2.0;
        let pill = Rect::from_min_size(pos2(left, screen.bottom() - 14.0 - pill_h + slide), vec2(total, pill_h));
        elevate(&p, &th, pill, 26.0, 4);
        let (g0, g1) = if th.dark { (c(26, 34, 52, 0.72), c(12, 18, 30, 0.80)) } else { (c(255, 255, 255, 0.62), c(236, 243, 255, 0.50)) };
        grad_fill(&p, pill, 26.0, g0, g1);
        rim(&p, pill, 26.0, 1.4, if th.dark { c(255, 255, 255, 0.22) } else { c(255, 255, 255, 0.95) }, true, 60.0);
        rim(&p, pill, 26.0, 1.0, if th.dark { c(255, 255, 255, 0.07) } else { c(255, 255, 255, 0.6) }, false, 14.0);
        p.rect_stroke(pill.shrink(0.5), cr(26.0), egui::Stroke::new(1.0, if th.dark { c(255, 255, 255, 0.08) } else { c(16, 26, 44, 0.07) }), egui::StrokeKind::Inside);
        let pill_resp = ui.interact(pill, Id::new("dock-pill"), Sense::click());
        if pill_resp.secondary_clicked() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                let d = &self.shell.dock;
                let items = vec![
                    mi("widget-4", "Small dock", Act::App("dock.size:38".into())).on((d.size - 38.0).abs() < 1.0),
                    mi("widget-4", "Medium dock", Act::App("dock.size:46".into())).on((d.size - 46.0).abs() < 1.0),
                    mi("widget-4", "Large dock", Act::App("dock.size:58".into())).on((d.size - 58.0).abs() < 1.0),
                    mi("magnifer-zoom-in", "Magnification", Act::App("dock.mag".into())).on(d.mag > 0.1).gap(),
                    mi("eye", "Auto-hide", Act::App("dock.hide".into())).on(d.autohide),
                    mi("settings-minimalistic", "Dock settings\u{2026}", Act::App("dock.settings".into())).gap(),
                ];
                self.open_app_menu(pos2(pos.x, pos.y - Self::menu_height(&items) - 10.0), items);
            }
        }

        let mut x = left + pad;
        let mut hovered_label: Option<(String, f32, f32)> = None;
        let mut clicked: Option<Di> = None;
        let mut preview: Option<(AppId, f32)> = None;
        let dragging_files = self.drag.is_some();
        for (i, it) in items.iter().enumerate() {
            if let Di::Sep = it {
                p.line_segment([pos2(x + 5.0, pill.top() + 14.0), pos2(x + 5.0, pill.bottom() - 14.0)], egui::Stroke::new(1.0, alpha(th.ink4, 0.4)));
                x += sepw + gap;
                continue;
            }
            let sz = base * scales[i];
            let (name, icon, c0, c1, running, id): (&str, &str, Color32, Color32, bool, Option<AppId>) = match it {
                Di::App(AppId::Files) => ("Files", "folder", th.accent_grad().0, th.accent_grad().1, !self.win_closed, Some(AppId::Files)),
                Di::App(AppId::Photos) => ("Photos", "gallery", hex("#ff9fb8"), hex("#ff7a59"), self.shell.photos_w.open, Some(AppId::Photos)),
                Di::App(AppId::Terminal) => ("Terminal", "terminal", hex("#2a3550"), hex("#0d1220"), self.shell.term_w.open, Some(AppId::Terminal)),
                Di::App(AppId::Capture) => ("Capture", "camera", hex("#ff7a7a"), hex("#d83b4b"), self.shell.capture_w.open, Some(AppId::Capture)),
                Di::App(a) => (a.name(), a.icon(), hex("#8f7cf5"), hex("#5b4fd6"), self.is_open(*a), Some(*a)),
                Di::Ai => ("Ask your assistant", "stars-minimalistic", hex("#a58bff"), hex("#e0569c"), self.shell.ai.open || self.shell.ai.mode, None),
                Di::Settings => ("Settings", "settings-minimalistic", hex("#9aa6bd"), hex("#6b778f"), self.shell.settings_w.open, None),
                _ => ("The Void", "trash-bin-2", hex("#27324f"), hex("#070a12"), false, None),
            };
            let bounce = match id.and_then(|a| self.shell.launched.get(&a)) {
                Some(t0) if self.time - t0 < 1.4 => {
                    ctx.request_repaint();
                    let a = (self.time - t0) as f32;
                    -26.0 * (-a * 3.4).exp() * (a * 12.0).sin().abs()
                }
                _ => 0.0,
            };
            let r = Rect::from_min_size(pos2(x, pill.bottom() - 8.0 - sz + bounce), vec2(sz, sz));
            let resp = ui.interact(Rect::from_min_size(pos2(x - gap / 2.0, pill.top() - 24.0), vec2(sz + gap, pill.height() + 24.0)), Id::new(("dock-i", i)), Sense::click());
            let down = resp.is_pointer_button_down_on();
            let rr = if down { r.shrink(sz * 0.04) } else { r };
            let rad = sz * 0.27;
            match it {
                Di::Trash => {
                    let over = dragging_files && resp.hovered();
                    let flash = (1.0 - ((self.time - self.shell.trash_flash) as f32 / 0.7)).clamp(0.0, 1.0);
                    self.draw_void(&p, rr, resp.hovered(), over, self.shell.trash_n, flash);
                    DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Trash, r.expand2(vec2(gap / 2.0, 10.0)))));
                    if resp.secondary_clicked() {
                        if let Some(pos) = ctx.pointer_latest_pos() {
                            let items = vec![mi("trash-bin-trash", "Empty the Void\u{2026}", Act::App("dock.empty".into())).off(self.shell.trash_n == 0), mi("info-circle", format!("{} item{} held", self.shell.trash_n, if self.shell.trash_n == 1 { "" } else { "s" }), Act::App("dock.void".into()))];
                            self.open_app_menu(pos2(pos.x, pos.y - Self::menu_height(&items) - 10.0), items);
                        }
                    }
                }
                _ => {
                    shadow(&p, rr, rad, 5.0, 12.0, -3.0, alpha(th.shadow, if th.dark { 0.55 } else { 0.30 }));
                    grad_fill(&p, rr, rad, c0, c1);
                    rim(&p, rr, rad, 1.4, Color32::from_white_alpha(if th.dark { 120 } else { 170 }), true, sz * 0.7);
                    rim(&p, rr, rad, 1.0, Color32::from_black_alpha(60), false, 8.0);
                    let gl = Rect::from_min_max(rr.min + vec2(sz * 0.06, sz * 0.05), pos2(rr.max.x - sz * 0.06, rr.top() + sz * 0.46));
                    grad_fill(&p, gl, rad * 0.8, Color32::from_white_alpha(46), Color32::from_white_alpha(0));
                    if id == Some(AppId::Terminal) {
                        p.text(rr.center() + vec2(0.0, -sz * 0.02), Align2::CENTER_CENTER, ">_", f_mono_b(sz * 0.34), hex("#7dffb2"));
                    } else {
                        if matches!(it, Di::Ai) && (self.shell.ai.busy || self.shell.ai.speaking) {
                            let pulse = 0.5 + 0.5 * (self.time * 5.0).sin() as f32;
                            p.circle_stroke(rr.center(), sz * (0.55 + 0.1 * pulse), egui::Stroke::new(2.0, alpha(Color32::WHITE, 0.6 * (1.0 - pulse))));
                        }
                        self.ico(&p, icon, if resp.hovered() { Ic::Filled } else { Ic::Duo }, rr.center(), sz * 0.52, Color32::WHITE);
                    }
                }
            }
            if running {
                p.circle_filled(pos2(r.center().x, pill.bottom() - 4.0), 2.2, alpha(th.ink2, 0.9));
            }
            if resp.hovered() && slide < 4.0 {
                hovered_label = Some((name.to_string(), r.center().x, r.top()));
                if let (Some(aid), true) = (id, running) {
                    if !dragging_files {
                        preview = Some((aid, r.center().x));
                    }
                }
            }
            if resp.clicked() {
                clicked = Some(*it);
            }
            x += sz + gap;
        }
        // preview card, kept alive for a short grace period so the pointer can travel onto it
        let now = self.time;
        if let Some((aid, cx)) = preview {
            self.shell.dock_prev = Some((aid, now));
            ctx.data_mut(|d| d.insert_temp(Id::new("dprev-x"), cx));
        }
        if let Some((aid, t0)) = self.shell.dock_prev {
            if preview.is_none() && now - t0 > 0.22 {
                self.shell.dock_prev = None;
                ctx.data_mut(|d| d.remove::<(f32, f32)>(Id::new(("spring", ("dprev", aid)))));
            } else {
                let cx = ctx.data_mut(|d| d.get_temp::<f32>(Id::new("dprev-x"))).unwrap_or(screen.center().x);
                let on_card = self.dock_preview(ui, aid, cx, pill.top(), screen);
                if on_card {
                    self.shell.dock_prev = Some((aid, now));
                }
                ctx.request_repaint();
            }
        } else if let Some((label, cx, top)) = hovered_label {
            let f = f_semi(11.5);
            let w = self.tw(&p, &label, &f) + 20.0;
            let r = Rect::from_center_size(pos2(cx, top - 20.0), vec2(w, 24.0));
            shadow(&p, r, 12.0, 4.0, 10.0, -2.0, alpha(th.shadow, 0.4));
            grad_fill(&p, r, 12.0, th.tooltip_bg, alpha(th.tooltip_bg, 0.92));
            rim(&p, r, 12.0, 1.0, Color32::from_white_alpha(40), true, 12.0);
            p.text(r.center(), Align2::CENTER_CENTER, label, f, if th.dark { hex("#10151f") } else { Color32::WHITE });
        }
        match clicked {
            Some(Di::App(id)) => {
                self.shell.snd.play(Snd::Click);
                self.open_app(id)
            }
            Some(Di::Ai) => self.acts.push(Act::App("ai.toggle".into())),
            Some(Di::Settings) => self.open_app(AppId::Settings),
            Some(Di::Trash) => self.acts.push(Act::App("dock.void".into())),
            _ => {}
        }
    }
}
