//! Desktop OS shell for the web build: menubar (top), dock (bottom) and floating app windows
//! with z-order. The file manager is one app in that ecosystem, not the whole screen.
use crate::app::*;
use crate::paint::*;
use crate::theme::*;
use egui::*;
use smart_core::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Focus {
    Files,
    Photos,
    Terminal,
    Transfer,
}

impl App {
    pub fn focused(&self) -> Focus {
        *self.z.last().unwrap_or(&Focus::Files)
    }
    pub fn focus(&mut self, f: Focus) {
        self.z.retain(|x| *x != f);
        self.z.push(f);
    }
    /// The order drawn each frame: insertion order, but only open apps. Recomputed before drawing
    /// and after any pointer interaction so the titlebar and the top window can never disagree.
    fn draw_order(&self) -> Vec<Focus> {
        let mut v: Vec<Focus> = self.z.iter().cloned().filter(|f| self.is_open(*f)).collect();
        if v.is_empty() {
            v.push(Focus::Files);
        }
        v
    }
    pub fn is_open(&self, f: Focus) -> bool {
        match f {
            Focus::Files => !self.win_hidden,
            Focus::Photos => self.photo_open && !self.photo_min,
            Focus::Terminal => self.term_open && !self.term_min,
            Focus::Transfer => self.store.transfer_job().is_some() && !self.tr_min,
        }
    }
    pub fn open_app(&mut self, f: Focus) {
        match f {
            Focus::Files => {
                self.win_hidden = false;
            }
            Focus::Photos => {
                if !self.photo_open {
                    self.photo_open = true;
                    let s = self.shell_bounds();
                    self.photo_win = Rect::from_center_size(s.center() + vec2(70.0, 30.0), vec2((s.width() * 0.62).clamp(560.0, 900.0), (s.height() * 0.78).clamp(420.0, 680.0)));
                }
                self.photo_min = false;
            }
            Focus::Terminal => {
                if !self.term_open {
                    self.term_open = true;
                    let s = self.shell_bounds();
                    self.term_win = Rect::from_center_size(s.center() + vec2(-80.0, 70.0), vec2((s.width() * 0.56).clamp(520.0, 780.0), (s.height() * 0.54).clamp(320.0, 460.0)));
                    if self.term.cwd.is_empty() {
                        self.term.cwd = self.store.current_folder();
                        self.term.lines.push((false, "Smart Terminal — same storage layer as the views. Type `help`.".into()));
                    }
                }
                self.term_min = false;
            }
            Focus::Transfer => self.tr_min = false,
        }
        self.focus(f);
    }
    pub fn minimize(&mut self, f: Focus) {
        match f {
            Focus::Files => self.win_hidden = true,
            Focus::Photos => self.photo_min = true,
            Focus::Terminal => self.term_min = true,
            Focus::Transfer => {
                self.tr_min = true;
                if let Some(j) = self.store.transfer_job() {
                    let id = j.id;
                    if let Some(j) = self.store.eng.job_mut(id) {
                        j.minimized = true;
                    }
                }
            }
        }
        self.z.retain(|x| *x != f);
        if self.z.is_empty() {
            self.z.push(Focus::Files);
            self.win_hidden = false;
        }
    }
    pub fn shell_bounds(&self) -> Rect {
        let s = self.notify_screen;
        Rect::from_min_max(pos2(s.left() + 14.0, s.top() + 34.0), pos2(s.right() - 14.0, s.bottom() - 86.0))
    }

    pub fn win_rect(&self, f: Focus) -> Rect {
        match f {
            Focus::Files => self.win,
            Focus::Photos => self.photo_win,
            Focus::Terminal => self.term_win,
            Focus::Transfer => self.tr_rect,
        }
    }

