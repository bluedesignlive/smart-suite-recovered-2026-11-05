//! egui frontend shell: window chrome (floating on the web, native window elsewhere), input, layout.
use crate::audio::Audio;
use crate::paint::*;
use crate::theme::*;
use crate::thumbs::Thumbs;
use egui::*;
use smart_core::*;
use smart_core::Theme;

pub struct MenuState {
    pub pos: Pos2,
    pub entries: Vec<MenuEntry>,
    pub sub: Option<(usize, Rect)>,
    pub t0: f64,
}
pub struct DragState {
    pub paths: Vec<Path>,
}
pub struct Wall {
    pub tex: TextureHandle,
    pub blur: TextureHandle,
    pub size: Vec2,
}

pub struct App {
    pub store: Store,
    pub pal: Pal,
    pub web: bool,
    pub win: Rect,
    pub win_hidden: bool,
    pub win_max: bool,
    pub win_saved: Rect,
    pub thumbs: Thumbs,
    pub audio: Audio,
    pub wall: Option<Wall>,
    pub menu: Option<MenuState>,
    pub drag: Option<DragState>,
    pub drops: Vec<(Rect, Path)>,
    pub item_rects: Vec<(Path, Rect)>,
    pub item_rects_prev: Vec<(Path, Rect)>,
    pub rubber: Option<Pos2>,
    pub rubber_base: Vec<Path>,
    pub pending: Vec<Action>,
    pub ai_input: String,
    pub cols: usize,
    pub t: f64,
    pub palette_fresh: bool,
    pub compose_focus: bool,
    pub sidebar_scroll: f32,
    pub lens_hover: Option<usize>,
    pub tr_flash: f32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, store: Store, web: bool, dark: bool) -> App {
        crate::icons::install_fonts(&cc.egui_ctx);
        let mut store = store;
        if dark {
            store.theme = Theme::Dark;
        }
        let wall = if web { load_wall(&cc.egui_ctx) } else { None };
        App {
            store,
            pal: Pal::new(dark, web),
            web,
            win: Rect::NOTHING,
            win_hidden: false,
            win_max: false,
            win_saved: Rect::NOTHING,
            thumbs: Thumbs::default(),
            audio: Audio::default(),
            wall,
            menu: None,
            drag: None,
            drops: vec![],
            item_rects: vec![],
            item_rects_prev: vec![],
            rubber: None,
            rubber_base: vec![],
            pending: vec![],
            ai_input: String::new(),
            cols: 4,
            t: 0.0,
            palette_fresh: false,
            compose_focus: false,
            sidebar_scroll: 0.0,
            lens_hover: None,
            tr_flash: 0.0,
        }
    }

    /// True on macOS (shows ⌘ in shortcut hints). On the web this asks the browser platform.
    pub fn mac(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            crate::web::is_mac()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            cfg!(target_os = "macos")
        }
    }

    pub fn act(&mut self, a: Action) {
        self.pending.push(a);
    }

    fn run(&mut self, ctx: &Context, a: Action) {
        match &a {
            Action::CopyPath => {
                let t = if self.store.sel.is_empty() { self.store.current_folder() } else { self.store.sel.join("\n") };
                ctx.copy_text(t);
            }
            Action::Palette(Some(_)) => self.palette_fresh = true,
            Action::ToggleAi => {
                self.compose_focus = !self.store.ai_open;
            }
            Action::AiAsk(_) => {
                self.store.ai_open = true;
                self.ai_input.clear();
            }
            Action::Open(_) | Action::OpenSelected | Action::Go(_) | Action::GoPath(_) | Action::Search(_) | Action::Paste | Action::Unmount(_) => self.menu = None,
            _ => {}
        }
        self.store.dispatch(a);
    }

    // ───────────── input ─────────────
    /// Web: chords arrive from the DOM listener in `lib.rs` (browsers swallow ⌘/Ctrl + letter
    /// before any canvas framework can see them). Native keeps egui's own key handling.
    fn drain_chords(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        {
            let typing = ctx.wants_keyboard_input();
            let mut open = self.store.palette.open;
            for c in crate::web::drain_chords() {
                let a = match c.as_str() {
                    "palette" => {
                        open = !open;
                        Some(if open { Action::Palette(Some(Rail::All)) } else { Action::Palette(None) })
                    }
                    "search" => Some(Action::FocusSearch),
                    "ai" if !typing => Some(Action::ToggleAi),
                    "sidebar" if !typing => Some(Action::ToggleSidebar),
                    "copy" if !typing => Some(Action::Copy),
                    "cut" if !typing => Some(Action::Cut),
                    "paste" if !typing => Some(Action::Paste),
                    "selectall" if !typing => Some(Action::SelectAll),
                    "undo" if !typing => Some(Action::Undo),
                    "duplicate" if !typing => Some(Action::Duplicate),
                    "info" if !typing => Some(Action::Properties(None)),
                    "newfolder" if !typing => Some(Action::NewFolder),
                    _ => None,
                };
                if let Some(a) = a {
                    self.act(a);
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = ctx;
    }

    fn keyboard(&mut self, ctx: &Context) {
        let typing = ctx.wants_keyboard_input();
        let blocked = self.store.modal.is_some() || self.store.palette.open;
        // ⌘ on macOS, Ctrl elsewhere. On the web egui reports these as *separate* flags
        // (`command` is the Meta key only), so the app treats them as one "command" modifier.
        let (cmd, shift, alt) = ctx.input(|i| (i.modifiers.command || i.modifiers.ctrl, i.modifiers.shift, i.modifiers.alt));
        let k = |key: Key| ctx.input(|i| i.key_pressed(key));
        if k(Key::Escape) {
            if self.menu.is_some() {
                self.menu = None;
            } else if self.drag.is_some() {
                self.drag = None;
            } else if self.store.modal.is_some() {
                self.act(Action::CloseModal);
            } else if self.store.palette.open {
                self.act(Action::Palette(None));
            } else if self.store.quicklook.is_some() {
                self.act(Action::QuickLook(None));
            } else if self.store.rename.is_some() {
                self.act(Action::CancelRename);
            } else if !typing {
                if !self.store.sel.is_empty() {
                    self.act(Action::SelectNone);
                } else if self.store.ai_open {
                    self.act(Action::ToggleAi);
                }
            }
        }
        // modifier chords are owned by the DOM layer on the web (see lib.rs); egui only handles
        // them natively, where ⌘/Ctrl arrive as regular modifiers.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let raw_cmd = ctx.input(|i| i.modifiers.command || i.modifiers.ctrl);
            if (raw_cmd && ctx.input(|i| i.key_pressed(Key::K))) || k(Key::F1) {
                let open = self.store.palette.open;
                self.act(if open { Action::Palette(None) } else { Action::Palette(Some(Rail::All)) });
            }
        }
        if blocked {
            return;
        }
        if !typing && ctx.input(|i| i.events.iter().any(|e| matches!(e, Event::Text(t) if t == "/"))) {
            self.act(Action::FocusSearch);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if cmd && k(Key::Backslash) {
                self.act(Action::ToggleSidebar);
            }
            if cmd && k(Key::F) {
                self.act(Action::FocusSearch);
            }
            if cmd && k(Key::J) {
                self.act(Action::ToggleAi);
            }
        }
        if alt && k(Key::ArrowLeft) {
            self.act(Action::Back);
        }
        if alt && k(Key::ArrowRight) {
            self.act(Action::Forward);
        }
        if alt && k(Key::ArrowUp) {
            self.act(Action::Up);
        }
        if typing {
            return;
        }
        let events = ctx.input(|i| i.events.clone());
        #[cfg(not(target_arch = "wasm32"))]
        {
            // native clipboard chords (browsers route these through the DOM layer instead)
            let mut fired = None;
            for e in events {
                match e {
                    Event::Copy => fired = Some(Action::Copy),
                    Event::Cut => fired = Some(Action::Cut),
                    Event::Paste(_) => fired = Some(Action::Paste),
                    _ => {}
                }
            }
            if cmd && k(Key::C) { fired = Some(Action::Copy); }
            if cmd && k(Key::X) { fired = Some(Action::Cut); }
            if cmd && k(Key::V) { fired = Some(Action::Paste); }
            if let Some(a) = fired {
                self.act(a);
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (events, cmd);
        if self.store.quicklook.is_some() {
            if k(Key::Space) {
                self.act(Action::QuickLook(None));
            }
            let d = if k(Key::ArrowRight) || k(Key::ArrowDown) { 1 } else if k(Key::ArrowLeft) || k(Key::ArrowUp) { -1 } else { 0 };
            if d != 0 {
                self.step_quicklook(d);
            }
            return;
        }
        if cmd && k(Key::A) {
            self.act(Action::SelectAll);
        }
        if cmd && k(Key::D) {
            self.act(Action::Duplicate);
        }
        if cmd && k(Key::I) {
            self.act(Action::Properties(None));
        }
        if cmd && k(Key::Z) {
            self.act(Action::Undo);
        }
        if cmd && shift && k(Key::N) {
            self.act(Action::NewFolder);
        }
        if k(Key::F2) {
            self.act(Action::BeginRename(None));
        }
        if k(Key::F5) {
            self.act(Action::Refresh);
        }
        if k(Key::Delete) {
            self.act(Action::Trash);
        }
        if k(Key::Backspace) && !cmd {
            self.act(Action::Back);
        }
        if k(Key::Enter) && !self.store.sel.is_empty() {
            self.act(Action::OpenSelected);
        }
        if k(Key::Space) && !self.store.sel.is_empty() {
            self.act(Action::QuickLook(None));
        }
        let cols = self.cols.max(1) as i32;
        let d = if k(Key::ArrowRight) { 1 } else if k(Key::ArrowLeft) { -1 } else if k(Key::ArrowDown) { cols } else if k(Key::ArrowUp) { -cols } else { 0 };
        if d != 0 && !self.store.items.is_empty() {
            let cur = self.store.sel.last().and_then(|p| self.store.index_of(p)).map(|i| i as i32).unwrap_or(if d > 0 { -1 } else { self.store.items.len() as i32 });
            let n = (cur + d).clamp(0, self.store.items.len() as i32 - 1) as usize;
            let path = self.store.items[n].path.clone();
            self.act(Action::Select { path, ctrl: false, shift });
        }
    }

    pub fn step_quicklook(&mut self, d: i32) {
        let files: Vec<&Entry> = self.store.items.iter().filter(|e| !e.is_dir).collect();
        if let Some(cur) = &self.store.quicklook {
            if let Some(i) = files.iter().position(|e| &e.path == cur) {
                let n = (i as i32 + d).rem_euclid(files.len() as i32) as usize;
                let p = files[n].path.clone();
                self.store.quicklook = Some(p.clone());
                self.store.sel = vec![p];
            }
        }
    }

    // ───────────── external drops ─────────────
    fn external_drops(&mut self, ctx: &Context) {
        let files = ctx.input(|i| i.raw.dropped_files.clone());
        if files.is_empty() || !self.store.in_folder() {
            return;
        }
        let dest = self.store.current_folder();
        let mut n = 0;
        for f in files {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(p) = &f.path {
                let sp = vfs::from_real(p);
                if self.store.eng.start_transfer(&[sp], &dest, false).is_ok() {
                    n += 1;
                }
                continue;
            }
            if let Some(bytes) = &f.bytes {
                let target = self.store.eng.unique(&dest, if f.name.is_empty() { "Dropped file" } else { &f.name }, "");
                if self.store.eng.create_file(&target, bytes).is_ok() {
                    n += 1;
                }
            }
        }
        if n > 0 {
            self.store.eng.revision += 1;
            self.store.toast(format!("Added {} to “{}”", plural(n, "file", "files"), name_of(&dest)), Icon::Upload, true, None);
        }
    }

    fn finish_drag(&mut self, ctx: &Context) {
        let Some(d) = &self.drag else { return };
        if ctx.input(|i| i.pointer.any_released()) {
            let pos = ctx.input(|i| i.pointer.latest_pos());
            let copy = ctx.input(|i| i.modifiers.alt);
            let target = pos.and_then(|p| self.drops.iter().rev().find(|(r, _)| r.contains(p)).map(|(_, t)| t.clone()));
            let paths = d.paths.clone();
            self.drag = None;
            if let Some(t) = target {
                if paths.iter().any(|p| *p == t || is_within(&t, p)) {
                    return;
                }
                let same = paths.first().map(|p| self.store.eng.route(p) == self.store.eng.route(&t)).unwrap_or(false);
                if paths.iter().all(|p| parent(p) == t) {
                    return;
                }
                self.act(if same && !copy { Action::MoveTo(paths, t) } else { Action::CopyTo(paths, t) });
            }
        }
        ctx.request_repaint();
    }
}

fn load_wall(ctx: &Context) -> Option<Wall> {
    let bytes = include_bytes!("../../../assets/wallpaper.jpg");
    let img = image::load_from_memory(bytes).ok()?;
    let big = img.thumbnail(1800, 1200).to_rgba8();
    let small = image::imageops::resize(&img.thumbnail(420, 300).to_rgba8(), 420, 280.max(1), image::imageops::FilterType::Triangle);
    let small = image::imageops::blur(&image::DynamicImage::ImageRgba8(small).to_rgba8(), 9.0);
    let ci = |i: &image::RgbaImage| ColorImage::from_rgba_unmultiplied([i.width() as usize, i.height() as usize], i.as_raw());
    Some(Wall { size: vec2(big.width() as f32, big.height() as f32), tex: ctx.load_texture("wall", ci(&big), TextureOptions::LINEAR), blur: ctx.load_texture("wall-blur", ci(&small), TextureOptions::LINEAR) })
}

pub fn cover_uv(target: Rect, img: Vec2) -> impl Fn(Pos2) -> Pos2 {
    let ta = target.width() / target.height();
    let ia = img.x / img.y;
    let (sx, sy) = if ta > ia { (1.0, ia / ta) } else { (ta / ia, 1.0) };
    move |p: Pos2| {
        let u = (p.x - target.left()) / target.width();
        let v = (p.y - target.top()) / target.height();
        pos2(0.5 + (u - 0.5) * sx, 0.5 + (v - 0.5) * sy)
    }
}

impl eframe::App for App {
    fn clear_color(&self, _v: &Visuals) -> [f32; 4] {
        if self.web { [0.07, 0.09, 0.14, 1.0] } else { [0.0; 4] }
    }

    fn update(&mut self, ctx: &Context, _f: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt as f64).clamp(0.0, 0.1);
        self.t += dt;
        self.store.tick(dt);
        self.thumbs.begin_frame(ctx);
        if let Some(cmd) = self.store.player.cmd.take() {
            self.audio.handle(cmd, &self.store);
        }
        let dark = self.store.theme == Theme::Dark;
        self.pal = Pal::new(dark, self.web);
        let mut v = if dark { Visuals::dark() } else { Visuals::light() };
        v.override_text_color = Some(self.pal.ink1);
        v.selection.bg_fill = with_a(self.pal.accent, 0.28);
        v.selection.stroke = Stroke::NONE;
        v.text_cursor.stroke = Stroke::new(1.5, self.pal.accent);
        v.window_shadow = epaint::Shadow::NONE;
        v.popup_shadow = epaint::Shadow::NONE;
        v.widgets.noninteractive.bg_stroke = Stroke::NONE;
        v.widgets.inactive.bg_stroke = Stroke::NONE;
        v.widgets.hovered.bg_stroke = Stroke::NONE;
        v.widgets.active.bg_stroke = Stroke::NONE;
        v.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        v.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
        v.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        v.widgets.hovered.bg_fill = Color32::TRANSPARENT;
        v.widgets.active.bg_fill = Color32::TRANSPARENT;
        v.striped = false;
        ctx.set_visuals(v);
        ctx.style_mut(|s| {
            s.spacing.scroll = egui::style::ScrollStyle { floating: true, bar_width: 7.0, floating_width: 5.0, floating_allocated_width: 0.0, handle_min_length: 36.0, ..egui::style::ScrollStyle::floating() };
        });

        self.drain_chords(ctx);
        self.keyboard(ctx);
        self.item_rects_prev = std::mem::take(&mut self.item_rects);
        self.drops.clear();

        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| self.desktop(ui));

        self.overlays(ctx);
        self.finish_drag(ctx);
        self.external_drops(ctx);
        for a in std::mem::take(&mut self.pending) {
            self.run(ctx, a);
        }
        if self.store.eng.jobs.iter().any(|j| j.active()) || self.store.ai_pending.is_some() || !self.store.toasts.is_empty() || self.store.player.playing || self.drag.is_some() || !self.store.eng.index.ready {
            ctx.request_repaint();
        }
    }
}

impl App {
    fn desktop(&mut self, ui: &mut Ui) {
        let screen = ui.max_rect();
        let p = ui.painter().clone();
        let pal = self.pal;
        if self.web {
            if let Some(w) = &self.wall {
                let uv = cover_uv(screen, w.size);
                let mut mesh = epaint::Mesh::with_texture(w.tex.id());
                let tl = uv(screen.left_top());
                let br = uv(screen.right_bottom());
                mesh.add_rect_with_uv(screen, Rect::from_min_max(tl, br), Color32::WHITE);
                p.add(Shape::mesh(mesh));
                if pal.dark {
                    p.rect_filled(screen, 0.0, rgba(8, 12, 22, 0.62));
                }
            } else {
                p.rect_filled(screen, 0.0, hex(0x9fb2cc));
            }
            let (t, d) = clock_strings();
            let ink = if pal.dark { Color32::WHITE } else { hex(0x1b2740) };
            text(&p, pos2(screen.right() - 22.0, screen.top() + 16.0), Align2::RIGHT_TOP, &t, 19.0, W::Semi, ink);
            text(&p, pos2(screen.right() - 22.0, screen.top() + 40.0), Align2::RIGHT_TOP, &d, 11.5, W::Medium, with_a(ink, 0.7));
            if self.win == Rect::NOTHING || self.win_saved == Rect::NOTHING {
                let w = (screen.width() * 0.9).clamp(760.0, 1440.0).min(screen.width() - 24.0);
                let h = (screen.height() * 0.86).clamp(520.0, 860.0).min(screen.height() - 24.0);
                self.win = Rect::from_center_size(screen.center() + vec2(0.0, 6.0), vec2(w, h));
                self.win_saved = self.win;
            }
            if self.win_hidden {
                let r = Rect::from_center_size(pos2(screen.center().x, screen.bottom() - 46.0), vec2(220.0, 38.0));
                if button(ui, &pal, r, Id::new("restore"), Some(Icon::Folder), "Restore Smart Files", Look::Plain, false, true).clicked() {
                    self.win_hidden = false;
                }
                return;
            }
        } else {
            self.win = screen;
        }
        let win = self.win;
        let rad = if self.win_max && !self.web { 0.0 } else { 22.0 };
        // window body ------------------------------------------------------
        if self.web {
            elevation(&p, win, rad, 5, pal.dark);
            if let Some(w) = &self.wall {
                let uv = cover_uv(screen, w.size);
                fill_poly(&p, &rrect_pts(win, [rad; 4]), &|_| Color32::WHITE, Some((w.blur.id(), &uv)));
            }
        }
        grad_rect(&p, win, rad, pal.win_top, pal.win_bottom);
        if self.web {
            reflect(&p, win, rad, if pal.dark { 0.12 } else { 0.9 }, 0.0);
        }
        let t_sb = ui.ctx().animate_bool_with_time_and_easing(Id::new("sb"), self.store.sidebar_open, 0.34, emath::easing::cubic_out);
        let sw = 252.0 * t_sb;
        let sidebar = Rect::from_min_size(win.min, vec2(sw, win.height()));
        let main = Rect::from_min_max(pos2(win.left() + sw, win.top()), win.max);
        let appbar = Rect::from_min_size(main.min, vec2(main.width(), 60.0));
        let status = Rect::from_min_max(pos2(main.left(), main.bottom() - 34.0), main.max);
        let content = Rect::from_min_max(pos2(main.left(), appbar.bottom()), pos2(main.right(), status.top()));
        if sw > 2.0 {
            grad_rect_r(&p, sidebar, [rad, 0.0, 0.0, rad], pal.side_top, pal.side_bottom);
        }
        // tonal lift for the content plane
        fill_poly(&p, &rrect_pts(main, [if sw > 2.0 { 0.0 } else { rad }, rad, rad, if sw > 2.0 { 0.0 } else { rad }]), &|_| pal.content, None);

        // drag zones (registered first so controls win)
        let drag_a = Rect::from_min_max(win.min, pos2(win.right(), win.top() + 60.0));
        let dr = ui.interact(drag_a, Id::new("win-drag"), Sense::click_and_drag());
        self.window_drag(ui.ctx(), &dr);

        self.sidebar_ui(ui, sidebar, win, rad);
        self.appbar_ui(ui, appbar, sw < 40.0, rad);
        self.content_ui(ui, content);
        self.status_ui(ui, status, rad);
        self.ai_panel(ui, win);
        self.window_resize(ui, win);
    }

    fn window_drag(&mut self, ctx: &Context, r: &Response) {
        if r.double_clicked() {
            self.toggle_max(ctx);
        }
        if r.drag_started() {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }
        if self.web && r.dragged() && !self.win_max {
            let screen = ctx.screen_rect();
            let mut w = self.win.translate(r.drag_delta());
            let dy = (screen.top() + 4.0 - w.top()).max(0.0);
            w = w.translate(vec2(0.0, dy));
            w = w.translate(vec2((screen.left() + 60.0 - w.right()).max(0.0) + (screen.right() - 60.0 - w.left()).min(0.0), (screen.bottom() - 50.0 - w.top()).min(0.0)));
            self.win = w;
            self.win_saved = w;
        }
    }

    pub fn toggle_max(&mut self, ctx: &Context) {
        if self.web {
            let screen = ctx.screen_rect();
            if self.win_max {
                self.win = self.win_saved;
            } else {
                self.win_saved = self.win;
                self.win = screen.shrink(10.0);
            }
            self.win_max = !self.win_max;
        } else {
            self.win_max = !self.win_max;
            ctx.send_viewport_cmd(ViewportCommand::Maximized(self.win_max));
        }
    }

    fn window_resize(&mut self, ui: &mut Ui, win: Rect) {
        if self.win_max && self.web {
            return;
        }
        let e = 6.0;
        let c = 14.0;
        // (rect, cursor, dx_min, dy_min, dx_max, dy_max)
        let zones: [(Rect, CursorIcon, [f32; 4], &str); 8] = [
            (Rect::from_min_max(pos2(win.left() + c, win.top() - 2.0), pos2(win.right() - c, win.top() + e - 2.0)), CursorIcon::ResizeVertical, [0.0, 1.0, 0.0, 0.0], "North"),
            (Rect::from_min_max(pos2(win.left() + c, win.bottom() - e + 2.0), pos2(win.right() - c, win.bottom() + 2.0)), CursorIcon::ResizeVertical, [0.0, 0.0, 0.0, 1.0], "South"),
            (Rect::from_min_max(pos2(win.left() - 2.0, win.top() + c), pos2(win.left() + e - 2.0, win.bottom() - c)), CursorIcon::ResizeHorizontal, [1.0, 0.0, 0.0, 0.0], "West"),
            (Rect::from_min_max(pos2(win.right() - e + 2.0, win.top() + c), pos2(win.right() + 2.0, win.bottom() - c)), CursorIcon::ResizeHorizontal, [0.0, 0.0, 1.0, 0.0], "East"),
            (Rect::from_min_size(pos2(win.left() - 2.0, win.top() - 2.0), vec2(c, c)), CursorIcon::ResizeNwSe, [1.0, 1.0, 0.0, 0.0], "NorthWest"),
            (Rect::from_min_size(pos2(win.right() - c + 2.0, win.top() - 2.0), vec2(c, c)), CursorIcon::ResizeNeSw, [0.0, 1.0, 1.0, 0.0], "NorthEast"),
            (Rect::from_min_size(pos2(win.left() - 2.0, win.bottom() - c + 2.0), vec2(c, c)), CursorIcon::ResizeNeSw, [1.0, 0.0, 0.0, 1.0], "SouthWest"),
            (Rect::from_min_size(pos2(win.right() - c + 2.0, win.bottom() - c + 2.0), vec2(c, c)), CursorIcon::ResizeNwSe, [0.0, 0.0, 1.0, 1.0], "SouthEast"),
        ];
        for (rect, cur, k, name) in zones {
            let r = ui.interact(rect, Id::new(("resize", name)), Sense::drag());
            if r.hovered() || r.dragged() {
                ui.ctx().set_cursor_icon(cur);
            }
            if r.drag_started() && !self.web {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let dir = match name {
                        "North" => ResizeDirection::North,
                        "South" => ResizeDirection::South,
                        "West" => ResizeDirection::West,
                        "East" => ResizeDirection::East,
                        "NorthWest" => ResizeDirection::NorthWest,
                        "NorthEast" => ResizeDirection::NorthEast,
                        "SouthWest" => ResizeDirection::SouthWest,
                        _ => ResizeDirection::SouthEast,
                    };
                    ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(dir));
                }
            }
            if r.dragged() && self.web {
                let d = r.drag_delta();
                let mut w = self.win;
                w.min.x += d.x * k[0];
                w.min.y += d.y * k[1];
                w.max.x += d.x * k[2];
                w.max.y += d.y * k[3];
                if w.width() >= 760.0 && w.height() >= 500.0 {
                    self.win = w;
                    self.win_saved = w;
                }
            }
        }
    }
}
