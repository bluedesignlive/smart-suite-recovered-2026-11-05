//! The desktop surface: icons you can drag, context menus, OS drag-and-drop, the wallpaper dialog (images and video),
//! clipboard capture, notifications, compact desktop dialogs and the boot sequence.
use super::*;
use crate::copyfx::{folder_back, folder_front};
use crate::widgets::*;
use egui::{LayerId, Order, Vec2};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, PartialEq)]
pub enum NoteKind {
    Info,
    Ok,
    Warn,
    Err,
}

pub struct Note {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub kind: NoteKind,
    pub born: f64,
    pub ttl: f64,
}

pub struct Dialog {
    pub title: String,
    pub body: String,
    pub ok: String,
    pub cancel: bool,
    pub icon: &'static str,
    pub danger: bool,
    pub cmd: String,
    pub born: f64,
}

pub struct Picked {
    pub name: String,
    pub bytes: Vec<u8>,
    pub video_url: Option<String>,
}

#[derive(Default)]
pub struct DesktopState {
    pub items: Vec<(Location, String, bool, Kind)>,
    pub pos: HashMap<Location, Pos2>,
    pub sel: HashSet<Location>,
    pub loaded: f64,
    pub wall_open: bool,
    pub picker: Option<crate::picker::Picker>,
    pub inbox: Arc<Mutex<Vec<Picked>>>,
    pub dropbox: Arc<Mutex<Vec<(String, Vec<u8>)>>>,
    pub hover_files: f32,
}

impl Shell {
    pub fn notify(&mut self, title: &str, body: &str, kind: NoteKind) {
        self.push_note(title, body, kind, true);
    }
    pub fn notify_done(&mut self, title: &str, body: &str) {
        self.push_note(title, body, NoteKind::Ok, false);
    }
    pub fn push_note(&mut self, title: &str, body: &str, kind: NoteKind, sound: bool) {
        if self.focus {
            return;
        }
        if sound {
            self.snd.play(match kind {
                NoteKind::Ok => crate::sound::Snd::Success,
                NoteKind::Info => crate::sound::Snd::Notify,
                NoteKind::Warn => crate::sound::Snd::Warn,
                NoteKind::Err => crate::sound::Snd::Error,
            });
        }
        if self.tts_on {
            crate::sound::speak(&format!("{}. {}", title, body), self.tts_rate, self.volume.max(0.3));
        }
        self.note_id += 1;
        self.notes.push(Note { id: self.note_id, title: title.to_string(), body: body.to_string(), kind, born: self.now, ttl: 4.6 });
        if self.notes.len() > 5 {
            self.notes.remove(0);
        }
    }
    pub fn confirm(&mut self, title: &str, body: &str, ok: &str, icon: &'static str, danger: bool, cmd: &str) {
        self.dialog = Some(Dialog { title: title.into(), body: body.into(), ok: ok.into(), cancel: true, icon, danger, cmd: cmd.into(), born: self.now });
    }
}

#[cfg(target_arch = "wasm32")]
pub fn web_pick(accept: &str, video: bool, inbox: Arc<Mutex<Vec<Picked>>>) {
    use wasm_bindgen::{closure::Closure, JsCast};
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(el) = doc.create_element("input") else { return };
    let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() else { return };
    input.set_type("file");
    input.set_accept(accept);
    let inp2 = input.clone();
    let cb = Closure::once_into_js(move || {
        let Some(files) = inp2.files() else { return };
        let Some(f) = files.get(0) else { return };
        let name = f.name();
        if video {
            let url = web_sys::Url::create_object_url_with_blob(&f).unwrap_or_default();
            inbox.lock().unwrap().push(Picked { name, bytes: vec![], video_url: Some(url) });
        } else {
            let Ok(fr) = web_sys::FileReader::new() else { return };
            let fr2 = fr.clone();
            let inbox2 = inbox.clone();
            let onload = Closure::once_into_js(move || {
                if let Ok(buf) = fr2.result() {
                    let arr = js_sys::Uint8Array::new(&buf);
                    inbox2.lock().unwrap().push(Picked { name, bytes: arr.to_vec(), video_url: None });
                }
            });
            fr.set_onload(Some(onload.unchecked_ref()));
            let _ = fr.read_as_array_buffer(&f);
        }
    });
    input.set_onchange(Some(cb.unchecked_ref()));
    input.click();
}
#[cfg(not(target_arch = "wasm32"))]
pub fn web_pick(_accept: &str, _video: bool, _inbox: Arc<Mutex<Vec<Picked>>>) {}

