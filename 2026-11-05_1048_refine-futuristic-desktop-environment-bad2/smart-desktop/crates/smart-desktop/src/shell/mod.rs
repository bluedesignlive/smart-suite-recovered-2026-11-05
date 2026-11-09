//! Smart Desktop shell. The shell owns the *scene graph*: every window is a node with animated geometry
//! (springs, so snaps, tiles and AI relayouts are seen moving in real time), a depth (z → shadow + parallax)
//! and an owner app. Apps never composite themselves — they draw into the frame the shell gives them.
//!   mod.rs     window manager entry: app registry, pipeline order, command routing
//!   scene.rs   animated geometry, snapping, layouts
//!   frame.rs   the unified app window chrome
//!   dock.rs    dock (magnification, previews, the Void trash)
//!   bar.rs     status bar, quick settings, clipboard + system popovers
//!   desktop.rs desktop surface, icons, context menus, drag & drop, wallpaper, notifications, dialogs, boot
//!   ai.rs      the OS-level assistant engine (providers, tools, memory)
use crate::app::*;
use crate::chrome::*;
use crate::icons::Ic;
use crate::overlays::{mi, MI};
use crate::paint::*;
use crate::sound::Snd;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Id, Pos2, Rect, Sense, Ui};
use smart_core::*;
use std::collections::HashMap;

pub mod ai;
pub mod bar;
pub mod desktop;
pub mod dock;
pub mod frame;
pub mod scene;

pub use desktop::{Dialog, Note, NoteKind};

pub const TOPBAR_H: f32 = 32.0;
pub const DOCK_H: f32 = 98.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppId {
    Files,
    Photos,
    Terminal,
    Capture,
    Settings,
    Studio,
}

impl AppId {
    pub fn name(self) -> &'static str {
        match self {
            AppId::Files => "Files",
            AppId::Photos => "Photos",
            AppId::Terminal => "Terminal",
            AppId::Capture => "Capture",
            AppId::Settings => "Settings",
            AppId::Studio => "Studio",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            AppId::Files => "folder",
            AppId::Photos => "gallery",
            AppId::Terminal => "terminal",
            AppId::Capture => "camera",
            AppId::Settings => "settings-minimalistic",
            AppId::Studio => "magic-stick-3",
        }
    }
    pub fn from_name(s: &str) -> Option<AppId> {
        let s = s.to_lowercase();
        Some(match s.as_str() {
            "files" | "file" | "finder" | "folders" | "file manager" => AppId::Files,
            "photos" | "photo" | "pictures" | "gallery" | "images" => AppId::Photos,
            "terminal" | "shell" | "console" | "term" | "smart-shell" => AppId::Terminal,
            "capture" | "screenshot" | "record" | "recorder" | "screen" => AppId::Capture,
            "settings" | "preferences" | "prefs" | "system settings" => AppId::Settings,
            "studio" | "apps" | "generated" => AppId::Studio,
            _ => return None,
        })
    }
    pub const ALL: [AppId; 6] = [AppId::Files, AppId::Photos, AppId::Terminal, AppId::Capture, AppId::Settings, AppId::Studio];
}

#[derive(Clone)]
pub struct AppWin {
    pub open: bool,
    pub min: bool,
    pub rect: Rect,
    pub restore: Option<Rect>,
    pub born: f64,
    pub placed: bool,
    pub side: bool,
}

impl AppWin {
    pub fn new(w: f32, h: f32, open: bool) -> Self {
        Self { open, min: false, rect: Rect::from_min_size(pos2(0.0, 0.0), vec2(w, h)), restore: None, born: 0.0, placed: false, side: true }
    }
}

pub struct WmPrefs {
    pub snap: bool,
    pub stiff: f32,
    pub damp: f32,
    pub depth: f32,
}

pub struct DockPrefs {
    pub size: f32,
    pub mag: f32,
    pub autohide: bool,
}

#[derive(Clone)]
pub struct ClipItem {
    pub text: String,
    pub files: usize,
    pub t: f64,
    pub pinned: bool,
    pub loc: Option<Location>,
}

pub struct Shell {
    pub z: Vec<AppId>,
    pub photos_w: AppWin,
    pub term_w: AppWin,
    pub capture_w: AppWin,
    pub settings_w: AppWin,
    pub studio_w: AppWin,
    pub term: crate::terminal::Terminal,
    pub photos: crate::photos::Photos,
    pub menu: Option<usize>,
    pub settings: Option<(AppId, usize)>,
    pub appmenu: Option<crate::appkit::AppMenu>,
    pub qs: bool,
    pub wifi: bool,
    pub bt: bool,
    pub focus: bool,
    pub brightness: f32,
    pub volume: f32,
    pub launched: HashMap<AppId, f64>,
    // ── smart desktop ──
    pub now: f64,
    pub screen: Rect,
    pub snd: crate::sound::Sound,
    pub tts_on: bool,
    pub tts_rate: f32,
    pub wm: WmPrefs,
    pub dock: DockPrefs,
    pub drag_win: Option<AppId>,
    pub snap_hint: Option<Rect>,
    pub trash_n: usize,
    pub trash_flash: f64,
    pub dock_prev: Option<(AppId, f64)>,
    pub desk: desktop::DesktopState,
    pub notes: Vec<Note>,
    pub note_id: u64,
    pub dialog: Option<Dialog>,
    pub clip: Vec<ClipItem>,
    pub clip_open: bool,
    pub sys_open: bool,
    pub clip_files_sig: usize,
    pub ai: ai::Ai,
    pub cap: crate::apps::capture::Capture,
    pub gen: Vec<ai::GenApp>,
    pub gen_sel: usize,
    pub set_page: usize,
    pub boot_t0: Option<f64>,
    pub video_bg: Option<String>,
    pub fps_hist: Vec<f32>,
    pub lat_hist: Vec<f32>,
    pub lat: f32,
    pub remote: bool,
    pub focus_prev: Option<AppId>,
    pub scroll_vol: f32,
}

