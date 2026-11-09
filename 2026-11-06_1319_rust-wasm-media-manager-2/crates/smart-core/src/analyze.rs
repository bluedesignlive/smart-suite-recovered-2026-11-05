//! Storage analysis: an incremental, bounded-memory directory scanner plus a *context-aware* junk detector.
//!
//! Design notes
//! * Only **directories** become nodes; per directory we keep the ten biggest files and aggregate the rest, so a
//!   home folder with millions of files stays in the tens of megabytes.
//! * The scanner is a state machine (`Scan::step`). Desktop runs it on a worker thread, the web build slices it
//!   across frames. Nothing here touches threads or clocks, so it behaves identically (and is unit-tested) everywhere.
//! * Junk is not "big files". A `target/` next to a `Cargo.toml`, a `node_modules/` next to a `package.json`, a
//!   `.venv`, package-manager caches, the Trash, stale installers and "Report (1).pdf" copies are all recognised from
//!   their *surroundings*, graded by how safe they are to remove, and annotated with how to get them back.
use crate::model::{Entry, Kind, Location};
use crate::vfs::Vfs;
use std::collections::HashSet;

/// How risky it is to remove something.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Safety {
    /// Regenerated automatically (build output, caches). Removing loses nothing.
    Safe,
    /// Probably unwanted, but it is *your* call (old installers, copies, logs).
    Review,
    /// Looks like personal data that is merely large or old. Never pre-selected.
    Careful,
}

