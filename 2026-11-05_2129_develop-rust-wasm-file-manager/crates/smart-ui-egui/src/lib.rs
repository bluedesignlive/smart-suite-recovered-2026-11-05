//! egui frontend for Smart Files. All behaviour lives in `smart-core`; this crate only draws and routes input.
pub mod app;
pub mod appkit;
pub mod chrome;
pub mod content;
pub mod copyfx;
pub mod icons;
pub mod icons_gen;
pub mod modals;
pub mod overlays;
pub mod paint;
pub mod photos;
pub mod shell;
pub mod terminal;
pub mod theme;

use app::*;
use eframe::egui;
use smart_core::*;

#[allow(non_snake_case)]
fn IS_OPAQUE() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::opaque()
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

impl eframe::App for SmartApp {
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        if IS_WEB {
            [0.56, 0.62, 0.72, 1.0]
        } else if IS_OPAQUE() {
            [0.10, 0.12, 0.17, 1.0]
        } else {
            [0.0, 0.0, 0.0, 0.0]
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.guarded_frame(ui);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("smart-library", self.be.library.read().unwrap().to_json());
        self.be.save_library();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.be.save_library();
    }
}

impl SmartApp {
    /// One UI frame that can never take the process down: a panic while drawing is caught, logged, and the
    /// transient UI state (modals, menus, drags) is reset so the next frame starts clean.
    #[cfg(not(target_arch = "wasm32"))]
    fn guarded_frame(&mut self, ui: &mut egui::Ui) {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.frame(ui)));
        match r {
            Ok(()) => self.panic_streak = 0,
            Err(_) => {
                self.panic_streak += 1;
                self.modal = Modal::None;
                self.menu = None;
                self.drag = None;
                self.rename = None;
                if self.panic_streak > 3 {
                    self.sel.clear();
                    self.error = Some("Smart Files hit an internal error and recovered. Details are in the crash log.".into());
                }
                self.toast("Recovered from an internal error (see crash log)", false);
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
            }
        }
        self.apply_native_cmds(ui.ctx());
    }

    #[cfg(target_arch = "wasm32")]
    fn guarded_frame(&mut self, ui: &mut egui::Ui) {
        self.frame(ui);
    }

    fn apply_native_cmds(&mut self, ctx: &egui::Context) {
        use egui::ViewportCommand as V;
        for c in std::mem::take(&mut self.native_cmds) {
            match c {
                NativeCmd::Minimize => ctx.send_viewport_cmd(V::Minimized(true)),
                NativeCmd::ToggleMaximize => {
                    let m = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                    ctx.send_viewport_cmd(V::Maximized(!m));
                }
                NativeCmd::ToggleFullscreen => {
                    let f = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                    ctx.send_viewport_cmd(V::Fullscreen(!f));
                }
                NativeCmd::Close => ctx.send_viewport_cmd(V::Close),
                NativeCmd::NewWindow => {
                    #[cfg(not(target_arch = "wasm32"))]
                    match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).args(std::env::args().skip(1).filter(|a| a != "--demo" || self.be.demo)).spawn()) {
                        Ok(_) => {}
                        Err(e) => self.toast(format!("Couldn\u{2019}t open a new window: {e}"), false),
                    }
                }
            }
        }
    }

    fn frame(&mut self, ui: &mut egui::Ui) {
        let ctx = &ui.ctx().clone();
        self.time = ctx.input(|i| i.time);
        self.dt = ctx.input(|i| i.stable_dt).min(0.1);
        self.pump(ctx);
        // debounce live search
        if self.search_active() && self.search.query.trim() != self.search.last_sent && self.time - self.search.dirty_at > 0.10 {
            self.run_search();
        }
        if !self.search_active() && !self.search.last_sent.is_empty() {
            self.clear_search();
        }
        // sidebar slide
        let target = if self.st().sidebar_collapsed || (IS_WEB && ctx.content_rect().width() < 760.0) { 0.0 } else { 1.0 };
        if (self.sidebar_open - target).abs() > 0.002 {
            self.sidebar_open += (target - self.sidebar_open) * (1.0 - (-self.dt * 16.0).exp());
            ctx.request_repaint();
        } else {
            self.sidebar_open = target;
        }
        self.handle_keys(ctx);
        let vis = if self.th.dark { egui::Visuals::dark() } else { egui::Visuals::light() };
        if ctx.global_style().visuals.dark_mode != vis.dark_mode {
            ctx.set_visuals(vis);
            ctx.global_style_mut(|s| {
                s.visuals.selection.bg_fill = theme::alpha(self.th.accent, 0.3);
                s.visuals.text_cursor.stroke = egui::Stroke::new(1.5, self.th.accent);
            });
        }
        overlays::WIN_RECT.with(|w| w.set(self.window_rect(ui.max_rect())));
        self.draw_window(ui);
        self.native_resize_grips(ctx);
        if !(self.win_closed || self.win_min) {
            self.draw_menu(ctx);
            self.draw_modal(ctx);
            self.draw_palette(ctx);
            self.draw_ai(ctx);
            self.draw_search_sheet(ctx);
            self.draw_jobs_card(ctx);
            self.draw_toasts(ctx);
            self.draw_drag_ghost(ctx);
        }
        // resolve a drag exactly once, against whatever was under the pointer
        if ctx.input(|i| i.pointer.any_released()) {
            if let Some(drag) = self.drag.take() {
                let modifiers = ctx.input(|i| i.modifiers);
                if let Some(pos) = ctx.pointer_latest_pos() {
                    if let Some(target) = chrome::drop_target_at(pos) {
                        match target {
                            chrome::DropTarget::Tag(id) => {
                                let n = drag.items.len();
                                for l in &drag.items {
                                    if !self.lib(|lib| lib.has_tag(l, &id)) {
                                        self.be.library.write().unwrap().toggle_tag(l, &id);
                                    }
                                }
                                let name = self.lib(|l| l.tags.iter().find(|t| t.id == id).map(|t| t.name.clone())).unwrap_or(id);
                                self.toast(format!("Tagged {} item{} as {}", n, if n == 1 { "" } else { "s" }, name), true);
                            }
                            chrome::DropTarget::Folder(dest) | chrome::DropTarget::Place(dest) => {
                                let srcs: Vec<_> = drag.items.into_iter().filter(|s| s != &dest && !dest.starts_with(s) && s.parent().as_ref() != Some(&dest)).collect();
                                if srcs.is_empty() {
                                    self.toast("That is already the destination", false);
                                } else {
                                    let cross = srcs.iter().any(|s| s.fs != dest.fs);
                                    let copy = modifiers.alt || modifiers.ctrl || modifiers.command || cross;
                                    let n = srcs.len();
                                    self.submit_job(if copy { smart_core::JobKind::Copy { srcs, dest: dest.clone() } } else { smart_core::JobKind::Move { srcs, dest: dest.clone() } });
                                    self.toast(format!("{} {} item{} to {}", if copy { "Copying" } else { "Moving" }, n, if n == 1 { "" } else { "s" }, dest.name()), true);
                                }
                            }
                            chrome::DropTarget::Trash => {
                                self.submit_job(smart_core::JobKind::Trash { srcs: drag.items });
                                self.toast("Moved to Trash", true);
                            }
                        }
                    }
                }
            }
        }
        // drain queued actions (widgets push, we apply after the frame)
        for _ in 0..4 {
            let mut v = std::mem::take(&mut self.acts);
            v.extend(chrome::ACTS.with(|q| std::mem::take(&mut *q.borrow_mut())));
            if v.is_empty() {
                break;
            }
            for a in v {
                match &a {
                    Act::Toast(t) if t == "\u{1}" => self.drag = None,
                    Act::Toast(t) if t.is_empty() || t == "\u{0}" => {}
                    _ => self.act(a),
                }
            }
        }
        if let Some(t) = self.pending_copy.take() {
            ctx.copy_text(t);
        }
        // big thumbnails for the preview
        if self.be.audio.is_loaded() {
            ctx.request_repaint_after(std::time::Duration::from_millis(60));
        }
        if self.search_active() && !self.search.done {
            ctx.request_repaint_after(std::time::Duration::from_millis(30));
        }
    }
}

