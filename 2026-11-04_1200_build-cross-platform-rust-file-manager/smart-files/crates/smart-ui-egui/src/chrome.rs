//! Window, sidebar, app bar, status bar + shared widget helpers.
use crate::app::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, FontId, Id, Painter, Pos2, Rect, Response, Sense, Ui};
use smart_core::*;

pub const R_WIN: f32 = 24.0;
pub const SIDEBAR_W: f32 = 252.0;
pub const APPBAR_H: f32 = 60.0;
pub const STATUS_H: f32 = 34.0;
pub const PAD_X: f32 = 26.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Btn {
    Raised,
    Quiet,
    Accent,
    Active,
}

impl SmartApp {
    // ───────────── text helpers ─────────────
    pub fn txt(&self, p: &Painter, pos: Pos2, a: Align2, s: &str, f: FontId, col: Color32) -> Rect {
        p.text(pos, a, s, f, col)
    }
    pub fn tw(&self, p: &Painter, s: &str, f: &FontId) -> f32 {
        p.layout_no_wrap(s.to_string(), f.clone(), Color32::WHITE).size().x
    }
    pub fn fit(&self, p: &Painter, s: &str, f: &FontId, w: f32) -> String {
        if self.tw(p, s, f) <= w {
            return s.to_string();
        }
        let chars: Vec<char> = s.chars().collect();
        let (mut lo, mut hi) = (0usize, chars.len());
        while lo < hi {
            let mid = (lo + hi).div_ceil(2);
            let t: String = chars[..mid].iter().collect::<String>() + "\u{2026}";
            if self.tw(p, &t, f) <= w { lo = mid } else { hi = mid - 1 }
        }
        chars[..lo].iter().collect::<String>().trim_end().to_string() + "\u{2026}"
    }
    pub fn ico(&self, p: &Painter, name: &'static str, v: Ic, center: Pos2, size: f32, col: Color32) {
        self.icons.paint_at(p, name, v, center, size, col);
    }
    pub fn hit(&self, ui: &Ui, rect: Rect, id: impl std::hash::Hash + std::fmt::Debug, sense: Sense) -> Response {
        ui.interact(rect, Id::unique(("hit", id)), sense)
    }
    pub fn anim(&self, ui: &Ui, id: impl std::hash::Hash + std::fmt::Debug, on: bool) -> f32 {
        ui.ctx().animate_bool_with_time(Id::unique(("anim", id)), on, 0.14)
    }
    pub fn keycap(&self, p: &Painter, right_center: Pos2, s: &str, strong: bool) -> f32 {
        let f = f_semi(10.0);
        let w = (self.tw(p, s, &f) + 9.0).max(18.0);
        let r = Rect::from_min_max(pos2(right_center.x - w, right_center.y - 9.0), pos2(right_center.x, right_center.y + 9.0));
        grad_fill(p, r, 5.0, alpha(self.th.well, if strong { 0.8 } else { 0.55 }), alpha(self.th.well, 0.35));
        rim(p, r, 5.0, 1.0, alpha(Color32::WHITE, if self.th.dark { 0.07 } else { 0.9 }), false, 4.0);
        p.text(r.center() + vec2(0.0, 0.5), Align2::CENTER_CENTER, s, f, self.th.ink3);
        w
    }

    /// Round icon button with the light-from-above reflection (all app-bar controls use this).
    pub fn icon_button(&self, ui: &Ui, rect: Rect, name: &'static str, id: &str, kind: Btn, enabled: bool) -> Response {
        let sense = if enabled { Sense::click() } else { Sense::hover() };
        let resp = self.hit(ui, rect, id, sense);
        let p = ui.painter();
        let th = &self.th;
        let hov = enabled && resp.hovered();
        let down = enabled && resp.is_pointer_button_down_on();
        let r = rect.height() / 2.0;
        let a = self.anim(ui, id, hov);
        let mut col = if enabled { mix(th.ink2, th.ink1, a) } else { alpha(th.ink2, 0.38) };
        match kind {
            Btn::Raised => raised(p, th, rect, r, hov, down),
            Btn::Active => {
                raised(p, th, rect, r, false, true);
                col = th.accent_ink;
            }
            Btn::Accent => {
                raised_accent(p, th, rect, r, hov, down);
                col = th.on_accent();
            }
            Btn::Quiet => {
                if hov {
                    p.rect_filled(rect, cr(r), if down { th.row_press } else { th.row_hover });
                }
            }
        }
        let v = if kind == Btn::Active { Ic::Filled } else if hov || kind == Btn::Accent { Ic::Duo } else { Ic::Outline };
        let off = if down && kind != Btn::Quiet { vec2(0.0, 0.6) } else { vec2(0.0, 0.0) };
        self.ico(p, name, v, rect.center() + off, 17.0, col);
        resp
    }

    pub fn label_button(&self, ui: &Ui, rect: Rect, icon: Option<&'static str>, label: &str, id: &str, kind: Btn) -> Response {
        let resp = self.hit(ui, rect, id, Sense::click());
        let p = ui.painter();
        let th = &self.th;
        let (hov, down) = (resp.hovered(), resp.is_pointer_button_down_on());
        let r = rect.height() / 2.0;
        let mut col = th.ink1;
        match kind {
            Btn::Accent => {
                raised_accent(p, th, rect, r, hov, down);
                col = th.on_accent();
            }
            Btn::Quiet => {
                if hov {
                    p.rect_filled(rect, cr(r), th.row_hover);
                }
                col = th.ink2;
            }
            _ => raised(p, th, rect, r, hov, down),
        }
        let f = f_semi(T_BODY);
        let tw = self.tw(p, label, &f);
        let iw = if icon.is_some() { 18.0 + 7.0 } else { 0.0 };
        let mut x = rect.center().x - (tw + iw) / 2.0;
        if let Some(i) = icon {
            self.ico(p, i, if kind == Btn::Accent || hov { Ic::Duo } else { Ic::Outline }, pos2(x + 8.0, rect.center().y), 16.0, if kind == Btn::Accent { col } else { th.ink2 });
            x += iw;
        }
        p.text(pos2(x, rect.center().y + (if down { 0.8 } else { 0.0 })), Align2::LEFT_CENTER, label, f, col);
        resp
    }

