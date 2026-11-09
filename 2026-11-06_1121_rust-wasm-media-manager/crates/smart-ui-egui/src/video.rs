//! **Eiga (映画)** — the suite's video player. Engine: `eiga-core` (state machine, subtitles, key map, ffmpeg pipes).
//! This file is only the surface: it draws frames, the controls, subtitles, menus and routes keys.
use crate::app::*;
use crate::appkit::Frame;
use crate::chrome::*;
use crate::icons::Ic;
use crate::overlays::{mi, MI};
use crate::paint::*;
use crate::theme::*;
use eiga_core::player::{default_bindings, lookup, Binding, Cmd, Event as PEvent, LoopMode, Repeat};
use eiga_core::source::{FrameSource, TestCard};
use eiga_core::{fmt_time, parse_time, MediaInfo, Player, Playlist, SubTrack};
use egui::{pos2, vec2, Align2, Color32, Key, Rect, Sense, Ui};
use smart_core::*;

const VIDEO_EXTS: [&str; 8] = ["mp4", "mkv", "mov", "webm", "avi", "m4v", "mpg", "ts"];

#[cfg(not(target_arch = "wasm32"))]
pub struct AudioOut {
    dev: rodio::MixerDeviceSink,
    player: rodio::Player,
    pipe: Option<eiga_core::source::ffmpeg::AudioPipe>,
}

#[cfg(not(target_arch = "wasm32"))]
impl AudioOut {
    fn new() -> Result<Self, String> {
        let mut dev = rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| format!("No audio output: {e}"))?;
        dev.log_on_drop(false);
        let player = rodio::Player::connect_new(dev.mixer());
        Ok(AudioOut { dev, player, pipe: None })
    }
    fn restart(&mut self, path: &str, start: f64, track: usize, speed: f64, gain: f32, paused: bool) {
        self.player.stop();
        self.player = rodio::Player::connect_new(self.dev.mixer());
        self.player.set_volume(gain);
        if paused {
            self.player.pause();
        }
        self.pipe = eiga_core::source::ffmpeg::AudioPipe::open(path, start, track, speed).ok();
    }
    fn pump(&mut self) {
        let (Some(ch), Some(sr)) = (std::num::NonZeroU16::new(2), std::num::NonZeroU32::new(48_000)) else { return };
        if let Some(p) = &self.pipe {
            while self.player.len() < 3 {
                match p.try_chunk() {
                    Some(c) => self.player.append(rodio::buffer::SamplesBuffer::new(ch, sr, c)),
                    None => break,
                }
            }
        }
    }
}

pub struct Eiga {
    pub player: Player,
    pub playlist: Playlist,
    pub src: Option<Box<dyn FrameSource>>,
    pub info: Option<MediaInfo>,
    pub tex: Option<egui::TextureHandle>,
    pub frame_wh: (usize, usize),
    pub subs: Vec<SubTrack>,
    pub sub_i: Option<usize>,
    pub sub_scale: f32,
    pub audio_i: usize,
    pub title: String,
    pub path: Option<String>,
    pub real: Option<String>,
    pub bindings: Vec<Binding>,
    pub last_move: f64,
    pub last_tick: f64,
    pub theater: bool,
    pub show_list: bool,
    pub show_stats: bool,
    pub show_help: bool,
    pub aspect: u8,
    pub zoom: f32,
    pub rotate: u8,
    pub osd: Option<(String, f64)>,
    pub error: Option<String>,
    pub jump: Option<String>,
    pub scrub: Option<f64>,
    pub frames_shown: u64,
    pub fps_meter: (f64, u64, f32),
    pub locs: Vec<Location>,
    pub pending_evs: Vec<PEvent>,
    pub pending_cmd: Option<Cmd>,
    pub last_rgba: Option<(usize, usize, Vec<u8>)>,
    pub fs_dirty: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pub audio: Option<AudioOut>,
}

impl Default for Eiga {
    fn default() -> Self {
        Eiga { player: Player::default(), playlist: Playlist::default(), src: None, info: None, tex: None, frame_wh: (0, 0), subs: vec![], sub_i: None, sub_scale: 1.0, audio_i: 0, title: String::new(), path: None, real: None, bindings: default_bindings(), last_move: 0.0, last_tick: 0.0, theater: false, show_list: true, show_stats: false, show_help: false, aspect: 0, zoom: 1.0, rotate: 0, osd: None, error: None, jump: None, scrub: None, frames_shown: 0, fps_meter: (0.0, 0, 0.0), locs: vec![], pending_evs: vec![], pending_cmd: None, last_rgba: None, fs_dirty: false, #[cfg(not(target_arch = "wasm32"))] audio: None }
    }
}

fn key_name(k: Key, shift: bool) -> Option<&'static str> {
    Some(match k {
        Key::Space => "space",
        Key::ArrowLeft => "left",
        Key::ArrowRight => "right",
        Key::ArrowUp => "up",
        Key::ArrowDown => "down",
        Key::Home => "home",
        Key::PageUp => "pageup",
        Key::PageDown => "pagedown",
        Key::Enter => "enter",
        Key::Escape => "escape",
        Key::Backspace => "backspace",
        Key::Tab => "tab",
        Key::Comma => if shift { "comma" } else { "," },
        Key::Period => if shift { "period" } else { "." },
        Key::OpenBracket => "[",
        Key::CloseBracket => "]",
        Key::OpenCurlyBracket => "{",
        Key::CloseCurlyBracket => "}",
        Key::Minus => "-",
        Key::Equals | Key::Plus => "=",
        Key::Slash => "/",
        Key::Questionmark => "?",
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::A => "a",
        Key::B => "b",
        Key::C => "c",
        Key::D => "d",
        Key::E => "e",
        Key::F => "f",
        Key::G => "g",
        Key::H => "h",
        Key::I => "i",
        Key::J => "j",
        Key::K => "k",
        Key::L => "l",
        Key::M => "m",
        Key::N => "n",
        Key::O => "o",
        Key::P => "p",
        Key::Q => "q",
        Key::R => "r",
        Key::S => "s",
        Key::T => "t",
        Key::U => "u",
        Key::V => "v",
        Key::W => "w",
        Key::X => "x",
        Key::Y => "y",
        Key::Z => "z",
        _ => return None,
    })
}

impl SmartApp {
    // ───────────── opening ─────────────
    pub fn eiga_cmd(&mut self, c: &str) {
        let (name, arg) = c.split_once(':').unwrap_or((c, ""));
        match name {
            "open" => {
                if let Some((fs, path)) = arg.split_once('|') {
                    self.eiga_open(Location::new(fs, path));
                }
            }
            "play_idx" => {
                if let Ok(i) = arg.parse::<usize>() {
                    self.eiga_play_index(i);
                }
            }
            "sub" => {
                let e = &mut self.shell.eiga;
                e.sub_i = if arg == "off" { None } else { arg.parse().ok().filter(|i| *i < e.subs.len()) };
                let msg = match e.sub_i.and_then(|i| e.subs.get(i)) {
                    Some(t) => format!("Subtitles: {}", t.name),
                    None => "Subtitles off".to_string(),
                };
                e.osd = Some((msg, self.time));
            }
            "audio" => {
                if let Ok(i) = arg.parse::<usize>() {
                    self.shell.eiga.audio_i = i;
                    self.eiga_restart_audio();
                    self.shell.eiga.osd = Some((format!("Audio track {}", i + 1), self.time));
                }
            }
            "speed" => {
                if let Ok(s) = arg.parse::<f64>() {
                    let ev = self.shell.eiga.player.set_speed(s);
                    self.eiga_events(ev);
                }
            }
            "cmd" => {
                let b = default_bindings();
                if let Some(x) = b.iter().find(|x| x.help == arg) {
                    self.eiga_run(x.cmd);
                }
            }
            "aspect" => {
                self.shell.eiga.aspect = arg.parse().unwrap_or(0);
            }
            "list" => self.shell.eiga.show_list = !self.shell.eiga.show_list,
            _ => {}
        }
    }

