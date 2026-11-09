//! The assistant's face: a Spotlight-style bar anywhere on the OS, per-app "Ask Smart" entries in each app's own
//! toolbar (apps own their AI; the shell owns the OS assistant), the full-screen AI mode with a premium orb
//! visualiser, and Studio — where generated apps are rendered from design-system blocks. Engine: `shell/ai.rs`.
use crate::app::*;
use crate::appkit::Frame;
use crate::chrome::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::shell::ai::*;
use crate::shell::{AppId, TOPBAR_H};
use crate::theme::*;
use crate::widgets::*;
use egui::{pos2, vec2, Align2, Color32, Id, Rect, Sense, Ui};

/// The shared AI visualiser: layered halo, radial spectrum bars and a domed core. Used by the bar and AI mode.
pub fn ai_visualiser(p: &egui::Painter, th: &Theme, ctr: egui::Pos2, r: f32, t: f32, level: f32, speaking: bool, listening: bool, busy: bool) {
    let (v, w) = th.accent_grad();
    let amp = if speaking {
        0.55 + 0.45 * (t * 7.0).sin().abs()
    } else if listening {
        (0.35 + 0.65 * level).max(0.3 * (t * 5.0).sin().abs())
    } else if busy {
        0.32 + 0.08 * (t * 3.0).sin()
    } else {
        0.08 + 0.03 * (t * 1.4).sin()
    };
    // halo
    for i in 0..6 {
        p.circle_filled(ctr, r * (2.0 - i as f32 * 0.16) * (1.0 + 0.04 * amp), alpha(v, 0.016));
    }
    // radial spectrum
    let n = 64;
    for i in 0..n {
        let a = -std::f32::consts::FRAC_PI_2 + i as f32 / n as f32 * std::f32::consts::TAU;
        let d = vec2(a.cos(), a.sin());
        let wave = 0.5 + 0.5 * (t * if speaking { 9.0 } else { 3.2 } + i as f32 * 0.5).sin();
        let fine = hash01(i as u32 * 31 + ((t * 12.0) as u32) % 997);
        let len = 2.5 + amp * (r * 0.55) * (0.3 * wave + 0.7 * if speaking || listening { fine } else { 0.3 });
        let c = mix(v, Color32::WHITE, (i as f32 / n as f32 * 2.0 - 1.0).abs() * 0.35);
        p.line_segment([ctr + d * (r + 6.0), ctr + d * (r + 6.0 + len)], egui::Stroke::new(2.4, alpha(c, 0.45 + 0.5 * amp)));
    }
    // orbiting ticks while thinking
    if busy && !speaking {
        for k in 0..3 {
            let a = t * 2.4 + k as f32 * std::f32::consts::TAU / 3.0;
            p.circle_filled(ctr + vec2(a.cos(), a.sin()) * (r + 14.0), 2.6, alpha(Color32::WHITE, 0.8));
        }
    }
    // machined core
    for i in 0..8 {
        p.circle_filled(ctr + vec2(0.0, r * 0.10 + i as f32 * 1.3), r + i as f32 * 1.5, alpha(Color32::BLACK, 0.05));
    }
    dome(p, ctr, r, mix(v, Color32::BLACK, 0.45), mix(w, Color32::WHITE, 0.15));
    p.circle_filled(ctr + vec2(-r * 0.26, -r * 0.40), r * 0.30, Color32::from_white_alpha(52));
    p.circle_filled(ctr + vec2(-r * 0.30, -r * 0.45), r * 0.15, Color32::from_white_alpha(72));
    let core = r * (0.36 + 0.10 * amp);
    p.circle_filled(ctr + vec2(0.0, r * 0.05), core, alpha(Color32::WHITE, 0.16));
}

impl SmartApp {
    fn ai_meta(&self) -> (String, bool) {
        let i = self.shell.ai.cfg.provider.min(4);
        if i == 0 {
            return ("On-device engine \u{b7} no key needed".into(), true);
        }
        let live = i == 4 || !self.shell.ai.cfg.keys[i].is_empty();
        (format!("{} \u{b7} {}", PROVIDERS[i].0, self.shell.ai.cfg.models[i]), live)
    }

