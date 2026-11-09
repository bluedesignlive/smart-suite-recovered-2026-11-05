//! Modal dialogs: Quick Look, Get Info, Open With, Devices, Transfer, Phone, Wallpaper, Tokens, About…
use crate::app::*;
use crate::chrome::*;
use crate::content::kind_color;
use crate::icons::Ic;
use crate::overlays::win_rect;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Context, FontId, Id, Order, Painter, Rect, Sense, Ui};
use smart_core::*;

const LAWS: [&str; 8] = [
    "No decorative strokes. Depth comes from tonal steps and the elevation ladder.",
    "Every tonal pair passes WCAG: 4.5 for text, 3.0 for large text and UI.",
    "Focus is always visible; removing strokes never removes focus.",
    "Five elevation levels only: canvas, window, raised, overlay, modal.",
    "Touch targets are at least 44 px on coarse pointers.",
    "Motion has a reduced-motion fallback; springs become fades.",
    "One smart.toml feeds every host. Spec-perfect, not pixel-perfect.",
    "Icons are Solar. Outline by default, duotone on hover, filled when active.",
];

const SHORTCUTS: [(&str, &str); 22] = [
    ("Ctrl K", "Command palette"),
    ("/  or  Ctrl F", "Search"),
    ("Enter", "Open"),
    ("Space", "Quick Look"),
    ("Ctrl I", "Get Info"),
    ("F2", "Rename"),
    ("Ctrl C / X / V", "Copy · cut · paste"),
    ("Ctrl D", "Duplicate"),
    ("Del", "Move to Trash"),
    ("Shift Del", "Delete permanently"),
    ("Ctrl A", "Select all"),
    ("Ctrl Shift N", "New folder"),
    ("Alt \u{2190} / \u{2192}", "Back · forward"),
    ("Backspace  or  Alt \u{2191}", "Enclosing folder"),
    ("Ctrl H", "Show hidden files"),
    ("Ctrl \\", "Toggle sidebar"),
    ("G / L", "Grid · list view"),
    ("D", "Light · dark"),
    ("T", "Design tokens"),
    ("F5", "Refresh"),
    ("Esc", "Close · clear search"),
    ("?", "This sheet"),
];

impl SmartApp {
    fn modal_head(&self, ui: &mut Ui, card: Rect, icon: &'static str, title: &str, sub: &str, close: bool) -> bool {
        let p = ui.painter().clone();
        let th = &self.th;
        let orb = Rect::from_min_size(card.min + vec2(22.0, 18.0), vec2(38.0, 38.0));
        let (a, b) = th.accent_grad();
        shadow(&p, orb, 19.0, 4.0, 12.0, -2.0, alpha(th.accent, 0.4));
        grad_fill(&p, orb, 19.0, a, b);
        rim(&p, orb, 19.0, 1.2, Color32::from_white_alpha(140), true, 22.0);
        self.ico(&p, icon, Ic::Filled, orb.center(), 19.0, th.on_accent());
        p.text(pos2(orb.right() + 13.0, orb.center().y - 8.0), Align2::LEFT_CENTER, self.fit(&p, title, &f_bold(T_TITLE), card.width() - 140.0), f_bold(T_TITLE), th.ink1);
        p.text(pos2(orb.right() + 13.0, orb.center().y + 9.0), Align2::LEFT_CENTER, self.fit(&p, sub, &f_medium(T_META), card.width() - 140.0), f_medium(T_META), th.ink3);
        if close {
            let c = Rect::from_center_size(pos2(card.right() - 30.0, orb.center().y), vec2(30.0, 30.0));
            let r = self.hit(ui, c, "modal-close", Sense::click());
            if r.hovered() {
                p.rect_filled(c, cr(15.0), th.row_hover);
            }
            self.ico(&p, "close-circle", if r.hovered() { Ic::Filled } else { Ic::Duo }, c.center(), 18.0, th.ink3);
            return r.clicked();
        }
        false
    }

    fn modal_foot_note(&self, p: &Painter, card: Rect, note: &str, max_w: f32) {
        let y = card.bottom() - 32.0;
        self.ico(p, "info-circle", Ic::Duo, pos2(card.left() + 30.0, y), 14.0, self.th.ink4);
        p.text(pos2(card.left() + 44.0, y), Align2::LEFT_CENTER, self.fit(p, note, &f_medium(T_META), max_w), f_medium(T_META), self.th.ink3);
    }

    /// Footer buttons laid out right-to-left. Returns true if clicked.
    fn foot_btn(&self, ui: &Ui, card: Rect, cursor: &mut f32, label: &str, kind: Btn, id: &str) -> bool {
        let w = self.tw(ui.painter(), label, &f_semi(T_BODY)) + 34.0;
        let r = Rect::from_min_size(pos2(*cursor - w, card.bottom() - 32.0 - 17.0), vec2(w, 34.0));
        *cursor -= w + 8.0;
        self.label_button(ui, r, None, label, id, kind).clicked()
    }

    pub fn draw_modal(&mut self, ctx: &Context) {
        if matches!(self.modal, Modal::None) {
            return;
        }
        let win = win_rect();
        let th = self.th.clone();
        let m = self.modal.clone();
        let (mut w, mut h): (f32, f32) = match &m {
            Modal::Preview(_) => ((win.width() - 80.0).min(960.0), (win.height() - 70.0).min(700.0)),
            Modal::Info(_) => (500.0, 560.0),
            Modal::OpenWith(_) => (480.0, 520.0),
            Modal::Wallpaper => (600.0, 470.0),
            Modal::About => (500.0, 590.0),
            Modal::Shortcuts => (640.0, 560.0),
            Modal::Confirm(_) => (440.0, 190.0),
            Modal::Devices => (680.0, 580.0),
            Modal::Transfer => (780.0, 540.0),
            Modal::Phone => (560.0, 560.0),
            Modal::Tokens => (680.0, 600.0),
            Modal::NewTag => (420.0, 190.0),
            Modal::Connect => (520.0, 330.0),
            Modal::None => (0.0, 0.0),
        };
        w = w.min(win.width() - 30.0);
        h = h.min(win.height() - 30.0);
        let mut close = false;
        let mut acts: Vec<Act> = vec![];
        egui::Area::new(Id::unique("modal")).order(Order::Foreground).fixed_pos(win.min).show(ctx, |ui| {
            let p = ui.painter().clone();
            ui.set_clip_rect(win);
            let scrim = ui.allocate_rect(win, Sense::click());
            grad_fill(&p, win, R_WIN, th.scrim, th.scrim);
            let card = Rect::from_center_size(win.center(), vec2(w, h));
            elevate(&p, &th, card, 22.0, 5);
            grad_fill(&p, card, 22.0, th.panel_top, th.panel_bottom);
            rim(&p, card, 22.0, 1.4, th.rim, true, 60.0);
            ui.allocate_rect(card, Sense::click());
            if scrim.clicked() && !matches!(m, Modal::Confirm(_)) {
                close = true;
            }
            match &m {
                Modal::Preview(l) => self.m_preview(ui, card, l, &mut close),
                Modal::Info(ls) => self.m_info(ui, card, ls, &mut close),
                Modal::OpenWith(l) => self.m_openwith(ui, card, l, &mut close),
                Modal::Wallpaper => self.m_wallpaper(ui, card, &mut close),
                Modal::About => self.m_about(ui, card, &mut close),
                Modal::Shortcuts => self.m_shortcuts(ui, card, &mut close),
                Modal::Confirm(c) => self.m_confirm(ui, card, c, &mut close, &mut acts),
                Modal::Devices => self.m_devices(ui, card, &mut close),
                Modal::Transfer => self.m_transfer(ui, card, &mut close),
                Modal::Phone => self.m_phone(ui, card, &mut close),
                Modal::Tokens => self.m_tokens(ui, card, &mut close),
                Modal::NewTag => self.m_newtag(ui, card, &mut close),
                Modal::Connect => self.m_connect(ui, card, &mut close),
                Modal::None => {}
            }
        });
        for a in acts {
            self.acts.push(a);
        }
        if close {
            self.acts.push(Act::CloseModal);
            if matches!(m, Modal::Preview(_)) {
                self.be.audio.stop();
                self.audio_loc = None;
            }
        }
    }