impl SmartApp {
    fn native_resize_grips(&mut self, ctx: &egui::Context) {
        if IS_WEB {
            return;
        }
        use egui::viewport::ResizeDirection as D;
        let r = ctx.content_rect();
        let Some(p) = ctx.pointer_latest_pos() else { return };
        let m = 6.0;
        let (l, rr, t, b) = (p.x - r.left() < m, r.right() - p.x < m, p.y - r.top() < m, r.bottom() - p.y < m);
        let dir = match (l, rr, t, b) {
            (true, _, true, _) => Some(D::NorthWest),
            (_, true, true, _) => Some(D::NorthEast),
            (true, _, _, true) => Some(D::SouthWest),
            (_, true, _, true) => Some(D::SouthEast),
            (true, ..) => Some(D::West),
            (_, true, ..) => Some(D::East),
            (_, _, true, _) => Some(D::North),
            (_, _, _, true) => Some(D::South),
            _ => None,
        };
        if let Some(d) = dir {
            ctx.set_cursor_icon(match d {
                D::North | D::South => egui::CursorIcon::ResizeVertical,
                D::East | D::West => egui::CursorIcon::ResizeHorizontal,
                D::NorthWest | D::SouthEast => egui::CursorIcon::ResizeNwSe,
                _ => egui::CursorIcon::ResizeNeSw,
            });
            if ctx.input(|i| i.pointer.primary_pressed()) {
                ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(d));
            }
        }
    }

    fn handle_keys(&mut self, ctx: &egui::Context) {
        use egui::Key;
        // the focused suite app (Terminal / Photos) owns the keyboard
        if IS_WEB && self.focused_app() != crate::shell::AppId::Files {
            let (cmd, k, comma) = ctx.input(|i| (i.modifiers.command, i.key_pressed(Key::K), i.key_pressed(Key::Comma)));
            if cmd && k {
                self.act(Act::Palette);
            } else if cmd && comma {
                self.app_cmd("settings");
            }
            return;
        }
        let typing = ctx.egui_wants_keyboard_input();
        let (cmd, shift, alt) = ctx.input(|i| (i.modifiers.command, i.modifiers.shift, i.modifiers.alt));
        let pressed = |k: Key| ctx.input(|i| i.key_pressed(k));
        let events: Vec<egui::Event> = ctx.input(|i| i.events.clone());
        if cmd && pressed(Key::K) {
            self.act(Act::Palette);
            return;
        }
        if cmd && pressed(Key::T) && !typing {
            self.act(Act::NewTab(None));
            return;
        }
        if cmd && pressed(Key::W) && !typing && self.tabs.len() > 1 {
            self.act(Act::CloseTab(self.tab_i));
            return;
        }
        if cmd && pressed(Key::Comma) {
            self.act(Act::Settings);
            return;
        }
        if pressed(Key::Escape) && self.search.sheet {
            self.act(Act::SearchSheet(false));
            return;
        }
        if pressed(Key::Escape) {
            if self.palette.open {
                // palette handles its own Esc
            } else if self.menu.is_some() {
                self.menu = None;
            } else if !matches!(self.modal, Modal::None) {
                if let Modal::Preview(_) = self.modal {
                    self.be.audio.stop();
                }
                self.act(Act::CloseModal);
            } else if self.rename.is_some() {
                self.rename = None;
            } else if self.ai.open && !typing {
                self.ai.open = false;
            } else if self.search_active() {
                self.clear_search();
            } else if !typing {
                self.sel.clear();
                self.select_mode = false;
            }
            return;
        }
        if self.palette.open {
            return;
        }
        // preview navigation
        if let Modal::Preview(l) = self.modal.clone() {
            let list: Vec<Location> = self.items.iter().filter(|x| x.kind.previewable()).map(|x| x.loc.clone()).collect();
            if let Some(i) = list.iter().position(|x| *x == l) {
                if pressed(Key::ArrowRight) && i + 1 < list.len() {
                    self.act(Act::Preview(list[i + 1].clone()));
                }
                if pressed(Key::ArrowLeft) && i > 0 {
                    self.act(Act::Preview(list[i - 1].clone()));
                }
            }
            if pressed(Key::Space) && !typing {
                self.be.audio.stop();
                self.act(Act::CloseModal);
            }
            return;
        }
        if !matches!(self.modal, Modal::None) {
            return;
        }
        if alt && pressed(Key::ArrowLeft) {
            self.act(Act::Back);
        }
        if alt && pressed(Key::ArrowRight) {
            self.act(Act::Forward);
        }
        if cmd && pressed(Key::Backslash) {
            self.act(Act::ToggleSidebar);
        }
        if cmd && pressed(Key::F) {
            self.act(Act::FocusSearch);
        }
        if pressed(Key::F5) {
            self.act(Act::Refresh);
        }
        // window-level shortcuts work even while a text field has focus
        if cmd && pressed(Key::Q) {
            return self.act(Act::WinClose);
        }
        if pressed(Key::F11) {
            self.act(Act::Native(NativeCmd::ToggleFullscreen));
        }
        if pressed(Key::F1) {
            return self.act(Act::Modal(Modal::Shortcuts));
        }
        if cmd && !shift && pressed(Key::N) && !typing {
            self.act(Act::Native(NativeCmd::NewWindow));
        }
        if cmd && pressed(Key::Tab) {
            self.act(Act::NextTab(!shift));
            return;
        }
        if alt && pressed(Key::Home) {
            self.act(Act::Home);
        }
        if typing {
            return;
        }
        if cmd && !shift && pressed(Key::Z) {
            self.act(Act::Undo);
        }
        if cmd && shift && pressed(Key::C) {
            self.act(Act::CopyPath);
        }
        if cmd && shift && pressed(Key::I) {
            self.act(Act::InvertSelection);
        } else if cmd && pressed(Key::I) {
            self.act(Act::Info);
        }
        if cmd && (pressed(Key::Plus) || pressed(Key::Equals)) {
            let z = (self.st().thumb_scale + 0.1).clamp(0.75, 1.4);
            self.act(Act::SetGridZoom(z));
        }
        if cmd && pressed(Key::Minus) {
            let z = (self.st().thumb_scale - 0.1).clamp(0.75, 1.4);
            self.act(Act::SetGridZoom(z));
        }
        if cmd && pressed(Key::O) {
            self.act(Act::OpenSel);
        }
        // Home / End / PageUp / PageDown move the focus like every other file manager
        if !self.items.is_empty() && !cmd && !alt {
            let n = self.items.len();
            let page = if self.st().grid { (self.cols.max(1) * 3) as i32 } else { 10 };
            let cur = self.focus.unwrap_or(0) as i32;
            let to = if pressed(Key::Home) { Some(0) } else if pressed(Key::End) { Some(n as i32 - 1) } else if pressed(Key::PageDown) { Some(cur + page) } else if pressed(Key::PageUp) { Some(cur - page) } else { None };
            if let Some(t) = to {
                let next = t.clamp(0, n as i32 - 1) as usize;
                self.focus = Some(next);
                if shift {
                    let a = self.anchor.unwrap_or(cur as usize);
                    let (lo, hi) = (a.min(next), a.max(next));
                    self.sel = self.items[lo..=hi].iter().map(|e| e.loc.clone()).collect();
                } else {
                    self.sel = [self.items[next].loc.clone()].into_iter().collect();
                    self.anchor = Some(next);
                }
                self.scroll_to = Some(next);
            }
        }
        for e in &events {
            match e {
                egui::Event::Copy if !shift => self.act(Act::Copy),
                egui::Event::Cut => self.act(Act::Cut),
                egui::Event::Paste(_) => self.act(Act::Paste),
                _ => {}
            }
        }
        if cmd && pressed(Key::A) {
            self.act(Act::SelectAll);
        }
        if cmd && pressed(Key::D) {
            self.act(Act::Duplicate);
        }
        if cmd && pressed(Key::H) {
            self.act(Act::ToggleHidden);
        }
        if cmd && shift && pressed(Key::N) {
            self.act(Act::NewFolder);
        }
        if pressed(Key::F2) {
            if let Some(e) = self.selected().first() {
                self.act(Act::Rename(e.loc.clone()));
            }
        }
        if pressed(Key::Delete) || (cmd && pressed(Key::Backspace)) {
            self.act(if shift { Act::DeleteForever } else { Act::Trash });
        }
        if pressed(Key::Enter) {
            self.act(Act::OpenSel);
        }
        if pressed(Key::Space) {
            let s = self.selected();
            if s.len() == 1 && s[0].kind.previewable() {
                self.act(Act::Preview(s[0].loc.clone()));
            } else if s.len() == 1 && s[0].kind == smart_core::Kind::Audio {
                self.act(Act::TogglePlayPause(s[0].loc.clone()));
            }
        }
        if (pressed(Key::Backspace) && !cmd) || (alt && pressed(Key::ArrowUp)) {
            self.act(Act::Up);
        }
        // arrow navigation
        if !self.search_active() && !self.items.is_empty() {
            let grid = self.st().grid;
            let cols = if grid { self.cols as i32 } else { 1 };
            let mut d = 0i32;
            if pressed(Key::ArrowRight) && grid { d = 1 }
            if pressed(Key::ArrowLeft) && grid { d = -1 }
            if pressed(Key::ArrowDown) { d = cols }
            if pressed(Key::ArrowUp) { d = -cols }
            if !alt && d != 0 {
                let n = self.items.len() as i32;
                let cur = self.focus.map(|f| f as i32).unwrap_or(if d > 0 { -cols.min(1) } else { n });
                let next = (cur + d).clamp(0, n - 1) as usize;
                self.focus = Some(next);
                let loc = self.items[next].loc.clone();
                if shift {
                    let a = self.anchor.unwrap_or(next);
                    let (lo, hi) = (a.min(next), a.max(next));
                    self.sel = self.items[lo..=hi].iter().map(|e| e.loc.clone()).collect();
                } else {
                    self.sel = [loc].into_iter().collect();
                    self.anchor = Some(next);
                }
                self.scroll_to = Some(next);
            }
        }
        // single-key shortcuts (match the status-bar hints) and type-to-search
        if !cmd && !alt {
            if pressed(Key::G) { return self.act(Act::SetGrid(true)); }
            if pressed(Key::L) { return self.act(Act::SetGrid(false)); }
            if pressed(Key::D) { return self.act(Act::ToggleTheme); }
            if pressed(Key::T) { return self.act(Act::Modal(Modal::Tokens)); }
            if pressed(Key::Questionmark) { return self.act(Act::Modal(Modal::Shortcuts)); }
            if pressed(Key::Slash) { return self.act(Act::FocusSearch); }
            for e in &events {
                if let egui::Event::Text(t) = e {
                    if t.chars().all(|c| !c.is_control()) && !t.trim().is_empty() && self.rename.is_none() {
                        self.search.query = t.clone();
                        self.search.dirty_at = self.time;
                        self.search.focus_next = true;
                        self.run_search();
                        return;
                    }
                }
            }
        }
    }
}