    fn eiga_siblings(&self, l: &Location) -> Vec<Location> {
        let Some(parent) = l.parent() else { return vec![l.clone()] };
        let Some(v) = self.be.registry.get(&parent.fs) else { return vec![l.clone()] };
        let mut out: Vec<Location> = v.list(&parent.path).unwrap_or_default().into_iter().filter(|e| !e.is_dir && (e.kind == Kind::Video || VIDEO_EXTS.contains(&e.ext.to_ascii_lowercase().as_str()))).map(|e| e.loc).collect();
        out.sort_by_key(|x| x.name().to_lowercase());
        if out.is_empty() {
            out.push(l.clone());
        }
        out
    }

    pub fn eiga_open(&mut self, l: Location) {
        self.open_app(crate::shell::AppId::Video);
        let locs = self.eiga_siblings(&l);
        let start = locs.iter().position(|x| *x == l).unwrap_or(0);
        self.shell.eiga.playlist = Playlist::new(locs.iter().map(|x| x.name()).collect(), start);
        self.shell.eiga.locs = locs;
        self.eiga_load(l);
    }

    fn eiga_play_index(&mut self, i: usize) {
        if let Some(l) = self.shell.eiga.locs.get(i).cloned() {
            self.shell.eiga.playlist.cur = Some(i);
            self.eiga_load(l);
        }
    }

