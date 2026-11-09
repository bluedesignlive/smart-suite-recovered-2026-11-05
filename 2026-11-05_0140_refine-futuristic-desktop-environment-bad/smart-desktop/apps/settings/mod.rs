//! Smart Settings — the native settings app for the desktop environment itself (Files has its own preferences).
//! Every row writes straight into shell state, so changes are visible and audible immediately.
use crate::app::*;
use crate::appkit::Frame;
use crate::chrome::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::shell::ai::PROVIDERS;
use crate::shell::AppId;
use crate::sound::Snd;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Id, Rect, Sense, Ui};

const PAGES: [(&str, &str); 8] = [
    ("palette", "Appearance"),
    ("widget-4", "Dock & desktop"),
    ("window-frame", "Windows"),
    ("volume-loud", "Sound & voice"),
    ("stars-minimalistic", "Assistant"),
    ("clipboard-list", "Clipboard"),
    ("global", "Session"),
    ("info-circle", "About"),
];

impl SmartApp {
    fn s_head(&self, p: &egui::Painter, x: f32, y: &mut f32, title: &str, sub: &str) {
        p.text(pos2(x, *y + 14.0), Align2::LEFT_CENTER, title, f_bold(T_PAGE * 0.85), self.th.ink1);
        p.text(pos2(x, *y + 38.0), Align2::LEFT_CENTER, sub, f_medium(T_BODY), self.th.ink3);
        *y += 62.0;
    }

    /// Row frame: icon tile + label/sub. Returns the control area (right side).
    fn s_row(&self, ui: &Ui, x: f32, y: &mut f32, w: f32, icon: &'static str, label: &str, sub: &str, ctl_w: f32) -> Rect {
        let th = &self.th;
        let p = ui.painter();
        let r = Rect::from_min_size(pos2(x, *y), vec2(w, 54.0));
        grad_fill(p, r, 14.0, th.card_top, th.card_bottom);
        rim(p, r, 14.0, 1.0, th.rim, true, 20.0);
        let ic = Rect::from_center_size(pos2(r.left() + 28.0, r.center().y), vec2(34.0, 34.0));
        grad_fill(p, ic, 11.0, th.accent_tint, th.accent_tint2);
        self.ico(p, icon, Ic::Duo, ic.center(), 18.0, th.accent_ink);
        let tw_ = w - 64.0 - ctl_w - 16.0;
        p.text(pos2(r.left() + 54.0, r.center().y - 8.0), Align2::LEFT_CENTER, self.fit(p, label, &f_semi(T_BODY), tw_), f_semi(T_BODY), th.ink1);
        p.text(pos2(r.left() + 54.0, r.center().y + 9.0), Align2::LEFT_CENTER, self.fit(p, sub, &f_medium(T_META), tw_), f_medium(T_META), th.ink3);
        *y += 62.0;
        Rect::from_min_max(pos2(r.right() - 14.0 - ctl_w, r.top()), pos2(r.right() - 14.0, r.bottom()))
    }

    fn s_toggle(&mut self, ui: &Ui, x: f32, y: &mut f32, w: f32, icon: &'static str, label: &str, sub: &str, on: bool, id: &str) -> bool {
        let ctl = self.s_row(ui, x, y, w, icon, label, sub, 46.0);
        let clicked = self.switch(ui, Rect::from_center_size(ctl.center(), vec2(46.0, 26.0)), on, id);
        if clicked {
            self.shell.snd.play(Snd::Toggle);
        }
        clicked
    }

    fn s_slider(&mut self, ui: &Ui, x: f32, y: &mut f32, w: f32, icon: &'static str, label: &str, sub: &str, v: f32, lo: f32, hi: f32, unit: &str, id: &str) -> Option<f32> {
        let ctl = self.s_row(ui, x, y, w, icon, label, sub, 190.0);
        let track = Rect::from_min_size(pos2(ctl.left(), ctl.center().y - 4.0), vec2(130.0, 8.0));
        let (nt, resp) = self.slider(ui, track, ((v - lo) / (hi - lo)).clamp(0.0, 1.0), id);
        ui.painter().text(pos2(ctl.right(), ctl.center().y), Align2::RIGHT_CENTER, format!("{:.0}{}", v, unit), f_mono_b(T_META), self.th.ink2);
        if resp.dragged() || resp.clicked() {
            Some(lo + nt * (hi - lo))
        } else {
            None
        }
    }

