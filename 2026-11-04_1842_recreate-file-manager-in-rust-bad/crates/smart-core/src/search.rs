//! Search: incremental in-memory index + natural-language / operator query parser + nucleo fuzzy ranking.
use crate::model::*;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher, Utf32Str,
};

#[derive(Clone, Debug)]
pub struct IndexEntry {
    pub path: String,
    pub name: String,
    pub lname: String,
    pub kind: Kind,
    pub size: u64,
    pub modified: i64,
    pub is_dir: bool,
    pub ext: String,
}

#[derive(Default)]
pub struct SearchIndex {
    pub entries: Vec<IndexEntry>,
    pub queue: Vec<String>,
    pub dirs_done: usize,
    pub ready: bool,
    pub cap: usize,
}

impl SearchIndex {
    pub fn reset(&mut self, roots: Vec<String>) {
        self.entries.clear();
        self.queue = roots;
        self.queue.reverse();
        self.dirs_done = 0;
        self.ready = false;
        self.cap = 400_000;
    }
    pub fn scanning(&self) -> bool {
        !self.ready
    }
    pub fn add(&mut self, e: &Entry) {
        if self.entries.len() >= self.cap {
            return;
        }
        self.entries.push(IndexEntry {
            path: e.path.clone(),
            lname: e.name.to_lowercase(),
            name: e.name.clone(),
            kind: e.kind,
            size: e.size,
            modified: e.modified,
            is_dir: e.is_dir,
            ext: e.ext.clone(),
        });
    }
    pub fn remove_prefix(&mut self, p: &str) {
        self.entries.retain(|e| !is_within(&e.path, p));
    }
}

/// Directories never worth indexing.
pub fn skip_dir(name: &str) -> bool {
    matches!(name, ".git" | "node_modules" | "target" | ".cargo" | ".rustup" | "proc" | "sys" | "dev" | ".npm" | "__pycache__")
}

// ───────────────────────── query ─────────────────────────
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Query {
    pub raw: String,
    pub text: String,
    pub kinds: Vec<Kind>,
    pub exts: Vec<String>,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
    pub after: Option<i64>,
    pub before: Option<i64>,
    pub tag: Option<String>,
    pub scope: Option<String>,
    pub content: Option<String>,
    pub only_dirs: bool,
    pub chips: Vec<String>,
}

fn start_of_today() -> i64 {
    use chrono::{Local, TimeZone};
    let d = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
    Local.from_local_datetime(&d).single().map(|t| t.timestamp()).unwrap_or_else(now_ts)
}

pub fn parse_size(s: &str) -> Option<u64> {
    let s = s.to_lowercase();
    let num: String = s.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    let n: f64 = num.parse().ok()?;
    let mul = match s[num.len()..].trim() {
        "" | "b" => 1.0,
        "k" | "kb" => 1e3,
        "m" | "mb" => 1e6,
        "g" | "gb" => 1e9,
        "t" | "tb" => 1e12,
        _ => return None,
    };
    Some((n * mul) as u64)
}