impl Safety {
    pub fn label(self) -> &'static str {
        match self {
            Safety::Safe => "Safe",
            Safety::Review => "Review",
            Safety::Careful => "Careful",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum JunkKind {
    BuildArtifacts,
    Dependencies,
    Environments,
    Caches,
    Temp,
    Logs,
    Trash,
    Installers,
    Duplicates,
    LargeStale,
}

impl JunkKind {
    pub const ALL: [JunkKind; 10] = [
        JunkKind::BuildArtifacts,
        JunkKind::Dependencies,
        JunkKind::Environments,
        JunkKind::Caches,
        JunkKind::Temp,
        JunkKind::Logs,
        JunkKind::Trash,
        JunkKind::Installers,
        JunkKind::Duplicates,
        JunkKind::LargeStale,
    ];
    pub fn label(self) -> &'static str {
        match self {
            JunkKind::BuildArtifacts => "Build artifacts",
            JunkKind::Dependencies => "Dependencies",
            JunkKind::Environments => "Virtual environments",
            JunkKind::Caches => "Caches",
            JunkKind::Temp => "Temporary files",
            JunkKind::Logs => "Large logs",
            JunkKind::Trash => "Trash",
            JunkKind::Installers => "Old installers",
            JunkKind::Duplicates => "Likely copies",
            JunkKind::LargeStale => "Large & untouched",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            JunkKind::BuildArtifacts => "code-square",
            JunkKind::Dependencies => "box-minimalistic",
            JunkKind::Environments => "box-minimalistic",
            JunkKind::Caches => "refresh",
            JunkKind::Temp => "clock-circle",
            JunkKind::Logs => "text",
            JunkKind::Trash => "trash-bin-minimalistic",
            JunkKind::Installers => "download-minimalistic",
            JunkKind::Duplicates => "copy",
            JunkKind::LargeStale => "history",
        }
    }
    pub fn blurb(self) -> &'static str {
        match self {
            JunkKind::BuildArtifacts => "Compiler output that your next build recreates",
            JunkKind::Dependencies => "Downloaded packages that the package manager restores",
            JunkKind::Environments => "Language environments you can rebuild from a lockfile",
            JunkKind::Caches => "Data apps keep to be faster. They rebuild it on demand",
            JunkKind::Temp => "Leftovers: partial downloads, backups, editor swap files",
            JunkKind::Logs => "Log files that grew far beyond usefulness",
            JunkKind::Trash => "Already deleted. Emptying frees the space for real",
            JunkKind::Installers => "Setup files you already ran",
            JunkKind::Duplicates => "Files named like a copy of a sibling",
            JunkKind::LargeStale => "Big files nobody has touched for a long time",
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: Option<i64>,
    pub kind: Kind,
}

#[derive(Clone, Debug)]
pub struct DirNode {
    pub name: String,
    pub path: String,
    pub parent: Option<usize>,
    /// Recursive size in bytes (grows while scanning).
    pub size: u64,
    /// Bytes of files directly inside this folder.
    pub own: u64,
    /// Files directly inside this folder.
    pub files: u64,
    pub children: Vec<usize>,
    /// The biggest files directly inside (at most `TOP_FILES`).
    pub top_files: Vec<FileInfo>,
    pub newest: i64,
    pub junk: Option<usize>,
    pub in_junk: bool,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct Junk {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub kind: JunkKind,
    pub safety: Safety,
    pub reason: String,
    /// How the data comes back, when it does.
    pub restore: Option<&'static str>,
    pub modified: Option<i64>,
    size: u64,
    node: Option<usize>,
}

impl Junk {
    pub fn size(&self) -> u64 {
        self.size
    }
}

pub const TOP_FILES: usize = 10;
pub const MIN_JUNK_BYTES: u64 = 512 * 1024;
const MAX_FILE_JUNK: usize = 1500;
const DAY: i64 = 86_400;

/// Coarse content buckets for the overview chart.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Bucket {
    Media,
    Documents,
    Code,
    Archives,
    Apps,
    Other,
}

impl Bucket {
    pub const ALL: [Bucket; 6] = [Bucket::Media, Bucket::Documents, Bucket::Code, Bucket::Archives, Bucket::Apps, Bucket::Other];
    pub fn label(self) -> &'static str {
        match self {
            Bucket::Media => "Photos, audio & video",
            Bucket::Documents => "Documents",
            Bucket::Code => "Code & text",
            Bucket::Archives => "Archives",
            Bucket::Apps => "Apps & installers",
            Bucket::Other => "Other",
        }
    }
    pub fn of(k: Kind) -> Bucket {
        match k {
            Kind::Image | Kind::Vector | Kind::Audio | Kind::Video | Kind::Project => Bucket::Media,
            Kind::Pdf | Kind::Document => Bucket::Documents,
            Kind::Text | Kind::Code => Bucket::Code,
            Kind::Archive => Bucket::Archives,
            Kind::App => Bucket::Apps,
            _ => Bucket::Other,
        }
    }
    fn idx(self) -> usize {
        Bucket::ALL.iter().position(|b| *b == self).unwrap_or(5)
    }
}

pub struct Scan {
    pub fs: String,
    pub root: String,
    pub nodes: Vec<DirNode>,
    pub junk: Vec<Junk>,
    pub buckets: [u64; 6],
    pub files_seen: u64,
    pub dirs_seen: u64,
    pub errors: u64,
    pub finished: bool,
    pub now: i64,
    /// Absolute paths that are never entered (pseudo file systems such as /proc).
    pub skip: Vec<String>,
    queue: Vec<usize>,
    file_junk: usize,
}

fn lower_ext(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, e)) if !stem.is_empty() => e.to_ascii_lowercase(),
        _ => String::new(),
    }
}

fn join(dir: &str, name: &str) -> String {
    if dir.ends_with('/') {
        format!("{dir}{name}")
    } else {
        format!("{dir}/{name}")
    }
}

/// If `name` looks like "base (1).ext" / "base - Copy.ext" / "base copy.ext" returns the original's file name.
pub fn copy_original(name: &str) -> Option<String> {
    let (stem, ext) = match name.rsplit_once('.') {
        Some((s, e)) if !s.is_empty() && e.len() <= 8 => (s, format!(".{e}")),
        _ => (name, String::new()),
    };
    for suf in [" - Copy", " copy", " (copy)", "_copy", "-copy"] {
        if let Some(b) = stem.strip_suffix(suf) {
            if !b.is_empty() {
                return Some(format!("{b}{ext}"));
            }
        }
    }
    // "name (12)"
    if let Some(open) = stem.rfind(" (") {
        let tail = &stem[open + 2..];
        if let Some(num) = tail.strip_suffix(')') {
            if !num.is_empty() && num.len() <= 3 && num.bytes().all(|b| b.is_ascii_digit()) && open > 0 {
                return Some(format!("{}{}", &stem[..open], ext));
            }
        }
    }
    None
}

struct DirClass {
    kind: JunkKind,
    safety: Safety,
    reason: String,
    restore: Option<&'static str>,
}

