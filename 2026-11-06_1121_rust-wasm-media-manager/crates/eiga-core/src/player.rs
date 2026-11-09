//! Playback state machine, playlist and key map. Pure logic: feed it `tick(dt)`, read back [`Event`]s.
use crate::subs::SubTrack;

pub const SPEEDS: [f64; 10] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
pub const MAX_VOLUME: f32 = 130.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LoopMode {
    Off,
    /// Repeat the current file.
    File,
    /// Repeat between A and B. `b` is `None` while the user has only set A.
    Ab { a: f64, b: Option<f64> },
}

/// What the engine/UI must do in response to a state change.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    /// Restart decoding at this media time.
    Seek(f64),
    Ended,
    PauseChanged(bool),
    SpeedChanged(f64),
    VolumeChanged,
    /// Short message for the on-screen display ("Speed ×1.25").
    Osd,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Chapter {
    pub start: f64,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub duration: f64,
    pub position: f64,
    pub paused: bool,
    pub speed: f64,
    pub volume: f32,
    pub muted: bool,
    pub looping: LoopMode,
    pub eof: bool,
    pub fps: f64,
    pub audio_delay: f64,
    pub chapters: Vec<f64>,
    pub osd: Option<String>,
}

impl Default for Player {
    fn default() -> Self {
        Player { duration: 0.0, position: 0.0, paused: false, speed: 1.0, volume: 100.0, muted: false, looping: LoopMode::Off, eof: false, fps: 25.0, audio_delay: 0.0, chapters: vec![], osd: None }
    }
}

impl Player {
    pub fn new(duration: f64, fps: f64) -> Player {
        Player { duration: if duration.is_finite() { duration.max(0.0) } else { 0.0 }, fps: if fps.is_finite() && fps > 1.0 && fps < 480.0 { fps } else { 25.0 }, ..Player::default() }
    }

    fn say(&mut self, s: impl Into<String>) {
        self.osd = Some(s.into());
    }

    pub fn clamp_pos(&self, t: f64) -> f64 {
        if !t.is_finite() {
            return 0.0;
        }
        if self.duration > 0.0 {
            t.clamp(0.0, (self.duration - 0.001).max(0.0))
        } else {
            t.max(0.0)
        }
    }