fn tokens(q: &str) -> Vec<String> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut quoted = false;
    for c in q.chars() {
        match c {
            '"' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur))
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn parse(raw: &str) -> Query {
    let mut q = Query { raw: raw.to_string(), ..Default::default() };
    let now = now_ts();
    let today = start_of_today();
    let mut s = raw.to_lowercase();
    for (phrase, after, before, chip) in [
        ("last week", Some(now - 7 * 86400), None, "Last 7 days"),
        ("this week", Some(now - 7 * 86400), None, "Last 7 days"),
        ("last month", Some(now - 30 * 86400), None, "Last 30 days"),
        ("this month", Some(now - 30 * 86400), None, "Last 30 days"),
        ("last year", Some(now - 365 * 86400), None, "Last year"),
        ("this year", Some(now - 365 * 86400), None, "This year"),
        ("yesterday", Some(today - 86400), Some(today), "Yesterday"),
        ("today", Some(today), None, "Today"),
    ] {
        if s.contains(phrase) {
            s = s.replace(phrase, " ");
            q.after = after;
            q.before = before;
            q.chips.push(chip.into());
        }
    }
    let mut words = vec![];
    for t in tokens(&s) {
        if let Some((k, v)) = t.split_once(':') {
            let v = v.trim();
            match k {
                "kind" | "type" | "is" => {
                    if let Some(kd) = kind_word(v) {
                        q.kinds.push(kd.0);
                        q.only_dirs |= kd.1;
                        q.chips.push(kd.0.plural().into());
                        continue;
                    }
                }
                "ext" => {
                    q.exts.push(v.trim_start_matches('.').to_string());
                    q.chips.push(format!(".{}", v.trim_start_matches('.')));
                    continue;
                }
                "size" => {
                    if let Some(r) = v.strip_prefix('>') {
                        q.size_min = parse_size(r);
                        q.chips.push(format!("> {}", r.to_uppercase()));
                        continue;
                    } else if let Some(r) = v.strip_prefix('<') {
                        q.size_max = parse_size(r);
                        q.chips.push(format!("< {}", r.to_uppercase()));
                        continue;
                    }
                }
                "modified" | "date" | "after" => {
                    let (a, c) = match v {
                        "today" => (today, "Today"),
                        "yesterday" => (today - 86400, "Since yesterday"),
                        "week" => (now - 7 * 86400, "Last 7 days"),
                        "month" => (now - 30 * 86400, "Last 30 days"),
                        _ => (now - 365 * 86400, "Last year"),
                    };
                    q.after = Some(a);
                    q.chips.push(c.into());
                    continue;
                }
                "tag" => {
                    q.tag = Some(v.to_string());
                    q.chips.push(format!("Tag: {v}"));
                    continue;
                }
                "in" => {
                    q.scope = Some(v.to_string());
                    q.chips.push(format!("In {v}"));
                    continue;
                }
                "content" | "has" => {
                    q.content = Some(v.to_string());
                    q.chips.push(format!("Contains “{v}”"));
                    continue;
                }
                _ => {}
            }
        }
        words.push(t);
    }
    // natural-language leftovers
    let mut text = vec![];
    for w in words {
        let wl = w.as_str();
        if matches!(wl, "show" | "find" | "me" | "my" | "all" | "the" | "of" | "from" | "in" | "files" | "file" | "a" | "an" | "that" | "with" | "named" | "called" | "for" | "search" | "looking" | "where" | "are" | "is" | "i" | "have" | "got" | "any" | "recent" | "new" | "latest") {
            if wl == "recent" || wl == "latest" || wl == "new" {
                if q.after.is_none() {
                    q.after = Some(now - 14 * 86400);
                    q.chips.push("Recent".into());
                }
            }
            continue;
        }
        if let Some((k, d)) = kind_word(wl) {
            if !q.kinds.contains(&k) {
                q.kinds.push(k);
                q.chips.push(k.plural().into());
            }
            q.only_dirs |= d;
            continue;
        }
        match wl {
            "big" | "large" | "huge" | "heavy" => {
                q.size_min = Some(100_000_000);
                q.chips.push("> 100 MB".into());
            }
            "small" | "tiny" => {
                q.size_max = Some(1_000_000);
                q.chips.push("< 1 MB".into());
            }
            _ => text.push(w),
        }
    }
    q.text = text.join(" ");
    q
}

