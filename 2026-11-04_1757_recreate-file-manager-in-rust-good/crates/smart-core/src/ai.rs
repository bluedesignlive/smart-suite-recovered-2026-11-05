//! Ask Smart: an on-device assistant (intent + query understanding, folder awareness, previewable plans)
//! and the disk cleaner / Space Lens analysis. Everything runs locally; the `reply` entry point is the
//! seam where a language model can be plugged in later.
use crate::archive::Format;
use crate::engine::Engine;
use crate::model::*;
use crate::search;
use crate::store::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum PlanOp {
    MkDir(Path),
    Move { from: Path, to_dir: Path },
    Rename { from: Path, to: String },
    Trash(Path),
    Compress { paths: Vec<Path>, name: String },
}
#[derive(Clone, Debug)]
pub struct Plan {
    pub title: String,
    pub ops: Vec<PlanOp>,
}
#[derive(Clone, Debug, Default)]
pub struct AiMsg {
    pub user: bool,
    pub text: String,
    pub stats: Vec<(Icon, String)>,
    pub results: Vec<Path>,
    pub plan: Option<Plan>,
    pub applied: bool,
    pub actions: Vec<(String, Action)>,
    pub suggestions: Vec<String>,
}

fn bold(s: &str) -> String {
    format!("**{s}**")
}

impl Store {
    pub fn ai_intro(&self) -> AiMsg {
        let name = self.title();
        let items = self.items.len();
        let size: u64 = self.items.iter().map(|e| e.size).sum();
        let mut m = AiMsg {
            text: format!("You’re in {}. I can tidy this folder, find duplicates, free up space, or answer questions about what’s here. What are we doing?", bold(&name)),
            ..Default::default()
        };
        m.stats = vec![(Icon::Layers, format!("{} · {}", plural(items, "item", "items"), fmt_size(size)))];
        m.suggestions = vec!["Organise this folder".into(), "Find duplicates".into(), "Free up space".into(), "Show big videos from last year".into()];
        m
    }