pub fn make_app(cc: &eframe::CreationContext<'_>) -> SmartApp {
    let mut a = SmartApp::new(cc, None);
    a.seed_demo_tags();
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(p) = native::START.get() {
        let path = std::fs::canonicalize(p).unwrap_or_else(|_| p.into());
        if path.is_dir() {
            a.act(Act::Nav(Route::Loc(Location::file(path.to_string_lossy().replace('\\', "/")))));
        } else {
            a.toast(format!("\u{201c}{p}\u{201d} is not a folder"), false);
        }
    }
    #[cfg(target_arch = "wasm32")]
    deep_link(&mut a);
    a
}

/// `#terminal=ls;neofetch`, `#photos`, `#photo` / `#photo=edit` / `#photo=info`, `#menu` — open the suite in a given state
/// (handy for sharing a view and for automated screenshots).
#[cfg(target_arch = "wasm32")]
fn deep_link(a: &mut SmartApp) {
    use crate::shell::AppId;
    let h = web_sys::window().and_then(|w| w.location().hash().ok()).unwrap_or_default();
    let h = h.trim_start_matches('#');
    let (name, arg) = h.split_once('=').unwrap_or((h, ""));
    match name {
        "terminal" => {
            a.open_app(AppId::Terminal);
            a.shell.term.command("new");
            for c in arg.split(';').filter(|c| !c.is_empty()) {
                a.term_submit(0, c.replace("%20", " "));
            }
        }
        "photos" => a.open_app(AppId::Photos),
        "top" => {
            a.open_app(AppId::Terminal);
            a.shell.term.command("new");
            a.term_submit(0, "top".into());
        }
        "palette" => {
            match arg {
                "photos" => a.open_app(AppId::Photos),
                "terminal" => a.open_app(AppId::Terminal),
                _ => {}
            }
            a.act(Act::Palette);
        }
        "settings" => {
            let app = if arg == "terminal" { AppId::Terminal } else { AppId::Photos };
            a.open_app(app);
            a.shell.settings = Some((app, 0));
        }
        "photomenu" => {
            a.open_app(AppId::Photos);
            if arg == "viewer" {
                if let Some(l) = a.photos_first() {
                    a.photos_show(l);
                }
                let items = a.photos_viewer_menu();
                a.open_app_menu(egui::pos2(760.0, 300.0), items);
            } else {
                let items = a.photos_library_menu();
                a.open_app_menu(egui::pos2(760.0, 300.0), items);
            }
        }
        "termmenu" => {
            a.open_app(AppId::Terminal);
            a.shell.term.command("new");
            a.term_submit(0, "ls".into());
            let items = a.term_menu_main();
            a.open_app_menu(egui::pos2(900.0, 300.0), items);
        }
        "photo" => {
            a.open_app(AppId::Photos);
            let first = a.photos_first();
            if let Some(l) = first {
                a.photos_show(l);
            }
            a.shell.photos.edit = arg == "edit";
            a.shell.photos.info = arg == "info";
            if arg == "edit" {
                a.shell.photos.adj = crate::photos::Adjust { exposure: 0.1, contrast: 0.25, saturation: 0.3, warmth: 0.3, vignette: 0.4 };
                a.shell.photos.dirty = true;
            }
        }
        "copy" => {
            // starts the paced 5 GB demo transfer so the dialog can be seen straight away
            let mut stack = vec![a.be.home()];
            let mut found = None;
            while let Some(d) = stack.pop() {
                if let Some(fs) = a.be.registry.get(&d.fs) {
                    if let Ok(l) = fs.list(&d.path) {
                        for e in l {
                            if e.paced {
                                found = Some(e.loc.clone());
                            } else if e.is_dir {
                                stack.push(e.loc.clone());
                            }
                        }
                    }
                }
                if found.is_some() {
                    break;
                }
            }
            let dest = a.places.iter().find(|p| p.id == "downloads").map(|p| p.loc.clone());
            if let (Some(src), Some(d)) = (found, dest) {
                a.submit_job(smart_core::JobKind::Copy { srcs: vec![src], dest: d });
            }
        }
        "menu" => {
            a.menu = Some(Menu { kind: MenuKind::More, pos: egui::pos2(1250.0, 130.0), target: None, device: None, sub: None, age: 0.0 });
        }
        _ => {}
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Native-only plumbing: command line, crash log, window mode.
#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::OnceLock;

    static OPAQUE: AtomicBool = AtomicBool::new(false);
    pub static START: OnceLock<String> = OnceLock::new();

    pub fn opaque() -> bool {
        OPAQUE.load(Ordering::Relaxed)
    }

    pub const HELP: &str = "Smart Files — a fast, fault-tolerant file manager written in Rust

USAGE:  smart-files [OPTIONS] [FOLDER]

OPTIONS:
    --opaque       Draw an opaque window (use when your compositor can't do transparency)
    --demo         Browse a built-in in-memory sample world instead of your real files
    --crash-log    Print the path of the crash log and exit
    -h, --help     Show this help
    -V, --version  Show the version

ENVIRONMENT:
    SMART_FILES_OPAQUE=1   same as --opaque
    WINIT_UNIX_BACKEND     force `x11` or `wayland`
    LIBGL_ALWAYS_SOFTWARE  =1 to force software rendering";

    pub fn crash_log_path() -> std::path::PathBuf {
        let base = std::env::var_os("XDG_STATE_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/state"))).unwrap_or_else(std::env::temp_dir);
        base.join("smart-files").join("crash.log")
    }

    /// Any panic anywhere (UI or worker thread) is appended to the crash log with a backtrace, then execution continues:
    /// workers turn it into a failed job, the UI thread recovers in `guarded_frame`.
    fn install_crash_hook() {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let path = crash_log_path();
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            let entry = format!("[{secs}] {info}\n{}\n\n", std::backtrace::Backtrace::force_capture());
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
                let _ = f.write_all(entry.as_bytes());
            }
            default(info);
        }));
    }

    pub fn run() -> eframe::Result {
        use eframe::egui;
        let mut args = std::env::args().skip(1);
        while let Some(a) = args.next() {
            match a.as_str() {
                "-h" | "--help" => {
                    println!("{HELP}");
                    return Ok(());
                }
                "-V" | "--version" => {
                    println!("smart-files {}", env!("CARGO_PKG_VERSION"));
                    return Ok(());
                }
                "--crash-log" => {
                    println!("{}", crash_log_path().display());
                    return Ok(());
                }
                "--opaque" => OPAQUE.store(true, Ordering::Relaxed),
                "--demo" => {}
                p if !p.starts_with('-') => {
                    let _ = START.set(p.to_string());
                }
                other => eprintln!("smart-files: ignoring unknown option {other} (try --help)"),
            }
        }
        if std::env::var_os("SMART_FILES_OPAQUE").is_some_and(|v| v != "0") {
            OPAQUE.store(true, Ordering::Relaxed);
        }
        install_crash_hook();
        let vp = egui::ViewportBuilder::default()
            .with_title("Smart Files")
            .with_app_id("smart-files")
            .with_inner_size([1440.0, 860.0])
            .with_min_inner_size([820.0, 540.0])
            .with_decorations(false)
            .with_transparent(!opaque())
            .with_icon(std::sync::Arc::new(app_icon()));
        eframe::run_native("Smart Files", eframe::NativeOptions { viewport: vp, ..Default::default() }, Box::new(|cc| Ok(Box::new(crate::make_app(cc)))))
    }

    fn app_icon() -> egui::IconData {
        let img = image::load_from_memory(include_bytes!("../../../assets/icon/smart-files-256.png")).map(|i| i.to_rgba8()).unwrap_or_else(|_| image::RgbaImage::new(1, 1));
        egui::IconData { width: img.width(), height: img.height(), rgba: img.into_raw() }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native() -> eframe::Result {
    native::run()
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    #[wasm_bindgen]
    pub async fn start(canvas_id: String) -> Result<(), JsValue> {
        console_error_panic_hook::set_once();
        let doc = web_sys::window().and_then(|w| w.document()).ok_or("no document")?;
        let canvas = doc.get_element_by_id(&canvas_id).ok_or("no canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
        eframe::WebRunner::new().start(canvas, eframe::WebOptions::default(), Box::new(|cc| Ok(Box::new(make_app(cc))))).await
    }
}