#[cfg(target_arch = "wasm32")]
pub fn set_video(url: &str) {
    use wasm_bindgen::JsCast;
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Some(old) = doc.get_element_by_id("smart-bg-video") {
        old.remove();
    }
    let Ok(el) = doc.create_element("video") else { return };
    let Ok(v) = el.dyn_into::<web_sys::HtmlVideoElement>() else { return };
    v.set_id("smart-bg-video");
    v.set_src(url);
    v.set_loop(true);
    v.set_muted(true);
    v.set_autoplay(true);
    let _ = v.set_attribute("playsinline", "true");
    v.style().set_css_text("position:fixed;inset:0;width:100%;height:100%;object-fit:cover;z-index:0;background:#000");
    if let Some(b) = doc.body() {
        let _ = b.append_child(&v);
    }
    let _ = v.play();
}
#[cfg(not(target_arch = "wasm32"))]
pub fn set_video(_url: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn clear_video() {
    if let Some(e) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id("smart-bg-video")) {
        e.remove();
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub fn clear_video() {}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl SmartApp {
    pub fn clip_push(&mut self, text: &str, loc: Option<Location>) {
        let t = text.trim_end();
        if t.is_empty() {
            return;
        }
        if self.shell.clip.first().is_some_and(|c| c.text == t) {
            return;
        }
        self.shell.clip.retain(|c| c.text != t || c.pinned);
        self.shell.clip.insert(0, ClipItem { text: t.to_string(), files: 0, t: self.shell.now, pinned: false, loc });
        let mut unpinned = 0;
        self.shell.clip.retain(|c| {
            if c.pinned {
                true
            } else {
                unpinned += 1;
                unpinned <= 24
            }
        });
        self.shell.snd.play(crate::sound::Snd::Copy);
    }

    pub(crate) fn desktop_loc(&self) -> Option<Location> {
        self.places.iter().find(|p| p.id == "desktop").map(|p| p.loc.clone())
    }

    fn desk_refresh(&mut self) {
        let Some(dl) = self.desktop_loc() else { return };
        if let Some(fs) = self.be.registry.get(&dl.fs) {
            if let Ok(mut l) = fs.list(&dl.path) {
                l.retain(|e| !e.hidden);
                l.sort_by(|a, b| natural_cmp(&a.name, &b.name));
                self.shell.desk.items = l.into_iter().map(|e| (e.loc.clone(), e.name.clone(), e.is_dir, e.kind)).collect();
            }
        }
        self.shell.desk.loaded = self.shell.now;
    }

    fn desk_import(&mut self, files: Vec<(String, Vec<u8>)>) {
        let Some(dl) = self.desktop_loc() else { return };
        let Some(fs) = self.be.registry.get(&dl.fs) else { return };
        let mut n = 0;
        for (name0, b) in files {
            let mut name = name0;
            if name.is_empty() {
                name = "Dropped file".into();
            }
            if fs.exists(&dl.join(&name).path) {
                name = format!("copy of {name}");
            }
            if fs.write(&dl.join(&name).path, &b).is_ok() {
                n += 1;
            }
        }
        if n > 0 {
            self.shell.snd.play(crate::sound::Snd::Drop);
            self.shell.notify(&format!("Added {n} file{} to the Desktop", if n == 1 { "" } else { "s" }), "Dropped from your computer", NoteKind::Ok);
            self.desk_refresh();
        }
    }

    /// Per-frame shell housekeeping: time, histories, keyboard, clipboard, drops, async inboxes.
    pub(crate) fn shell_frame_begin(&mut self, ctx: &egui::Context, screen: Rect) {
        let now = ctx.input(|i| i.time);
        self.shell.now = now;
        self.shell.screen = screen;
        self.shell.snd.vol = self.shell.volume;
        if self.shell.boot_t0.is_none() {
            self.shell.boot_t0 = Some(now);
            self.shell.snd.play(crate::sound::Snd::Boot);
        }
        self.shell.lat = 38.0 + 6.0 * (now * 0.7).sin() as f32 + 3.0 * (now * 2.3).sin() as f32 + if self.shell.remote { 0.0 } else { -30.0 };
        let tick = (now * 2.0) as u64;
        let last: u64 = ctx.data_mut(|d| d.get_temp(Id::new("hist-tick")).unwrap_or(0));
        if tick != last {
            ctx.data_mut(|d| d.insert_temp(Id::new("hist-tick"), tick));
            let (lat, fps) = (self.shell.lat, 1.0 / ctx.input(|i| i.stable_dt).max(0.004));
            self.shell.lat_hist.push(lat);
            self.shell.fps_hist.push(fps.min(144.0));
            for v in [&mut self.shell.lat_hist, &mut self.shell.fps_hist] {
                if v.len() > 40 {
                    v.remove(0);
                }
            }
            self.shell.ai.speaking = crate::sound::is_speaking();
        }
        // keyboard
        let (cmd_space, esc, enter) = ctx.input(|i| (i.modifiers.command && i.key_pressed(egui::Key::Space), i.key_pressed(egui::Key::Escape), i.key_pressed(egui::Key::Enter)));
        if cmd_space {
            self.app_cmd("ai.toggle");
        }
        if self.shell.ai.open && !self.shell.ai.busy {
            // the command bar owns the keyboard while open: chars, paste and backspace land here directly
            for e in ctx.input(|i| i.events.clone()) {
                match e {
                    egui::Event::Text(t) if t.chars().all(|c| !c.is_control()) => self.shell.ai.input.push_str(&t),
                    egui::Event::Paste(t) => self.shell.ai.input.push_str(&t),
                    egui::Event::Key { key: egui::Key::Backspace, pressed: true, .. } => {
                        self.shell.ai.input.pop();
                    }
                    _ => {}
                }
            }
            if enter && !self.shell.ai.input.trim().is_empty() {
                let t = std::mem::take(&mut self.shell.ai.input);
                self.os_ai_send(t);
            }
        }
        if esc {
            if self.shell.ai.mode {
                self.shell.ai.mode = false;
            } else if self.shell.ai.open {
                self.shell.ai.open = false;
            } else if self.shell.desk.wall_open {
                self.shell.desk.wall_open = false;
            } else if self.shell.dialog.is_some() {
                self.shell.dialog = None;
            } else if self.shell.desk.picker.is_some() {
                self.shell.desk.picker = None;
            }
            self.shell.clip_open = false;
            self.shell.sys_open = false;
        }
        // clipboard: pasted text + file clipboard
        let pasted: Vec<String> = ctx.input(|i| i.events.iter().filter_map(|e| if let egui::Event::Paste(t) = e { Some(t.clone()) } else { None }).collect());
        for t in pasted {
            self.clip_push(&t, None);
        }
        let sig = self.clipboard.as_ref().map(|(v, cut)| v.len() * 2 + *cut as usize + v.first().map(|l| l.path.len()).unwrap_or(0) * 7).unwrap_or(0);
        if sig != self.shell.clip_files_sig {
            self.shell.clip_files_sig = sig;
            if let Some((v, cut)) = &self.clipboard {
                let names: Vec<String> = v.iter().map(|l| l.name()).collect();
                self.shell.clip.insert(0, ClipItem { text: names.join(", "), files: v.len(), t: now, pinned: false, loc: v.first().cloned() });
                self.shell.snd.play(if *cut { crate::sound::Snd::Pick } else { crate::sound::Snd::Copy });
            }
        }
        // OS drag and drop onto the desktop
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        for h in dropped {
            let name = h.path().file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let inbox = self.shell.desk.dropbox.clone();
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(b) = h.bytes_async().await {
                    inbox.lock().unwrap().push((name, b));
                }
            });
            #[cfg(not(target_arch = "wasm32"))]
            if let Ok(b) = h.bytes() {
                inbox.lock().unwrap().push((name, b));
            }
        }
        let files: Vec<(String, Vec<u8>)> = std::mem::take(&mut *self.shell.desk.dropbox.lock().unwrap());
        if !files.is_empty() {
            self.desk_import(files);
        }
        // browser file picker results (wallpaper image / video)
        let picked: Vec<Picked> = std::mem::take(&mut *self.shell.desk.inbox.lock().unwrap());
        for pk in picked {
            if let Some(url) = pk.video_url {
                set_video(&url);
                self.shell.video_bg = Some(pk.name.clone());
                self.shell.notify("Video wallpaper set", &pk.name, NoteKind::Ok);
            } else if let Ok(im) = image::load_from_memory(&pk.bytes) {
                let im = if im.width() > 3200 { im.resize(3200, 3200, image::imageops::FilterType::Triangle) } else { im };
                let rgba = im.to_rgba8();
                let sz = [rgba.width() as usize, rgba.height() as usize];
                let tex = ctx.load_texture("custom-wall", egui::ColorImage::from_rgba_unmultiplied(sz, rgba.as_raw()), egui::TextureOptions::LINEAR);
                self.custom_wall = Some((tex, [sz[0] as f32, sz[1] as f32]));
                self.set(|s| s.wallpaper = 8);
                clear_video();
                self.shell.video_bg = None;
                self.shell.notify("Wallpaper updated", &pk.name, NoteKind::Ok);
            } else {
                self.shell.notify("That file isn\u{2019}t an image I can read", &pk.name, NoteKind::Err);
            }
        }
        self.ai_pump(ctx);
        self.cap_pump(ctx);
        if now - self.shell.desk.loaded > 1.5 {
            self.desk_refresh();
        }
        // window focus change → soft tick
        let f = self.focused_app();
        if self.shell.focus_prev != Some(f) {
            if self.shell.focus_prev.is_some() {
                self.shell.snd.play(crate::sound::Snd::Tick);
            }
            self.shell.focus_prev = Some(f);
        }
        // notification aging
        self.shell.notes.retain(|n| now - n.born < n.ttl);
        // dock-level ctx
        self.shell.drag_win = if self.shell.drag_win.is_some() && ctx.input(|i| i.pointer.any_down()) { self.shell.drag_win } else { None };
    }

    pub(crate) fn shell_cmd(&mut self, cmd: &str) -> bool {
        let screen = self.shell.screen;
        if cmd.starts_with("ai.") {
            return self.ai_cmd(cmd);
        }
        if cmd.starts_with("cap.") {
            return self.cap_cmd(cmd);
        }
        if let Some(c) = cmd.strip_prefix("desktop.") {
            let dl = self.desktop_loc();
            match c {
                "newfolder" => {
                    if let Some(dl) = dl {
                        let _ = self.be.create_folder(&dl, "New folder");
                        self.desk_refresh();
                        self.shell.snd.play(crate::sound::Snd::Pick);
                    }
                }
                "newfile" => {
                    if let (Some(dl), true) = (dl.clone(), true) {
                        if let Some(fs) = self.be.registry.get(&dl.fs) {
                            let mut n = "Untitled.txt".to_string();
                            let mut i = 2;
                            while fs.exists(&dl.join(&n).path) {
                                n = format!("Untitled {i}.txt");
                                i += 1;
                            }
                            let _ = fs.write(&dl.join(&n).path, b"");
                        }
                        self.desk_refresh();
                        self.shell.snd.play(crate::sound::Snd::Pick);
                    }
                }
                "paste" => {
                    if let (Some(dl), Some((srcs, cut))) = (dl, self.clipboard.clone()) {
                        self.submit_job(if cut { JobKind::Move { srcs, dest: dl } } else { JobKind::Copy { srcs, dest: dl } });
                        self.shell.snd.play(crate::sound::Snd::Paste);
                    }
                }
                "wallpaper" => {
                    self.shell.desk.wall_open = true;
                    self.shell.snd.play(crate::sound::Snd::Open);
                }
                "pick_wall" => self.open_picker(crate::picker::PickPurpose::Wallpaper),
                "upload_img" => web_pick("image/*", false, self.shell.desk.inbox.clone()),
                "upload_vid" => web_pick("video/*", true, self.shell.desk.inbox.clone()),
                "cleanup" => {
                    self.shell.desk.pos.clear();
                    self.shell.snd.play(crate::sound::Snd::Snap);
                }
                _ => {
                    if let Some(path) = c.strip_prefix("copy:") {
                        self.pending_copy = Some(path.to_string());
                    }
                }
            }
            return true;
        }
        if let Some(c) = cmd.strip_prefix("dock.") {
            match c {
                "mag" => self.shell.dock.mag = if self.shell.dock.mag > 0.1 { 0.0 } else { 1.0 },
                "hide" => self.shell.dock.autohide = !self.shell.dock.autohide,
                "settings" => {
                    self.shell.set_page = 1;
                    self.open_app(AppId::Settings);
                }
                "void" => {
                    let n = self.shell.trash_n;
                    self.shell.dialog = Some(Dialog {
                        title: "The Void".into(),
                        body: if n == 0 { "Nothing is waiting here. Drag files onto the vortex to remove them.".into() } else { format!("{n} item{} are waiting. Emptying releases them for good.", if n == 1 { "" } else { "s" }) },
                        ok: if n == 0 { "Close".into() } else { "Empty".into() },
                        cancel: n > 0,
                        icon: "trash-bin-2",
                        danger: n > 0,
                        cmd: if n == 0 { "".into() } else { "dock.empty.go".into() },
                        born: self.shell.now,
                    });
                }
                "empty" => self.shell.confirm("Empty the Void?", "Everything inside is released permanently. This can\u{2019}t be undone.", "Empty", "trash-bin-2", true, "dock.empty.go"),
                "empty.go" => {
                    self.shell.trash_n = 0;
                    self.shell.trash_flash = self.shell.now;
                    self.shell.snd.play(crate::sound::Snd::Empty);
                }
                _ => {
                    if let Some(v) = c.strip_prefix("size:") {
                        self.shell.dock.size = v.parse().unwrap_or(46.0);
                        self.shell.snd.play(crate::sound::Snd::Toggle);
                    }
                }
            }
            return true;
        }
        if let Some(c) = cmd.strip_prefix("wm.") {
            let _ = screen;
            let f = self.focused_app();
            match c {
                "tile" => {
                    self.apply_layout("grid");
                }
                "cascade" => {
                    self.apply_layout("cascade");
                }
                "split" => {
                    self.apply_layout("split");
                }
                "focus" => {
                    self.apply_layout("focus");
                }
                "max" => {
                    self.snap_app(f, "max");
                }
                "min" => match f {
                    AppId::Files => self.win_min = true,
                    o => self.app_win_mut(o).min = true,
                },
                "close" => {
                    match f {
                        AppId::Files => self.win_closed = true,
                        o => self.app_win_mut(o).open = false,
                    }
                    self.shell.snd.play(crate::sound::Snd::Close);
                }
                _ => {}
            }
            return true;
        }
        if let Some(c) = cmd.strip_prefix("sys.") {
            match c {
                "settings" => self.open_app(AppId::Settings),
                "about" => {
                    self.shell.set_page = 7;
                    self.open_app(AppId::Settings);
                }
                "restart" => self.shell.confirm("Restart the session?", "Windows stay put; the shell restarts with the boot sequence.", "Restart", "restart", false, "sys.restart.go"),
                "restart.go" => self.shell.boot_t0 = None,
                _ => {}
            }
            return true;
        }
        if cmd == "clip.clear" {
            self.shell.clip.retain(|c| c.pinned);
            return true;
        }
        false
    }

    // ───────────── desktop surface ─────────────
    fn desk_icon(&mut self, ctx: &egui::Context, p: &egui::Painter, r: Rect, loc: &Location, name: &str, is_dir: bool, kind: Kind, sel: bool, lift: f32) {
        let th = self.th.clone();
        let tile = Rect::from_center_size(pos2(r.center().x, r.top() + 36.0 - lift), vec2(62.0, 62.0));
        if sel {
            grad_fill(p, r.shrink2(vec2(4.0, 0.0)), 14.0, alpha(th.accent, 0.30), alpha(th.accent, 0.16));
            rim(p, r.shrink2(vec2(4.0, 0.0)), 14.0, 1.0, alpha(Color32::WHITE, 0.5), true, 20.0);
        }
        if is_dir {
            folder_back(p, &th, tile.center() + vec2(0.0, 4.0), 56.0, 42.0, 0.0, 0.0, th.accent);
            folder_front(p, tile.center() + vec2(0.0, 4.0), 56.0, 42.0, 0.0, 0.0, th.accent);
        } else {
            let mut budget = 1u32;
            let thumb = if kind == Kind::Image { self.photos_thumb(ctx, loc, &mut budget) } else { None };
            shadow(p, tile, 12.0, 5.0 + lift, 12.0, -3.0, alpha(th.shadow, 0.4));
            if let Some((tex, sz)) = thumb {
                textured_rrect(p, tile, 12.0, tex, cover_uv(sz[0], sz[1], tile), Color32::WHITE);
                rim(p, tile, 12.0, 1.3, Color32::from_white_alpha(170), true, 30.0);
            } else {
                let cols = ["#6e9bff", "#59c79a", "#e5b05a", "#d86fb0", "#8f7cf5", "#ff8a6b"];
                let h = kind.label().bytes().fold(0usize, |a, b| a.wrapping_mul(31).wrapping_add(b as usize)) % cols.len();
                let c0 = hex(cols[h]);
                grad_fill(p, tile, 12.0, mix(c0, Color32::WHITE, 0.35), mix(c0, Color32::BLACK, 0.18));
                rim(p, tile, 12.0, 1.3, Color32::from_white_alpha(170), true, 30.0);
                self.ico(p, kind.icon(), Ic::Duo, tile.center() - vec2(0.0, 2.0), 28.0, Color32::WHITE);
                let ext = ext_of(name).to_uppercase();
                if !ext.is_empty() {
                    p.text(pos2(tile.center().x, tile.bottom() - 8.0), Align2::CENTER_CENTER, ext.chars().take(4).collect::<String>(), f_semi(8.5), Color32::from_white_alpha(230));
                }
            }
        }
        let label = self.fit(p, name, &f_semi(11.5), r.width() - 8.0);
        let lp = pos2(r.center().x, r.bottom() - 14.0);
        p.text(lp + vec2(0.0, 1.0), Align2::CENTER_CENTER, &label, f_semi(11.5), Color32::from_black_alpha(150));
        p.text(lp, Align2::CENTER_CENTER, &label, f_semi(11.5), Color32::WHITE);
    }

    pub(crate) fn draw_desktop_layer(&mut self, ui: &mut Ui, screen: Rect, ai_t: f32) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        if ai_t > 0.02 {
            return;
        }
        let area = Rect::from_min_max(pos2(screen.left(), screen.top() + TOPBAR_H), screen.max);
        let p = ui.painter().clone();
        let bg = ui.interact(area, Id::new("desk-bg"), Sense::click_and_drag());
        if bg.clicked() {
            self.shell.desk.sel.clear();
        }
        if bg.secondary_clicked() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                let st = &self.shell;
                let items = vec![
                    mi("add-folder", "New folder", Act::App("desktop.newfolder".into())),
                    mi("document-text", "New text file", Act::App("desktop.newfile".into())),
                    mi("clipboard", "Paste", Act::App("desktop.paste".into())).off(self.clipboard.is_none()).gap(),
                    mi("gallery-wide", "Change wallpaper\u{2026}", Act::App("desktop.wallpaper".into())),
                    mi("widget-4", "Clean up icons", Act::App("desktop.cleanup".into())),
                    mi("window-frame", "Tile windows", Act::App("wm.tile".into())).off(!AppId::ALL.iter().any(|a| self.is_visible(*a))).gap(),
                    mi("layers-minimalistic", "Cascade windows", Act::App("wm.cascade".into())).off(!AppId::ALL.iter().any(|a| self.is_visible(*a))),
                    mi("stars-minimalistic", format!("Ask {}\u{2026}", st.ai.name), Act::App("ai.toggle".into())).k("Ctrl+Space").gap(),
                    mi("terminal", "Open terminal here", Act::OpenApp(AppId::Terminal)),
                    mi("camera", "Capture screen\u{2026}", Act::OpenApp(AppId::Capture)),
                    mi("settings-minimalistic", "Desktop & dock settings\u{2026}", Act::App("dock.settings".into())).gap(),
                ];
                self.open_app_menu(pos, items);
                self.shell.snd.play(crate::sound::Snd::Menu);
            }
        }
        // icons
        let cell = vec2(92.0, 104.0);
        let rows = (((area.height() - 130.0) / cell.y) as usize).max(1);
        let items = self.shell.desk.items.clone();
        let ptr = ctx.pointer_latest_pos();
        let mut trash: Vec<Location> = vec![];
        for (i, (loc, name, is_dir, kind)) in items.iter().enumerate() {
            let def = pos2(area.right() - cell.x - 14.0 - (i / rows) as f32 * cell.x, area.top() + 12.0 + (i % rows) as f32 * cell.y);
            let pos = *self.shell.desk.pos.get(loc).unwrap_or(&def);
            let r = Rect::from_min_size(pos, cell);
            let resp = ui.interact(r, Id::new(("desk-i", loc.path.clone())), Sense::click_and_drag());
            let sel = self.shell.desk.sel.contains(loc);
            let lift_t = spring_to(&ctx, ("desk-lift", loc.path.clone()), if resp.dragged() { 1.0 } else { 0.0 }, 300.0, 30.0).clamp(0.0, 1.0);
            if resp.hovered() && !sel {
                p.rect_filled(r.shrink2(vec2(4.0, 0.0)), cr(14.0), Color32::from_white_alpha(26));
            }
            if resp.clicked() {
                if !ctx.input(|i| i.modifiers.command) {
                    self.shell.desk.sel.clear();
                }
                self.shell.desk.sel.insert(loc.clone());
                self.shell.snd.play(crate::sound::Snd::Tick);
            }
            if resp.double_clicked() {
                self.acts.push(Act::Seq(vec![Act::OpenApp(AppId::Files), Act::Open(loc.clone())]));
            }
            if resp.dragged() {
                let e = self.shell.desk.pos.entry(loc.clone()).or_insert(def);
                *e += resp.drag_delta();
                if self.shell.desk.sel.len() <= 1 {
                    self.shell.desk.sel = [loc.clone()].into_iter().collect();
                }
                ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            }
            if resp.drag_started() {
                self.shell.snd.play(crate::sound::Snd::Pick);
            }
            if resp.drag_stopped() {
                if ptr.is_some_and(|pp| matches!(drop_target_at(pp), Some(DropTarget::Trash))) {
                    trash.push(loc.clone());
                } else if let Some(pp) = self.shell.desk.pos.get(loc).copied() {
                    let sx = ((pp.x - area.left() - 14.0) / cell.x).round() * cell.x + area.left() + 14.0;
                    let sy = ((pp.y - area.top() - 12.0) / cell.y).round() * cell.y + area.top() + 12.0;
                    self.shell.desk.pos.insert(loc.clone(), pos2(sx.clamp(area.left(), area.right() - cell.x), sy.clamp(area.top(), area.bottom() - cell.y - 70.0)));
                    self.shell.snd.play(crate::sound::Snd::Drop);
                }
            }
            if resp.secondary_clicked() {
                if let Some(pp) = ctx.pointer_latest_pos() {
                    self.shell.desk.sel = [loc.clone()].into_iter().collect();
                    let mut del = mi("trash-bin-minimalistic", "Move to the Void", Act::Job(JobKind::Trash { srcs: vec![loc.clone()] }));
                    del.danger = true;
                    let items = vec![
                        mi("arrow-right-up", "Open", Act::Seq(vec![Act::OpenApp(AppId::Files), Act::Open(loc.clone())])),
                        mi("copy", "Copy path", Act::App(format!("desktop.copy:{}", loc.path))),
                        mi("stars-minimalistic", format!("Ask {} about this", self.shell.ai.name), Act::App(format!("ai.ask:Tell me about the file {}", name))).gap(),
                        del.gap(),
                    ];
                    self.open_app_menu(pp, items);
                }
            }
            self.desk_icon(&ctx, &p, r, loc, name, *is_dir, *kind, sel, lift_t * 8.0);
        }
        if !trash.is_empty() {
            self.shell.trash_flash = self.shell.now;
            self.submit_job(JobKind::Trash { srcs: trash });
        }
        // files hovering over the browser window: show where they will land
        let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
        let hf = spring_to(&ctx, "desk-hover", if hovering { 1.0 } else { 0.0 }, 180.0, 22.0).clamp(0.0, 1.0);
        self.shell.desk.hover_files = hf;
        if hf > 0.01 {
            let fr = area.shrink(14.0 - 6.0 * hf);
            let pulse = 0.6 + 0.4 * (self.time * 4.0).sin() as f32;
            shadow(&p, fr, 22.0, 0.0, 30.0, 4.0, alpha(th.accent, 0.5 * hf * pulse));
            p.rect_stroke(fr, cr(22.0), egui::Stroke::new(2.5, alpha(th.accent_grad().0, 0.9 * hf)), egui::StrokeKind::Inside);
            let pill = Rect::from_center_size(fr.center(), vec2(280.0, 56.0));
            elevate(&p, &th, pill, 28.0, 5);
            grad_fill(&p, pill, 28.0, th.panel_top, th.panel_bottom);
            self.ico(&p, "download-minimalistic", Ic::Duo, pill.left_center() + vec2(30.0, 0.0), 22.0, th.accent_ink);
            p.text(pill.center() + vec2(14.0, -8.0), Align2::CENTER_CENTER, "Drop to add to Desktop", f_bold(T_BODY), th.ink1);
            p.text(pill.center() + vec2(14.0, 10.0), Align2::CENTER_CENTER, "files land as real icons", f_medium(T_META), th.ink3);
        }
    }

    /// Desktop drop of a Files drag (called from the central drag resolver).
    pub(crate) fn drop_on_desktop(&mut self, pos: Pos2, items: Vec<Location>, copy: bool) -> bool {
        if pos.y < TOPBAR_H || self.over_window(pos) {
            return false;
        }
        let Some(dl) = self.desktop_loc() else { return false };
        let srcs: Vec<Location> = items.into_iter().filter(|s| s.parent().as_ref() != Some(&dl)).collect();
        if srcs.is_empty() {
            return false;
        }
        let n = srcs.len();
        self.submit_job(if copy { JobKind::Copy { srcs, dest: dl } } else { JobKind::Move { srcs, dest: dl } });
        self.shell.snd.play(crate::sound::Snd::Drop);
        self.shell.notify(&format!("{} {n} item{} to the Desktop", if copy { "Copying" } else { "Moving" }, if n == 1 { "" } else { "s" }), "", NoteKind::Info);
        true
    }

    // ───────────── wallpaper dialog ─────────────
    pub(crate) fn draw_wallpaper_dialog(&mut self, ui: &mut Ui, screen: Rect) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let open = self.shell.desk.wall_open;
        let s = spring_to(&ctx, "wall-dlg", if open { 1.0 } else { 0.0 }, 280.0, 25.0);
        if s < 0.01 {
            return;
        }
        let (w, h) = (620.0f32.min(screen.width() - 24.0), 382.0f32.min(screen.height() - 80.0));
        let card = Rect::from_center_size(screen.center() + vec2(0.0, -8.0 + (1.0 - s.min(1.0)) * 18.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((s * 2.0).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, card, 24.0);
        cui.allocate_rect(card, Sense::click());
        let pad = 22.0;
        let tile = Rect::from_min_size(card.min + vec2(pad, 18.0), vec2(36.0, 36.0));
        let (a, b) = th.accent_grad();
        grad_fill(&p, tile, 12.0, a, b);
        rim(&p, tile, 12.0, 1.0, Color32::from_white_alpha(130), true, 16.0);
        self.ico(&p, "gallery-wide", Ic::Duo, tile.center(), 19.0, Color32::WHITE);
        p.text(pos2(tile.right() + 12.0, tile.center().y - 8.0), Align2::LEFT_CENTER, "Desktop wallpaper", f_bold(T_TITLE), th.ink1);
        p.text(pos2(tile.right() + 12.0, tile.center().y + 10.0), Align2::LEFT_CENTER, "Pictures or video. Nothing is laid over your desktop.", f_medium(T_META), th.ink3);
        let x = Rect::from_center_size(pos2(card.right() - 30.0, tile.center().y), vec2(28.0, 28.0));
        let mut close = self.icon_button(&cui, x, "close-circle", "wall-x", Btn::Quiet, true).clicked();
        let n = self.walls.len();
        let cols = 4usize;
        let gap = 10.0;
        let tw_ = (w - 2.0 * pad - gap * (cols as f32 - 1.0)) / cols as f32;
        let th_ = tw_ * 0.62;
        let cur = self.st().wallpaper;
        let mut pick: Option<usize> = None;
        let total = n + usize::from(self.custom_wall.is_some());
        let grid_top = card.top() + 70.0;
        for i in 0..total {
            let r = Rect::from_min_size(pos2(card.left() + pad + (i % cols) as f32 * (tw_ + gap), grid_top + (i / cols) as f32 * (th_ + gap)), vec2(tw_, th_));
            let resp = self.hit(&cui, r, ("wall-t", i), Sense::click());
            let hv = self.anim(&cui, ("wall-h", i), resp.hovered());
            let rr = r.expand(2.0 * hv);
            let is_custom = i == n;
            let (tex, sz, name) = if is_custom {
                let (t, sz) = self.custom_wall.as_ref().map(|(t, sz)| (t.id(), *sz)).unwrap();
                (t, sz, "Custom".to_string())
            } else {
                (self.walls[i].0.id(), self.walls[i].2, WALLPAPERS[i.min(WALLPAPERS.len() - 1)].0.to_string())
            };
            shadow(&p, rr, 12.0, 3.0 + 3.0 * hv, 10.0, -2.0, alpha(th.shadow, 0.3));
            textured_rrect(&p, rr, 12.0, tex, cover_uv(sz[0], sz[1], rr), Color32::WHITE);
            rim(&p, rr, 12.0, 1.2, Color32::from_white_alpha(150), true, 24.0);
            let sel_i = if is_custom { 8 } else { i };
            let on = cur == sel_i && self.shell.video_bg.is_none();
            if on {
                p.rect_stroke(rr.expand(1.5), cr(13.0), egui::Stroke::new(2.2, th.accent), egui::StrokeKind::Outside);
                let ck = Rect::from_center_size(pos2(rr.right() - 14.0, rr.top() + 14.0), vec2(18.0, 18.0));
                p.circle_filled(ck.center(), 9.0, th.accent);
                self.ico(&p, "check-circle", Ic::Filled, ck.center(), 14.0, Color32::WHITE);
            }
            let lw = self.tw(&p, &name, &f_semi(10.0)) + 16.0;
            let lab = Rect::from_min_size(pos2(rr.left() + 6.0, rr.bottom() - 22.0), vec2(lw, 17.0));
            p.rect_filled(lab, cr(8.5), Color32::from_black_alpha(120));
            p.text(lab.center(), Align2::CENTER_CENTER, &name, f_semi(10.0), Color32::WHITE);
            if resp.clicked() {
                pick = Some(sel_i);
            }
        }
        if let Some(i) = pick {
            self.acts.push(Act::SetWallpaper(i));
            clear_video();
            self.shell.video_bg = None;
            self.shell.snd.play(crate::sound::Snd::Pick);
        }
        if let Some(v) = &self.shell.video_bg {
            p.text(pos2(card.left() + pad, card.bottom() - 100.0), Align2::LEFT_CENTER, format!("\u{25b6}  Video wallpaper: {}", self.fit(&p, v, &f_semi(T_META), 380.0)), f_semi(T_META), th.accent_ink);
        }
        let by = card.bottom() - 82.0 + 14.0;
        let btns: [(&str, &str, &str, &str); 3] = [("folder-open", "From Files\u{2026}", "wall-b1", "desktop.pick_wall"), ("gallery-add", "Upload image\u{2026}", "wall-b2", "desktop.upload_img"), ("videocamera-record", "Video\u{2026}", "wall-b3", "desktop.upload_vid")];
        let mut bx = card.left() + pad;
        for (ic, lab, id, cmd) in btns {
            let bw = self.tw(&p, lab, &f_semi(T_BODY)) + 56.0;
            if self.label_button(&cui, Rect::from_min_size(pos2(bx, by), vec2(bw, 34.0)), Some(ic), lab, id, Btn::Raised).clicked() {
                self.app_cmd(cmd);
            }
            bx += bw + 8.0;
        }
        if self.label_button(&cui, Rect::from_min_size(pos2(card.right() - pad - 84.0, by), vec2(84.0, 34.0)), None, "Done", "wall-done", Btn::Accent).clicked() {
            close = true;
        }
        p.text(pos2(card.left() + pad, card.bottom() - 20.0), Align2::LEFT_CENTER, "JPG \u{b7} PNG \u{b7} WebP \u{b7} GIF \u{b7} BMP images, MP4 \u{b7} WebM video", f_medium(10.5), th.ink4);
        if close || (ctx.input(|i| i.pointer.primary_pressed()) && ctx.pointer_interact_pos().is_some_and(|pp| !card.contains(pp)) && self.shell.desk.picker.is_none()) {
            self.shell.desk.wall_open = false;
        }
    }

    // ───────────── notifications ─────────────
    pub(crate) fn draw_notes(&mut self, ui: &mut Ui, screen: Rect) {
        let th = self.th.clone();
        let now = self.shell.now;
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        let p = cui.painter().clone();
        let (w, h) = (324.0, 62.0);
        let mut kill: Option<u64> = None;
        let notes: Vec<(u64, String, String, NoteKind, f64, f64)> = self.shell.notes.iter().map(|n| (n.id, n.title.clone(), n.body.clone(), n.kind, n.born, n.ttl)).collect();
        for (i, (id, title, body, kind, born, ttl)) in notes.iter().enumerate() {
            let age = (now - born) as f32;
            ui.ctx().request_repaint();
            let slide = spring_out(age.min(1.2) * 1.2).min(1.05);
            let out = ((*ttl as f32 - age) / 0.35).clamp(0.0, 1.0);
            let x = screen.right() - w - 12.0 + (1.0 - slide) * (w + 20.0) + (1.0 - out) * 40.0;
            let y = TOPBAR_H + 10.0 + i as f32 * (h + 8.0);
            let r = Rect::from_min_size(pos2(x, y), vec2(w, if body.is_empty() { 50.0 } else { h }));
            let mut c2 = cui.new_child(egui::UiBuilder::new().max_rect(screen));
            c2.set_opacity(out);
            let p2 = c2.painter().clone();
            elevate(&p2, &th, r, 18.0, 4);
            grad_fill(&p2, r, 18.0, th.panel_top, th.panel_bottom);
            rim(&p2, r, 18.0, 1.3, th.rim, true, 30.0);
            let col = match kind {
                NoteKind::Ok => th.good,
                NoteKind::Info => th.accent,
                NoteKind::Warn => th.warn,
                NoteKind::Err => th.danger,
            };
            let orb = Rect::from_center_size(pos2(r.left() + 28.0, r.center().y), vec2(30.0, 30.0));
            grad_fill(&p2, orb, 15.0, mix(col, Color32::WHITE, 0.3), mix(col, Color32::BLACK, 0.15));
            rim(&p2, orb, 15.0, 1.0, Color32::from_white_alpha(150), true, 15.0);
            let ic = match kind {
                NoteKind::Ok => "check-circle",
                NoteKind::Info => "info-circle",
                NoteKind::Warn => "bell",
                NoteKind::Err => "close-circle",
            };
            self.ico(&p2, ic, Ic::Filled, orb.center(), 16.0, Color32::WHITE);
            let ty = if body.is_empty() { r.center().y } else { r.center().y - 9.0 };
            p2.text(pos2(r.left() + 52.0, ty), Align2::LEFT_CENTER, self.fit(&p2, title, &f_semi(T_BODY), w - 76.0), f_semi(T_BODY), th.ink1);
            if !body.is_empty() {
                p2.text(pos2(r.left() + 52.0, r.center().y + 10.0), Align2::LEFT_CENTER, self.fit(&p2, body, &f_medium(T_META), w - 76.0), f_medium(T_META), th.ink3);
            }
            let left = (1.0 - age / *ttl as f32).clamp(0.0, 1.0);
            p2.rect_filled(Rect::from_min_size(pos2(r.left() + 16.0, r.bottom() - 4.0), vec2((w - 32.0) * left, 2.0)), cr(1.0), alpha(col, 0.55));
            let resp = self.hit(&c2, r, ("note", *id), Sense::click());
            if resp.clicked() {
                kill = Some(*id);
            }
            let _ = &p;
        }
        if let Some(id) = kill {
            self.shell.notes.retain(|n| n.id != id);
        }
    }

    // ───────────── compact dialog ─────────────
    pub(crate) fn draw_dialog(&mut self, ui: &mut Ui, screen: Rect) {
        let Some(d) = self.shell.dialog.as_ref() else { return };
        let (title, body, ok, cancel, icon, danger, cmd, born) = (d.title.clone(), d.body.clone(), d.ok.clone(), d.cancel, d.icon, d.danger, d.cmd.clone(), d.born);
        let th = self.th.clone();
        let ctx = ui.ctx().clone();
        let age = (self.shell.now - born) as f32;
        let pop = spring_out(age).min(1.05);
        ctx.request_repaint();
        let w = 380.0;
        let galley = ui.painter().layout(body.clone(), f_medium(T_BODY), th.ink2, w - 48.0);
        let h = 130.0 + galley.size().y;
        let card = Rect::from_center_size(screen.center() + vec2(0.0, -screen.height() * 0.08 + (1.0 - pop) * 16.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((age * 8.0).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, card, 22.0);
        cui.allocate_rect(card, Sense::click());
        let col = if danger { th.danger } else { th.accent };
        let orb = Rect::from_min_size(card.min + vec2(22.0, 20.0), vec2(40.0, 40.0));
        shadow(&p, orb, 14.0, 4.0, 10.0, -2.0, alpha(col, 0.4));
        grad_fill(&p, orb, 14.0, mix(col, Color32::WHITE, 0.3), mix(col, Color32::BLACK, 0.12));
        rim(&p, orb, 14.0, 1.1, Color32::from_white_alpha(150), true, 18.0);
        self.ico(&p, icon, Ic::Duo, orb.center(), 21.0, Color32::WHITE);
        p.text(pos2(orb.right() + 14.0, orb.center().y), Align2::LEFT_CENTER, &title, f_bold(T_TITLE), th.ink1);
        p.galley(pos2(card.left() + 24.0, orb.bottom() + 14.0), galley, th.ink2);
        let by = card.bottom() - 18.0 - 32.0;
        let mut done = false;
        if self.label_button(&cui, Rect::from_min_size(pos2(card.right() - 22.0 - 100.0, by), vec2(100.0, 32.0)), None, &ok, "dlg-ok", if danger { Btn::Danger } else { Btn::Accent }).clicked() {
            if !cmd.is_empty() {
                self.acts.push(Act::App(cmd.clone()));
            }
            done = true;
        }
        if cancel && self.label_button(&cui, Rect::from_min_size(pos2(card.right() - 22.0 - 100.0 - 8.0 - 84.0, by), vec2(84.0, 32.0)), None, "Cancel", "dlg-cancel", Btn::Raised).clicked() {
            done = true;
        }
        if done {
            self.shell.dialog = None;
            self.shell.snd.play(crate::sound::Snd::Click);
        }
    }

    // ───────────── boot ─────────────
    pub(crate) fn draw_boot(&mut self, ui: &mut Ui, screen: Rect) {
        let Some(t0) = self.shell.boot_t0 else { return };
        let t = (self.shell.now - t0) as f32;
        let total = 4.0;
        if t > total {
            return;
        }
        let ctx = ui.ctx().clone();
        ctx.request_repaint();
        let reveal = smooth((t - 3.2) / 0.8);
        let a = 1.0 - reveal;
        let lp = ctx.layer_painter(LayerId::new(Order::Tooltip, Id::new("boot")));
        if t < 3.6 {
            ui.interact(screen, Id::new("boot-block"), Sense::click_and_drag());
        }
        let ctr = screen.center() + vec2(0.0, -34.0);
        // background: deep gradient + slow aurora blobs
        let mut m = egui::epaint::Mesh::default();
        for (y, col) in [(screen.top(), c(10, 16, 36, a)), (screen.bottom(), c(3, 5, 11, a))] {
            m.vertices.push(egui::epaint::Vertex { pos: pos2(screen.left(), y), uv: egui::epaint::WHITE_UV, color: col });
            m.vertices.push(egui::epaint::Vertex { pos: pos2(screen.right(), y), uv: egui::epaint::WHITE_UV, color: col });
        }
        m.indices.extend([0, 1, 2, 1, 3, 2]);
        lp.add(egui::Shape::mesh(m));
        for (k, col) in [(0.0f32, c(80, 120, 255, 1.0)), (2.1, c(180, 90, 255, 1.0)), (4.2, c(60, 220, 200, 1.0))] {
            let c0 = ctr + vec2((t * 0.5 + k).sin() * 260.0, (t * 0.4 + k * 1.3).cos() * 120.0 - 40.0);
            for i in 0..7 {
                lp.circle_filled(c0, 260.0 - i as f32 * 30.0, alpha(col, 0.012 * a));
            }
        }
        // logo tile with depth
        let sc = 1.0 + reveal * 0.45 + 0.025 * (t * 2.2).sin();
        let logo = Rect::from_center_size(ctr, vec2(92.0, 92.0) * sc);
        let th = Theme::dark();
        shadow(&lp, logo, 28.0, 14.0, 34.0, -4.0, alpha(th.accent, 0.5 * a));
        grad_fill(&lp, logo, 28.0 * sc, alpha(hex("#86acff"), a), alpha(hex("#3a62d8"), a));
        rim(&lp, logo, 28.0 * sc, 1.6, c(255, 255, 255, 0.7 * a), true, 60.0);
        grad_fill(&lp, Rect::from_min_max(logo.min + vec2(6.0, 5.0), pos2(logo.max.x - 6.0, logo.top() + 44.0 * sc)), 22.0, c(255, 255, 255, 0.22 * a), c(255, 255, 255, 0.0));
        lp.text(logo.center() + vec2(0.0, 2.0), Align2::CENTER_CENTER, "S", f_bold(52.0 * sc), c(255, 255, 255, a));
        // skeuomorphic ring + live steps
        let frac = smooth(t / 3.15);
        if a > 0.01 {
            let fade = a;
            let ring_c = ctr + vec2(0.0, 118.0);
            ring_gauge(&lp.with_clip_rect(screen), &th, ring_c, 30.0, frac, &format!("{}", (frac * 100.0) as u32), t as f64);
            let steps = ["Starting the compositor", "Mounting Smart Drive", "Waking the shell", "Joining your session", "Waking your assistant"];
            let si = ((frac * steps.len() as f32) as usize).min(steps.len() - 1);
            let last: usize = ctx.data_mut(|d| d.get_temp(Id::new("boot-step")).unwrap_or(99));
            if last != si {
                ctx.data_mut(|d| d.insert_temp(Id::new("boot-step"), si));
                if si > 0 {
                    self.shell.snd.play(crate::sound::Snd::Tick);
                }
            }
            lp.text(ring_c + vec2(0.0, 52.0), Align2::CENTER_CENTER, steps[si], f_semi(12.5), c(200, 214, 245, 0.9 * fade));
            lp.text(ring_c + vec2(0.0, 72.0), Align2::CENTER_CENTER, "SMART DESKTOP", f_semi(9.5), c(140, 156, 190, 0.8 * fade));
        }
        let _: Vec2 = vec2(0.0, 0.0);
    }
}