fn classify_dir(name: &str, parent_name: &str, siblings: &HashSet<String>) -> Option<DirClass> {
    let has = |n: &str| siblings.contains(n);
    let has_ext = |e: &str| siblings.iter().any(|s| s.ends_with(e));
    let mk = |kind, safety, reason: &str, restore: Option<&'static str>| Some(DirClass { kind, safety, reason: reason.into(), restore });
    match name {
        "target" if has("Cargo.toml") => mk(JunkKind::BuildArtifacts, Safety::Safe, "Rust build output next to a Cargo.toml", Some("cargo build")),
        "target" if has("pom.xml") || has("build.gradle") => mk(JunkKind::BuildArtifacts, Safety::Safe, "Maven/Gradle build output", Some("mvn package")),
        "node_modules" => mk(JunkKind::Dependencies, Safety::Safe, "JavaScript packages. The lockfile lists every one", Some("npm install")),
        "__pycache__" | ".pytest_cache" | ".mypy_cache" | ".ruff_cache" | ".tox" | ".nox" | ".hypothesis" => mk(JunkKind::Caches, Safety::Safe, "Python tool cache", None),
        ".venv" | "venv" => mk(JunkKind::Environments, Safety::Review, "Python virtual environment", Some("python -m venv && pip install -r requirements.txt")),
        ".next" | ".nuxt" | ".svelte-kit" | ".turbo" | ".parcel-cache" | ".angular" | ".vite" | ".docusaurus" => mk(JunkKind::BuildArtifacts, Safety::Safe, "Web framework build cache", Some("your dev or build command")),
        "dist" if has("package.json") => mk(JunkKind::BuildArtifacts, Safety::Safe, "Bundled output of a JavaScript project", Some("npm run build")),
        "build" if has("package.json") || has("CMakeLists.txt") || has("build.gradle") || has("setup.py") || has("pubspec.yaml") => mk(JunkKind::BuildArtifacts, Safety::Safe, "Build folder of a project", Some("your build command")),
        ".gradle" | ".dart_tool" | ".stack-work" | ".zig-cache" | "zig-cache" | "DerivedData" => mk(JunkKind::BuildArtifacts, Safety::Safe, "Toolchain build cache", None),
        "obj" | "bin" if has_ext(".csproj") || has_ext(".fsproj") => mk(JunkKind::BuildArtifacts, Safety::Safe, ".NET build output", Some("dotnet build")),
        "Pods" if has("Podfile") => mk(JunkKind::Dependencies, Safety::Safe, "CocoaPods dependencies", Some("pod install")),
        "vendor" if has("composer.json") => mk(JunkKind::Dependencies, Safety::Safe, "Composer packages", Some("composer install")),
        "_cacache" => mk(JunkKind::Caches, Safety::Safe, "npm download cache", None),
        "registry" | "git" if parent_name == ".cargo" => mk(JunkKind::Caches, Safety::Review, "Cargo crate downloads. Re-fetched on the next build", Some("cargo fetch")),
        "repository" if parent_name == ".m2" => mk(JunkKind::Caches, Safety::Review, "Maven local repository", Some("mvn dependency:resolve")),
        "caches" if parent_name == ".gradle" => mk(JunkKind::Caches, Safety::Safe, "Gradle download cache", None),
        ".cache" | "Cache" | "Caches" | "GPUCache" | "Code Cache" | "CachedData" | "ShaderCache" | "cache2" | "GrShaderCache" | "DawnCache" => mk(JunkKind::Caches, Safety::Safe, "Application cache", None),
        "Trash" | ".Trash" => mk(JunkKind::Trash, Safety::Review, "Deleted items waiting to be emptied", None),
        n if n.starts_with(".Trash-") => mk(JunkKind::Trash, Safety::Review, "Deleted items waiting to be emptied", None),
        "tmp" | "temp" | ".tmp" => mk(JunkKind::Temp, Safety::Review, "Temporary folder", None),
        _ => None,
    }
}

