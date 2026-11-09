//! Subtitle parsing. Accepts SRT, WebVTT and ASS/SSA, tolerates BOMs, UTF-16, CRLF, missing indices and garbage.

#[derive(Clone, Debug, PartialEq)]
pub struct Cue {
    pub start: f64,
    pub end: f64,
    /// Plain text, lines separated by `\n`. Markup is stripped.
    pub text: String,
    pub italic: bool,
    /// ASS `\an7..9` (top row) positioning.
    pub top: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SubTrack {
    pub name: String,
    pub lang: String,
    cues: Vec<Cue>,
    max_dur: f64,
    /// Seconds added to every cue (positive = later).
    pub delay: f64,
}

/// Decodes subtitle bytes: UTF-8 (BOM aware), UTF-16 LE/BE with BOM, otherwise Latin-1/Windows-1252-ish.
pub fn decode_bytes(b: &[u8]) -> String {
    if b.starts_with(&[0xFF, 0xFE]) || b.starts_with(&[0xFE, 0xFF]) {
        let le = b[0] == 0xFF;
        let units: Vec<u16> = b[2..].chunks_exact(2).map(|c| if le { u16::from_le_bytes([c[0], c[1]]) } else { u16::from_be_bytes([c[0], c[1]]) }).collect();
        return String::from_utf16_lossy(&units);
    }
    let b = b.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(b);
    match std::str::from_utf8(b) {
        Ok(s) => s.to_string(),
        Err(_) => b.iter().map(|&x| x as char).collect(),
    }
}

/// `00:01:02,500`, `1:02.5`, `0:00:01.00`, `62.5` → seconds.
pub fn parse_ts(s: &str) -> Option<f64> {
    let s = s.trim().replace(',', ".");
    if s.is_empty() || s.len() > 16 {
        return None;
    }
    let mut total = 0.0f64;
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() > 3 {
        return None;
    }
    for p in &parts {
        let v: f64 = p.trim().parse().ok()?;
        if !v.is_finite() || v < 0.0 {
            return None;
        }
        total = total * 60.0 + v;
    }
    Some(total)
}

fn decode_entities(s: &str) -> String {
    s.replace("&nbsp;", " ").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&")
}

/// Strips `<i>…</i>`, `<font …>`, `{\an8}` etc. Returns (text, italic, top).
fn clean(raw: &str) -> (String, bool, bool) {
    let mut italic = false;
    let mut top = false;
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '<' => {
                let mut tag = String::new();
                let mut closed = false;
                for t in chars.by_ref() {
                    if t == '>' {
                        closed = true;
                        break;
                    }
                    tag.push(t);
                    if tag.len() > 80 {
                        break;
                    }
                }
                if !closed {
                    out.push('<');
                    out.push_str(&tag);
                } else if tag.eq_ignore_ascii_case("i") {
                    italic = true;
                }
            }
            '{' => {
                let mut tag = String::new();
                let mut closed = false;
                for t in chars.by_ref() {
                    if t == '}' {
                        closed = true;
                        break;
                    }
                    tag.push(t);
                    if tag.len() > 200 {
                        break;
                    }
                }
                if !closed {
                    out.push('{');
                    out.push_str(&tag);
                } else {
                    if tag.contains("\\i1") {
                        italic = true;
                    }
                    for n in ["\\an7", "\\an8", "\\an9"] {
                        if tag.contains(n) {
                            top = true;
                        }
                    }
                }
            }
            '\\' if matches!(chars.peek(), Some('N') | Some('n')) => {
                chars.next();
                out.push('\n');
            }
            _ => out.push(c),
        }
    }
    let text = decode_entities(&out).lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n");
    (text, italic, top)
}

fn parse_arrow_block(lines: &[&str]) -> Option<Cue> {
    let ai = lines.iter().position(|l| l.contains("-->"))?;
    let (a, b) = lines[ai].split_once("-->")?;
    let start = parse_ts(a)?;
    // VTT cue settings follow the end time ("align:start position:10%")
    let b = b.trim().split_whitespace().next()?;
    let end = parse_ts(b)?;
    let (text, italic, top) = clean(&lines[ai + 1..].join("\n"));
    if text.is_empty() || end <= start {
        return None;
    }
    Some(Cue { start, end, text, italic, top })
}

