//! Context menus, command palette, Ask Smart panel, toasts, drag ghost, transfer jobs.
use crate::app::*;
use crate::chrome::*;
use crate::content::kind_color;
use crate::icons::Ic;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Context, Id, Order, Rect, Sense, Ui};
use smart_core::*;

thread_local! {
    pub static WIN_RECT: std::cell::Cell<Rect> = const { std::cell::Cell::new(Rect::NOTHING) };
}

pub struct MI {
    pub icon: &'static str,
    pub label: String,
    pub kbd: &'static str,
    pub act: Option<Act>,
    pub danger: bool,
    pub disabled: bool,
    pub checked: bool,
    pub sub: Vec<MI>,
    pub gap: bool,
    pub dot: Option<Color32>,
}

fn mi(icon: &'static str, label: impl Into<String>, act: Act) -> MI {
    MI { icon, label: label.into(), kbd: "", act: Some(act), danger: false, disabled: false, checked: false, sub: vec![], gap: false, dot: None }
}
impl MI {
    fn k(mut self, k: &'static str) -> Self {
        self.kbd = k;
        self
    }
    fn gap(mut self) -> Self {
        self.gap = true;
        self
    }
    fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
    fn off(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    fn on(mut self, c: bool) -> Self {
        self.checked = c;
        self
    }
    fn sub(mut self, s: Vec<MI>) -> Self {
        self.act = None;
        self.sub = s;
        self
    }
}

pub fn win_rect() -> Rect {
    WIN_RECT.with(|c| c.get())
}

impl SmartApp {
    // ───────────── menus ─────────────
    fn menu_items(&self, m: &Menu) -> Vec<MI> {
        let st = self.st();
        let sel = self.selected();
        let one = m.target.clone();
        match m.kind {
            MenuKind::Item => {
                let Some(t) = one else { return vec![] };
                let e = self.be.stat(&t).ok();
                let is_dir = e.as_ref().is_some_and(|e| e.is_dir);
                let prev = e.as_ref().is_some_and(|e| e.kind.previewable());
                let multi = sel.len() > 1;
                let mut apps: Vec<MI> = if is_dir || multi { vec![] } else { self.be.apps_for(&t).into_iter().take(7).map(|a| MI { icon: a.icon, label: format!("{}{}", a.name, if a.is_default { "  \u{b7} default" } else { "" }), kbd: "", act: Some(Act::OpenWith(t.clone(), a)), danger: false, disabled: false, checked: false, sub: vec![], gap: false, dot: None }).collect() };
                apps.push(mi("menu-dots", "Other\u{2026}", Act::Modal(Modal::OpenWith(t.clone()))).gap());
                let tags: Vec<MI> = self
                    .lib(|l| l.tags.clone())
                    .into_iter()
                    .map(|tg| {
                        let on = sel.iter().all(|s| self.lib(|l| l.has_tag(&s.loc, &tg.id))) && !sel.is_empty();
                        let mut x = mi("tag", tg.name.clone(), Act::ToggleTag(t.clone(), tg.id.clone())).on(on);
                        x.dot = Some(hex(&tg.color));
                        x
                    })
                    .chain([mi("add", "New tag\u{2026}", Act::Modal(Modal::NewTag)).gap()])
                    .collect();
                let mut v = vec![
                    mi(if is_dir { "folder-open" } else { "arrow-right-up" }, if multi { format!("Open {} items", sel.len()) } else { "Open".into() }, Act::OpenSel).k("Enter"),
                    mi("widget", "Open With", Act::Toast(String::new())).sub(apps).off(is_dir || multi),
                    mi("eye", "Quick Look", Act::Preview(t.clone())).k("Space").off(!prev || multi),
                    mi("info-circle", "Get Info", Act::Info).k("Ctrl+I"),
                    mi("share", "Send to\u{2026}", Act::Modal(Modal::OpenWith(t.clone()))).off(is_dir || multi),
                    mi("copy", "Copy", Act::Copy).k("Ctrl+C").gap(),
                    mi("scissors", "Cut", Act::Cut).k("Ctrl+X"),
                    mi("copy", "Copy to\u{2026}", Act::CopyTo(false)),
                    mi("folder", "Move to\u{2026}", Act::CopyTo(true)),
                    mi("posts-carousel-horizontal", "Duplicate", Act::Duplicate).k("Ctrl+D"),
                    mi("pen", "Rename", Act::Rename(t.clone())).k("F2").gap().off(multi),
                    mi("tag", "Tags", Act::Toast(String::new())).sub(tags),
                    mi("link", "Copy path", Act::CopyPath),
                ];
                if is_dir && !multi {
                    let pinned = self.lib(|l| l.is_pinned(&t));
                    v.push(mi("bookmark", if pinned { "Remove from sidebar" } else { "Pin to sidebar" }, Act::TogglePin(t.clone())));
                }
                if !IS_WEB && !self.be.demo {
                    v.push(mi("folder-open", "Show in file manager", Act::Reveal));
                }
                v.push(mi("trash-bin-minimalistic", "Move to Trash", Act::Trash).k("Del").gap().danger());
                v
            }
            MenuKind::Background => vec![
                mi("add-folder", "New folder", Act::NewFolder).k("Ctrl+Shift+N"),
                mi("file", "New text file", Act::NewFile),
                mi("clipboard", "Paste", Act::Paste).k("Ctrl+V").off(self.clipboard.is_none() || self.current_loc().is_none()),
                mi("sort-from-top-to-bottom", "Sort by", Act::Toast(String::new())).sub(self.sort_items()).gap(),
                mi("widget", "View", Act::Toast(String::new())).sub(vec![mi("widget", "As grid", Act::SetGrid(true)).on(st.grid).k("G"), mi("list-check", "As list", Act::SetGrid(false)).on(!st.grid).k("L")]),
                mi("eye", "Show hidden files", Act::ToggleHidden).on(st.show_hidden).k("Ctrl+H"),
                mi("checklist-minimalistic", "Select all", Act::SelectAll).k("Ctrl+A").gap(),
                mi("info-circle", "Get Info", Act::Info),
                mi("refresh", "Refresh", Act::Refresh).k("F5"),
            ],
            MenuKind::Place => {
                let t = one.unwrap();
                let pinned = self.lib(|l| l.is_pinned(&t));
                let mut v = vec![mi("folder-open", "Open", Act::Nav(Route::Loc(t.clone())))];
                if self.be.registry.get(&t.fs).and_then(|f| f.real_path(&t.path)).is_some() && !IS_WEB {
                    v.push(mi("folder-open", "Show in file manager", Act::Reveal));
                }
                if pinned {
                    v.push(mi("bookmark", "Remove from sidebar", Act::TogglePin(t.clone())));
                }
                v.push(mi("link", "Copy path", Act::CopyPath));
                v
            }
            MenuKind::Device => {
                let id = m.device.clone().unwrap_or_default();
                let d = self.devices.iter().find(|d| d.id == id).cloned();
                let mut v = vec![];
                if let Some(d) = d {
                    if d.mounted {
                        if let Some(l) = &d.loc {
                            v.push(mi("folder-open", "Open", Act::Nav(Route::Loc(l.clone()))));
                        }
                        if d.can_unmount {
                            v.push(mi("upload-minimalistic", "Unmount", Act::Unmount(id.clone())).gap());
                        }
                        if d.can_eject {
                            v.push(mi("upload-minimalistic", "Eject", Act::Eject(id.clone())));
                        }
                    } else {
                        v.push(mi("plug-circle", "Mount", Act::Mount(id.clone())));
                    }
                    v.push(mi("info-circle", "Devices & locations\u{2026}", Act::Modal(Modal::Devices)).gap());
                }
                v
            }
            MenuKind::Sort => self.sort_items(),
            MenuKind::Workspace => vec![
                mi("check-circle", format!("{} \u{b7} Personal", self.user_name()), Act::Toast("Personal workspace".into())).on(true),
                mi("cloud", "Add cloud account\u{2026}", Act::Modal(Modal::Connect)).gap(),
                mi("ssd-square", "Devices & locations\u{2026}", Act::Modal(Modal::Devices)),
                mi("smartphone", "Phone continuity\u{2026}", Act::Modal(Modal::Phone)),
                mi("stars-minimalistic", "About Smart Files", Act::Modal(Modal::About)).gap(),
            ],
            MenuKind::More => vec![
                mi("sidebar-minimalistic", "Toggle sidebar", Act::ToggleSidebar).k("Ctrl+\\"),
                mi("stars-minimalistic", if st.dark { "Switch to light" } else { "Switch to dark" }, Act::ToggleTheme).k("D"),
                mi("eye", "Show hidden files", Act::ToggleHidden).on(st.show_hidden).k("Ctrl+H"),
                mi("folder", "Folders first", Act::ToggleFoldersFirst).on(st.folders_first),
                mi("gallery-wide", "Desktop wallpaper\u{2026}", Act::Modal(Modal::Wallpaper)).gap(),
                mi("widget", "Design tokens", Act::Modal(Modal::Tokens)).k("T"),
                mi("magnifer-zoom-in", "8 px grid overlay", Act::ToggleGridOverlay).on(self.grid_overlay),
                mi("ssd-square", "Devices & locations\u{2026}", Act::Modal(Modal::Devices)).gap(),
                mi("smartphone", "Phone continuity\u{2026}", Act::Modal(Modal::Phone)),
                mi("cloud", "Connect to server\u{2026}", Act::Modal(Modal::Connect)),
                mi("list-check", "Keyboard shortcuts", Act::Modal(Modal::Shortcuts)).gap().k("?"),
                mi("info-circle", "About Smart Files", Act::Modal(Modal::About)),
            ],
        }
    }