    fn eiga_load(&mut self, l: Location) {
        let now = self.time;
        let vfs = self.be.registry.get(&l.fs);
        let real = vfs.as_ref().and_then(|v| v.real_path(&l.path)).map(|p| p.to_string_lossy().to_string());
        let entry = self.be.stat(&l).ok();
        let _ = self.be.library.write().map(|mut x| x.touch_recent(&l));
        let mut src: Option<Box<dyn FrameSource>> = None;
        let mut info: Option<MediaInfo> = None;
        let mut error = None;
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(rp) = &real {
            if eiga_core::source::ffmpeg::available() {
                match eiga_core::source::ffmpeg::probe(rp) {
                    Ok(i) => match eiga_core::source::ffmpeg::FfmpegSource::open(rp, i.clone(), 1280, 720) {
                        Ok(s) => {
                            src = Some(Box::new(s));
                            info = Some(i);
                        }
                        Err(e) => error = Some(e),
                    },
                    Err(e) => error = Some(e),
                }
            } else {
                error = Some("ffmpeg was not found. Install it (apt install ffmpeg) to play real files. Showing the test card.".to_string());
            }
        }
        let name = l.name();
        let (src, info) = match (src, info) {
            (Some(s), Some(i)) => (s, i),
            _ => {
                let dur = entry.as_ref().map(|e| (e.size as f64 / 4_000_000.0).clamp(40.0, 600.0)).unwrap_or(120.0);
                if error.is_none() {
                    error = Some("Web preview: decoding runs natively (ffmpeg). This is the built-in test card with real subtitles.".into());
                }
                let s = TestCard::new(&name, dur);
                let i = s.info().clone();
                (Box::new(s) as Box<dyn FrameSource>, i)
            }
        };
        // subtitles: sidecar files first, then embedded text tracks (extracted lazily)
        let mut subs: Vec<SubTrack> = vec![];
        if let (Some(parent), Some(v)) = (l.parent(), vfs.as_ref()) {
            let names: Vec<String> = v.list(&parent.path).unwrap_or_default().into_iter().map(|e| e.name).collect();
            for n in eiga_core::subs::find_sidecars(&name, &names) {
                if let Ok(bytes) = v.read(&format!("{}/{}", parent.path.trim_end_matches('/'), n), 8 * 1024 * 1024) {
                    let t = SubTrack::from_bytes(&n, &bytes);
                    if !t.is_empty() {
                        subs.push(t);
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(rp) = &real {
            for (n, s) in info.subs.iter().enumerate().filter(|(_, s)| s.is_text()) {
                if let Ok(txt) = eiga_core::source::ffmpeg::extract_sub(rp, n) {
                    let mut t = SubTrack::new(format!("{} (embedded){}", if s.lang.is_empty() { "Track".to_string() } else { s.lang.clone() }, if s.forced { " forced" } else { "" }), s.lang.clone(), eiga_core::subs::parse(&txt));
                    if !t.is_empty() {
                        t.delay = 0.0;
                        subs.push(t);
                    }
                }
            }
        }
        let e = &mut self.shell.eiga;
        e.player = Player::new(info.duration, info.fps());
        e.player.chapters = info.chapters.iter().map(|c| c.0).collect();
        e.sub_i = if subs.is_empty() { None } else { Some(0) };
        e.subs = subs;
        e.title = if info.title.is_empty() { name } else { info.title.clone() };
        e.path = Some(l.path.clone());
        e.real = real.clone();
        e.info = Some(info);
        e.src = Some(src);
        e.error = error;
        e.audio_i = 0;
        e.last_tick = now;
        e.last_move = now;
        e.frames_shown = 0;
        e.osd = Some((format!("Playing {}", e.title), now));
        self.eiga_restart_audio();
    }

    fn eiga_restart_audio(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let e = &mut self.shell.eiga;
            let Some(rp) = e.real.clone() else { return };
            if e.info.as_ref().map(|i| i.audio.is_empty()).unwrap_or(true) {
                e.audio = None;
                return;
            }
            if e.audio.is_none() {
                e.audio = AudioOut::new().ok();
            }
            let (pos, tr, sp, g, paused) = (e.player.position, e.audio_i, e.player.speed, e.player.gain(), e.player.paused);
            if let Some(a) = e.audio.as_mut() {
                a.restart(&rp, pos, tr, sp, g, paused);
            }
        }
    }

    fn eiga_events(&mut self, evs: Vec<PEvent>) {
        for ev in evs {
            match ev {
                PEvent::Seek(t) => {
                    if let Some(s) = self.shell.eiga.src.as_mut() {
                        s.seek(t);
                    }
                    self.eiga_restart_audio();
                }
                PEvent::PauseChanged(p) => {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(a) = self.shell.eiga.audio.as_mut() {
                        if p {
                            a.player.pause()
                        } else {
                            a.player.play()
                        }
                    }
                    let _ = p;
                }
                PEvent::SpeedChanged(_) => self.eiga_restart_audio(),
                PEvent::VolumeChanged => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let g = self.shell.eiga.player.gain();
                        if let Some(a) = self.shell.eiga.audio.as_mut() {
                            a.player.set_volume(g);
                        }
                    }
                }
                PEvent::Ended => {
                    let next = self.shell.eiga.playlist.next(true);
                    if let Some(i) = next {
                        self.eiga_play_index(i);
                    }
                }
                PEvent::Osd => {}
            }
        }
        let t = self.time;
        if let Some(m) = self.shell.eiga.player.osd.take() {
            self.shell.eiga.osd = Some((m, t));
        }
    }

    fn eiga_run(&mut self, cmd: Cmd) {
        let (evs, rest) = self.shell.eiga.player.apply(cmd);
        self.eiga_events(evs);
        let Some(cmd) = rest else { return };
        let t = self.time;
        match cmd {
            Cmd::ToggleFullscreen => {
                let e = &mut self.shell.eiga;
                e.theater = !e.theater;
                e.fs_dirty = true;
            }
            Cmd::Close => {
                if self.shell.eiga.theater {
                    self.eiga_run(Cmd::ToggleFullscreen);
                } else {
                    self.shell.video_w.open = false;
                }
            }
            Cmd::ToggleStats => self.shell.eiga.show_stats = !self.shell.eiga.show_stats,
            Cmd::ToggleHelp => self.shell.eiga.show_help = !self.shell.eiga.show_help,
            Cmd::TogglePlaylist => self.shell.eiga.show_list = !self.shell.eiga.show_list,
            Cmd::ToggleHud => self.shell.eiga.last_move = if self.shell.eiga.last_move > t + 1000.0 { t } else { t + 100000.0 },
            Cmd::JumpTo => self.shell.eiga.jump = Some(String::new()),
            Cmd::CycleAspect => {
                let e = &mut self.shell.eiga;
                e.aspect = (e.aspect + 1) % 4;
                e.osd = Some((format!("Aspect: {}", ["fit", "fill", "16:9", "4:3"][e.aspect as usize]), t));
            }
            Cmd::Zoom(d) => {
                let e = &mut self.shell.eiga;
                e.zoom = (e.zoom + d).clamp(0.5, 3.0);
                e.osd = Some((format!("Zoom {:.0}%", e.zoom * 100.0), t));
            }
            Cmd::Rotate => {
                let e = &mut self.shell.eiga;
                e.rotate = (e.rotate + 1) % 4;
                e.osd = Some((format!("Rotated {}\u{b0}", e.rotate as u32 * 90), t));
            }
            Cmd::CycleSub => {
                let e = &mut self.shell.eiga;
                e.sub_i = match e.sub_i {
                    None if !e.subs.is_empty() => Some(0),
                    Some(i) if i + 1 < e.subs.len() => Some(i + 1),
                    _ => None,
                };
                let m = match e.sub_i.and_then(|i| e.subs.get(i)) {
                    Some(s) => format!("Subtitles: {}", s.name),
                    None => if e.subs.is_empty() { "No subtitles found for this video".to_string() } else { "Subtitles off".to_string() },
                };
                e.osd = Some((m, t));
            }
            Cmd::CycleAudio => {
                let n = self.shell.eiga.info.as_ref().map(|i| i.audio.len()).unwrap_or(0);
                if n > 1 {
                    self.shell.eiga.audio_i = (self.shell.eiga.audio_i + 1) % n;
                    self.eiga_restart_audio();
                    self.shell.eiga.osd = Some((format!("Audio track {} of {n}", self.shell.eiga.audio_i + 1), t));
                } else {
                    self.shell.eiga.osd = Some(("Only one audio track".into(), t));
                }
            }
            Cmd::SubDelay(d) => {
                let e = &mut self.shell.eiga;
                if let Some(tr) = e.sub_i.and_then(|i| e.subs.get_mut(i)) {
                    let m = eiga_core::player::nudge_sub_delay(tr, d);
                    e.osd = Some((m, t));
                }
            }
            Cmd::SubScale(d) => {
                let e = &mut self.shell.eiga;
                e.sub_scale = (e.sub_scale + d).clamp(0.5, 2.5);
                e.osd = Some((format!("Subtitle size {:.0}%", e.sub_scale * 100.0), t));
            }
            Cmd::NextSubCue | Cmd::PrevSubCue => {
                let (pos, next) = (self.shell.eiga.player.position, cmd == Cmd::NextSubCue);
                let target = {
                    let e = &self.shell.eiga;
                    e.sub_i.and_then(|i| e.subs.get(i)).and_then(|s| if next { s.next_start(pos) } else { s.prev_start(pos) })
                };
                if let Some(t) = target {
                    let ev = self.shell.eiga.player.seek_abs(t);
                    self.eiga_events(ev);
                }
            }
            Cmd::NextItem | Cmd::PrevItem => {
                let i = if cmd == Cmd::NextItem { self.shell.eiga.playlist.next(false) } else { self.shell.eiga.playlist.prev() };
                if let Some(i) = i {
                    self.eiga_play_index(i);
                }
            }
            Cmd::ToggleShuffle => {
                let pl = &mut self.shell.eiga.playlist;
                let on = !pl.shuffle;
                pl.set_shuffle(on);
                self.shell.eiga.osd = Some((format!("Shuffle {}", if on { "on" } else { "off" }), t));
            }
            Cmd::CycleRepeat => {
                self.shell.eiga.playlist.cycle_repeat();
                let r = self.shell.eiga.playlist.repeat;
                self.shell.eiga.osd = Some((format!("Repeat: {}", match r { Repeat::Off => "off", Repeat::All => "all", Repeat::One => "one" }), t));
            }
            Cmd::OpenFile => self.acts.push(Act::OpenApp(crate::shell::AppId::Files)),
            Cmd::Screenshot => self.eiga_screenshot(),
            _ => {}
        }
    }

    fn eiga_screenshot(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let t = self.time;
            let e = &self.shell.eiga;
            let msg = match (&e.src, std::env::var("HOME")) {
                (Some(_), Ok(home)) if e.frames_shown > 0 => {
                    let dir = std::path::PathBuf::from(home).join("Pictures");
                    let _ = std::fs::create_dir_all(&dir);
                    let f = dir.join(format!("Eiga {} {}.png", e.title.replace('/', "-"), fmt_time(e.player.position).replace(':', "-")));
                    match e.last_rgba.as_ref() {
                        Some((w, h, px)) => match image::save_buffer(&f, px, *w as u32, *h as u32, image::ColorType::Rgba8) {
                            Ok(()) => format!("Saved {}", f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()),
                            Err(er) => format!("Screenshot failed: {er}"),
                        },
                        None => "Nothing to capture yet".to_string(),
                    }
                }
                _ => "Nothing to capture yet".to_string(),
            };
            self.shell.eiga.osd = Some((msg, t));
        }
        #[cfg(target_arch = "wasm32")]
        {
            let t = self.time;
            self.shell.eiga.osd = Some(("Screenshots are saved by the desktop app".into(), t));
        }
    }

    fn eiga_keys(&mut self, ctx: &egui::Context) {
        if self.shell.eiga.jump.is_some() || ctx.egui_wants_keyboard_input() {
            return;
        }
        let evs: Vec<egui::Event> = ctx.input(|i| i.events.clone());
        for ev in evs {
            if let egui::Event::Key { key, pressed: true, modifiers, .. } = ev {
                if modifiers.command && !modifiers.ctrl {
                    continue;
                }
                let Some(n) = key_name(key, modifiers.shift) else { continue };
                let cmd = lookup(&self.shell.eiga.bindings, n, modifiers.ctrl || modifiers.command, modifiers.shift, modifiers.alt);
                if let Some(c) = cmd {
                    self.shell.eiga.last_move = self.time;
                    self.eiga_run(c);
                }
            }
        }
    }