fn parse_srt_vtt(src: &str) -> Vec<Cue> {
    let norm = src.replace("\r\n", "\n").replace('\r', "\n");
    let mut cues = vec![];
    for block in norm.split("\n\n") {
        let lines: Vec<&str> = block.lines().map(|l| l.trim_end()).filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() || lines[0].starts_with("WEBVTT") || lines[0].starts_with("NOTE") || lines[0].starts_with("STYLE") || lines[0].starts_with("REGION") {
            continue;
        }
        if let Some(c) = parse_arrow_block(&lines) {
            cues.push(c);
        }
    }
    cues
}

fn parse_ass(src: &str) -> Vec<Cue> {
    let mut cues = vec![];
    let (mut in_events, mut si, mut ei, mut ti, mut nfields) = (false, 1usize, 2usize, 9usize, 10usize);
    for raw in src.replace("\r\n", "\n").lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            in_events = line.eq_ignore_ascii_case("[events]");
            continue;
        }
        if !in_events {
            continue;
        }
        if let Some(f) = line.strip_prefix("Format:") {
            let cols: Vec<String> = f.split(',').map(|c| c.trim().to_ascii_lowercase()).collect();
            nfields = cols.len();
            si = cols.iter().position(|c| c == "start").unwrap_or(1);
            ei = cols.iter().position(|c| c == "end").unwrap_or(2);
            ti = cols.iter().position(|c| c == "text").unwrap_or(nfields.saturating_sub(1));
        } else if let Some(d) = line.strip_prefix("Dialogue:") {
            let f: Vec<&str> = d.trim_start().splitn(nfields.max(1), ',').collect();
            if f.len() <= ti.max(si).max(ei) {
                continue;
            }
            let (Some(start), Some(end)) = (parse_ts(f[si]), parse_ts(f[ei])) else { continue };
            let (text, italic, top) = clean(f[ti]);
            if !text.is_empty() && end > start {
                cues.push(Cue { start, end, text, italic, top });
            }
        }
    }
    cues
}

/// Parses any supported format by sniffing the content. Never panics; unparseable input yields no cues.
pub fn parse(src: &str) -> Vec<Cue> {
    let head = src.trim_start_matches('\u{feff}').trim_start();
    let mut cues = if head.starts_with("[Script Info]") || head.contains("\n[Events]") || head.starts_with("[V4") {
        parse_ass(src)
    } else {
        parse_srt_vtt(head)
    };
    cues.retain(|c| c.start.is_finite() && c.end.is_finite());
    cues.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap_or(std::cmp::Ordering::Equal));
    cues
}

impl SubTrack {
    pub fn new(name: impl Into<String>, lang: impl Into<String>, cues: Vec<Cue>) -> Self {
        let max_dur = cues.iter().map(|c| c.end - c.start).fold(0.0, f64::max);
        SubTrack { name: name.into(), lang: lang.into(), cues, max_dur, delay: 0.0 }
    }
    pub fn from_bytes(name: &str, bytes: &[u8]) -> SubTrack {
        let lang = guess_lang(name);
        SubTrack::new(name, lang, parse(&decode_bytes(bytes)))
    }
    pub fn len(&self) -> usize {
        self.cues.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cues.is_empty()
    }
    pub fn cues(&self) -> &[Cue] {
        &self.cues
    }

    /// Cues visible at media time `t` (the delay is applied here).
    pub fn active(&self, t: f64) -> Vec<&Cue> {
        let t = t - self.delay;
        let idx = self.cues.partition_point(|c| c.start <= t);
        let mut out = vec![];
        for c in self.cues[..idx].iter().rev() {
            if c.start + self.max_dur < t {
                break;
            }
            if c.end > t {
                out.push(c);
            }
        }
        out.reverse();
        out
    }

