//! The assistant's face: a Spotlight-style bar anywhere on the OS, per-window orbs that open it in context, the full-screen
//! AI mode (dock steps aside, a voice orb breathes and listens), and Studio — where generated apps are rendered from
//! design-system blocks. The engine lives in `shell/ai.rs`.
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

impl SmartApp {
    /// Small orb in the corner of every window: one tap asks the assistant about *this* app.
    pub fn ai_orb(&mut self, ui: &mut Ui, id: AppId, win: Rect) {
        if matches!(id, AppId::Settings) && false {
            return;
        }
        let th = self.th.clone();
        let r = Rect::from_center_size(pos2(win.right() - 36.0, win.bottom() - 36.0), vec2(30.0, 30.0));
        let resp = self.hit(ui, r.expand(4.0), ("ai-orb", id), Sense::click());
        let h = self.anim(ui, ("ai-orb-h", id), resp.hovered());
        let p = ui.painter().clone();
        let name = self.shell.ai.name.clone();
        if h > 0.02 {
            let lab = format!("Ask {name} about {}", id.name());
            let lw = self.tw(&p, &lab, &f_semi(11.0)) + 22.0;
            let pill = Rect::from_min_max(pos2(r.left() - 6.0 - lw * h, r.top() + 3.0), pos2(r.right() - 10.0, r.bottom() - 3.0));
            shadow(&p, pill, 12.0, 3.0, 8.0, -2.0, alpha(th.shadow, 0.3 * h));
            grad_fill(&p, pill, 12.0, alpha(th.panel_top, h), alpha(th.panel_bottom, h));
            rim(&p, pill, 12.0, 1.0, alpha(th.rim, h), true, 12.0);
            p.with_clip_rect(pill).text(pos2(pill.left() + 11.0, pill.center().y), Align2::LEFT_CENTER, lab, f_semi(11.0), alpha(th.ink1, h));
        }
        let busy = self.shell.ai.busy;
        shadow(&p, r, 15.0, 3.0, 9.0 + 5.0 * h, -2.0, alpha(hex("#e0569c"), 0.45));
        grad_fill(&p, r.expand(h * 1.5), 16.0, hex("#a58bff"), hex("#e0569c"));
        rim(&p, r.expand(h * 1.5), 16.0, 1.2, Color32::from_white_alpha(160), true, 14.0);
        if busy {
            let pulse = 0.5 + 0.5 * (self.time * 6.0).sin() as f32;
            p.circle_stroke(r.center(), 15.0 + 5.0 * pulse, egui::Stroke::new(1.5, alpha(Color32::WHITE, 0.5 * (1.0 - pulse))));
        }
        self.ico(&p, "stars-minimalistic", Ic::Filled, r.center(), 15.0, Color32::WHITE);
        if resp.clicked() {
            self.acts.push(Act::App(format!("ai.open:{}", id.name())));
        }
    }

    fn voice_orb(&self, p: &egui::Painter, ctr: egui::Pos2, r: f32) {
        let t = self.time as f32;
        let ai = &self.shell.ai;
        let (sp, li, bu) = (ai.speaking, ai.listening, ai.busy);
        let amp = if sp { 0.65 + 0.35 * (t * 7.0).sin().abs() } else if li { 0.3 + 0.7 * ai.level.max(0.25 * (t * 5.0).sin().abs()) } else if bu { 0.3 } else { 0.07 + 0.03 * (t * 1.4).sin() };
        let (v, pk) = (hex("#a58bff"), hex("#ff7ac0"));
        for i in 0..5 {
            p.circle_filled(ctr, r * (1.9 - i as f32 * 0.14) * (1.0 + 0.05 * amp), alpha(v, 0.018 + 0.004 * amp));
        }
        for i in 0..72 {
            let a = i as f32 / 72.0 * std::f32::consts::TAU;
            let d = vec2(a.cos(), a.sin());
            let wave = (t * if sp { 9.0 } else { 3.0 } + i as f32 * 0.55).sin() * 0.5 + 0.5;
            let rnd = hash01(i * 13 + (t * 14.0) as u32);
            let len = 3.0 + amp * (10.0 + r * 0.55) * (0.35 * wave + 0.65 * if sp || li { rnd } else { 0.35 });
            p.line_segment([ctr + d * (r + 7.0), ctr + d * (r + 7.0 + len)], egui::Stroke::new(2.6, mix(v, pk, i as f32 / 72.0)));
        }
        for i in 0..8 {
            p.circle_filled(ctr + vec2(0.0, r * 0.12 + i as f32 * 1.4), r + 1.0 + i as f32, alpha(Color32::BLACK, 0.04));
        }
        dome(p, ctr, r, hex("#3a2b86"), hex("#e07ac8"));
        p.circle_filled(ctr + vec2(-r * 0.28, -r * 0.42), r * 0.30, Color32::from_white_alpha(46));
        p.circle_filled(ctr + vec2(-r * 0.30, -r * 0.46), r * 0.16, Color32::from_white_alpha(60));
        let core = r * (0.34 + 0.12 * amp);
        p.circle_filled(ctr + vec2(0.0, r * 0.06), core, alpha(Color32::WHITE, 0.16));
    }