    // ───────────── menus (dropdowns) ─────────────
    fn eiga_speed_menu(&self) -> Vec<MI> {
        let sp = self.shell.eiga.player.speed;
        let mut v: Vec<MI> = eiga_core::player::SPEEDS.iter().map(|s| mi("bolt", format!("\u{d7}{s}"), Act::App(format!("eiga.speed:{s}"))).on((sp - s).abs() < 1e-6)).collect();
        v.push(mi("restart", "Reset to \u{d7}1", Act::App("eiga.speed:1".into())).k("Backspace").gap());
        v
    }
    fn eiga_sub_menu(&self) -> Vec<MI> {
        let e = &self.shell.eiga;
        let mut v = vec![mi("close-circle", "Off", Act::App("eiga.sub:off".into())).on(e.sub_i.is_none())];
        for (i, s) in e.subs.iter().enumerate() {
            v.push(mi("text", format!("{}{}", s.name, if s.lang.is_empty() { String::new() } else { format!("  \u{b7}  {}", s.lang) }), Act::App(format!("eiga.sub:{i}"))).on(e.sub_i == Some(i)));
        }
        if e.subs.is_empty() {
            v.push(mi("info-circle", "No subtitles found next to this video", Act::Toast(String::new())).off(true));
        }
        v.push(mi("sort-vertical", "Earlier 100 ms", Act::App("eiga.cmd:Subtitles earlier 100 ms".into())).k("Shift Z").gap());
        v.push(mi("sort-vertical", "Later 100 ms", Act::App("eiga.cmd:Subtitles later 100 ms".into())).k("Shift X"));
        v.push(mi("text", "Larger", Act::App("eiga.cmd:Larger subtitles".into())).k("T"));
        v.push(mi("text", "Smaller", Act::App("eiga.cmd:Smaller subtitles".into())).k("R"));
        v
    }
    fn eiga_audio_menu(&self) -> Vec<MI> {
        let e = &self.shell.eiga;
        let mut v = vec![];
        if let Some(i) = &e.info {
            for (n, a) in i.audio.iter().enumerate() {
                let label = format!("{}{} \u{b7} {} \u{b7} {} ch", if a.lang.is_empty() { "Track ".to_string() + &(n + 1).to_string() } else { a.lang.clone() }, if a.title.is_empty() { String::new() } else { format!(" ({})", a.title) }, a.codec, a.channels);
                v.push(mi("volume-loud", label, Act::App(format!("eiga.audio:{n}"))).on(e.audio_i == n));
            }
        }
        if v.is_empty() {
            v.push(mi("volume-cross", "No audio track", Act::Toast(String::new())).off(true));
        }
        v.push(mi("sort-vertical", "Audio earlier 100 ms", Act::App("eiga.cmd:Audio earlier 100 ms".into())).k("Z").gap());
        v.push(mi("sort-vertical", "Audio later 100 ms", Act::App("eiga.cmd:Audio later 100 ms".into())).k("X"));
        v
    }
    fn eiga_more_menu(&self) -> Vec<MI> {
        let e = &self.shell.eiga;
        let a = |s: &str| Act::App(format!("eiga.cmd:{s}"));
        vec![
            mi("maximize", "Fullscreen", a("Fullscreen")).k("F"),
            mi("full-screen", "Aspect ratio", Act::Toast(String::new())).sub(["Fit", "Fill", "16:9", "4:3"].iter().enumerate().map(|(i, n)| mi("widget-4", *n, Act::App(format!("eiga.aspect:{i}"))).on(e.aspect as usize == i)).collect()),
            mi("repeat", "Repeat file", a("Repeat this file")).k("Alt L").on(e.player.looping == LoopMode::File),
            mi("repeat", "Loop A \u{2192} B", a("Set A \u{2192} B loop (press again to clear)")).k("Shift L").on(matches!(e.player.looping, LoopMode::Ab { .. })),
            mi("shuffle", "Shuffle", a("Shuffle")).k("Ctrl S").on(e.playlist.shuffle).gap(),
            mi("playlist-minimalistic", "Playlist", a("Show / hide playlist")).k("P").on(e.show_list),
            mi("pie-chart-2", "Statistics", a("Statistics overlay")).k("I").on(e.show_stats),
            mi("camera-minimalistic", "Screenshot", a("Save screenshot")).k("S").gap(),
            mi("keyboard", "Keyboard shortcuts", a("Shortcut sheet")).k("H"),
        ]
    }