    pub fn fraction(&self) -> f64 {
        if self.duration > 0.0 {
            (self.position / self.duration).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Advance the clock. Returns events (A-B wrap, loop, end of file).
    pub fn tick(&mut self, dt: f64) -> Vec<Event> {
        let mut ev = vec![];
        if self.paused || self.eof || !dt.is_finite() || dt <= 0.0 {
            return ev;
        }
        let dt = dt.min(0.25); // a stalled frame must not skip half a minute of video
        self.position += dt * self.speed;
        if let LoopMode::Ab { a, b: Some(b) } = self.looping {
            if self.position >= b {
                self.position = a;
                ev.push(Event::Seek(a));
                return ev;
            }
        }
        if self.duration > 0.0 && self.position >= self.duration {
            if self.looping == LoopMode::File {
                self.position = 0.0;
                ev.push(Event::Seek(0.0));
            } else {
                self.position = self.duration;
                self.eof = true;
                self.paused = true;
                ev.push(Event::Ended);
            }
        }
        ev
    }

    pub fn seek_abs(&mut self, t: f64) -> Vec<Event> {
        self.position = self.clamp_pos(t);
        self.eof = false;
        vec![Event::Seek(self.position)]
    }
    pub fn seek_rel(&mut self, d: f64) -> Vec<Event> {
        let was_eof = self.eof;
        let base = if was_eof { self.duration } else { self.position };
        let ev = self.seek_abs(base + d);
        self.say(format!("{}{}s", if d >= 0.0 { "+" } else { "\u{2212}" }, d.abs()));
        ev
    }
    pub fn seek_frac(&mut self, f: f64) -> Vec<Event> {
        self.seek_abs(f.clamp(0.0, 1.0) * self.duration)
    }

    pub fn set_paused(&mut self, p: bool) -> Vec<Event> {
        if self.eof && !p {
            // pressing play at the end restarts, like every player
            self.position = 0.0;
            self.eof = false;
            self.paused = false;
            return vec![Event::Seek(0.0), Event::PauseChanged(false)];
        }
        if self.paused == p {
            return vec![];
        }
        self.paused = p;
        vec![Event::PauseChanged(p)]
    }
    pub fn toggle_pause(&mut self) -> Vec<Event> {
        let p = !self.paused;
        self.set_paused(p)
    }

    /// Step one frame (mpv `.` and `,`). Pauses first.
    pub fn frame_step(&mut self, dir: i32) -> Vec<Event> {
        let mut ev = vec![];
        if !self.paused {
            self.paused = true;
            ev.push(Event::PauseChanged(true));
        }
        let t = self.position + dir as f64 / self.fps;
        ev.extend(self.seek_abs(t));
        self.say(if dir >= 0 { "Next frame" } else { "Previous frame" });
        ev
    }

    pub fn set_speed(&mut self, s: f64) -> Vec<Event> {
        self.speed = if s.is_finite() { s.clamp(0.1, 8.0) } else { 1.0 };
        let msg = format!("Speed \u{d7}{:.2}", self.speed);
        self.say(msg);
        vec![Event::SpeedChanged(self.speed)]
    }
    /// mpv: `]` ×1.1, `[` ×(1/1.1)
    pub fn speed_scale(&mut self, f: f64) -> Vec<Event> {
        let s = (self.speed * f * 100.0).round() / 100.0;
        self.set_speed(s)
    }
    /// Move through the preset ladder ({/} in mpv halve/double; here ↑/↓ nudges along presets).
    pub fn speed_step(&mut self, up: bool) -> Vec<Event> {
        let cur = self.speed;
        let next = if up { SPEEDS.iter().cloned().find(|s| *s > cur + 1e-6) } else { SPEEDS.iter().rev().cloned().find(|s| *s < cur - 1e-6) };
        match next {
            Some(s) => self.set_speed(s),
            None => vec![],
        }
    }

    pub fn volume_step(&mut self, d: f32) -> Vec<Event> {
        self.volume = (self.volume + d).clamp(0.0, MAX_VOLUME);
        if d > 0.0 {
            self.muted = false;
        }
        let v = self.volume.round();
        self.say(format!("Volume {v}%"));
        vec![Event::VolumeChanged]
    }
    pub fn toggle_mute(&mut self) -> Vec<Event> {
        self.muted = !self.muted;
        self.say(if self.muted { "Muted".to_string() } else { "Unmuted".to_string() });
        vec![Event::VolumeChanged]
    }
    /// 0..1 gain for the output device (volume above 100 amplifies).
    pub fn gain(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.volume / 100.0
        }
    }

    /// mpv `l`: first press sets A, second sets B (and starts looping), third clears.
    pub fn cycle_ab(&mut self) {
        self.looping = match self.looping {
            LoopMode::Ab { a, b: None } if self.position > a + 0.2 => LoopMode::Ab { a, b: Some(self.position) },
            LoopMode::Ab { .. } => LoopMode::Off,
            _ => LoopMode::Ab { a: self.position, b: None },
        };
        let m = match self.looping {
            LoopMode::Ab { a, b: None } => format!("Loop A set at {}", crate::fmt_time_ms(a)),
            LoopMode::Ab { a, b: Some(b) } => format!("Looping {} \u{2192} {}", crate::fmt_time_ms(a), crate::fmt_time_ms(b)),
            _ => "Loop cleared".to_string(),
        };
        self.say(m);
    }
    pub fn toggle_loop_file(&mut self) {
        self.looping = if self.looping == LoopMode::File { LoopMode::Off } else { LoopMode::File };
        self.say(if self.looping == LoopMode::File { "Repeat file" } else { "Repeat off" });
    }

    pub fn next_chapter(&mut self) -> Vec<Event> {
        match self.chapters.iter().cloned().find(|c| *c > self.position + 0.5) {
            Some(c) => self.seek_abs(c),
            None => vec![],
        }
    }
    pub fn prev_chapter(&mut self) -> Vec<Event> {
        // within the first 3 s of a chapter, jump to the previous one; otherwise restart this one
        let cur = self.chapters.iter().cloned().filter(|c| *c <= self.position - 3.0).last();
        match cur {
            Some(c) => self.seek_abs(c),
            None => self.seek_abs(0.0),
        }
    }
    pub fn chapter_index(&self) -> Option<usize> {
        self.chapters.iter().rposition(|c| *c <= self.position + 0.01)
    }
}

// ─────────────────────────────────────────── playlist ───────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Repeat {
    Off,
    All,
    One,
}