    fn ensure_windows(&mut self, screen: Rect) {
        self.notify_screen = screen;
        let b = self.shell_bounds();
        // Default desktop: each app gets its own region, so opening one does not look like a
        // half-covered copy of another. Files left, Photos top-right, Terminal bottom-right.
        let right_x = b.left() + (b.width() * 0.615).round();
        let right_w = b.right() - right_x;
        if self.win == Rect::NOTHING || self.win_saved == Rect::NOTHING {
            self.win = Rect::from_min_size(b.min, vec2(b.width() * 0.60, b.height()));
            self.win_saved = self.win;
        }
        if self.photo_win == Rect::NOTHING {
            self.photo_win = Rect::from_min_size(pos2(right_x, b.top()), vec2(right_w, (b.height() * 0.56).round()));
        }
        if self.term_win == Rect::NOTHING {
            self.term_win = Rect::from_min_size(pos2(right_x, b.top() + (b.height() * 0.56).round() + 8.0), vec2(right_w, b.height() - (b.height() * 0.56).round() - 8.0));
        }
        let has_tr = self.store.transfer_job().is_some();
        if has_tr {
            if self.tr_rect == Rect::NOTHING {
                let h = 360.0f32 + if self.tr_graph { 58.0 } else { 0.0 };
                self.tr_rect = Rect::from_center_size(b.center() + vec2(40.0, 30.0), vec2(600.0, h));
            }
            if !self.tr_min && !self.z.contains(&Focus::Transfer) {
                self.z.push(Focus::Transfer);
            }
        } else {
            self.z.retain(|f| *f != Focus::Transfer);
            self.tr_rect = Rect::NOTHING;
        }
        if !self.photo_open {
            self.z.retain(|f| *f != Focus::Photos);
        }
        if !self.term_open {
            self.z.retain(|f| *f != Focus::Terminal);
        }
        if self.z.is_empty() {
            self.z.push(Focus::Files);
        }
        // keep windows reachable when the browser resizes
        for f in [Focus::Files, Focus::Photos, Focus::Terminal] {
            let mut r = self.win_rect(f);
            r = r.translate(vec2((b.left() + 80.0 - r.left()).max(0.0) + (b.right() - 80.0 - r.right()).min(0.0), (b.top() + 40.0 - r.top()).max(0.0) + (b.bottom() - 40.0 - r.bottom()).min(0.0)));
            match f {
                Focus::Files => self.win = r,
                Focus::Photos => self.photo_win = r,
                _ => self.term_win = r,
            }
        }
    }

    pub fn shell(&mut self, ui: &mut Ui) {
        let screen = ui.max_rect();
        let pal = self.pal;
        let p = ui.painter().clone();
        self.ensure_windows(screen);
        // wallpaper
        if let Some(w) = &self.wall {
            let uv = cover_uv(screen, w.size);
            let mut mesh = epaint::Mesh::with_texture(w.tex.id());
            let tl = uv(screen.left_top());
            let br = uv(screen.right_bottom());
            mesh.add_rect_with_uv(screen, Rect::from_min_max(tl, br), Color32::WHITE);
            p.add(Shape::mesh(mesh));
            if self.pal.dark {
                p.rect_filled(screen, 0.0, rgba(10, 15, 26, 0.35));
            }
        } else {
            p.rect_filled(screen, 0.0, hex(0x9fb2cc));
        }
        // raise the window under the pointer — before drawing, so this frame is already correct
        if ui.input(|i| i.pointer.any_pressed()) {
            if let Some(pos) = ui.input(|i| i.pointer.latest_pos()) {
                let mut hit = None;
                for f in self.draw_order().iter().rev() {
                    if self.win_rect(*f).contains(pos) {
                        hit = Some(*f);
                        break;
                    }
                }
                if let Some(f) = hit {
                    self.focus(f);
                }
            }
        }
        // draw windows bottom → top; only the top-most one is interactive (covered windows must not
        // hover-flash underneath the one you are using)
        let order = self.draw_order();
        let top = order.last().cloned().unwrap_or(Focus::Files);
        for f in order {
            let r = self.win_rect(f);
            if !r.intersects(screen) {
                continue;
            }
            let is_top = f == top;
            Area::new(Id::new(("window", f as u8)))
                .order(Order::Middle)
                .fixed_pos(r.min)
                .constrain(false)
                .interactable(is_top)
                .show(ui.ctx(), |aui| {
                    aui.set_clip_rect(r.intersect(screen));
                    let (rr, _) = aui.allocate_exact_size(r.size(), Sense::hover());
                    let mut fui = aui.new_child(UiBuilder::new().max_rect(rr));
                    fui.set_clip_rect(r.intersect(screen));
                    if !is_top {
                        fui.disable();
                    }
                    match f {
                        Focus::Files => self.files_frame(&mut fui, r),
                        Focus::Transfer => {}
                        Focus::Photos => self.photos_window(&mut fui, r),
                        Focus::Terminal => self.terminal_window(&mut fui, r),
                    }
                });
        }
        // windows "closed" into the dock still leave Files reachable
        if self.win_hidden && self.z.is_empty() {
            self.z.push(Focus::Files);
            self.win_hidden = false;
        }
        // a window raised during this pass (dock / toolbar) must paint on top next frame
        if ui.input(|i| i.pointer.any_click()) {
            ui.ctx().request_repaint();
        }
        // OS chrome on top
        let bar = Rect::from_min_size(screen.min, vec2(screen.width(), 30.0));
        Area::new(Id::new("os-menubar")).order(Order::Foreground).fixed_pos(bar.min).constrain(false).show(ui.ctx(), |mui| {
            mui.set_clip_rect(bar);
            let (rr, _) = mui.allocate_exact_size(bar.size(), Sense::hover());
            let mut child = mui.new_child(UiBuilder::new().max_rect(rr));
            child.set_clip_rect(bar);
            self.menubar_ui(&mut child, bar);
        });
        Area::new(Id::new("os-dock")).order(Order::Foreground).fixed_pos(pos2(0.0, screen.bottom() - 64.0)).constrain(false).show(ui.ctx(), |dui| {
            dui.allocate_exact_size(vec2(1.0, 1.0), Sense::hover());
            self.dock_ui(dui, screen);
        });
    }