    // ───────────── Quick Look ─────────────
    fn m_preview(&mut self, ui: &mut Ui, card: Rect, l: &Location, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let Ok(e) = self.be.stat(l) else {
            *close = true;
            return;
        };
        let media = Rect::from_min_max(card.min + vec2(12.0, 12.0), pos2(card.right() - 12.0, card.bottom() - 82.0));
        // dark stage for media so artwork pops without strokes
        grad_fill(&p, media, 16.0, if th.dark { c(6, 10, 18, 0.9) } else { c(20, 28, 46, 0.10) }, if th.dark { c(10, 14, 24, 0.9) } else { c(20, 28, 46, 0.18) });
        rim(&p, media, 16.0, 3.0, alpha(th.shadow, if th.dark { 0.5 } else { 0.10 }), true, 16.0);
        match e.kind {
            Kind::Image => {
                let tex = self.big_thumbs.get(l).or_else(|| self.thumbs.get(l));
                if let Some(t) = tex {
                    let sz = t.tex.size_vec2();
                    let k = ((media.width() - 24.0) / sz.x).min((media.height() - 24.0) / sz.y);
                    let r = Rect::from_center_size(media.center(), sz * k.max(0.05));
                    elevate(&p, &th, r, 10.0, 3);
                    textured_rrect(&p, r, 10.0, t.tex.id(), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                } else {
                    p.text(media.center(), Align2::CENTER_CENTER, "Loading\u{2026}", f_medium(T_BODY), th.ink3);
                    ui.ctx().request_repaint();
                }
            }
            Kind::Audio => self.preview_audio(ui, media, &e),
            Kind::Text | Kind::Code => {
                let code = Rect::from_min_max(media.min + vec2(28.0, 28.0), media.max - vec2(28.0, 28.0));
                grad_fill(&p, code, 14.0, hex("#0e1625"), hex("#0a111d"));
                for (i, col) in [hex("#ff6b62"), hex("#f7c04a"), hex("#4fd17a")].into_iter().enumerate() {
                    p.circle_filled(code.min + vec2(20.0 + 16.0 * i as f32, 18.0), 5.0, col);
                }
                p.text(pos2(code.center().x, code.top() + 18.0), Align2::CENTER_CENTER, &e.name, f_medium(11.5), hex("#8fa0bd"));
                let txt = self.text_preview.as_ref().filter(|(x, _)| x == l).map(|(_, t)| t.clone()).unwrap_or_default();
                let g = p.layout(txt, FontId::monospace(12.5), hex("#d8e2f2"), code.width() - 40.0);
                let pp = p.with_clip_rect(Rect::from_min_max(code.min + vec2(0.0, 38.0), code.max));
                pp.galley(code.min + vec2(20.0, 50.0), g, hex("#d8e2f2"));
            }
            _ => {
                let kc = kind_color(e.kind, &th);
                let t = Rect::from_center_size(media.center() - vec2(0.0, 12.0), vec2(110.0, 110.0));
                grad_fill(&p, t, 32.0, alpha(kc, 0.28), alpha(kc, 0.12));
                rim(&p, t, 32.0, 1.2, Color32::from_white_alpha(60), true, 40.0);
                self.ico(&p, e.kind.icon(), Ic::Duo, t.center(), 52.0, kc);
                p.text(media.center() + vec2(0.0, 84.0), Align2::CENTER_CENTER, "No built-in preview for this kind of file", f_medium(T_BODY), th.ink3);
            }
        }
        // prev / next
        let list: Vec<Location> = self.items.iter().filter(|x| x.kind.previewable()).map(|x| x.loc.clone()).collect();
        if let Some(i) = list.iter().position(|x| x == l) {
            for (dir, icon, x) in [(-1i32, "alt-arrow-left", media.left() + 30.0), (1, "alt-arrow-right", media.right() - 30.0)] {
                let ok = (dir < 0 && i > 0) || (dir > 0 && i + 1 < list.len());
                if !ok {
                    continue;
                }
                let r = Rect::from_center_size(pos2(x, media.center().y), vec2(42.0, 42.0));
                if self.icon_button(ui, r, icon, if dir < 0 { "pv-prev" } else { "pv-next" }, Btn::Raised, true).clicked() {
                    self.acts.push(Act::Preview(list[(i as i32 + dir) as usize].clone()));
                }
            }
            let t = format!("{} / {}", i + 1, list.len());
            p.text(pos2(media.center().x, media.bottom() - 16.0), Align2::CENTER_CENTER, t, f_semi(T_META), alpha(th.ink2, 0.8));
        }
        let c = Rect::from_center_size(media.right_top() + vec2(-28.0, 28.0), vec2(34.0, 34.0));
        if self.icon_button(ui, c, "close-circle", "pv-close", Btn::Raised, true).clicked() {
            *close = true;
        }
        // info row
        let y = card.bottom() - 40.0;
        p.text(pos2(card.left() + 26.0, y - 8.0), Align2::LEFT_CENTER, self.fit(&p, &e.name, &f_bold(T_TITLE), card.width() - 320.0), f_bold(T_TITLE), th.ink1);
        let dims = self.thumbs.get(l).and_then(|t| t.orig).or_else(|| self.big_thumbs.get(l).and_then(|t| t.orig)).map(|(a, b)| format!("  \u{b7}  {} \u{d7} {}", a, b)).unwrap_or_default();
        let meta = format!("{}{}  \u{b7}  {}  \u{b7}  {}", e.kind_label(), dims, format_size(e.size), e.modified.map(format_date).unwrap_or_default());
        p.text(pos2(card.left() + 26.0, y + 12.0), Align2::LEFT_CENTER, meta, f_medium(T_META), th.ink3);
        let mut cur = card.right() - 22.0;
        let open_r = Rect::from_min_size(pos2(cur - 96.0, y - 17.0), vec2(96.0, 34.0));
        if self.label_button(ui, open_r, Some("arrow-right-up"), "Open", "pv-open", Btn::Accent).clicked() {
            self.be.open(l);
        }
        cur -= 96.0 + 8.0;
        let rev = Rect::from_min_size(pos2(cur - 104.0, y - 17.0), vec2(104.0, 34.0));
        if self.label_button(ui, rev, Some("folder"), "Reveal", "pv-reveal", Btn::Raised).clicked() {
            if let Some(par) = l.parent() {
                self.acts.push(Act::CloseModal);
                self.acts.push(Act::Nav(Route::Loc(par)));
                self.select_after_list = vec![l.clone()];
            }
        }
    }

    fn preview_audio(&mut self, ui: &mut Ui, media: Rect, e: &Entry) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let kc = th.warn;
        let c = media.center();
        let art = Rect::from_center_size(c - vec2(0.0, 92.0), vec2(120.0, 120.0));
        elevate(&p, &th, art, 30.0, 3);
        grad_fill(&p, art, 30.0, mix(kc, Color32::WHITE, 0.2), mix(kc, Color32::BLACK, 0.2));
        rim(&p, art, 30.0, 1.4, Color32::from_white_alpha(120), true, 50.0);
        self.ico(&p, "music-notes", Ic::Filled, art.center(), 54.0, Color32::from_white_alpha(235));
        let wf: Vec<f32> = self.waveform.as_ref().filter(|(l, _)| *l == e.loc).map(|(_, w)| w.clone()).unwrap_or_default();
        let wr = Rect::from_center_size(c + vec2(0.0, 36.0), vec2((media.width() - 120.0).min(640.0), 80.0));
        let playing_this = self.audio_loc.as_ref() == Some(&e.loc);
        let pos = if playing_this { self.be.audio.position() } else { 0.0 };
        let dur = if playing_this { self.be.audio.duration() } else { None }.or_else(|| Some((e.size as f32 / 176_400.0).max(1.0)));
        let frac = dur.map(|d| (pos / d).clamp(0.0, 1.0)).unwrap_or(0.0);
        let n = wf.len().max(1);
        let bw = wr.width() / n as f32;
        for (i, v) in wf.iter().enumerate() {
            let h = (wr.height() * v).max(3.0);
            let r = Rect::from_center_size(pos2(wr.left() + bw * (i as f32 + 0.5), wr.center().y), vec2((bw - 2.0).max(1.5), h));
            let played = (i as f32 / n as f32) < frac;
            grad_fill(&p, r, 1.5, if played { th.accent } else { alpha(th.ink2, 0.35) }, if played { th.accent_press } else { alpha(th.ink2, 0.2) });
        }
        let seek = self.hit(ui, wr, "wf-seek", Sense::click());
        if seek.clicked() && playing_this {
            if let (Some(pp), Some(d)) = (seek.interact_pointer_pos(), dur) {
                self.be.audio.seek(((pp.x - wr.left()) / wr.width()).clamp(0.0, 1.0) * d);
            }
        }
        let fmt = |s: f32| format!("{}:{:02}", (s as u32) / 60, (s as u32) % 60);
        p.text(pos2(wr.left(), wr.bottom() + 16.0), Align2::LEFT_CENTER, fmt(pos), f_semi(T_META), th.ink3);
        p.text(pos2(wr.right(), wr.bottom() + 16.0), Align2::RIGHT_CENTER, dur.map(fmt).unwrap_or_default(), f_semi(T_META), th.ink3);
        let btn = Rect::from_center_size(pos2(c.x, wr.bottom() + 58.0), vec2(56.0, 56.0));
        let playing = playing_this && self.be.audio.is_playing();
        let r = self.hit(ui, btn, "pv-play", Sense::click());
        raised_accent(&p, &th, btn, 28.0, r.hovered(), r.is_pointer_button_down_on());
        self.ico(&p, if playing { "pause" } else { "play" }, Ic::Filled, btn.center() + vec2(if playing { 0.0 } else { 1.5 }, 0.0), 24.0, th.on_accent());
        if r.clicked() {
            if playing {
                self.be.audio.pause();
            } else if playing_this && self.be.audio.is_loaded() {
                self.be.audio.resume();
            } else {
                self.acts.push(Act::Play(e.loc.clone()));
            }
        }
        if playing {
            ui.ctx().request_repaint();
        }
    }