#[derive(Clone, Debug)]
pub struct Playlist {
    pub items: Vec<String>,
    pub cur: Option<usize>,
    pub shuffle: bool,
    pub repeat: Repeat,
    order: Vec<usize>,
    seed: u64,
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist { items: vec![], cur: None, shuffle: false, repeat: Repeat::Off, order: vec![], seed: 0x9E3779B97F4A7C15 }
    }
}

impl Playlist {
    pub fn new(items: Vec<String>, start: usize) -> Playlist {
        let mut p = Playlist { items, ..Playlist::default() };
        p.cur = if p.items.is_empty() { None } else { Some(start.min(p.items.len() - 1)) };
        p.rebuild_order();
        p
    }
    fn rng(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    fn rebuild_order(&mut self) {
        let mut o: Vec<usize> = (0..self.items.len()).collect();
        if self.shuffle {
            for i in (1..o.len()).rev() {
                let j = (self.rng() % (i as u64 + 1)) as usize;
                o.swap(i, j);
            }
            // current item first so toggling shuffle never changes what is playing
            if let Some(c) = self.cur {
                if let Some(p) = o.iter().position(|x| *x == c) {
                    o.swap(0, p);
                }
            }
        }
        self.order = o;
    }
    pub fn set_shuffle(&mut self, on: bool) {
        self.shuffle = on;
        self.rebuild_order();
    }
    pub fn add(&mut self, path: String) {
        self.items.push(path);
        if self.cur.is_none() {
            self.cur = Some(0);
        }
        self.rebuild_order();
    }
    pub fn remove(&mut self, i: usize) {
        if i >= self.items.len() {
            return;
        }
        self.items.remove(i);
        self.cur = match self.cur {
            _ if self.items.is_empty() => None,
            Some(c) if c > i => Some(c - 1),
            Some(c) => Some(c.min(self.items.len() - 1)),
            None => None,
        };
        self.rebuild_order();
    }
    pub fn current(&self) -> Option<&str> {
        self.cur.and_then(|c| self.items.get(c)).map(|s| s.as_str())
    }
    fn pos_in_order(&self) -> Option<usize> {
        let c = self.cur?;
        self.order.iter().position(|x| *x == c)
    }
    /// Index to play after the current one finished (`auto`) or when the user presses Next.
    pub fn next(&mut self, auto: bool) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        if auto && self.repeat == Repeat::One {
            return self.cur;
        }
        let p = self.pos_in_order()?;
        let n = if p + 1 < self.order.len() {
            self.order[p + 1]
        } else if self.repeat == Repeat::All || !auto {
            if self.shuffle {
                self.rebuild_order();
            }
            self.order[0]
        } else {
            return None;
        };
        self.cur = Some(n);
        self.cur
    }
    pub fn prev(&mut self) -> Option<usize> {
        let p = self.pos_in_order()?;
        let n = if p > 0 { self.order[p - 1] } else { *self.order.last()? };
        self.cur = Some(n);
        self.cur
    }
    pub fn cycle_repeat(&mut self) {
        self.repeat = match self.repeat {
            Repeat::Off => Repeat::All,
            Repeat::All => Repeat::One,
            Repeat::One => Repeat::Off,
        };
    }
}