    fn sort_items(&self) -> Vec<MI> {
        let st = self.st();
        let mut v: Vec<MI> = SortKey::ALL.iter().map(|k| mi(match k { SortKey::Name => "text", SortKey::Modified => "clock-circle", SortKey::Size => "database", SortKey::Kind => "tag" }, k.label(), Act::SetSort(*k)).on(st.sort == *k)).collect();
        v.push(mi("sort-vertical", if st.ascending { "Descending order" } else { "Ascending order" }, Act::ToggleAsc).gap());
        v.push(mi("folder", "Folders first", Act::ToggleFoldersFirst).on(st.folders_first));
        v
    }

    fn menu_height(items: &[MI]) -> f32 {
        14.0 + items.iter().map(|i| 33.0 + if i.gap { 9.0 } else { 0.0 }).sum::<f32>()
    }

    fn paint_menu(&mut self, ui: &mut Ui, pos: egui::Pos2, items: &[MI], width: f32, id: &str, sub_open: Option<usize>) -> (Rect, Option<usize>, Option<Act>, Option<(usize, Rect)>) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let h = Self::menu_height(items);
        let r = Rect::from_min_size(pos, vec2(width, h));
        elevate(&p, &th, r, 18.0, 4);
        grad_fill(&p, r, 18.0, th.menu, mix(th.menu, th.panel_bottom, 0.5));
        rim(&p, r, 18.0, 1.3, th.rim, true, 40.0);
        ui.allocate_rect(r, Sense::hover());
        let mut y = r.top() + 7.0;
        let mut act = None;
        let mut hover_sub = None;
        let mut hovered_idx = None;
        let mut sub_rect = None;
        for (i, it) in items.iter().enumerate() {
            if it.gap {
                y += 9.0;
            }
            let row = Rect::from_min_size(pos2(r.left() + 7.0, y), vec2(width - 14.0, 33.0));
            let resp = ui.interact(row, Id::unique((id, i)), if it.disabled { Sense::hover() } else { Sense::click() });
            let hov = !it.disabled && (resp.hovered() || sub_open == Some(i));
            if hov {
                hovered_idx = Some(i);
                let tint = if it.danger { th.danger_tint } else { th.row_press };
                p.rect_filled(row, cr(9.0), tint);
            }
            let col = if it.disabled { alpha(th.ink2, 0.4) } else if it.danger { th.danger } else { th.ink1 };
            let ic = row.left() + 12.0 + 8.0;
            if let Some(dot) = it.dot {
                grad_fill(&p, Rect::from_center_size(pos2(ic, row.center().y), vec2(9.0, 9.0)), 4.5, mix(dot, Color32::WHITE, 0.25), dot);
            } else {
                self.ico(&p, it.icon, if it.danger || hov { Ic::Filled } else { Ic::Duo }, pos2(ic, row.center().y), 16.0, if it.disabled { alpha(th.ink3, 0.4) } else if it.danger { th.danger } else { th.ink3 });
            }
            p.text(pos2(row.left() + 38.0, row.center().y), Align2::LEFT_CENTER, &it.label, f_medium(T_BODY), col);
            if !it.sub.is_empty() {
                let c = pos2(row.right() - 14.0, row.center().y);
                let s = egui::Stroke::new(1.5, th.ink4);
                p.line_segment([c + vec2(-1.5, -3.5), c + vec2(2.0, 0.0)], s);
                p.line_segment([c + vec2(2.0, 0.0), c + vec2(-1.5, 3.5)], s);
                if hov {
                    hover_sub = Some(i);
                    sub_rect = Some((i, row));
                }
            } else if it.checked {
                self.ico(&p, "check-circle", Ic::Filled, pos2(row.right() - 18.0, row.center().y), 15.0, th.accent);
            } else if !it.kbd.is_empty() {
                p.text(pos2(row.right() - 12.0, row.center().y), Align2::RIGHT_CENTER, it.kbd, f_medium(T_META), th.ink4);
            }
            if resp.clicked() && it.act.is_some() {
                act = it.act.clone();
            }
            y += 33.0;
        }
        let _ = hovered_idx;
        (r, hover_sub, act, sub_rect)
    }