    pub fn ai_reply(&self, prompt: &str) -> AiMsg {
        let p = prompt.to_lowercase();
        let has = |ws: &[&str]| ws.iter().any(|w| p.contains(w));
        let folder = self.current_folder();
        let files: Vec<&Entry> = self.items.iter().filter(|e| !e.is_dir).collect();

        if has(&["free up", "free some", "disk space", "storage", "junk", "cache", "reclaim", "clean my", "clean the disk", "running out"]) {
            let r = analyze(&self.eng);
            let total: u64 = r.cats.iter().map(|c| c.selected_size()).sum();
            let all: u64 = r.cats.iter().map(|c| c.size()).sum();
            let mut m = AiMsg { text: format!("I found {} that can be cleaned, and {} of it is safe to remove right now.", bold(&fmt_size(all)), bold(&fmt_size(total))), ..Default::default() };
            for c in r.cats.iter().filter(|c| c.size() > 0).take(4) {
                m.stats.push((c.icon, format!("{} · {}", c.title, fmt_size(c.size()))));
            }
            m.actions = vec![("Open Storage Cleanup".into(), Action::Go(Route::Storage))];
            return m;
        }
        if has(&["duplicate", "same file", "copies of"]) {
            let d = find_duplicates(&self.eng, &folder);
            if d.is_empty() {
                return AiMsg { text: format!("No duplicates in {}. Everything here is unique.", bold(&self.title())), ..Default::default() };
            }
            let waste: u64 = d.iter().map(|(_, e)| e.size).sum();
            let mut m = AiMsg { text: format!("Found {} duplicate files wasting {}. I kept the oldest copy of each.", bold(&d.len().to_string()), bold(&fmt_size(waste))), ..Default::default() };
            m.results = d.iter().map(|(_, e)| e.path.clone()).take(8).collect();
            m.plan = Some(Plan { title: format!("Move {} duplicates to Trash", d.len()), ops: d.iter().map(|(_, e)| PlanOp::Trash(e.path.clone())).collect() });
            return m;
        }
        if has(&["organi", "tidy", "sort this", "group", "declutter"]) {
            let mut by: HashMap<Kind, Vec<&Entry>> = HashMap::new();
            for f in &files {
                by.entry(f.kind).or_default().push(f);
            }
            let mut ops = vec![];
            let mut kinds: Vec<_> = by.into_iter().filter(|(_, v)| v.len() >= 2).collect();
            kinds.sort_by_key(|(k, _)| *k);
            for (k, v) in &kinds {
                let dir = join(&folder, k.plural());
                if !self.eng.exists(&dir) {
                    ops.push(PlanOp::MkDir(dir.clone()));
                }
                for f in v {
                    ops.push(PlanOp::Move { from: f.path.clone(), to_dir: dir.clone() });
                }
            }
            if ops.is_empty() {
                return AiMsg { text: format!("{} is already tidy — I couldn’t find groups of 2+ similar files to separate.", bold(&self.title())), ..Default::default() };
            }
            let n: usize = kinds.iter().map(|(_, v)| v.len()).sum();
            let mut m = AiMsg { text: format!("Here’s a plan: group {} files into {}. Nothing changes until you apply it.", bold(&n.to_string()), bold(&plural(kinds.len(), "folder", "folders"))), ..Default::default() };
            m.plan = Some(Plan { title: format!("Organise {} by kind", self.title()), ops });
            return m;
        }
        if has(&["rename", "clean names", "fix names", "messy names"]) {
            let targets: Vec<&Entry> = if self.sel.is_empty() { self.items.iter().collect() } else { self.items.iter().filter(|e| self.sel.contains(&e.path)).collect() };
            let ops: Vec<PlanOp> = targets
                .iter()
                .filter_map(|e| {
                    let stem = stem_of(&e.name);
                    let mut n = stem.replace(['_', '-'], " ");
                    for junk in [" (1)", " copy", " Copy", " final", " FINAL"] {
                        n = n.replace(junk, "");
                    }
                    let n = n.split_whitespace().collect::<Vec<_>>().join(" ");
                    let full = if e.is_dir || e.ext.is_empty() { n } else { format!("{n}.{}", e.ext) };
                    (full != e.name && !full.is_empty() && !self.eng.exists(&join(&parent(&e.path), &full))).then(|| PlanOp::Rename { from: e.path.clone(), to: full })
                })
                .collect();
            if ops.is_empty() {
                return AiMsg { text: "Names here already look clean — no underscores, copies, or double spaces to fix.".into(), ..Default::default() };
            }
            let mut m = AiMsg { text: format!("I can normalise {} names (underscores → spaces, remove “copy” and “(1)”).", bold(&ops.len().to_string())), ..Default::default() };
            m.plan = Some(Plan { title: format!("Rename {} items", ops.len()), ops });
            return m;
        }
        if has(&["zip", "compress", "archive these", "bundle"]) {
            let paths: Vec<Path> = if self.sel.is_empty() { files.iter().map(|e| e.path.clone()).collect() } else { self.sel.clone() };
            if paths.is_empty() {
                return AiMsg { text: "Select some files first and I’ll bundle them.".into(), ..Default::default() };
            }
            let size: u64 = self.items.iter().filter(|e| paths.contains(&e.path)).map(|e| e.size).sum();
            let name = if paths.len() == 1 { stem_of(name_of(&paths[0])).to_string() } else { self.title() };
            let mut m = AiMsg { text: format!("I’ll bundle {} ({}) into {}.", bold(&plural(paths.len(), "item", "items")), fmt_size(size), bold(&format!("{name}.zip"))), ..Default::default() };
            m.plan = Some(Plan { title: format!("Create {name}.zip"), ops: vec![PlanOp::Compress { paths, name }] });
            return m;
        }
        if has(&["unused", "not opened", "stale", "haven't touched", "old files", "forgotten"]) {
            let cut = now_ts() - 90 * 86400;
            let mut old: Vec<&search::IndexEntry> = self.eng.index.entries.iter().filter(|e| !e.is_dir && e.modified < cut && is_within(&e.path, &folder)).collect();
            old.sort_by_key(|e| std::cmp::Reverse(e.size));
            if old.is_empty() {
                return AiMsg { text: "Nothing here is older than 90 days. This folder is fresh.".into(), ..Default::default() };
            }
            let total: u64 = old.iter().map(|e| e.size).sum();
            let mut m = AiMsg { text: format!("{} haven’t changed in 90+ days and take {}. Largest first:", bold(&plural(old.len(), "file", "files")), bold(&fmt_size(total))), ..Default::default() };
            m.results = old.iter().take(6).map(|e| e.path.clone()).collect();
            m.actions = vec![("Show all in Search".into(), Action::Search(format!("in:{} modified:year", name_of(&folder))))];
            return m;
        }
        if has(&["summar", "what's in", "what is in", "overview", "explain", "describe", "tell me about"]) {
            if let Some(f) = self.sel.first().and_then(|p| self.items.iter().find(|e| &e.path == p)) {
                if matches!(f.kind, Kind::Text | Kind::Code) {
                    if let Ok(b) = self.eng.read_all(&f.path, 256_000) {
                        let t = String::from_utf8_lossy(&b).to_string();
                        let words = t.split_whitespace().count();
                        let lines = t.lines().count();
                        let head = t.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim_start_matches('#').trim().to_string();
                        let mut freq: HashMap<String, usize> = HashMap::new();
                        for w in t.split(|c: char| !c.is_alphanumeric()).map(|w| w.to_lowercase()).filter(|w| w.len() > 4) {
                            *freq.entry(w).or_default() += 1;
                        }
                        let mut kw: Vec<_> = freq.into_iter().collect();
                        kw.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                        let kws: Vec<String> = kw.into_iter().take(4).map(|k| k.0).collect();
                        let mut m = AiMsg { text: format!("{} opens with “{}”. {} words across {} lines{}.", bold(&f.name), head, words, lines, if kws.is_empty() { String::new() } else { format!(", mostly about {}", kws.join(", ")) }), ..Default::default() };
                        m.stats = vec![(Icon::Doc, format!("{words} words")), (Icon::List, format!("{lines} lines"))];
                        return m;
                    }
                }
            }
            let mut by: HashMap<Kind, (usize, u64)> = HashMap::new();
            for e in &self.items {
                let x = by.entry(e.kind).or_default();
                x.0 += 1;
                x.1 += e.size;
            }
            let mut v: Vec<_> = by.into_iter().collect();
            v.sort_by_key(|(_, (n, _))| std::cmp::Reverse(*n));
            let newest = self.items.iter().max_by_key(|e| e.modified);
            let biggest = self.items.iter().filter(|e| !e.is_dir).max_by_key(|e| e.size);
            let mut m = AiMsg { text: format!("{} holds {}. {}{}", bold(&self.title()), plural(self.items.len(), "item", "items"), newest.map(|n| format!("Newest is {} ({}). ", bold(&n.name), fmt_date(n.modified).to_lowercase())).unwrap_or_default(), biggest.map(|b| format!("Largest is {} at {}.", bold(&b.name), fmt_size(b.size))).unwrap_or_default()), ..Default::default() };
            m.stats = v.iter().take(4).map(|(k, (n, s))| (k.icon(), format!("{} {} · {}", n, k.plural().to_lowercase(), fmt_size(*s)))).collect();
            return m;
        }
        if has(&["hello", "hi ", "help", "what can you"]) || p.trim() == "hi" {
            let mut m = self.ai_intro();
            m.text = "I work on-device and understand your folders. Ask in plain language — “screenshots from last week”, “big videos”, “pdf invoices in Documents” — or ask me to organise, dedupe, rename, zip, or free up space.".into();
            return m;
        }
        // default: natural-language search
        let q = search::parse(prompt);
        if !q.is_empty() {
            let hits = self.run_query(&q, 40);
            if hits.is_empty() {
                let mut m = AiMsg { text: format!("I searched {} entries and found nothing for {}. Try fewer words or a broader time range.", self.eng.index.entries.len(), bold(&q.raw)), ..Default::default() };
                m.suggestions = vec!["Show images from last month".into(), "Find big files".into()];
                return m;
            }
            let total: u64 = hits.iter().map(|h| self.eng.index.entries[h.idx].size).sum();
            let mut m = AiMsg { text: format!("Found {} ({}). {}", bold(&plural(hits.len(), "match", "matches")), fmt_size(total), if q.chips.is_empty() { String::new() } else { format!("I read that as: {}.", q.chips.join(" · ")) }), ..Default::default() };
            m.results = hits.iter().take(6).map(|h| self.eng.index.entries[h.idx].path.clone()).collect();
            m.actions = vec![("Open in Search".into(), Action::Search(prompt.to_string()))];
            return m;
        }
        let mut m = self.ai_intro();
        m.text = "I didn’t catch a task there. I can search, organise, dedupe, rename, compress, summarise and clean up — try one of these.".into();
        m
    }
}

