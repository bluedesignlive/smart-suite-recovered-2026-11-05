//! # eiga-core — the engine behind **Eiga (映画)**, the Smart suite's video player.
//!
//! * [`subs`]   SRT / WebVTT / ASS-SSA parsing, encoding detection, fast "what is on screen now" lookup.
//! * [`player`] pure playback state machine (seek, speed, A-B loop, chapters, frame-step), playlist and key map.
//! * [`source`] ffprobe parsing, a procedural test-card source (works everywhere, incl. WebAssembly) and — on native
//!   targets — an `ffmpeg` pipe source that decodes any format ffmpeg knows.
//!
//! Nothing in this crate draws, opens windows or reads clocks, so every behaviour is unit-tested.
pub mod player;
pub mod source;
pub mod subs;

pub use player::{Cmd, Event, LoopMode, Player, Playlist, Repeat};
pub use source::{Frame, MediaInfo};
pub use subs::{Cue, SubTrack};

/// `83.4` → `1:23`, `3723` → `1:02:03`.
pub fn fmt_time(secs: f64) -> String {
    if !secs.is_finite() || secs < 0.0 {
        return "0:00".into();
    }
    let s = secs as u64;
    let (h, m, s) = (s / 3600, (s / 60) % 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

/// Like [`fmt_time`] with milliseconds, used for sync offsets and the stats overlay.
pub fn fmt_time_ms(secs: f64) -> String {
    if !secs.is_finite() || secs < 0.0 {
        return "0:00.000".into();
    }
    let ms = (secs * 1000.0).round() as u64;
    let (h, m, s, ms) = (ms / 3_600_000, (ms / 60_000) % 60, (ms / 1000) % 60, ms % 1000);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}.{ms:03}")
    } else {
        format!("{m}:{s:02}.{ms:03}")
    }
}

/// Parses `90`, `1:30`, `1:02:03.5`. Used by "jump to time".
pub fn parse_time(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut total = 0.0;
    for part in s.split(':') {
        let v: f64 = part.trim().parse().ok()?;
        if !v.is_finite() || v < 0.0 {
            return None;
        }
        total = total * 60.0 + v;
    }
    if s.split(':').count() > 3 {
        return None;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_formatting() {
        assert_eq!(fmt_time(0.0), "0:00");
        assert_eq!(fmt_time(83.9), "1:23");
        assert_eq!(fmt_time(3723.0), "1:02:03");
        assert_eq!(fmt_time(f64::NAN), "0:00");
        assert_eq!(fmt_time(-4.0), "0:00");
        assert_eq!(fmt_time_ms(3723.456), "1:02:03.456");
        assert_eq!(fmt_time_ms(5.5), "0:05.500");
    }

    #[test]
    fn time_parsing() {
        assert_eq!(parse_time("90"), Some(90.0));
        assert_eq!(parse_time("1:30"), Some(90.0));
        assert_eq!(parse_time("1:02:03.5"), Some(3723.5));
        assert_eq!(parse_time(" 0:05 "), Some(5.0));
        assert_eq!(parse_time(""), None);
        assert_eq!(parse_time("a:b"), None);
        assert_eq!(parse_time("-3"), None);
        assert_eq!(parse_time("1:2:3:4"), None);
        assert_eq!(parse_time("inf"), None);
    }
}
