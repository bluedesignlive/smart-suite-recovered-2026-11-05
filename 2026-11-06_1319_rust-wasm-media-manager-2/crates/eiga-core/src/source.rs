//! Media metadata (ffprobe), frame sources (procedural test card everywhere; ffmpeg pipe on native).
use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VideoStream {
    pub index: usize,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bitrate: Option<u64>,
    pub pix_fmt: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AudioStream {
    pub index: usize,
    pub codec: String,
    pub channels: u32,
    pub sample_rate: u32,
    pub lang: String,
    pub title: String,
    pub default: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubStream {
    pub index: usize,
    pub codec: String,
    pub lang: String,
    pub title: String,
    pub default: bool,
    pub forced: bool,
}

impl SubStream {
    /// Text subtitles can be extracted and rendered; bitmap ones (PGS, VobSub) cannot.
    pub fn is_text(&self) -> bool {
        matches!(self.codec.as_str(), "subrip" | "srt" | "ass" | "ssa" | "webvtt" | "mov_text" | "text")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaInfo {
    pub title: String,
    pub container: String,
    pub duration: f64,
    pub size: u64,
    pub bitrate: u64,
    pub video: Vec<VideoStream>,
    pub audio: Vec<AudioStream>,
    pub subs: Vec<SubStream>,
    pub chapters: Vec<(f64, String)>,
}

impl MediaInfo {
    pub fn primary_video(&self) -> Option<&VideoStream> {
        self.video.first()
    }
    pub fn size_wh(&self) -> (u32, u32) {
        self.primary_video().map(|v| (v.width, v.height)).unwrap_or((0, 0))
    }
    pub fn fps(&self) -> f64 {
        self.primary_video().map(|v| v.fps).filter(|f| *f > 1.0).unwrap_or(25.0)
    }
}

fn parse_rate(s: &str) -> f64 {
    match s.split_once('/') {
        Some((n, d)) => {
            let (n, d): (f64, f64) = (n.parse().unwrap_or(0.0), d.parse().unwrap_or(0.0));
            if d > 0.0 {
                n / d
            } else {
                0.0
            }
        }
        None => s.parse().unwrap_or(0.0),
    }
}

fn s(v: &Value, k: &str) -> String {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string()
}
fn num(v: &Value, k: &str) -> f64 {
    match v.get(k) {
        Some(Value::String(t)) => t.parse().unwrap_or(0.0),
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        _ => 0.0,
    }
}

/// Parses `ffprobe -print_format json -show_format -show_streams -show_chapters` output.
pub fn parse_ffprobe(json: &str) -> Result<MediaInfo, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("unreadable ffprobe output: {e}"))?;
    let mut m = MediaInfo::default();
    let f = v.get("format").cloned().unwrap_or(Value::Null);
    m.container = s(&f, "format_name").split(',').next().unwrap_or("").to_string();
    m.duration = num(&f, "duration");
    m.size = num(&f, "size") as u64;
    m.bitrate = num(&f, "bit_rate") as u64;
    m.title = f.get("tags").map(|t| s(t, "title")).unwrap_or_default();
    for st in v.get("streams").and_then(|x| x.as_array()).cloned().unwrap_or_default() {
        let index = num(&st, "index") as usize;
        let tags = st.get("tags").cloned().unwrap_or(Value::Null);
        let disp = st.get("disposition").cloned().unwrap_or(Value::Null);
        let flag = |k: &str| num(&disp, k) as i64 == 1;
        match s(&st, "codec_type").as_str() {
            "video" if !flag("attached_pic") => {
                let mut fps = parse_rate(&s(&st, "avg_frame_rate"));
                if fps <= 0.0 {
                    fps = parse_rate(&s(&st, "r_frame_rate"));
                }
                if m.duration <= 0.0 {
                    m.duration = num(&st, "duration");
                }
                m.video.push(VideoStream { index, codec: s(&st, "codec_name"), width: num(&st, "width") as u32, height: num(&st, "height") as u32, fps, bitrate: Some(num(&st, "bit_rate") as u64).filter(|b| *b > 0), pix_fmt: s(&st, "pix_fmt") });
            }
            "audio" => m.audio.push(AudioStream { index, codec: s(&st, "codec_name"), channels: num(&st, "channels") as u32, sample_rate: num(&st, "sample_rate") as u32, lang: s(&tags, "language"), title: s(&tags, "title"), default: flag("default") }),
            "subtitle" => m.subs.push(SubStream { index, codec: s(&st, "codec_name"), lang: s(&tags, "language"), title: s(&tags, "title"), default: flag("default"), forced: flag("forced") }),
            _ => {}
        }
    }
    for c in v.get("chapters").and_then(|x| x.as_array()).cloned().unwrap_or_default() {
        let t = c.get("tags").map(|t| s(t, "title")).unwrap_or_default();
        m.chapters.push((num(&c, "start_time"), t));
    }
    if !m.duration.is_finite() || m.duration < 0.0 {
        m.duration = 0.0;
    }
    if m.video.is_empty() && m.audio.is_empty() {
        return Err("no playable streams".into());
    }
    Ok(m)
}

// ─────────────────────────────────────────── frames ───────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Frame {
    pub w: usize,
    pub h: usize,
    pub rgba: Vec<u8>,
    pub pts: f64,
}

pub trait FrameSource: Send {
    fn info(&self) -> &MediaInfo;
    fn seek(&mut self, t: f64);
    /// The newest frame with `pts <= clock` that has not been returned yet.
    fn poll(&mut self, clock: f64) -> Option<Frame>;
    fn name(&self) -> &'static str;
    /// No more frames will ever arrive (end of file or decoder died).
    fn exhausted(&self) -> bool {
        false
    }
}

/// Even output dimensions that fit inside `max`, preserving aspect and never upscaling.
pub fn fit_size(sw: u32, sh: u32, max_w: u32, max_h: u32) -> (u32, u32) {
    if sw == 0 || sh == 0 {
        return (640, 360);
    }
    let k = (max_w as f64 / sw as f64).min(max_h as f64 / sh as f64).min(1.0);
    let ev = |x: f64| ((x.round() as u32).max(2)) & !1;
    (ev(sw as f64 * k), ev(sh as f64 * k))
}

// 5×7 digit font for the burnt-in timecode
const GLYPHS: [[u8; 7]; 12] = [
    [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
    [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
    [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
    [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
    [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
    [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
    [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
    [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
    [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
    [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
    [0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
];

fn put(buf: &mut [u8], w: usize, x: usize, y: usize, c: [u8; 3]) {
    let i = (y * w + x) * 4;
    if i + 3 < buf.len() {
        buf[i] = c[0];
        buf[i + 1] = c[1];
        buf[i + 2] = c[2];
        buf[i + 3] = 255;
    }
}

fn draw_text(buf: &mut [u8], w: usize, h: usize, x0: usize, y0: usize, scale: usize, text: &str, c: [u8; 3]) {
    let mut x = x0;
    for ch in text.chars() {
        let g = match ch {
            '0'..='9' => GLYPHS[ch as usize - '0' as usize],
            ':' => GLYPHS[10],
            _ => GLYPHS[11],
        };
        for (row, bits) in g.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let (px, py) = (x + col * scale + dx, y0 + row * scale + dy);
                            if px < w && py < h {
                                put(buf, w, px, py, c);
                            }
                        }
                    }
                }
            }
        }
        x += 6 * scale;
    }
}

/// SMPTE-style bars, a bouncing ball and a burnt-in timecode. Deterministic: same `t` → same pixels.
pub fn render_testcard(w: usize, h: usize, t: f64, fps: f64) -> Frame {
    let (w, h) = (w.max(16), h.max(16));
    let mut rgba = vec![0u8; w * h * 4];
    const BARS: [[u8; 3]; 7] = [[235, 235, 235], [235, 235, 16], [16, 235, 235], [16, 235, 16], [235, 16, 235], [235, 16, 16], [16, 16, 235]];
    let bars_h = h * 2 / 3;
    for y in 0..h {
        for x in 0..w {
            let c = if y < bars_h {
                BARS[(x * 7 / w).min(6)]
            } else if y < bars_h + h / 12 {
                let g = (x * 255 / w) as u8;
                [g, g, g]
            } else {
                [18, 22, 34]
            };
            put(&mut rgba, w, x, y, c);
        }
    }
    // dim the bars a little so overlaid UI stays legible
    for px in rgba[..bars_h * w * 4].chunks_exact_mut(4) {
        for c in &mut px[..3] {
            *c = (*c as u16 * 7 / 10) as u8;
        }
    }
    // ball
    let phase = (t / 6.0 * std::f64::consts::TAU).sin();
    let bx = (w as f64 * (0.5 + 0.42 * phase)) as i64;
    let by = (bars_h as f64 + (h - bars_h) as f64 * 0.62) as i64;
    let r = (h / 16).max(3) as i64;
    for y in (by - r).max(0)..(by + r).min(h as i64) {
        for x in (bx - r).max(0)..(bx + r).min(w as i64) {
            if (x - bx).pow(2) + (y - by).pow(2) <= r * r {
                put(&mut rgba, w, x as usize, y as usize, [255, 214, 96]);
            }
        }
    }
    let tt = t.max(0.0);
    let frame_no = (tt * fps.max(1.0)) as u64;
    let tc = format!("{:02}:{:02}:{:02}", (tt as u64) / 3600, ((tt as u64) / 60) % 60, (tt as u64) % 60);
    let scale = (w / 90).max(1);
    draw_text(&mut rgba, w, h, w / 16, h * 2 / 3 + h / 12 + scale * 2, scale, &tc, [236, 240, 250]);
    draw_text(&mut rgba, w, h, w / 16, h - 9 * scale - 4, scale.max(2) / 2 + 1, &format!("{frame_no:06}"), [140, 156, 190]);
    Frame { w, h, rgba, pts: frame_no as f64 / fps.max(1.0) }
}

/// A synthetic "video" with a known duration. Powers the web preview and every UI test.
pub struct TestCard {
    info: MediaInfo,
    w: usize,
    h: usize,
    last: Option<u64>,
    fps: f64,
}

impl TestCard {
    pub fn new(title: &str, duration: f64) -> TestCard {
        let (w, h, fps) = (480usize, 270usize, 30.0);
        let info = MediaInfo { title: title.into(), container: "testcard".into(), duration, video: vec![VideoStream { index: 0, codec: "procedural".into(), width: w as u32, height: h as u32, fps, bitrate: None, pix_fmt: "rgba".into() }], chapters: vec![(0.0, "Intro".into()), (duration * 0.35, "Middle".into()), (duration * 0.7, "Finale".into())], ..Default::default() };
        TestCard { info, w, h, last: None, fps }
    }
}

impl FrameSource for TestCard {
    fn info(&self) -> &MediaInfo {
        &self.info
    }
    fn seek(&mut self, _t: f64) {
        self.last = None;
    }
    fn poll(&mut self, clock: f64) -> Option<Frame> {
        let idx = (clock.max(0.0) * self.fps) as u64;
        if self.last == Some(idx) {
            return None;
        }
        self.last = Some(idx);
        Some(render_testcard(self.w, self.h, idx as f64 / self.fps, self.fps))
    }
    fn name(&self) -> &'static str {
        "test card"
    }
}

// ─────────────────────────────────────────── ffmpeg (native) ───────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
pub mod ffmpeg {
    use super::*;
    use std::io::Read;
    use std::process::{Child, Command, Stdio};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::{sync_channel, Receiver};
    use std::sync::Arc;

    pub fn available() -> bool {
        Command::new("ffmpeg").arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false)
    }

    pub fn probe(path: &str) -> Result<MediaInfo, String> {
        let out = Command::new("ffprobe")
            .args(["-v", "error", "-print_format", "json", "-show_format", "-show_streams", "-show_chapters", "--", path])
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("ffprobe is not installed ({e})"))?;
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(format!("not a playable media file: {}", err.lines().next().unwrap_or("unknown error")));
        }
        let mut info = parse_ffprobe(&String::from_utf8_lossy(&out.stdout))?;
        if info.title.is_empty() {
            info.title = path.rsplit('/').next().unwrap_or(path).to_string();
        }
        Ok(info)
    }

    /// `atempo` only accepts 0.5–2.0 per instance, so chain them.
    pub fn atempo_chain(speed: f64) -> String {
        let mut s = speed.clamp(0.25, 8.0);
        let mut parts = vec![];
        while s > 2.0 {
            parts.push("atempo=2.0".to_string());
            s /= 2.0;
        }
        while s < 0.5 {
            parts.push("atempo=0.5".to_string());
            s /= 0.5;
        }
        parts.push(format!("atempo={s:.4}"));
        parts.join(",")
    }

    pub fn video_args(path: &str, start: f64, w: u32, h: u32) -> Vec<String> {
        let mut a: Vec<String> = ["-hide_banner", "-loglevel", "error", "-nostdin", "-hwaccel", "auto"].iter().map(|s| s.to_string()).collect();
        a.extend(["-ss".into(), format!("{:.3}", start.max(0.0)), "-i".into(), path.into(), "-an".into(), "-sn".into(), "-vf".into(), format!("scale={w}:{h}:flags=fast_bilinear"), "-pix_fmt".into(), "rgba".into(), "-f".into(), "rawvideo".into(), "-".into()]);
        a
    }

    /// Interleaved stereo f32le at 48 kHz.
    pub fn audio_args(path: &str, start: f64, audio_n: usize, speed: f64) -> Vec<String> {
        let mut a: Vec<String> = ["-hide_banner", "-loglevel", "error", "-nostdin"].iter().map(|s| s.to_string()).collect();
        a.extend(["-ss".into(), format!("{:.3}", start.max(0.0)), "-i".into(), path.into(), "-map".into(), format!("0:a:{audio_n}"), "-vn".into(), "-sn".into()]);
        if (speed - 1.0).abs() > 1e-3 {
            a.extend(["-af".into(), atempo_chain(speed)]);
        }
        a.extend(["-ac".into(), "2".into(), "-ar".into(), "48000".into(), "-f".into(), "f32le".into(), "-".into()]);
        a
    }

    pub fn sub_args(path: &str, sub_n: usize) -> Vec<String> {
        ["-hide_banner", "-loglevel", "error", "-nostdin", "-i", path, "-map", &format!("0:s:{sub_n}"), "-f", "srt", "-"].iter().map(|s| s.to_string()).collect()
    }

    /// Extracts an embedded text subtitle track as SRT.
    pub fn extract_sub(path: &str, sub_n: usize) -> Result<String, String> {
        let out = Command::new("ffmpeg").args(sub_args(path, sub_n)).stdin(Stdio::null()).output().map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err("could not extract that subtitle track".into());
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    pub struct FfmpegSource {
        info: MediaInfo,
        path: String,
        child: Option<Child>,
        rx: Option<Receiver<Frame>>,
        pending: Option<Frame>,
        w: u32,
        h: u32,
        done: Arc<AtomicBool>,
    }

    impl FfmpegSource {
        pub fn open(path: &str, info: MediaInfo, max_w: u32, max_h: u32) -> Result<FfmpegSource, String> {
            let (sw, sh) = info.size_wh();
            if sw == 0 || sh == 0 {
                return Err("this file has no video stream".into());
            }
            let (w, h) = fit_size(sw, sh, max_w, max_h);
            let mut s = FfmpegSource { info, path: path.into(), child: None, rx: None, pending: None, w, h, done: Arc::new(AtomicBool::new(false)) };
            s.spawn(0.0)?;
            Ok(s)
        }

        fn kill(&mut self) {
            self.rx = None; // unblocks the reader thread's send()
            if let Some(mut c) = self.child.take() {
                let _ = c.kill();
                let _ = c.wait();
            }
            self.pending = None;
        }

        fn spawn(&mut self, start: f64) -> Result<(), String> {
            self.kill();
            let mut child = Command::new("ffmpeg").args(video_args(&self.path, start, self.w, self.h)).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().map_err(|e| format!("ffmpeg is not installed ({e})"))?;
            let mut out = child.stdout.take().ok_or("no pipe")?;
            let (tx, rx) = sync_channel::<Frame>(4);
            let (w, h, fps) = (self.w as usize, self.h as usize, self.info.fps());
            let done = Arc::new(AtomicBool::new(false));
            self.done = done.clone();
            std::thread::spawn(move || {
                let mut n = 0u64;
                loop {
                    let mut buf = vec![0u8; w * h * 4];
                    if out.read_exact(&mut buf).is_err() {
                        break;
                    }
                    let f = Frame { w, h, rgba: buf, pts: start + n as f64 / fps };
                    n += 1;
                    if tx.send(f).is_err() {
                        return;
                    }
                }
                done.store(true, Ordering::SeqCst);
            });
            self.child = Some(child);
            self.rx = Some(rx);
            Ok(())
        }
    }

    impl FrameSource for FfmpegSource {
        fn info(&self) -> &MediaInfo {
            &self.info
        }
        fn seek(&mut self, t: f64) {
            let _ = self.spawn(t);
        }
        fn poll(&mut self, clock: f64) -> Option<Frame> {
            let mut best = None;
            if let Some(p) = self.pending.take() {
                if p.pts <= clock {
                    best = Some(p);
                } else {
                    self.pending = Some(p);
                    return None;
                }
            }
            if let Some(rx) = &self.rx {
                while let Ok(f) = rx.try_recv() {
                    if f.pts <= clock {
                        best = Some(f);
                    } else {
                        self.pending = Some(f);
                        break;
                    }
                }
            }
            best
        }
        fn name(&self) -> &'static str {
            "ffmpeg"
        }
        fn exhausted(&self) -> bool {
            self.done.load(Ordering::SeqCst) && self.pending.is_none() && self.rx.as_ref().map(|r| r.try_recv().is_err()).unwrap_or(true)
        }
    }

    impl Drop for FfmpegSource {
        fn drop(&mut self) {
            self.kill();
        }
    }

    /// Streams decoded audio in ~100 ms chunks. The host appends each chunk to its output device.
    pub struct AudioPipe {
        child: Child,
        rx: Receiver<Vec<f32>>,
    }

    impl AudioPipe {
        pub fn open(path: &str, start: f64, audio_n: usize, speed: f64) -> Result<AudioPipe, String> {
            let mut child = Command::new("ffmpeg").args(audio_args(path, start, audio_n, speed)).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().map_err(|e| e.to_string())?;
            let mut out = child.stdout.take().ok_or("no pipe")?;
            let (tx, rx) = sync_channel::<Vec<f32>>(24);
            std::thread::spawn(move || {
                let mut raw = vec![0u8; 48_000 / 10 * 2 * 4];
                while out.read_exact(&mut raw).is_ok() {
                    let v: Vec<f32> = raw.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
                    if tx.send(v).is_err() {
                        return;
                    }
                }
            });
            Ok(AudioPipe { child, rx })
        }
        pub fn try_chunk(&self) -> Option<Vec<f32>> {
            self.rx.try_recv().ok()
        }
    }

    impl Drop for AudioPipe {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBE: &str = r#"{"streams":[
      {"index":0,"codec_type":"video","codec_name":"h264","width":1920,"height":1080,"avg_frame_rate":"30000/1001","r_frame_rate":"30000/1001","pix_fmt":"yuv420p","bit_rate":"4000000"},
      {"index":1,"codec_type":"audio","codec_name":"aac","channels":2,"sample_rate":"48000","disposition":{"default":1},"tags":{"language":"jpn","title":"Japanese"}},
      {"index":2,"codec_type":"audio","codec_name":"ac3","channels":6,"sample_rate":"48000","tags":{"language":"eng"}},
      {"index":3,"codec_type":"subtitle","codec_name":"subrip","tags":{"language":"eng"},"disposition":{"default":0,"forced":1}},
      {"index":4,"codec_type":"subtitle","codec_name":"hdmv_pgs_subtitle","tags":{"language":"fra"}},
      {"index":5,"codec_type":"video","codec_name":"mjpeg","width":600,"height":600,"avg_frame_rate":"0/0","disposition":{"attached_pic":1}}
    ],"format":{"format_name":"matroska,webm","duration":"5400.250000","size":"2500000000","bit_rate":"3700000","tags":{"title":"Night Ride"}},
    "chapters":[{"start_time":"0.000000","tags":{"title":"Opening"}},{"start_time":"600.5","tags":{}}]}"#;

    #[test]
    fn parses_ffprobe_json() {
        let m = parse_ffprobe(PROBE).unwrap();
        assert_eq!(m.title, "Night Ride");
        assert_eq!(m.container, "matroska");
        assert!((m.duration - 5400.25).abs() < 1e-6);
        assert_eq!(m.video.len(), 1, "cover art is not a video stream");
        assert_eq!(m.size_wh(), (1920, 1080));
        assert!((m.fps() - 29.97).abs() < 0.01);
        assert_eq!(m.audio.len(), 2);
        assert!(m.audio[0].default && m.audio[0].lang == "jpn");
        assert_eq!(m.subs.len(), 2);
        assert!(m.subs[0].is_text() && m.subs[0].forced);
        assert!(!m.subs[1].is_text(), "PGS is bitmap");
        assert_eq!(m.chapters.len(), 2);
        assert_eq!(m.chapters[0].1, "Opening");
        assert!((m.chapters[1].0 - 600.5).abs() < 1e-9);
    }

    #[test]
    fn rejects_bad_probe_output() {
        assert!(parse_ffprobe("").is_err());
        assert!(parse_ffprobe("{}").is_err());
        assert!(parse_ffprobe("[1,2,3]").is_err());
        assert!(parse_ffprobe(r#"{"streams":[{"codec_type":"data"}]}"#).is_err());
        let audio_only = parse_ffprobe(r#"{"streams":[{"index":0,"codec_type":"audio","codec_name":"mp3"}],"format":{"duration":"3.5"}}"#).unwrap();
        assert_eq!(audio_only.size_wh(), (0, 0));
        assert_eq!(audio_only.fps(), 25.0);
    }

    #[test]
    fn fit_keeps_aspect_even_and_never_upscales() {
        assert_eq!(fit_size(1920, 1080, 960, 540), (960, 540));
        assert_eq!(fit_size(320, 240, 960, 540), (320, 240));
        let (w, h) = fit_size(1001, 563, 500, 500);
        assert!(w % 2 == 0 && h % 2 == 0 && w <= 500 && h <= 500);
        assert_eq!(fit_size(0, 0, 100, 100), (640, 360));
        let (w, h) = fit_size(1, 1000, 100, 100);
        assert!(w >= 2 && h <= 100);
    }

    #[test]
    fn testcard_is_deterministic_and_animated() {
        let a = render_testcard(160, 90, 1.0, 30.0);
        let b = render_testcard(160, 90, 1.0, 30.0);
        assert_eq!(a.rgba, b.rgba);
        assert_eq!(a.rgba.len(), 160 * 90 * 4);
        assert!(a.rgba.chunks_exact(4).all(|p| p[3] == 255));
        let c = render_testcard(160, 90, 4.0, 30.0);
        assert_ne!(a.rgba, c.rgba, "ball and timecode move");
        let tiny = render_testcard(1, 1, 0.0, 0.0);
        assert!(tiny.w >= 16 && tiny.rgba.len() == tiny.w * tiny.h * 4, "degenerate sizes are clamped");
    }

    #[test]
    fn testcard_source_returns_each_frame_once() {
        let mut s = TestCard::new("x", 10.0);
        assert!(s.poll(0.0).is_some());
        assert!(s.poll(0.01).is_none(), "same frame index");
        let f = s.poll(0.5).unwrap();
        assert!((f.pts - 0.5).abs() < 1.0 / 30.0 + 1e-9);
        s.seek(0.0);
        assert!(s.poll(0.5).is_some(), "seek invalidates the cache");
        assert_eq!(s.info().chapters.len(), 3);
        assert!(!s.exhausted());
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::super::ffmpeg::*;
        use super::super::*;
        use std::process::{Command, Stdio};

        #[test]
        fn argument_builders() {
            let v = video_args("/a b/c.mkv", 12.5, 640, 360);
            assert!(v.windows(2).any(|w| w == ["-ss", "12.500"]));
            assert!(v.iter().any(|a| a == "scale=640:360:flags=fast_bilinear"));
            assert_eq!(v.last().unwrap(), "-");
            assert!(v.contains(&"/a b/c.mkv".to_string()), "paths are passed as one argument, never through a shell");
            let a = audio_args("x.mkv", -3.0, 1, 1.5);
            assert!(a.windows(2).any(|w| w == ["-map", "0:a:1"]));
            assert!(a.windows(2).any(|w| w == ["-ss", "0.000"]), "negative starts are clamped");
            assert!(a.iter().any(|x| x == "atempo=1.5000"));
            assert!(!audio_args("x", 0.0, 0, 1.0).iter().any(|x| x == "-af"));
            assert_eq!(atempo_chain(4.0), "atempo=2.0,atempo=2.0000");
            assert_eq!(atempo_chain(0.25), "atempo=0.5,atempo=0.5000");
            assert_eq!(sub_args("m.mkv", 2)[sub_args("m.mkv", 2).iter().position(|x| x == "-map").unwrap() + 1], "0:s:2");
        }

        fn have_ffmpeg() -> bool {
            available() && Command::new("ffprobe").arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false)
        }

        fn make_clip(dir: &std::path::Path) -> String {
            let srt = dir.join("s.srt");
            std::fs::write(&srt, "1\n00:00:00,200 --> 00:00:01,000\nHello\n\n2\n00:00:01,200 --> 00:00:01,900\nWorld\n").unwrap();
            let out = dir.join("clip.mkv");
            let st = Command::new("ffmpeg")
                .args(["-y", "-v", "error", "-f", "lavfi", "-i", "testsrc=duration=2:size=160x90:rate=10", "-f", "lavfi", "-i", "sine=frequency=440:duration=2", "-i"])
                .arg(&srt)
                .args(["-c:v", "libx264", "-pix_fmt", "yuv420p", "-c:a", "aac", "-c:s", "srt", "-metadata:s:s:0", "language=eng"])
                .arg(&out)
                .stdin(Stdio::null())
                .status()
                .unwrap();
            assert!(st.success(), "ffmpeg failed to build the fixture");
            out.to_string_lossy().into_owned()
        }

        #[test]
        fn end_to_end_with_real_ffmpeg() {
            if !have_ffmpeg() {
                eprintln!("ffmpeg/ffprobe not installed: skipping");
                return;
            }
            let dir = std::env::temp_dir().join(format!("eiga-test-{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let clip = make_clip(&dir);

            let info = probe(&clip).unwrap();
            assert!((info.duration - 2.0).abs() < 0.3, "duration {}", info.duration);
            assert_eq!(info.size_wh(), (160, 90));
            assert!((info.fps() - 10.0).abs() < 0.1);
            assert_eq!(info.audio.len(), 1);
            assert_eq!(info.subs.len(), 1);

            // decode: poll with an advancing clock, expect roughly one frame per tick
            let mut src = FfmpegSource::open(&clip, info.clone(), 160, 90).unwrap();
            let (mut n, mut last_pts) = (0, -1.0);
            let t0 = std::time::Instant::now();
            while t0.elapsed().as_secs_f64() < 6.0 {
                let clock = t0.elapsed().as_secs_f64();
                if let Some(f) = src.poll(clock) {
                    assert_eq!((f.w, f.h, f.rgba.len()), (160, 90, 160 * 90 * 4));
                    assert!(f.pts > last_pts, "pts must increase");
                    last_pts = f.pts;
                    n += 1;
                }
                if src.exhausted() && clock > 2.5 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            assert!(n >= 15, "decoded {n} frames");
            assert!(src.exhausted());

            // seek
            src.seek(1.0);
            let t1 = std::time::Instant::now();
            let mut first = None;
            while t1.elapsed().as_secs_f64() < 5.0 && first.is_none() {
                first = src.poll(1.5);
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            assert!(first.expect("a frame after seeking").pts >= 0.9);

            // embedded subtitles
            let text = extract_sub(&clip, 0).unwrap();
            let cues = crate::subs::parse(&text);
            assert_eq!(cues.len(), 2);
            assert_eq!(cues[1].text, "World");

            // audio pipe delivers PCM
            let ap = AudioPipe::open(&clip, 0.0, 0, 1.0).unwrap();
            let mut samples = 0;
            let t2 = std::time::Instant::now();
            while t2.elapsed().as_secs_f64() < 5.0 && samples < 48_000 {
                if let Some(c) = ap.try_chunk() {
                    assert!(c.iter().all(|s| s.is_finite() && s.abs() <= 1.5));
                    samples += c.len();
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
            assert!(samples >= 48_000, "got {samples} samples");
            drop(ap);
            drop(src);
            let _ = std::fs::remove_dir_all(&dir);
        }

        #[test]
        fn broken_input_fails_cleanly() {
            if !have_ffmpeg() {
                return;
            }
            let dir = std::env::temp_dir().join(format!("eiga-bad-{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let bad = dir.join("bad.mp4");
            std::fs::write(&bad, (0..100u8).collect::<Vec<_>>()).unwrap();
            assert!(probe(bad.to_str().unwrap()).is_err());
            assert!(probe("/definitely/not/here.mkv").is_err());
            assert!(extract_sub("/definitely/not/here.mkv", 0).is_err());
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}