    pub fn draw_menu(&mut self, ctx: &Context) {
        let Some(mut m) = self.menu.clone() else { return };
        let screen = ctx.content_rect();
        let items = self.menu_items(&m);
        if items.is_empty() {
            self.menu = None;
            return;
        }
        let h = Self::menu_height(&items);
        let mut pos = m.pos;
        if matches!(m.kind, MenuKind::More | MenuKind::Sort) {
            pos.x -= 252.0;
        }
        pos.x = pos.x.min(screen.right() - 252.0 - 10.0).max(screen.left() + 8.0);
        pos.y = if pos.y + h > screen.bottom() - 10.0 { (screen.bottom() - h - 10.0).max(8.0) } else { pos.y };
        let mut chosen: Option<Act> = None;
        let mut inside = false;
        let sub = m.sub;
        egui::Area::new(Id::unique("ctx-menu")).order(Order::Foreground).fixed_pos(screen.min).interactable(true).show(ctx, |ui| {
            ui.set_clip_rect(screen);
            let bgr = ui.allocate_rect(screen, Sense::click());
            let (r, hs, act, sub_rect) = self.paint_menu(ui, pos, &items, 252.0, "m", sub);
            inside |= ui.rect_contains_pointer(r);
            chosen = act;
            let mut new_sub = hs.or_else(|| if inside { None } else { sub });
            if let (Some(si), Some((_, row))) = (new_sub.or(sub), sub_rect.or_else(|| None)) {
                let _ = (si, row);
            }
            // keep a submenu open while the pointer is inside it
            let open = hs.or(sub);
            if let Some(si) = open {
                let sub_items = &items[si].sub;
                if !sub_items.is_empty() {
                    let sh = Self::menu_height(sub_items);
                    let row_y = r.top() + 7.0 + items.iter().take(si).map(|i| 33.0 + if i.gap { 9.0 } else { 0.0 }).sum::<f32>() + if items[si].gap { 9.0 } else { 0.0 };
                    let mut sp = pos2(r.right() + 4.0, row_y - 7.0);
                    if sp.x + 214.0 > screen.right() - 8.0 {
                        sp.x = r.left() - 214.0 - 4.0;
                    }
                    sp.y = sp.y.min(screen.bottom() - sh - 10.0).max(8.0);
                    let (sr, _, sact, _) = self.paint_menu(ui, sp, sub_items, 214.0, "ms", None);
                    if sact.is_some() {
                        chosen = sact;
                    }
                    let in_sub = ui.rect_contains_pointer(sr.expand2(vec2(6.0, 0.0)));
                    let on_row = ui.rect_contains_pointer(Rect::from_min_size(pos2(r.left(), row_y), vec2(252.0, 33.0)));
                    new_sub = if hs.is_some() || in_sub || (on_row && sub.is_some()) { Some(si) } else { None };
                }
            }
            m.sub = new_sub;
            if bgr.clicked() || bgr.secondary_clicked() {
                if !inside {
                    chosen = Some(Act::Toast("\u{0}".into()));
                }
            }
        });
        if let Some(a) = chosen {
            self.menu = None;
            if !matches!(&a, Act::Toast(t) if t.is_empty() || t == "\u{0}") {
                self.acts.push(a);
            }
        } else if let Some(cur) = self.menu.as_mut() {
            cur.sub = m.sub;
        }
    }

    // ───────────── toasts + drag ghost + jobs ─────────────
    pub fn draw_toasts(&mut self, ctx: &Context) {
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        for t in &mut self.toasts {
            t.age += dt;
        }
        self.toasts.retain(|t| t.age < 3.6);
        for j in &mut self.jobs {
            j.age += dt;
        }
        self.jobs.retain(|j| j.state == JobState::Running || j.age < 0.1);
        if self.toasts.is_empty() {
            return;
        }
        ctx.request_repaint();
        let win = win_rect();
        let th = self.th.clone();
        let mut y = win.bottom() - STATUS_H - 18.0;
        egui::Area::new(Id::unique("toasts")).order(Order::Tooltip).fixed_pos(win.min).interactable(false).show(ctx, |ui| {
            let p = ui.painter().clone();
            for t in self.toasts.iter().rev() {
                let a = (1.0 - ((t.age - 3.0) / 0.6).clamp(0.0, 1.0)).min((t.age / 0.18).clamp(0.0, 1.0));
                let f = f_semi(T_BODY);
                let w = (self.tw(&p, &t.text, &f) + 62.0).min(win.width() - 40.0);
                let r = Rect::from_center_size(pos2(win.center().x + 126.0 * self.sidebar_open, y - 20.0 + (1.0 - a) * 10.0), vec2(w, 40.0));
                let sh = |c: Color32| alpha(c, a);
                shadow(&p, r, 20.0, 12.0, 30.0, -8.0, sh(alpha(th.shadow, if th.dark { 0.7 } else { 0.35 })));
                grad_fill(&p, r, 20.0, sh(if th.dark { c(40, 50, 70, 0.98) } else { c(28, 38, 58, 0.97) }), sh(if th.dark { c(26, 34, 50, 0.98) } else { c(20, 28, 44, 0.97) }));
                rim(&p, r, 20.0, 1.2, sh(c(255, 255, 255, 0.28)), true, 24.0);
                self.ico(&p, if t.ok { "check-circle" } else { "info-circle" }, Ic::Filled, pos2(r.left() + 22.0, r.center().y), 16.0, sh(if t.ok { hex("#59c79a") } else { hex("#ff7a84") }));
                let s = self.fit(&p, &t.text, &f, w - 62.0);
                p.text(pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, s, f, sh(hex("#f4f7fd")));
                y -= 48.0;
            }
        });
    }