impl Shell {
    pub fn new(home: Location) -> Self {
        Self {
            z: vec![AppId::Studio, AppId::Settings, AppId::Capture, AppId::Terminal, AppId::Photos, AppId::Files],
            photos_w: AppWin::new(980.0, 640.0, false),
            term_w: AppWin::new(880.0, 540.0, false),
            capture_w: AppWin::new(760.0, 520.0, false),
            settings_w: AppWin::new(940.0, 620.0, false),
            studio_w: AppWin::new(820.0, 560.0, false),
            term: crate::terminal::Terminal::new(home),
            photos: crate::photos::Photos::default(),
            menu: None,
            settings: None,
            appmenu: None,
            qs: false,
            wifi: true,
            bt: true,
            focus: false,
            brightness: 1.0,
            volume: 0.62,
            launched: HashMap::new(),
            now: 0.0,
            screen: Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0)),
            snd: crate::sound::Sound::new(),
            tts_on: false,
            tts_rate: 1.0,
            wm: WmPrefs { snap: true, stiff: 230.0, damp: 27.0, depth: 1.0 },
            dock: DockPrefs { size: 46.0, mag: 1.0, autohide: false },
            drag_win: None,
            snap_hint: None,
            trash_n: 0,
            trash_flash: -10.0,
            dock_prev: None,
            desk: desktop::DesktopState::default(),
            notes: vec![],
            note_id: 1,
            dialog: None,
            clip: vec![],
            clip_open: false,
            sys_open: false,
            clip_files_sig: 0,
            ai: ai::Ai::load(),
            cap: crate::apps::capture::Capture::default(),
            gen: vec![],
            gen_sel: 0,
            set_page: 0,
            boot_t0: None,
            video_bg: None,
            fps_hist: vec![],
            lat_hist: vec![],
            lat: 38.0,
            remote: true,
            focus_prev: None,
            scroll_vol: 0.0,
        }
    }
    pub fn active(&self) -> AppId {
        *self.z.last().unwrap_or(&AppId::Files)
    }
}

impl SmartApp {
    pub fn open_app(&mut self, id: AppId) {
        let t = self.time;
        let was_open = match id {
            AppId::Files => !self.win_closed && !self.win_min,
            other => self.app_win(other).map(|w| w.open && !w.min).unwrap_or(false),
        };
        match id {
            AppId::Files => {
                self.win_closed = false;
                self.win_min = false;
            }
            other => {
                let w = self.app_win_mut(other);
                if !w.open {
                    w.born = t;
                }
                w.open = true;
                w.min = false;
            }
        }
        if !was_open {
            self.shell.snd.play(Snd::Open);
        }
        self.shell.launched.insert(id, t);
        self.raise(id);
    }

    /// Topmost window that is actually on screen.
    pub fn focused_app(&self) -> AppId {
        for id in self.shell.z.iter().rev() {
            let vis = match id {
                AppId::Files => !(self.win_closed || self.win_min),
                other => self.app_win(*other).map(|w| w.open && !w.min).unwrap_or(false),
            };
            if vis {
                return *id;
            }
        }
        AppId::Files
    }

    pub fn is_open(&self, id: AppId) -> bool {
        match id {
            AppId::Files => !self.win_closed,
            other => self.app_win(other).map(|w| w.open).unwrap_or(false),
        }
    }

    pub fn is_visible(&self, id: AppId) -> bool {
        match id {
            AppId::Files => !(self.win_closed || self.win_min),
            other => self.app_win(other).map(|w| w.open && !w.min).unwrap_or(false),
        }
    }

    /// Window the palette should centre itself in.
    pub fn palette_win(&self) -> Rect {
        match self.palette.app {
            Some(a) if a != AppId::Files => self.app_win(a).map(|w| w.rect).unwrap_or_else(crate::overlays::win_rect),
            _ => crate::overlays::win_rect(),
        }
    }

    pub fn raise(&mut self, id: AppId) {
        self.shell.z.retain(|x| *x != id);
        self.shell.z.push(id);
    }