// ─────────────────────────────────────────── key map ───────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cmd {
    TogglePause,
    SeekRel(f64),
    FrameStep(i32),
    SpeedScale(f64),
    SpeedStep(bool),
    SpeedReset,
    VolumeStep(f32),
    ToggleMute,
    LoopAb,
    LoopFile,
    NextChapter,
    PrevChapter,
    NextItem,
    PrevItem,
    CycleSub,
    CycleAudio,
    SubDelay(f64),
    AudioDelay(f64),
    SubScale(f32),
    NextSubCue,
    PrevSubCue,
    ToggleFullscreen,
    ToggleStats,
    ToggleHud,
    TogglePlaylist,
    ToggleShuffle,
    CycleRepeat,
    CycleAspect,
    Screenshot,
    JumpTo,
    OpenFile,
    Zoom(f32),
    Rotate,
    ToggleHelp,
    Close,
    SeekStart,
    SeekFrac(u8),
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub key: &'static str,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub cmd: Cmd,
    pub group: &'static str,
    pub help: &'static str,
}

const fn b(key: &'static str, ctrl: bool, shift: bool, alt: bool, cmd: Cmd, group: &'static str, help: &'static str) -> Binding {
    Binding { key, ctrl, shift, alt, cmd, group, help }
}

/// Defaults follow mpv wherever mpv has an opinion, so muscle memory transfers.
pub fn default_bindings() -> Vec<Binding> {
    use Cmd::*;
    vec![
        b("space", false, false, false, TogglePause, "Playback", "Play / pause"),
        b("k", false, false, false, TogglePause, "Playback", "Play / pause"),
        b("left", false, false, false, SeekRel(-5.0), "Seek", "Back 5 s"),
        b("right", false, false, false, SeekRel(5.0), "Seek", "Forward 5 s"),
        b("left", false, true, false, SeekRel(-1.0), "Seek", "Back 1 s (exact)"),
        b("right", false, true, false, SeekRel(1.0), "Seek", "Forward 1 s (exact)"),
        b("j", false, false, false, SeekRel(-10.0), "Seek", "Back 10 s"),
        b("l", false, false, false, SeekRel(10.0), "Seek", "Forward 10 s"),
        b("down", false, false, false, SeekRel(-60.0), "Seek", "Back 1 min"),
        b("up", false, false, false, SeekRel(60.0), "Seek", "Forward 1 min"),
        b("home", false, false, false, SeekStart, "Seek", "Jump to start"),
        b(",", false, false, false, FrameStep(-1), "Seek", "Previous frame"),
        b(".", false, false, false, FrameStep(1), "Seek", "Next frame"),
        b("pageup", false, false, false, NextChapter, "Seek", "Next chapter"),
        b("pagedown", false, false, false, PrevChapter, "Seek", "Previous chapter"),
        b("g", true, false, false, JumpTo, "Seek", "Jump to time\u{2026}"),
        b("]", false, false, false, SpeedScale(1.1), "Speed", "Faster (\u{d7}1.1)"),
        b("[", false, false, false, SpeedScale(1.0 / 1.1), "Speed", "Slower (\u{f7}1.1)"),
        b("}", false, false, false, SpeedScale(2.0), "Speed", "Double speed"),
        b("{", false, false, false, SpeedScale(0.5), "Speed", "Half speed"),
        b("backspace", false, false, false, SpeedReset, "Speed", "Reset speed"),
        b("up", false, true, false, SpeedStep(true), "Speed", "Next speed preset"),
        b("down", false, true, false, SpeedStep(false), "Speed", "Previous speed preset"),
        b("0", false, false, false, VolumeStep(5.0), "Audio", "Volume +5"),
        b("9", false, false, false, VolumeStep(-5.0), "Audio", "Volume \u{2212}5"),
        b("/", false, false, false, VolumeStep(-5.0), "Audio", "Volume \u{2212}5"),
        b("*", false, false, false, VolumeStep(5.0), "Audio", "Volume +5"),
        b("m", false, false, false, ToggleMute, "Audio", "Mute"),
        b("up", true, false, false, VolumeStep(5.0), "Audio", "Volume +5"),
        b("down", true, false, false, VolumeStep(-5.0), "Audio", "Volume \u{2212}5"),
        b("#", false, false, false, CycleAudio, "Audio", "Next audio track"),
        b("z", false, false, false, AudioDelay(-0.1), "Audio", "Audio earlier 100 ms"),
        b("x", false, false, false, AudioDelay(0.1), "Audio", "Audio later 100 ms"),
        b("v", false, false, false, CycleSub, "Subtitles", "Cycle subtitle track / off"),
        b("j", false, true, false, CycleSub, "Subtitles", "Cycle subtitle track"),
        b("z", false, true, false, SubDelay(-0.1), "Subtitles", "Subtitles earlier 100 ms"),
        b("x", false, true, false, SubDelay(0.1), "Subtitles", "Subtitles later 100 ms"),
        b("r", false, false, false, SubScale(-0.05), "Subtitles", "Smaller subtitles"),
        b("t", false, false, false, SubScale(0.05), "Subtitles", "Larger subtitles"),
        b("n", false, false, false, NextSubCue, "Subtitles", "Next subtitle line"),
        b("n", false, true, false, PrevSubCue, "Subtitles", "Previous subtitle line"),
        b("f", false, false, false, ToggleFullscreen, "Window", "Fullscreen"),
        b("enter", false, false, false, ToggleFullscreen, "Window", "Fullscreen"),
        b("escape", false, false, false, Close, "Window", "Leave fullscreen / close player"),
        b("a", false, false, false, CycleAspect, "Window", "Cycle aspect: fit \u{b7} fill \u{b7} 16:9 \u{b7} 4:3"),
        b("=", false, false, false, Zoom(0.1), "Window", "Zoom in"),
        b("-", false, false, false, Zoom(-0.1), "Window", "Zoom out"),
        b("r", true, false, false, Rotate, "Window", "Rotate 90\u{b0}"),
        b("tab", false, false, false, ToggleHud, "Window", "Show / hide controls"),
        b("i", false, false, false, ToggleStats, "Window", "Statistics overlay"),
        b("s", false, false, false, Screenshot, "Window", "Save screenshot"),
        b("o", true, false, false, OpenFile, "Playlist", "Open file\u{2026}"),
        b("p", false, false, false, TogglePlaylist, "Playlist", "Show / hide playlist"),
        b("n", true, false, false, NextItem, "Playlist", "Next item"),
        b("p", true, false, false, PrevItem, "Playlist", "Previous item"),
        b("period", false, true, false, NextItem, "Playlist", "Next item"),
        b("comma", false, true, false, PrevItem, "Playlist", "Previous item"),
        b("s", true, false, false, ToggleShuffle, "Playlist", "Shuffle"),
        b("l", true, false, false, CycleRepeat, "Playlist", "Repeat: off \u{b7} all \u{b7} one"),
        b("l", false, true, false, LoopAb, "Loop", "Set A \u{2192} B loop (press again to clear)"),
        b("l", false, false, true, LoopFile, "Loop", "Repeat this file"),
        b("h", false, false, false, ToggleHelp, "Help", "Shortcut sheet"),
        b("?", false, false, false, ToggleHelp, "Help", "Shortcut sheet"),
    ]
}