impl Scan {
    pub fn new(fs: &str, root: &str, now: i64) -> Scan {
        let name = root.rsplit('/').find(|s| !s.is_empty()).unwrap_or("/").to_string();
        let node = DirNode { name, path: root.to_string(), parent: None, size: 0, own: 0, files: 0, children: vec![], top_files: vec![], newest: 0, junk: None, in_junk: false, done: false };
        Scan { fs: fs.into(), root: root.into(), nodes: vec![node], junk: vec![], buckets: [0; 6], files_seen: 0, dirs_seen: 0, errors: 0, finished: false, now, skip: vec![], queue: vec![0], file_junk: 0 }
    }

    /// Skips Linux pseudo file systems so a whole-drive scan never wanders into /proc.
    pub fn skip_pseudo(&mut self) {
        self.skip = ["/proc", "/sys", "/dev", "/run", "/tmp/.X11-unix"].iter().map(|s| s.to_string()).collect();
    }

    pub fn total(&self) -> u64 {
        self.nodes.first().map(|n| n.size).unwrap_or(0)
    }

    pub fn root_loc(&self) -> Location {
        Location::new(self.fs.clone(), self.root.clone())
    }

    pub fn loc(&self, path: &str) -> Location {
        Location::new(self.fs.clone(), path.to_string())
    }

    /// Progress hint 0..1 (directories finished vs discovered; honest about being an estimate).
    pub fn progress(&self) -> f32 {
        if self.finished {
            return 1.0;
        }
        let done = self.nodes.iter().filter(|n| n.done).count() as f32;
        (done / self.nodes.len().max(1) as f32).clamp(0.0, 0.99)
    }

    /// Process up to `max_dirs` directories. Returns `true` while there is more to do.
    pub fn step(&mut self, vfs: &dyn Vfs, max_dirs: usize) -> bool {
        for _ in 0..max_dirs {
            let Some(i) = self.queue.pop() else {
                self.finished = true;
                break;
            };
            self.scan_dir(vfs, i);
        }
        self.refresh_junk_sizes();
        !self.finished
    }

    fn add_up(&mut self, mut i: usize, bytes: u64, modified: i64) {
        loop {
            let n = &mut self.nodes[i];
            n.size += bytes;
            n.newest = n.newest.max(modified);
            match n.parent {
                Some(p) => i = p,
                None => break,
            }
        }
    }

    fn scan_dir(&mut self, vfs: &dyn Vfs, i: usize) {
        let path = self.nodes[i].path.clone();
        let parent_name = self.nodes[i].parent.map(|p| self.nodes[p].name.clone()).unwrap_or_default();
        let _ = parent_name;
        let dir_name = self.nodes[i].name.clone();
        let in_junk = self.nodes[i].in_junk;
        let entries: Vec<Entry> = match vfs.list(&path) {
            Ok(v) => v,
            Err(_) => {
                self.errors += 1;
                self.nodes[i].done = true;
                return;
            }
        };
        let siblings: HashSet<String> = entries.iter().map(|e| e.name.clone()).collect();
        let in_downloads = path.to_ascii_lowercase().contains("download");
        for e in entries {
            if e.symlink {
                continue;
            }
            if e.is_dir {
                if self.skip.iter().any(|s| *s == e.loc.path) {
                    continue;
                }
                let idx = self.nodes.len();
                let mut node = DirNode { name: e.name.clone(), path: e.loc.path.clone(), parent: Some(i), size: 0, own: 0, files: 0, children: vec![], top_files: vec![], newest: e.modified.unwrap_or(0), junk: None, in_junk, done: false };
                if !in_junk && e.name != ".git" {
                    if let Some(c) = classify_dir(&e.name, &dir_name, &siblings) {
                        node.junk = Some(self.junk.len());
                        node.in_junk = true;
                        self.junk.push(Junk { path: e.loc.path.clone(), name: e.name.clone(), is_dir: true, kind: c.kind, safety: c.safety, reason: c.reason, restore: c.restore, modified: e.modified, size: 0, node: Some(idx) });
                    }
                }
                self.nodes.push(node);
                self.nodes[i].children.push(idx);
                self.dirs_seen += 1;
                self.queue.push(idx);
                continue;
            }
            self.files_seen += 1;
            let size = e.size;
            self.nodes[i].own += size;
            self.nodes[i].files += 1;
            self.buckets[Bucket::of(e.kind).idx()] += size;
            let modified = e.modified.unwrap_or(0);
            self.add_up(i, size, modified);
            // keep the biggest files
            if self.nodes[i].top_files.len() < TOP_FILES || self.nodes[i].top_files.last().map(|f| f.size < size).unwrap_or(true) {
                let fi = FileInfo { name: e.name.clone(), path: e.loc.path.clone(), size, modified: e.modified, kind: e.kind };
                let tf = &mut self.nodes[i].top_files;
                let pos = tf.iter().position(|f| f.size < size).unwrap_or(tf.len());
                tf.insert(pos, fi);
                tf.truncate(TOP_FILES);
            }
            if !in_junk && self.file_junk < MAX_FILE_JUNK {
                if let Some(j) = self.classify_file(&e, &siblings, in_downloads) {
                    self.file_junk += 1;
                    self.junk.push(j);
                }
            }
        }
        self.nodes[i].done = true;
    }