    fn ai_input(&mut self, ui: &mut Ui, r: Rect, big: bool) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        well(&p, &th, r, r.height() / 2.0);
        let orb = Rect::from_center_size(pos2(r.left() + 24.0, r.center().y), vec2(26.0, 26.0));
        let (a, b) = th.accent_grad();
        shadow(&p, orb, 13.0, 2.0, 5.0, -1.0, alpha(th.accent, 0.4));
        grad_fill(&p, orb, 13.0, a, b);
        rim(&p, orb, 13.0, 1.0, Color32::from_white_alpha(150), true, 12.0);
        self.ico(&p, "stars-minimalistic", Ic::Filled, orb.center(), 13.0, Color32::WHITE);
        let mic = Rect::from_center_size(pos2(r.right() - 60.0, r.center().y), vec2(30.0, 30.0));
        let send = Rect::from_center_size(pos2(r.right() - 24.0, r.center().y), vec2(32.0, 32.0));
        let field = Rect::from_min_max(pos2(orb.right() + 12.0, r.top() + 4.0), pos2(mic.left() - 6.0, r.bottom() - 4.0));
        // custom single-line field: the shell feeds it keys directly (see shell_frame_begin),
        // so it never loses the keyboard to focus races
        let font = if big { f_semi(15.0) } else { f_medium(14.0) };
        let empty = self.shell.ai.input.is_empty();
        let shown = if empty {
            if self.shell.ai.listening { "Listening\u{2026}".to_string() } else { format!("Ask {} or tell it what to do\u{2026}", self.shell.ai.name) }
        } else {
            self.fit(&p, &self.shell.ai.input.clone(), &font, field.width() - 8.0)
        };
        let col = if empty { th.ink4 } else { th.ink1 };
        let ty = field.center().y;
        p.text(pos2(field.left() + 2.0, ty), Align2::LEFT_CENTER, &shown, font.clone(), col);
        let caret_on = (self.time * 2.2) as i32 % 2 == 0;
        if !empty && caret_on {
            let cw = self.tw(&p, &shown, &font.clone());
            p.line_segment([pos2(field.left() + 4.0 + cw, ty - 8.0), pos2(field.left() + 4.0 + cw, ty + 8.0)], egui::Stroke::new(1.8, th.accent));
        }
        let enter = false;
        let lis = self.shell.ai.listening;
        if self.icon_button(ui, mic, "microphone", "ai-mic", if lis { Btn::Danger } else { Btn::Quiet }, true).on_hover_text(if lis { "Stop listening" } else { "Speak" }).clicked() {
            self.acts.push(Act::App("ai.mic".into()));
        }
        let can = !self.shell.ai.input.trim().is_empty() && !self.shell.ai.busy;
        if self.icon_button(ui, send, "send-square", "ai-send", Btn::Accent, can).on_hover_text("Send").clicked() || (enter && can) {
            let t = self.shell.ai.input.clone();
            self.os_ai_send(t);
            self.shell.ai.focus_input = true;
        }
    }

    fn ai_messages(&mut self, ui: &Ui, area: Rect, key: &str) {
        let th = self.th.clone();
        let p = ui.painter().with_clip_rect(area);
        let now = self.shell.now;
        let msgs = self.shell.ai.msgs.clone();
        let mw = area.width() * 0.82 - 30.0;
        let mut items = vec![];
        let mut total = 10.0;
        for (i, m) in msgs.iter().enumerate() {
            let mut txt = m.text.clone();
            if m.role == Role::Ai && i + 1 == msgs.len() {
                let n = ((now - m.born) * 90.0) as usize;
                if n < txt.chars().count() {
                    txt = txt.chars().take(n).collect();
                    ui.ctx().request_repaint();
                }
            }
            let (font, col) = match m.role {
                Role::Tool => (f_medium(10.5), th.ink3),
                Role::User => (f_medium(T_BODY), th.on_accent()),
                Role::Ai => (f_medium(T_BODY), th.ink1),
            };
            let g = p.layout(txt, font, col, mw);
            let h = g.size().y + if m.role == Role::Tool { 12.0 } else { 22.0 };
            total += h + 8.0;
            items.push((m.role, g, h));
        }
        if self.shell.ai.busy {
            total += 34.0;
        }
        let n = msgs.len() + usize::from(self.shell.ai.busy);
        let last: usize = ui.ctx().data_mut(|d| d.get_temp(Id::new(("ai-n", key))).unwrap_or(0));
        if last != n {
            ui.ctx().data_mut(|d| {
                d.insert_temp(Id::new(("ai-n", key)), n);
                d.insert_temp(Id::new(("vscroll", key)), f32::MAX);
            });
        }
        let off = self.vscroll(ui, area, key, total);
        let mut y = area.top() + 10.0 - off;
        for (role, g, h) in items {
            let w = g.size().x + 26.0;
            match role {
                Role::User => {
                    let r = Rect::from_min_size(pos2(area.right() - 14.0 - w, y), vec2(w, h));
                    let (a, b) = th.accent_grad();
                    shadow(&p, r, 14.0, 3.0, 8.0, -2.0, alpha(th.accent, 0.3));
                    grad_fill(&p, r, 14.0, a, b);
                    rim(&p, r, 14.0, 1.0, Color32::from_white_alpha(120), true, 14.0);
                    p.galley(r.min + vec2(13.0, 11.0), g, th.on_accent());
                }
                Role::Ai => {
                    let r = Rect::from_min_size(pos2(area.left() + 14.0, y), vec2(w, h));
                    grad_fill(&p, r, 14.0, th.card_top, th.card_bottom);
                    rim(&p, r, 14.0, 1.0, th.rim, true, 14.0);
                    p.galley(r.min + vec2(13.0, 11.0), g, th.ink1);
                }
                Role::Tool => {
                    let r = Rect::from_min_size(pos2(area.left() + 14.0, y), vec2(w + 22.0, h));
                    grad_fill(&p, r, h / 2.0, alpha(th.good, 0.16), alpha(th.good, 0.07));
                    self.ico(&p, "check-circle", Ic::Filled, pos2(r.left() + 13.0, r.center().y), 12.0, th.good);
                    p.galley(r.min + vec2(26.0, 6.0), g, th.ink3);
                }
            }
            y += h + 8.0;
        }
        if self.shell.ai.busy {
            for i in 0..3 {
                let ph = (self.time as f32 * 6.0 - i as f32 * 0.8).sin() * 0.5 + 0.5;
                p.circle_filled(pos2(area.left() + 28.0 + i as f32 * 14.0, y + 14.0 - ph * 4.0), 4.0, alpha(th.accent, 0.5 + 0.5 * ph));
            }
        }
    }

    fn ai_chips(&self) -> Vec<&'static str> {
        match self.shell.ai.ctx_app {
            Some(AppId::Photos) => vec!["Make this photo warmer", "Snap photos to the left", "Set a new wallpaper"],
            Some(AppId::Terminal) => vec!["Explain my last command", "run ls", "Snap terminal to the right"],
            Some(AppId::Files) => vec!["Summarize this folder", "Tile my windows", "Make a timer app"],
            Some(AppId::Capture) => vec!["Take a screenshot", "Open Photos", "Tile my windows"],
            _ => vec!["Tile my windows", "Switch to dark mode", "Make a notes app", "What\u{2019}s open?"],
        }
    }

    pub(crate) fn draw_assistant(&mut self, ui: &mut Ui, screen: Rect, ai_t: f32) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let t = self.time as f32;
        let s = spring_to(&ctx, "aipal", if self.shell.ai.open { 1.0 } else { 0.0 }, 260.0, 25.0);
        if s < 0.01 && ai_t < 0.01 {
            return;
        }
        let (meta, live) = self.ai_meta();
        if ai_t > 0.01 {
            // ───────── full-screen AI mode ─────────
            let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
            cui.set_opacity(ai_t);
            let p = cui.painter().clone();
            // keep the wallpaper alive under a subtle themed veil
            grad_fill(&p, screen, 0.0, alpha(if th.dark { Color32::BLACK } else { hex("#e8edf6") }, 0.34 * ai_t), alpha(if th.dark { Color32::BLACK } else { hex("#dfe6f2") }, 0.5 * ai_t));
            let cx = screen.center().x;
            let w = 660.0f32.min(screen.width() - 40.0);
            let ctr = pos2(cx, screen.center().y - 36.0);
            let ai = &self.shell.ai;
            ai_visualiser(&p, &th, ctr, 52.0, t, ai.level, ai.speaking, ai.listening, ai.busy);
            self.ico(&p, "stars-minimalistic", Ic::Filled, ctr + vec2(0.0, 52.0 * 0.05), 22.0, Color32::WHITE);
            // status line under the orb
            let status = if ai.speaking {
                "Speaking\u{2026}".to_string()
            } else if ai.listening {
                "Listening".into()
            } else if ai.busy {
                "Thinking".into()
            } else {
                format!("{} is ready", ai.name)
            };
            p.text(pos2(cx, ctr.y + 96.0), Align2::CENTER_CENTER, status, f_semi(13.0), th.ink2);
            // last exchange
            let last_ai = ai.msgs.iter().rev().find(|m| m.role == Role::Ai).cloned();
            let last_user = ai.msgs.iter().rev().find(|m| m.role == Role::User).cloned();
            if let Some(m) = last_ai {
                let n = ((self.shell.now - m.born) * 70.0) as usize;
                let txt: String = m.text.chars().take(n.max(1)).collect();
                let g = p.layout(txt, f_semi(20.0), th.ink1, (w + 60.0).min(screen.width() - 60.0));
                let card = Rect::from_center_size(pos2(cx, ctr.y - 170.0 - g.size().y / 2.0), g.size() + vec2(56.0, 36.0));
                menu_surface(&p, &th, card, 24.0);
                p.galley(card.center() - g.size() / 2.0, g, th.ink1);
                if let Some(u) = last_user {
                    p.text(pos2(cx, card.top() - 18.0), Align2::CENTER_CENTER, self.fit(&p, &format!("\u{201c}{}\u{201d}", u.text), &f_medium(13.0), w), f_medium(13.0), th.ink3);
                }
            }
            // suggestion chips
            let chips = self.ai_chips();
            let cw: Vec<f32> = chips.iter().map(|c| self.tw(&p, c, &f_semi(T_BODY)) + 30.0).collect();
            let total_w: f32 = cw.iter().sum::<f32>() + (chips.len() - 1) as f32 * 10.0;
            let mut x = cx - total_w / 2.0;
            let cy = screen.bottom() - 168.0;
            for (i, c) in chips.iter().enumerate() {
                let r = Rect::from_min_size(pos2(x, cy), vec2(cw[i], 34.0));
                if self.label_button(&mut cui, r, None, c, &format!("aim-chip-{i}"), Btn::Tonal).clicked() {
                    self.os_ai_send(c.to_string());
                }
                x += cw[i] + 10.0;
            }
            let bar = Rect::from_center_size(pos2(cx, screen.bottom() - 96.0), vec2(w, 58.0));
            elevate(&p, &th, bar, 29.0, 4);
            grad_fill(&p, bar, 29.0, th.panel_top, th.panel_bottom);
            rim(&p, bar, 29.0, 1.4, th.rim, true, 50.0);
            self.ai_input(&mut cui, bar, true);
            let x_ = Rect::from_center_size(pos2(screen.right() - 38.0, screen.top() + TOPBAR_H + 30.0), vec2(36.0, 36.0));
            if self.icon_button(&cui, x_, "close-circle", "ai-exit", Btn::Raised, true).on_hover_text("Leave AI mode  Esc").clicked() {
                self.shell.ai.mode = false;
            }
            p.text(pos2(cx, bar.bottom() + 20.0), Align2::CENTER_CENTER, "Esc to return to the desktop", f_medium(10.5), th.ink3);
            let _ = live;
            let _ = meta;
            return;
        }
        // ───────── Spotlight-style bar ─────────
        let msgs_empty = self.shell.ai.msgs.is_empty();
        let w = 720.0f32.min(screen.width() - 28.0);
        let msg_h = if msgs_empty { 0.0 } else { 300.0f32.min(screen.height() - 320.0) };
        let chips_h = if msgs_empty { 46.0 } else { 0.0 };
        let h = 64.0 + 58.0 + msg_h + chips_h;
        let card = Rect::from_center_size(pos2(screen.center().x, TOPBAR_H + 44.0 + h / 2.0 - (1.0 - s.min(1.0)) * 16.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((s * 2.2).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        elevate(&p, &th, card, 26.0, 5);
        menu_surface(&p, &th, card, 26.0);
        cui.allocate_rect(card, Sense::click());
        // header: accent tile + name + meta chips
        let tile = Rect::from_min_size(card.min + vec2(20.0, 16.0), vec2(34.0, 34.0));
        let (a, b) = th.accent_grad();
        shadow(&p, tile, 11.0, 3.0, 8.0, -2.0, alpha(th.accent, 0.4));
        grad_fill(&p, tile, 11.0, a, b);
        rim(&p, tile, 11.0, 1.0, Color32::from_white_alpha(140), true, 16.0);
        self.ico(&p, "stars-minimalistic", Ic::Duo, tile.center(), 18.0, Color32::WHITE);
        p.text(pos2(tile.right() + 12.0, tile.center().y - 8.0), Align2::LEFT_CENTER, format!("Ask {}", self.shell.ai.name), f_bold(T_TITLE), th.ink1);
        p.text(pos2(tile.right() + 12.0, tile.center().y + 9.0), Align2::LEFT_CENTER, "It can see this app and act on the whole desktop", f_medium(T_META), th.ink3);
        let (mw_, _) = self.ai_meta();
        let chipw = self.tw(&p, &mw_, &f_medium(10.0)) + 20.0;
        let chip = Rect::from_min_size(pos2(card.right() - 84.0 - chipw, tile.center().y - 10.0), vec2(chipw, 20.0));
        grad_fill(&p, chip, 10.0, alpha(if live { th.good } else { th.ink3 }, if live { 0.16 } else { 0.10 }), alpha(if live { th.good } else { th.ink3 }, if live { 0.06 } else { 0.04 }));
        p.circle_filled(pos2(chip.left() + 10.0, chip.center().y), 3.0, if live { th.good } else { th.ink3 });
        p.text(pos2(chip.left() + 18.0, chip.center().y), Align2::LEFT_CENTER, self.fit(&p, &mw_, &f_medium(10.0), chipw - 26.0), f_medium(10.0), th.ink2);
        if let Some(a) = self.shell.ai.ctx_app {
            let cs = format!("in {}", a.name());
            p.text(pos2(chip.right() + 10.0, chip.center().y), Align2::LEFT_CENTER, cs, f_medium(10.0), th.ink3);
        }
        let xb = Rect::from_center_size(pos2(card.right() - 26.0, tile.center().y), vec2(28.0, 28.0));
        let mb = Rect::from_center_size(pos2(card.right() - 58.0, tile.center().y), vec2(28.0, 28.0));
        if self.icon_button(&cui, xb, "close-circle", "ai-x", Btn::Quiet, true).on_hover_text("Close").clicked() {
            self.shell.ai.open = false;
        }
        if self.icon_button(&cui, mb, "full-screen", "ai-full", Btn::Quiet, true).on_hover_text("AI mode").clicked() {
            self.acts.push(Act::App("ai.mode".into()));
        }
        let mut y = card.top() + 62.0;
        if msgs_empty {
            let mut x = card.left() + 20.0;
            for (i, ch) in self.ai_chips().iter().enumerate() {
                let cw = self.tw(&p, ch, &f_semi(T_BODY)) + 30.0;
                if x + cw > card.right() - 16.0 {
                    break;
                }
                let r = Rect::from_min_size(pos2(x, y + 2.0), vec2(cw, 34.0));
                if self.label_button(&cui, r, None, ch, &format!("ai-chip-{i}"), Btn::Tonal).clicked() {
                    self.os_ai_send(ch.to_string());
                }
                x += cw + 10.0;
            }
            y += chips_h;
        } else {
            let area = Rect::from_min_size(pos2(card.left() + 6.0, y), vec2(w - 12.0, msg_h));
            self.ai_messages(&cui, area, "ai-msgs");
            y += msg_h;
        }
        let _ = y;
        let bar = Rect::from_min_size(pos2(card.left() + 16.0, card.bottom() - 60.0), vec2(w - 32.0, 46.0));
        self.ai_input(&mut cui, bar, false);
        if ctx.input(|i| i.pointer.primary_pressed()) && ctx.pointer_interact_pos().is_some_and(|pp| !card.contains(pp) && pp.y > TOPBAR_H) {
            self.shell.ai.open = false;
        }
    }

    // ───────────── Studio: apps generated from design-system blocks ─────────────
    pub fn draw_studio(&mut self, ui: &mut Ui, fr: &Frame) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let gens = self.shell.gen.clone();
        if fr.has_side() {
            p.text(pos2(fr.side_body.left() + 20.0, fr.side_body.top() + 4.0), Align2::LEFT_CENTER, "MADE BY YOUR ASSISTANT", f_semi(9.5), th.ink4);
            for (i, g) in gens.iter().enumerate() {
                let r = Rect::from_min_size(pos2(fr.side_body.left() + 10.0, fr.side_body.top() + 20.0 + i as f32 * 38.0), vec2(fr.side_body.width() - 20.0, 34.0));
                let resp = self.hit(ui, r, ("gen-row", i), Sense::click());
                let on = i == self.shell.gen_sel;
                if on {
                    grad_fill(&p, r, 10.0, th.accent_tint, th.accent_tint2);
                } else if resp.hovered() {
                    p.rect_filled(r, cr(10.0), th.row_hover);
                }
                self.ico(&p, "widget-4", if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 20.0, r.center().y), 16.0, if on { th.accent_ink } else { th.ink3 });
                p.text(pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, self.fit(&p, &g.name, &f_semi(T_BODY), r.width() - 50.0), f_semi(T_BODY), th.ink1);
                if resp.clicked() {
                    self.shell.gen_sel = i;
                }
            }
        }
        let st = fr.stage.shrink2(vec2(26.0, 18.0));
        let Some(app) = gens.get(self.shell.gen_sel).cloned() else {
            let c = fr.stage.center();
            let (a, b) = th.accent_grad();
            let tile = Rect::from_center_size(c - vec2(0.0, 78.0), vec2(64.0, 64.0));
            shadow(&p, tile, 20.0, 8.0, 18.0, -4.0, alpha(th.accent, 0.4));
            grad_fill(&p, tile, 20.0, a, b);
            rim(&p, tile, 20.0, 1.3, Color32::from_white_alpha(150), true, 40.0);
            self.ico(&p, "magic-stick-3", Ic::Duo, tile.center(), 30.0, Color32::WHITE);
            p.text(c - vec2(0.0, 24.0), Align2::CENTER_CENTER, "Nothing here yet", f_bold(21.0), th.ink1);
            p.text(c - vec2(0.0, 2.0), Align2::CENTER_CENTER, format!("Ask {} to build you an app \u{2014} apps are just design-system blocks.", self.shell.ai.name), f_medium(T_BODY), th.ink3);
            for (i, (k, lab)) in [("timer", "Timer"), ("notes", "Notes"), ("habit", "Habits")].iter().enumerate() {
                let r = Rect::from_center_size(c + vec2(-110.0 + i as f32 * 110.0, 44.0), vec2(98.0, 34.0));
                if self.label_button(ui, r, None, lab, &format!("gen-t-{i}"), Btn::Raised).clicked() {
                    self.shell.gen.push(gen_template(k, &format!("{lab} app")));
                    self.shell.gen_sel = self.shell.gen.len() - 1;
                    self.shell.snd.play(crate::sound::Snd::Success);
                }
            }
            return;
        };
        let area = Rect::from_min_max(st.min, fr.stage.max - vec2(0.0, 6.0));
        let pc = p.with_clip_rect(area);
        let off = self.vscroll(ui, area, "studio", ui.ctx().data_mut(|d| d.get_temp::<f32>(Id::new("studio-h"))).unwrap_or(600.0));
        let mut y = area.top() - off;
        let mut clicked: Option<String> = None;
        for (bi, b) in app.blocks.iter().enumerate() {
            match b.kind.as_str() {
                "heading" => {
                    pc.text(pos2(area.left(), y + 20.0), Align2::LEFT_CENTER, &b.text, f_bold(T_PAGE), th.ink1);
                    y += 52.0;
                }
                "text" => {
                    let g = pc.layout(b.text.clone(), f_medium(T_BODY), th.ink2, area.width() - 20.0);
                    let h = g.size().y;
                    pc.galley(pos2(area.left(), y), g, th.ink2);
                    y += h + 16.0;
                }
                "stat" => {
                    let r = Rect::from_min_size(pos2(area.left(), y), vec2(area.width() - 20.0, 92.0));
                    grad_fill(&pc, r, 20.0, th.card_top, th.card_bottom);
                    rim(&pc, r, 20.0, 1.3, th.rim, true, 36.0);
                    pc.text(pos2(r.left() + 22.0, r.center().y - 4.0), Align2::LEFT_CENTER, &b.text, f_bold(38.0), th.ink1);
                    pc.text(pos2(r.right() - 22.0, r.center().y), Align2::RIGHT_CENTER, b.items.first().cloned().unwrap_or_default(), f_medium(T_BODY), th.ink3);
                    y += 108.0;
                }
                "progress" => {
                    pc.text(pos2(area.left(), y + 8.0), Align2::LEFT_CENTER, &b.text, f_semi(T_BODY), th.ink1);
                    pc.text(pos2(area.right() - 20.0, y + 8.0), Align2::RIGHT_CENTER, format!("{:.0}%", b.value * 100.0), f_semi(T_BODY), th.ink2);
                    let bar = Rect::from_min_size(pos2(area.left(), y + 24.0), vec2(area.width() - 20.0, 10.0));
                    well(&pc, &th, bar, 5.0);
                    let f = Rect::from_min_size(bar.min, vec2((bar.width() * b.value.clamp(0.0, 1.0)).max(10.0), 10.0));
                    let (a, c2) = th.accent_grad();
                    grad_fill(&pc, f, 5.0, a, c2);
                    y += 48.0;
                }
                "list" => {
                    for it in &b.items {
                        let r = Rect::from_min_size(pos2(area.left(), y), vec2(area.width() - 20.0, 40.0));
                        grad_fill(&pc, r, 12.0, th.card_top, th.card_bottom);
                        rim(&pc, r, 12.0, 1.0, th.rim, true, 14.0);
                        pc.circle_stroke(pos2(r.left() + 20.0, r.center().y), 7.0, egui::Stroke::new(1.5, th.accent));
                        pc.text(pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, self.fit(&pc, it, &f_semi(T_BODY), r.width() - 56.0), f_semi(T_BODY), th.ink1);
                        y += 46.0;
                    }
                    y += 8.0;
                }
                "buttons" => {
                    let mut x = area.left();
                    for (ii, it) in b.items.iter().enumerate() {
                        let bw = self.tw(&pc, it, &f_semi(T_BODY)) + 36.0;
                        let r = Rect::from_min_size(pos2(x, y), vec2(bw, 36.0));
                        if self.label_button(ui, r, None, it, &format!("gen-b-{bi}-{ii}"), if ii == 0 { Btn::Accent } else { Btn::Raised }).clicked() {
                            clicked = Some(it.clone());
                        }
                        x += bw + 10.0;
                    }
                    y += 54.0;
                }
                _ => {}
            }
        }
        let content_h = y + off - area.top() + 20.0;
        ui.ctx().data_mut(|d| d.insert_temp(Id::new("studio-h"), content_h));
        if let Some(c) = clicked {
            self.shell.notify(&format!("{} \u{b7} {}", app.name, c), "Wire this button to a tool \u{2014} ask your assistant", crate::shell::NoteKind::Info);
        }
    }
}
