//! The Smart sound suite — synthesised in Rust at first use (no samples shipped, zero cost), played through WebAudio.
//! One glassy-bell timbre in D-major pentatonic so every sound belongs to the same family: rising = opening / success,
//! falling = closing / leaving, low + dissonant = error. Users learn the OS by ear.
//! Also hosts the speech layer (system TTS today; the `speak` seam is where a WASM neural voice drops in).
use std::collections::HashMap;
use std::f32::consts::TAU;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Snd {
    Click,
    Tick,
    Menu,
    Toggle,
    Open,
    Close,
    Minimize,
    Restore,
    Snap,
    Pick,
    Drop,
    Copy,
    Paste,
    Error,
    Warn,
    Success,
    Notify,
    Mount,
    Unmount,
    Trash,
    Empty,
    Boot,
    Listen,
    Think,
    Reply,
    Shutter,
    RecStart,
    RecStop,
    XferStart,
    XferDone,
    Unzip,
    UnzipDone,
    Disk,
    Volume,
}

const SR: u32 = 32_000;

struct Synth {
    buf: Vec<f32>,
    rng: u32,
}

impl Synth {
    fn new(secs: f32) -> Self {
        Self { buf: vec![0.0; (secs * SR as f32) as usize], rng: 0x1234_5679 }
    }
    fn noise(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        (self.rng as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
    fn add(&mut self, at: f32, dur: f32, mut f: impl FnMut(f32) -> f32) {
        let n0 = (at * SR as f32) as usize;
        let n = (dur * SR as f32) as usize;
        for i in 0..n {
            let idx = n0 + i;
            if idx >= self.buf.len() {
                break;
            }
            self.buf[idx] += f(i as f32 / SR as f32);
        }
    }
    /// Glassy bell: sine + two fast-decaying partials + a hair of detune.
    fn bell(&mut self, at: f32, f: f32, dur: f32, amp: f32) {
        self.add(at, dur, |t| {
            let env = (t * 500.0).min(1.0) * (-t * (5.2 / dur)).exp();
            let s = (t * f * TAU).sin() + 0.34 * (t * f * 2.01 * TAU).sin() * (-t * 14.0).exp() + 0.17 * (t * f * 3.99 * TAU).sin() * (-t * 24.0).exp() + 0.10 * (t * f * 1.004 * TAU).sin();
            s * env * amp * 0.5
        });
    }
    /// Slow swell used for the boot chord.
    fn pad(&mut self, at: f32, f: f32, dur: f32, amp: f32, attack: f32) {
        self.add(at, dur, |t| {
            let a = (t / attack).clamp(0.0, 1.0);
            let a = a * a * (3.0 - 2.0 * a);
            let tail = ((dur - t) / (dur * 0.45)).clamp(0.0, 1.0);
            let s = (t * f * TAU).sin() * 0.6 + (t * f * 1.003 * TAU).sin() * 0.3 + (t * f * 2.0 * TAU + (t * 0.8).sin()).sin() * 0.12;
            s * a * tail * amp * 0.5
        });
    }
    fn thud(&mut self, at: f32, f0: f32, dur: f32, amp: f32) {
        let mut ph = 0.0f32;
        self.add(at, dur, |t| {
            let f = f0 * (0.55 + 0.45 * (-t * 22.0).exp());
            ph += f / SR as f32;
            (ph * TAU).sin() * (t * 800.0).min(1.0) * (-t * (4.5 / dur)).exp() * amp
        });
    }
    fn sweep(&mut self, at: f32, f0: f32, f1: f32, dur: f32, amp: f32) {
        let mut ph = 0.0f32;
        self.add(at, dur, |t| {
            let k = (t / dur).clamp(0.0, 1.0);
            let f = f0 + (f1 - f0) * k * k * (3.0 - 2.0 * k);
            ph += f / SR as f32;
            let env = (t * 300.0).min(1.0) * (1.0 - k).powf(0.7);
            (ph * TAU).sin() * env * amp * 0.5
        });
    }
    /// Filtered noise burst; `rise` swells in, otherwise it decays.
    fn hiss(&mut self, at: f32, dur: f32, amp: f32, lp: f32, rise: bool) {
        let mut y = 0.0f32;
        let mut rng = self.rng ^ (at * 1000.0) as u32;
        self.add(at, dur, |t| {
            rng ^= rng << 13;
            rng ^= rng >> 17;
            rng ^= rng << 5;
            let n = (rng as f32 / u32::MAX as f32) * 2.0 - 1.0;
            y += (n - y) * lp.clamp(0.02, 1.0);
            let k = (t / dur).clamp(0.0, 1.0);
            let env = if rise { k * (1.0 - k * 0.3) } else { (1.0 - k).powi(2) };
            y * env * amp
        });
        let _ = self.noise();
    }
}

fn render(s: Snd) -> Vec<f32> {
    let (d5, e5, fs5, a5, d6) = (587.33, 659.25, 739.99, 880.0, 1174.66);
    let mut y = Synth::new(match s {
        Snd::Boot => 3.8,
        Snd::Unzip => 0.8,
        Snd::Empty | Snd::XferDone | Snd::UnzipDone => 1.6,
        Snd::Notify | Snd::Success => 1.1,
        Snd::Error | Snd::Mount | Snd::Unmount | Snd::Trash => 0.6,
        _ => 0.45,
    });
    match s {
        Snd::Click => {
            y.bell(0.0, 1568.0, 0.07, 0.35);
            y.thud(0.0, 220.0, 0.05, 0.18);
        }
        Snd::Tick => y.bell(0.0, 2093.0, 0.045, 0.22),
        Snd::Menu => y.bell(0.0, 1318.5, 0.09, 0.22),
        Snd::Volume => y.bell(0.0, 1046.5, 0.06, 0.24),
        Snd::Toggle => {
            y.bell(0.0, 987.77, 0.09, 0.28);
            y.bell(0.045, 1318.5, 0.12, 0.28);
        }
        Snd::Open => {
            y.sweep(0.0, 420.0, 760.0, 0.16, 0.30);
            y.bell(0.05, a5, 0.26, 0.28);
        }
        Snd::Close => {
            y.sweep(0.0, 760.0, 380.0, 0.14, 0.30);
            y.thud(0.02, 160.0, 0.12, 0.26);
        }
        Snd::Minimize => {
            y.sweep(0.0, 640.0, 300.0, 0.20, 0.26);
            y.hiss(0.0, 0.15, 0.10, 0.30, false);
        }
        Snd::Restore => {
            y.sweep(0.0, 300.0, 640.0, 0.18, 0.26);
            y.hiss(0.0, 0.14, 0.10, 0.30, true);
        }
        Snd::Snap => {
            y.thud(0.0, 150.0, 0.12, 0.55);
            y.bell(0.0, 1046.5, 0.12, 0.24);
        }
        Snd::Pick => y.bell(0.0, d6, 0.10, 0.28),
        Snd::Drop => {
            y.thud(0.0, 120.0, 0.15, 0.55);
            y.bell(0.02, e5, 0.18, 0.26);
            y.hiss(0.0, 0.06, 0.16, 0.5, false);
        }
        Snd::Copy => {
            y.bell(0.0, a5, 0.14, 0.30);
            y.bell(0.06, d6, 0.20, 0.30);
        }
        Snd::Paste => {
            y.bell(0.0, d6, 0.14, 0.30);
            y.bell(0.06, a5, 0.20, 0.30);
        }
        Snd::Success => {
            for (i, f) in [d5, fs5, a5].iter().enumerate() {
                y.bell(i as f32 * 0.07, *f, 0.5, 0.34);
            }
        }
        Snd::Error => {
            y.bell(0.0, 233.1, 0.36, 0.5);
            y.bell(0.09, 220.0, 0.42, 0.5);
            y.thud(0.0, 90.0, 0.2, 0.4);
        }
        Snd::Warn => {
            y.bell(0.0, 523.25, 0.2, 0.4);
            y.bell(0.12, 523.25, 0.26, 0.4);
        }
        Snd::Notify => {
            y.bell(0.0, a5, 0.32, 0.34);
            y.bell(0.11, d6, 0.62, 0.38);
        }
        Snd::Mount => {
            y.sweep(0.0, 330.0, 990.0, 0.22, 0.28);
            y.thud(0.19, 100.0, 0.1, 0.3);
            y.bell(0.2, 1318.5, 0.34, 0.34);
        }
        Snd::Unmount => {
            y.sweep(0.0, 990.0, 330.0, 0.22, 0.26);
            y.bell(0.0, a5, 0.12, 0.2);
        }
        Snd::Trash => {
            y.hiss(0.0, 0.18, 0.36, 0.18, false);
            y.thud(0.05, 90.0, 0.18, 0.5);
            y.sweep(0.0, 500.0, 200.0, 0.15, 0.16);
        }
        Snd::Empty => {
            y.hiss(0.0, 0.5, 0.40, 0.3, true);
            y.thud(0.45, 70.0, 0.3, 0.6);
            y.bell(0.5, 523.25, 0.7, 0.3);
            y.bell(0.62, 783.99, 0.8, 0.26);
        }
        Snd::Boot => {
            for (f, at) in [(146.83, 0.0), (220.0, 0.1), (293.66, 0.2), (369.99, 0.35), (440.0, 0.5), (587.33, 0.7)] {
                y.pad(at, f, 3.3 - at, 0.42, 0.9);
            }
            y.sweep(0.0, 80.0, 160.0, 1.2, 0.3);
            y.bell(1.3, d6, 1.7, 0.26);
            y.bell(1.55, 1479.98, 1.6, 0.22);
            y.bell(1.8, 1760.0, 1.6, 0.2);
        }
        Snd::Listen => {
            y.sweep(0.0, 520.0, 880.0, 0.12, 0.26);
            y.bell(0.1, 1318.5, 0.26, 0.3);
        }
        Snd::Think => {
            y.bell(0.0, e5, 0.15, 0.2);
            y.bell(0.09, a5, 0.2, 0.2);
        }
        Snd::Reply => {
            y.bell(0.0, d6, 0.2, 0.3);
            y.bell(0.08, a5, 0.3, 0.3);
        }
        Snd::Shutter => {
            y.hiss(0.0, 0.02, 0.6, 0.8, false);
            y.hiss(0.07, 0.03, 0.5, 0.6, false);
            y.thud(0.0, 300.0, 0.05, 0.3);
            y.thud(0.07, 200.0, 0.06, 0.3);
        }
        Snd::RecStart => {
            y.bell(0.0, a5, 0.12, 0.35);
            y.bell(0.12, d6, 0.3, 0.4);
        }
        Snd::RecStop => {
            y.bell(0.0, d6, 0.12, 0.35);
            y.bell(0.12, a5, 0.3, 0.4);
        }
        Snd::XferStart => {
            y.sweep(0.0, 260.0, 900.0, 0.35, 0.26);
            y.hiss(0.0, 0.35, 0.12, 0.4, true);
            y.bell(0.33, d6, 0.26, 0.3);
        }
        Snd::XferDone => {
            y.thud(0.0, 100.0, 0.2, 0.4);
            for (i, f) in [d5, fs5, a5, d6].iter().enumerate() {
                y.bell(0.04 + i as f32 * 0.08, *f, 0.9, 0.34);
            }
            y.pad(0.0, 440.0, 0.9, 0.2, 0.15);
        }
        Snd::Unzip => {
            for k in 0..24 {
                let at = k as f32 * 0.03;
                y.hiss(at, 0.014, 0.4, 0.6, false);
                y.bell(at, 900.0 + k as f32 * 38.0, 0.03, 0.07);
            }
        }
        Snd::UnzipDone => {
            y.hiss(0.0, 0.08, 0.3, 0.5, false);
            y.thud(0.0, 80.0, 0.2, 0.5);
            y.bell(0.05, a5, 0.5, 0.34);
            y.bell(0.13, d6, 0.6, 0.34);
            y.bell(0.21, 1479.98, 0.9, 0.34);
        }
        Snd::Disk => y.hiss(0.0, 0.012, 0.3, 0.7, false),
    }
    let peak = y.buf.iter().fold(0.0f32, |m, v| m.max(v.abs())).max(0.001);
    let g = (0.82 / peak).min(1.6);
    let n = y.buf.len();
    let fade = (SR as f32 * 0.02) as usize;
    for (i, v) in y.buf.iter_mut().enumerate() {
        *v *= g;
        if i + fade > n {
            *v *= (n - i) as f32 / fade as f32;
        }
    }
    y.buf
}

pub struct Sound {
    pub on: bool,
    pub vol: f32,
    #[cfg(target_arch = "wasm32")]
    eng: web::Engine,
    pub log: Vec<(f64, Snd)>,
}

impl Default for Sound {
    fn default() -> Self {
        Self::new()
    }
}

impl Sound {
    pub fn new() -> Self {
        Self {
            on: true,
            vol: 0.7,
            #[cfg(target_arch = "wasm32")]
            eng: web::Engine { ctx: None, cache: HashMap::new(), last: HashMap::new() },
            log: vec![],
        }
    }
    pub fn play(&mut self, s: Snd) {
        self.play_vol(s, 1.0);
    }
    pub fn play_vol(&mut self, s: Snd, v: f32) {
        if !self.on || self.vol < 0.01 {
            return;
        }
        let _ = (&s, v);
        #[cfg(target_arch = "wasm32")]
        self.eng.play(s, (self.vol * v).clamp(0.0, 1.0));
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use web_sys::{AudioBuffer, AudioContext};
    pub struct Engine {
        pub ctx: Option<AudioContext>,
        pub cache: HashMap<Snd, AudioBuffer>,
        pub last: HashMap<Snd, f64>,
    }
    impl Engine {
        pub fn play(&mut self, s: Snd, vol: f32) {
            let now = js_sys::Date::now();
            if let Some(t) = self.last.get(&s) {
                if now - t < 35.0 {
                    return;
                }
            }
            self.last.insert(s, now);
            if self.ctx.is_none() {
                self.ctx = AudioContext::new().ok();
            }
            let Some(ctx) = &self.ctx else { return };
            let _ = ctx.resume();
            let buf = if let Some(b) = self.cache.get(&s) {
                b.clone()
            } else {
                let mut data = render(s);
                let Ok(b) = ctx.create_buffer(1, data.len() as u32, SR as f32) else { return };
                let _ = b.copy_to_channel(&mut data, 0);
                self.cache.insert(s, b.clone());
                b
            };
            let Ok(src) = ctx.create_buffer_source() else { return };
            src.set_buffer(Some(&buf));
            let Ok(g) = ctx.create_gain() else { return };
            g.gain().set_value(vol);
            let _ = src.connect_with_audio_node(&g);
            let _ = g.connect_with_audio_node(&ctx.destination());
            let _ = src.start();
        }
    }
}

// ───────────────────────────── speech ─────────────────────────────

/// Speak text with the best voice the system offers (prefers neural / natural voices). The same seam accepts a
/// WASM neural TTS engine later — callers only ever say `speak(text)`.
pub fn speak(text: &str, rate: f32, volume: f32) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let Some(win) = web_sys::window() else { return };
        let Ok(ss) = win.speech_synthesis() else { return };
        let Ok(u) = web_sys::SpeechSynthesisUtterance::new_with_text(text) else { return };
        u.set_rate(rate);
        u.set_volume(volume);
        let mut best: Option<(i32, web_sys::SpeechSynthesisVoice)> = None;
        for v in ss.get_voices().iter() {
            let Ok(v) = v.dyn_into::<web_sys::SpeechSynthesisVoice>() else { continue };
            let (n, l) = (v.name().to_lowercase(), v.lang().to_lowercase());
            if !l.starts_with("en") {
                continue;
            }
            let mut sc = 0;
            for (k, w) in [("natural", 40), ("neural", 40), ("premium", 30), ("enhanced", 25), ("google", 18), ("samantha", 16), ("aria", 15), ("jenny", 15), ("zira", 8)] {
                if n.contains(k) {
                    sc += w;
                }
            }
            if l.contains("za") || l.contains("ng") || l.contains("ke") {
                sc += 6;
            }
            if best.as_ref().map(|b| sc > b.0).unwrap_or(true) {
                best = Some((sc, v));
            }
        }
        if let Some((_, v)) = best {
            u.set_voice(Some(&v));
        }
        ss.speak(&u);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (text, rate, volume);
}

pub fn stop_speaking() {
    #[cfg(target_arch = "wasm32")]
    if let Some(ss) = web_sys::window().and_then(|w| w.speech_synthesis().ok()) {
        ss.cancel();
    }
}

pub fn is_speaking() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        return web_sys::window().and_then(|w| w.speech_synthesis().ok()).map(|s| s.speaking()).unwrap_or(false);
    }
    #[allow(unreachable_code)]
    false
}