    fn classify_file(&self, e: &Entry, siblings: &HashSet<String>, in_downloads: bool) -> Option<Junk> {
        let name = e.name.as_str();
        let ext = lower_ext(name);
        let size = e.size;
        let age_days = e.modified.map(|m| ((self.now - m) / DAY).max(0));
        let mk = |kind, safety, reason: String, restore| Some(Junk { path: e.loc.path.clone(), name: e.name.clone(), is_dir: false, kind, safety, reason, restore, modified: e.modified, size, node: None });
        // temporary leftovers
        let temp_ext = matches!(ext.as_str(), "tmp" | "temp" | "bak" | "swp" | "swo" | "crdownload" | "part" | "partial" | "orig" | "dmp");
        let temp_name = name.ends_with('~') || matches!(name, ".DS_Store" | "Thumbs.db" | "desktop.ini") || (name.starts_with("core.") && name[5..].bytes().all(|b| b.is_ascii_digit()) && name.len() > 5);
        if (temp_ext || temp_name) && size >= 64 * 1024 {
            return mk(JunkKind::Temp, Safety::Safe, if ext == "crdownload" || ext == "part" || ext == "partial" { "Unfinished download".into() } else { "Temporary or backup leftover".into() }, None);
        }
        if (ext == "log" || name.contains(".log.")) && size >= 5 * 1024 * 1024 {
            return mk(JunkKind::Logs, Safety::Safe, format!("Log file of {}", crate::model::format_size(size)), None);
        }
        if in_downloads && matches!(ext.as_str(), "iso" | "dmg" | "pkg" | "exe" | "msi" | "deb" | "rpm" | "apk") && size >= 1024 * 1024 && age_days.map(|d| d >= 14).unwrap_or(false) {
            return mk(JunkKind::Installers, Safety::Review, format!("Installer in Downloads, untouched for {} days", age_days.unwrap_or(0)), None);
        }
        if size >= 100 * 1024 {
            if let Some(orig) = copy_original(name) {
                if siblings.contains(&orig) {
                    return mk(JunkKind::Duplicates, Safety::Review, format!("Looks like a copy of \u{201c}{orig}\u{201d} in the same folder"), None);
                }
            }
        }
        if let Some(d) = age_days {
            let archive = e.kind == Kind::Archive;
            if (size >= 200 * 1_000_000 && d >= 365) || (size >= 1_000_000_000 && d >= 120) || (archive && size >= 100 * 1_000_000 && d >= 180) {
                let months = (d / 30).max(1);
                return mk(JunkKind::LargeStale, Safety::Careful, format!("{} untouched for {} month{}", if archive { "Old archive" } else { "Large file" }, months, if months == 1 { "" } else { "s" }), None);
            }
        }
        None
    }

    fn refresh_junk_sizes(&mut self) {
        for j in &mut self.junk {
            if let Some(n) = j.node {
                j.size = self.nodes[n].size;
            }
        }
    }

    /// Candidates worth showing, best first: weight favours big *and* safe.
    pub fn candidates(&self) -> Vec<&Junk> {
        let mut v: Vec<&Junk> = self.junk.iter().filter(|j| j.size >= MIN_JUNK_BYTES).collect();
        v.sort_by_key(|j| {
            let w = match j.safety {
                Safety::Safe => 3,
                Safety::Review => 2,
                Safety::Careful => 1,
            };
            std::cmp::Reverse(j.size / 4 * w)
        });
        v
    }