    // ───────────── drawing ─────────────
    pub fn draw_eiga(&mut self, ui: &mut Ui, fr: &Frame) {
        let th = self.th.clone();
        let ctx = ui.ctx().clone();
        let mut e = std::mem::take(&mut self.shell.eiga);
        if e.src.is_none() && e.path.is_none() {
            // nothing loaded yet: open the first demo/real video we can find so the window is never empty
            self.shell.eiga = e;
            let start = self.eiga_default_video();
            if let Some(l) = start {
                self.eiga_load_first(l);
            }
            e = std::mem::take(&mut self.shell.eiga);
        }
        // clock
        let now = self.time;
        let dt = (now - e.last_tick).clamp(0.0, 0.25);
        e.last_tick = now;
        self.shell.eiga = e;
        let evs = self.shell.eiga.player.tick(dt);
        self.eiga_events(evs);
        if fr.active && self.shell.settings.is_none() && !self.palette.open && self.shell.appmenu.is_none() {
            self.eiga_keys(&ctx);
        }
        let mut e = std::mem::take(&mut self.shell.eiga);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(a) = e.audio.as_mut() {
            a.pump();
        }
        // pull a frame
        let clock = e.player.position;
        if let Some(src) = e.src.as_mut() {
            if let Some(f) = src.poll(clock) {
                let img = egui::ColorImage::from_rgba_unmultiplied([f.w, f.h], &f.rgba);
                match e.tex.as_mut() {
                    Some(t) => t.set(img, egui::TextureOptions::LINEAR),
                    None => e.tex = Some(ctx.load_texture("eiga-frame", img, egui::TextureOptions::LINEAR)),
                }
                e.frame_wh = (f.w, f.h);
                e.frames_shown += 1;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    e.last_rgba = Some((f.w, f.h, f.rgba));
                }
            }
        }
        if now - e.fps_meter.0 >= 1.0 {
            e.fps_meter = (now, e.frames_shown, (e.frames_shown - e.fps_meter.1) as f32 / (now - e.fps_meter.0).max(0.001) as f32);
        }
        if !e.player.paused {
            ctx.request_repaint();
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(120));
        }

        if e.fs_dirty {
            e.fs_dirty = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(e.theater));
        }
        let p = ui.painter().clone();
        let theater = e.theater;
        let stage = if theater { fr.win } else { fr.stage };
        // toolbar (hidden in theater mode)
        if !theater {
            self.eiga_toolbar(ui, fr, &e);
            self.eiga_sidebar(ui, fr, &e);
        }
        let area = if theater { stage } else { Rect::from_min_max(stage.min + vec2(8.0, 2.0), stage.max - vec2(10.0, 10.0)) };
        let radius = if theater { 0.0 } else { 16.0 };
        // black stage
        grad_fill(&p, area, radius, c(4, 6, 10, 1.0), c(8, 10, 16, 1.0));
        let bg = ui.interact(area, egui::Id::new("eiga-surface"), Sense::click_and_drag());
        if ui.input(|i| i.pointer.delta().length() > 0.1) && bg.hovered() {
            e.last_move = now;
        }
        let hud = e.player.paused || now - e.last_move < 2.6 || e.last_move > now + 1000.0 && false || ui.rect_contains_pointer(Rect::from_min_max(pos2(area.left(), area.bottom() - 120.0), area.max));
        // video
        let vp = p.with_clip_rect(area);
        if let Some(tex) = &e.tex {
            let (fw, fh) = (e.frame_wh.0 as f32, e.frame_wh.1 as f32);
            let rot = e.rotate % 2 == 1;
            let src_ar = if rot { fh / fw } else { fw / fh };
            let ar = match e.aspect {
                2 => 16.0 / 9.0,
                3 => 4.0 / 3.0,
                _ => src_ar,
            };
            let (aw, ah) = (area.width(), area.height());
            let (mut w, mut h) = if e.aspect == 1 { if aw / ah > ar { (aw, aw / ar) } else { (ah * ar, ah) } } else if aw / ah > ar { (ah * ar, ah) } else { (aw, aw / ar) };
            w *= e.zoom;
            h *= e.zoom;
            let dest = Rect::from_center_size(area.center(), vec2(w, h));
            let uvs = [pos2(0.0, 0.0), pos2(1.0, 0.0), pos2(1.0, 1.0), pos2(0.0, 1.0)];
            let k = e.rotate as usize % 4;
            let corners = [dest.left_top(), dest.right_top(), dest.right_bottom(), dest.left_bottom()];
            let mut m = egui::epaint::Mesh::with_texture(tex.id());
            for (i, cpos) in corners.iter().enumerate() {
                m.vertices.push(egui::epaint::Vertex { pos: *cpos, uv: uvs[(i + k) % 4], color: Color32::WHITE });
            }
            m.indices.extend([0, 1, 2, 0, 2, 3]);
            vp.add(egui::Shape::mesh(m));
        }
        // subtitles
        if let Some(track) = e.sub_i.and_then(|i| e.subs.get(i)) {
            let cues = track.active(e.player.position);
            let size = (area.height() / 540.0 * 24.0 * e.sub_scale).clamp(12.0, 64.0);
            let mut y = area.bottom() - if hud { 118.0 } else { 38.0 };
            for cue in cues.iter().rev() {
                for line in cue.text.lines().rev() {
                    let pos = pos2(area.center().x, if cue.top { area.top() + 40.0 } else { y });
                    let font = egui::FontId::proportional(size);
                    let w = self.tw(&p, line, &font);
                    let pill = Rect::from_center_size(pos, vec2(w + 22.0, size + 10.0));
                    vp.rect_filled(pill, cr(8.0), c(0, 0, 0, 0.55));
                    vp.text(pos, Align2::CENTER_CENTER, line, font, if cue.italic { hex("#ffe9a8") } else { Color32::WHITE });
                    y -= size + 12.0;
                }
            }
        }
        // OSD
        if let Some((m, at)) = e.osd.clone() {
            let age = (now - at) as f32;
            if age < 1.8 {
                let a = (1.0 - ((age - 1.2) / 0.6).max(0.0)).clamp(0.0, 1.0);
                let w = self.tw(&p, &m, &f_semi(T_BODY)) + 28.0;
                let r = Rect::from_min_size(area.min + vec2(18.0, 18.0 - (1.0 - spring_out(age)) * 8.0), vec2(w, 34.0));
                grad_fill(&vp, r, 17.0, c(10, 14, 24, 0.72 * a), c(10, 14, 24, 0.62 * a));
                vp.text(r.center(), Align2::CENTER_CENTER, &m, f_semi(T_BODY), alpha(Color32::WHITE, a));
                ctx.request_repaint();
            }
        }
        if let Some(err) = e.error.clone() {
            if e.frames_shown < 400 {
                let t = self.fit(&p, &err, &f_medium(T_META), area.width() - 60.0);
                let r = Rect::from_center_size(pos2(area.center().x, area.top() + 38.0), vec2(self.tw(&p, &t, &f_medium(T_META)) + 28.0, 28.0));
                grad_fill(&vp, r, 14.0, c(60, 40, 8, 0.78), c(60, 40, 8, 0.7));
                vp.text(r.center(), Align2::CENTER_CENTER, t, f_medium(T_META), hex("#ffd98a"));
            }
        }
        if e.show_stats {
            self.eiga_stats(&vp, area, &e);
        }
        // controls
        let hud_t = spring_to(&ctx, "eiga-hud", if hud { 1.0 } else { 0.0 }, 300.0, 30.0).clamp(0.0, 1.0);
        if hud_t > 0.02 {
            self.eiga_controls(ui, area, &mut e, hud_t);
        }
        {
            let evs = std::mem::take(&mut e.pending_evs);
            let cmd = e.pending_cmd.take();
            if !evs.is_empty() || cmd.is_some() {
                self.shell.eiga = e;
                self.eiga_events(evs);
                if let Some(cm) = cmd {
                    self.eiga_run(cm);
                }
                e = std::mem::take(&mut self.shell.eiga);
            }
        }
        if bg.clicked() {
            let ev = e.player.toggle_pause();
            self.shell.eiga = e;
            self.eiga_events(ev);
            e = std::mem::take(&mut self.shell.eiga);
        }
        if bg.double_clicked() {
            self.shell.eiga = e;
            self.eiga_run(Cmd::ToggleFullscreen);
            e = std::mem::take(&mut self.shell.eiga);
        }
        if e.player.paused && !hud {
            ctx.request_repaint();
        }
        if !hud && !e.player.paused && ui.rect_contains_pointer(area) {
            ctx.set_cursor_icon(egui::CursorIcon::None);
        }
        // jump-to-time sheet
        if let Some(mut text) = e.jump.take() {
            let r = Rect::from_center_size(area.center_top() + vec2(0.0, 90.0), vec2(300.0, 52.0));
            elevate(&p, &th, r, 18.0, 4);
            grad_fill(&p, r, 18.0, th.panel_top, th.panel_bottom);
            rim(&p, r, 18.0, 1.2, th.rim, true, 30.0);
            let rr = ui.put(Rect::from_min_max(r.min + vec2(18.0, 8.0), r.max - vec2(18.0, 8.0)), egui::TextEdit::singleline(&mut text).id(egui::Id::new("eiga-jump")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("Jump to  1:23:45").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center));
            if !rr.has_focus() {
                rr.request_focus();
            }
            let (enter, esc) = ctx.input(|i| (i.key_pressed(Key::Enter), i.key_pressed(Key::Escape)));
            if enter {
                if let Some(t) = parse_time(&text) {
                    let ev = e.player.seek_abs(t);
                    self.shell.eiga = e;
                    self.eiga_events(ev);
                    e = std::mem::take(&mut self.shell.eiga);
                }
            } else if !esc {
                e.jump = Some(text);
            }
        }
        if e.show_help {
            self.eiga_help(ui, area, &mut e);
        }
        // restore state, then drain actions that need &mut self
        if e.player.eof && e.playlist.items.len() > 1 && e.playlist.repeat != Repeat::Off {
            // handled by Ended event
        }
        self.shell.eiga = e;
    }

    fn eiga_default_video(&self) -> Option<Location> {
        let mut dirs = vec![];
        for id in ["movies", "videos"] {
            if let Some(p) = self.places.iter().find(|p| p.id == id) {
                dirs.push(p.loc.clone());
            }
        }
        dirs.push(self.be.home());
        for d in dirs {
            if let Some(v) = self.be.registry.get(&d.fs) {
                if let Some(e) = v.list(&d.path).unwrap_or_default().into_iter().find(|e| !e.is_dir && e.kind == Kind::Video) {
                    return Some(e.loc);
                }
            }
        }
        None
    }

    fn eiga_load_first(&mut self, l: Location) {
        let locs = self.eiga_siblings(&l);
        let start = locs.iter().position(|x| *x == l).unwrap_or(0);
        self.shell.eiga.playlist = Playlist::new(locs.iter().map(|x| x.name()).collect(), start);
        self.shell.eiga.locs = locs;
        self.eiga_load(l);
        self.shell.eiga.player.paused = true;
    }

    fn eiga_toolbar(&mut self, ui: &Ui, fr: &Frame, e: &Eiga) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let cy = fr.bar.center().y;
        let mut rx = fr.right_x;
        let more = Rect::from_min_size(pos2(rx - 34.0, cy - 17.0), vec2(34.0, 34.0));
        if self.icon_button(ui, more, "menu-dots", "ei-more", Btn::Raised, true).on_hover_text("More").clicked() {
            let items = self.eiga_more_menu();
            self.open_app_menu(pos2(more.right() - 250.0, more.bottom() + 8.0), items);
        }
        rx = more.left() - 8.0;
        let avail = rx - fr.lead_x;
        // dropdown chips: speed · subtitles · audio
        let speed_label = format!("\u{d7}{}", (e.player.speed * 100.0).round() / 100.0);
        let sub_label = match e.sub_i.and_then(|i| e.subs.get(i)) {
            Some(s) => if s.lang.is_empty() { "Subtitles".to_string() } else { s.lang.to_uppercase() },
            None => "Subs off".to_string(),
        };
        let chips: [(&'static str, String, &str, u8); 3] = [("bolt", speed_label, "ei-speed", 0), ("subtitles", sub_label, "ei-subs", 1), ("volume-loud", format!("Audio {}", e.audio_i + 1), "ei-audio", 2)];
        for (icon, label, id, which) in chips {
            if avail < 380.0 && which == 2 {
                continue;
            }
            let w = self.tw(&p, &label, &f_semi(T_META)) + 52.0;
            let r = Rect::from_min_size(pos2(rx - w, cy - 16.0), vec2(w, 32.0));
            let resp = self.hit(ui, r, id, Sense::click());
            raised(&p, &th, r, 16.0, resp.hovered(), resp.is_pointer_button_down_on());
            self.ico(&p, icon, Ic::Duo, pos2(r.left() + 18.0, r.center().y), 15.0, th.accent_ink);
            p.text(pos2(r.left() + 32.0, r.center().y), Align2::LEFT_CENTER, &label, f_semi(T_META), th.ink1);
            self.ico(&p, "alt-arrow-down", Ic::Outline, pos2(r.right() - 12.0, r.center().y), 11.0, th.ink3);
            if resp.clicked() {
                let items = match which {
                    0 => self.eiga_speed_menu(),
                    1 => self.eiga_sub_menu(),
                    _ => self.eiga_audio_menu(),
                };
                self.open_app_menu(pos2(r.left(), r.bottom() + 8.0), items);
            }
            rx = r.left() - 8.0;
        }
        // title
        let tx = fr.lead_x;
        let w = (rx - tx - 12.0).max(40.0);
        self.ico(&p, "clapperboard", Ic::Duo, pos2(tx + 10.0, cy), 18.0, th.accent_ink);
        p.text(pos2(tx + 28.0, cy - 7.0), Align2::LEFT_CENTER, self.fit(&p, &e.title, &f_semi(T_BODY), w - 30.0), f_semi(T_BODY), th.ink1);
        let sub = match &e.info {
            Some(i) => {
                let (w, h) = i.size_wh();
                let mut parts = vec![];
                if w > 0 {
                    parts.push(format!("{w}\u{d7}{h}"));
                }
                if let Some(v) = i.primary_video() {
                    parts.push(format!("{} {:.0} fps", v.codec, i.fps()));
                }
                parts.push(fmt_time(i.duration));
                parts.join(" \u{b7} ")
            }
            None => String::new(),
        };
        p.text(pos2(tx + 28.0, cy + 9.0), Align2::LEFT_CENTER, self.fit(&p, &sub, &f_medium(10.5), w - 30.0), f_medium(10.5), th.ink3);
    }

    fn eiga_sidebar(&mut self, ui: &mut Ui, fr: &Frame, e: &Eiga) {
        if fr.side.width() < 24.0 {
            return;
        }
        let th = self.th.clone();
        let saved = ui.clip_rect();
        let mut ui2 = ui.new_child(egui::UiBuilder::new().max_rect(fr.side));
        ui2.set_clip_rect(fr.side.intersect(saved));
        let ui = &ui2;
        let p = ui.painter().clone();
        let head = Rect::from_min_size(pos2(fr.win.left() + 14.0, fr.win.top() + 54.0), vec2(SIDEBAR_W - 28.0, 44.0));
        p.text(pos2(head.left() + 4.0, head.center().y - 7.0), Align2::LEFT_CENTER, "Eiga", f_bold(T_TITLE), th.ink1);
        p.text(pos2(head.left() + 4.0, head.center().y + 11.0), Align2::LEFT_CENTER, "Video player \u{b7} \u{6620}\u{753b}", f_medium(10.5), th.ink3);
        p.text(pos2(head.left() + 4.0, head.bottom() + 16.0), Align2::LEFT_CENTER, if e.locs.len() > 1 { "UP NEXT" } else { "PLAYLIST" }, f_semi(T_LABEL), th.ink4);
        let list = Rect::from_min_max(pos2(fr.win.left() + 10.0, head.bottom() + 30.0), pos2(fr.win.left() + SIDEBAR_W - 6.0, fr.win.bottom() - 12.0));
        let row_h = 46.0;
        let off = self.vscroll(ui, list, "eiga-list", e.locs.len() as f32 * row_h + 4.0);
        let lp = p.with_clip_rect(list);
        for (i, l) in e.locs.iter().enumerate() {
            let r = Rect::from_min_size(pos2(list.left(), list.top() + i as f32 * row_h - off), vec2(list.width() - 10.0, 40.0));
            if r.bottom() < list.top() || r.top() > list.bottom() {
                continue;
            }
            let resp = self.hit(ui, r, ("eiga-row", i), Sense::click());
            let on = e.playlist.cur == Some(i);
            if on {
                grad_fill(&lp, r, 12.0, th.accent_tint, th.accent_tint2);
                rim(&lp, r, 12.0, 1.0, alpha(Color32::WHITE, if th.dark { 0.10 } else { 0.7 }), true, 12.0);
            } else if resp.hovered() {
                lp.rect_filled(r, cr(12.0), th.row_hover);
            }
            self.ico(&lp, if on && !e.player.paused { "volume-loud" } else { "clapperboard" }, if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 20.0, r.center().y), 16.0, if on { th.accent_ink } else { th.ink3 });
            lp.text(pos2(r.left() + 38.0, r.center().y), Align2::LEFT_CENTER, self.fit(&lp, &l.name(), &if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, r.width() - 48.0), if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, if on { th.ink1 } else { th.ink2 });
            if resp.clicked() {
                self.push_act(Act::App(format!("eiga.play_idx:{i}")));
            }
        }
    }

    fn eiga_controls(&mut self, ui: &Ui, area: Rect, e: &mut Eiga, t: f32) {
        let th = self.th.clone();
        let p = ui.painter().with_clip_rect(area);
        let h = 104.0;
        let shade = Rect::from_min_max(pos2(area.left(), area.bottom() - h - 30.0), area.max);
        let mut m = egui::epaint::Mesh::default();
        for (y, col) in [(shade.top(), Color32::TRANSPARENT), (shade.bottom(), c(0, 0, 0, 0.72 * t))] {
            m.vertices.push(egui::epaint::Vertex { pos: pos2(shade.left(), y), uv: egui::epaint::WHITE_UV, color: col });
            m.vertices.push(egui::epaint::Vertex { pos: pos2(shade.right(), y), uv: egui::epaint::WHITE_UV, color: col });
        }
        m.indices.extend([0, 1, 2, 1, 3, 2]);
        p.add(egui::Shape::mesh(m));
        let lift = (1.0 - t) * 14.0;
        let white = alpha(Color32::WHITE, t);
        // seek bar
        let bar = Rect::from_min_max(pos2(area.left() + 24.0, area.bottom() - h + 10.0 + lift), pos2(area.right() - 24.0, area.bottom() - h + 22.0 + lift));
        let hit = self.hit(ui, bar.expand2(vec2(0.0, 10.0)), "eiga-seek", Sense::click_and_drag());
        let dur = e.player.duration.max(0.001);
        let frac = match e.scrub {
            Some(s) => (s / dur).clamp(0.0, 1.0),
            None => e.player.fraction(),
        };
        let hv = self.anim(ui, "eiga-seek-h", hit.hovered() || hit.dragged());
        let track = Rect::from_center_size(bar.center(), vec2(bar.width(), 4.0 + 3.0 * hv));
        p.rect_filled(track, cr(3.0), alpha(Color32::WHITE, 0.22 * t));
        for ch in &e.player.chapters {
            let x = track.left() + track.width() * (*ch / dur) as f32;
            if x > track.left() + 3.0 {
                p.rect_filled(Rect::from_center_size(pos2(x, track.center().y), vec2(2.0, track.height() + 4.0)), cr(1.0), alpha(Color32::BLACK, 0.55 * t));
            }
        }
        if let LoopMode::Ab { a, b } = e.player.looping {
            let xa = track.left() + track.width() * (a / dur) as f32;
            let xb = b.map(|b| track.left() + track.width() * (b / dur) as f32).unwrap_or(xa + 4.0);
            p.rect_filled(Rect::from_min_max(pos2(xa, track.top() - 3.0), pos2(xb, track.bottom() + 3.0)), cr(2.0), alpha(th.warn, 0.55 * t));
        }
        let (c0, c1) = th.accent_grad();
        let fill = Rect::from_min_size(track.min, vec2((track.width() * frac as f32).max(3.0), track.height()));
        grad_fill(&p, fill, 2.5, alpha(c0, t), alpha(c1, t));
        let knob = spring_to(ui.ctx(), "eiga-knob", 5.0 + 3.0 * hv, 380.0, 26.0);
        p.circle_filled(pos2(fill.right(), track.center().y), knob, white);
        if hit.dragged() || hit.clicked() {
            if let Some(pp) = hit.interact_pointer_pos() {
                e.scrub = Some(((pp.x - track.left()) / track.width()).clamp(0.0, 1.0) as f64 * dur);
            }
        }
        if hit.hovered() || hit.dragged() {
            if let Some(pp) = ui.input(|i| i.pointer.hover_pos()) {
                let tt = ((pp.x - track.left()) / track.width()).clamp(0.0, 1.0) as f64 * dur;
                let label = match e.player.chapters.iter().rposition(|c| *c <= tt) {
                    Some(i) if e.info.as_ref().map(|x| x.chapters.get(i).map(|c| !c.1.is_empty()).unwrap_or(false)).unwrap_or(false) => format!("{} \u{b7} {}", fmt_time(tt), e.info.as_ref().unwrap().chapters[i].1),
                    _ => fmt_time(tt),
                };
                let w = self.tw(&p, &label, &f_semi(T_META)) + 20.0;
                let r = Rect::from_center_size(pos2(pp.x.clamp(area.left() + w / 2.0 + 8.0, area.right() - w / 2.0 - 8.0), track.top() - 22.0), vec2(w, 26.0));
                grad_fill(&p, r, 13.0, c(10, 14, 24, 0.85), c(10, 14, 24, 0.8));
                p.text(r.center(), Align2::CENTER_CENTER, label, f_semi(T_META), Color32::WHITE);
            }
        }
        let mut seek_to = None;
        if hit.drag_stopped() || hit.clicked() {
            seek_to = e.scrub.take();
        }
        // transport row
        let cy = area.bottom() - 40.0 + lift;
        let cx = area.center().x;
        let btn = |app: &SmartApp, ui: &Ui, x: f32, icon: &'static str, id: &str, size: f32, kind: Btn| -> bool { app.icon_button(ui, Rect::from_center_size(pos2(x, cy), vec2(size, size)), icon, id, kind, true).clicked() };
        let mut cmd: Option<Cmd> = None;
        if btn(self, ui, cx - 108.0, "skip-previous", "eiga-prev", 36.0, Btn::Quiet) {
            cmd = Some(Cmd::PrevItem);
        }
        if btn(self, ui, cx - 62.0, "rewind-10-seconds-back", "eiga-b10", 38.0, Btn::Quiet) {
            cmd = Some(Cmd::SeekRel(-10.0));
        }
        let pb = Rect::from_center_size(pos2(cx, cy), vec2(52.0, 52.0));
        let pr = self.hit(ui, pb, "eiga-play", Sense::click());
        raised_accent(&p, &th, pb, 26.0, pr.hovered(), pr.is_pointer_button_down_on());
        self.ico(&p, if e.player.paused { "play" } else { "pause" }, Ic::Filled, pb.center() + vec2(if e.player.paused { 2.0 } else { 0.0 }, 0.0), 24.0, th.on_accent());
        if pr.clicked() {
            cmd = Some(Cmd::TogglePause);
        }
        if btn(self, ui, cx + 62.0, "rewind-10-seconds-forward", "eiga-f10", 38.0, Btn::Quiet) {
            cmd = Some(Cmd::SeekRel(10.0));
        }
        if btn(self, ui, cx + 108.0, "skip-next", "eiga-next", 36.0, Btn::Quiet) {
            cmd = Some(Cmd::NextItem);
        }
        // time
        let pos = e.scrub.unwrap_or(e.player.position);
        p.text(pos2(area.left() + 28.0, cy), Align2::LEFT_CENTER, format!("{} / {}", fmt_time(pos), fmt_time(e.player.duration)), f_semi(T_BODY), white);
        // volume + fullscreen
        let fs = Rect::from_center_size(pos2(area.right() - 40.0, cy), vec2(36.0, 36.0));
        if self.icon_button(ui, fs, "maximize", "eiga-fs", Btn::Quiet, true).on_hover_text("Fullscreen  F").clicked() {
            cmd = Some(Cmd::ToggleFullscreen);
        }
        let vol_track = Rect::from_min_size(pos2(fs.left() - 112.0, cy - 4.0), vec2(84.0, 8.0));
        let mute = Rect::from_center_size(pos2(vol_track.left() - 24.0, cy), vec2(34.0, 34.0));
        if self.icon_button(ui, mute, if e.player.muted || e.player.volume == 0.0 { "volume-cross" } else { "volume-loud" }, "eiga-mute", Btn::Quiet, true).on_hover_text("Mute  M").clicked() {
            cmd = Some(Cmd::ToggleMute);
        }
        if area.width() > 640.0 {
            let (nv, r) = self.slider(ui, vol_track, if e.player.muted { 0.0 } else { e.player.volume / eiga_core::player::MAX_VOLUME }, "eiga-vol");
            if r.dragged() || r.clicked() {
                let target = nv * eiga_core::player::MAX_VOLUME;
                let d = target - e.player.volume;
                if d.abs() > 0.5 {
                    let evs = e.player.volume_step(d);
                    e.player.muted = false;
                    e.pending_evs.extend(evs);
                }
            }
        }
        if let Some(t) = seek_to {
            e.pending_evs.extend(e.player.seek_abs(t));
        }
        if let Some(cmd) = cmd {
            e.pending_cmd = Some(cmd);
        }
    }

    fn eiga_stats(&self, p: &egui::Painter, area: Rect, e: &Eiga) {
        let rows: Vec<(String, String)> = vec![
            ("Source".into(), e.src.as_ref().map(|s| s.name().to_string()).unwrap_or_default()),
            ("Resolution".into(), format!("{}\u{d7}{}", e.frame_wh.0, e.frame_wh.1)),
            ("Display rate".into(), format!("{:.1} fps", e.fps_meter.2)),
            ("Clock".into(), format!("{} / {}", eiga_core::fmt_time_ms(e.player.position), eiga_core::fmt_time_ms(e.player.duration))),
            ("Speed".into(), format!("\u{d7}{:.2}", e.player.speed)),
            ("Volume".into(), format!("{:.0}%{}", e.player.volume, if e.player.muted { " (muted)" } else { "" })),
            ("Audio delay".into(), format!("{:+.0} ms", e.player.audio_delay * 1000.0)),
            ("Subtitles".into(), e.sub_i.and_then(|i| e.subs.get(i)).map(|s| format!("{} \u{b7} {} cues \u{b7} {:+.0} ms", s.name, s.len(), s.delay * 1000.0)).unwrap_or_else(|| "off".into())),
            ("Container".into(), e.info.as_ref().map(|i| format!("{} \u{b7} {} kb/s", i.container, i.bitrate / 1000)).unwrap_or_default()),
        ];
        let r = Rect::from_min_size(area.right_top() + vec2(-288.0, 16.0), vec2(272.0, 18.0 + rows.len() as f32 * 20.0));
        grad_fill(p, r, 14.0, c(8, 12, 20, 0.82), c(8, 12, 20, 0.76));
        for (i, (k, v)) in rows.iter().enumerate() {
            let y = r.top() + 16.0 + i as f32 * 20.0;
            p.text(pos2(r.left() + 14.0, y), Align2::LEFT_CENTER, k, f_medium(10.5), Color32::from_white_alpha(150));
            p.text(pos2(r.right() - 14.0, y), Align2::RIGHT_CENTER, self.fit(p, v, &f_semi(10.5), 160.0), f_semi(10.5), Color32::WHITE);
        }
    }

    fn eiga_help(&mut self, ui: &Ui, area: Rect, e: &mut Eiga) {
        let th = self.th.clone();
        let p = ui.painter().with_clip_rect(area);
        let mut groups: Vec<(&str, Vec<(String, &str)>)> = vec![];
        for b in &e.bindings {
            let mut k = String::new();
            if b.ctrl {
                k += "Ctrl ";
            }
            if b.alt {
                k += "Alt ";
            }
            if b.shift {
                k += "Shift ";
            }
            k += &match b.key {
                "space" => "Space".to_string(),
                "left" => "\u{2190}".into(),
                "right" => "\u{2192}".into(),
                "up" => "\u{2191}".into(),
                "down" => "\u{2193}".into(),
                o if o.len() == 1 => o.to_uppercase(),
                o => o[..1].to_uppercase() + &o[1..],
            };
            match groups.iter_mut().find(|g| g.0 == b.group) {
                Some(g) => {
                    if let Some(x) = g.1.iter_mut().find(|x| x.1 == b.help) {
                        x.0 += &format!(" / {k}");
                    } else {
                        g.1.push((k, b.help));
                    }
                }
                None => groups.push((b.group, vec![(k, b.help)])),
            }
        }
        let scrim = ui.interact(area, egui::Id::new("eiga-help-scrim"), Sense::click());
        p.rect_filled(area, cr(16.0), c(0, 0, 0, 0.55));
        let card = Rect::from_center_size(area.center(), vec2((area.width() - 60.0).min(780.0), (area.height() - 50.0).min(520.0)));
        elevate(&p, &th, card, 20.0, 5);
        grad_fill(&p, card, 20.0, th.panel_top, th.panel_bottom);
        rim(&p, card, 20.0, 1.3, th.rim, true, 40.0);
        p.text(card.min + vec2(24.0, 26.0), Align2::LEFT_CENTER, "Eiga shortcuts", f_bold(T_TITLE), th.ink1);
        p.text(card.min + vec2(24.0, 46.0), Align2::LEFT_CENTER, "Same keys as mpv wherever mpv has an opinion. Press H or Esc to close.", f_medium(T_META), th.ink3);
        let inner = Rect::from_min_max(card.min + vec2(20.0, 66.0), card.max - vec2(20.0, 14.0));
        let off = self.vscroll(ui, inner, "eiga-help", 1100.0);
        let ip = p.with_clip_rect(inner);
        let colw = inner.width() / 2.0;
        let (mut y, mut col) = (inner.top() - off, 0usize);
        let mut ys = [inner.top() - off; 2];
        for (g, rows) in &groups {
            let x = inner.left() + col as f32 * colw;
            ip.text(pos2(x + 4.0, ys[col] + 10.0), Align2::LEFT_CENTER, g.to_uppercase(), f_semi(T_LABEL), th.ink4);
            ys[col] += 26.0;
            for (k, h) in rows {
                ip.text(pos2(x + 4.0, ys[col] + 11.0), Align2::LEFT_CENTER, self.fit(&ip, h, &f_medium(T_BODY), colw - 150.0), f_medium(T_BODY), th.ink2);
                let kw = self.tw(&ip, k, &f_semi(10.5)) + 14.0;
                let kr = Rect::from_min_size(pos2(x + colw - 20.0 - kw, ys[col] + 2.0), vec2(kw, 20.0));
                well(&ip, &th, kr, 7.0);
                ip.text(kr.center(), Align2::CENTER_CENTER, k, f_semi(10.5), th.ink2);
                ys[col] += 26.0;
            }
            ys[col] += 8.0;
            col = if ys[0] <= ys[1] { 0 } else { 1 };
        }
        y += 0.0;
        let _ = y;
        if scrim.clicked() && !ui.rect_contains_pointer(card) || ui.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::H)) {
            e.show_help = false;
        }
    }
}