    // ───────────── window ─────────────
    pub fn window_rect(&self, screen: Rect) -> Rect {
        if !IS_WEB {
            return screen;
        }
        if self.win_zoom {
            return screen.shrink(6.0);
        }
        let padx = (screen.width() * 0.036).clamp(14.0, 58.0);
        let pady = (screen.height() * 0.034).clamp(14.0, 38.0);
        let w = (screen.width() - 2.0 * padx).min(1440.0);
        let h = (screen.height() - 2.0 * pady).min(860.0);
        let mut c = screen.center() + self.win_off;
        c.x = c.x.clamp(screen.left() + w / 2.0 - w * 0.6, screen.right() - w / 2.0 + w * 0.6);
        c.y = c.y.clamp(screen.top() + h / 2.0 - 8.0 - pady, screen.bottom() - h / 2.0 + h * 0.6);
        Rect::from_center_size(c, vec2(w, h))
    }

    pub fn draw_desktop(&self, p: &Painter, screen: Rect) {
        let wi = self.st().wallpaper.min(self.walls.len() - 1);
        let (tex, _, sz) = &self.walls[wi];
        let uv = cover_uv(sz[0], sz[1], screen);
        p.image(tex.id(), screen, uv, Color32::WHITE);
        let th = &self.th;
        let mut m = egui::epaint::Mesh::default();
        for (y, col) in [(screen.top(), th.veil_top), (screen.bottom(), th.veil_bottom)] {
            m.vertices.push(egui::epaint::Vertex { pos: pos2(screen.left(), y), uv: egui::epaint::WHITE_UV, color: col });
            m.vertices.push(egui::epaint::Vertex { pos: pos2(screen.right(), y), uv: egui::epaint::WHITE_UV, color: col });
        }
        m.indices.extend([0, 1, 2, 1, 3, 2]);
        p.add(egui::Shape::mesh(m));
        if screen.height() >= 900.0 {
            let shadow_c = Color32::from_black_alpha(80);
            p.text(pos2(screen.right() - 22.0 + 0.5, 25.0 + 1.0), Align2::RIGHT_CENTER, clock_hm(), f_semi(17.0), shadow_c);
            p.text(pos2(screen.right() - 22.0, 25.0), Align2::RIGHT_CENTER, clock_hm(), f_semi(17.0), Color32::from_white_alpha(235));
            p.text(pos2(screen.right() - 22.0, 45.0), Align2::RIGHT_CENTER, clock_date(), f_medium(11.0), Color32::from_white_alpha(210));
        }
    }