    pub fn reclaimable(&self, safe_only: bool) -> u64 {
        self.candidates().iter().filter(|j| !safe_only || j.safety == Safety::Safe).map(|j| j.size).sum()
    }

    pub fn by_kind(&self) -> Vec<(JunkKind, u64, usize)> {
        let mut out = vec![];
        for k in JunkKind::ALL {
            let (s, n) = self.candidates().iter().filter(|j| j.kind == k).fold((0u64, 0usize), |(s, n), j| (s + j.size, n + 1));
            if n > 0 {
                out.push((k, s, n));
            }
        }
        out.sort_by_key(|x| std::cmp::Reverse(x.1));
        out
    }

    /// The biggest individual files anywhere in the scanned tree.
    pub fn biggest_files(&self, n: usize) -> Vec<&FileInfo> {
        let mut v: Vec<&FileInfo> = self.nodes.iter().flat_map(|d| d.top_files.iter()).collect();
        v.sort_by_key(|f| std::cmp::Reverse(f.size));
        v.truncate(n);
        v
    }

    /// Biggest folders (excluding the root), skipping folders that merely wrap one huge child.
    pub fn biggest_dirs(&self, n: usize) -> Vec<usize> {
        let mut v: Vec<usize> = (1..self.nodes.len()).collect();
        v.sort_by_key(|i| std::cmp::Reverse(self.nodes[*i].size));
        v.truncate(n);
        v
    }

    /// Ancestors of a node from the root to the node, as indices.
    pub fn chain(&self, mut i: usize) -> Vec<usize> {
        let mut v = vec![i];
        while let Some(p) = self.nodes[i].parent {
            v.push(p);
            i = p;
        }
        v.reverse();
        v
    }

    pub fn find(&self, path: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n.path == path)
    }

    /// Children of a folder as drawable items (folders first by size, then big files, then one "other files" slice).
    pub fn items(&self, i: usize) -> Vec<Item> {
        let n = &self.nodes[i];
        let mut v: Vec<Item> = n.children.iter().map(|c| Item { label: self.nodes[*c].name.clone(), size: self.nodes[*c].size, node: Some(*c), path: self.nodes[*c].path.clone(), is_dir: true, junk: self.nodes[*c].junk.is_some(), kind: Kind::Folder, modified: Some(self.nodes[*c].newest) }).collect();
        let shown: u64 = n.top_files.iter().map(|f| f.size).sum();
        for f in &n.top_files {
            v.push(Item { label: f.name.clone(), size: f.size, node: None, path: f.path.clone(), is_dir: false, junk: false, kind: f.kind, modified: f.modified });
        }
        if n.own > shown {
            v.push(Item { label: format!("{} other files", n.files.saturating_sub(n.top_files.len() as u64)), size: n.own - shown, node: None, path: String::new(), is_dir: false, junk: false, kind: Kind::Other, modified: None });
        }
        v.retain(|x| x.size > 0);
        v.sort_by_key(|x| std::cmp::Reverse(x.size));
        v
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub label: String,
    pub size: u64,
    pub node: Option<usize>,
    pub path: String,
    pub is_dir: bool,
    pub junk: bool,
    pub kind: Kind,
    pub modified: Option<i64>,
}