    fn ai_messages(&mut self, ui: &Ui, area: Rect, key: &str) {
        let th = self.th.clone();
        let p = ui.painter().with_clip_rect(area);
        let now = self.shell.now;
        let msgs = self.shell.ai.msgs.clone();
        let mw = area.width() * 0.8 - 30.0;
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
                Role::Tool => (f_mono(10.5), th.ink3),
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
                p.circle_filled(pos2(area.left() + 28.0 + i as f32 * 14.0, y + 14.0 - ph * 4.0), 4.0, alpha(hex("#a58bff"), 0.5 + 0.5 * ph));
            }
        }
    }

    fn ai_input(&mut self, ui: &mut Ui, r: Rect, big: bool) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        well(&p, &th, r, r.height() / 2.0);
        let orb = Rect::from_center_size(pos2(r.left() + 24.0, r.center().y), vec2(26.0, 26.0));
        grad_fill(&p, orb, 13.0, hex("#a58bff"), hex("#e0569c"));
        rim(&p, orb, 13.0, 1.0, Color32::from_white_alpha(140), true, 12.0);
        self.ico(&p, "stars-minimalistic", Ic::Filled, orb.center(), 14.0, Color32::WHITE);
        let send = Rect::from_center_size(pos2(r.right() - 22.0, r.center().y), vec2(32.0, 32.0));
        let mic = Rect::from_center_size(pos2(r.right() - 58.0, r.center().y), vec2(32.0, 32.0));
        let field = Rect::from_min_max(pos2(orb.right() + 12.0, r.top() + 4.0), pos2(mic.left() - 8.0, r.bottom() - 4.0));
        let mut text = std::mem::take(&mut self.shell.ai.input);
        let hint = if self.shell.ai.listening { "Listening\u{2026}".to_string() } else { format!("Ask {} or tell it what to do\u{2026}", self.shell.ai.name) };
        let te = egui::TextEdit::singleline(&mut text).font(f_mono_b(if big { 15.5 } else { 14.0 })).hint_text(hint).frame(egui::Frame::NONE).text_color(th.ink1).desired_width(field.width());
        let resp = ui.put(Rect::from_center_size(field.center(), vec2(field.width(), 26.0)), te);
        self.shell.ai.input = text;
        if self.shell.ai.focus_input {
            resp.request_focus();
            self.shell.ai.focus_input = false;
        }
        let enter = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        let lis = self.shell.ai.listening;
        if self.icon_button(ui, mic, "microphone", "ai-mic", if lis { Btn::Danger } else { Btn::Quiet }, true).on_hover_text("Speak").clicked() {
            self.acts.push(Act::App("ai.mic".into()));
        }
        let can = !self.shell.ai.input.trim().is_empty() && !self.shell.ai.busy;
        if self.icon_button(ui, send, "send-square", "ai-send", Btn::Accent, can).clicked() || (enter && can) {
            let t = self.shell.ai.input.clone();
            self.os_ai_send(t);
            self.shell.ai.focus_input = true;
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
        let s = spring_to(&ctx, "aipal", if self.shell.ai.open { 1.0 } else { 0.0 }, 260.0, 25.0);
        if s < 0.01 && ai_t < 0.01 {
            return;
        }
        if ai_t > 0.01 {
            // ───────── full-screen AI mode ─────────
            let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
            cui.set_opacity(ai_t);
            let p = cui.painter().clone();
            let cx = screen.center().x;
            let w = 640.0f32.min(screen.width() - 40.0);
            let ctr = pos2(cx, screen.center().y - 10.0);
            self.voice_orb(&p, ctr, 54.0);
            let last_ai = self.shell.ai.msgs.iter().rev().find(|m| m.role == Role::Ai).cloned();
            let last_user = self.shell.ai.msgs.iter().rev().find(|m| m.role == Role::User).cloned();
            if let Some(m) = last_ai {
                let n = ((self.shell.now - m.born) * 70.0) as usize;
                let txt: String = m.text.chars().take(n.max(1)).collect();
                let g = p.layout(txt, f_semi(22.0), th.ink1, (w + 80.0).min(screen.width() - 60.0));
                let card = Rect::from_center_size(pos2(cx, ctr.y - 150.0 - g.size().y / 2.0), g.size() + vec2(56.0, 36.0));
                menu_surface(&p, &th, card, 24.0);
                p.galley(card.center() - g.size() / 2.0, g, th.ink1);
                if let Some(u) = last_user {
                    p.text(pos2(cx, card.top() - 18.0), Align2::CENTER_CENTER, self.fit(&p, &format!("\u{201c}{}\u{201d}", u.text), &f_medium(13.0), w), f_medium(13.0), Color32::WHITE);
                }
            } else {
                let lab = format!("{} is listening", self.shell.ai.name);
                let g = p.layout(lab.clone(), f_semi(20.0), th.ink1, 500.0);
                let card = Rect::from_center_size(pos2(cx, ctr.y - 130.0), g.size() + vec2(44.0, 26.0));
                menu_surface(&p, &th, card, 20.0);
                p.galley(card.center() - g.size() / 2.0, g, th.ink1);
            }
            let bar = Rect::from_center_size(pos2(cx, screen.bottom() - 100.0), vec2(w, 56.0));
            menu_surface(&p, &th, bar, 28.0);
            self.ai_input(&mut cui, bar, true);
            let x = Rect::from_center_size(pos2(screen.right() - 38.0, screen.top() + TOPBAR_H + 30.0), vec2(36.0, 36.0));
            if self.icon_button(&cui, x, "close-circle", "ai-exit", Btn::Raised, true).on_hover_text("Leave AI mode  Esc").clicked() {
                self.shell.ai.mode = false;
            }
            p.text(pos2(cx, bar.bottom() + 20.0), Align2::CENTER_CENTER, "Esc to return to the desktop", f_mono(10.5), alpha(Color32::WHITE, 0.75));
            return;
        }
        // ───────── Spotlight-style bar ─────────
        let msgs_empty = self.shell.ai.msgs.is_empty();
        let w = 700.0f32.min(screen.width() - 28.0);
        let msg_h = if msgs_empty { 0.0 } else { 300.0f32.min(screen.height() - 300.0) };
        let chips_h = if msgs_empty { 50.0 } else { 0.0 };
        let h = 62.0 + 64.0 + msg_h + chips_h;
        let card = Rect::from_center_size(pos2(screen.center().x, TOPBAR_H + 40.0 + h / 2.0 - (1.0 - s.min(1.0)) * 16.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((s * 2.2).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, card, 26.0);
        cui.allocate_rect(card, Sense::click());
        let name = self.shell.ai.name.clone();
        p.text(pos2(card.left() + 22.0, card.top() + 28.0), Align2::LEFT_CENTER, format!("Ask {name}"), f_bold(T_TITLE), th.ink1);
        let prov = PROVIDERS[self.shell.ai.cfg.provider.min(4)].0;
        let live = self.shell.ai.cfg.provider != 0 && (self.shell.ai.cfg.provider == 4 || !self.shell.ai.cfg.keys[self.shell.ai.cfg.provider].is_empty());
        let chip = format!("{} \u{b7} {}", prov, if live { "connected" } else { "no key \u{2014} on-device skills" });
        let cw = self.tw(&p, &chip, &f_mono(9.5)) + 18.0;
        let cr_ = Rect::from_min_size(pos2(card.left() + 110.0, card.top() + 19.0), vec2(cw, 18.0));
        grad_fill(&p, cr_, 9.0, alpha(if live { th.good } else { th.warn }, 0.2), alpha(if live { th.good } else { th.warn }, 0.08));
        p.text(cr_.center(), Align2::CENTER_CENTER, chip, f_mono(9.5), if live { th.good } else { th.warn });
        if let Some(a) = self.shell.ai.ctx_app {
            let ctx_s = format!("in {}", a.name());
            p.text(pos2(card.left() + 120.0 + cw, card.top() + 28.0), Align2::LEFT_CENTER, ctx_s, f_medium(T_META), th.ink3);
        }
        let xb = Rect::from_center_size(pos2(card.right() - 26.0, card.top() + 28.0), vec2(28.0, 28.0));
        let mb = Rect::from_center_size(pos2(card.right() - 60.0, card.top() + 28.0), vec2(28.0, 28.0));
        if self.icon_button(&cui, xb, "close-circle", "ai-x", Btn::Quiet, true).clicked() {
            self.shell.ai.open = false;
        }
        if self.icon_button(&cui, mb, "full-screen", "ai-full", Btn::Quiet, true).on_hover_text("AI mode").clicked() {
            self.acts.push(Act::App("ai.mode".into()));
        }
        let mut y = card.top() + 54.0;
        if msgs_empty {
            let mut x = card.left() + 20.0;
            for (i, ch) in self.ai_chips().iter().enumerate() {
                let cw = self.tw(&p, ch, &f_semi(T_META)) + 26.0;
                if x + cw > card.right() - 16.0 {
                    break;
                }
                let r = Rect::from_min_size(pos2(x, y + 4.0), vec2(cw, 30.0));
                if self.label_button(&cui, r, None, ch, &format!("ai-chip-{i}"), Btn::Tonal).clicked() {
                    self.os_ai_send(ch.to_string());
                }
                x += cw + 8.0;
            }
            y += chips_h;
        } else {
            let area = Rect::from_min_size(pos2(card.left() + 6.0, y), vec2(w - 12.0, msg_h));
            self.ai_messages(&cui, area, "ai-msgs");
            y += msg_h;
        }
        let bar = Rect::from_min_size(pos2(card.left() + 16.0, card.bottom() - 62.0), vec2(w - 32.0, 46.0));
        let _ = y;
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
            self.ico(&p, "magic-stick-3", Ic::Duo, c - vec2(0.0, 70.0), 44.0, th.accent_ink);
            p.text(c - vec2(0.0, 28.0), Align2::CENTER_CENTER, "Nothing here yet", f_bold(T_PAGE * 0.8), th.ink1);
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
        let mut content_h = 0.0;
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
                    pc.text(pos2(r.left() + 22.0, r.center().y - 4.0), Align2::LEFT_CENTER, &b.text, f_mono_b(38.0), th.ink1);
                    pc.text(pos2(r.right() - 22.0, r.center().y), Align2::RIGHT_CENTER, b.items.first().cloned().unwrap_or_default(), f_medium(T_BODY), th.ink3);
                    y += 108.0;
                }
                "progress" => {
                    pc.text(pos2(area.left(), y + 8.0), Align2::LEFT_CENTER, &b.text, f_semi(T_BODY), th.ink1);
                    pc.text(pos2(area.right() - 20.0, y + 8.0), Align2::RIGHT_CENTER, format!("{:.0}%", b.value * 100.0), f_mono_b(T_BODY), th.ink2);
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
        content_h += y + off - area.top() + 20.0;
        ui.ctx().data_mut(|d| d.insert_temp(Id::new("studio-h"), content_h));
        if let Some(c) = clicked {
            self.shell.notify(&format!("{} \u{b7} {}", app.name, c), "Wire this button to a tool \u{2014} ask your assistant", crate::shell::NoteKind::Info);
        }
    }
}