    // ───────────── Get Info ─────────────
    fn m_info(&mut self, ui: &mut Ui, card: Rect, ls: &[Location], close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let ents: Vec<Entry> = ls.iter().filter_map(|l| self.be.stat(l).ok()).collect();
        let multi = ents.len() > 1;
        let title = if multi { format!("{} items", ents.len()) } else { ents.first().map(|e| e.name.clone()).unwrap_or_default() };
        let sub = if multi { "Multiple selection".to_string() } else { ents.first().map(|e| e.kind_label()).unwrap_or_default() };
        if self.modal_head(ui, card, "info-circle", &title, &sub, true) {
            *close = true;
        }
        let Some(first) = ents.first().cloned() else { return };
        // hero
        let hero = Rect::from_min_size(card.min + vec2(22.0, 76.0), vec2(card.width() - 44.0, 130.0));
        grad_fill(&p, hero, 16.0, alpha(kind_color(first.kind, &th), 0.14), alpha(kind_color(first.kind, &th), 0.06));
        if let (Some(t), false) = (self.thumbs.get(&first.loc), multi) {
            let sz = t.tex.size_vec2();
            let r = Rect::from_center_size(hero.center(), vec2(150.0, 100.0));
            elevate(&p, &th, r, 12.0, 2);
            textured_rrect(&p, r, 12.0, t.tex.id(), cover_uv(sz.x, sz.y, r), Color32::WHITE);
        } else {
            let kc = kind_color(first.kind, &th);
            let t = Rect::from_center_size(hero.center(), vec2(84.0, 84.0));
            grad_fill(&p, t, 26.0, alpha(kc, 0.26), alpha(kc, 0.12));
            rim(&p, t, 26.0, 1.2, Color32::from_white_alpha(if th.dark { 40 } else { 160 }), true, 30.0);
            self.ico(&p, if multi { "copy" } else { first.kind.icon() }, if first.is_dir { Ic::Filled } else { Ic::Duo }, t.center(), 40.0, kc);
        }
        let total: u64 = ents.iter().map(|e| e.size).sum();
        let mut rows: Vec<(&str, String)> = vec![];
        if multi {
            rows.push(("Total size", format_size(total)));
            rows.push(("Folders", ents.iter().filter(|e| e.is_dir).count().to_string()));
            rows.push(("Files", ents.iter().filter(|e| !e.is_dir).count().to_string()));
        } else {
            rows.push(("Kind", first.kind_label()));
            if first.is_dir {
                let s = self.summaries.get(&first.loc).map(|(n, b)| format!("{} item{} \u{b7} {}", n, if *n == 1 { "" } else { "s" }, format_size(*b))).unwrap_or("Calculating\u{2026}".into());
                rows.push(("Contains", s));
            } else {
                rows.push(("Size", format!("{}  ({} bytes)", format_size(first.size), first.size)));
                if let Some(t) = self.thumbs.get(&first.loc).and_then(|t| t.orig) {
                    rows.push(("Dimensions", format!("{} \u{d7} {} px", t.0, t.1)));
                }
            }
            rows.push(("Where", self.pretty_path(&first.loc.parent().unwrap_or(first.loc.clone()))));
            if let Some(m) = first.modified {
                rows.push(("Modified", format_full_date(m)));
            }
            rows.push(("Access", if first.readonly { "Read only".into() } else { "Read & write".into() }));
            if let Some(real) = self.be.registry.get(&first.loc.fs).and_then(|v| v.real_path(&first.loc.path)) {
                rows.push(("Path", real.to_string_lossy().to_string()));
            }
        }
        let mut y = hero.bottom() + 16.0;
        for (k, v) in rows {
            p.text(pos2(card.left() + 28.0, y + 10.0), Align2::LEFT_CENTER, k, f_medium(T_META), th.ink3);
            p.text(pos2(card.left() + 136.0, y + 10.0), Align2::LEFT_CENTER, self.fit(&p, &v, &f_semi(T_BODY), card.width() - 170.0), f_semi(T_BODY), th.ink1);
            y += 28.0;
        }
        // tags
        y += 8.0;
        p.text(pos2(card.left() + 28.0, y + 12.0), Align2::LEFT_CENTER, "Tags", f_medium(T_META), th.ink3);
        let mut x = card.left() + 136.0;
        for t in self.lib(|l| l.tags.clone()) {
            let on = ents.iter().all(|e| self.lib(|l| l.has_tag(&e.loc, &t.id)));
            let f = f_semi(11.5);
            let wd = self.tw(&p, &t.name, &f) + 38.0;
            let r = Rect::from_min_size(pos2(x, y), vec2(wd, 26.0));
            let resp = self.hit(ui, r, ("info-tag", &t.id), Sense::click());
            let col = hex(&t.color);
            if on {
                grad_fill(&p, r, 13.0, alpha(col, 0.28), alpha(col, 0.16));
            } else {
                raised(&p, &th, r, 13.0, resp.hovered(), false);
            }
            grad_fill(&p, Rect::from_center_size(pos2(r.left() + 14.0, r.center().y), vec2(8.0, 8.0)), 4.0, mix(col, Color32::WHITE, 0.3), col);
            p.text(pos2(r.left() + 26.0, r.center().y), Align2::LEFT_CENTER, &t.name, f, th.ink1);
            if resp.clicked() {
                for e in &ents {
                    let has = self.lib(|l| l.has_tag(&e.loc, &t.id));
                    if has == on {
                        self.be.library.write().unwrap().toggle_tag(&e.loc, &t.id);
                    }
                }
            }
            x += wd + 6.0;
        }
        self.modal_foot_note(&p, card, "Tags and metadata travel with the file.", 260.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Done", Btn::Raised, "info-done") {
            *close = true;
        }
        if !multi && !first.is_dir && self.foot_btn(ui, card, &mut cur, "Open", Btn::Accent, "info-open") {
            self.be.open(&first.loc);
        }
    }