/// Number keys 1..9 seek to 10%..90%, like YouTube. Returned separately since they are one binding family.
pub fn lookup(bindings: &[Binding], key: &str, ctrl: bool, shift: bool, alt: bool) -> Option<Cmd> {
    if !ctrl && !alt && key.len() == 1 && key.as_bytes()[0].is_ascii_digit() && key != "0" && key != "9" {
        return Some(Cmd::SeekFrac(key.as_bytes()[0] - b'0'));
    }
    bindings.iter().find(|x| x.key == key && x.ctrl == ctrl && x.shift == shift && x.alt == alt).map(|x| x.cmd)
}

impl Player {
    /// Applies a command that only touches playback state. UI-level commands (fullscreen, playlist, tracks…)
    /// are returned unchanged for the host to handle.
    pub fn apply(&mut self, cmd: Cmd) -> (Vec<Event>, Option<Cmd>) {
        let ev = match cmd {
            Cmd::TogglePause => self.toggle_pause(),
            Cmd::SeekRel(d) => self.seek_rel(d),
            Cmd::SeekStart => self.seek_abs(0.0),
            Cmd::SeekFrac(n) => self.seek_frac(n as f64 / 10.0),
            Cmd::FrameStep(d) => self.frame_step(d),
            Cmd::SpeedScale(f) => self.speed_scale(f),
            Cmd::SpeedStep(u) => self.speed_step(u),
            Cmd::SpeedReset => self.set_speed(1.0),
            Cmd::VolumeStep(d) => self.volume_step(d),
            Cmd::ToggleMute => self.toggle_mute(),
            Cmd::LoopAb => {
                self.cycle_ab();
                vec![Event::Osd]
            }
            Cmd::LoopFile => {
                self.toggle_loop_file();
                vec![Event::Osd]
            }
            Cmd::NextChapter => self.next_chapter(),
            Cmd::PrevChapter => self.prev_chapter(),
            Cmd::AudioDelay(d) => {
                self.audio_delay = ((self.audio_delay + d) * 1000.0).round() / 1000.0;
                let m = format!("Audio delay {:+.0} ms", self.audio_delay * 1000.0);
                self.say(m);
                vec![Event::Osd]
            }
            other => return (vec![], Some(other)),
        };
        (ev, None)
    }
}