    pub fn app_cmd(&mut self, cmd: &str) {
        if self.shell_cmd(cmd) {
            return;
        }
        if cmd == "settings" {
            let a = self.focused_app();
            if a != AppId::Files && !matches!(a, AppId::Settings | AppId::Capture | AppId::Studio) {
                self.shell.settings = Some((a, 0));
            } else {
                self.open_app(AppId::Settings);
            }
            return;
        }
        if let Some(c) = cmd.strip_prefix("term.") {
            self.term_cmd(c);
        } else if let Some(c) = cmd.strip_prefix("photos.") {
            self.photos_cmd(c);
        }
    }

    pub fn app_win(&self, id: AppId) -> Option<&AppWin> {
        match id {
            AppId::Files => None,
            AppId::Photos => Some(&self.shell.photos_w),
            AppId::Terminal => Some(&self.shell.term_w),
            AppId::Capture => Some(&self.shell.capture_w),
            AppId::Settings => Some(&self.shell.settings_w),
            AppId::Studio => Some(&self.shell.studio_w),
        }
    }
    pub fn app_win_mut(&mut self, id: AppId) -> &mut AppWin {
        match id {
            AppId::Terminal => &mut self.shell.term_w,
            AppId::Capture => &mut self.shell.capture_w,
            AppId::Settings => &mut self.shell.settings_w,
            AppId::Studio => &mut self.shell.studio_w,
            _ => &mut self.shell.photos_w,
        }
    }

    /// Current on-screen rectangle of a visible window (Files included).
    pub fn win_rect_of(&self, id: AppId, screen: Rect) -> Option<Rect> {
        if !self.is_visible(id) {
            return None;
        }
        Some(match id {
            AppId::Files => self.window_rect(screen),
            other => self.app_win(other)?.rect,
        })
    }

    pub fn over_window(&self, pos: Pos2) -> bool {
        let screen = self.shell.screen;
        AppId::ALL.iter().any(|id| self.win_rect_of(*id, screen).is_some_and(|r| r.contains(pos)))
    }

    // ───────────── entry point ─────────────
    pub fn draw_window(&mut self, ui: &mut Ui) {
        if !IS_WEB {
            clear_drop_targets();
            return self.draw_files_window(ui);
        }
        clear_drop_targets();
        let screen = ui.max_rect();
        let ctx = ui.ctx().clone();
        let p = ui.painter().clone();
        self.shell_frame_begin(&ctx, screen);
        if self.shell.video_bg.is_none() {
            self.draw_desktop(&p, screen);
        }
        let ai_t = spring_to(&ctx, "aimode", if self.shell.ai.mode { 1.0 } else { 0.0 }, 130.0, 22.0).clamp(0.0, 1.0);
        self.draw_desktop_layer(ui, screen, ai_t);
        let cap_sel = self.shell.cap.selecting;
        if ai_t < 0.999 && !cap_sel {
            self.shell_focus(ui, screen);
            for id in self.shell.z.clone() {
                match id {
                    AppId::Files => {
                        if !(self.win_closed || self.win_min) {
                            self.draw_files_window(ui);
                            let r = self.window_rect(screen);
                        }
                    }
                    _ => {
                        self.draw_app(ui, id, screen);
                        if let Some(r) = self.win_rect_of(id, screen) {
                        }
                    }
                }
            }
            self.draw_snap_preview(ui);
            self.draw_snap_divider(ui, screen);
            if ai_t > 0.01 && self.shell.video_bg.is_none() {
                let (tex, sz, _) = self.active_wall();
                ui.painter().image(tex, screen, cover_uv(sz[0], sz[1], screen), Color32::from_white_alpha((ai_t * 255.0) as u8));
            }
            self.draw_app_menu(ui, screen);
            self.draw_dock(ui, screen, ai_t);
        }
        self.draw_topbar(ui, screen, ai_t);
        self.draw_clip_popover(ui, screen);
        self.draw_sys_popover(ui, screen);
        self.draw_notes(ui, screen);
        self.draw_assistant(ui, screen, ai_t);
        self.draw_wallpaper_dialog(ui, screen);
        self.draw_picker(ui, screen);
        self.draw_dialog(ui, screen);
        let b = self.shell.brightness;
        if b < 0.99 {
            let lp = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, Id::new("dim")));
            lp.rect_filled(screen, 0.0, Color32::from_black_alpha(((1.0 - b) * 150.0) as u8));
        }
        self.draw_boot(ui, screen);
    }

    /// Pressing inside a window raises it; the top bar and dock never steal focus.
    pub(crate) fn shell_focus(&mut self, ui: &Ui, screen: Rect) {
        let Some(pos) = ui.input(|i| if i.pointer.primary_pressed() { i.pointer.interact_pos() } else { None }) else { return };
        if pos.y < TOPBAR_H || pos.y > screen.bottom() - 70.0 {
            return;
        }
        for id in self.shell.z.clone().into_iter().rev() {
            let r = match id {
                AppId::Files => {
                    if self.win_closed || self.win_min {
                        continue;
                    }
                    self.window_rect(screen)
                }
                other => {
                    let w = self.app_win(other).unwrap();
                    if !w.open || w.min {
                        continue;
                    }
                    w.rect
                }
            };
            if r.contains(pos) {
                if self.focused_app() != id {
                    self.raise(id);
                }
                break;
            }
        }
    }

}