    pub fn draw_drag_ghost(&mut self, ctx: &Context) {
        let Some(d) = &self.drag else { return };
        let Some(pos) = ctx.pointer_latest_pos() else { return };
        ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
        let th = self.th.clone();
        let label = d.label.clone();
        let n = d.items.len();
        egui::Area::new(Id::unique("drag-ghost")).order(Order::Tooltip).fixed_pos(pos + vec2(14.0, 10.0)).interactable(false).show(ctx, |ui| {
            let p = ui.painter().clone();
            let f = f_semi(T_BODY);
            let w = self.tw(&p, &label, &f).min(220.0) + 44.0;
            let r = Rect::from_min_size(pos + vec2(14.0, 10.0), vec2(w, 34.0));
            elevate(&p, &th, r, 17.0, 3);
            grad_fill(&p, r, 17.0, th.panel_top, th.panel_bottom);
            rim(&p, r, 17.0, 1.2, th.rim, true, 20.0);
            self.ico(&p, "copy", Ic::Duo, pos2(r.left() + 20.0, r.center().y), 15.0, th.accent);
            let s = self.fit(&p, &label, &f, 220.0);
            p.text(pos2(r.left() + 36.0, r.center().y), Align2::LEFT_CENTER, s, f, th.ink1);
            if n > 1 {
                let b = Rect::from_center_size(r.right_top() + vec2(-4.0, 4.0), vec2(20.0, 20.0));
                grad_fill(&p, b, 10.0, th.accent, th.accent_press);
                p.text(b.center(), Align2::CENTER_CENTER, n.to_string(), f_bold(10.5), th.on_accent());
            }
        });
        if ctx.input(|i| i.pointer.any_released()) {
            // released over nothing droppable → cancel
            self.acts.push(Act::Toast("\u{1}".into()));
        }
    }

    pub fn draw_jobs_card(&mut self, ctx: &Context) {
        let running: Vec<(u64, String, f32, String)> = self.jobs.iter().filter(|j| j.state == JobState::Running).map(|j| (j.id, j.title.clone(), j.done as f32 / j.total.max(1) as f32, j.current.clone())).collect();
        if running.is_empty() {
            return;
        }
        let win = win_rect();
        let th = self.th.clone();
        let mut cancel = None;
        egui::Area::new(Id::unique("jobs-card")).order(Order::Foreground).fixed_pos(win.min).show(ctx, |ui| {
            let p = ui.painter().clone();
            let mut y = win.bottom() - STATUS_H - 16.0;
            for (id, title, frac, cur) in running.iter().rev() {
                let r = Rect::from_min_size(pos2(win.right() - 16.0 - 330.0, y - 74.0), vec2(330.0, 74.0));
                elevate(&p, &th, r, 18.0, 4);
                grad_fill(&p, r, 18.0, th.panel_top, th.panel_bottom);
                rim(&p, r, 18.0, 1.3, th.rim, true, 40.0);
                self.ico(&p, "copy", Ic::Filled, pos2(r.left() + 26.0, r.top() + 24.0), 18.0, th.accent);
                let t = self.fit(&p, title, &f_semi(T_BODY), 230.0);
                p.text(pos2(r.left() + 46.0, r.top() + 24.0), Align2::LEFT_CENTER, t, f_semi(T_BODY), th.ink1);
                let c = Rect::from_center_size(pos2(r.right() - 24.0, r.top() + 24.0), vec2(26.0, 26.0));
                let cr_ = ui.interact(c, Id::unique(("cancel-job", id)), Sense::click());
                self.ico(&p, "close-circle", if cr_.hovered() { Ic::Filled } else { Ic::Duo }, c.center(), 18.0, th.ink3);
                if cr_.clicked() {
                    cancel = Some(*id);
                }
                let bar = Rect::from_min_size(pos2(r.left() + 18.0, r.top() + 46.0), vec2(r.width() - 36.0, 6.0));
                well(&p, &th, bar, 3.0);
                let (a, b) = th.accent_grad();
                grad_fill(&p, Rect::from_min_size(bar.min, vec2((bar.width() * frac).max(6.0), 6.0)), 3.0, a, b);
                let cs = self.fit(&p, cur, &f_medium(T_META), 200.0);
                p.text(pos2(r.left() + 18.0, r.top() + 62.0), Align2::LEFT_CENTER, cs, f_medium(T_META), th.ink3);
                p.text(pos2(r.right() - 18.0, r.top() + 62.0), Align2::RIGHT_CENTER, format!("{:.0}%", frac * 100.0), f_semi(T_META), th.ink2);
                y -= 84.0;
            }
        });
        if let Some(id) = cancel {
            self.be.cancel_job(id);
        }
    }

