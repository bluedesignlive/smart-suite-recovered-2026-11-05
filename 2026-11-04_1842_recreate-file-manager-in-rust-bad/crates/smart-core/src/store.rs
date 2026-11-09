//! `Store` — the UI-agnostic application model. A frontend renders `Store` and sends `Action`s.
//! Toolbars, context menus and the command palette are *computed here* from state, so every
//! frontend (egui, gpui, native widgets…) gets identical, context-aware behaviour.
use crate::ai::*;
use crate::archive::{self, Format};
use crate::engine::*;
use crate::model::*;
use crate::search::{self, Hit, Query};
use crate::vfs::MemFs;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher, Utf32Str,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Folder(Path),
    Photos,
    Recents,
    Tag(String),
    Trash,
    Search(String),
    Storage,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortKey {
    Name,
    Kind,
    Modified,
    Size,
}
impl SortKey {
    pub const ALL: [SortKey; 4] = [SortKey::Name, SortKey::Kind, SortKey::Modified, SortKey::Size];
    pub fn label(self) -> &'static str {
        match self {
            SortKey::Name => "Name",
            SortKey::Kind => "Kind",
            SortKey::Modified => "Modified",
            SortKey::Size => "Size",
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    Grid,
    List,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Light,
    Dark,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rail {
    All,
    Go,
    Files,
    Make,
    Look,
    Ask,
}
impl Rail {
    pub const ALL: [Rail; 6] = [Rail::All, Rail::Go, Rail::Files, Rail::Make, Rail::Look, Rail::Ask];
    pub fn label(self) -> &'static str {
        match self {
            Rail::All => "All",
            Rail::Go => "Go",
            Rail::Files => "Files",
            Rail::Make => "Make",
            Rail::Look => "Look",
            Rail::Ask => "Ask",
        }
    }
    pub fn icon(self) -> Icon {
        match self {
            Rail::All => Icon::Command,
            Rail::Go => Icon::Forward,
            Rail::Files => Icon::Folder,
            Rail::Make => Icon::Plus,
            Rail::Look => Icon::Sliders,
            Rail::Ask => Icon::Sparkle,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Noop,
    Go(Route),
    GoPath(Path),
    Back,
    Forward,
    Up,
    Refresh,
    Open(Path),
    OpenSelected,
    OpenWith(Path, String),
    OpenWithDialog(Path),
    QuickLook(Option<Path>),
    Select { path: Path, ctrl: bool, shift: bool },
    SelectAll,
    SelectNone,
    InvertSelection,
    BeginRename(Option<Path>),
    CommitRename,
    CancelRename,
    NewFolder,
    NewFile(&'static str),
    Copy,
    Cut,
    Paste,
    PasteInto(Path),
    Duplicate,
    Trash,
    AskDelete,
    ConfirmDelete(Vec<Path>),
    Restore,
    EmptyTrash,
    Undo,
    SetSort(SortKey),
    ToggleSortDir,
    SetView(ViewMode),
    ToggleHidden,
    Tag(Option<String>),
    CompressDialog,
    CompressNow { name: String, fmt: Format },
    ExtractHere,
    Properties(Option<Path>),
    CopyPath,
    OpenTerminal,
    ToggleSidebar,
    ToggleAi,
    Palette(Option<Rail>),
    ToggleTheme,
    Search(String),
    SearchEverywhere(bool),
    Unmount(String),
    Mount(String),
    CloseModal,
    ShowShortcuts,
    ShowAbout,
    AddCloudDialog,
    AddCloud,
    MoveTo(Vec<Path>, Path),
    CopyTo(Vec<Path>, Path),
    JobPause(u64),
    JobCancel(u64),
    JobMinimize(u64),
    JobResolve { id: u64, r: Resolution, all: bool },
    AiAsk(String),
    AiApply(usize),
    CleanToggle(usize, usize),
    CleanToggleCat(usize),
    CleanExpand(usize),
    CleanApply,
    Rescan,
    LensGo(Path),
    PlayToggle(Path),
    CreateDemo,
    SetPace(Pace),
    FocusSearch,
    Terminal,
}

#[derive(Clone, Debug)]
pub enum Modal {
    Terminal,
    Properties(Path),
    Compress { paths: Vec<Path>, name: String, fmt: Format },
    OpenWith(Path),
    Delete(Vec<Path>),
    Shortcuts,
    About,
    AddCloud { provider: usize, account: String },
}

pub const CLOUD_PROVIDERS: [&str; 6] = ["Smart Cloud", "Google Drive", "Dropbox", "OneDrive", "S3 bucket", "WebDAV · Nextcloud"];

#[derive(Clone, Debug)]
pub struct RenameState {
    pub path: Path,
    pub text: String,
    pub fresh: bool,
}
#[derive(Clone, Debug)]
pub struct Toast {
    pub id: u64,
    pub text: String,
    pub icon: Icon,
    pub good: bool,
    pub action: Option<(String, Action)>,
    pub ttl: f32,
    pub age: f32,
}
#[derive(Clone, Debug)]
pub struct Palette {
    pub open: bool,
    pub rail: Rail,
    pub query: String,
    pub sel: usize,
}
#[derive(Clone, Debug)]
pub struct Cmd {
    pub title: String,
    pub sub: String,
    pub icon: Icon,
    pub action: Action,
    pub rail: Rail,
    pub group: &'static str,
    pub shortcut: &'static str,
    pub path: Option<Path>,
    pub indices: Vec<u32>,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tone {
    Plain,
    Accent,
    Danger,
}
#[derive(Clone, Debug)]
pub struct Tool {
    pub label: String,
    pub icon: Icon,
    pub action: Action,
    pub tone: Tone,
    pub tip: String,
}
#[derive(Clone, Debug)]
pub struct MenuItem {
    pub label: String,
    pub icon: Icon,
    pub action: Action,
    pub shortcut: &'static str,
    pub danger: bool,
    pub enabled: bool,
    pub checked: bool,
    pub swatch: Option<[u8; 3]>,
}
#[derive(Clone, Debug)]
pub enum MenuEntry {
    Item(MenuItem),
    Sep,
    Sub(String, Icon, Vec<MenuEntry>),
}
fn mi(label: impl Into<String>, icon: Icon, action: Action) -> MenuItem {
    MenuItem { label: label.into(), icon, action, shortcut: "", danger: false, enabled: true, checked: false, swatch: None }
}
impl MenuItem {
    fn sc(mut self, s: &'static str) -> Self {
        self.shortcut = s;
        self
    }
    fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
    fn on(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }
    fn check(mut self, v: bool) -> Self {
        self.checked = v;
        self
    }
    fn swatch(mut self, c: [u8; 3]) -> Self {
        self.swatch = Some(c);
        self
    }
    pub fn e(self) -> MenuEntry {
        MenuEntry::Item(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerCmd {
    Start(Path),
    Pause,
    Resume,
    Stop,
}
#[derive(Clone, Debug, Default)]
pub struct Player {
    pub path: Option<Path>,
    pub playing: bool,
    pub pos: f64,
    pub dur: f64,
    pub cmd: Option<PlayerCmd>,
}

pub struct Group {
    pub title: String,
    pub idx: Vec<usize>,
}

pub struct StorageState {
    pub report: CleanReport,
    pub lens_root: Path,
    pub lens: Vec<LensItem>,
    pub last_freed: Option<u64>,
}

pub struct Store {
    pub eng: Engine,
    pub route: Route,
    back: Vec<Route>,
    fwd: Vec<Route>,
    pub items: Vec<Entry>,
    pub match_idx: HashMap<Path, Vec<u32>>,
    pub load_error: Option<String>,
    pub sel: Vec<Path>,
    pub anchor: Option<Path>,
    pub sort: SortKey,
    pub sort_asc: bool,
    pub view: ViewMode,
    pub theme: Theme,
    pub sidebar_open: bool,
    pub ai_open: bool,
    pub clip: Option<(Vec<Path>, bool)>,
    pub rename: Option<RenameState>,
    pub modal: Option<Modal>,
    pub quicklook: Option<Path>,
    pub palette: Palette,
    pub toasts: Vec<Toast>,
    pub msgs: Vec<AiMsg>,
    pub ai_pending: Option<(f64, AiMsg)>,
    pub search_text: String,
    pub search_everywhere: bool,
    search_origin: Option<Route>,
    pub query: Query,
    pub storage: StorageState,
    pub player: Player,
    pub focus_search: bool,
    pub place_counts: HashMap<Path, usize>,
    seen_rev: u64,
    next_toast: u64,
    pub scan_prev: usize,
}

fn nat_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase().cmp(&b.to_lowercase())
}

impl Store {
    pub fn new(eng: Engine, start_in_pictures: bool) -> Store {
        let home = eng.home.clone();
        let pics = join(&home, "Pictures");
        let start = if start_in_pictures && eng.exists(&pics) { pics } else { home.clone() };
        let mut s = Store {
            route: Route::Folder(start),
            back: vec![],
            fwd: vec![],
            items: vec![],
            match_idx: HashMap::new(),
            load_error: None,
            sel: vec![],
            anchor: None,
            sort: SortKey::Name,
            sort_asc: true,
            view: ViewMode::Grid,
            theme: Theme::Light,
            sidebar_open: true,
            ai_open: false,
            clip: None,
            rename: None,
            modal: None,
            quicklook: None,
            palette: Palette { open: false, rail: Rail::All, query: String::new(), sel: 0 },
            toasts: vec![],
            msgs: vec![],
            ai_pending: None,
            search_text: String::new(),
            search_everywhere: false,
            search_origin: None,
            query: Query::default(),
            storage: StorageState { report: CleanReport::default(), lens_root: home.clone(), lens: vec![], last_freed: None },
            player: Player::default(),
            focus_search: false,
            place_counts: HashMap::new(),
            seen_rev: 0,
            next_toast: 1,
            scan_prev: 0,
            eng,
        };
        s.refresh();
        s.msgs.push(s.ai_intro());
        s
    }

    // ───────────── derived state ─────────────
    pub fn current_folder(&self) -> Path {
        match &self.route {
            Route::Folder(p) => p.clone(),
            Route::Trash => self.eng.trash_dir.clone().unwrap_or_else(|| self.eng.home.clone()),
            Route::Search(_) => match &self.search_origin {
                Some(Route::Folder(p)) => p.clone(),
                _ => self.eng.home.clone(),
            },
            _ => self.eng.home.clone(),
        }
    }
    pub fn search_origin_is_none(&self) -> bool {
        !matches!(self.search_origin, Some(Route::Folder(_)))
    }
    pub fn in_folder(&self) -> bool {
        matches!(self.route, Route::Folder(_))
    }
    pub fn title(&self) -> String {
        match &self.route {
            Route::Folder(p) => {
                if p == &self.eng.home {
                    "Home".into()
                } else if let Some(m) = self.eng.mounts.iter().find(|m| &m.root == p) {
                    m.label.clone()
                } else {
                    name_of(p).to_string()
                }
            }
            Route::Photos => "Photos".into(),
            Route::Recents => "Recents".into(),
            Route::Tag(t) => tag_def(t).map(|d| d.label.to_string()).unwrap_or_default(),
            Route::Trash => "Trash".into(),
            Route::Search(_) => "Search".into(),
            Route::Storage => "Storage".into(),
        }
    }
    pub fn route_icon(&self) -> Icon {
        match &self.route {
            Route::Folder(_) => Icon::Folder,
            Route::Photos => Icon::Image,
            Route::Recents => Icon::Clock,
            Route::Tag(_) => Icon::Tag,
            Route::Trash => Icon::Trash,
            Route::Search(_) => Icon::Search,
            Route::Storage => Icon::Pie,
        }
    }
    pub fn can_back(&self) -> bool {
        !self.back.is_empty()
    }
    pub fn can_forward(&self) -> bool {
        !self.fwd.is_empty()
    }
    pub fn breadcrumb(&self) -> Vec<(String, Route)> {
        match &self.route {
            Route::Folder(p) => {
                let (base_label, base) = if is_within(p, &self.eng.home) {
                    ("Home".to_string(), self.eng.home.clone())
                } else if let Some(m) = self.eng.mount_of(p) {
                    (m.label.clone(), m.root.clone())
                } else {
                    ("Computer".into(), "/".into())
                };
                let mut out = vec![(base_label, Route::Folder(base.clone()))];
                let mut cur = base.clone();
                for seg in p[base.len().min(p.len())..].split('/').filter(|s| !s.is_empty()) {
                    cur = join(&cur, seg);
                    out.push((seg.to_string(), Route::Folder(cur.clone())));
                }
                out
            }
            Route::Search(q) => vec![("Search".into(), Route::Search(q.clone()))],
            _ => vec![(self.title(), self.route.clone())],
        }
    }
    pub fn entry(&self, p: &str) -> Option<&Entry> {
        self.items.iter().find(|e| e.path == p)
    }
    pub fn selected_entries(&self) -> Vec<Entry> {
        self.sel.iter().filter_map(|p| self.entry(p).cloned()).collect()
    }
    pub fn is_selected(&self, p: &str) -> bool {
        self.sel.iter().any(|s| s == p)
    }
    pub fn tag_of(&self, p: &str) -> Option<&'static TagDef> {
        self.eng.tags.get(p).and_then(|t| tag_def(t))
    }
    pub fn tag_count(&self, id: &str) -> usize {
        self.eng.tags.values().filter(|t| *t == id).count()
    }
    pub fn total_size(&self) -> u64 {
        self.items.iter().filter(|e| !e.is_dir).map(|e| e.size).sum()
    }
    pub fn groups(&self) -> Vec<Group> {
        if matches!(self.route, Route::Storage) {
            return vec![];
        }
        let grouped = matches!(self.sort, SortKey::Name | SortKey::Kind) && !matches!(self.route, Route::Search(_) | Route::Recents);
        if !grouped {
            let title = match self.route {
                Route::Search(_) => "Results",
                Route::Recents => "Recently modified",
                _ => "Items",
            };
            return vec![Group { title: title.into(), idx: (0..self.items.len()).collect() }];
        }
        let mut out: Vec<Group> = vec![];
        for k in [Kind::Folder, Kind::Image, Kind::Video, Kind::Audio, Kind::Document, Kind::Text, Kind::Code, Kind::Archive, Kind::App, Kind::Data, Kind::Other] {
            let idx: Vec<usize> = self.items.iter().enumerate().filter(|(_, e)| e.kind == k).map(|(i, _)| i).collect();
            if !idx.is_empty() {
                out.push(Group { title: k.plural().into(), idx });
            }
        }
        out
    }
    pub fn index_of(&self, p: &str) -> Option<usize> {
        self.items.iter().position(|e| e.path == p)
    }
    pub fn run_query(&self, q: &Query, limit: usize) -> Vec<Hit> {
        search::run(&self.eng.index, q, &|p| self.eng.tags.get(p).cloned(), &|p| self.eng.frecency(p), limit)
    }
    pub fn active_jobs(&self) -> Vec<&Job> {
        self.eng.jobs.iter().filter(|j| j.active()).collect()
    }
    pub fn transfer_job(&self) -> Option<&Job> {
        self.eng.jobs.iter().filter(|j| !j.minimized).find(|j| j.active() || (j.finished_at > 0.0 && self.eng.clock - j.finished_at < 2.2 && j.state == JobState::Done && (j.throttled || j.elapsed > 0.6)))
    }
    pub fn dests(&self) -> Vec<(String, Icon, Path)> {
        let h = &self.eng.home;
        let mut v = vec![("Home".to_string(), Icon::Home, h.clone())];
        for (n, i) in [("Desktop", Icon::Monitor), ("Documents", Icon::Doc), ("Downloads", Icon::Download), ("Pictures", Icon::Image)] {
            let p = join(h, n);
            if self.eng.exists(&p) {
                v.push((n.to_string(), i, p));
            }
        }
        for m in self.eng.mounts.iter().filter(|m| m.mounted && &m.root != "/") {
            v.push((m.label.clone(), m.icon(), m.root.clone()));
        }
        v
    }

    // ───────────── listing ─────────────
    pub fn refresh(&mut self) {
        self.seen_rev = self.eng.revision;
        self.load_error = None;
        self.match_idx.clear();
        let mut items: Vec<Entry> = vec![];
        match self.route.clone() {
            Route::Folder(p) => match self.eng.list(&p) {
                Ok(l) => items = l,
                Err(e) => self.load_error = Some(e),
            },
            Route::Photos => {
                let mut v: Vec<&search::IndexEntry> = self.eng.index.entries.iter().filter(|e| e.kind == Kind::Image && !e.is_dir).collect();
                v.sort_by_key(|e| std::cmp::Reverse(e.modified));
                items = v.into_iter().take(300).map(|e| Entry::new(&e.path, false, e.size, e.modified)).collect();
            }
            Route::Trash => match self.eng.trash_dir.clone() {
                Some(t) => items = self.eng.list(&t).unwrap_or_default(),
                None => self.load_error = Some("Trash isn’t available on this system".into()),
            },
            Route::Recents => {
                let mut v: Vec<&search::IndexEntry> = self.eng.index.entries.iter().filter(|e| !e.is_dir && !e.path.contains("/.")).collect();
                v.sort_by_key(|e| std::cmp::Reverse(e.modified));
                items = v.into_iter().take(60).map(|e| Entry::new(&e.path, false, e.size, e.modified)).collect();
            }
            Route::Tag(t) => {
                let mut paths: Vec<&Path> = self.eng.tags.iter().filter(|(_, v)| **v == t).map(|(k, _)| k).collect();
                paths.sort();
                items = paths.into_iter().filter_map(|p| self.eng.stat(p).ok()).collect();
            }
            Route::Search(raw) => {
                let q = search::parse(&raw);
                let scope = if self.search_everywhere { None } else { self.search_origin.as_ref().and_then(|r| if let Route::Folder(p) = r { Some(p.clone()) } else { None }) };
                let hits = self.run_query(&q, if scope.is_some() || q.content.is_some() { 4000 } else { 400 });
                let mut n = 0;
                for h in hits {
                    let ie = &self.eng.index.entries[h.idx];
                    if let Some(s) = &scope {
                        if !is_within(&ie.path, s) {
                            continue;
                        }
                    }
                    if let Some(c) = &q.content {
                        if ie.is_dir || !matches!(ie.kind, Kind::Text | Kind::Code) || ie.size > 2_000_000 {
                            continue;
                        }
                        n += 1;
                        if n > 800 {
                            break;
                        }
                        let ok = self.eng.read_all(&ie.path, 2_000_000).map(|b| String::from_utf8_lossy(&b).to_lowercase().contains(&c.to_lowercase())).unwrap_or(false);
                        if !ok {
                            continue;
                        }
                    }
                    let mut e = Entry::new(&ie.path, ie.is_dir, ie.size, ie.modified);
                    e.size = ie.size;
                    self.match_idx.insert(e.path.clone(), h.indices.clone());
                    items.push(e);
                    if items.len() >= 300 {
                        break;
                    }
                }
                self.query = q;
            }
            Route::Storage => {
                self.storage.report = analyze(&self.eng);
                self.storage.lens = lens(&self.eng, &self.storage.lens_root);
                self.scan_prev = self.eng.index.entries.len();
            }
        }
        if !self.eng.settings.show_hidden {
            items.retain(|e| !e.hidden || matches!(self.route, Route::Trash | Route::Tag(_)));
        }
        if !matches!(self.route, Route::Search(_) | Route::Recents) {
            self.sort_items(&mut items);
        }
        self.items = items;
        let alive: Vec<Path> = self.sel.iter().filter(|p| self.items.iter().any(|e| &e.path == *p)).cloned().collect();
        self.sel = alive;
        self.place_counts.insert(self.current_folder(), self.items.len());
    }
    fn sort_items(&self, items: &mut [Entry]) {
        let asc = self.sort_asc;
        let key = self.sort;
        items.sort_by(|a, b| {
            let fd = b.is_dir.cmp(&a.is_dir);
            if fd != std::cmp::Ordering::Equal {
                return fd;
            }
            let o = match key {
                SortKey::Name => nat_cmp(&a.name, &b.name),
                SortKey::Kind => a.kind.cmp(&b.kind).then(nat_cmp(&a.name, &b.name)),
                SortKey::Modified => b.modified.cmp(&a.modified),
                SortKey::Size => b.size.cmp(&a.size),
            };
            if asc { o } else { o.reverse() }
        });
    }

    pub fn toast(&mut self, text: impl Into<String>, icon: Icon, good: bool, action: Option<(String, Action)>) {
        let id = self.next_toast;
        self.next_toast += 1;
        self.toasts.push(Toast { id, text: text.into(), icon, good, action, ttl: if matches!(icon, Icon::Warning) { 6.0 } else { 4.2 }, age: 0.0 });
        if self.toasts.len() > 4 {
            self.toasts.remove(0);
        }
    }
    fn err(&mut self, e: impl Into<String>) {
        self.toast(e, Icon::Warning, false, None);
    }

    pub fn tick(&mut self, dt: f64) {
        self.eng.tick(dt);
        for n in std::mem::take(&mut self.eng.notices) {
            let undo = if n.good && !self.eng.undo.is_empty() { Some(("Undo".to_string(), Action::Undo)) } else { None };
            self.toast(n.text, n.icon, n.good, undo);
        }
        if self.eng.revision != self.seen_rev {
            self.refresh();
        }
        if matches!(self.route, Route::Storage) && !self.eng.index.ready && self.eng.index.entries.len() > self.scan_prev + 400 {
            self.refresh();
        }
        for t in self.toasts.iter_mut() {
            t.age += dt as f32;
        }
        self.toasts.retain(|t| t.age < t.ttl);
        if let Some((at, _)) = &self.ai_pending {
            if self.eng.clock >= *at {
                if let Some((_, m)) = self.ai_pending.take() {
                    self.msgs.push(m);
                }
            }
        }
        if self.player.playing {
            self.player.pos += dt;
            if self.player.dur > 0.0 && self.player.pos >= self.player.dur {
                self.player.playing = false;
                self.player.pos = 0.0;
                self.player.cmd = Some(PlayerCmd::Stop);
            }
        }
    }

    // ───────────── navigation ─────────────
    fn navigate(&mut self, r: Route, push: bool) {
        if r == self.route {
            return self.refresh();
        }
        if push {
            self.back.push(self.route.clone());
            self.fwd.clear();
        }
        if !matches!(r, Route::Search(_)) {
            self.search_text.clear();
            self.search_origin = None;
        }
        self.route = r;
        self.sel.clear();
        self.anchor = None;
        self.rename = None;
        if matches!(self.route, Route::Storage) {
            self.storage.lens_root = self.eng.home.clone();
            self.storage.last_freed = None;
        }
        self.refresh();
    }

    pub fn audio_duration(&self, p: &str) -> f64 {
        if let Ok(h) = self.eng.read_head(p, 64) {
            if h.len() >= 44 && &h[0..4] == b"RIFF" {
                let rate = u32::from_le_bytes([h[24], h[25], h[26], h[27]]) as f64;
                let data = u32::from_le_bytes([h[40], h[41], h[42], h[43]]) as f64;
                if rate > 0.0 {
                    return data / (rate * 2.0);
                }
            }
        }
        184.0
    }
    pub fn wave_peaks(&self, p: &str, n: usize) -> Vec<f32> {
        let mut out = vec![];
        if let Ok(b) = self.eng.read_all(p, 4_000_000) {
            if b.len() > 64 && &b[0..4] == b"RIFF" {
                let samples: Vec<i16> = b[44..].chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect();
                if !samples.is_empty() {
                    let step = (samples.len() / n).max(1);
                    for c in samples.chunks(step).take(n) {
                        out.push(c.iter().map(|s| (*s as f32 / 32768.0).abs()).fold(0.0, f32::max));
                    }
                    let mx = out.iter().cloned().fold(0.01, f32::max);
                    return out.into_iter().map(|v| (v / mx).clamp(0.05, 1.0)).collect();
                }
            }
        }
        let mut h: u32 = p.bytes().fold(2166136261, |a, b| (a ^ b as u32).wrapping_mul(16777619));
        for i in 0..n {
            h = h.wrapping_mul(1664525).wrapping_add(1013904223);
            let env = ((i as f32 / n as f32) * std::f32::consts::PI).sin().powf(0.6);
            out.push((0.2 + 0.8 * ((h >> 8) as f32 / 16777216.0)) * env.max(0.15));
        }
        out
    }

    fn sel_paths(&self) -> Vec<Path> {
        self.sel.clone()
    }
    fn only_sel(&self) -> Option<Entry> {
        if self.sel.len() == 1 { self.entry(&self.sel[0]).cloned() } else { None }
    }

    // ───────────── dispatch ─────────────
    pub fn dispatch(&mut self, a: Action) {
        match a {
            Action::Noop => {}
            Action::Go(r) => {
                self.palette.open = false;
                self.navigate(r, true)
            }
            Action::GoPath(p) => self.navigate(Route::Folder(p), true),
            Action::Back => {
                if let Some(r) = self.back.pop() {
                    self.fwd.push(self.route.clone());
                    self.navigate(r, false);
                }
            }
            Action::Forward => {
                if let Some(r) = self.fwd.pop() {
                    self.back.push(self.route.clone());
                    self.navigate(r, false);
                }
            }
            Action::Up => {
                if let Route::Folder(p) = &self.route {
                    let up = parent(p);
                    let is_root = self.eng.mounts.iter().any(|m| &m.root == p) || p == "/";
                    if !is_root && &up != p {
                        self.navigate(Route::Folder(up), true);
                    }
                }
            }
            Action::Refresh => {
                self.refresh();
            }
            Action::Open(p) => {
                self.palette.open = false;
                match self.eng.stat(&p) {
                    Ok(e) if e.is_dir => self.navigate(Route::Folder(p), true),
                    Ok(e) => {
                        // .app launchers on the Desktop open in-app tools
                        if e.ext == "app" {
                            self.eng.bump_open(&p);
                            match stem_of(&e.name) {
                                "Photos" => self.navigate(Route::Photos, true),
                                "Terminal" => self.modal = Some(Modal::Terminal),
                                _ => self.quicklook = Some(p),
                            }
                            return;
                        }
                        if archive::is_archive(&e.name) && !self.eng.native {
                            self.quicklook = Some(p);
                        } else {
                            match self.eng.open_default(&p) {
                                Ok(()) => self.toast(format!("Opening “{}”", e.name), Icon::OpenExt, true, None),
                                Err(_) => self.quicklook = Some(p),
                            }
                        }
                    }
                    Err(e) => self.err(e),
                }
            }
            Action::OpenSelected => {
                if let Some(p) = self.sel.first().cloned() {
                    self.dispatch(Action::Open(p));
                }
            }
            Action::OpenWith(p, app) => {
                self.modal = None;
                if let Ok(e) = self.eng.stat(&p) {
                    if let Some(info) = self.eng.apps_for(&e).into_iter().find(|a| a.id == app) {
                        match self.eng.launch(&info, &p) {
                            Ok(()) => self.toast(format!("Opening in {}", info.name), Icon::OpenExt, true, None),
                            Err(_) => self.quicklook = Some(p),
                        }
                    }
                }
            }
            Action::OpenWithDialog(p) => self.modal = Some(Modal::OpenWith(p)),
            Action::QuickLook(p) => {
                if self.quicklook.is_some() && p.is_none() {
                    self.quicklook = None;
                    if self.player.playing {
                        self.player.playing = false;
                        self.player.cmd = Some(PlayerCmd::Stop);
                    }
                } else {
                    let t = p.or_else(|| self.sel.first().cloned());
                    if let Some(t) = t {
                        if self.entry(&t).map(|e| !e.is_dir).unwrap_or(true) {
                            self.quicklook = Some(t);
                        }
                    }
                }
            }
            Action::Select { path, ctrl, shift } => {
                if shift && self.anchor.is_some() {
                    let (a, b) = (self.index_of(self.anchor.as_ref().unwrap()), self.index_of(&path));
                    if let (Some(a), Some(b)) = (a, b) {
                        let (lo, hi) = (a.min(b), a.max(b));
                        self.sel = self.items[lo..=hi].iter().map(|e| e.path.clone()).collect();
                    }
                } else if ctrl {
                    if let Some(i) = self.sel.iter().position(|s| *s == path) {
                        self.sel.remove(i);
                    } else {
                        self.sel.push(path.clone());
                    }
                    self.anchor = Some(path);
                } else {
                    self.sel = vec![path.clone()];
                    self.anchor = Some(path);
                }
                if self.rename.as_ref().map(|r| !self.sel.contains(&r.path)).unwrap_or(false) {
                    self.rename = None;
                }
            }
            Action::SelectAll => self.sel = self.items.iter().map(|e| e.path.clone()).collect(),
            Action::SelectNone => {
                self.sel.clear();
                self.rename = None;
            }
            Action::InvertSelection => self.sel = self.items.iter().filter(|e| !self.sel.contains(&e.path)).map(|e| e.path.clone()).collect(),
            Action::BeginRename(p) => {
                let p = p.or_else(|| self.sel.first().cloned());
                if let Some(p) = p {
                    if let Some(e) = self.entry(&p) {
                        self.rename = Some(RenameState { path: p.clone(), text: e.name.clone(), fresh: true });
                        self.sel = vec![p];
                    }
                }
            }
            Action::CancelRename => self.rename = None,
            Action::CommitRename => {
                if let Some(r) = self.rename.take() {
                    let new = r.text.trim().to_string();
                    let old = name_of(&r.path).to_string();
                    if new.is_empty() || new == old {
                        return;
                    }
                    if new.contains('/') {
                        return self.err("Names can’t contain “/”");
                    }
                    let to = join(&parent(&r.path), &new);
                    match self.eng.rename(&r.path, &to) {
                        Ok(()) => {
                            self.eng.undo.push(Undo::Rename { from: r.path.clone(), to: to.clone() });
                            self.refresh();
                            self.sel = vec![to];
                            self.toast(format!("Renamed to “{new}”"), Icon::Rename, true, Some(("Undo".into(), Action::Undo)));
                        }
                        Err(e) => self.err(e),
                    }
                }
            }
            Action::NewFolder => {
                let dir = self.current_folder();
                let p = self.eng.unique(&dir, "Untitled folder", "");
                match self.eng.mkdir(&p) {
                    Ok(()) => {
                        self.eng.undo.push(Undo::Created(vec![p.clone()]));
                        self.refresh();
                        self.dispatch(Action::BeginRename(Some(p)));
                    }
                    Err(e) => self.err(e),
                }
            }
            Action::NewFile(ext) => {
                let dir = self.current_folder();
                let p = self.eng.unique(&dir, &format!("Untitled.{ext}"), "");
                match self.eng.create_file(&p, if ext == "md" { b"# Untitled\n" } else { b"" }) {
                    Ok(()) => {
                        self.eng.undo.push(Undo::Created(vec![p.clone()]));
                        self.refresh();
                        self.dispatch(Action::BeginRename(Some(p)));
                    }
                    Err(e) => self.err(e),
                }
            }
            Action::Copy | Action::Cut => {
                if self.sel.is_empty() {
                    return;
                }
                let cut = a == Action::Cut;
                self.clip = Some((self.sel_paths(), cut));
                let n = self.sel.len();
                self.toast(format!("{} {}", if cut { "Cut" } else { "Copied" }, plural(n, "item", "items")), if cut { Icon::Cut } else { Icon::Copy }, true, None);
            }
            Action::Paste => {
                let dest = self.current_folder();
                self.dispatch(Action::PasteInto(dest));
            }
            Action::PasteInto(dest) => {
                if let Some((paths, cut)) = self.clip.clone() {
                    match self.eng.start_transfer(&paths, &dest, cut) {
                        Ok(_) => {
                            if cut {
                                self.clip = None;
                            }
                        }
                        Err(e) => self.err(e),
                    }
                }
            }
            Action::Duplicate => {
                let paths = self.sel_paths();
                if let Some(first) = paths.first() {
                    let dest = parent(first);
                    if let Err(e) = self.eng.start_transfer(&paths, &dest, false) {
                        self.err(e);
                    }
                }
            }
            Action::MoveTo(paths, dest) => {
                if let Err(e) = self.eng.start_transfer(&paths, &dest, true) {
                    self.err(e);
                }
            }
            Action::CopyTo(paths, dest) => {
                if let Err(e) = self.eng.start_transfer(&paths, &dest, false) {
                    self.err(e);
                }
            }
            Action::Trash => {
                if matches!(self.route, Route::Trash) {
                    return self.dispatch(Action::AskDelete);
                }
                let paths = self.sel_paths();
                let mut pairs = vec![];
                for p in &paths {
                    match self.eng.trash(p) {
                        Ok(Some(t)) => pairs.push((p.clone(), t)),
                        Ok(None) => {}
                        Err(e) => return self.err(e),
                    }
                }
                if !pairs.is_empty() {
                    self.eng.undo.push(Undo::Trashed(pairs.clone()));
                }
                let undo = if pairs.is_empty() { None } else { Some(("Undo".into(), Action::Undo)) };
                self.sel.clear();
                self.eng.revision += 1;
                self.refresh();
                self.toast(format!("Moved {} to Trash", plural(paths.len(), "item", "items")), Icon::Trash, true, undo);
            }
            Action::AskDelete => {
                if !self.sel.is_empty() {
                    self.modal = Some(Modal::Delete(self.sel_paths()));
                }
            }
            Action::ConfirmDelete(p) => {
                self.modal = None;
                self.sel.clear();
                self.eng.start_delete(&p);
            }
            Action::Restore => {
                let mut n = 0;
                for p in self.sel_paths() {
                    match self.eng.restore(&p) {
                        Ok(_) => n += 1,
                        Err(e) => self.err(e),
                    }
                }
                self.sel.clear();
                self.eng.revision += 1;
                self.refresh();
                if n > 0 {
                    self.toast(format!("Restored {}", plural(n, "item", "items")), Icon::Restore, true, None);
                }
            }
            Action::EmptyTrash => match self.eng.empty_trash() {
                Ok(n) => {
                    self.eng.revision += 1;
                    self.refresh();
                    self.toast(format!("Trash emptied · {}", plural(n, "item", "items")), Icon::Trash, true, None)
                }
                Err(e) => self.err(e),
            },
            Action::Undo => match self.eng.undo_last() {
                Ok(m) => {
                    self.eng.revision += 1;
                    self.refresh();
                    self.toast(m, Icon::Undo, true, None)
                }
                Err(e) => self.toast(e, Icon::Undo, false, None),
            },
            Action::SetSort(k) => {
                if self.sort == k {
                    self.sort_asc = !self.sort_asc;
                } else {
                    self.sort = k;
                    self.sort_asc = true;
                }
                self.refresh();
            }
            Action::ToggleSortDir => {
                self.sort_asc = !self.sort_asc;
                self.refresh();
            }
            Action::SetView(v) => self.view = v,
            Action::ToggleHidden => {
                self.eng.settings.show_hidden = !self.eng.settings.show_hidden;
                self.refresh();
            }
            Action::Tag(t) => {
                for p in self.sel_paths() {
                    self.eng.set_tag(&p, t.as_deref());
                }
                self.toast(match &t { Some(t) => format!("Tagged {}", tag_def(t).map(|d| d.label).unwrap_or("")), None => "Tag removed".into() }, Icon::Tag, true, None);
            }
            Action::CompressDialog => {
                if !self.sel.is_empty() {
                    let paths = self.sel_paths();
                    let name = if paths.len() == 1 { stem_of(name_of(&paths[0])).to_string() } else { format!("Archive of {}", self.title()) };
                    self.modal = Some(Modal::Compress { paths, name, fmt: Format::Zip });
                }
            }
            Action::CompressNow { name, fmt } => {
                if let Some(Modal::Compress { paths, .. }) = self.modal.take() {
                    let dest = parent(&paths[0]);
                    match self.eng.start_compress(&paths, &dest, fmt, name.trim()) {
                        Ok(_) => {}
                        Err(e) => self.err(e),
                    }
                }
            }
            Action::ExtractHere => {
                for e in self.selected_entries().into_iter().filter(|e| archive::is_archive(&e.name)) {
                    if let Err(er) = self.eng.start_extract(&e.path, &parent(&e.path)) {
                        self.err(er);
                    }
                }
            }
            Action::Properties(p) => {
                let t = p.or_else(|| self.sel.first().cloned()).unwrap_or_else(|| self.current_folder());
                self.modal = Some(Modal::Properties(t));
            }
            Action::CopyPath => {
                self.toast("Path copied", Icon::Link, true, None);
            }
            Action::OpenTerminal => {
                let d = self.current_folder();
                if let Err(e) = self.eng.terminal_at(&d) {
                    self.err(e);
                }
            }
            Action::ToggleSidebar => self.sidebar_open = !self.sidebar_open,
            Action::ToggleAi => self.ai_open = !self.ai_open,
            Action::Palette(r) => {
                match r {
                    Some(r) => {
                        self.palette.open = true;
                        self.palette.rail = r;
                        self.palette.sel = 0;
                        if !self.palette.open {
                            self.palette.query.clear();
                        }
                    }
                    None => self.palette.open = false,
                }
                self.palette.sel = 0;
            }
            Action::ToggleTheme => self.theme = if self.theme == Theme::Light { Theme::Dark } else { Theme::Light },
            Action::Search(q) => {
                self.palette.open = false;
                if q.trim().is_empty() {
                    if matches!(self.route, Route::Search(_)) {
                        let r = self.search_origin.take().unwrap_or(Route::Folder(self.eng.home.clone()));
                        self.search_text.clear();
                        self.route = r;
                        self.refresh();
                    }
                    return;
                }
                self.search_text = q.clone();
                if !matches!(self.route, Route::Search(_)) {
                    self.search_origin = Some(self.route.clone());
                    self.back.push(self.route.clone());
                    self.fwd.clear();
                    if !matches!(self.route, Route::Folder(_)) {
                        self.search_everywhere = true;
                    }
                }
                self.route = Route::Search(q);
                self.sel.clear();
                self.refresh();
            }
            Action::SearchEverywhere(v) => {
                self.search_everywhere = v;
                self.refresh();
            }
            Action::Unmount(id) => {
                let label = self.eng.mounts.iter().find(|m| m.id == id).map(|m| m.label.clone()).unwrap_or_default();
                let root = self.eng.mounts.iter().find(|m| m.id == id).map(|m| m.root.clone()).unwrap_or_default();
                match self.eng.unmount(&id) {
                    Ok(()) => {
                        if let Route::Folder(p) = &self.route {
                            if is_within(p, &root) && root != "/" {
                                self.route = Route::Folder(self.eng.home.clone());
                            }
                        }
                        self.back.retain(|r| !matches!(r, Route::Folder(p) if is_within(p, &root)));
                        self.refresh();
                        self.toast(format!("“{label}” can be safely removed"), Icon::Eject, true, Some(("Mount again".into(), Action::Mount(id))));
                    }
                    Err(e) => self.err(format!("Couldn’t unmount “{label}”: {e}")),
                }
            }
            Action::Mount(id) => {
                let label = self.eng.mounts.iter().find(|m| m.id == id).map(|m| m.label.clone()).unwrap_or_default();
                let root = self.eng.mounts.iter().find(|m| m.id == id).map(|m| m.root.clone());
                match self.eng.mount(&id) {
                    Ok(()) => {
                        self.toast(format!("“{label}” mounted"), Icon::Plug, true, None);
                        let r = self.eng.mounts.iter().find(|m| m.id == id).map(|m| m.root.clone()).or(root);
                        if let Some(r) = r {
                            self.navigate(Route::Folder(r), true);
                        }
                    }
                    Err(e) => self.err(e),
                }
            }
            Action::CloseModal => self.modal = None,
            Action::ShowShortcuts => self.modal = Some(Modal::Shortcuts),
            Action::ShowAbout => self.modal = Some(Modal::About),
            Action::AddCloudDialog => self.modal = Some(Modal::AddCloud { provider: 0, account: String::new() }),
            Action::AddCloud => {
                if let Some(Modal::AddCloud { provider, account }) = self.modal.take() {
                    let label = CLOUD_PROVIDERS[provider].to_string();
                    let slug: String = label.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
                    let root = format!("/cloud/{slug}");
                    if self.eng.mounts.iter().any(|m| m.root == root) {
                        return self.err(format!("{label} is already connected"));
                    }
                    let mut fs = MemFs::default();
                    fs.put_dir(&root, now_ts());
                    for d in ["Documents", "Photos", "Shared"] {
                        fs.put_dir(&join(&root, d), now_ts());
                    }
                    fs.put_file(&join(&root, "Welcome.md"), crate::vfs::Content::Buf(format!("# {label}\nConnected as {}", if account.is_empty() { "you" } else { &account }).into_bytes()), now_ts());
                    let acct = if account.trim().is_empty() { "demo account".to_string() } else { account.clone() };
                    self.eng.add_mount(Mount { id: format!("cloud:{slug}"), label: label.clone(), root: root.clone(), kind: MountKind::Cloud, total: 100_000_000_000, free: 88_000_000_000, mounted: true, detail: format!("{acct} · demo connector"), device: None }, Box::new(fs));
                    self.eng.index.queue.push(root.clone());
                    self.eng.index.ready = false;
                    self.toast(format!("{label} connected"), Icon::Cloud, true, None);
                    self.navigate(Route::Folder(root), true);
                }
            }
            Action::JobPause(id) => self.eng.pause_job(id),
            Action::JobCancel(id) => {
                self.eng.cancel_job(id);
                self.refresh();
                self.toast("Transfer cancelled", Icon::Close, false, None);
            }
            Action::JobMinimize(id) => {
                if let Some(j) = self.eng.job_mut(id) {
                    j.minimized = true;
                }
            }
            Action::JobResolve { id, r, all } => self.eng.resolve(id, r, all),
            Action::AiAsk(t) => {
                if t.trim().is_empty() {
                    return;
                }
                self.msgs.push(AiMsg { user: true, text: t.clone(), ..Default::default() });
                let reply = self.ai_reply(&t);
                self.ai_pending = Some((self.eng.clock + 0.65 + (reply.text.len() as f64) * 0.004, reply));
            }
            Action::AiApply(i) => self.apply_plan(i),
            Action::CleanToggle(c, i) => {
                if let Some(it) = self.storage.report.cats.get_mut(c).and_then(|c| c.items.get_mut(i)) {
                    it.selected = !it.selected;
                }
            }
            Action::CleanToggleCat(c) => {
                if let Some(cat) = self.storage.report.cats.get_mut(c) {
                    let all = cat.items.iter().all(|i| i.selected);
                    cat.items.iter_mut().for_each(|i| i.selected = !all);
                }
            }
            Action::CleanExpand(c) => {
                if let Some(cat) = self.storage.report.cats.get_mut(c) {
                    cat.open = !cat.open;
                }
            }
            Action::CleanApply => {
                let mut freed = 0;
                let mut perm = vec![];
                let mut tr = vec![];
                for c in &self.storage.report.cats {
                    for i in c.items.iter().filter(|i| i.selected) {
                        freed += i.size;
                        if i.permanent { perm.push(i.path.clone()) } else { tr.push(i.path.clone()) }
                    }
                }
                let mut pairs = vec![];
                for p in tr {
                    if let Ok(Some(t)) = self.eng.trash(&p) {
                        pairs.push((p, t));
                    }
                }
                if !pairs.is_empty() {
                    self.eng.undo.push(Undo::Trashed(pairs));
                }
                if !perm.is_empty() {
                    self.eng.start_delete(&perm);
                }
                self.eng.revision += 1;
                self.refresh();
                self.storage.last_freed = Some(freed);
                self.toast(format!("Freed {}", fmt_size(freed)), Icon::Broom, true, None);
            }
            Action::Rescan => {
                self.eng.reindex();
                self.refresh();
            }
            Action::LensGo(p) => {
                self.storage.lens_root = p;
                self.storage.lens = lens(&self.eng, &self.storage.lens_root);
            }
            Action::PlayToggle(p) => {
                if self.player.path.as_ref() == Some(&p) {
                    self.player.playing = !self.player.playing;
                    self.player.cmd = Some(if self.player.playing { PlayerCmd::Resume } else { PlayerCmd::Pause });
                } else {
                    self.player = Player { path: Some(p.clone()), playing: true, pos: 0.0, dur: self.audio_duration(&p), cmd: Some(PlayerCmd::Start(p)) };
                }
            }
            Action::CreateDemo => {
                #[cfg(not(target_arch = "wasm32"))]
                match crate::demo::create_native_workspace(&mut self.eng) {
                    Ok(p) => {
                        self.palette.open = false;
                        self.navigate(Route::Folder(p), true);
                        self.toast("Demo workspace created — try copying “file transfer.txt”", Icon::Rocket, true, None);
                    }
                    Err(e) => self.err(e),
                }
                #[cfg(target_arch = "wasm32")]
                self.toast("The browser demo already includes it on the Desktop", Icon::Rocket, true, None);
            }
            Action::SetPace(p) => self.eng.settings.pace = p,
            Action::FocusSearch => self.focus_search = true,
            Action::Terminal => self.modal = Some(Modal::Terminal),
        }
    }

    fn apply_plan(&mut self, idx: usize) {
        let Some(plan) = self.msgs.get(idx).and_then(|m| m.plan.clone()) else { return };
        let mut moves: HashMap<Path, Vec<Path>> = HashMap::new();
        let mut done = 0;
        for op in plan.ops {
            match op {
                PlanOp::MkDir(p) => {
                    let _ = self.eng.mkdir(&p);
                }
                PlanOp::Move { from, to_dir } => moves.entry(to_dir).or_default().push(from),
                PlanOp::Rename { from, to } => {
                    let dst = join(&parent(&from), &to);
                    if self.eng.rename(&from, &dst).is_ok() {
                        self.eng.undo.push(Undo::Rename { from, to: dst });
                        done += 1;
                    }
                }
                PlanOp::Trash(p) => {
                    if let Ok(t) = self.eng.trash(&p) {
                        if let Some(t) = t {
                            self.eng.undo.push(Undo::Trashed(vec![(p, t)]));
                        }
                        done += 1;
                    }
                }
                PlanOp::Compress { paths, name } => {
                    let dest = parent(&paths[0]);
                    let _ = self.eng.start_compress(&paths, &dest, Format::Zip, &name);
                    done += 1;
                }
            }
        }
        for (dir, paths) in moves {
            done += paths.len();
            let _ = self.eng.start_transfer(&paths, &dir, true);
        }
        if let Some(m) = self.msgs.get_mut(idx) {
            m.applied = true;
        }
        self.eng.revision += 1;
        self.refresh();
        self.toast(format!("Applied · {}", plural(done, "change", "changes")), Icon::Wand, true, Some(("Undo".into(), Action::Undo)));
    }

    // ───────────── context-aware appbar ─────────────
    /// Buttons for the contextual island in the appbar. Different state → different tools.
    pub fn tools(&self) -> Vec<Tool> {
        let t = |l: &str, i: Icon, a: Action, tone: Tone, tip: &str| Tool { label: l.into(), icon: i, action: a, tone, tip: tip.into() };
        let sel = self.selected_entries();
        let mut v = vec![];
        match &self.route {
            Route::Trash => {
                if sel.is_empty() {
                    v.push(t("Empty Trash", Icon::Trash, Action::EmptyTrash, Tone::Danger, "Permanently delete everything"));
                } else {
                    v.push(t("Restore", Icon::Restore, Action::Restore, Tone::Accent, "Put back where it was"));
                    v.push(t("Delete forever", Icon::Trash, Action::AskDelete, Tone::Danger, "Cannot be undone"));
                }
            }
            Route::Storage => {
                v.push(t("Rescan", Icon::Refresh, Action::Rescan, Tone::Plain, "Re-index storage"));
                let n = self.storage.report.selected_total();
                if n > 0 {
                    v.push(t(&format!("Clean {}", fmt_size(n)), Icon::Broom, Action::CleanApply, Tone::Accent, "Remove selected items"));
                }
            }
            _ if sel.is_empty() => {
                if let Some((p, cut)) = &self.clip {
                    if self.in_folder() {
                        v.push(t(&format!("Paste {}", plural(p.len(), "item", "items")), Icon::Paste, Action::Paste, Tone::Accent, if *cut { "Move here" } else { "Copy here" }));
                    }
                }
                if self.in_folder() {
                    v.push(t("New folder", Icon::NewFolder, Action::NewFolder, Tone::Plain, "Ctrl+Shift+N"));
                }
            }
            _ => {
                let single = sel.len() == 1;
                let e0 = &sel[0];
                let all_arch = sel.iter().all(|e| archive::is_archive(&e.name));
                if single && e0.is_dir {
                    v.push(t("Open", Icon::FolderOpen, Action::Open(e0.path.clone()), Tone::Accent, "Enter"));
                } else if single && e0.kind == Kind::Audio {
                    let playing = self.player.playing && self.player.path.as_ref() == Some(&e0.path);
                    v.push(t(if playing { "Pause" } else { "Play" }, if playing { Icon::Pause } else { Icon::Play }, Action::PlayToggle(e0.path.clone()), Tone::Accent, "Preview audio"));
                } else if all_arch {
                    v.push(t("Extract", Icon::Archive, Action::ExtractHere, Tone::Accent, "Extract here"));
                } else if single {
                    v.push(t("Open", Icon::OpenExt, Action::Open(e0.path.clone()), Tone::Accent, "Enter"));
                } else {
                    v.push(t("Open", Icon::OpenExt, Action::OpenSelected, Tone::Accent, "Enter"));
                }
                if single && !e0.is_dir {
                    v.push(t("Quick Look", Icon::Eye, Action::QuickLook(None), Tone::Plain, "Space"));
                }
                v.push(t("Copy", Icon::Copy, Action::Copy, Tone::Plain, "Ctrl+C"));
                v.push(t("Cut", Icon::Cut, Action::Cut, Tone::Plain, "Ctrl+X"));
                if single && e0.is_dir && self.clip.is_some() {
                    v.push(t("Paste into", Icon::Paste, Action::PasteInto(e0.path.clone()), Tone::Plain, "Paste into this folder"));
                }
                if !all_arch {
                    v.push(t("Compress", Icon::Zip, Action::CompressDialog, Tone::Plain, "Create an archive"));
                }
                if single {
                    v.push(t("Rename", Icon::Rename, Action::BeginRename(None), Tone::Plain, "F2"));
                }
                v.push(t("Info", Icon::Info, Action::Properties(None), Tone::Plain, "Ctrl+I"));
                v.push(t("Trash", Icon::Trash, Action::Trash, Tone::Danger, "Delete"));
            }
        }
        v
    }

    // ───────────── context menus ─────────────
    fn move_copy_subs(&self, paths: &[Path], mv: bool) -> Vec<MenuEntry> {
        let here = self.current_folder();
        self.dests()
            .into_iter()
            .filter(|(_, _, p)| *p != here || !mv)
            .map(|(l, i, p)| mi(l, i, if mv { Action::MoveTo(paths.to_vec(), p) } else { Action::CopyTo(paths.to_vec(), p) }).e())
            .collect()
    }

    pub fn menu_for_items(&self) -> Vec<MenuEntry> {
        let sel = self.selected_entries();
        if sel.is_empty() {
            return self.menu_background();
        }
        let paths = self.sel_paths();
        let single = sel.len() == 1;
        let e0 = &sel[0];
        let mut m: Vec<MenuEntry> = vec![];
        if matches!(self.route, Route::Trash) {
            m.push(mi("Restore", Icon::Restore, Action::Restore).e());
            m.push(mi("Get Info", Icon::Info, Action::Properties(None)).sc("Ctrl+I").e());
            m.push(MenuEntry::Sep);
            m.push(mi("Delete Permanently", Icon::Trash, Action::AskDelete).danger().e());
            return m;
        }
        let all_arch = sel.iter().all(|e| archive::is_archive(&e.name));
        if single && e0.is_dir {
            m.push(mi("Open", Icon::FolderOpen, Action::Open(e0.path.clone())).sc("Enter").e());
        } else {
            m.push(mi("Open", Icon::OpenExt, Action::OpenSelected).sc("Enter").e());
            if single {
                let mut apps: Vec<MenuEntry> = self.eng.apps_for(e0).into_iter().take(6).map(|a| MenuItem { icon: a.icon, ..mi(a.name.clone() + if a.default { "  · default" } else { "" }, a.icon, Action::OpenWith(e0.path.clone(), a.id)) }.e()).collect();
                apps.push(MenuEntry::Sep);
                apps.push(mi("Other app…", Icon::App, Action::OpenWithDialog(e0.path.clone())).e());
                m.push(MenuEntry::Sub("Open With".into(), Icon::App, apps));
                m.push(mi("Quick Look", Icon::Eye, Action::QuickLook(None)).sc("Space").e());
            }
        }
        if single && e0.kind == Kind::Audio {
            m.push(mi("Play", Icon::Play, Action::PlayToggle(e0.path.clone())).e());
        }
        m.push(MenuEntry::Sep);
        m.push(mi("Copy", Icon::Copy, Action::Copy).sc("Ctrl+C").e());
        m.push(mi("Cut", Icon::Cut, Action::Cut).sc("Ctrl+X").e());
        if single && e0.is_dir {
            m.push(mi("Paste Into Folder", Icon::Paste, Action::PasteInto(e0.path.clone())).sc("Ctrl+V").on(self.clip.is_some()).e());
        }
        m.push(mi("Duplicate", Icon::Duplicate, Action::Duplicate).sc("Ctrl+D").e());
        m.push(MenuEntry::Sep);
        if single {
            m.push(mi("Rename", Icon::Rename, Action::BeginRename(None)).sc("F2").e());
        }
        m.push(MenuEntry::Sub("Move to".into(), Icon::Move, self.move_copy_subs(&paths, true)));
        m.push(MenuEntry::Sub("Copy to".into(), Icon::Copy, self.move_copy_subs(&paths, false)));
        let sends: Vec<MenuEntry> = self.eng.mounts.iter().filter(|mt| mt.mounted && matches!(mt.kind, MountKind::Removable | MountKind::Phone | MountKind::Cloud | MountKind::Network)).map(|mt| mi(mt.label.clone(), mt.icon(), Action::CopyTo(paths.clone(), mt.root.clone())).e()).collect();
        if !sends.is_empty() {
            m.push(MenuEntry::Sub("Send to".into(), Icon::Send, sends));
        }
        m.push(MenuEntry::Sep);
        if all_arch {
            m.push(mi("Extract Here", Icon::Archive, Action::ExtractHere).e());
        } else {
            m.push(mi("Compress…", Icon::Zip, Action::CompressDialog).e());
        }
        let mut tags: Vec<MenuEntry> = TAGS.iter().map(|t| mi(t.label, Icon::Tag, Action::Tag(Some(t.id.into()))).swatch(t.rgb).check(single && self.eng.tags.get(&e0.path).map(|x| x == t.id).unwrap_or(false)).e()).collect();
        tags.push(MenuEntry::Sep);
        tags.push(mi("Remove tag", Icon::Close, Action::Tag(None)).e());
        m.push(MenuEntry::Sub("Tag".into(), Icon::Tag, tags));
        m.push(mi("Copy Path", Icon::Link, Action::CopyPath).e());
        m.push(mi("Get Info", Icon::Info, Action::Properties(None)).sc("Ctrl+I").e());
        m.push(MenuEntry::Sep);
        m.push(mi("Move to Trash", Icon::Trash, Action::Trash).sc("Del").danger().e());
        m
    }

    pub fn menu_background(&self) -> Vec<MenuEntry> {
        let mut m = vec![];
        if matches!(self.route, Route::Trash) {
            m.push(mi("Empty Trash", Icon::Trash, Action::EmptyTrash).danger().e());
            m.push(mi("Select All", Icon::SelectAll, Action::SelectAll).sc("Ctrl+A").e());
            return m;
        }
        let editable = self.in_folder();
        m.push(MenuEntry::Sub(
            "New".into(),
            Icon::Plus,
            vec![
                mi("Folder", Icon::NewFolder, Action::NewFolder).sc("Ctrl+Shift+N").on(editable).e(),
                mi("Text Document", Icon::NewFile, Action::NewFile("txt")).on(editable).e(),
                mi("Markdown Note", Icon::Doc, Action::NewFile("md")).on(editable).e(),
            ],
        ));
        m.push(mi("Paste", Icon::Paste, Action::Paste).sc("Ctrl+V").on(self.clip.is_some() && editable).e());
        m.push(mi("Select All", Icon::SelectAll, Action::SelectAll).sc("Ctrl+A").e());
        m.push(MenuEntry::Sep);
        m.push(MenuEntry::Sub(
            "View".into(),
            Icon::Grid,
            vec![mi("Grid", Icon::Grid, Action::SetView(ViewMode::Grid)).check(self.view == ViewMode::Grid).e(), mi("List", Icon::List, Action::SetView(ViewMode::List)).check(self.view == ViewMode::List).e(), MenuEntry::Sep, mi("Show Hidden Files", Icon::Eye, Action::ToggleHidden).check(self.eng.settings.show_hidden).e()],
        ));
        m.push(MenuEntry::Sub("Sort by".into(), Icon::Sort, SortKey::ALL.iter().map(|k| mi(k.label(), Icon::Sort, Action::SetSort(*k)).check(self.sort == *k).e()).collect()));
        m.push(MenuEntry::Sep);
        if self.in_folder() {
            m.push(mi("Open Terminal Here", Icon::Terminal, Action::OpenTerminal).on(self.eng.native).e());
        }
        m.push(mi("Ask Smart about this folder", Icon::Sparkle, Action::ToggleAi).e());
        m.push(mi("Refresh", Icon::Refresh, Action::Refresh).sc("F5").e());
        m.push(mi("Get Info", Icon::Info, Action::Properties(Some(self.current_folder()))).e());
        m
    }

    pub fn menu_for_mount(&self, id: &str) -> Vec<MenuEntry> {
        let Some(m) = self.eng.mounts.iter().find(|m| m.id == id) else { return vec![] };
        let mut v = vec![];
        if m.mounted {
            v.push(mi("Open", Icon::FolderOpen, Action::GoPath(m.root.clone())).e());
            v.push(mi("Get Info", Icon::Info, Action::Properties(Some(m.root.clone()))).e());
            if m.root != "/" {
                v.push(MenuEntry::Sep);
                v.push(mi(if matches!(m.kind, MountKind::Cloud | MountKind::Network) { "Disconnect" } else { "Eject" }, Icon::Eject, Action::Unmount(m.id.clone())).e());
            }
        } else {
            v.push(mi(if matches!(m.kind, MountKind::Cloud | MountKind::Network) { "Connect" } else { "Mount" }, Icon::Plug, Action::Mount(m.id.clone())).e());
        }
        v
    }

    pub fn menu_for_sidebar_place(&self, route: &Route) -> Vec<MenuEntry> {
        vec![mi("Open", Icon::FolderOpen, Action::Go(route.clone())).e(), mi("Ask Smart about this", Icon::Sparkle, Action::ToggleAi).e()]
    }

    pub fn menu_view_options(&self) -> Vec<MenuEntry> {
        let mut v: Vec<MenuEntry> = SortKey::ALL.iter().map(|k| mi(k.label(), Icon::Sort, Action::SetSort(*k)).check(self.sort == *k).e()).collect();
        v.push(MenuEntry::Sep);
        v.push(mi(if self.sort_asc { "Ascending" } else { "Descending" }, if self.sort_asc { Icon::ArrowUp } else { Icon::ArrowDown }, Action::ToggleSortDir).e());
        v
    }
    pub fn menu_more(&self) -> Vec<MenuEntry> {
        let mut v = vec![
            mi("Show Hidden Files", Icon::Eye, Action::ToggleHidden).check(self.eng.settings.show_hidden).e(),
            mi("Undo", Icon::Undo, Action::Undo).sc("Ctrl+Z").on(!self.eng.undo.is_empty()).e(),
            mi("Storage & Cleanup", Icon::Pie, Action::Go(Route::Storage)).e(),
            mi("Connect cloud account…", Icon::Cloud, Action::AddCloudDialog).e(),
            MenuEntry::Sep,
            MenuEntry::Sub(
                "Transfer pace".into(),
                Icon::Bolt,
                vec![
                    mi("Demo · slow big copies", Icon::Rocket, Action::SetPace(Pace::Demo)).check(self.eng.settings.pace == Pace::Demo).e(),
                    mi("Real speed", Icon::Bolt, Action::SetPace(Pace::Realistic)).check(self.eng.settings.pace == Pace::Realistic).e(),
                    mi("Instant", Icon::Bolt, Action::SetPace(Pace::Instant)).check(self.eng.settings.pace == Pace::Instant).e(),
                ],
            ),
            mi("Toggle Theme", if self.theme == Theme::Light { Icon::Moon } else { Icon::Sun }, Action::ToggleTheme).e(),
            mi("Keyboard Shortcuts", Icon::Keyboard, Action::ShowShortcuts).e(),
            mi("About Smart Files", Icon::Info, Action::ShowAbout).e(),
        ];
        if self.eng.native {
            v.insert(4, mi("Create demo workspace", Icon::Rocket, Action::CreateDemo).e());
        }
        v
    }
    pub fn menu_workspace(&self) -> Vec<MenuEntry> {
        vec![mi("Storage & Cleanup", Icon::Pie, Action::Go(Route::Storage)).e(), mi("Connect cloud account…", Icon::Cloud, Action::AddCloudDialog).e(), mi("Toggle Theme", Icon::Moon, Action::ToggleTheme).e(), mi("Keyboard Shortcuts", Icon::Keyboard, Action::ShowShortcuts).e()]
    }

    // ───────────── command palette ─────────────
    pub fn palette_items(&self) -> Vec<Cmd> {
        let mut all: Vec<Cmd> = vec![];
        let home = self.eng.home.clone();
        let c = |title: &str, sub: &str, icon: Icon, action: Action, rail: Rail, group: &'static str, sc: &'static str| Cmd { title: title.into(), sub: sub.into(), icon, action, rail, group, shortcut: sc, path: None, indices: vec![] };
        // Navigate
        for (n, i) in [("Home", Icon::Home), ("Desktop", Icon::Monitor), ("Documents", Icon::Doc), ("Downloads", Icon::Download), ("Pictures", Icon::Image), ("Music", Icon::Music), ("Movies", Icon::Film)] {
            let p = if n == "Home" { home.clone() } else { join(&home, n) };
            if self.eng.exists(&p) {
                all.push(c(&format!("Go to {n}"), &p, i, Action::GoPath(p.clone()), Rail::Go, "Navigate", ""));
            }
        }
        all.push(c("Go to Recents", "Recently modified files", Icon::Clock, Action::Go(Route::Recents), Rail::Go, "Navigate", ""));
        all.push(c("Go to Trash", "Deleted items", Icon::Trash, Action::Go(Route::Trash), Rail::Go, "Navigate", ""));
        for m in &self.eng.mounts {
            all.push(c(&format!("{} {}", if m.mounted { "Open" } else { "Mount" }, m.label), &m.detail, m.icon(), if m.mounted { Action::GoPath(m.root.clone()) } else { Action::Mount(m.id.clone()) }, Rail::Go, "Navigate", ""));
        }
        for t in TAGS.iter() {
            all.push(c(&format!("Tag: {}", t.label), &format!("{} tagged items", self.tag_count(t.id)), Icon::Tag, Action::Go(Route::Tag(t.id.into())), Rail::Go, "Navigate", ""));
        }
        all.push(c("Back", "", Icon::Back, Action::Back, Rail::Go, "Navigate", "Alt+←"));
        all.push(c("Enclosing folder", "", Icon::Up, Action::Up, Rail::Go, "Navigate", "Alt+↑"));
        all.push(c("Storage & Cleanup", "Space Lens, caches, duplicates", Icon::Pie, Action::Go(Route::Storage), Rail::Go, "Navigate", ""));
        // Files
        let has_sel = !self.sel.is_empty();
        let one = self.sel.len() == 1;
        if has_sel {
            all.push(c("Open selection", "", Icon::OpenExt, Action::OpenSelected, Rail::Files, "Files", "Enter"));
            all.push(c("Copy", "", Icon::Copy, Action::Copy, Rail::Files, "Files", "Ctrl+C"));
            all.push(c("Cut", "", Icon::Cut, Action::Cut, Rail::Files, "Files", "Ctrl+X"));
            all.push(c("Duplicate", "", Icon::Duplicate, Action::Duplicate, Rail::Files, "Files", "Ctrl+D"));
            all.push(c("Compress…", "Create zip, tar.gz", Icon::Zip, Action::CompressDialog, Rail::Files, "Files", ""));
            if self.selected_entries().iter().any(|e| archive::is_archive(&e.name)) {
                all.push(c("Extract archive", "Extract here", Icon::Archive, Action::ExtractHere, Rail::Files, "Files", ""));
            }
            all.push(c("Move to Trash", "", Icon::Trash, Action::Trash, Rail::Files, "Files", "Del"));
            all.push(c("Get Info", "", Icon::Info, Action::Properties(None), Rail::Files, "Files", "Ctrl+I"));
            if one {
                all.push(c("Rename", "", Icon::Rename, Action::BeginRename(None), Rail::Files, "Files", "F2"));
                all.push(c("Quick Look", "", Icon::Eye, Action::QuickLook(None), Rail::Files, "Files", "Space"));
            }
        }
        if self.clip.is_some() {
            all.push(c("Paste here", "", Icon::Paste, Action::Paste, Rail::Files, "Files", "Ctrl+V"));
        }
        all.push(c("Select all", "", Icon::SelectAll, Action::SelectAll, Rail::Files, "Files", "Ctrl+A"));
        all.push(c("Undo", "", Icon::Undo, Action::Undo, Rail::Files, "Files", "Ctrl+Z"));
        all.push(c("Empty Trash", "", Icon::Trash, Action::EmptyTrash, Rail::Files, "Files", ""));
        all.push(c("Open terminal here", "", Icon::Terminal, Action::OpenTerminal, Rail::Files, "Files", ""));
        // Make
        all.push(c("New folder", "In the current folder", Icon::NewFolder, Action::NewFolder, Rail::Make, "Create", "Ctrl+Shift+N"));
        all.push(c("New text document", "", Icon::NewFile, Action::NewFile("txt"), Rail::Make, "Create", ""));
        all.push(c("New markdown note", "", Icon::Doc, Action::NewFile("md"), Rail::Make, "Create", ""));
        all.push(c("Connect cloud account", "Drive, Dropbox, S3, WebDAV", Icon::Cloud, Action::AddCloudDialog, Rail::Make, "Create", ""));
        if self.eng.native {
            all.push(c("Create demo workspace", "Includes a 5 GB “file transfer.txt”", Icon::Rocket, Action::CreateDemo, Rail::Make, "Create", ""));
        }
        // Look
        all.push(c(if self.theme == Theme::Light { "Switch to dark" } else { "Switch to light" }, "Theme", if self.theme == Theme::Light { Icon::Moon } else { Icon::Sun }, Action::ToggleTheme, Rail::Look, "Look", ""));
        all.push(c("Grid view", "", Icon::Grid, Action::SetView(ViewMode::Grid), Rail::Look, "Look", ""));
        all.push(c("List view", "", Icon::List, Action::SetView(ViewMode::List), Rail::Look, "Look", ""));
        for k in SortKey::ALL {
            all.push(c(&format!("Sort by {}", k.label()), "", Icon::Sort, Action::SetSort(k), Rail::Look, "Look", ""));
        }
        all.push(c("Toggle hidden files", "", Icon::Eye, Action::ToggleHidden, Rail::Look, "Look", ""));
        all.push(c("Toggle sidebar", "", Icon::Sidebar, Action::ToggleSidebar, Rail::Look, "Look", "Ctrl+\\"));
        all.push(c("Keyboard shortcuts", "", Icon::Keyboard, Action::ShowShortcuts, Rail::Look, "Look", ""));
        all.push(c("About Smart Files", "", Icon::Info, Action::ShowAbout, Rail::Look, "Look", ""));
        // Ask
        for q in ["Organise this folder", "Find duplicates", "Free up space", "Summarise this folder", "Clean up file names"] {
            all.push(c(q, "Ask Smart · on-device", Icon::Sparkle, Action::AiAsk(q.into()), Rail::Ask, "Ask Smart", ""));
        }

        let q = self.palette.query.trim().to_string();
        let rail = self.palette.rail;
        let mut matcher = Matcher::new(Config::DEFAULT);
        let mut buf = Vec::new();
        let mut out: Vec<Cmd> = vec![];
        if q.is_empty() {
            let mut pool: Vec<Cmd> = all.into_iter().filter(|c| rail == Rail::All || c.rail == rail).collect();
            if rail == Rail::All {
                // curated overview: a few from each rail
                let mut seen: HashMap<&str, usize> = HashMap::new();
                pool.retain(|c| {
                    let n = seen.entry(c.group).or_default();
                    *n += 1;
                    *n <= 4
                });
            }
            out = pool;
            if matches!(rail, Rail::All | Rail::Files) {
                let mut recents: Vec<&search::IndexEntry> = self.eng.index.entries.iter().filter(|e| !e.is_dir && !e.path.contains("/.")).collect();
                recents.sort_by_key(|e| std::cmp::Reverse(e.modified));
                for e in recents.into_iter().take(4) {
                    out.push(Cmd { title: e.name.clone(), sub: parent(&e.path), icon: e.kind.icon(), action: Action::Open(e.path.clone()), rail: Rail::Files, group: "Recent files", shortcut: "", path: Some(e.path.clone()), indices: vec![] });
                }
            }
        } else {
            if matches!(rail, Rail::All | Rail::Ask) {
                out.push(Cmd { title: format!("Ask Smart: “{q}”"), sub: "On-device assistant".into(), icon: Icon::Sparkle, action: Action::AiAsk(q.clone()), rail: Rail::Ask, group: "Ask Smart", shortcut: "Ctrl+↵", path: None, indices: vec![] });
            }
            let pat = Pattern::parse(&q, CaseMatching::Smart, Normalization::Smart);
            let mut scored: Vec<(u32, Cmd)> = vec![];
            for mut cmd in all.into_iter().filter(|c| rail == Rail::All || c.rail == rail) {
                if let Some(s) = pat.score(Utf32Str::new(&cmd.title, &mut buf), &mut matcher) {
                    let mut ix = vec![];
                    pat.indices(Utf32Str::new(&cmd.title, &mut buf), &mut matcher, &mut ix);
                    cmd.indices = ix;
                    scored.push((s, cmd));
                }
            }
            scored.sort_by(|a, b| b.0.cmp(&a.0));
            out.extend(scored.into_iter().take(8).map(|x| x.1));
            if matches!(rail, Rail::All | Rail::Files | Rail::Go) {
                let parsed = search::parse(&q);
                let hits = self.run_query(&parsed, 7);
                for h in hits {
                    let e = &self.eng.index.entries[h.idx];
                    out.push(Cmd { title: e.name.clone(), sub: parent(&e.path), icon: e.kind.icon(), action: Action::Open(e.path.clone()), rail: Rail::Files, group: "Files", shortcut: "", path: Some(e.path.clone()), indices: h.indices });
                }
                out.push(Cmd { title: format!("Search everywhere for “{q}”"), sub: format!("{} indexed items · supports kind:, size:, in:, content:", self.eng.index.entries.len()), icon: Icon::Search, action: Action::Search(q.clone()), rail: Rail::Files, group: "Files", shortcut: "↵", path: None, indices: vec![] });
            }
        }
        // stable grouping by group order
        let order = ["Ask Smart", "Recent files", "Navigate", "Files", "Create", "Look"];
        out.sort_by_key(|c| order.iter().position(|g| *g == c.group).unwrap_or(9));
        out
    }
}