    // ───────────── Open With ─────────────
    fn m_openwith(&mut self, ui: &mut Ui, card: Rect, l: &Location, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "share", "Open with\u{2026}", &l.name(), true) {
            *close = true;
        }
        let apps = self.be.apps_for(l);
        let mut y = card.top() + 84.0;
        let row = |app: &Self, ui: &mut Ui, y: f32, icon: &'static str, name: &str, sub: &str, id: String, is_default: bool| -> (bool, bool) {
            let r = Rect::from_min_size(pos2(card.left() + 16.0, y), vec2(card.width() - 32.0, 52.0));
            let resp = app.hit(ui, r, ("owrow", &id), Sense::click());
            let p = ui.painter();
            if resp.hovered() {
                grad_fill(p, r, 14.0, app.th.row_press, app.th.row_hover);
            }
            let ic = Rect::from_min_size(r.min + vec2(10.0, 8.0), vec2(36.0, 36.0));
            grad_fill(p, ic, 11.0, alpha(app.th.accent, 0.16), alpha(app.th.accent, 0.08));
            rim(p, ic, 11.0, 1.0, Color32::from_white_alpha(if app.th.dark { 30 } else { 150 }), true, 14.0);
            app.ico(p, icon, Ic::Duo, ic.center(), 18.0, app.th.accent_ink);
            p.text(pos2(ic.right() + 12.0, r.center().y - 8.0), Align2::LEFT_CENTER, name, f_semi(T_BODY), app.th.ink1);
            p.text(pos2(ic.right() + 12.0, r.center().y + 9.0), Align2::LEFT_CENTER, sub, f_medium(T_META), app.th.ink3);
            let mut always = false;
            if resp.hovered() && !id.starts_with("default") {
                let b = Rect::from_center_size(pos2(r.right() - 56.0, r.center().y), vec2(92.0, 28.0));
                always = app.label_button(ui, b, None, "Always", &format!("al-{}", id), Btn::Raised).clicked();
            }
            if is_default {
                let b = Rect::from_center_size(pos2(r.right() - 36.0, r.center().y), vec2(56.0, 22.0));
                grad_fill(ui.painter(), b, 11.0, alpha(app.th.good, 0.2), alpha(app.th.good, 0.12));
                ui.painter().text(b.center(), Align2::CENTER_CENTER, "Default", f_semi(10.5), app.th.good);
            }
            (resp.clicked() && !always, always)
        };
        let _ = &p;
        let (clicked, _) = row(self, ui, y, "arrow-right-up", "System default", "Opens with whatever your system prefers", "default-sys".into(), false);
        if clicked {
            self.be.open(l);
            *close = true;
        }
        y += 56.0;
        if apps.is_empty() {
            p.text(pos2(card.center().x, y + 60.0), Align2::CENTER_CENTER, "No other applications are registered for this type", f_medium(T_BODY), th.ink3);
        }
        for a in apps.iter().take(6) {
            let (c, always) = row(self, ui, y, a.icon, &a.name, if a.is_default { "Current default for this type" } else { "Application" }, a.id.clone(), a.is_default);
            if c {
                self.be.open_with(l, a);
                *close = true;
            }
            if always {
                self.acts.push(Act::SetDefault(l.clone(), a.clone()));
                self.be.open_with(l, a);
                *close = true;
            }
            y += 56.0;
        }
        self.modal_foot_note(&p, card, "Nothing leaves this device until you send it.", 300.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Done", Btn::Raised, "ow-done") {
            *close = true;
        }
    }

    // ───────────── Wallpaper / About / Shortcuts / Tokens ─────────────
    fn m_wallpaper(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "gallery-wide", "Desktop wallpaper", "The surface behind the Smart window", true) {
            *close = true;
        }
        let cur = self.st().wallpaper;
        let gw = (card.width() - 44.0 - 16.0) / 2.0;
        let gh = (card.height() - 76.0 - 76.0 - 16.0) / 2.0;
        for (i, (name, _, _)) in WALLPAPERS.iter().enumerate() {
            let r = Rect::from_min_size(card.min + vec2(22.0 + (i % 2) as f32 * (gw + 16.0), 78.0 + (i / 2) as f32 * (gh + 16.0)), vec2(gw, gh));
            let resp = self.hit(ui, r, ("wall", i), Sense::click());
            elevate(&p, &th, r, 16.0, if resp.hovered() { 3 } else { 2 });
            let (tex, _, sz) = &self.walls[i];
            textured_rrect(&p, r, 16.0, tex.id(), cover_uv(sz[0], sz[1], r), Color32::WHITE);
            let lab = Rect::from_min_max(pos2(r.left() + 10.0, r.bottom() - 36.0), pos2(r.left() + 20.0 + self.tw(&p, name, &f_semi(11.5)) + 20.0, r.bottom() - 10.0));
            grad_fill(&p, lab, 13.0, c(10, 16, 28, 0.55), c(10, 16, 28, 0.45));
            p.text(lab.center(), Align2::CENTER_CENTER, *name, f_semi(11.5), Color32::WHITE);
            if cur == i {
                p.rect_stroke(r.expand(3.0), cr(19.0), egui::Stroke::new(2.0, th.accent), egui::StrokeKind::Inside);
                self.ico(&p, "check-circle", Ic::Filled, r.right_top() + vec2(-22.0, 22.0), 22.0, Color32::WHITE);
            }
            if resp.clicked() {
                self.acts.push(Act::SetWallpaper(i));
            }
        }
        self.modal_foot_note(&p, card, "The palette stays anchored to cobalt \u{2014} only the surface changes.", 380.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Done", Btn::Raised, "wp-done") {
            *close = true;
        }
    }

    fn m_about(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "stars-minimalistic", "Smart Files", "0.1 \u{b7} built on smart-core + egui", true) {
            *close = true;
        }
        let hero = Rect::from_min_size(card.min + vec2(22.0, 78.0), vec2(card.width() - 44.0, 64.0));
        grad_fill(&p, hero, 16.0, th.card_top, th.card_bottom);
        elevate(&p, &th, hero, 16.0, 1);
        grad_fill(&p, hero, 16.0, th.card_top, th.card_bottom);
        rim(&p, hero, 16.0, 1.0, th.rim, true, 30.0);
        let mark = Rect::from_min_size(hero.min + vec2(14.0, 12.0), vec2(40.0, 40.0));
        let (a, b) = th.accent_grad();
        grad_fill(&p, mark, 13.0, a, b);
        rim(&p, mark, 13.0, 1.2, Color32::from_white_alpha(130), true, 20.0);
        p.text(mark.center(), Align2::CENTER_CENTER, self.user_name().chars().next().unwrap_or('N').to_string(), f_bold(17.0), Color32::WHITE);
        p.text(pos2(mark.right() + 12.0, mark.center().y - 8.0), Align2::LEFT_CENTER, format!("{} workspace", self.user_name()), f_semi(T_BODY), th.ink1);
        let dev = self.current_device().map(|d| format!("Personal \u{b7} {}", d.capacity_text())).unwrap_or_default();
        p.text(pos2(mark.right() + 12.0, mark.center().y + 9.0), Align2::LEFT_CENTER, dev, f_medium(T_META), th.ink3);
        let mut y = hero.bottom() + 18.0;
        p.text(pos2(card.left() + 28.0, y), Align2::LEFT_CENTER, "THE LAWS", f_semi(T_LABEL), th.ink4);
        y += 14.0;
        for (i, l) in LAWS.iter().enumerate() {
            let g = p.layout(l.to_string(), f_medium(12.0), th.ink2, card.width() - 100.0);
            let h = g.size().y;
            let badge = Rect::from_center_size(pos2(card.left() + 40.0, y + 12.0), vec2(20.0, 20.0));
            grad_fill(&p, badge, 10.0, alpha(th.accent, 0.18), alpha(th.accent, 0.08));
            p.text(badge.center(), Align2::CENTER_CENTER, (i + 1).to_string(), f_bold(10.5), th.accent_ink);
            p.galley(pos2(card.left() + 60.0, y + 4.0), g, th.ink2);
            y += h.max(20.0) + 10.0;
        }
        self.modal_foot_note(&p, card, "Smart Foundation \u{b7} design contract v0.1", 320.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Close", Btn::Raised, "about-close") {
            *close = true;
        }
    }

    fn m_shortcuts(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "widget", "Keyboard shortcuts", "Everything here is reachable without a mouse", true) {
            *close = true;
        }
        let colw = (card.width() - 56.0) / 2.0;
        for (i, (k, d)) in SHORTCUTS.iter().enumerate() {
            let col = i / 11;
            let row = i % 11;
            let x = card.left() + 28.0 + col as f32 * colw;
            let y = card.top() + 90.0 + row as f32 * 36.0;
            p.text(pos2(x, y + 12.0), Align2::LEFT_CENTER, *d, f_medium(T_BODY), th.ink2);
            let kw = self.tw(&p, k, &f_semi(10.5)) + 12.0;
            let r = Rect::from_min_size(pos2(x + colw - 24.0 - kw, y + 2.0), vec2(kw, 22.0));
            well(&p, &th, r, 7.0);
            p.text(r.center(), Align2::CENTER_CENTER, *k, f_semi(10.5), th.ink2);
        }
        self.modal_foot_note(&p, card, "Shortcuts follow the same language as the command palette.", 380.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Close", Btn::Raised, "sc-close") {
            *close = true;
        }
    }

    fn m_tokens(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "widget", "Design tokens", "smart.toml \u{b7} draft 0.1 \u{b7} one contract, every host", true) {
            *close = true;
        }
        let code = Rect::from_min_max(card.min + vec2(22.0, 78.0), pos2(card.right() - 22.0, card.bottom() - 74.0));
        grad_fill(&p, code, 16.0, hex("#0e1625"), hex("#0a111d"));
        for (i, col) in [hex("#ff6b62"), hex("#f7c04a"), hex("#4fd17a")].into_iter().enumerate() {
            p.circle_filled(code.min + vec2(20.0 + 16.0 * i as f32, 20.0), 5.0, col);
        }
        p.text(pos2(code.center().x, code.top() + 20.0), Align2::CENTER_CENTER, "design/smart.toml", f_medium(11.5), hex("#8fa0bd"));
        let cb = Rect::from_center_size(pos2(code.right() - 52.0, code.top() + 20.0), vec2(78.0, 24.0));
        if self.label_button(ui, cb, Some("copy"), "Copy", "tok-copy", Btn::Raised).clicked() {
            self.acts.push(Act::CopyTokens);
        }
        let body = Rect::from_min_max(code.min + vec2(0.0, 42.0), code.max);
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(body).layout(egui::Layout::top_down(egui::Align::Min)));
        cui.set_clip_rect(body.intersect(ui.clip_rect()));
        egui::ScrollArea::vertical().id_salt("tok-scroll").auto_shrink([false, false]).show(&mut cui, |ui| {
            let g = ui.painter().layout(include_str!("../../../assets/tokens.toml").to_string(), FontId::monospace(11.5), hex("#c9d6ee"), body.width() - 40.0);
            let r = Rect::from_min_size(pos2(body.left() + 20.0, ui.max_rect().top() + 6.0), g.size());
            ui.allocate_rect(r.expand2(vec2(0.0, 14.0)), Sense::hover());
            ui.painter().galley(r.min, g, hex("#c9d6ee"));
        });
        self.modal_foot_note(&p, card, "These are the numbers behind this screen.", 380.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Close", Btn::Raised, "tok-close") {
            *close = true;
        }
    }

    fn m_confirm(&mut self, ui: &mut Ui, card: Rect, cf: &Confirm, close: &mut bool, acts: &mut Vec<Act>) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let orb = Rect::from_min_size(card.min + vec2(22.0, 22.0), vec2(40.0, 40.0));
        let col = if cf.danger { th.danger } else { th.accent };
        grad_fill(&p, orb, 20.0, alpha(col, 0.22), alpha(col, 0.12));
        rim(&p, orb, 20.0, 1.2, Color32::from_white_alpha(if th.dark { 40 } else { 160 }), true, 20.0);
        self.ico(&p, if cf.danger { "trash-bin-minimalistic" } else { "info-circle" }, Ic::Filled, orb.center(), 20.0, col);
        p.text(pos2(orb.right() + 14.0, orb.center().y - 9.0), Align2::LEFT_CENTER, self.fit(&p, &cf.title, &f_bold(T_TITLE), card.width() - 100.0), f_bold(T_TITLE), th.ink1);
        p.text(pos2(orb.right() + 14.0, orb.center().y + 11.0), Align2::LEFT_CENTER, self.fit(&p, &cf.body, &f_medium(T_META), card.width() - 100.0), f_medium(T_META), th.ink3);
        let mut cur = card.right() - 22.0;
        let w = self.tw(&p, &cf.ok, &f_semi(T_BODY)) + 34.0;
        let r = Rect::from_min_size(pos2(cur - w, card.bottom() - 54.0), vec2(w, 36.0));
        cur -= w + 8.0;
        let resp = self.hit(ui, r, "cf-ok", Sense::click());
        if cf.danger {
            let (a, b) = if th.dark { (hex("#ff8a93"), hex("#e5525e")) } else { (hex("#dc5560"), hex("#bd3441")) };
            shadow(&p, r, 18.0, 5.0, 14.0, -4.0, alpha(th.danger, 0.4));
            grad_fill(&p, r, 18.0, if resp.hovered() { mix(a, Color32::WHITE, 0.1) } else { a }, b);
            rim(&p, r, 18.0, 1.3, Color32::from_white_alpha(140), true, 20.0);
            p.text(r.center(), Align2::CENTER_CENTER, &cf.ok, f_semi(T_BODY), Color32::WHITE);
        } else {
            self.label_button(ui, r, None, &cf.ok, "cf-ok-draw", Btn::Accent);
        }
        if resp.clicked() {
            acts.push((*cf.act).clone());
            *close = false;
            acts.push(Act::CloseModal);
        }
        if self.foot_btn_at(ui, cur, card.bottom() - 54.0, "Cancel", "cf-cancel") {
            *close = true;
        }
    }

    fn foot_btn_at(&self, ui: &Ui, cursor: f32, y: f32, label: &str, id: &str) -> bool {
        let w = self.tw(ui.painter(), label, &f_semi(T_BODY)) + 34.0;
        let r = Rect::from_min_size(pos2(cursor - w, y), vec2(w, 36.0));
        self.label_button(ui, r, None, label, id, Btn::Raised).clicked()
    }

    fn m_newtag(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "tag", "New tag", "Colour-coded, travels with your files", true) {
            *close = true;
        }
        let f = Rect::from_min_size(card.min + vec2(22.0, 78.0), vec2(card.width() - 44.0, 40.0));
        well(&p, &th, f, 20.0);
        let mut t = std::mem::take(&mut self.new_tag_name);
        let r = ui.put(f.shrink2(vec2(16.0, 6.0)), egui::TextEdit::singleline(&mut t).id(Id::unique("newtag")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("Tag name").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
        if !r.has_focus() && t.is_empty() {
            r.request_focus();
        }
        let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        self.new_tag_name = t;
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Create tag", Btn::Accent, "nt-ok") || enter {
            self.acts.push(Act::NewTag);
        }
        if self.foot_btn(ui, card, &mut cur, "Cancel", Btn::Raised, "nt-cancel") {
            *close = true;
        }
    }

    fn m_connect(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        if self.modal_head(ui, card, "cloud", "Connect to server", "Network shares, cloud drives and phones", true) {
            *close = true;
        }
        let f = Rect::from_min_size(card.min + vec2(22.0, 80.0), vec2(card.width() - 44.0, 40.0));
        well(&p, &th, f, 20.0);
        self.ico(&p, "global", Ic::Duo, pos2(f.left() + 22.0, f.center().y), 16.0, th.ink3);
        let mut t = std::mem::take(&mut self.connect_url);
        let r = ui.put(Rect::from_min_max(f.min + vec2(40.0, 6.0), f.max - vec2(16.0, 6.0)), egui::TextEdit::singleline(&mut t).id(Id::unique("connect")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("smb://nas.local/share").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
        let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        self.connect_url = t.clone();
        p.text(pos2(card.left() + 28.0, f.bottom() + 24.0), Align2::LEFT_CENTER, "RECENT PROTOCOLS", f_semi(T_LABEL), th.ink4);
        let chips = ["smb://", "sftp://", "dav://", "davs://", "ftp://", "mtp://", "nfs://"];
        let mut x = card.left() + 22.0;
        for ch in chips {
            let wd = self.tw(&p, ch, &f_semi(11.5)) + 26.0;
            let rr = Rect::from_min_size(pos2(x, f.bottom() + 40.0), vec2(wd, 28.0));
            if self.label_button(ui, rr, None, ch, ch, Btn::Raised).clicked() {
                self.connect_url = ch.to_string();
            }
            x += wd + 8.0;
        }
        p.text(pos2(card.left() + 28.0, f.bottom() + 100.0), Align2::LEFT_TOP, "On Linux this uses GVfs, so shares appear in every app. Phones connected over USB (MTP/ADB) show up under Locations automatically.", f_medium(T_META), th.ink3);
        self.modal_foot_note(&p, card, "Credentials stay in your system keyring.", 300.0);
        let mut cur = card.right() - 22.0;
        if (self.foot_btn(ui, card, &mut cur, "Connect", Btn::Accent, "cn-ok") || enter) && !t.trim().is_empty() {
            self.acts.push(Act::Connect(t.trim().to_string()));
        }
        if self.foot_btn(ui, card, &mut cur, "Cancel", Btn::Raised, "cn-cancel") {
            *close = true;
        }
    }

    // ───────────── Devices ─────────────
    fn m_devices(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let mounted = self.devices.iter().filter(|d| d.mounted).count();
        if self.modal_head(ui, card, "ssd-square", "Devices & locations", &format!("{} mounted \u{b7} {} known \u{b7} mount, eject, and open storage", mounted, self.devices.len()), true) {
            *close = true;
        }
        let list = Rect::from_min_max(card.min + vec2(16.0, 76.0), pos2(card.right() - 16.0, card.bottom() - 74.0));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(list).layout(egui::Layout::top_down(egui::Align::Min)));
        cui.set_clip_rect(list.intersect(ui.clip_rect()));
        let devs = self.devices.clone();
        egui::ScrollArea::vertical().id_salt("dev-scroll").auto_shrink([false, false]).show(&mut cui, |ui| {
            let top = ui.max_rect().top();
            ui.allocate_rect(Rect::from_min_size(pos2(list.left(), top), vec2(list.width(), devs.len() as f32 * 88.0 + 8.0)), Sense::hover());
            for (i, d) in devs.iter().enumerate() {
                let r = Rect::from_min_size(pos2(list.left(), top + i as f32 * 88.0), vec2(list.width(), 80.0));
                elevate(&p, &th, r, 16.0, 1);
                grad_fill(&p, r, 16.0, th.card_top, th.card_bottom);
                rim(&p, r, 16.0, 1.2, th.rim, true, 30.0);
                let orb = Rect::from_min_size(r.min + vec2(14.0, 16.0), vec2(48.0, 48.0));
                let kc = if d.mounted { th.accent } else { th.ink4 };
                grad_fill(&p, orb, 24.0, alpha(kc, 0.22), alpha(kc, 0.10));
                rim(&p, orb, 24.0, 1.0, Color32::from_white_alpha(if th.dark { 30 } else { 150 }), true, 22.0);
                self.ico(&p, d.kind.icon(), Ic::Filled, orb.center(), 22.0, kc);
                p.text(pos2(orb.right() + 14.0, r.top() + 22.0), Align2::LEFT_CENTER, &d.name, f_semi(T_BODY), th.ink1);
                let status = if d.mounted { "Mounted" } else { "Not mounted" };
                p.text(pos2(orb.right() + 14.0 + self.tw(&p, &d.name, &f_semi(T_BODY)) + 10.0, r.top() + 22.0), Align2::LEFT_CENTER, format!("{} \u{b7} {}", d.kind.label(), status), f_medium(T_META), if d.mounted { th.good } else { th.ink4 });
                p.text(pos2(orb.right() + 14.0, r.top() + 41.0), Align2::LEFT_CENTER, self.fit(&p, &format!("{} \u{b7} {}", d.detail, d.capacity_text()), &f_medium(T_META), 300.0), f_medium(T_META), th.ink3);
                if d.total > 0 {
                    let bar = Rect::from_min_size(pos2(orb.right() + 14.0, r.top() + 54.0), vec2(220.0, 5.0));
                    well(&p, &th, bar, 2.5);
                    let (a, b) = th.accent_grad();
                    grad_fill(&p, Rect::from_min_size(bar.min, vec2((bar.width() * d.used_fraction()).max(5.0), 5.0)), 2.5, a, b);
                }
                let mut x = r.right() - 14.0;
                let mut btn = |app: &mut Self, ui: &mut Ui, label: &str, kind: Btn, id: String| -> bool {
                    let w = app.tw(ui.painter(), label, &f_semi(T_BODY)) + 30.0;
                    let b = Rect::from_min_size(pos2(x - w, r.center().y - 16.0), vec2(w, 32.0));
                    x -= w + 8.0;
                    app.label_button(ui, b, None, label, &id, kind).clicked()
                };
                if d.mounted {
                    if let Some(l) = &d.loc {
                        if d.kind == DeviceKind::Phone {
                            if btn(self, ui, "Continuity", Btn::Accent, format!("dv-ph-{}", i)) {
                                self.acts.push(Act::Modal(Modal::Phone));
                            }
                        } else if btn(self, ui, "Open", Btn::Accent, format!("dv-open-{}", i)) {
                            self.acts.push(Act::Nav(Route::Loc(l.clone())));
                        }
                    }
                    if d.can_eject && btn(self, ui, "Eject", Btn::Raised, format!("dv-eject-{}", i)) {
                        self.acts.push(Act::Eject(d.id.clone()));
                    }
                    if d.can_unmount && !d.can_eject && btn(self, ui, "Unmount", Btn::Raised, format!("dv-un-{}", i)) {
                        self.acts.push(Act::Unmount(d.id.clone()));
                    }
                    if d.can_unmount && d.can_eject && btn(self, ui, "Unmount", Btn::Raised, format!("dv-un-{}", i)) {
                        self.acts.push(Act::Unmount(d.id.clone()));
                    }
                } else if btn(self, ui, "Mount", Btn::Accent, format!("dv-mount-{}", i)) {
                    self.acts.push(Act::Mount(d.id.clone()));
                }
            }
        });
        self.modal_foot_note(&p, card, "Mounted devices stay available until you eject them.", 300.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Done", Btn::Raised, "dv-done") {
            *close = true;
        }
        if self.foot_btn(ui, card, &mut cur, "Connect to server\u{2026}", Btn::Raised, "dv-connect") {
            self.acts.push(Act::Modal(Modal::Connect));
        }
    }

    // ───────────── Phone continuity ─────────────
    fn m_phone(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let phone = self.devices.iter().find(|d| d.kind == DeviceKind::Phone).cloned();
        let name = phone.as_ref().map(|d| d.name.clone()).unwrap_or_else(|| "No phone connected".into());
        let sub = phone.as_ref().map(|d| format!("Continuity \u{b7} {}", d.detail)).unwrap_or_else(|| "Connect an Android phone with USB debugging, or pair over Wi-Fi".into());
        if self.modal_head(ui, card, "smartphone", &name, &sub, true) {
            *close = true;
        }
        let Some(d) = phone else {
            p.text(card.center(), Align2::CENTER_CENTER, "Plug in a phone and enable USB debugging (adb).\nIt will appear here and under Locations.", f_medium(T_BODY), th.ink3);
            return;
        };
        let st = Rect::from_min_size(card.min + vec2(22.0, 80.0), vec2(card.width() - 44.0, 84.0));
        elevate(&p, &th, st, 16.0, 1);
        grad_fill(&p, st, 16.0, th.card_top, th.card_bottom);
        rim(&p, st, 16.0, 1.2, th.rim, true, 30.0);
        let bat = d.battery.unwrap_or(0) as f32 / 100.0;
        self.ico(&p, "battery-charge-minimalistic", Ic::Duo, pos2(st.left() + 28.0, st.top() + 28.0), 20.0, th.good);
        p.text(pos2(st.left() + 48.0, st.top() + 28.0), Align2::LEFT_CENTER, format!("{}% battery", d.battery.unwrap_or(0)), f_semi(T_BODY), th.ink1);
        p.text(pos2(st.right() - 18.0, st.top() + 28.0), Align2::RIGHT_CENTER, "Connected", f_semi(T_META), th.good);
        let bar = Rect::from_min_size(pos2(st.left() + 20.0, st.top() + 48.0), vec2(st.width() - 40.0, 8.0));
        well(&p, &th, bar, 4.0);
        grad_fill(&p, Rect::from_min_size(bar.min, vec2((bar.width() * bat).max(8.0), 8.0)), 4.0, mix(th.good, Color32::WHITE, 0.25), th.good);
        p.text(pos2(st.left() + 20.0, st.bottom() - 14.0), Align2::LEFT_CENTER, "Hand-offs happen over your local network \u{2014} never through a server.", f_medium(T_META), th.ink3);
        // folders
        let id = Id::unique("phone-folders");
        let folders: Vec<Entry> = ui.data_mut(|dd| dd.get_temp::<Vec<Entry>>(id)).unwrap_or_else(|| {
            let l = d.loc.clone().unwrap_or(Location::new("phone", "/"));
            let root = if l.fs.starts_with("adb:") { l.clone() } else { Location::new(l.fs.clone(), "/") };
            let v = self.be.registry.get(&root.fs).and_then(|v| v.list(&root.path).ok()).unwrap_or_default();
            let v: Vec<Entry> = v.into_iter().filter(|e| e.is_dir).collect();
            ui.data_mut(|dd| dd.insert_temp(id, v.clone()));
            v
        });
        p.text(pos2(card.left() + 28.0, st.bottom() + 24.0), Align2::LEFT_CENTER, "BROWSE", f_semi(T_LABEL), th.ink4);
        let mut y = st.bottom() + 40.0;
        for e in folders.iter().take(5) {
            let r = Rect::from_min_size(pos2(card.left() + 16.0, y), vec2(card.width() - 32.0, 44.0));
            let resp = self.hit(ui, r, ("ph-f", &e.loc), Sense::click());
            if resp.hovered() {
                grad_fill(&p, r, 13.0, th.row_press, th.row_hover);
            }
            self.ico(&p, "folder", Ic::Filled, pos2(r.left() + 28.0, r.center().y), 20.0, th.accent);
            p.text(pos2(r.left() + 50.0, r.center().y), Align2::LEFT_CENTER, &e.name, f_semi(T_BODY), th.ink1);
            p.text(pos2(r.right() - 16.0, r.center().y), Align2::RIGHT_CENTER, "Open", f_semi(T_META), th.accent_ink);
            if resp.clicked() {
                self.acts.push(Act::Nav(Route::Loc(e.loc.clone())));
                self.acts.push(Act::CloseModal);
            }
            y += 46.0;
        }
        // send selection
        let sel = self.selected();
        let note = if sel.is_empty() { "Select files in the main window to send them to your phone.".to_string() } else { format!("{} selected \u{b7} ready to send", sel.len()) };
        self.modal_foot_note(&p, card, &note, 300.0);
        let mut cur = card.right() - 22.0;
        if self.foot_btn(ui, card, &mut cur, "Done", Btn::Raised, "ph-done") {
            *close = true;
        }
        if !sel.is_empty() {
            let dest = folders.iter().find(|e| e.name == "Download").map(|e| e.loc.clone()).or_else(|| folders.first().map(|e| e.loc.clone()));
            if let Some(dest) = dest {
                if self.foot_btn(ui, card, &mut cur, "Send to phone", Btn::Accent, "ph-send") {
                    let srcs = sel.iter().map(|e| e.loc.clone()).collect();
                    self.be.submit(JobKind::Copy { srcs, dest });
                    self.acts.push(Act::CloseModal);
                }
            }
        }
    }

    // ───────────── Copy / Move to… ─────────────
    fn m_transfer(&mut self, ui: &mut Ui, card: Rect, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let Some(t) = self.transfer.as_ref() else {
            *close = true;
            return;
        };
        let (mv, srcs, dest, folders) = (t.mv, t.srcs.clone(), t.dest.clone(), t.folders.clone());
        let title = if mv { "Move to\u{2026}" } else { "Copy to\u{2026}" };
        let sub = if srcs.len() == 1 { format!("Choose where \u{201c}{}\u{201d} should land", srcs[0].name()) } else { format!("Choose where {} items should land", srcs.len()) };
        if self.modal_head(ui, card, if mv { "folder" } else { "copy" }, title, &sub, true) {
            *close = true;
        }
        let body = Rect::from_min_max(card.min + vec2(16.0, 76.0), pos2(card.right() - 16.0, card.bottom() - 74.0));
        // rail
        let rail = Rect::from_min_size(body.min, vec2(176.0, body.height()));
        grad_fill(&p, rail, 14.0, th.well, alpha(th.well, 0.5));
        let mut y = rail.top() + 8.0;
        let mut go: Option<Location> = None;
        let mut entries: Vec<(&'static str, String, Location)> = self.places.iter().map(|pl| (pl.icon, pl.title.clone(), pl.loc.clone())).collect();
        for d in &self.devices {
            if let (Some(l), true) = (&d.loc, d.mounted) {
                if d.kind != DeviceKind::Phone || true {
                    entries.push((d.kind.icon(), d.name.clone(), l.clone()));
                }
            }
        }
        for (ic, name, l) in entries {
            let r = Rect::from_min_size(pos2(rail.left() + 6.0, y), vec2(rail.width() - 12.0, 32.0));
            let on = dest == l;
            let resp = self.hit(ui, r, ("tr-rail", &name), Sense::click());
            if on {
                p.rect_filled(r, cr(10.0), th.accent_tint);
            } else if resp.hovered() {
                p.rect_filled(r, cr(10.0), th.row_hover);
            }
            self.ico(&p, ic, if on { Ic::Filled } else { Ic::Duo }, pos2(r.left() + 18.0, r.center().y), 15.0, if on { th.accent_ink } else { th.ink3 });
            p.text(pos2(r.left() + 36.0, r.center().y), Align2::LEFT_CENTER, self.fit(&p, &name, &f_medium(T_BODY), 112.0), if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, if on { th.accent_ink } else { th.ink2 });
            if resp.clicked() {
                go = Some(l);
            }
            y += 34.0;
        }
        // main
        let main = Rect::from_min_max(pos2(rail.right() + 14.0, body.top()), pos2(body.right() - 196.0, body.bottom()));
        let mut x = main.left();
        let cy = main.top() + 14.0;
        let anc = dest.ancestors();
        let skip = anc.len().saturating_sub(3);
        for (i, a) in anc.iter().enumerate().skip(skip) {
            let name = self.loc_title(a);
            let f = if a == &dest { f_semi(T_BODY) } else { f_medium(T_BODY) };
            let w = self.tw(&p, &name, &f) + 14.0;
            let r = Rect::from_min_size(pos2(x, cy - 13.0), vec2(w, 26.0));
            let resp = self.hit(ui, r, ("tr-crumb", i), Sense::click());
            if resp.hovered() {
                p.rect_filled(r, cr(8.0), th.row_hover);
            }
            p.text(pos2(x + 7.0, cy), Align2::LEFT_CENTER, &name, f, if a == &dest { th.ink1 } else { th.ink3 });
            if resp.clicked() && a != &dest {
                go = Some(a.clone());
            }
            x += w;
            if a != &dest {
                p.text(pos2(x + 4.0, cy), Align2::LEFT_CENTER, "\u{203a}", f_regular(T_BODY), th.ink4);
                x += 14.0;
            }
        }
        let list = Rect::from_min_max(pos2(main.left(), main.top() + 36.0), main.max);
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(list).layout(egui::Layout::top_down(egui::Align::Min)));
        cui.set_clip_rect(list.intersect(ui.clip_rect()));
        egui::ScrollArea::vertical().id_salt("tr-scroll").auto_shrink([false, false]).show(&mut cui, |ui| {
            let top = ui.max_rect().top();
            ui.allocate_rect(Rect::from_min_size(pos2(list.left(), top), vec2(list.width(), folders.len() as f32 * 42.0 + 8.0)), Sense::hover());
            if folders.is_empty() {
                p.text(pos2(list.center().x, top + 70.0), Align2::CENTER_CENTER, "No subfolders here", f_medium(T_BODY), th.ink4);
            }
            for (i, f) in folders.iter().enumerate() {
                let r = Rect::from_min_size(pos2(list.left(), top + i as f32 * 42.0), vec2(list.width(), 38.0));
                let blocked = srcs.contains(&f.loc);
                let resp = ui.interact(r, Id::unique(("tr-f", &f.loc)), if blocked { Sense::hover() } else { Sense::click() });
                if resp.hovered() && !blocked {
                    grad_fill(&p, r, 11.0, th.row_press, th.row_hover);
                }
                self.ico(&p, "folder", Ic::Filled, pos2(r.left() + 20.0, r.center().y), 18.0, alpha(th.accent, if blocked { 0.35 } else { 1.0 }));
                p.text(pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, &f.name, f_semi(T_BODY), alpha(th.ink1, if blocked { 0.4 } else { 1.0 }));
                if resp.clicked() {
                    go = Some(f.loc.clone());
                }
            }
        });
        // side summary
        let side = Rect::from_min_max(pos2(body.right() - 182.0, body.top()), body.max);
        p.text(pos2(side.left(), side.top() + 14.0), Align2::LEFT_CENTER, format!("{} ITEM{}", srcs.len(), if srcs.len() == 1 { "" } else { "S" }), f_semi(T_LABEL), th.ink4);
        let mut yy = side.top() + 32.0;
        for l in srcs.iter().take(6) {
            let r = Rect::from_min_size(pos2(side.left(), yy), vec2(side.width(), 36.0));
            let k = Kind::from_ext(&ext_of(&l.name()));
            let e = self.be.stat(l).ok();
            let kind = e.as_ref().map(|e| e.kind).unwrap_or(k);
            let kc = kind_color(kind, &th);
            if let Some(t) = self.thumbs.get(l) {
                let sz = t.tex.size_vec2();
                let tr = Rect::from_min_size(r.min, vec2(36.0, 36.0));
                textured_rrect(&p, tr, 9.0, t.tex.id(), cover_uv(sz.x, sz.y, tr), Color32::WHITE);
            } else {
                let tr = Rect::from_min_size(r.min, vec2(36.0, 36.0));
                grad_fill(&p, tr, 9.0, alpha(kc, 0.22), alpha(kc, 0.10));
                self.ico(&p, kind.icon(), Ic::Duo, tr.center(), 17.0, kc);
            }
            p.text(pos2(r.left() + 46.0, r.center().y), Align2::LEFT_CENTER, self.fit(&p, &l.name(), &f_medium(T_META), side.width() - 50.0), f_medium(T_META), th.ink2);
            yy += 42.0;
        }
        if let Some(l) = go {
            if let Some(t) = self.transfer.as_mut() {
                t.dest = l.clone();
            }
            let req = self.be.list(&l);
            if let Some(t) = self.transfer.as_mut() {
                t.req = req;
            }
        }
        self.modal_foot_note(&p, card, "Tags and metadata travel with the file.", 230.0);
        let mut cur = card.right() - 22.0;
        let label = if mv { "Move here" } else { "Copy here" };
        let can = !srcs.iter().any(|s| dest.starts_with(s)) && !(mv && srcs.iter().all(|s| s.parent().as_ref() == Some(&dest)));
        if can {
            if self.foot_btn(ui, card, &mut cur, label, Btn::Accent, "tr-go") {
                self.acts.push(Act::Job(if mv { JobKind::Move { srcs: srcs.clone(), dest: dest.clone() } } else { JobKind::Copy { srcs: srcs.clone(), dest: dest.clone() } }));
            }
        }
        if self.foot_btn(ui, card, &mut cur, "Cancel", Btn::Raised, "tr-cancel") {
            *close = true;
        }
        if self.foot_btn(ui, card, &mut cur, "New folder", Btn::Raised, "tr-newf") {
            if let Ok(_l) = self.be.create_folder(&dest, "untitled folder") {
                let req = self.be.list(&dest);
                if let Some(t) = self.transfer.as_mut() {
                    t.req = req;
                }
            }
        }
    }
}