    // ───────────────────────── top menubar ─────────────────────────
    pub fn menubar_ui(&mut self, ui: &mut Ui, bar: Rect) {
        let pal = self.pal;
        let p = ui.painter().clone();
        let glass_top = if pal.dark { rgba(18, 25, 40, 0.80) } else { rgba(255, 255, 255, 0.66) };
        let glass_bot = if pal.dark { rgba(12, 18, 30, 0.68) } else { rgba(250, 252, 255, 0.46) };
        grad_rect(&p, bar, 0.0, glass_top, glass_bot);
        reflect(&p, bar, 0.0, if pal.dark { 0.08 } else { 0.7 }, 0.0);
        let ink = if pal.dark { hex(0xeaf1ff) } else { hex(0x17233a) };
        let dim = with_a(ink, 0.62);
        let cy = bar.center().y;
        // brand
        let logo = Rect::from_center_size(pos2(bar.left() + 22.0, cy), vec2(18.0, 18.0));
        shadow(&p, logo, 7.0, 0.0, 2.0, 5.0, -1.0, with_a(pal.accent, 0.5));
        grad_rect(&p, logo, 7.0, pal.accent_hover, pal.accent_press);
        reflect(&p, logo, 7.0, 0.5, 0.3);
        icon(&p, logo.center(), Icon::Sparkle, 11.0, Color32::WHITE, true);
        let app = match self.focused() {
            Focus::Files => "Files",
            Focus::Photos => "Photos",
            Focus::Terminal => "Terminal",
            Focus::Transfer => "Transfer",
        };
        text(&p, pos2(bar.left() + 38.0, cy), Align2::LEFT_CENTER, app, 12.5, W::Semi, ink);
        // app menus
        let menus: [(&str, MenuKind); 6] = [("File", MenuKind::File), ("Edit", MenuKind::Edit), ("View", MenuKind::View), ("Go", MenuKind::Go), ("Window", MenuKind::Window), ("Help", MenuKind::Help)];
        let mut mx = bar.left() + 120.0;
        for (label, kind) in menus {
            let w = text_w(&p, label, 12.0, W::Medium) + 16.0;
            let r = Rect::from_min_size(pos2(mx, bar.top() + 2.0), vec2(w, bar.height() - 4.0));
            let resp = ui.interact(r, Id::new(("mb", label)), Sense::click());
            if resp.hovered() {
                p.rect_filled(r, 6.0, with_a(ink, 0.09));
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            text(&p, r.center(), Align2::CENTER_CENTER, label, 12.0, W::Medium, ink);
            if resp.clicked() {
                let e = self.shell_menu(kind);
                self.open_menu(ui.ctx(), Some(pos2(r.left(), bar.bottom() + 4.0)), e);
            }
            mx += w + 4.0;
        }
        // right side: transfer, status, theme, clock/date
        let mut rx = bar.right() - 16.0;
        let (t, d) = clock_strings();
        let dw = text_w(&p, &d, 11.0, W::Medium);
        text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, &d, 11.0, W::Medium, dim);
        rx -= dw + 12.0;
        let tw = text_w(&p, &t, 12.0, W::Semi);
        text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, &t, 12.0, W::Semi, ink);
        rx -= tw + 18.0;
        // theme
        let th = Rect::from_center_size(pos2(rx - 10.0, cy), vec2(24.0, 24.0));
        let thresp = ui.interact(th, Id::new("mb-theme"), Sense::click());
        icon(&p, th.center(), if pal.dark { Icon::Sun } else { Icon::Moon }, 15.0, if thresp.hovered() { pal.accent_ink } else { ink }, false);
        if thresp.clicked() {
            self.act(Action::ToggleTheme);
        }
        rx -= 34.0;
        // indexing / sync
        let idx = &self.store.eng.index;
        let sync = if idx.ready {
            format!("Synced · {}", idx.entries.len())
        } else {
            format!("Indexing… {}", idx.entries.len())
        };
        let sw = text_w(&p, &sync, 11.0, W::Medium);
        text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, &sync, 11.0, W::Medium, dim);
        dot(&p, pos2(rx - sw - 8.0, cy), 3.0, if idx.ready { pal.good } else { pal.accent });
        rx -= sw + 22.0;
        // transfer indicator (raises the window)
        if let Some(j) = self.store.transfer_job() {
            let prog = j.progress();
            let w = 92.0;
            let r = Rect::from_center_size(pos2(rx - w / 2.0, cy), vec2(w, 20.0));
            let resp = ui.interact(r, Id::new("mb-transfer"), Sense::click());
            ring_progress(&p, pos2(r.left() + 10.0, cy), 6.0, prog, with_a(ink, 0.18), pal.accent, 2.0);
            text_fit(&p, pos2(r.left() + 22.0, cy), Align2::LEFT_CENTER, &format!("{} {:.0}%", j.verb(), prog * 100.0), 10.5, W::Medium, if resp.hovered() { pal.accent_ink } else { ink }, w - 24.0);
            if resp.clicked() {
                self.open_app(Focus::Transfer);
            }
            rx -= w + 12.0;
        }
        if let Some(m) = self.store.eng.mount_of(&self.store.current_folder()) {
            if m.kind == MountKind::Removable {
                let w = text_w(&p, &m.label, 10.5, W::Medium);
                text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, &m.label, 10.5, W::Medium, dim);
                icon(&p, pos2(rx - w - 9.0, cy), Icon::Eject, 12.0, dim, false);
                rx -= w + 26.0;
            }
        }
    }

    pub fn shell_menu(&self, kind: MenuKind) -> Vec<MenuEntry> {
        let mk = |l: &str, i: Icon, a: Action| MenuEntry::Item(MenuItem { label: l.into(), icon: i, action: a, shortcut: "", danger: false, enabled: true, checked: false, swatch: None });
        match kind {
            MenuKind::File if self.focused() == Focus::Photos => vec![
                mk("Open in Files", Icon::FolderOpen, Action::Go(Route::Photos)),
                MenuEntry::Sep,
                mk("Info for selection", Icon::Info, Action::Properties(None)),
                mk("Move to Trash", Icon::Trash, Action::Trash),
            ],
            MenuKind::File if self.focused() == Focus::Terminal => vec![
                mk("Clear buffer", Icon::Broom, Action::ClearTerminal),
                mk("Reveal current folder", Icon::FolderOpen, Action::GoPath(String::new())),
                MenuEntry::Sep,
                mk("Close window", Icon::Close, Action::CloseTerminal),
            ],
            MenuKind::File => vec![
                mk("New Finder-style folder", Icon::NewFolder, Action::NewFolder),
                MenuEntry::Sep,
                mk("Open selected", Icon::OpenExt, Action::OpenSelected),
                mk("Get Info", Icon::Info, Action::Properties(None)),
                MenuEntry::Sep,
                mk("Move to Trash", Icon::Trash, Action::Trash),
                MenuEntry::Sep,
                mk("Close window", Icon::Close, Action::ToggleSidebar),
            ],
            MenuKind::Edit => vec![
                mk("Undo", Icon::Undo, Action::Undo),
                MenuEntry::Sep,
                mk("Cut", Icon::Cut, Action::Cut),
                mk("Copy", Icon::Copy, Action::Copy),
                mk("Paste", Icon::Paste, Action::Paste),
                MenuEntry::Sep,
                mk("Select All", Icon::SelectAll, Action::SelectAll),
                mk("Deselect All", Icon::Close, Action::SelectNone),
                MenuEntry::Sep,
                mk("Rename", Icon::Rename, Action::BeginRename(None)),
            ],
            MenuKind::View => {
                let mut v = vec![
                    mk("as Grid", Icon::Grid, Action::SetView(ViewMode::Grid)),
                    mk("as List", Icon::List, Action::SetView(ViewMode::List)),
                    MenuEntry::Sep,
                ];
                for k in SortKey::ALL {
                    v.push(mk(&format!("Sort by {}", k.label()), Icon::Sort, Action::SetSort(k)));
                }
                v.push(MenuEntry::Sep);
                v.push(mk("Show Hidden Files", Icon::Eye, Action::ToggleHidden));
                v.push(mk("Toggle Sidebar", Icon::Sidebar, Action::ToggleSidebar));
                v
            }
            MenuKind::Go => {
                let home = self.store.eng.home.clone();
                let mut v = vec![mk("Home", Icon::Home, Action::GoPath(home.clone()))];
                for (n, i) in [("Desktop", Icon::Monitor), ("Documents", Icon::Doc), ("Downloads", Icon::Download), ("Pictures", Icon::Image), ("Music", Icon::Music), ("Movies", Icon::Film)] {
                    let p = join(&home, n);
                    if self.store.eng.exists(&p) {
                        v.push(mk(n, i, Action::GoPath(p)));
                    }
                }
                v.push(MenuEntry::Sep);
                v.push(mk("Recents", Icon::Clock, Action::Go(Route::Recents)));
                v.push(mk("Photos", Icon::Image, Action::Go(Route::Photos)));
                v.push(mk("Storage", Icon::Pie, Action::Go(Route::Storage)));
                v.push(mk("Trash", Icon::Trash, Action::Go(Route::Trash)));
                v
            }
            MenuKind::Window => vec![
                mk("Files", Icon::FolderOpen, Action::GoPath(self.store.current_folder())),
                mk("Photos", Icon::Image, Action::Photos),
                mk("Terminal", Icon::Terminal, Action::Terminal),
                MenuEntry::Sep,
                mk("Transfer progress", Icon::Bolt, Action::RaiseTransfer),
                mk("Ask Smart panel", Icon::Sparkle, Action::ToggleAi),
            ],
            MenuKind::Help => vec![mk("Keyboard Shortcuts", Icon::Keyboard, Action::ShowShortcuts), mk("About Smart Files", Icon::Info, Action::ShowAbout)],
        }
    }

    // ───────────────────────── dock ─────────────────────────
    pub fn dock_ui(&mut self, ui: &mut Ui, screen: Rect) {
        let pal = self.pal;
        let reduced = self.reduced;
        let coarse = self.coarse;
        let dt = ui.input(|i| i.stable_dt);
        let mut apps: Vec<(&str, Icon, Option<Focus>, Option<Action>)> = vec![
            ("Files", Icon::FolderOpen, Some(Focus::Files), None),
            ("Photos", Icon::Image, Some(Focus::Photos), None),
            ("Terminal", Icon::Terminal, Some(Focus::Terminal), None),
            ("Transfer", Icon::Bolt, Some(Focus::Transfer), None),
            ("", Icon::Archive, None, None),
            ("Storage", Icon::Pie, None, Some(Action::Go(Route::Storage))),
            ("Ask Smart", Icon::Sparkle, None, Some(Action::ToggleAi)),
            ("", Icon::Sliders, None, None),
            ("Theme", if pal.dark { Icon::Sun } else { Icon::Moon }, None, Some(Action::ToggleTheme)),
        ];
        apps.retain(|(l, _, f, _)| *l != "Transfer" || self.store.transfer_job().is_some());
        let tile = if coarse { 46.0 } else { 42.0 };
        let gap = 7.0;
        let seps = apps.iter().filter(|a| a.0.is_empty()).count() as f32;
        let n = apps.len() as f32 - seps;
        let w = n * tile + (n - 1.0) * gap + seps * 17.0 + 22.0;
        let h = tile + 12.0;
        let rect = Rect::from_min_size(pos2(screen.center().x - w / 2.0, screen.bottom() - h - 8.0), vec2(w, h));
        let p = ui.painter().clone();
        elevation(&p, rect, 18.0, 4, pal.dark);
        grad_rect(&p, rect, 18.0, if pal.dark { rgba(30, 39, 57, 0.86) } else { rgba(255, 255, 255, 0.60) }, if pal.dark { rgba(15, 22, 35, 0.90) } else { rgba(240, 246, 255, 0.48) });
        reflect(&p, rect, 18.0, if pal.dark { 0.15 } else { 0.85 }, 0.0);
        let mut x = rect.left() + 11.0;
        let cy = rect.center().y;
        for (i, (label, ic, focus, action)) in apps.into_iter().enumerate() {
            if label.is_empty() {
                let sx = x + 5.0;
                p.rect_filled(Rect::from_center_size(pos2(sx, cy), vec2(1.0, tile * 0.62)), 0.5, with_a(pal.ink1, 0.16));
                x += 17.0;
                continue;
            }
            let id = Id::new(("dockapp", i));
            let r = Rect::from_min_size(pos2(x, cy - tile / 2.0), vec2(tile, tile));
            let resp = ui.interact(r, id, Sense::click());
            let lift = spring(ui.ctx(), id.with("lift"), if resp.hovered() { 1.0 } else { 0.0 }, dt, reduced);
            let running = focus.map(|f| match f {
                Focus::Files => !self.win_hidden || true,
                Focus::Photos => self.photo_open,
                Focus::Terminal => self.term_open,
                Focus::Transfer => self.store.transfer_job().is_some(),
            }).unwrap_or(false);
            let active = focus.map(|f| self.focused() == f && self.is_open(f)).unwrap_or(false);
            let rr = Rect::from_center_size(r.center() + vec2(0.0, -6.0 * lift), r.size());
            let col = pal.kind_color(match ic {
                Icon::Image => Kind::Image,
                Icon::Terminal => Kind::Code,
                Icon::Pie => Kind::Archive,
                Icon::Sparkle => Kind::Document,
                _ => Kind::Folder,
            });
            shadow(&p, rr, 13.0, 0.0, 4.0 + 5.0 * lift, 11.0, -3.0, with_a(col, 0.32 + 0.18 * lift));
            grad_rect(&p, rr, 13.0, if pal.dark { rgba(255, 255, 255, 0.16) } else { Color32::WHITE }, if pal.dark { rgba(255, 255, 255, 0.07) } else { rgba(243, 248, 255, 0.92) });
            reflect(&p, rr, 13.0, if pal.dark { 0.15 } else { 0.9 }, if pal.dark { 0.08 } else { 0.5 });
            grad_rect(&p, Rect::from_center_size(rr.center(), vec2(28.0, 28.0)), 9.0, with_a(col, 0.26), with_a(col, 0.12));
            icon(&p, rr.center(), ic, 18.0, col, active);
            if resp.hovered() && lift > 0.25 {
                let g = ellipsized(&p, label, 11.0, W::Semi, pal.ink1, 140.0);
                let gw = g.size().x + 18.0;
                let lp = Rect::from_center_size(pos2(rr.center().x, rr.top() - 18.0 - 6.0 * lift), vec2(gw, 23.0));
                surface(&p, lp, 11.0, &pal, 3, pal.menu, pal.menu);
                p.galley(pos2(lp.center().x - g.size().x / 2.0, lp.center().y - g.size().y / 2.0), g, pal.ink1);
            }
            if running {
                dot(&p, pos2(rr.center().x, rect.bottom() - 6.0), 2.3, if active { pal.accent } else { with_a(pal.ink1, 0.45) });
            }
            if resp.clicked() {
                if let Some(f) = focus {
                    if self.focused() == f && self.is_open(f) {
                        self.minimize(f);
                    } else {
                        self.open_app(f);
                    }
                } else if let Some(a) = action {
                    self.act(a);
                }
            }
            x += tile + gap;
        }
    }
}

#[derive(Clone, Copy)]
pub enum MenuKind {
    File,
    Edit,
    View,
    Go,
    Window,
    Help,
}