    fn s_seg(&mut self, ui: &Ui, x: f32, y: &mut f32, w: f32, icon: &'static str, label: &str, sub: &str, opts: &[&str], sel: usize, id: &str) -> Option<usize> {
        let cw: f32 = opts.iter().map(|o| self.tw(ui.painter(), o, &f_semi(T_META)) + 24.0).sum::<f32>() + 6.0;
        let ctl = self.s_row(ui, x, y, w, icon, label, sub, cw);
        let o: Vec<(&'static str, &str)> = opts.iter().map(|s| ("", *s)).collect();
        let r = self.segment(ui, Rect::from_center_size(ctl.center(), vec2(cw, 30.0)), &o, false, sel, id);
        if r.is_some() {
            self.shell.snd.play(Snd::Toggle);
        }
        r
    }

    fn s_text(&mut self, ui: &mut Ui, x: f32, y: &mut f32, w: f32, icon: &'static str, label: &str, sub: &str, buf: &mut String, hint: &str, pw: bool) -> bool {
        let ctl = self.s_row(ui, x, y, w, icon, label, sub, 260.0);
        let r = Rect::from_center_size(ctl.center(), vec2(260.0, 32.0));
        well(ui.painter(), &self.th, r, 10.0);
        let te = egui::TextEdit::singleline(buf).font(f_mono(12.0)).hint_text(hint).password(pw).frame(egui::Frame::NONE).text_color(self.th.ink1).desired_width(236.0);
        let resp = ui.put(Rect::from_center_size(r.center(), vec2(240.0, 24.0)), te);
        resp.changed()
    }

    fn s_btn(&mut self, ui: &Ui, x: f32, y: &mut f32, w: f32, icon: &'static str, label: &str, sub: &str, cta: &str, id: &str) -> bool {
        let bw = self.tw(ui.painter(), cta, &f_semi(T_BODY)) + 34.0;
        let ctl = self.s_row(ui, x, y, w, icon, label, sub, bw);
        self.label_button(ui, Rect::from_center_size(ctl.center(), vec2(bw, 32.0)), None, cta, id, Btn::Raised).clicked()
    }

    pub fn draw_settings(&mut self, ui: &mut Ui, fr: &Frame) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let page = self.shell.set_page.min(PAGES.len() - 1);
        if fr.has_side() {
            p.text(pos2(fr.side_body.left() + 20.0, fr.side_body.top() + 6.0), Align2::LEFT_CENTER, "SMART DESKTOP", f_semi(9.5), th.ink4);
            for (i, (icon, name)) in PAGES.iter().enumerate() {
                let r = Rect::from_min_size(pos2(fr.side_body.left() + 10.0, fr.side_body.top() + 22.0 + i as f32 * 38.0), vec2(fr.side_body.width() - 20.0, 34.0));
                let resp = self.hit(ui, r, ("set-p", i), Sense::click());
                let on = i == page;
                if on {
                    grad_fill(&p, r, 10.0, th.accent_tint, th.accent_tint2);
                    rim(&p, r, 10.0, 1.0, alpha(Color32::WHITE, if th.dark { 0.10 } else { 0.7 }), true, 12.0);
                } else if resp.hovered() {
                    p.rect_filled(r, cr(10.0), th.row_hover);
                }
                self.ico(&p, icon, if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 20.0, r.center().y), 16.0, if on { th.accent_ink } else { th.ink3 });
                p.text(pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, *name, if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, th.ink1);
                if resp.clicked() {
                    self.shell.set_page = i;
                    self.shell.snd.play(Snd::Tick);
                }
            }
        }
        let area = Rect::from_min_max(pos2(fr.stage.left() + 24.0, fr.stage.top() + 6.0), pos2(fr.stage.right() - 24.0, fr.stage.bottom() - 6.0));
        let hkey = Id::new(("set-h", page));
        let ch = ui.ctx().data_mut(|d| d.get_temp::<f32>(hkey)).unwrap_or(900.0);
        let off = self.vscroll(ui, area, &format!("settings-{page}"), ch);
        let w = (area.width() - 14.0).min(760.0);
        let x = area.left();
        let mut y = area.top() - off + 4.0;
        let y0 = y;
        let (tfp, ui2) = (th.clone(), &mut *ui);
        let _ = &tfp;
        match page {
            0 => {
                self.s_head(&p, x, &mut y, "Appearance", "How the whole desktop looks. Apps follow instantly.");
                let dark = self.st().dark;
                if self.s_seg(ui2, x, &mut y, w, "moon-stars", "Theme", "Light or dark \u{2014} no overlay is ever placed over the desktop", &["Light", "Dark"], dark as usize, "set-theme").is_some() {
                    self.acts.push(Act::ToggleTheme);
                }
                if self.s_btn(ui2, x, &mut y, w, "gallery-wide", "Wallpaper", "Pictures, built-in surfaces and video backgrounds", "Choose\u{2026}", "set-wall") {
                    self.app_cmd("desktop.wallpaper");
                }
                let rm = self.lib(|l| l.settings.reduce_motion);
                if self.s_toggle(ui2, x, &mut y, w, "slider-horizontal", "Reduce motion", "Windows jump instead of gliding", rm, "set-rm") {
                    self.set(|s| s.reduce_motion = !rm);
                }
            }
            1 => {
                self.s_head(&p, x, &mut y, "Dock & desktop", "Size, magnification and where things hide.");
                let sz = self.shell.dock.size;
                if let Some(v) = self.s_slider(ui2, x, &mut y, w, "widget-4", "Icon size", "Base size of dock icons", sz, 34.0, 64.0, " px", "set-dsz") {
                    self.shell.dock.size = v;
                }
                let m = self.shell.dock.mag;
                if let Some(v) = self.s_slider(ui2, x, &mut y, w, "magnifer-zoom-in", "Magnification", "How much icons swell under the pointer", m * 100.0, 0.0, 100.0, "%", "set-mag") {
                    self.shell.dock.mag = v / 100.0;
                }
                let ah = self.shell.dock.autohide;
                if self.s_toggle(ui2, x, &mut y, w, "eye", "Hide the dock automatically", "It returns when the pointer touches the bottom edge", ah, "set-ah") {
                    self.shell.dock.autohide = !ah;
                }
                if self.s_btn(ui2, x, &mut y, w, "widget-4", "Desktop icons", "Snap every icon back to the grid", "Clean up", "set-clean") {
                    self.app_cmd("desktop.cleanup");
                }
            }
            2 => {
                self.s_head(&p, x, &mut y, "Windows", "The scene-graph window manager.");
                let sn = self.shell.wm.snap;
                if self.s_toggle(ui2, x, &mut y, w, "window-frame", "Snap to edges", "Drag a window to an edge or corner to tile it", sn, "set-snap") {
                    self.shell.wm.snap = !sn;
                }
                let st = self.shell.wm.stiff;
                if let Some(v) = self.s_slider(ui2, x, &mut y, w, "bolt", "Glide speed", "How quickly windows settle into place", st, 120.0, 520.0, "", "set-stiff") {
                    self.shell.wm.stiff = v;
                    self.shell.wm.damp = 2.0 * v.sqrt() * 0.92;
                }
                for (i, (ic, l, s, cmd)) in [("widget-4", "Tile all windows", "Grid layout", "wm.tile"), ("layers-minimalistic", "Cascade windows", "Overlapping stack", "wm.cascade"), ("sidebar-minimalistic", "Split view", "Front window left, the rest right", "wm.split")].iter().enumerate() {
                    if self.s_btn(ui2, x, &mut y, w, ic, l, s, "Apply", &format!("set-wm{i}")) {
                        self.app_cmd(cmd);
                    }
                }
            }
            3 => {
                self.s_head(&p, x, &mut y, "Sound & voice", "Every action has a sound; learn them and you can work by ear.");
                let on = self.shell.snd.on;
                if self.s_toggle(ui2, x, &mut y, w, "volume-loud", "Interface sounds", "Clicks, windows, transfers, devices, errors", on, "set-snd") {
                    self.shell.snd.on = !on;
                }
                let v = self.shell.volume;
                if let Some(nv) = self.s_slider(ui2, x, &mut y, w, "volume-small", "Volume", "System output level", v * 100.0, 0.0, 100.0, "%", "set-vol") {
                    self.shell.volume = nv / 100.0;
                }
                let tts = self.shell.tts_on;
                if self.s_toggle(ui2, x, &mut y, w, "soundwave", "Read notifications aloud", "Speaks banners and dialogs as they appear", tts, "set-tts") {
                    self.shell.tts_on = !tts;
                }
                let rate = self.shell.tts_rate;
                if let Some(nv) = self.s_slider(ui2, x, &mut y, w, "bolt", "Speaking rate", "Voice speed", rate * 100.0, 70.0, 160.0, "%", "set-rate") {
                    self.shell.tts_rate = nv / 100.0;
                }
                for (i, (l, s, snd)) in [("Notification", "Two-note bell", Snd::Notify), ("Transfer complete", "Rising arpeggio", Snd::XferDone), ("Error", "Low, dissonant", Snd::Error), ("Device mounted", "Glide up + click", Snd::Mount)].iter().enumerate() {
                    if self.s_btn(ui2, x, &mut y, w, "volume-loud", l, s, "Play", &format!("set-play{i}")) {
                        self.shell.snd.play(*snd);
                    }
                }
            }
            4 => {
                self.s_head(&p, x, &mut y, "Assistant", "One brain across every app, with a memory you control.");
                let mut name = self.shell.ai.name.clone();
                if self.s_text(ui2, x, &mut y, w, "user-circle", "Name", "Call it anything \u{2014} it answers to it", &mut name, "Zuri", false) {
                    self.shell.ai.name = name;
                    self.shell.ai.save();
                }
                let names: Vec<&str> = PROVIDERS.iter().map(|p| p.0).collect();
                let prov = self.shell.ai.cfg.provider;
                if let Some(i) = self.s_seg(ui2, x, &mut y, w + 0.0, "cloud", "Provider", "OpenRouter, NVIDIA NIM, any OpenAI-compatible endpoint, Ollama or on-device", &names, prov, "set-prov") {
                    self.shell.ai.cfg.provider = i;
                    self.shell.ai.save();
                }
                let prov = self.shell.ai.cfg.provider;
                if prov != 0 {
                    let mut model = self.shell.ai.cfg.models[prov].clone();
                    if self.s_text(ui2, x, &mut y, w, "cpu", "Model", "As named by the provider", &mut model, PROVIDERS[prov].2, false) {
                        self.shell.ai.cfg.models[prov] = model;
                        self.shell.ai.save();
                    }
                    if prov != 4 {
                        let mut key = std::mem::take(&mut self.shell.ai.cfg.keys[prov]);
                        let ch = self.s_text(ui2, x, &mut y, w, "lock", "API key", "Stored on this device only", &mut key, "paste your key", true);
                        self.shell.ai.cfg.keys[prov] = key;
                        if ch {
                            self.shell.ai.save();
                        }
                    }
                    if prov == 3 {
                        let mut base = self.shell.ai.cfg.base.clone();
                        if self.s_text(ui2, x, &mut y, w, "link", "Base URL", "Any OpenAI-compatible gateway", &mut base, "https://api.openai.com/v1", false) {
                            self.shell.ai.cfg.base = base;
                            self.shell.ai.save();
                        }
                    }
                }
                let vr = self.shell.ai.cfg.voice_reply;
                if self.s_toggle(ui2, x, &mut y, w, "soundwave", "Speak replies", "Read answers aloud (always on in AI mode)", vr, "set-vr") {
                    self.shell.ai.cfg.voice_reply = !vr;
                    self.shell.ai.save();
                }
                let n = self.shell.ai.memory.len();
                if self.s_btn(ui2, x, &mut y, w, "database", "Global memory", &format!("{n} facts shared by every app \u{2014} say \u{201c}remember\u{2026}\u{201d} to add"), "Forget all", "set-forget") {
                    self.shell.ai.memory.clear();
                    self.shell.ai.save();
                }
            }
            5 => {
                self.s_head(&p, x, &mut y, "Clipboard", "A history of everything you copy, in every app.");
                let n = self.shell.clip.len();
                if self.s_btn(ui2, x, &mut y, w, "clipboard-list", "History", &format!("{n} items kept (pinned items stay)"), "Clear", "set-clip") {
                    self.app_cmd("clip.clear");
                }
                if self.s_btn(ui2, x, &mut y, w, "keyboard", "Open history", "Also in the status bar", "Show", "set-clip-open") {
                    self.shell.clip_open = true;
                }
            }
            6 => {
                self.s_head(&p, x, &mut y, "Session", "Your desktop can run anywhere and be used from anywhere.");
                let rem = self.shell.remote;
                if self.s_toggle(ui2, x, &mut y, w, "global", "Remote session", "Compositor in one region, you in another \u{2014} only deltas travel", rem, "set-rem") {
                    self.shell.remote = !rem;
                }
                if self.s_btn(ui2, x, &mut y, w, "monitor", "Live state", "Frame rate, latency, scene nodes", "Open", "set-live") {
                    self.shell.sys_open = true;
                }
                if self.s_btn(ui2, x, &mut y, w, "restart", "Restart the session", "Replays the boot sequence", "Restart\u{2026}", "set-restart") {
                    self.app_cmd("sys.restart");
                }
            }
            _ => {
                self.s_head(&p, x, &mut y, "About", "Smart Desktop \u{b7} version 1.0");
                let r = Rect::from_min_size(pos2(x, y), vec2(w, 150.0));
                elevate(&p, &th, r, 20.0, 2);
                grad_fill(&p, r, 20.0, th.card_top, th.card_bottom);
                rim(&p, r, 20.0, 1.3, th.rim, true, 40.0);
                let logo = Rect::from_min_size(r.min + vec2(22.0, 22.0), vec2(64.0, 64.0));
                let (a, b) = th.accent_grad();
                shadow(&p, logo, 20.0, 8.0, 18.0, -4.0, alpha(th.accent, 0.4));
                grad_fill(&p, logo, 20.0, a, b);
                rim(&p, logo, 20.0, 1.3, Color32::from_white_alpha(150), true, 40.0);
                p.text(logo.center() + vec2(0.0, 1.0), Align2::CENTER_CENTER, "S", f_bold(34.0), Color32::WHITE);
                p.text(pos2(logo.right() + 18.0, r.top() + 38.0), Align2::LEFT_CENTER, "Smart Desktop Pro", f_bold(20.0), th.ink1);
                p.text(pos2(logo.right() + 18.0, r.top() + 62.0), Align2::LEFT_CENTER, "$299 \u{b7} perpetual licence \u{b7} custom apps \u{b7} integrated AI \u{b7} remote sessions", f_medium(T_BODY), th.ink3);
                p.text(pos2(logo.right() + 18.0, r.top() + 88.0), Align2::LEFT_CENTER, "Licence  SMRT-\u{2022}\u{2022}\u{2022}\u{2022}-\u{2022}\u{2022}\u{2022}\u{2022}-4F2A  \u{b7}  Activated", f_mono(11.0), th.good);
                p.text(pos2(r.left() + 22.0, r.bottom() - 26.0), Align2::LEFT_CENTER, "Rust \u{b7} WebAssembly \u{b7} scene-graph compositor \u{b7} no GNU userland \u{b7} Smart Protocol contracts", f_mono(10.5), th.ink3);
                y += 166.0;
                for (ic, l, s) in [("shield-check", "Smart Protocol", "89 capability schemas: compositor, clipboard, capture, audio, ai\u{2026}"), ("laptop", "Portable by design", "Windows & macOS clients, FreeBSD hosts, zero-cost runtime"), ("code-square", "smart-shell", "The non-GNU shell that powers the terminal")] {
                    self.s_row(ui2, x, &mut y, w, ic, l, s, 0.0);
                }
            }
        }
        let content_h = y - y0 + 40.0;
        ui.ctx().data_mut(|d| d.insert_temp(hkey, content_h));
        let _ = AppId::Settings;
    }
}