/// Applies a subtitle delay step and returns the OSD text.
pub fn nudge_sub_delay(t: &mut SubTrack, d: f64) -> String {
    t.delay = ((t.delay + d) * 1000.0).round() / 1000.0;
    format!("Subtitle delay {:+.0} ms", t.delay * 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p() -> Player {
        Player::new(100.0, 25.0)
    }

    #[test]
    fn clock_advances_with_speed_and_stops_at_end() {
        let mut p = p();
        p.set_speed(2.0);
        p.tick(0.1);
        assert!((p.position - 0.2).abs() < 1e-9);
        p.position = 99.9;
        let ev = p.tick(0.1);
        assert!(ev.contains(&Event::Ended));
        assert!(p.eof && p.paused);
        assert_eq!(p.position, 100.0);
        assert!(p.tick(1.0).is_empty(), "no movement after the end");
    }

    #[test]
    fn long_frame_stalls_are_capped() {
        let mut p = p();
        p.tick(30.0);
        assert!(p.position <= 0.25 + 1e-9);
    }

    #[test]
    fn play_at_end_restarts() {
        let mut p = p();
        p.position = 99.99;
        p.tick(0.2);
        assert!(p.eof);
        let ev = p.toggle_pause();
        assert!(ev.contains(&Event::Seek(0.0)));
        assert!(!p.paused && !p.eof && p.position == 0.0);
    }

    #[test]
    fn seeking_is_clamped_and_clears_eof() {
        let mut p = p();
        p.seek_abs(-5.0);
        assert_eq!(p.position, 0.0);
        p.seek_abs(1e9);
        assert!(p.position < 100.0);
        p.eof = true;
        p.seek_rel(-10.0);
        assert!(!p.eof);
        assert!((p.position - 89.999).abs() < 0.01 || p.position > 80.0);
        p.seek_abs(f64::NAN);
        assert_eq!(p.position, 0.0);
        p.seek_frac(0.5);
        assert!((p.position - 50.0).abs() < 1e-9);
    }

    #[test]
    fn frame_step_pauses_and_moves_one_frame() {
        let mut p = p();
        p.position = 10.0;
        let ev = p.frame_step(1);
        assert!(p.paused);
        assert!(ev.contains(&Event::PauseChanged(true)));
        assert!((p.position - 10.04).abs() < 1e-9);
        p.frame_step(-1);
        assert!((p.position - 10.0).abs() < 1e-9);
        p.position = 0.0;
        p.frame_step(-1);
        assert_eq!(p.position, 0.0);
    }

    #[test]
    fn speed_controls() {
        let mut p = p();
        p.speed_scale(1.1);
        assert!((p.speed - 1.1).abs() < 1e-9);
        p.set_speed(1.0);
        p.speed_step(true);
        assert_eq!(p.speed, 1.25);
        p.speed_step(false);
        p.speed_step(false);
        assert_eq!(p.speed, 0.75);
        p.set_speed(0.0);
        assert_eq!(p.speed, 0.1, "never stalls the clock");
        p.set_speed(f64::NAN);
        assert_eq!(p.speed, 1.0);
        p.set_speed(1e9);
        assert_eq!(p.speed, 8.0);
        p.set_speed(4.0);
        assert!(p.speed_step(true).is_empty(), "top of the ladder");
    }

    #[test]
    fn volume_and_mute() {
        let mut p = p();
        p.volume_step(100.0);
        assert_eq!(p.volume, MAX_VOLUME);
        p.volume_step(-500.0);
        assert_eq!(p.volume, 0.0);
        p.toggle_mute();
        assert_eq!(p.gain(), 0.0);
        p.volume_step(10.0);
        assert!(!p.muted, "raising the volume unmutes");
        assert!((p.gain() - 0.1).abs() < 1e-6);
    }

    #[test]
    fn ab_loop_wraps_and_clears() {
        let mut p = p();
        p.position = 10.0;
        p.cycle_ab();
        assert_eq!(p.looping, LoopMode::Ab { a: 10.0, b: None });
        p.position = 10.05;
        p.cycle_ab(); // B too close to A: treated as clear
        assert_eq!(p.looping, LoopMode::Off);
        p.position = 10.0;
        p.cycle_ab();
        p.position = 20.0;
        p.cycle_ab();
        assert_eq!(p.looping, LoopMode::Ab { a: 10.0, b: Some(20.0) });
        p.position = 19.95;
        let ev = p.tick(0.1);
        assert_eq!(ev, vec![Event::Seek(10.0)]);
        assert_eq!(p.position, 10.0);
        p.cycle_ab();
        assert_eq!(p.looping, LoopMode::Off);
    }

    #[test]
    fn file_loop_restarts_instead_of_ending() {
        let mut p = p();
        p.toggle_loop_file();
        p.position = 99.95;
        let ev = p.tick(0.1);
        assert_eq!(ev, vec![Event::Seek(0.0)]);
        assert!(!p.eof);
    }

    #[test]
    fn chapters() {
        let mut p = p();
        p.chapters = vec![0.0, 30.0, 60.0];
        p.position = 10.0;
        p.next_chapter();
        assert_eq!(p.position, 30.0);
        assert_eq!(p.chapter_index(), Some(1));
        p.position = 31.0;
        p.prev_chapter(); // within 3 s: previous chapter
        assert_eq!(p.position, 0.0);
        p.position = 45.0;
        p.prev_chapter();
        assert_eq!(p.position, 30.0);
        p.position = 70.0;
        assert!(p.next_chapter().is_empty());
    }

    #[test]
    fn apply_routes_ui_commands_back() {
        let mut p = p();
        let (_, rest) = p.apply(Cmd::ToggleFullscreen);
        assert_eq!(rest, Some(Cmd::ToggleFullscreen));
        let (ev, rest) = p.apply(Cmd::SeekRel(5.0));
        assert!(rest.is_none() && !ev.is_empty());
        assert_eq!(p.position, 5.0);
        p.apply(Cmd::AudioDelay(0.1));
        p.apply(Cmd::AudioDelay(0.1));
        assert!((p.audio_delay - 0.2).abs() < 1e-9);
        assert!(p.osd.as_deref().unwrap().contains("+200"));
    }

    #[test]
    fn unknown_duration_streams_never_end() {
        let mut p = Player::new(0.0, 0.0);
        assert_eq!(p.fps, 25.0);
        for _ in 0..100 {
            assert!(p.tick(0.1).is_empty());
        }
        assert!(!p.eof);
        assert_eq!(p.fraction(), 0.0);
    }

    fn pl(n: usize) -> Playlist {
        Playlist::new((0..n).map(|i| format!("f{i}.mkv")).collect(), 0)
    }

    #[test]
    fn playlist_sequential_and_repeat() {
        let mut l = pl(3);
        assert_eq!(l.next(true), Some(1));
        assert_eq!(l.next(true), Some(2));
        assert_eq!(l.next(true), None, "stops at the end");
        assert_eq!(l.cur, Some(2));
        assert_eq!(l.next(false), Some(0), "manual next wraps");
        l.repeat = Repeat::All;
        l.cur = Some(2);
        assert_eq!(l.next(true), Some(0));
        l.repeat = Repeat::One;
        assert_eq!(l.next(true), Some(0));
        assert_eq!(l.prev(), Some(2), "prev wraps");
    }

    #[test]
    fn shuffle_visits_everything_once_and_keeps_current_first() {
        let mut l = pl(8);
        l.cur = Some(3);
        l.set_shuffle(true);
        let mut seen = vec![l.cur.unwrap()];
        while let Some(n) = l.next(true) {
            seen.push(n);
        }
        assert_eq!(seen[0], 3);
        seen.sort();
        assert_eq!(seen, (0..8).collect::<Vec<_>>());
    }

    #[test]
    fn playlist_edit_keeps_cursor_valid() {
        let mut l = pl(4);
        l.cur = Some(2);
        l.remove(0);
        assert_eq!(l.current(), Some("f2.mkv"));
        l.remove(1);
        assert_eq!(l.cur, Some(1));
        l.remove(0);
        l.remove(0);
        assert!(l.cur.is_none() && l.items.is_empty());
        assert!(l.next(false).is_none() && l.prev().is_none());
        l.add("x.mp4".into());
        assert_eq!(l.current(), Some("x.mp4"));
        l.remove(99);
        assert_eq!(l.items.len(), 1);
    }

    #[test]
    fn bindings_are_unique_and_documented() {
        let b = default_bindings();
        for (i, x) in b.iter().enumerate() {
            assert!(!x.help.is_empty() && !x.group.is_empty());
            for y in b.iter().skip(i + 1) {
                assert!(!(x.key == y.key && x.ctrl == y.ctrl && x.shift == y.shift && x.alt == y.alt), "duplicate binding {} ctrl={} shift={}", x.key, x.ctrl, x.shift);
            }
        }
    }

    #[test]
    fn mpv_muscle_memory_is_preserved() {
        let b = default_bindings();
        assert_eq!(lookup(&b, "space", false, false, false), Some(Cmd::TogglePause));
        assert_eq!(lookup(&b, "left", false, false, false), Some(Cmd::SeekRel(-5.0)));
        assert_eq!(lookup(&b, "up", false, false, false), Some(Cmd::SeekRel(60.0)));
        assert_eq!(lookup(&b, "f", false, false, false), Some(Cmd::ToggleFullscreen));
        assert_eq!(lookup(&b, "m", false, false, false), Some(Cmd::ToggleMute));
        assert_eq!(lookup(&b, ".", false, false, false), Some(Cmd::FrameStep(1)));
        assert_eq!(lookup(&b, ",", false, false, false), Some(Cmd::FrameStep(-1)));
        assert_eq!(lookup(&b, "[", false, false, false), Some(Cmd::SpeedScale(1.0 / 1.1)));
        assert_eq!(lookup(&b, "{", false, false, false), Some(Cmd::SpeedScale(0.5)));
        assert_eq!(lookup(&b, "backspace", false, false, false), Some(Cmd::SpeedReset));
        assert_eq!(lookup(&b, "v", false, false, false), Some(Cmd::CycleSub));
        assert_eq!(lookup(&b, "z", false, false, false), Some(Cmd::AudioDelay(-0.1)));
        assert_eq!(lookup(&b, "x", false, true, false), Some(Cmd::SubDelay(0.1)));
        assert_eq!(lookup(&b, "l", false, true, false), Some(Cmd::LoopAb));
        assert_eq!(lookup(&b, "4", false, false, false), Some(Cmd::SeekFrac(4)));
        assert_eq!(lookup(&b, "nonsense", false, false, false), None);
        assert_eq!(lookup(&b, "space", true, false, false), None, "modifiers must match exactly");
    }

    #[test]
    fn sub_delay_nudge_rounds() {
        let mut t = SubTrack::default();
        nudge_sub_delay(&mut t, 0.1);
        nudge_sub_delay(&mut t, 0.1);
        let s = nudge_sub_delay(&mut t, 0.1);
        assert_eq!(t.delay, 0.3);
        assert!(s.contains("+300"));
    }
}