    pub fn draw_window(&mut self, ui: &mut Ui) {
        let screen = ui.max_rect();
        let p = ui.painter().clone();
        if IS_WEB {
            self.draw_desktop(&p, screen);
        }
        if self.win_closed || self.win_min {
            self.draw_dock(ui, screen);
            return;
        }
        let win = self.window_rect(screen);
        let th = self.th.clone();
        if IS_WEB {
            elevate(&p, &th, win, R_WIN, 5);
        }
        // glass: blurred wallpaper under translucent tint
        let wi = self.st().wallpaper.min(self.walls.len() - 1);
        let (_, blur, sz) = &self.walls[wi];
        let suv = cover_uv(sz[0], sz[1], screen);
        let f = |pt: Pos2| pos2(suv.min.x + (pt.x - screen.min.x) / screen.width() * suv.width(), suv.min.y + (pt.y - screen.min.y) / screen.height() * suv.height());
        let uv = Rect::from_min_max(f(win.min), f(win.max));
        textured_rrect(&p, win, R_WIN, blur.id(), uv, Color32::from_white_alpha(if th.dark { 210 } else { 255 }));
        grad_fill(&p, win, R_WIN, th.window_top, th.window_bottom);

        let open = self.sidebar_open;
        let sb_w = SIDEBAR_W * open;
        let sb = Rect::from_min_size(win.min, vec2(sb_w, win.height()));
        let stage = Rect::from_min_max(pos2(win.min.x + sb_w, win.min.y), win.max);
        // sidebar tone
        if sb_w > 1.0 {
            let pp = p.with_clip_rect(sb);
            grad_fill(&pp, Rect::from_min_max(win.min, pos2(win.min.x + sb_w + R_WIN + 4.0, win.max.y)), R_WIN, th.sidebar_top, th.sidebar_bottom);
            // tonal seam
            let mut m = egui::epaint::Mesh::default();
            let x = sb.right() - 1.0;
            let (c0, c1) = if th.dark { (c(255, 255, 255, 0.09), c(0, 0, 0, 0.16)) } else { (c(255, 255, 255, 0.52), c(20, 29, 48, 0.04)) };
            for (y, col) in [(sb.top() + 14.0, c0), (sb.bottom() - 14.0, c1)] {
                m.vertices.push(egui::epaint::Vertex { pos: pos2(x, y), uv: egui::epaint::WHITE_UV, color: col });
                m.vertices.push(egui::epaint::Vertex { pos: pos2(x + 1.0, y), uv: egui::epaint::WHITE_UV, color: col });
            }
            m.indices.extend([0, 1, 2, 1, 3, 2]);
            p.add(egui::Shape::mesh(m));
        }
        // content tone
        {
            let pp = p.with_clip_rect(stage);
            grad_fill(&pp, Rect::from_min_max(pos2(stage.min.x - R_WIN - 4.0, win.min.y), win.max), R_WIN, th.content_top, th.content_bottom);
        }

        // sidebar content (clipped so the collapse animation never spills)
        if sb_w > 8.0 {
            let mut sui = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(win.min, vec2(SIDEBAR_W, win.height()))).layout(egui::Layout::top_down(egui::Align::Min)));
            sui.set_clip_rect(sb.intersect(ui.clip_rect()));
            self.draw_sidebar(&mut sui, Rect::from_min_size(win.min, vec2(SIDEBAR_W, win.height())));
        }
        // stage
        let appbar = Rect::from_min_size(stage.min, vec2(stage.width(), APPBAR_H));
        let status = Rect::from_min_max(pos2(stage.min.x, win.max.y - STATUS_H), win.max);
        let content = Rect::from_min_max(pos2(stage.min.x, appbar.max.y), pos2(stage.max.x, status.min.y));
        self.draw_content(ui, content);
        self.draw_status(ui, status, win);
        self.draw_appbar(ui, appbar, win);
        if self.grid_overlay {
            self.draw_grid_overlay(&p, win);
        }
        // window rim
        rim(&p, win, R_WIN, 1.4, if th.dark { c(255, 255, 255, 0.12) } else { c(255, 255, 255, 0.9) }, true, 42.0);
        p.rect_stroke(win.shrink(0.5), cr(R_WIN), egui::Stroke::new(1.0, if th.dark { c(255, 255, 255, 0.035) } else { c(255, 255, 255, 0.12) }), egui::StrokeKind::Inside);
        self.win_rect_cache(win);
    }

    fn win_rect_cache(&self, w: Rect) {
        crate::overlays::WIN_RECT.with(|c| c.set(w));
    }

    fn draw_grid_overlay(&self, p: &Painter, win: Rect) {
        let pp = p.with_clip_rect(win);
        let col = alpha(self.th.accent, 0.10);
        let major = alpha(self.th.accent, 0.20);
        let mut x = win.left();
        let mut i = 0;
        while x < win.right() {
            pp.line_segment([pos2(x, win.top()), pos2(x, win.bottom())], egui::Stroke::new(1.0, if i % 8 == 0 { major } else { col }));
            x += 8.0;
            i += 1;
        }
        let mut y = win.top();
        i = 0;
        while y < win.bottom() {
            pp.line_segment([pos2(win.left(), y), pos2(win.right(), y)], egui::Stroke::new(1.0, if i % 8 == 0 { major } else { col }));
            y += 8.0;
            i += 1;
        }
        let r = Rect::from_min_size(win.left_bottom() + vec2(16.0, -52.0), vec2(250.0, 28.0));
        grad_fill(&pp, r, 14.0, alpha(self.th.accent, 0.95), alpha(self.th.accent, 0.85));
        pp.text(r.center(), Align2::CENTER_CENTER, "8 px grid \u{b7} 64 px major \u{b7} 4 px baseline", f_semi(11.0), self.th.on_accent());
    }

    fn draw_dock(&mut self, ui: &mut Ui, screen: Rect) {
        let p = ui.painter().clone();
        let r = Rect::from_center_size(pos2(screen.center().x, screen.bottom() - 46.0), vec2(250.0, 52.0));
        elevate(&p, &self.th, r, 26.0, 4);
        grad_fill(&p, r, 26.0, self.th.panel_top, self.th.panel_bottom);
        rim(&p, r, 26.0, 1.2, self.th.rim, true, 30.0);
        let ic = Rect::from_center_size(pos2(r.left() + 30.0, r.center().y), vec2(32.0, 32.0));
        let (a, b) = self.th.accent_grad();
        grad_fill(&p, ic, 10.0, a, b);
        p.text(ic.center(), Align2::CENTER_CENTER, "S", f_bold(17.0), Color32::WHITE);
        p.text(pos2(ic.right() + 12.0, r.center().y - 7.0), Align2::LEFT_CENTER, "Smart Files", f_semi(13.0), self.th.ink1);
        p.text(pos2(ic.right() + 12.0, r.center().y + 8.0), Align2::LEFT_CENTER, if self.win_min { "Minimised \u{b7} click to restore" } else { "Closed \u{b7} click to reopen" }, f_medium(11.0), self.th.ink3);
        if self.hit(ui, r, "dock", Sense::click()).clicked() {
            self.acts.push(Act::WinOpen);
        }
    }

    // ───────────── sidebar ─────────────
    fn traffic(&self, ui: &Ui, origin: Pos2, id: &str) {
        let p = ui.painter();
        let cols = [("close", hex("#ff7c72"), hex("#f2554c")), ("min", hex("#ffd264"), hex("#f5b93c")), ("zoom", hex("#6fdc8c"), hex("#38bd62"))];
        let group = Rect::from_min_size(origin - vec2(2.0, 8.0), vec2(64.0, 16.0));
        let g_hover = ui.rect_contains_pointer(group);
        for (i, (n, a, b)) in cols.iter().enumerate() {
            let ctr = origin + vec2(6.0 + 20.0 * i as f32, 0.0);
            let rect = Rect::from_center_size(ctr, vec2(16.0, 16.0));
            let resp = self.hit(ui, rect, (id, n), Sense::click());
            let s = if resp.hovered() { 1.14 } else { 1.0 };
            let dot = Rect::from_center_size(ctr, vec2(12.0 * s, 12.0 * s));
            shadow(p, dot, 6.0, 0.5, 1.0, 0.0, c(20, 29, 48, 0.12));
            let (a, b) = if resp.is_pointer_button_down_on() { (mix(*a, Color32::BLACK, 0.12), mix(*b, Color32::BLACK, 0.12)) } else { (*a, *b) };
            grad_fill(p, dot, 6.0 * s, a, b);
            rim(p, dot, 6.0, 1.0, Color32::from_white_alpha(95), true, 7.0);
            if g_hover {
                let k = c(40, 20, 18, 0.55);
                let st = egui::Stroke::new(1.35, k);
                match *n {
                    "close" => {
                        p.line_segment([ctr + vec2(-2.8, -2.8), ctr + vec2(2.8, 2.8)], st);
                        p.line_segment([ctr + vec2(2.8, -2.8), ctr + vec2(-2.8, 2.8)], st);
                    }
                    "min" => {
                        p.line_segment([ctr + vec2(-3.0, 0.0), ctr + vec2(3.0, 0.0)], st);
                    }
                    _ => {
                        p.line_segment([ctr + vec2(-2.6, 2.6), ctr + vec2(2.6, -2.6)], st);
                        p.line_segment([ctr + vec2(2.6, -2.6), ctr + vec2(0.4, -2.6)], st);
                        p.line_segment([ctr + vec2(2.6, -2.6), ctr + vec2(2.6, -0.4)], st);
                    }
                }
            }
            if resp.clicked() {
                self.push_act(match *n {
                    "close" => Act::WinClose,
                    "min" => Act::WinMin,
                    _ => Act::WinZoom,
                });
            }
        }
    }

    pub fn push_act(&self, a: Act) {
        ACTS.with(|q| q.borrow_mut().push(a));
    }

    pub fn drag_region(&mut self, ui: &Ui, rect: Rect, id: &str) {
        let r = self.hit(ui, rect, id, Sense::click_and_drag());
        if r.drag_started() && !IS_WEB {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if r.dragged() && IS_WEB && !self.win_zoom {
            self.win_off += r.drag_delta();
        }
        if r.double_clicked() {
            if IS_WEB {
                self.win_zoom = !self.win_zoom;
            } else {
                let m = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!m));
            }
        }
    }

    fn draw_sidebar(&mut self, ui: &mut Ui, sb: Rect) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        // top: traffic lights + hide button
        let top = Rect::from_min_size(sb.min, vec2(sb.width(), 54.0));
        self.drag_region(ui, top, "sb-drag");
        self.traffic(ui, pos2(sb.left() + 20.0, sb.top() + 27.0), "sb");
        let hide = Rect::from_center_size(pos2(sb.right() - 13.0 - 15.0, sb.top() + 27.0), vec2(30.0, 30.0));
        let hov = ui.rect_contains_pointer(sb);
        if hov {
            if self.icon_button(ui, hide, "sidebar-minimalistic", "sb-hide", Btn::Quiet, true).clicked() {
                self.push_act(Act::ToggleSidebar);
            }
        }
        // workspace switcher
        let ws = Rect::from_min_size(pos2(sb.left() + 12.0, sb.top() + 54.0), vec2(SIDEBAR_W - 24.0, 42.0));
        let r = self.hit(ui, ws, "workspace", Sense::click());
        if r.hovered() {
            p.rect_filled(ws, cr(12.0), th.row_hover);
        }
        let av = Rect::from_min_size(ws.min + vec2(7.0, 7.0), vec2(28.0, 28.0));
        let (a, b) = th.accent_grad();
        shadow(&p, av, 9.0, 2.0, 6.0, -1.0, alpha(th.accent, 0.35));
        grad_fill(&p, av, 9.0, a, b);
        rim(&p, av, 9.0, 1.0, Color32::from_white_alpha(130), true, 14.0);
        let user = self.user_name();
        p.text(av.center(), Align2::CENTER_CENTER, user.chars().next().unwrap_or('N').to_uppercase().to_string(), f_bold(13.0), Color32::WHITE);
        p.text(pos2(av.right() + 9.0, ws.center().y - 7.5), Align2::LEFT_CENTER, &user, f_semi(13.0), th.ink1);
        p.text(pos2(av.right() + 9.0, ws.center().y + 8.0), Align2::LEFT_CENTER, "Personal workspace", f_medium(11.0), th.ink3);
        let ch = pos2(ws.right() - 16.0, ws.center().y);
        let st = egui::Stroke::new(1.5, th.ink4);
        p.line_segment([ch + vec2(-3.5, -1.5), ch + vec2(0.0, 2.0)], st);
        p.line_segment([ch + vec2(0.0, 2.0), ch + vec2(3.5, -1.5)], st);
        if r.clicked() {
            self.menu = Some(Menu { kind: MenuKind::Workspace, pos: pos2(ws.left(), ws.bottom() + 6.0), target: None, device: None, sub: None, age: 0.0 });
        }

        // footer storage card
        let foot_h = 73.0;
        let card = Rect::from_min_size(pos2(sb.left() + 12.0, sb.bottom() - 14.0 - foot_h), vec2(SIDEBAR_W - 24.0, foot_h));
        self.draw_storage_card(ui, card);

        // scrollable nav
        let nav = Rect::from_min_max(pos2(sb.left(), ws.bottom() + 6.0), pos2(sb.right(), card.top() - 8.0));
        let mut nui = ui.new_child(egui::UiBuilder::new().max_rect(nav).layout(egui::Layout::top_down(egui::Align::Min)));
        nui.set_clip_rect(nav.intersect(ui.clip_rect()));
        {
            let fade = Rect::from_min_max(pos2(nav.left(), nav.bottom() - 18.0), pos2(nav.right(), nav.bottom() + 8.0));
            let mut m = egui::epaint::Mesh::default();
            let base = th.sidebar_bottom;
            for (y, col) in [(fade.top(), alpha(base, 0.0)), (fade.bottom(), alpha(mix(base, Color32::WHITE, 0.0), 0.0))] { let _ = (y, col); }
            let _ = &mut m;
        }
        egui::ScrollArea::vertical().id_salt("sb-scroll").auto_shrink([false, false]).scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(&mut nui, |ui| {
            self.sidebar_rows(ui, nav);
        });
    }

    pub fn user_name(&self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        if !self.be.demo {
            let u = std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "You".into());
            let mut c = u.chars();
            return c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or(u);
        }
        "Naledzi".into()
    }

    fn sidebar_rows(&mut self, ui: &mut Ui, nav: Rect) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let mut y = nav.top() + 10.0;
        let x0 = nav.left() + 12.0;
        let w = SIDEBAR_W - 24.0;
        let cur = self.route.clone();
        let label = |p: &Painter, y: f32, s: &str| {
            p.text(pos2(nav.left() + 20.0, y + 7.0), Align2::LEFT_CENTER, s.to_uppercase().chars().map(|c| c.to_string()).collect::<Vec<_>>().join("\u{200a}"), f_semi(T_LABEL), th.ink4);
        };
        // ── Favorites
        label(&p, y, "Favorites");
        y += 22.0 + 8.0 - 7.0 + 1.0;
        let mut rows: Vec<(String, &'static str, Route, Option<usize>, Option<Location>)> = vec![];
        let places = self.places.clone();
        for pl in &places {
            rows.push((pl.title.clone(), pl.icon, Route::Loc(pl.loc.clone()), self.place_counts.get(&pl.loc).copied().filter(|_| pl.id == "pictures" || pl.id == "downloads"), Some(pl.loc.clone())));
            if pl.id == "home" {
                rows.push(("Recents".into(), "clock-circle", Route::Recents, None, None));
            }
        }
        for pin in self.lib(|l| l.pins.clone()) {
            rows.push((pin.name(), "folder", Route::Loc(pin.clone()), None, Some(pin)));
        }
        for (name, icon, route, count, loc) in rows {
            let rect = Rect::from_min_size(pos2(x0, y), vec2(w, 32.0));
            let active = cur == route;
            let dropping = self.drag.is_some() && loc.is_some() && ui.rect_contains_pointer(rect);
            let r = self.sb_row(ui, rect, icon, &name, active, count.map(|c| c.to_string()), dropping, &name);
            if r.clicked() {
                self.push_act(Act::Nav(route.clone()));
            }
            if r.secondary_clicked() {
                if let Some(l) = loc.clone() {
                    self.menu = Some(Menu { kind: MenuKind::Place, pos: ui.ctx().pointer_latest_pos().unwrap_or(rect.center()), target: Some(l), device: None, sub: None, age: 0.0 });
                }
            }
            if dropping && ui.input(|i| i.pointer.any_released()) {
                if let Some(l) = loc {
                    let copy = ui.input(|i| i.modifiers.alt);
                    self.drop_on(l, copy);
                }
            }
            y += 33.0;
        }
        // ── Locations
        y += 8.0 + 18.0 - 7.0;
        label(&p, y, "Locations");
        y += 23.0;
        for d in self.devices.clone() {
            let rect = Rect::from_min_size(pos2(x0, y), vec2(w, 32.0));
            let active = matches!(&cur, Route::Loc(l) if d.loc.as_ref().is_some_and(|dl| l == dl || (d.kind != DeviceKind::Internal && l.fs == dl.fs && l.starts_with(dl))));
            let r = self.sb_row_device(ui, rect, &d, active);
            y += 33.0;
            if r.clicked() {
                match (&d.loc, d.mounted, d.kind) {
                    (_, _, DeviceKind::Phone) => self.push_act(Act::Modal(Modal::Phone)),
                    (Some(l), true, _) => self.push_act(Act::Nav(Route::Loc(l.clone()))),
                    _ => self.push_act(Act::Mount(d.id.clone())),
                }
            }
            if r.secondary_clicked() {
                self.menu = Some(Menu { kind: MenuKind::Device, pos: ui.ctx().pointer_latest_pos().unwrap_or(rect.center()), target: d.loc.clone(), device: Some(d.id.clone()), sub: None, age: 0.0 });
            }
        }
        {
            let rect = Rect::from_min_size(pos2(x0, y), vec2(w, 32.0));
            let r = self.sb_row(ui, rect, "add", "Mount a device\u{2026}", false, None, false, "mount-dev");
            if r.clicked() {
                self.push_act(Act::Modal(Modal::Devices));
            }
            y += 33.0;
        }
        // ── Tags
        y += 8.0 + 18.0 - 7.0;
        label(&p, y, "Tags");
        y += 23.0;
        let tags = self.lib(|l| l.tags.clone());
        for t in tags {
            let rect = Rect::from_min_size(pos2(x0, y), vec2(w, 32.0));
            let active = cur == Route::Tag(t.id.clone());
            let n = self.lib(|l| l.tag_count(&t.id));
            let dropping = self.drag.is_some() && ui.rect_contains_pointer(rect);
            let r = self.sb_row_tag(ui, rect, &t.name, hex(&t.color), active, n, dropping);
            if r.clicked() {
                self.push_act(Act::Nav(Route::Tag(t.id.clone())));
            }
            if dropping && ui.input(|i| i.pointer.any_released()) {
                if let Some(d) = self.drag.take() {
                    for l in d.items {
                        if !self.lib(|lib| lib.has_tag(&l, &t.id)) {
                            self.be.library.write().unwrap().toggle_tag(&l, &t.id);
                        }
                    }
                    self.toast(format!("Tagged {}", t.name), true);
                }
            }
            y += 33.0;
        }
        {
            let rect = Rect::from_min_size(pos2(x0, y), vec2(w, 32.0));
            let r = self.sb_row(ui, rect, "add", "New tag\u{2026}", false, None, false, "new-tag");
            if r.clicked() {
                self.push_act(Act::Modal(Modal::NewTag));
            }
            y += 33.0;
        }
        ui.allocate_rect(Rect::from_min_max(nav.min, pos2(nav.right(), y + 6.0)), Sense::hover());
    }

    #[allow(clippy::too_many_arguments)]
    fn sb_row(&self, ui: &Ui, rect: Rect, icon: &'static str, name: &str, active: bool, tail: Option<String>, drop: bool, id: &str) -> Response {
        let r = self.hit(ui, rect, ("sbrow", id), Sense::click());
        let p = ui.painter();
        let th = &self.th;
        if active {
            p.rect_filled(rect, cr(10.0), th.accent_tint);
        } else if drop {
            p.rect_filled(rect, cr(10.0), th.accent_tint);
        } else if r.is_pointer_button_down_on() {
            p.rect_filled(rect, cr(10.0), th.row_press);
        } else if r.hovered() {
            p.rect_filled(rect, cr(10.0), th.row_hover);
        }
        let col = if active { th.accent_ink } else { th.ink2 };
        self.ico(p, icon, if active { Ic::Filled } else { Ic::Duo }, pos2(rect.left() + 11.0 + 8.0, rect.center().y), 16.0, if active { th.accent_ink } else { th.ink3 });
        let f = if active { f_semi(T_BODY) } else { f_medium(T_BODY) };
        p.text(pos2(rect.left() + 11.0 + 16.0 + 10.0, rect.center().y), Align2::LEFT_CENTER, name, f, col);
        if let Some(t) = tail {
            p.text(pos2(rect.right() - 12.0, rect.center().y), Align2::RIGHT_CENTER, t, f_medium(T_META), if active { alpha(th.accent_ink, 0.9) } else { th.ink4 });
        }
        r
    }

    fn sb_row_tag(&self, ui: &Ui, rect: Rect, name: &str, color: Color32, active: bool, n: usize, drop: bool) -> Response {
        let r = self.hit(ui, rect, ("tag", name), Sense::click());
        let p = ui.painter();
        let th = &self.th;
        if active || drop {
            p.rect_filled(rect, cr(10.0), th.accent_tint);
        } else if r.hovered() {
            p.rect_filled(rect, cr(10.0), th.row_hover);
        }
        let c0 = pos2(rect.left() + 19.0, rect.center().y);
        shadow(p, Rect::from_center_size(c0, vec2(8.0, 8.0)), 4.0, 1.0, 4.0, 0.0, alpha(color, 0.5));
        grad_fill(p, Rect::from_center_size(c0, vec2(9.0, 9.0)), 4.5, mix(color, Color32::WHITE, 0.25), color);
        p.text(pos2(rect.left() + 37.0, rect.center().y), Align2::LEFT_CENTER, name, if active { f_semi(T_BODY) } else { f_medium(T_BODY) }, if active { th.accent_ink } else { th.ink2 });
        p.text(pos2(rect.right() - 12.0, rect.center().y), Align2::RIGHT_CENTER, n.to_string(), f_medium(T_META), th.ink4);
        r
    }

    fn sb_row_device(&self, ui: &Ui, rect: Rect, d: &Device, active: bool) -> Response {
        let r = self.hit(ui, rect, ("dev", &d.id), Sense::click());
        let p = ui.painter();
        let th = &self.th;
        if active {
            p.rect_filled(rect, cr(10.0), th.accent_tint);
        } else if r.hovered() {
            p.rect_filled(rect, cr(10.0), th.row_hover);
        }
        let dim = if d.mounted { 1.0 } else { 0.55 };
        let col = if active { th.accent_ink } else { th.ink2 };
        self.ico(p, d.kind.icon(), if active { Ic::Filled } else { Ic::Duo }, pos2(rect.left() + 19.0, rect.center().y), 16.0, alpha(if active { th.accent_ink } else { th.ink3 }, dim));
        let name = self.fit(p, &d.name, &f_medium(T_BODY), rect.width() - 84.0);
        p.text(pos2(rect.left() + 37.0, rect.center().y), Align2::LEFT_CENTER, name, if active { f_semi(T_BODY) } else { f_medium(T_BODY) }, alpha(col, dim));
        // eject / mount affordance
        let btn = Rect::from_center_size(pos2(rect.right() - 20.0, rect.center().y), vec2(24.0, 24.0));
        if (d.can_unmount || d.can_eject) && d.mounted {
            let b = self.hit(ui, btn, ("eject", &d.id), Sense::click());
            if r.hovered() || b.hovered() {
                if b.hovered() {
                    p.rect_filled(btn, cr(12.0), th.row_press);
                }
                self.ico(p, "upload-minimalistic", Ic::Duo, btn.center(), 15.0, th.ink2);
                if b.clicked() {
                    self.push_act(if d.can_eject { Act::Eject(d.id.clone()) } else { Act::Unmount(d.id.clone()) });
                }
            }
        } else if !d.mounted {
            p.text(pos2(rect.right() - 12.0, rect.center().y), Align2::RIGHT_CENTER, "Mount", f_semi(T_LABEL), alpha(th.accent_ink, if r.hovered() { 1.0 } else { 0.0 }));
        } else if d.kind == DeviceKind::Phone {
            if let Some(b) = d.battery {
                p.text(pos2(rect.right() - 12.0, rect.center().y), Align2::RIGHT_CENTER, format!("{}%", b), f_medium(T_META), th.ink4);
            }
        }
        r
    }

    fn draw_storage_card(&mut self, ui: &mut Ui, card: Rect) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let dev = self.current_device();
        elevate(&p, &th, card, 14.0, 2);
        grad_fill(&p, card, 14.0, th.card_top, th.card_bottom);
        rim(&p, card, 14.0, 1.2, th.rim, true, 30.0);
        let (name, used, text) = match &dev {
            Some(d) => (d.name.clone(), d.used_fraction(), d.capacity_text()),
            None => ("Computer".to_string(), 0.0, String::new()),
        };
        self.ico(&p, "ssd-square", Ic::Duo, pos2(card.left() + 21.0, card.top() + 21.0), 16.0, th.ink3);
        p.text(pos2(card.left() + 37.0, card.top() + 21.0), Align2::LEFT_CENTER, &name, f_semi(T_META), th.ink1);
        p.text(pos2(card.right() - 14.0, card.top() + 21.0), Align2::RIGHT_CENTER, format!("{:.0}%", used * 100.0), f_medium(T_META), th.ink3);
        let bar = Rect::from_min_size(pos2(card.left() + 12.0, card.top() + 37.0), vec2(card.width() - 24.0, 5.0));
        well(&p, &th, bar, 2.5);
        let fill = Rect::from_min_size(bar.min, vec2((bar.width() * used).max(5.0), 5.0));
        let (a, b) = th.accent_grad();
        grad_fill(&p, fill, 2.5, a, b);
        p.text(pos2(card.left() + 14.0, card.top() + 56.0), Align2::LEFT_CENTER, text, f_medium(11.0), th.ink3);
    }

    pub fn current_device(&self) -> Option<Device> {
        let loc = self.current_loc().or_else(|| Some(self.be.home()))?;
        self.devices
            .iter()
            .filter(|d| d.loc.as_ref().is_some_and(|dl| loc.fs == dl.fs && (loc.starts_with(dl) || dl.is_root())))
            .max_by_key(|d| d.loc.as_ref().map(|l| l.path.len()).unwrap_or(0))
            .cloned()
            .or_else(|| self.devices.first().cloned())
    }

    // ───────────── app bar ─────────────
    fn draw_appbar(&mut self, ui: &mut Ui, bar: Rect, win: Rect) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let cy = bar.center().y;
        self.drag_region(ui, bar, "appbar-drag");
        let mut x = bar.left() + 21.0;
        if self.sidebar_open < 0.5 {
            self.traffic(ui, pos2(win.left() + 20.0, cy), "ab");
            x = win.left() + 20.0 + 64.0 + 10.0;
        }
        // sidebar toggle
        let r = Rect::from_min_size(pos2(x, cy - 17.0), vec2(34.0, 34.0));
        if self.icon_button(ui, r, "sidebar-minimalistic", "sb-toggle", Btn::Raised, true).on_hover_text("Toggle sidebar  Ctrl+\\").clicked() {
            self.push_act(Act::ToggleSidebar);
        }
        x = r.right() + 10.0;
        // back / forward capsule
        let cap = Rect::from_min_size(pos2(x, cy - 17.0), vec2(70.0, 34.0));
        elevate(&p, &th, cap, 17.0, 1);
        grad_fill(&p, cap, 17.0, th.control_top, th.control_bottom);
        rim(&p, cap, 17.0, 1.2, th.rim, true, 18.0);
        rim(&p, cap, 17.0, 1.0, th.rim_under, false, 6.0);
        let b1 = Rect::from_center_size(pos2(cap.left() + 18.0, cy), vec2(30.0, 30.0));
        let b2 = Rect::from_center_size(pos2(cap.right() - 18.0, cy), vec2(30.0, 30.0));
        if self.icon_button(ui, b1, "alt-arrow-left", "back", Btn::Quiet, self.can_back()).on_hover_text("Back  Alt+\u{2190}").clicked() {
            self.push_act(Act::Back);
        }
        if self.icon_button(ui, b2, "alt-arrow-right", "fwd", Btn::Quiet, self.can_fwd()).on_hover_text("Forward  Alt+\u{2192}").clicked() {
            self.push_act(Act::Forward);
        }
        x = cap.right() + 16.0;

        // right cluster (computed from the right edge)
        let mut rx = bar.right() - 14.0;
        let more = Rect::from_min_size(pos2(rx - 34.0, cy - 17.0), vec2(34.0, 34.0));
        if self.icon_button(ui, more, "menu-dots", "more", Btn::Raised, true).on_hover_text("More").clicked() {
            self.menu = Some(Menu { kind: MenuKind::More, pos: pos2(more.right(), more.bottom() + 8.0), target: None, device: None, sub: None, age: 0.0 });
        }
        rx = more.left() - 8.0;
        let st = self.st();
        let sort_label = st.sort.short();
        let f = f_semi(T_BODY);
        let sw = self.tw(&p, sort_label, &f) + 52.0;
        let sort = Rect::from_min_size(pos2(rx - sw, cy - 17.0), vec2(sw, 34.0));
        let sr = self.hit(ui, sort, "sort", Sense::click());
        raised(&p, &th, sort, 17.0, sr.hovered(), sr.is_pointer_button_down_on());
        self.ico(&p, "sort-from-top-to-bottom", if sr.hovered() { Ic::Duo } else { Ic::Outline }, pos2(sort.left() + 20.0, cy), 16.0, th.ink2);
        p.text(pos2(sort.left() + 36.0, cy), Align2::LEFT_CENTER, sort_label, f, th.ink1);
        if sr.clicked() {
            self.menu = Some(Menu { kind: MenuKind::Sort, pos: pos2(sort.right(), sort.bottom() + 8.0), target: None, device: None, sub: None, age: 0.0 });
        }
        rx = sort.left() - 8.0;
        // segmented grid/list
        let seg = Rect::from_min_size(pos2(rx - 62.0, cy - 17.0), vec2(62.0, 34.0));
        well(&p, &th, seg, 17.0);
        for (i, (icon, grid)) in [("widget", true), ("list-check", false)].into_iter().enumerate() {
            let b = Rect::from_center_size(pos2(seg.left() + 17.0 + 28.0 * i as f32, cy), vec2(28.0, 28.0));
            let on = st.grid == grid;
            let resp = self.hit(ui, b, ("seg", i), Sense::click());
            if on {
                raised(&p, &th, b, 14.0, false, false);
            }
            let col = if on { th.accent_ink } else if resp.hovered() { th.ink1 } else { th.ink3 };
            self.ico(&p, icon, if on { Ic::Filled } else if resp.hovered() { Ic::Duo } else { Ic::Outline }, b.center(), 16.0, col);
            if resp.clicked() {
                self.push_act(Act::SetGrid(grid));
            }
            resp.on_hover_text(if grid { "Grid view  G" } else { "List view  L" });
        }
        rx = seg.left() - 8.0;
        // Ask Smart pill
        let ask_on = self.ai.open;
        let ask = Rect::from_min_size(pos2(rx - 108.0, cy - 17.0), vec2(108.0, 34.0));
        let ar = self.hit(ui, ask, "ask", Sense::click());
        if ask_on {
            raised_accent(&p, &th, ask, 17.0, ar.hovered(), ar.is_pointer_button_down_on());
        } else {
            // tinted-accent glass pill, same light treatment
            if ar.hovered() { elevate(&p, &th, ask, 17.0, 2) } else { elevate(&p, &th, ask, 17.0, 1) }
            grad_fill(&p, ask, 17.0, mix(th.accent_tint, th.control_top, if th.dark { 0.0 } else { 0.45 }), mix(th.accent_tint, th.control_bottom, if th.dark { 0.0 } else { 0.2 }));
            grad_fill(&p, ask, 17.0, alpha(th.accent, 0.13), alpha(th.accent, 0.07));
            rim(&p, ask, 17.0, 1.2, th.rim, true, 20.0);
            rim(&p, ask, 17.0, 1.0, th.rim_under, false, 6.0);
        }
        let acol = if ask_on { th.on_accent() } else { th.accent_ink };
        self.ico(&p, "stars-minimalistic", if ask_on || ar.hovered() { Ic::Filled } else { Ic::Duo }, pos2(ask.left() + 22.0, cy), 17.0, acol);
        p.text(pos2(ask.left() + 38.0, cy), Align2::LEFT_CENTER, "Ask Smart", f_semi(T_BODY), acol);
        if ar.clicked() {
            self.push_act(Act::Ask);
        }
        rx = ask.left() - 14.0;

        // breadcrumbs
        let crumbs = self.crumbs();
        let crumb_right = rx - 200.0;
        self.draw_crumbs(ui, pos2(x, cy), crumbs, crumb_right);

        // search field
        let focus_t = ui.ctx().animate_bool_with_time(Id::unique("search-w"), self.search.focused || self.search_active(), 0.18);
        let sw = 214.0 + 110.0 * focus_t;
        let search = Rect::from_min_size(pos2(rx - sw.min((rx - x - 120.0).max(170.0)), cy - 17.0), vec2(sw.min((rx - x - 120.0).max(170.0)), 34.0));
        self.draw_search(ui, search);
    }

    fn draw_crumbs(&mut self, ui: &mut Ui, left: Pos2, crumbs: Vec<(String, Route)>, max_x: f32) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let n = crumbs.len();
        let fm = f_medium(T_BODY);
        let fs = f_semi(T_BODY);
        // measure; collapse middle crumbs when too wide
        let sep = 22.0;
        let widths: Vec<f32> = crumbs.iter().enumerate().map(|(i, (s, _))| self.tw(&p, s, if i + 1 == n { &fs } else { &fm }) + 16.0).collect();
        let total: f32 = widths.iter().sum::<f32>() + sep * (n.saturating_sub(1)) as f32;
        let avail = (max_x - left.x).max(80.0);
        let mut skip_from = None;
        if total > avail && n > 3 {
            skip_from = Some(1);
        }
        let mut x = left.x;
        let mut shown_ellipsis = false;
        for (i, (name, route)) in crumbs.iter().enumerate() {
            if let Some(s) = skip_from {
                if i >= s && i + 2 < n {
                    if !shown_ellipsis {
                        shown_ellipsis = true;
                        p.text(pos2(x + 8.0, left.y), Align2::LEFT_CENTER, "\u{2026}", fm.clone(), th.ink3);
                        x += 24.0;
                        p.text(pos2(x - 12.0 + 4.0, left.y), Align2::LEFT_CENTER, "\u{203a}", f_regular(T_BODY), th.ink4);
                        x += sep - 12.0;
                    }
                    continue;
                }
            }
            let last = i + 1 == n;
            let w = widths[i].min(avail * 0.5);
            let r = Rect::from_min_size(pos2(x, left.y - 14.0), vec2(w, 28.0));
            let resp = self.hit(ui, r, ("crumb", i), if last { Sense::hover() } else { Sense::click() });
            if resp.hovered() && !last {
                p.rect_filled(r, cr(9.0), th.row_hover);
            }
            let f = if last { fs.clone() } else { fm.clone() };
            let t = self.fit(&p, name, &f, w - 16.0);
            p.text(pos2(x + 8.0, left.y), Align2::LEFT_CENTER, t, f, if last { th.ink1 } else if resp.hovered() { th.ink1 } else { th.ink3 });
            x += w;
            if !last {
                p.text(pos2(x + 5.0, left.y), Align2::LEFT_CENTER, "\u{203a}", f_regular(T_BODY), th.ink4);
                x += sep - 6.0 + 6.0 - 6.0;
            }
            if resp.clicked() {
                self.push_act(Act::Nav(route.clone()));
            }
        }
    }

    fn draw_search(&mut self, ui: &mut Ui, r: Rect) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let focused = self.search.focused;
        well(&p, &th, r, 17.0);
        if focused {
            p.rect_stroke(r.expand(1.0), cr(18.0), egui::Stroke::new(2.0, alpha(th.accent, 0.55)), egui::StrokeKind::Outside);
            shadow(&p, r, 17.0, 0.0, 12.0, 2.0, alpha(th.accent, 0.18));
        }
        self.ico(&p, "magnifer", if focused { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 20.0, r.center().y), 16.0, if focused { th.accent_ink } else { th.ink3 });
        let active = self.search_active();
        let right_pad = if active { 34.0 } else { 46.0 };
        let te = Rect::from_min_max(pos2(r.left() + 36.0, r.top() + 5.0), pos2(r.right() - right_pad, r.bottom() - 5.0));
        let hint = format!("Search {}", self.title());
        let mut q = std::mem::take(&mut self.search.query);
        let id = Id::unique("search-input");
        let before = q.clone();
        let out = ui.put(te, egui::TextEdit::singleline(&mut q).id(id).frame(egui::Frame::NONE).hint_text(egui::RichText::new(hint).color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
        if self.search.focus_next {
            out.request_focus();
            self.search.focus_next = false;
        }
        self.search.focused = out.has_focus();
        self.search.query = q;
        if before != self.search.query {
            self.search.dirty_at = self.time;
            if !self.search.query.is_empty() && self.search.last_sent.is_empty() {
                // first keystroke: search right away so results feel instant
                self.run_search();
            }
        }
        if out.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.clear_search();
            out.surrender_focus();
        }
        if active {
            let b = Rect::from_center_size(pos2(r.right() - 18.0, r.center().y), vec2(24.0, 24.0));
            let resp = self.hit(ui, b, "search-clear", Sense::click());
            self.ico(&p, "close-circle", if resp.hovered() { Ic::Filled } else { Ic::Duo }, b.center(), 16.0, th.ink3);
            if resp.clicked() {
                self.push_act(Act::ClearSearch);
            }
        } else {
            self.keycap(&p, pos2(r.right() - 11.0, r.center().y), "\u{2318}K", false);
            let kb = Rect::from_min_max(pos2(r.right() - 48.0, r.top()), r.right_bottom());
            if self.hit(ui, kb, "search-k", Sense::click()).clicked() {
                self.push_act(Act::Palette);
            }
        }
    }

    // ───────────── status bar ─────────────
    fn draw_status(&mut self, ui: &mut Ui, bar: Rect, _win: Rect) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        // top tonal seam instead of a stroke
        let mut m = egui::epaint::Mesh::default();
        for (x, col) in [(bar.left(), Color32::TRANSPARENT), (bar.left() + 40.0, alpha(th.shadow, if th.dark { 0.35 } else { 0.05 })), (bar.right() - 40.0, alpha(th.shadow, if th.dark { 0.35 } else { 0.05 })), (bar.right(), Color32::TRANSPARENT)] {
            m.vertices.push(egui::epaint::Vertex { pos: pos2(x, bar.top()), uv: egui::epaint::WHITE_UV, color: col });
            m.vertices.push(egui::epaint::Vertex { pos: pos2(x, bar.top() + 1.0), uv: egui::epaint::WHITE_UV, color: col });
        }
        for i in 0..3u32 {
            m.indices.extend([2 * i, 2 * i + 1, 2 * i + 2, 2 * i + 1, 2 * i + 3, 2 * i + 2]);
        }
        p.add(egui::Shape::mesh(m));
        let cy = bar.center().y;
        let sel = self.selected();
        let mut x = bar.left() + 16.0;
        let f = f_semi(T_META);
        let fm = f_medium(T_META);
        let (head, tail): (String, String) = if self.search_active() {
            (format!("{} result{}", self.search.hits.len(), if self.search.hits.len() == 1 { "" } else { "s" }), if self.search.done { format!("{} {} indexed \u{b7} {} ms", if self.search.cached { "\u{26a1}" } else { "" }, self.search.scanned, self.search.ms).trim().to_string() } else { "searching\u{2026}".into() })
        } else if !sel.is_empty() {
            let b: u64 = sel.iter().filter(|e| !e.is_dir).map(|e| e.size).sum();
            (format!("{} selected", sel.len()), format!("{} of {} items", format_size(b), self.items.len()))
        } else {
            (format!("{} item{}", self.items.len(), if self.items.len() == 1 { "" } else { "s" }), format_size(self.total_bytes()))
        };
        let r = p.text(pos2(x, cy), Align2::LEFT_CENTER, head, f, th.ink2);
        x = r.right() + 8.0;
        p.circle_filled(pos2(x, cy), 1.5, th.ink4);
        p.text(pos2(x + 8.0, cy), Align2::LEFT_CENTER, tail, fm.clone(), th.ink3);

        // jobs (right-aligned, before hints)
        let mut rx = bar.right() - 16.0;
        if !IS_WEB || bar.width() > 760.0 {
            for (k, label) in [("D", "theme"), ("G", "grid"), ("T", "tokens"), ("\u{2318}K", "commands")].into_iter().rev() {
                let w = self.tw(&p, label, &fm);
                p.text(pos2(rx, cy), Align2::RIGHT_CENTER, label, fm.clone(), th.ink4);
                let kw = self.keycap(&p, pos2(rx - w - 6.0, cy), k, false);
                rx -= w + 6.0 + kw + 14.0;
            }
        }
        rx -= 6.0;
        // device sync / free space
        if let Some(d) = self.current_device() {
            let t = if d.total > 0 { format!("{} \u{b7} {} free", d.name, format_size(d.free)) } else { d.name.clone() };
            let w = self.tw(&p, &t, &fm);
            p.text(pos2(rx, cy), Align2::RIGHT_CENTER, &t, fm.clone(), th.ink3);
            self.ico(&p, "check-circle", Ic::Duo, pos2(rx - w - 12.0, cy), 14.0, th.good);
            rx -= w + 30.0;
        }
        // active job pill
        if let Some(j) = self.jobs.iter().find(|j| j.state == JobState::Running) {
            let frac = j.done as f32 / j.total.max(1) as f32;
            let w = 190.0;
            let pill = Rect::from_min_size(pos2(rx - w, cy - 11.0), vec2(w, 22.0));
            well(&p, &th, pill, 11.0);
            let fill = Rect::from_min_size(pill.min, vec2((w * frac).max(14.0), 22.0));
            let (a, b) = th.accent_grad();
            grad_fill(&p, fill, 11.0, alpha(a, 0.9), alpha(b, 0.9));
            let t = self.fit(&p, &j.title, &f_semi(10.5), w - 18.0);
            p.text(pill.center(), Align2::CENTER_CENTER, t, f_semi(10.5), if frac > 0.5 { th.on_accent() } else { th.ink2 });
        }
    }
}

thread_local! {
    pub static ACTS: std::cell::RefCell<Vec<Act>> = const { std::cell::RefCell::new(Vec::new()) };
}