pub fn find_duplicates(eng: &Engine, root: &str) -> Vec<(Path, Entry)> {
    let mut groups: HashMap<(u64, String), Vec<&search::IndexEntry>> = HashMap::new();
    for e in eng.index.entries.iter().filter(|e| !e.is_dir && e.size >= 1000 && is_within(&e.path, root)) {
        groups.entry((e.size, e.ext.clone())).or_default().push(e);
    }
    let mut out = vec![];
    for (_, mut g) in groups {
        if g.len() < 2 {
            continue;
        }
        g.sort_by_key(|e| e.modified);
        let keep = g[0];
        for d in &g[1..] {
            let same = if d.size > 64 << 20 {
                norm_name(&d.name) == norm_name(&keep.name)
            } else {
                match (eng.read_head(&keep.path, 65536), eng.read_head(&d.path, 65536)) {
                    (Ok(a), Ok(b)) => a == b,
                    _ => false,
                }
            };
            if same {
                let mut e = Entry::new(&d.path, false, d.size, d.modified);
                e.size = d.size;
                out.push((keep.path.clone(), e));
            }
        }
    }
    out.sort_by_key(|(_, e)| std::cmp::Reverse(e.size));
    out
}
fn norm_name(n: &str) -> String {
    stem_of(n).to_lowercase().replace(" (1)", "").replace(" copy", "").replace(" 2", "")
}