    /// Start of the next cue after `t` (for "jump to next subtitle").
    pub fn next_start(&self, t: f64) -> Option<f64> {
        let t = t - self.delay;
        let i = self.cues.partition_point(|c| c.start <= t + 1e-3);
        self.cues.get(i).map(|c| c.start + self.delay)
    }
    pub fn prev_start(&self, t: f64) -> Option<f64> {
        let t = t - self.delay;
        let i = self.cues.partition_point(|c| c.start < t - 1e-3);
        i.checked_sub(1).and_then(|i| self.cues.get(i)).map(|c| c.start + self.delay)
    }
}

/// "Movie.en.srt" → "en"; "Movie.srt" → "".
pub fn guess_lang(file: &str) -> String {
    let stem = file.rsplit_once('.').map(|x| x.0).unwrap_or(file);
    match stem.rsplit_once('.') {
        Some((_, l)) if (2..=3).contains(&l.len()) && l.chars().all(|c| c.is_ascii_alphabetic()) => l.to_ascii_lowercase(),
        _ => String::new(),
    }
}

/// Sibling subtitle files for `video` out of `names`: same stem (case-insensitive), ext srt/vtt/ass/ssa.
pub fn find_sidecars(video: &str, names: &[String]) -> Vec<String> {
    let stem = video.rsplit_once('.').map(|x| x.0).unwrap_or(video).to_lowercase();
    let mut v: Vec<String> = names
        .iter()
        .filter(|n| {
            let l = n.to_lowercase();
            let Some((s, e)) = l.rsplit_once('.') else { return false };
            matches!(e, "srt" | "vtt" | "ass" | "ssa") && (s == stem || s.starts_with(&format!("{stem}.")))
        })
        .cloned()
        .collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRT: &str = "\u{feff}1\r\n00:00:01,000 --> 00:00:03,500\r\nHello <i>there</i>\r\n\r\n2\r\n00:00:04,000 --> 00:00:06,000\r\nLine one\r\nLine two\r\n\r\n3\r\n00:00:05,000 --> 00:00:07,000\r\n{\\an8}Overlap &amp; top\r\n";

    #[test]
    fn srt_basics() {
        let c = parse(SRT);
        assert_eq!(c.len(), 3);
        assert_eq!(c[0].text, "Hello there");
        assert!(c[0].italic);
        assert_eq!(c[1].text, "Line one\nLine two");
        assert!(c[2].top);
        assert_eq!(c[2].text, "Overlap & top");
        assert!((c[0].end - 3.5).abs() < 1e-9);
    }

    #[test]
    fn srt_without_indices_and_dot_separators() {
        let c = parse("00:00:01.250 --> 00:00:02.000\nA\n\n00:00:03.000 --> 00:00:04.000\nB");
        assert_eq!(c.len(), 2);
        assert!((c[0].start - 1.25).abs() < 1e-9);
    }

    #[test]
    fn vtt_with_settings_notes_and_short_timestamps() {
        let v = "WEBVTT\n\nNOTE a comment\n\nintro\n00:01.000 --> 00:02.500 align:start position:10%\n<v Bob>Hi</v>\n\nSTYLE\n::cue{color:red}\n\n01:00:00.000 --> 01:00:01.000\nLate";
        let c = parse(v);
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].text, "Hi");
        assert!((c[0].end - 2.5).abs() < 1e-9);
        assert!((c[1].start - 3600.0).abs() < 1e-9);
    }

    #[test]
    fn ass_with_overrides_and_commas_in_text() {
        let a = "[Script Info]\nTitle: x\n\n[V4+ Styles]\nFormat: Name\nStyle: Default\n\n[Events]\nFormat: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\nDialogue: 0,0:00:01.00,0:00:03.00,Default,,0,0,0,,{\\i1}Well, hello\\Nworld{\\an8}\nDialogue: 0,0:00:05.00,0:00:04.00,Default,,0,0,0,,backwards\nComment: 0,0:00:05.00,0:00:06.00,Default,,0,0,0,,ignored\n";
        let c = parse(a);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].text, "Well, hello\nworld");
        assert!(c[0].italic && c[0].top);
    }

    #[test]
    fn active_lookup_handles_overlap_and_boundaries() {
        let t = SubTrack::new("x", "", parse(SRT));
        assert!(t.active(0.5).is_empty());
        assert_eq!(t.active(1.0).len(), 1);
        assert!(t.active(3.5).is_empty(), "end is exclusive");
        assert_eq!(t.active(5.5).len(), 2, "two overlapping cues");
        assert!(t.active(100.0).is_empty());
    }

    #[test]
    fn long_cue_is_found_after_many_short_ones() {
        let mut cues = vec![Cue { start: 0.0, end: 1000.0, text: "long".into(), italic: false, top: false }];
        for i in 0..200 {
            cues.push(Cue { start: 1.0 + i as f64, end: 1.5 + i as f64, text: format!("s{i}"), italic: false, top: false });
        }
        let t = SubTrack::new("x", "", cues);
        assert!(t.active(500.0).iter().any(|c| c.text == "long"));
    }

    #[test]
    fn delay_shifts_everything() {
        let mut t = SubTrack::new("x", "", parse(SRT));
        t.delay = 2.0;
        assert!(t.active(1.5).is_empty());
        assert_eq!(t.active(3.5).len(), 1);
        assert_eq!(t.next_start(0.0), Some(3.0));
        assert_eq!(t.prev_start(6.5), Some(6.0));
        t.delay = -0.5;
        assert_eq!(t.active(0.6).len(), 1);
    }

    #[test]
    fn navigation_between_cues() {
        let t = SubTrack::new("x", "", parse(SRT));
        assert_eq!(t.next_start(0.0), Some(1.0));
        assert_eq!(t.next_start(1.0), Some(4.0));
        assert_eq!(t.next_start(5.0), None);
        assert_eq!(t.prev_start(4.0), Some(1.0));
        assert_eq!(t.prev_start(0.5), None);
    }

    #[test]
    fn encodings() {
        let mut u16le = vec![0xFF, 0xFE];
        for u in "1\n00:00:01,000 --> 00:00:02,000\nCaf\u{e9}\n".encode_utf16() {
            u16le.extend(u.to_le_bytes());
        }
        assert_eq!(SubTrack::from_bytes("a.srt", &u16le).cues()[0].text, "Caf\u{e9}");
        let latin1 = b"1\n00:00:01,000 --> 00:00:02,000\nCaf\xe9\n";
        assert_eq!(SubTrack::from_bytes("a.srt", latin1).cues()[0].text, "Caf\u{e9}");
    }

    #[test]
    fn garbage_never_panics() {
        let mut x: u64 = 0x2545F4914F6CDD1D;
        for _ in 0..300 {
            let mut bytes = vec![];
            for _ in 0..(x % 300) {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                let set: &[u8] = b"-->:,.\n\r <>{}\\0123[]Dialogue:";
                bytes.push(set[(x % set.len() as u64) as usize]);
            }
            let _ = SubTrack::from_bytes("g.srt", &bytes);
        }
        assert!(parse("").is_empty());
        assert!(parse("00:00:99999999999999999 --> 1").is_empty());
        assert!(parse("<<<<<<").is_empty());
    }

    #[test]
    fn sidecars_and_languages() {
        let names: Vec<String> = ["Film.mkv", "film.srt", "Film.en.srt", "Film.pt-BR.ass", "Other.srt", "Film.txt", "Film.nl.vtt"].iter().map(|s| s.to_string()).collect();
        assert_eq!(find_sidecars("Film.mkv", &names), vec!["Film.en.srt", "Film.nl.vtt", "Film.pt-BR.ass", "film.srt"]);
        assert_eq!(guess_lang("Film.en.srt"), "en");
        assert_eq!(guess_lang("Film.srt"), "");
    }
}