    // ───────────── command palette ─────────────
    fn palette_items(&self) -> Vec<(String, Vec<(&'static str, String, String, Act, &'static str)>)> {
        // (group, [(icon, title, sub, act, kbd)])
        let q = self.palette.query.trim().to_lowercase();
        let st = self.st();
        let mut cmds: Vec<(&'static str, String, String, Act, &'static str)> = vec![
            ("add-folder", "New folder".into(), "Create an untitled folder here".into(), Act::NewFolder, "Ctrl+Shift+N"),
            ("stars-minimalistic", "Ask Smart".into(), "Search and tidy with on-device help".into(), Act::Ask, ""),
            ("sidebar-minimalistic", if self.sidebar_open > 0.5 { "Hide sidebar" } else { "Show sidebar" }.into(), "Layout".into(), Act::ToggleSidebar, "Ctrl+\\"),
            ("widget", "Grid view".into(), "Large media, adaptive columns".into(), Act::SetGrid(true), "G"),
            ("list-check", "List view".into(), "Dense rows with kinds and dates".into(), Act::SetGrid(false), "L"),
            ("stars-minimalistic", if st.dark { "Switch to light" } else { "Switch to dark" }.into(), "Appearance".into(), Act::ToggleTheme, "D"),
            ("eye", "Toggle hidden files".into(), "Show or hide dotfiles".into(), Act::ToggleHidden, "Ctrl+H"),
            ("ssd-square", "Devices & locations".into(), "Mount, unmount, eject".into(), Act::Modal(Modal::Devices), ""),
            ("smartphone", "Phone continuity".into(), "Browse and send to your phone".into(), Act::Modal(Modal::Phone), ""),
            ("cloud", "Connect to server".into(), "SMB, SFTP, WebDAV, MTP".into(), Act::Modal(Modal::Connect), ""),
            ("gallery-wide", "Change wallpaper".into(), "Surfaces behind the window".into(), Act::Modal(Modal::Wallpaper), ""),
            ("widget", "Open design tokens".into(), "smart.toml \u{b7} draft 0.1".into(), Act::Modal(Modal::Tokens), "T"),
            ("copy", "Copy design tokens".into(), "Put smart.toml on the clipboard".into(), Act::CopyTokens, ""),
            ("magnifer-zoom-in", "8 px grid overlay".into(), "Craft".into(), Act::ToggleGridOverlay, ""),
            ("list-check", "Keyboard shortcuts".into(), "Every binding in one sheet".into(), Act::Modal(Modal::Shortcuts), "?"),
            ("info-circle", "About Smart Files".into(), "0.1 \u{b7} built on smart-core".into(), Act::Modal(Modal::About), ""),
            ("refresh", "Refresh".into(), "Reload this folder".into(), Act::Refresh, "F5"),
        ];
        let mut nav: Vec<(&'static str, String, String, Act, &'static str)> = vec![];
        for p in &self.places {
            nav.push((p.icon, format!("Open {}", p.title), self.pretty_path(&p.loc), Act::Nav(Route::Loc(p.loc.clone())), ""));
        }
        nav.push(("clock-circle", "Open Recents".into(), "Everything you touched lately".into(), Act::Nav(Route::Recents), ""));
        for d in &self.devices {
            if let (Some(l), true) = (&d.loc, d.mounted) {
                nav.push((d.kind.icon(), format!("Open {}", d.name), d.capacity_text(), Act::Nav(Route::Loc(l.clone())), ""));
            }
        }
        for t in self.lib(|l| l.tags.clone()) {
            nav.push(("tag", format!("Open {} tag", t.name), format!("{} items", self.lib(|l| l.tag_count(&t.id))), Act::Nav(Route::Tag(t.id.clone())), ""));
        }
        let sel = self.selected();
        let mut selection: Vec<(&'static str, String, String, Act, &'static str)> = vec![];
        if !sel.is_empty() {
            selection.push(("info-circle", "Get Info on the selection".into(), format!("{} selected", sel.len()), Act::Info, "Ctrl+I"));
            selection.push(("pen", "Rename the selection".into(), "Edit the name in place".into(), Act::Rename(sel[0].loc.clone()), "F2"));
            selection.push(("copy", "Copy to\u{2026}".into(), "Pick a destination".into(), Act::CopyTo(false), ""));
            selection.push(("folder", "Move to\u{2026}".into(), "Relocate without copy-paste".into(), Act::CopyTo(true), ""));
            selection.push(("trash-bin-minimalistic", "Move to Trash".into(), "Recoverable".into(), Act::Trash, "Del"));
        }
        let score = |s: &str| -> Option<i32> {
            let s = s.to_lowercase();
            if q.is_empty() {
                return Some(0);
            }
            if let Some(i) = s.find(&q) {
                return Some(1000 - i as i32);
            }
            let mut it = s.chars();
            let mut n = 0;
            for qc in q.chars() {
                n += 1;
                it.find(|c| *c == qc)?;
            }
            Some(100 - n)
        };
        let filt = |v: Vec<(&'static str, String, String, Act, &'static str)>, limit: usize| -> Vec<(&'static str, String, String, Act, &'static str)> {
            let mut s: Vec<_> = v.into_iter().filter_map(|x| score(&format!("{} {}", x.1, x.2)).map(|sc| (sc, x))).collect();
            s.sort_by_key(|x| -x.0);
            s.into_iter().take(limit).map(|x| x.1).collect()
        };
        let mut out = vec![];
        let sug = filt(selection, 4);
        if !sug.is_empty() {
            out.push(("Selection".to_string(), sug));
        }
        let c = filt(cmds.drain(..).collect(), if q.is_empty() { 6 } else { 6 });
        if !c.is_empty() {
            out.push((if q.is_empty() { "Suggested" } else { "Commands" }.to_string(), c));
        }
        let n = filt(nav, if q.is_empty() { 5 } else { 5 });
        if !n.is_empty() {
            out.push(("Navigate".to_string(), n));
        }
        if !q.is_empty() {
            let files: Vec<_> = self.palette.hits.iter().take(6).map(|h| (h.entry.kind.icon(), h.entry.name.clone(), h.entry.loc.parent().map(|p| self.pretty_path(&p)).unwrap_or_default(), if h.entry.is_dir { Act::Nav(Route::Loc(h.entry.loc.clone())) } else { Act::Open(h.entry.loc.clone()) }, "")).collect();
            if !files.is_empty() {
                out.push(("Files".to_string(), files));
            }
            out.push(("Search".to_string(), vec![("magnifer", format!("Search everywhere for \u{201c}{}\u{201d}", self.palette.query.trim()), "Fuzzy name search \u{b7} add kind: size: modified: tag: filters".into(), Act::RunSearch(self.palette.query.trim().to_string()), "Enter")]));
        }
        out
    }