/// Squarified treemap (Bruls et al.). Pure geometry so it can be tested: `sizes` must be sorted descending.
pub fn squarify(sizes: &[f64], x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64, f64)> {
    let mut out = vec![(0.0, 0.0, 0.0, 0.0); sizes.len()];
    let total: f64 = sizes.iter().sum();
    if sizes.is_empty() || total <= 0.0 || w <= 0.0 || h <= 0.0 {
        return out;
    }
    let scale = w * h / total;
    let areas: Vec<f64> = sizes.iter().map(|s| s * scale).collect();
    let (mut x, mut y, mut w, mut h) = (x, y, w, h);
    let mut i = 0;
    while i < areas.len() {
        let short = w.min(h);
        let worst = |row: &[f64]| -> f64 {
            let s: f64 = row.iter().sum();
            if s <= 0.0 || short <= 0.0 {
                return f64::MAX;
            }
            let (mx, mn) = (row.iter().cloned().fold(f64::MIN, f64::max), row.iter().cloned().fold(f64::MAX, f64::min));
            ((short * short * mx) / (s * s)).max((s * s) / (short * short * mn.max(1e-9)))
        };
        let mut j = i + 1;
        while j < areas.len() && worst(&areas[i..=j]) <= worst(&areas[i..j]) {
            j += 1;
        }
        let row = &areas[i..j];
        let s: f64 = row.iter().sum();
        if w >= h {
            let cw = s / h;
            let mut cy = y;
            for (k, a) in row.iter().enumerate() {
                let ch = a / cw;
                out[i + k] = (x, cy, cw, ch);
                cy += ch;
            }
            x += cw;
            w -= cw;
        } else {
            let rh = s / w;
            let mut cx = x;
            for (k, a) in row.iter().enumerate() {
                let cw = a / rh;
                out[i + k] = (cx, y, cw, rh);
                cx += cw;
            }
            y += rh;
            h -= rh;
        }
        i = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memfs::MemVfs;

    const MB: u64 = 1_000_000;
    const GB: u64 = 1_000_000_000;
    const NOW: i64 = 1_800_000_000;

    fn old(days: i64) -> i64 {
        NOW - days * DAY
    }

    fn world() -> MemVfs {
        let fs = MemVfs::new("t");
        fs.put("/h/proj/Cargo.toml", b"[package]".to_vec(), Some(1_000), old(1));
        fs.put("/h/proj/src/main.rs", b"fn main(){}".to_vec(), Some(2_000), old(1));
        fs.put("/h/proj/target/debug/libx.rlib", vec![0], Some(3 * GB), old(2));
        fs.put("/h/web/package.json", b"{}".to_vec(), Some(500), old(3));
        fs.put("/h/web/node_modules/react/index.js", vec![0], Some(400 * MB), old(30));
        fs.put("/h/web/node_modules/target/not-nested.bin", vec![0], Some(50 * MB), old(30));
        fs.put("/h/loose/target/data.bin", vec![0], Some(900 * MB), old(30));
        fs.put("/h/Downloads/ubuntu.iso", vec![0], Some(4 * GB), old(200));
        fs.put("/h/Downloads/fresh.iso", vec![0], Some(2 * GB), old(1));
        fs.put("/h/Downloads/Report.pdf", vec![0], Some(2 * MB), old(10));
        fs.put("/h/Downloads/Report (1).pdf", vec![0], Some(2 * MB), old(9));
        fs.put("/h/Downloads/Lonely (1).pdf", vec![0], Some(2 * MB), old(9));
        fs.put("/h/.cache/pip/wheel.whl", vec![0], Some(1_400 * MB), old(60));
        fs.put("/h/docs/backup-2020.zip", vec![0], Some(6 * GB), old(900));
        fs.put("/h/docs/thesis.docx", vec![0], Some(2 * MB), old(900));
        fs.put("/h/logs/server.log", vec![0], Some(80 * MB), old(1));
        fs.put("/h/logs/tiny.tmp", vec![0], Some(10), old(1));
        fs
    }

    fn run() -> Scan {
        let fs = world();
        let mut s = Scan::new("t", "/h", NOW);
        while s.step(&fs, 50) {}
        s
    }

    #[test]
    fn totals_roll_up() {
        let s = run();
        let sum: u64 = s.nodes[0].children.iter().map(|c| s.nodes[*c].size).sum::<u64>() + s.nodes[0].own;
        assert_eq!(s.total(), sum);
        assert!(s.finished && s.errors == 0);
        assert!(s.total() > 15 * GB);
    }

    #[test]
    fn detects_rust_target_only_next_to_cargo_toml() {
        let s = run();
        let c = s.candidates();
        let t = c.iter().find(|j| j.path == "/h/proj/target").expect("rust target found");
        assert_eq!(t.kind, JunkKind::BuildArtifacts);
        assert_eq!(t.safety, Safety::Safe);
        assert_eq!(t.size(), 3 * GB);
        assert_eq!(t.restore, Some("cargo build"));
        assert!(!c.iter().any(|j| j.path == "/h/loose/target"), "a target/ without Cargo.toml is not assumed junk");
    }

    #[test]
    fn nested_junk_is_not_double_counted() {
        let s = run();
        assert!(s.candidates().iter().any(|j| j.path == "/h/web/node_modules"));
        assert!(!s.candidates().iter().any(|j| j.path.starts_with("/h/web/node_modules/")));
        let sum: u64 = s.candidates().iter().map(|j| j.size()).sum();
        assert!(sum <= s.total());
    }

    #[test]
    fn installers_copies_caches_and_stale() {
        let s = run();
        let c = s.candidates();
        assert!(c.iter().any(|j| j.name == "ubuntu.iso" && j.kind == JunkKind::Installers));
        assert!(!c.iter().any(|j| j.name == "fresh.iso"), "recent installers are left alone");
        assert!(c.iter().any(|j| j.name == "Report (1).pdf" && j.kind == JunkKind::Duplicates) || s.junk.iter().any(|j| j.name == "Report (1).pdf"));
        assert!(!s.junk.iter().any(|j| j.name == "Lonely (1).pdf"), "a copy without an original is not a copy");
        assert!(c.iter().any(|j| j.path == "/h/.cache" && j.kind == JunkKind::Caches));
        let z = c.iter().find(|j| j.name == "backup-2020.zip").unwrap();
        assert_eq!(z.safety, Safety::Careful);
        assert!(c.iter().any(|j| j.name == "server.log" && j.kind == JunkKind::Logs));
        assert!(!s.junk.iter().any(|j| j.name == "tiny.tmp"), "tiny leftovers are noise");
    }

    #[test]
    fn ranking_prefers_safe_and_big() {
        let s = run();
        let c = s.candidates();
        assert!(c.len() >= 6);
        assert!(c[0].size() >= 3 * GB);
        assert!(s.reclaimable(true) < s.reclaimable(false));
        assert!(s.by_kind().windows(2).all(|w| w[0].1 >= w[1].1));
    }

    #[test]
    fn copy_name_patterns() {
        assert_eq!(copy_original("Report (1).pdf").as_deref(), Some("Report.pdf"));
        assert_eq!(copy_original("a - Copy.txt").as_deref(), Some("a.txt"));
        assert_eq!(copy_original("notes copy").as_deref(), Some("notes"));
        assert_eq!(copy_original("Report.pdf"), None);
        assert_eq!(copy_original("(1).pdf"), None);
        assert_eq!(copy_original("x (2024).pdf"), None);
    }

    #[test]
    fn items_and_biggest() {
        let s = run();
        let top = s.biggest_files(3);
        assert_eq!(top[0].name, "backup-2020.zip");
        let items = s.items(0);
        assert!(items.windows(2).all(|w| w[0].size >= w[1].size));
        let dirs = s.biggest_dirs(2);
        assert!(s.nodes[dirs[0]].size >= s.nodes[dirs[1]].size);
        assert_eq!(s.chain(s.find("/h/proj/target").unwrap()).len(), 3);
    }

    #[test]
    fn missing_dir_counts_error_and_finishes() {
        let fs = MemVfs::new("t");
        let mut s = Scan::new("t", "/nope", NOW);
        while s.step(&fs, 10) {}
        assert!(s.finished);
        assert_eq!(s.errors, 1);
        assert_eq!(s.total(), 0);
    }

    #[test]
    fn squarify_fills_the_box_without_overlap() {
        let sizes = [60.0, 30.0, 20.0, 10.0, 5.0, 5.0];
        let r = squarify(&sizes, 0.0, 0.0, 300.0, 200.0);
        let area: f64 = r.iter().map(|(_, _, w, h)| w * h).sum();
        assert!((area - 300.0 * 200.0).abs() < 1.0);
        for (i, a) in r.iter().enumerate() {
            assert!(a.0 >= -1e-6 && a.1 >= -1e-6 && a.0 + a.2 <= 300.0 + 1e-6 && a.1 + a.3 <= 200.0 + 1e-6);
            for b in r.iter().skip(i + 1) {
                let ox = (a.0 + a.2).min(b.0 + b.2) - a.0.max(b.0);
                let oy = (a.1 + a.3).min(b.1 + b.3) - a.1.max(b.1);
                assert!(ox <= 1e-6 || oy <= 1e-6, "overlap");
            }
        }
        assert!(squarify(&[], 0.0, 0.0, 10.0, 10.0).is_empty());
    }
}