// ───────────────────────── cleaner ─────────────────────────
#[derive(Clone, Debug)]
pub struct CleanItem {
    pub path: Path,
    pub size: u64,
    pub note: String,
    pub selected: bool,
    pub permanent: bool,
}
#[derive(Clone, Debug)]
pub struct CleanCat {
    pub id: &'static str,
    pub title: String,
    pub desc: String,
    pub icon: Icon,
    pub items: Vec<CleanItem>,
    pub open: bool,
}
impl CleanCat {
    pub fn size(&self) -> u64 {
        self.items.iter().map(|i| i.size).sum()
    }
    pub fn selected_size(&self) -> u64 {
        self.items.iter().filter(|i| i.selected).map(|i| i.size).sum()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CleanReport {
    pub cats: Vec<CleanCat>,
    pub scanned: usize,
}
impl CleanReport {
    pub fn selected_total(&self) -> u64 {
        self.cats.iter().map(|c| c.selected_size()).sum()
    }
}

pub fn analyze(eng: &Engine) -> CleanReport {
    let now = now_ts();
    let home = &eng.home;
    let mut caches: HashMap<String, u64> = HashMap::new();
    let mut large = vec![];
    let mut installers = vec![];
    let mut old_dl = vec![];
    let mut trash = vec![];
    let dl = join(home, "Downloads");
    for e in eng.index.entries.iter().filter(|e| !e.is_dir && is_within(&e.path, home)) {
        let rel = &e.path[home.len()..];
        if rel.starts_with("/.cache/") || rel.contains("/Caches/") {
            let top: String = rel.split('/').take(4).collect::<Vec<_>>().join("/");
            *caches.entry(join(home, top.trim_start_matches('/'))).or_default() += e.size;
            continue;
        }
        if eng.trash_dir.as_deref().map(|t| is_within(&e.path, t)).unwrap_or(false) {
            trash.push(e);
            continue;
        }
        if is_within(&e.path, &dl) {
            if matches!(e.ext.as_str(), "dmg" | "exe" | "msi" | "deb" | "rpm" | "appimage" | "pkg") {
                installers.push(e);
            } else if e.modified < now - 90 * 86400 {
                old_dl.push(e);
            }
        }
        if e.size >= 500_000_000 {
            large.push(e);
        }
    }
    let item = |p: &str, size: u64, note: String, sel: bool, perm: bool| CleanItem { path: p.to_string(), size, note, selected: sel, permanent: perm };
    let age = |t: i64| format!("untouched {} days", ((now - t) / 86400).max(0));
    let mut cats = vec![];
    let mut c: Vec<_> = caches.into_iter().collect();
    c.sort_by_key(|(_, s)| std::cmp::Reverse(*s));
    cats.push(CleanCat { id: "cache", title: "App caches".into(), desc: "Rebuilt automatically — safe to remove".into(), icon: Icon::Bolt, items: c.into_iter().map(|(p, s)| item(&p, s, "Cache".into(), true, true)).collect(), open: false });
    installers.sort_by_key(|e| std::cmp::Reverse(e.size));
    cats.push(CleanCat { id: "installers", title: "Installers".into(), desc: "Already installed? Disk images and setup files".into(), icon: Icon::Download, items: installers.iter().map(|e| item(&e.path, e.size, age(e.modified), true, false)).collect(), open: false });
    let d = find_duplicates(eng, home);
    cats.push(CleanCat { id: "dups", title: "Duplicates".into(), desc: "Identical copies — the oldest is kept".into(), icon: Icon::Duplicate, items: d.iter().map(|(k, e)| item(&e.path, e.size, format!("copy of {}", name_of(k)), true, false)).collect(), open: false });
    old_dl.sort_by_key(|e| std::cmp::Reverse(e.size));
    cats.push(CleanCat { id: "olddl", title: "Old downloads".into(), desc: "Downloaded 90+ days ago and never touched".into(), icon: Icon::Clock, items: old_dl.iter().map(|e| item(&e.path, e.size, age(e.modified), false, false)).collect(), open: false });
    large.sort_by_key(|e| std::cmp::Reverse(e.size));
    cats.push(CleanCat { id: "large", title: "Large files".into(), desc: "Over 500 MB — review before removing".into(), icon: Icon::Layers, items: large.iter().take(12).map(|e| item(&e.path, e.size, age(e.modified), false, false)).collect(), open: false });
    cats.push(CleanCat { id: "trash", title: "Trash".into(), desc: "Already deleted — empty it for good".into(), icon: Icon::Trash, items: trash.iter().map(|e| item(&e.path, e.size, "In Trash".into(), true, true)).collect(), open: false });
    CleanReport { cats, scanned: eng.index.entries.len() }
}

#[derive(Clone, Debug)]
pub struct LensItem {
    pub path: Path,
    pub name: String,
    pub size: u64,
    pub kind: Kind,
    pub is_dir: bool,
}
/// Size of each direct child of `root`, from the index.
pub fn lens(eng: &Engine, root: &str) -> Vec<LensItem> {
    let mut m: HashMap<String, LensItem> = HashMap::new();
    let pre = if root == "/" { "/".to_string() } else { format!("{root}/") };
    for e in eng.index.entries.iter().filter(|e| e.path.starts_with(&pre)) {
        let rest = &e.path[pre.len()..];
        let (child, is_dir) = match rest.split_once('/') {
            Some((c, _)) => (c, true),
            None => (rest, e.is_dir),
        };
        let it = m.entry(child.to_string()).or_insert_with(|| LensItem { path: format!("{pre}{child}"), name: child.to_string(), size: 0, kind: if is_dir { Kind::Folder } else { e.kind }, is_dir });
        if !e.is_dir {
            it.size += e.size;
        }
    }
    let mut v: Vec<_> = m.into_values().filter(|i| i.size > 0).collect();
    v.sort_by(|a, b| b.size.cmp(&a.size).then(a.name.cmp(&b.name)));
    v
}

pub fn compress_default_format() -> Format {
    Format::Zip
}