    pub fn draw_palette(&mut self, ctx: &Context) {
        if !self.palette.open {
            return;
        }
        let win = win_rect();
        let th = self.th.clone();
        // live file search
        let q = self.palette.query.trim().to_string();
        if q != self.palette.last_sent {
            self.palette.last_sent = q.clone();
            self.palette.hits.clear();
            if q.len() >= 2 {
                let home = self.be.home();
                self.palette.req = self.be.search(&q, &home, false);
            }
        }
        let groups = self.palette_items();
        let flat: Vec<(&'static str, String, String, Act, &'static str)> = groups.iter().flat_map(|g| g.1.clone()).collect();
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.palette.sel = (self.palette.sel + 1).min(flat.len().saturating_sub(1));
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.palette.sel = self.palette.sel.saturating_sub(1);
        }
        self.palette.sel = self.palette.sel.min(flat.len().saturating_sub(1));
        let enter = ctx.input(|i| i.key_pressed(egui::Key::Enter));
        let mut run: Option<Act> = None;
        let mut close = false;
        let w = 640.0f32.min(win.width() - 40.0);
        let n_groups = groups.len() as f32;
        let h = (72.0 + n_groups * 30.0 + flat.len() as f32 * 44.0 + 52.0).clamp(220.0, win.height() - 110.0);
        egui::Area::new(Id::unique("palette")).order(Order::Foreground).fixed_pos(win.min).show(ctx, |ui| {
            let p = ui.painter().clone();
            ui.set_clip_rect(win);
            let scrim = ui.allocate_rect(win, Sense::click());
            grad_fill(&p, win, R_WIN, th.scrim, th.scrim);
            if scrim.clicked() {
                close = true;
            }
            let card = Rect::from_min_size(pos2(win.center().x - w / 2.0, win.top() + 84.0), vec2(w, h));
            elevate(&p, &th, card, 22.0, 5);
            grad_fill(&p, card, 22.0, th.panel_top, th.panel_bottom);
            rim(&p, card, 22.0, 1.4, th.rim, true, 50.0);
            ui.allocate_rect(card, Sense::click());
            // input
            let inp = Rect::from_min_size(card.min + vec2(14.0, 14.0), vec2(w - 28.0, 44.0));
            well(&p, &th, inp, 22.0);
            self.ico(&p, "magnifer", Ic::Duo, pos2(inp.left() + 24.0, inp.center().y), 18.0, th.accent_ink);
            let mut qq = std::mem::take(&mut self.palette.query);
            let te = ui.put(Rect::from_min_max(inp.min + vec2(46.0, 8.0), inp.max - vec2(60.0, 8.0)), egui::TextEdit::singleline(&mut qq).id(Id::unique("palette-input")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("Type a command, jump to a place, or search files\u{2026}").color(th.ink4)).font(f_medium(T_TITLE)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
            if self.palette.focus {
                te.request_focus();
                self.palette.focus = false;
            }
            if self.palette.query != qq {
                self.palette.sel = 0;
            }
            self.palette.query = qq;
            self.keycap(&p, pos2(inp.right() - 14.0, inp.center().y), "Esc", false);
            // results
            let mut y = inp.bottom() + 10.0;
            let mut idx = 0usize;
            let scroll_rect = Rect::from_min_max(pos2(card.left(), y), pos2(card.right(), card.bottom() - 44.0));
            let mut sui = ui.new_child(egui::UiBuilder::new().max_rect(scroll_rect).layout(egui::Layout::top_down(egui::Align::Min)));
            sui.set_clip_rect(scroll_rect.intersect(ui.clip_rect()));
            let sel_i = self.palette.sel;
            egui::ScrollArea::vertical().id_salt("palette-scroll").auto_shrink([false, false]).show(&mut sui, |ui| {
                let top = ui.max_rect().top();
                let mut yy = top;
                ui.allocate_rect(Rect::from_min_size(pos2(card.left(), top), vec2(w, n_groups * 30.0 + flat.len() as f32 * 44.0 + 8.0)), Sense::hover());
                for (gname, items) in &groups {
                    p.text(pos2(card.left() + 26.0, yy + 15.0), Align2::LEFT_CENTER, gname.to_uppercase(), f_semi(T_LABEL), th.ink4);
                    yy += 30.0;
                    for (icon, title, sub, act, kbd) in items {
                        let row = Rect::from_min_size(pos2(card.left() + 10.0, yy), vec2(w - 20.0, 42.0));
                        let r = ui.interact(row, Id::unique(("pal", idx)), Sense::click());
                        let on = idx == sel_i;
                        if on {
                            grad_fill(&p, row, 12.0, th.accent_tint, th.accent_tint2);
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::ArrowUp)) {
                                ui.scroll_to_rect(row, None);
                            }
                        } else if r.hovered() {
                            p.rect_filled(row, cr(12.0), th.row_hover);
                        }
                        let ic = Rect::from_center_size(pos2(row.left() + 26.0, row.center().y), vec2(28.0, 28.0));
                        grad_fill(&p, ic, 9.0, alpha(th.accent, if on { 0.22 } else { 0.10 }), alpha(th.accent, if on { 0.14 } else { 0.05 }));
                        self.ico(&p, icon, if on { Ic::Filled } else { Ic::Duo }, ic.center(), 15.0, if on { th.accent_ink } else { th.ink2 });
                        p.text(pos2(row.left() + 52.0, row.center().y - 7.0), Align2::LEFT_CENTER, self.fit(&p, title, &f_semi(T_BODY), w - 240.0), f_semi(T_BODY), th.ink1);
                        p.text(pos2(row.left() + 52.0, row.center().y + 9.0), Align2::LEFT_CENTER, self.fit(&p, sub, &f_medium(T_META), w - 240.0), f_medium(T_META), th.ink3);
                        if !kbd.is_empty() {
                            self.keycap(&p, pos2(row.right() - 12.0, row.center().y), kbd, false);
                        }
                        if r.clicked() || (on && enter) {
                            run = Some(act.clone());
                        }
                        if r.hovered() && ui.input(|i| i.pointer.delta().length() > 0.5) {
                            self.palette.sel = idx;
                        }
                        yy += 44.0;
                        idx += 1;
                    }
                }
                if flat.is_empty() {
                    p.text(pos2(card.center().x, top + 60.0), Align2::CENTER_CENTER, "No matching commands", f_medium(T_BODY), th.ink3);
                }
            });
            y = card.bottom() - 22.0;
            for (i, (k, t)) in [("\u{2191}\u{2193}", "navigate"), ("Enter", "run"), ("Esc", "close")].into_iter().enumerate() {
                let x = card.left() + 22.0 + i as f32 * 92.0;
                let kw = self.keycap(&p, pos2(x + 28.0, y), k, false);
                let _ = kw;
                p.text(pos2(x + 34.0, y), Align2::LEFT_CENTER, t, f_medium(T_META), th.ink4);
            }
            if !self.palette.query.is_empty() {
                p.text(pos2(card.right() - 20.0, y), Align2::RIGHT_CENTER, format!("{} indexed files \u{b7} {}", self.be.indexed_count(), if self.search.cached { "instant" } else { "live" }), f_medium(T_META), th.ink4);
            }
        });
        if close || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.palette.open = false;
        }
        if let Some(a) = run {
            self.palette.open = false;
            self.acts.push(a);
        }
    }

    // ───────────── Ask Smart panel ─────────────
    pub fn draw_ai(&mut self, ctx: &Context) {
        let dt = ctx.input(|i| i.stable_dt).min(0.05);
        let target = if self.ai.open { 1.0 } else { 0.0 };
        self.ai.anim += (target - self.ai.anim) * (1.0 - (-dt * 18.0).exp());
        if (target - self.ai.anim).abs() > 0.002 {
            ctx.request_repaint();
        }
        if self.ai.anim < 0.01 {
            return;
        }
        let win = win_rect();
        let th = self.th.clone();
        let a = self.ai.anim;
        let w = 394.0f32.min(win.width() - 40.0);
        let panel = Rect::from_min_max(pos2(win.right() - 10.0 - w + (1.0 - a) * 40.0, win.top() + 68.0), pos2(win.right() - 10.0 + (1.0 - a) * 40.0, win.bottom() - 44.0));
        let mut act: Vec<Act> = vec![];
        let mut send = false;
        egui::Area::new(Id::unique("ai-panel")).order(Order::Foreground).fixed_pos(win.min).show(ctx, |ui| {
            let p = ui.painter().clone();
            ui.set_clip_rect(win.expand(80.0));
            let fa = |c: Color32| alpha(c, a);
            elevate(&p, &th, panel, 20.0, 4);
            grad_fill(&p, panel, 20.0, fa(th.panel_top), fa(th.panel_bottom));
            rim(&p, panel, 20.0, 1.3, fa(th.rim), true, 40.0);
            ui.allocate_rect(panel, Sense::click());
            // header
            let orb = Rect::from_min_size(panel.min + vec2(14.0, 14.0), vec2(38.0, 38.0));
            let (c0, c1) = th.accent_grad();
            shadow(&p, orb, 12.0, 4.0, 12.0, -2.0, alpha(th.accent, 0.4));
            grad_fill(&p, orb, 12.0, c0, c1);
            rim(&p, orb, 12.0, 1.2, Color32::from_white_alpha(140), true, 20.0);
            self.ico(&p, "stars-minimalistic", Ic::Filled, orb.center(), 20.0, th.on_accent());
            p.text(pos2(orb.right() + 12.0, orb.center().y - 8.0), Align2::LEFT_CENTER, "Ask Smart", f_bold(T_TITLE), th.ink1);
            let loc = self.title();
            p.circle_filled(pos2(orb.right() + 17.0, orb.center().y + 9.0), 3.0, th.good);
            p.text(pos2(orb.right() + 26.0, orb.center().y + 9.0), Align2::LEFT_CENTER, format!("On-device \u{b7} aware of {}", loc), f_medium(T_META), th.ink3);
            let close = Rect::from_center_size(pos2(panel.right() - 28.0, orb.center().y), vec2(30.0, 30.0));
            let cr_ = ui.interact(close, Id::unique("ai-close"), Sense::click());
            self.ico(&p, "close-circle", if cr_.hovered() { Ic::Filled } else { Ic::Duo }, close.center(), 18.0, th.ink3);
            if cr_.clicked() {
                act.push(Act::Ask);
            }
            // body (scrolling conversation)
            let body = Rect::from_min_max(pos2(panel.left(), orb.bottom() + 10.0), pos2(panel.right(), panel.bottom() - 112.0));
            let mut bui = ui.new_child(egui::UiBuilder::new().max_rect(body).layout(egui::Layout::top_down(egui::Align::Min)));
            bui.set_clip_rect(body.intersect(ui.clip_rect()));
            let stick = self.ai.msgs.len();
            egui::ScrollArea::vertical().id_salt("ai-scroll").auto_shrink([false, false]).stick_to_bottom(true).show(&mut bui, |ui| {
                let _ = stick;
                let x0 = body.left() + 16.0;
                let mw = body.width() - 32.0;
                let top = ui.max_rect().top();
                let mut y = top + 4.0;
                // intro
                let intro = format!("You\u{2019}re in {}. I can search by kind, date or size, tidy this folder, or find duplicates. What are we making?", loc);
                y += self.bubble(&p, x0, y, mw, &intro, false);
                // awareness card
                let aw = Rect::from_min_size(pos2(x0, y + 4.0), vec2(mw, 112.0));
                grad_fill(&p, aw, 14.0, fa(th.card_top), fa(th.card_bottom));
                elevate(&p, &th, aw, 14.0, 1);
                grad_fill(&p, aw, 14.0, fa(th.card_top), fa(th.card_bottom));
                rim(&p, aw, 14.0, 1.0, fa(th.rim), true, 30.0);
                self.ico(&p, "folder-check", Ic::Duo, pos2(aw.left() + 20.0, aw.top() + 20.0), 14.0, th.accent_ink);
                p.text(pos2(aw.left() + 34.0, aw.top() + 20.0), Align2::LEFT_CENTER, "Awareness \u{b7} current folder", f_semi(T_META), th.ink2);
                let mut tx = aw.left() + 14.0;
                let thumbs: Vec<Entry> = self.items.iter().filter(|e| e.kind == Kind::Image).take(3).cloned().collect();
                for e in &thumbs {
                    let tr = Rect::from_min_size(pos2(tx, aw.top() + 36.0), vec2(52.0, 38.0));
                    if let Some(t) = self.thumbs.get(&e.loc) {
                        let sz = t.tex.size_vec2();
                        textured_rrect(&p, tr, 7.0, t.tex.id(), cover_uv(sz.x, sz.y, tr), Color32::WHITE);
                    } else {
                        grad_fill(&p, tr, 7.0, alpha(th.accent, 0.2), alpha(th.accent, 0.1));
                    }
                    tx += 58.0;
                }
                let old = smart_core::tools::older_than(&self.items, 90).len();
                let stats = format!("{} items \u{b7} {} \u{b7} {} folders \u{b7} {} stale", self.items.len(), format_size(self.total_bytes()), self.items.iter().filter(|e| e.is_dir).count(), old);
                p.text(pos2(aw.left() + 14.0, aw.bottom() - 18.0), Align2::LEFT_CENTER, self.fit(&p, &stats, &f_medium(11.0), mw - 28.0), f_medium(11.0), th.ink3);
                y = aw.bottom() + 12.0;
                // chips
                let mut cx = x0;
                let mut cy = y;
                for (ic, label, a_) in [("folder-check", "Organise this folder", Act::Organise), ("copy", "Find duplicates", Act::FindDuplicates), ("database", "Largest files", Act::FindLarge), ("clock-circle", "Find old files", Act::FindOld)] {
                    let f = f_semi(11.5);
                    let wd = self.tw(&p, label, &f) + 44.0;
                    if cx + wd > x0 + mw {
                        cx = x0;
                        cy += 36.0;
                    }
                    let r = Rect::from_min_size(pos2(cx, cy), vec2(wd, 30.0));
                    let resp = ui.interact(r, Id::unique(("ai-chip", label)), Sense::click());
                    raised(&p, &th, r, 15.0, resp.hovered(), resp.is_pointer_button_down_on());
                    self.ico(&p, ic, Ic::Duo, pos2(r.left() + 19.0, r.center().y), 14.0, th.accent_ink);
                    p.text(pos2(r.left() + 32.0, r.center().y), Align2::LEFT_CENTER, label, f, th.ink1);
                    if resp.clicked() {
                        act.push(a_);
                    }
                    cx += wd + 8.0;
                }
                y = cy + 44.0;
                // conversation
                let msgs: Vec<(bool, String, Vec<Location>, Vec<(String, Act)>)> = self.ai.msgs.iter().map(|m| (m.user, m.text.clone(), m.files.clone(), m.actions.clone())).collect();
                for (mi_, (user, text, files, actions)) in msgs.iter().enumerate() {
                    let h = self.bubble(&p, x0, y, mw, text, *user);
                    y += h;
                    if !files.is_empty() {
                        let mut fx = x0 + 36.0;
                        for l in files.iter().take(6) {
                            let r = Rect::from_min_size(pos2(fx, y), vec2(46.0, 34.0));
                            if let Some(t) = self.thumbs.get(l) {
                                let sz = t.tex.size_vec2();
                                textured_rrect(&p, r, 7.0, t.tex.id(), cover_uv(sz.x, sz.y, r), Color32::WHITE);
                            } else {
                                let k = Kind::from_ext(&ext_of(&l.name()));
                                grad_fill(&p, r, 7.0, alpha(kind_color(k, &th), 0.2), alpha(kind_color(k, &th), 0.1));
                                self.ico(&p, k.icon(), Ic::Duo, r.center(), 15.0, kind_color(k, &th));
                            }
                            fx += 52.0;
                        }
                        y += 42.0;
                    }
                    if !actions.is_empty() {
                        let mut bx = x0 + 36.0;
                        for (label, a_) in actions {
                            let wd = self.tw(&p, label, &f_semi(11.5)) + 28.0;
                            let r = Rect::from_min_size(pos2(bx, y), vec2(wd, 30.0));
                            let resp = ui.interact(r, Id::unique(("ai-act", mi_, label)), Sense::click());
                            raised_accent(&p, &th, r, 15.0, resp.hovered(), resp.is_pointer_button_down_on());
                            p.text(r.center(), Align2::CENTER_CENTER, label, f_semi(11.5), th.on_accent());
                            if resp.clicked() {
                                act.push(a_.clone());
                            }
                            bx += wd + 8.0;
                        }
                        y += 40.0;
                    }
                    y += 4.0;
                }
                ui.allocate_rect(Rect::from_min_max(pos2(body.left(), top), pos2(body.right(), y + 20.0)), Sense::hover());
            });
            // composer
            let comp = Rect::from_min_size(pos2(panel.left() + 14.0, panel.bottom() - 100.0), vec2(panel.width() - 28.0, 78.0));
            well(&p, &th, comp, 16.0);
            let mut inp = std::mem::take(&mut self.ai.input);
            let te = ui.put(Rect::from_min_size(comp.min + vec2(14.0, 8.0), vec2(comp.width() - 28.0, 34.0)), egui::TextEdit::singleline(&mut inp).id(Id::unique("ai-input")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("Ask about these files\u{2026}").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
            self.ai.input = inp;
            if te.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                send = true;
                te.request_focus();
            }
            let chip = Rect::from_min_size(comp.left_bottom() + vec2(12.0, -28.0), vec2(112.0, 22.0));
            grad_fill(&p, chip, 8.0, alpha(th.well, 0.7), alpha(th.well, 0.4));
            self.ico(&p, "widget", Ic::Duo, pos2(chip.left() + 13.0, chip.center().y), 12.0, th.ink3);
            p.text(pos2(chip.left() + 24.0, chip.center().y), Align2::LEFT_CENTER, "Smart Neura \u{b7} local", f_medium(10.5), th.ink3);
            let sb = Rect::from_center_size(pos2(comp.right() - 24.0, comp.bottom() - 19.0), vec2(32.0, 32.0));
            let sr = ui.interact(sb, Id::unique("ai-send"), Sense::click());
            raised_accent(&p, &th, sb, 16.0, sr.hovered(), sr.is_pointer_button_down_on());
            self.ico(&p, "arrow-right-up", Ic::Duo, sb.center(), 16.0, th.on_accent());
            if sr.clicked() {
                send = true;
            }
            p.text(pos2(panel.center().x, panel.bottom() - 10.0), Align2::CENTER_CENTER, "AI actions are previews \u{b7} nothing leaves this device", f_medium(10.5), th.ink4);
        });
        for a in act {
            self.acts.push(a);
        }
        if send {
            self.ai_send();
        }
    }

    fn bubble(&self, p: &egui::Painter, x: f32, y: f32, w: f32, text: &str, user: bool) -> f32 {
        let th = &self.th;
        let f = f_medium(T_BODY);
        let maxw = if user { w - 60.0 } else { w - 36.0 };
        let galley = p.layout(text.to_string(), f.clone(), if user { th.on_accent() } else { th.ink1 }, maxw - 24.0);
        let h = galley.size().y + 22.0;
        let bw = if user { galley.size().x + 24.0 } else { maxw };
        if user {
            let r = Rect::from_min_size(pos2(x + w - bw, y), vec2(bw, h));
            let (a, b) = th.accent_grad();
            raised_accent(p, th, r, 16.0, false, false);
            let _ = (a, b);
            p.galley(r.min + vec2(12.0, 11.0), galley, th.on_accent());
        } else {
            let ic = Rect::from_min_size(pos2(x, y + 2.0), vec2(26.0, 26.0));
            grad_fill(p, ic, 9.0, alpha(th.accent, 0.16), alpha(th.accent, 0.08));
            self.icons.paint_at(p, "stars-minimalistic", Ic::Duo, ic.center(), 14.0, th.accent_ink);
            let r = Rect::from_min_size(pos2(x + 36.0, y), vec2(w - 36.0, h));
            p.galley(r.min + vec2(0.0, 5.0), galley, th.ink1);
        }
        h + 10.0
    }
}