fn kind_word(w: &str) -> Option<(Kind, bool)> {
    Some(match w {
        "image" | "images" | "photo" | "photos" | "picture" | "pictures" | "pic" | "pics" | "screenshot" | "screenshots" => (Kind::Image, false),
        "video" | "videos" | "movie" | "movies" | "clip" | "clips" => (Kind::Video, false),
        "music" | "song" | "songs" | "audio" | "sound" | "sounds" | "track" | "tracks" => (Kind::Audio, false),
        "document" | "documents" | "doc" | "docs" | "pdf" | "pdfs" | "paper" | "papers" => (Kind::Document, false),
        "archive" | "archives" | "zip" | "zips" => (Kind::Archive, false),
        "folder" | "folders" | "directory" | "directories" | "dir" => (Kind::Folder, true),
        "code" | "script" | "scripts" | "source" => (Kind::Code, false),
        "app" | "apps" | "installer" | "installers" | "application" => (Kind::App, false),
        "text" | "note" | "notes" => (Kind::Text, false),
        _ => return None,
    })
}

impl Query {
    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.kinds.is_empty() && self.exts.is_empty() && self.size_min.is_none() && self.size_max.is_none() && self.after.is_none() && self.tag.is_none() && self.scope.is_none() && self.content.is_none()
    }
    pub fn matches_meta(&self, e: &IndexEntry, tag_of: &dyn Fn(&str) -> Option<String>) -> bool {
        if !self.kinds.is_empty() && !self.kinds.contains(&e.kind) {
            return false;
        }
        if self.only_dirs && !e.is_dir {
            return false;
        }
        if !self.exts.is_empty() && !self.exts.contains(&e.ext) {
            return false;
        }
        if let Some(m) = self.size_min {
            if e.is_dir || e.size < m {
                return false;
            }
        }
        if let Some(m) = self.size_max {
            if e.is_dir || e.size > m {
                return false;
            }
        }
        if let Some(a) = self.after {
            if e.modified < a {
                return false;
            }
        }
        if let Some(b) = self.before {
            if e.modified >= b {
                return false;
            }
        }
        if let Some(t) = &self.tag {
            if tag_of(&e.path).as_deref() != Some(t.as_str()) {
                return false;
            }
        }
        if let Some(s) = &self.scope {
            let lp = e.path.to_lowercase();
            if !(lp.contains(&format!("/{s}/")) || lp.ends_with(&format!("/{s}"))) {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct Hit {
    pub idx: usize,
    pub score: u32,
    pub indices: Vec<u32>,
}

pub fn run(index: &SearchIndex, q: &Query, tag_of: &dyn Fn(&str) -> Option<String>, boost: &dyn Fn(&str) -> u32, limit: usize) -> Vec<Hit> {
    let now = now_ts();
    let mut matcher = Matcher::new(Config::DEFAULT);
    let pat = (!q.text.is_empty()).then(|| Pattern::parse(&q.text, CaseMatching::Smart, Normalization::Smart));
    let ltext = q.text.to_lowercase();
    let mut buf = Vec::new();
    let mut hits: Vec<Hit> = Vec::new();
    for (idx, e) in index.entries.iter().enumerate() {
        if !q.matches_meta(e, tag_of) {
            continue;
        }
        let mut score = 0u32;
        if let Some(p) = &pat {
            let Some(s) = p.score(Utf32Str::new(&e.name, &mut buf), &mut matcher) else { continue };
            score = s;
            if e.lname == ltext {
                score += 600;
            } else if e.lname.starts_with(&ltext) {
                score += 160;
            }
        }
        let age_days = ((now - e.modified).max(0) / 86400) as u32;
        score += 60u32.saturating_sub(age_days.min(60));
        score += boost(&e.path);
        if e.is_dir {
            score += 8;
        }
        hits.push(Hit { idx, score, indices: vec![] });
    }
    hits.sort_unstable_by(|a, b| b.score.cmp(&a.score).then(a.idx.cmp(&b.idx)));
    hits.truncate(limit);
    if let Some(p) = &pat {
        for h in hits.iter_mut() {
            let mut ix = Vec::new();
            p.indices(Utf32Str::new(&index.entries[h.idx].name, &mut buf), &mut matcher, &mut ix);
            ix.sort_unstable();
            ix.dedup();
            h.indices = ix;
        }
    }
    hits
}
