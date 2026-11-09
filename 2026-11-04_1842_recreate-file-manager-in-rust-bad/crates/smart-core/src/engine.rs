//! The engine: mounts, routing, jobs (copy/move/delete/compress/extract), trash, undo, tags, index.
use crate::archive::{self, Compressor, Format};
use crate::model::*;
use crate::search::{self, SearchIndex};
use crate::sys::{self, AppInfo, HostDisk};
use crate::vfs::*;
use std::collections::HashMap;
use web_time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MountKind {
    Internal,
    Removable,
    Network,
    Cloud,
    Phone,
}
#[derive(Clone, Debug)]
pub struct Mount {
    pub id: String,
    pub label: String,
    pub root: Path,
    pub kind: MountKind,
    pub total: u64,
    pub free: u64,
    pub mounted: bool,
    pub detail: String,
    pub device: Option<String>,
}
impl Mount {
    pub fn icon(&self) -> Icon {
        match self.kind {
            MountKind::Internal => Icon::Drive,
            MountKind::Removable => Icon::Usb,
            MountKind::Network => Icon::Globe,
            MountKind::Cloud => Icon::Cloud,
            MountKind::Phone => Icon::Phone,
        }
    }
    pub fn used_frac(&self) -> f32 {
        if self.total == 0 { 0.0 } else { 1.0 - self.free as f32 / self.total as f32 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pace {
    /// Big transfers are paced so the copy experience is visible (default in the demo).
    Demo,
    Realistic,
    Instant,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Resolution {
    Replace,
    Skip,
    KeepBoth,
}
pub struct Settings {
    pub pace: Pace,
    pub show_hidden: bool,
    pub conflict: Option<Resolution>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JobKind {
    Copy,
    Move,
    Delete,
    Compress,
    Extract,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JobState {
    Running,
    Paused,
    NeedsDecision,
    Done,
    Failed,
    Cancelled,
}
#[derive(Clone, Debug)]
pub struct PlanItem {
    pub src: Path,
    pub dst: Path,
    pub is_dir: bool,
    pub size: u64,
    pub kind: Kind,
    pub atomic: bool,
}

pub struct Job {
    pub id: u64,
    pub kind: JobKind,
    pub title: String,
    pub src_dir: Path,
    pub dest: Path,
    pub srcs: Vec<Path>,
    pub state: JobState,
    pub plan: Vec<PlanItem>,
    pub cur: usize,
    pub off: u64,
    pub done_bytes: u64,
    pub total_bytes: u64,
    pub elapsed: f64,
    pub speeds: Vec<f32>,
    pub speed: f64,
    pub error: Option<String>,
    pub conflict: Option<String>,
    pub throttled: bool,
    pub results: Vec<(Path, Path)>,
    pub minimized: bool,
    pub apply_all: Option<Resolution>,
    pub finished_at: f64,
    credit: f64,
    sample_t: f64,
    sample_b: u64,
    checked: bool,
    comp: Option<Compressor>,
    out_path: Path,
    ext: Option<Vec<(String, bool, Vec<u8>)>>,
    permanent: bool,
}
impl Job {
    pub fn progress(&self) -> f32 {
        if self.total_bytes == 0 { 1.0 } else { (self.done_bytes as f32 / self.total_bytes as f32).clamp(0.0, 1.0) }
    }
    pub fn active(&self) -> bool {
        matches!(self.state, JobState::Running | JobState::Paused | JobState::NeedsDecision)
    }
    pub fn items_done(&self) -> usize {
        self.cur.min(self.plan.len())
    }
    pub fn eta(&self) -> f64 {
        if self.speed > 1.0 { (self.total_bytes - self.done_bytes) as f64 / self.speed } else { f64::INFINITY }
    }
    pub fn current_name(&self) -> String {
        self.plan.get(self.cur).map(|p| name_of(&p.src).to_string()).unwrap_or_default()
    }
    pub fn verb(&self) -> &'static str {
        match self.kind {
            JobKind::Copy => "Copying",
            JobKind::Move => "Moving",
            JobKind::Delete => "Deleting",
            JobKind::Compress => "Compressing",
            JobKind::Extract => "Extracting",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Undo {
    Rename { from: Path, to: Path },
    Created(Vec<Path>),
    Moved(Vec<(Path, Path)>),
    Trashed(Vec<(Path, Path)>),
}

pub struct Notice {
    pub text: String,
    pub icon: Icon,
    pub good: bool,
}

pub struct Engine {
    pub mounts: Vec<Mount>,
    fss: Vec<Box<dyn Vfs>>,
    pub jobs: Vec<Job>,
    pub index: SearchIndex,
    pub tags: HashMap<Path, String>,
    pub opens: HashMap<Path, (u32, i64)>,
    pub undo: Vec<Undo>,
    pub settings: Settings,
    pub home: Path,
    pub trash_dir: Option<Path>,
    pub trash_origin: HashMap<Path, Path>,
    pub notices: Vec<Notice>,
    pub clock: f64,
    next_job: u64,
    pub native: bool,
    pub revision: u64,
}

const THROTTLE_MIN: u64 = 512_000_000;

impl Engine {
    pub fn empty(home: &str, native: bool) -> Engine {
        Engine {
            mounts: vec![],
            fss: vec![],
            jobs: vec![],
            index: SearchIndex::default(),
            tags: HashMap::new(),
            opens: HashMap::new(),
            undo: vec![],
            settings: Settings { pace: Pace::Demo, show_hidden: false, conflict: None },
            home: home.into(),
            trash_dir: None,
            trash_origin: HashMap::new(),
            notices: vec![],
            clock: 0.0,
            next_job: 1,
            native,
            revision: 0,
        }
    }

    /// Fully in-memory demo world (what runs in the browser).
    pub fn demo() -> Engine {
        crate::demo::build()
    }

    /// Real world: real disks + real home folder.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn native() -> Engine {
        let home = sys::home().unwrap_or_else(|| "/".into());
        let mut e = Engine::empty(&home, true);
        e.refresh_host_mounts();
        if !e.mounts.iter().any(|m| m.root == "/") && !cfg!(windows) {
            e.add_mount(Mount { id: "root".into(), label: "Computer".into(), root: "/".into(), kind: MountKind::Internal, total: 0, free: 0, mounted: true, detail: "Local disk".into(), device: None }, Box::new(StdFs));
        }
        e.trash_dir = if cfg!(target_os = "macos") { Some(join(&home, ".Trash")) } else if cfg!(target_os = "linux") { Some(join(&home, ".local/share/Trash/files")) } else { None };
        e.reindex();
        e
    }

    pub fn add_mount(&mut self, m: Mount, fs: Box<dyn Vfs>) {
        self.mounts.push(m);
        self.fss.push(fs);
    }
    pub fn fs_mut(&mut self, mount_id: &str) -> Option<&mut Box<dyn Vfs>> {
        let i = self.mounts.iter().position(|m| m.id == mount_id)?;
        self.fss.get_mut(i)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn refresh_host_mounts(&mut self) {
        let disks = sys::host_disks();
        // drop vanished host mounts, update existing
        let ids: Vec<String> = disks.iter().map(|d| format!("host:{}", if d.mounted { &d.mount_point } else { &d.device })).collect();
        let mut i = 0;
        while i < self.mounts.len() {
            if self.mounts[i].id.starts_with("host:") && !ids.contains(&self.mounts[i].id) {
                self.mounts.remove(i);
                self.fss.remove(i);
            } else {
                i += 1;
            }
        }
        for (d, id) in disks.iter().zip(ids) {
            if let Some(m) = self.mounts.iter_mut().find(|m| m.id == id) {
                m.free = d.free;
                m.total = d.total;
                m.mounted = d.mounted;
                continue;
            }
            let root = if d.mounted { d.mount_point.clone() } else { format!("/__unmounted/{}", name_of(&d.device)) };
            let kind = if d.removable { MountKind::Removable } else { MountKind::Internal };
            self.add_mount(
                Mount { id, label: d.label.clone(), root, kind, total: d.total, free: d.free, mounted: d.mounted, detail: d.device.clone(), device: Some(d.device.clone()) },
                Box::new(StdFs),
            );
        }
        for (serial, model) in sys::adb_devices() {
            let id = format!("adb:{serial}");
            if !self.mounts.iter().any(|m| m.id == id) {
                let root = format!("/phone/{}", serial);
                self.add_mount(Mount { id, label: model, root: root.clone(), kind: MountKind::Phone, total: 0, free: 0, mounted: true, detail: "Android · adb".into(), device: None }, Box::new(sys::AdbFs { serial, root }));
            }
        }
    }

    // ───────────── routing ─────────────
    pub fn route(&self, p: &str) -> Option<usize> {
        self.mounts.iter().enumerate().filter(|(_, m)| m.mounted && is_within(p, &m.root)).max_by_key(|(_, m)| m.root.len()).map(|(i, _)| i)
    }
    fn fs(&self, p: &str) -> R<&dyn Vfs> {
        self.route(p).map(|i| self.fss[i].as_ref()).ok_or_else(|| "That location isn’t mounted".to_string())
    }
    fn fs_m(&mut self, p: &str) -> R<&mut Box<dyn Vfs>> {
        let i = self.route(p).ok_or_else(|| "That location isn’t mounted".to_string())?;
        Ok(&mut self.fss[i])
    }
    pub fn mount_of(&self, p: &str) -> Option<&Mount> {
        self.route(p).map(|i| &self.mounts[i])
    }
    pub fn is_local(&self, p: &str) -> bool {
        self.native && self.route(p).map(|i| self.fss[i].local_path(p).is_some()).unwrap_or(false)
    }

    // ───────────── basic ops ─────────────
    pub fn list(&self, p: &str) -> R<Vec<Entry>> {
        self.fs(p)?.read_dir(p)
    }
    pub fn stat(&self, p: &str) -> R<Entry> {
        self.fs(p)?.stat(p)
    }
    pub fn exists(&self, p: &str) -> bool {
        self.stat(p).is_ok()
    }
    pub fn mkdir(&mut self, p: &str) -> R<()> {
        self.fs_m(p)?.mkdir(p)?;
        self.touch_index(p);
        Ok(())
    }
    pub fn create_file(&mut self, p: &str, data: &[u8]) -> R<()> {
        self.fs_m(p)?.write_chunk(p, 0, Chunk::Data(data.to_vec()))?;
        self.touch_index(p);
        Ok(())
    }
    pub fn rename(&mut self, from: &str, to: &str) -> R<()> {
        let (a, b) = (self.route(from), self.route(to));
        if a != b || a.is_none() {
            return Err("Can’t rename across storage — use Move".into());
        }
        self.fs_m(from)?.rename(from, to)?;
        self.migrate(from, to);
        self.index.remove_prefix(from);
        self.touch_index(to);
        Ok(())
    }
    fn migrate(&mut self, from: &str, to: &str) {
        let moved: Vec<(String, String)> = self.tags.iter().filter(|(k, _)| is_within(k, from)).map(|(k, v)| (k.clone(), v.clone())).collect();
        for (k, v) in moved {
            self.tags.remove(&k);
            self.tags.insert(format!("{}{}", to, &k[from.len()..]), v);
        }
    }
    pub fn unique(&self, dir: &str, name: &str, suffix: &str) -> String {
        let (stem, ext) = match name.rfind('.') {
            Some(i) if i > 0 => (&name[..i], &name[i..]),
            _ => (name, ""),
        };
        let mut cand = join(dir, name);
        let mut n = 1;
        while self.exists(&cand) {
            n += 1;
            let label = if suffix.is_empty() { format!("{stem} {n}{ext}") } else if n == 2 { format!("{stem} {suffix}{ext}") } else { format!("{stem} {suffix} {}{ext}", n - 1) };
            cand = join(dir, &label);
        }
        cand
    }
    pub fn read_all(&self, p: &str, max: u64) -> R<Vec<u8>> {
        let fs = self.fs(p)?;
        let size = fs.stat(p)?.size;
        if size > max {
            return Err(format!("File is too large to load ({})", fmt_size(size)));
        }
        let mut out = Vec::with_capacity(size as usize);
        let mut off = 0;
        while off < size {
            let c = fs.read_at(p, off, (size - off).min(8 << 20))?;
            if c.is_empty() {
                break;
            }
            off += c.len();
            out.extend(c.into_vec());
        }
        Ok(out)
    }
    pub fn read_head(&self, p: &str, n: u64) -> R<Vec<u8>> {
        Ok(self.fs(p)?.read_at(p, 0, n)?.into_vec())
    }
    pub fn delete_permanent(&mut self, p: &str) -> R<()> {
        self.fs_m(p)?.remove(p)?;
        self.tags.retain(|k, _| !is_within(k, p));
        self.index.remove_prefix(p);
        Ok(())
    }
    pub fn trash(&mut self, p: &str) -> R<Option<Path>> {
        #[cfg(not(target_arch = "wasm32"))]
        if self.is_local(p) {
            trash::delete(to_real(p)).map_err(|e| e.to_string())?;
            self.tags.retain(|k, _| !is_within(k, p));
            self.index.remove_prefix(p);
            return Ok(None);
        }
        let Some(td) = self.trash_dir.clone() else {
            self.delete_permanent(p)?;
            return Ok(None);
        };
        if self.route(&td) != self.route(p) {
            self.delete_permanent(p)?;
            return Ok(None);
        }
        let _ = self.fs_m(&td).and_then(|f| f.mkdir(&td));
        let dst = self.unique(&td, name_of(p), "");
        self.fs_m(p)?.rename(p, &dst)?;
        self.trash_origin.insert(dst.clone(), p.to_string());
        self.index.remove_prefix(p);
        Ok(Some(dst))
    }
    pub fn restore(&mut self, p: &str) -> R<Path> {
        let name = name_of(p).to_string();
        #[allow(unused_mut)]
        let mut origin = self.trash_origin.get(p).cloned();
        #[cfg(target_os = "linux")]
        if origin.is_none() && self.native {
            let info = join(&parent(&parent(p)), &format!("info/{name}.trashinfo"));
            if let Ok(t) = std::fs::read_to_string(to_real(&info)) {
                origin = t.lines().find_map(|l| l.strip_prefix("Path=")).map(|s| s.replace("%20", " ").to_string());
            }
        }
        let origin = origin.ok_or("Original location unknown")?;
        let dest = if self.exists(&origin) { self.unique(&parent(&origin), name_of(&origin), "restored") } else { origin };
        self.fs_m(p)?.rename(p, &dest)?;
        self.trash_origin.remove(p);
        self.touch_index(&dest);
        Ok(dest)
    }
    pub fn empty_trash(&mut self) -> R<usize> {
        let Some(td) = self.trash_dir.clone() else { return Ok(0) };
        let items = self.list(&td).unwrap_or_default();
        for e in &items {
            self.fs_m(&td)?.remove(&e.path)?;
        }
        self.trash_origin.clear();
        Ok(items.len())
    }
    pub fn set_tag(&mut self, p: &str, tag: Option<&str>) {
        match tag {
            Some(t) => {
                self.tags.insert(p.to_string(), t.to_string());
            }
            None => {
                self.tags.remove(p);
            }
        }
        self.revision += 1;
    }
    pub fn bump_open(&mut self, p: &str) {
        let e = self.opens.entry(p.to_string()).or_insert((0, 0));
        e.0 += 1;
        e.1 = now_ts();
    }
    pub fn frecency(&self, p: &str) -> u32 {
        self.opens.get(p).map(|(n, t)| {
            let age = ((now_ts() - t).max(0) / 86400) as u32;
            (n * 25).min(200).saturating_sub(age.min(30) * 3)
        }).unwrap_or(0)
    }

    // ───────────── host integration ─────────────
    pub fn open_default(&mut self, p: &str) -> R<()> {
        self.bump_open(p);
        if self.is_local(p) { sys::open_default(p) } else { Err("Opened in preview".into()) }
    }
    pub fn apps_for(&self, e: &Entry) -> Vec<AppInfo> {
        let mut v = sys::apps_for(e.kind, &e.ext);
        if !self.native || v.len() < 2 {
            v.extend([
                AppInfo { id: "smart.preview".into(), name: "Smart Preview".into(), exec: None, icon: Icon::Eye, default: false },
                AppInfo { id: "smart.player".into(), name: "Smart Player".into(), exec: None, icon: Icon::Play, default: false },
                AppInfo { id: "smart.text".into(), name: "Smart Text".into(), exec: None, icon: Icon::Doc, default: false },
            ]);
        }
        if !self.native {
            v.retain(|a| a.id != "system");
        }
        v
    }
    pub fn launch(&mut self, app: &AppInfo, p: &str) -> R<()> {
        self.bump_open(p);
        if app.id.starts_with("smart.") || !self.is_local(p) { Err("Opened in preview".into()) } else { sys::launch(app, p) }
    }
    pub fn terminal_at(&self, p: &str) -> R<()> {
        sys::terminal_at(p)
    }
    pub fn unmount(&mut self, id: &str) -> R<()> {
        let i = self.mounts.iter().position(|m| m.id == id).ok_or("Unknown device")?;
        let m = self.mounts[i].clone();
        if self.native && m.id.starts_with("host:") {
            sys::unmount(&HostDisk { label: m.label.clone(), mount_point: m.root.clone(), device: m.device.clone().unwrap_or_default(), removable: true, total: m.total, free: m.free, mounted: true })?;
        }
        self.mounts[i].mounted = false;
        self.index.remove_prefix(&m.root);
        self.revision += 1;
        Ok(())
    }
    pub fn mount(&mut self, id: &str) -> R<()> {
        let i = self.mounts.iter().position(|m| m.id == id).ok_or("Unknown device")?;
        let m = self.mounts[i].clone();
        if self.native && m.id.starts_with("host:") {
            let mp = sys::mount(&HostDisk { label: m.label.clone(), mount_point: String::new(), device: m.device.clone().unwrap_or_default(), removable: true, total: m.total, free: m.free, mounted: false })?;
            self.mounts[i].root = if mp.is_empty() { m.root.clone() } else { mp };
        }
        self.mounts[i].mounted = true;
        let root = self.mounts[i].root.clone();
        self.index.queue.push(root);
        self.index.ready = false;
        self.revision += 1;
        Ok(())
    }

    // ───────────── index ─────────────
    pub fn reindex(&mut self) {
        let roots = vec![self.home.clone()];
        self.index.reset(roots);
    }
    fn touch_index(&mut self, p: &str) {
        if let Ok(e) = self.stat(p) {
            self.index.remove_prefix(p);
            self.index.add(&e);
            if e.is_dir {
                self.index.queue.push(p.to_string());
                self.index.ready = false;
            }
        }
    }
    fn index_step(&mut self, budget_ms: u128) {
        let t = Instant::now();
        while let Some(dir) = self.index.queue.pop() {
            if let Ok(list) = self.list(&dir) {
                for e in list {
                    self.index.add(&e);
                    if e.is_dir && !search::skip_dir(&e.name) {
                        self.index.queue.push(e.path.clone());
                    }
                }
            }
            self.index.dirs_done += 1;
            if t.elapsed().as_millis() >= budget_ms {
                return;
            }
        }
        if !self.index.ready {
            self.index.ready = true;
            self.revision += 1;
        }
    }
    pub fn dir_size(&self, p: &str) -> u64 {
        self.index.entries.iter().filter(|e| !e.is_dir && is_within(&e.path, p)).map(|e| e.size).sum()
    }

    // ───────────── jobs ─────────────
    fn new_job(&mut self, kind: JobKind, title: String, src_dir: &str, dest: &str, srcs: Vec<Path>) -> Job {
        let id = self.next_job;
        self.next_job += 1;
        Job {
            id, kind, title, src_dir: src_dir.into(), dest: dest.into(), srcs, state: JobState::Running, plan: vec![], cur: 0, off: 0, done_bytes: 0, total_bytes: 0,
            elapsed: 0.0, speeds: vec![], speed: 0.0, error: None, conflict: None, throttled: false, results: vec![], minimized: false, apply_all: self.settings.conflict,
            finished_at: 0.0, credit: 0.0, sample_t: 0.0, sample_b: 0, checked: false, comp: None, out_path: String::new(), ext: None, permanent: false,
        }
    }
    fn tree_size(&self, e: &Entry) -> u64 {
        if !e.is_dir {
            return e.size;
        }
        self.list(&e.path).map(|l| l.iter().map(|c| self.tree_size(c)).sum()).unwrap_or(0)
    }
    fn expand(&self, e: &Entry, dst: &str, atomic: bool, out: &mut Vec<PlanItem>) {
        let it = PlanItem { src: e.path.clone(), dst: dst.into(), is_dir: e.is_dir, size: if e.is_dir { 1 } else { e.size.max(1) }, kind: e.kind, atomic };
        out.push(it);
        if e.is_dir && !atomic {
            for c in self.list(&e.path).unwrap_or_default() {
                self.expand(&c, &join(dst, &c.name), false, out);
            }
        }
    }

    pub fn start_transfer(&mut self, srcs: &[Path], dest: &str, mv: bool) -> R<u64> {
        if srcs.is_empty() {
            return Err("Nothing to transfer".into());
        }
        if !self.stat(dest).map(|e| e.is_dir).unwrap_or(false) {
            return Err("Destination isn’t a folder".into());
        }
        let mut plan = vec![];
        let demo_big = self.settings.pace == Pace::Demo;
        let big: u64 = srcs.iter().filter_map(|s| self.stat(s).ok()).map(|e| self.tree_size(&e)).sum();
        let throttled = demo_big && big >= THROTTLE_MIN;
        let src_dir = parent(&srcs[0]);
        for s in srcs {
            let e = self.stat(s)?;
            if e.is_dir && is_within(dest, s) {
                return Err(format!("Can’t put “{}” inside itself", e.name));
            }
            if mv && parent(s) == dest {
                continue;
            }
            let same_fs = self.route(s) == self.route(dest);
            let dst = if !mv && parent(s) == dest { self.unique(dest, &e.name, "copy") } else { join(dest, &e.name) };
            self.expand(&e, &dst, mv && same_fs && !throttled, &mut plan);
        }
        if plan.is_empty() {
            return Err("Already in that folder".into());
        }
        let n = srcs.len();
        let what = if n == 1 { format!("“{}”", name_of(&srcs[0])) } else { plural(n, "item", "items") };
        let dest_label = self.mounts.iter().find(|m| m.root == dest).map(|m| m.label.clone()).unwrap_or_else(|| name_of(dest).to_string());
        let mut job = self.new_job(if mv { JobKind::Move } else { JobKind::Copy }, format!("{what} to “{dest_label}”"), &src_dir, dest, srcs.to_vec());
        job.total_bytes = plan.iter().map(|p| p.size).sum();
        job.plan = plan;
        job.throttled = throttled;
        let id = job.id;
        self.jobs.push(job);
        Ok(id)
    }

    pub fn start_delete(&mut self, srcs: &[Path]) -> u64 {
        let mut job = self.new_job(JobKind::Delete, format!("{}", plural(srcs.len(), "item", "items")), &parent(&srcs[0]), "", srcs.to_vec());
        for s in srcs {
            if let Ok(e) = self.stat(s) {
                job.plan.push(PlanItem { src: s.clone(), dst: String::new(), is_dir: e.is_dir, size: 1, kind: e.kind, atomic: true });
            }
        }
        job.total_bytes = job.plan.len() as u64;
        job.permanent = true;
        let id = job.id;
        self.jobs.push(job);
        id
    }

    pub fn start_compress(&mut self, srcs: &[Path], dest_dir: &str, fmt: Format, name: &str) -> R<u64> {
        let mut plan = vec![];
        for s in srcs {
            let e = self.stat(s)?;
            // plan.dst holds the path *inside* the archive
            self.expand(&e, &e.name, false, &mut plan);
        }
        let out = self.unique(dest_dir, &format!("{}.{}", name, fmt.ext()), "");
        let mut job = self.new_job(JobKind::Compress, format!("“{}”", name_of(&out)), &parent(&srcs[0]), dest_dir, srcs.to_vec());
        job.total_bytes = plan.iter().map(|p| p.size).sum();
        job.plan = plan;
        job.comp = Some(Compressor::new(fmt));
        job.out_path = out;
        let id = job.id;
        self.jobs.push(job);
        Ok(id)
    }

    pub fn start_extract(&mut self, archive_path: &str, dest_dir: &str) -> R<u64> {
        let e = self.stat(archive_path)?;
        if !archive::is_archive(&e.name) {
            return Err("Not a supported archive".into());
        }
        let bytes = self.read_all(archive_path, 1 << 30)?;
        let items = archive::read_all(bytes, &e.name)?;
        let single = items.len() == 1 && !items[0].1 && !e.name.to_lowercase().ends_with(".zip");
        let root = if single { dest_dir.to_string() } else { self.unique(dest_dir, &archive::extract_stem(&e.name), "") };
        let mut job = self.new_job(JobKind::Extract, format!("“{}”", e.name), &parent(archive_path), &root, vec![archive_path.into()]);
        job.total_bytes = items.iter().map(|i| i.2.len() as u64 + 1).sum();
        job.plan = items.iter().map(|i| PlanItem { src: i.0.clone(), dst: join(&root, &i.0), is_dir: i.1, size: i.2.len() as u64 + 1, kind: if i.1 { Kind::Folder } else { Kind::from_ext(&ext_of(&i.0)) }, atomic: false }).collect();
        job.ext = Some(items);
        if !single {
            let _ = self.fs_m(&root).and_then(|f| f.mkdir(&root));
        }
        let id = job.id;
        self.jobs.push(job);
        Ok(id)
    }

    pub fn job_mut(&mut self, id: u64) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }
    pub fn pause_job(&mut self, id: u64) {
        if let Some(j) = self.job_mut(id) {
            j.state = match j.state {
                JobState::Running => JobState::Paused,
                JobState::Paused => JobState::Running,
                s => s,
            };
        }
    }
    pub fn cancel_job(&mut self, id: u64) {
        let mut cleanup = None;
        if let Some(j) = self.job_mut(id) {
            if j.active() {
                j.state = JobState::Cancelled;
                if matches!(j.kind, JobKind::Copy | JobKind::Move) {
                    if let Some(p) = j.plan.get(j.cur) {
                        if j.off > 0 {
                            cleanup = Some(p.dst.clone());
                        }
                    }
                }
            }
        }
        if let Some(p) = cleanup {
            let _ = self.delete_permanent(&p); // never leave a half-written file behind
        }
    }
    pub fn resolve(&mut self, id: u64, r: Resolution, all: bool) {
        let keep = self.jobs.iter().find(|j| j.id == id && j.state == JobState::NeedsDecision).map(|j| j.plan[j.cur].dst.clone()).map(|d| self.unique(&parent(&d), name_of(&d), ""));
        if let Some(j) = self.job_mut(id) {
            if j.state != JobState::NeedsDecision {
                return;
            }
            j.conflict = None;
            j.state = JobState::Running;
            j.apply_all = if all { Some(r) } else { None };
            j.checked = true;
            match r {
                Resolution::Skip => {
                    j.done_bytes += j.plan[j.cur].size;
                    j.cur += 1;
                    j.checked = false;
                }
                Resolution::KeepBoth => {
                    if let Some(u) = keep {
                        let c = j.cur;
                        j.plan[c].dst = u;
                    }
                }
                Resolution::Replace => {
                    let d = j.plan[j.cur].dst.clone();
                    j.credit += 0.0;
                    j.error = None;
                    j.conflict = Some(String::new()); // marker consumed by engine.tick
                    let _ = d;
                }
            }
        }
        // Replace: remove the existing destination now
        let rep = self.jobs.iter().find(|j| j.id == id && j.conflict.as_deref() == Some("")).map(|j| j.plan[j.cur].dst.clone());
        if let Some(d) = rep {
            let _ = self.delete_permanent(&d);
            if let Some(j) = self.job_mut(id) {
                j.conflict = None;
            }
        }
    }

    pub fn tick(&mut self, dt: f64) {
        let dt = dt.clamp(0.0, 0.1);
        self.clock += dt;
        self.index_step(if self.index.ready { 0 } else { 3 });
        let mut i = 0;
        while i < self.jobs.len() {
            if self.jobs[i].state == JobState::Running {
                let mut job = self.jobs.remove(i);
                self.step(&mut job, dt);
                self.jobs.insert(i, job);
            }
            i += 1;
        }
        // drop finished jobs after a while
        let now = self.clock;
        self.jobs.retain(|j| j.active() || j.finished_at == 0.0 || now - j.finished_at < 600.0);
    }

    fn finish(&mut self, job: &mut Job, ok: bool) {
        job.finished_at = self.clock.max(0.001);
        job.state = if ok { JobState::Done } else { JobState::Failed };
        if !ok {
            self.notices.push(Notice { text: job.error.clone().unwrap_or_else(|| "Something went wrong".into()), icon: Icon::Warning, good: false });
            return;
        }
        let tops: Vec<Path> = job.results.iter().map(|r| r.1.clone()).collect();
        match job.kind {
            JobKind::Copy => self.undo.push(Undo::Created(tops.clone())),
            JobKind::Move => self.undo.push(Undo::Moved(job.results.clone())),
            JobKind::Compress | JobKind::Extract => self.undo.push(Undo::Created(tops.clone())),
            JobKind::Delete => {}
        }
        if job.kind == JobKind::Move {
            for s in job.srcs.clone() {
                if self.route(&s).is_some() && self.exists(&s) {
                    let _ = self.delete_permanent(&s);
                }
                self.index.remove_prefix(&s);
            }
        }
        if job.kind == JobKind::Delete {
            for s in &job.srcs {
                self.index.remove_prefix(s);
            }
        }
        for t in &tops {
            self.touch_index(t);
        }
        self.revision += 1;
        let text = match job.kind {
            JobKind::Copy => format!("Copied {}", job.title),
            JobKind::Move => format!("Moved {}", job.title),
            JobKind::Delete => format!("Deleted {}", job.title),
            JobKind::Compress => format!("Created {}", job.title),
            JobKind::Extract => format!("Extracted {}", job.title),
        };
        self.notices.push(Notice { text, icon: Icon::CheckCircle, good: true });
    }

    fn step(&mut self, job: &mut Job, dt: f64) {
        job.elapsed += dt;
        let before = job.done_bytes;
        // pacing for demo transfers: a believable, wobbling link speed
        let rate = if job.throttled {
            let t = job.elapsed;
            let wob = 1.0 + 0.22 * (t * 1.3).sin() + 0.1 * (t * 4.1).sin() - if (t * 0.35).sin() > 0.93 { 0.5 } else { 0.0 };
            250e6 * wob.max(0.15)
        } else {
            f64::INFINITY
        };
        if job.throttled {
            job.credit += rate * dt;
        }
        let deadline = Instant::now();
        let budget_ms = 10u128;
        match job.kind {
            JobKind::Copy | JobKind::Move => self.step_transfer(job, deadline, budget_ms),
            JobKind::Delete => self.step_delete(job, deadline, budget_ms),
            JobKind::Compress => self.step_compress(job, deadline, budget_ms),
            JobKind::Extract => self.step_extract(job, deadline, budget_ms),
        }
        // speed sampling (4 Hz)
        if job.elapsed - job.sample_t >= 0.25 {
            let inst = (job.done_bytes - job.sample_b) as f64 / (job.elapsed - job.sample_t);
            job.speed = if job.speed == 0.0 { inst } else { job.speed * 0.6 + inst * 0.4 };
            job.speeds.push(inst as f32);
            if job.speeds.len() > 80 {
                job.speeds.remove(0);
            }
            job.sample_t = job.elapsed;
            job.sample_b = job.done_bytes;
        }
        let _ = before;
    }

    fn step_transfer(&mut self, job: &mut Job, t0: Instant, budget_ms: u128) {
        loop {
            if job.state != JobState::Running {
                return;
            }
            if job.cur >= job.plan.len() {
                return self.finish(job, true);
            }
            if job.throttled && job.credit < 1.0 && !job.plan[job.cur].is_dir {
                return;
            }
            let it = job.plan[job.cur].clone();
            let is_top = job.srcs.iter().any(|s| *s == it.src);
            let fail = |job: &mut Job, e: String| {
                job.error = Some(e);
            };
            if it.is_dir && !it.atomic {
                if let Err(e) = self.fs_m(&it.dst).and_then(|f| if f.exists(&it.dst) { Ok(()) } else { f.mkdir(&it.dst) }) {
                    fail(job, e);
                    return self.finish(job, false);
                }
                if is_top {
                    job.results.push((it.src.clone(), it.dst.clone()));
                }
                job.done_bytes += it.size;
                job.cur += 1;
                continue;
            }
            // conflicts (checked once per item)
            let dst = it.dst.clone();
            if job.off == 0 && !job.checked {
                if self.exists(&dst) {
                    match job.apply_all {
                        None => {
                            job.conflict = Some(name_of(&dst).to_string());
                            job.state = JobState::NeedsDecision;
                            return;
                        }
                        Some(Resolution::Skip) => {
                            job.done_bytes += it.size;
                            job.cur += 1;
                            continue;
                        }
                        Some(Resolution::KeepBoth) => {
                            let u = self.unique(&parent(&dst), name_of(&dst), "");
                            job.plan[job.cur].dst = u;
                        }
                        Some(Resolution::Replace) => {
                            let _ = self.delete_permanent(&dst);
                        }
                    }
                }
                job.checked = true;
                continue;
            }
            let dst = job.plan[job.cur].dst.clone();
            // atomic move (rename)
            if it.atomic {
                match self.fs_m(&it.src).and_then(|f| f.rename(&it.src, &dst)) {
                    Ok(()) => {
                        self.migrate(&it.src, &dst);
                        job.results.push((it.src.clone(), dst));
                        job.done_bytes += it.size;
                        job.cur += 1;
                        job.checked = false;
                        job.srcs.retain(|s| *s != it.src); // already moved: nothing to clean up
                        continue;
                    }
                    Err(e) => {
                        fail(job, e);
                        return self.finish(job, false);
                    }
                }
            }
            // chunked copy
            let want = if job.throttled { (job.credit as u64).clamp(1, 4 << 20) } else { 4 << 20 };
            let len = want.min(it.size.saturating_sub(job.off));
            let chunk = match self.fs(&it.src).and_then(|f| f.read_at(&it.src, job.off, len)) {
                Ok(c) => c,
                Err(e) => {
                    fail(job, e);
                    return self.finish(job, false);
                }
            };
            let n = chunk.len();
            if let Err(e) = self.fs_m(&dst).and_then(|f| f.write_chunk(&dst, job.off, chunk)) {
                fail(job, e);
                return self.finish(job, false);
            }
            job.off += n;
            job.done_bytes += n;
            if job.throttled {
                job.credit -= n as f64;
            }
            // planned size 1 may really be an empty file
            let real = if it.size == 1 { self.stat(&it.src).map(|e| e.size).unwrap_or(0) } else { it.size };
            if job.off >= real || n == 0 {
                if is_top {
                    job.results.push((it.src.clone(), dst.clone()));
                } else if !job.results.iter().any(|r| is_within(&dst, &r.1)) {
                    job.results.push((it.src.clone(), dst.clone()));
                }
                // top-level progress accounting for planned size=1 empty files
                if real == 0 {
                    job.done_bytes += 1;
                }
                job.cur += 1;
                job.off = 0;
                job.checked = false;
            }
            if t0.elapsed().as_millis() >= budget_ms && !job.throttled {
                return;
            }
            if job.throttled && job.credit < 1.0 {
                return;
            }
        }
    }

    fn step_delete(&mut self, job: &mut Job, t0: Instant, budget_ms: u128) {
        while job.cur < job.plan.len() {
            let p = job.plan[job.cur].src.clone();
            if let Err(e) = self.delete_permanent(&p) {
                job.error = Some(e);
                return self.finish(job, false);
            }
            job.done_bytes += 1;
            job.cur += 1;
            if t0.elapsed().as_millis() >= budget_ms {
                return;
            }
        }
        self.finish(job, true)
    }

    fn step_compress(&mut self, job: &mut Job, t0: Instant, budget_ms: u128) {
        while job.cur < job.plan.len() {
            let it = job.plan[job.cur].clone();
            let mtime = self.stat(&it.src).map(|e| e.modified).unwrap_or(0);
            let r = if it.is_dir {
                job.comp.as_mut().unwrap().add_dir(&it.dst, mtime)
            } else {
                match self.read_all(&it.src, 768 << 20) {
                    Ok(d) => job.comp.as_mut().unwrap().add_file(&it.dst, &d, mtime),
                    Err(e) => Err(e),
                }
            };
            if let Err(e) = r {
                job.error = Some(e);
                return self.finish(job, false);
            }
            job.done_bytes += it.size;
            job.cur += 1;
            if t0.elapsed().as_millis() >= budget_ms {
                return;
            }
        }
        let comp = job.comp.take().unwrap();
        match comp.finish().and_then(|bytes| {
            let out = job.out_path.clone();
            self.fs_m(&out)?.write_chunk(&out, 0, Chunk::Data(bytes))?;
            Ok(out)
        }) {
            Ok(out) => {
                job.results.push((String::new(), out));
                self.finish(job, true)
            }
            Err(e) => {
                job.error = Some(e);
                self.finish(job, false)
            }
        }
    }

    fn step_extract(&mut self, job: &mut Job, t0: Instant, budget_ms: u128) {
        while job.cur < job.plan.len() {
            let it = job.plan[job.cur].clone();
            let data = job.ext.as_mut().map(|v| std::mem::take(&mut v[job.cur].2)).unwrap_or_default();
            let r = (|| -> R<()> {
                self.ensure_dir(&parent(&it.dst))?;
                if it.is_dir {
                    self.ensure_dir(&it.dst)
                } else {
                    let d = if self.exists(&it.dst) { self.unique(&parent(&it.dst), name_of(&it.dst), "") } else { it.dst.clone() };
                    self.fs_m(&d)?.write_chunk(&d, 0, Chunk::Data(data))
                }
            })();
            if let Err(e) = r {
                job.error = Some(e);
                return self.finish(job, false);
            }
            job.done_bytes += it.size;
            job.cur += 1;
            if t0.elapsed().as_millis() >= budget_ms {
                return;
            }
        }
        job.results.push((job.srcs[0].clone(), job.dest.clone()));
        self.finish(job, true)
    }

    fn ensure_dir(&mut self, p: &str) -> R<()> {
        if p == "/" || self.exists(p) {
            return Ok(());
        }
        self.ensure_dir(&parent(p))?;
        self.fs_m(p)?.mkdir(p)
    }

    // ───────────── undo ─────────────
    pub fn undo_last(&mut self) -> R<String> {
        let u = self.undo.pop().ok_or("Nothing to undo")?;
        match u {
            Undo::Rename { from, to } => {
                self.rename(&to, &from)?;
                Ok(format!("Renamed back to “{}”", name_of(&from)))
            }
            Undo::Created(ps) => {
                for p in &ps {
                    let _ = self.delete_permanent(p);
                }
                Ok(format!("Removed {}", plural(ps.len(), "created item", "created items")))
            }
            Undo::Moved(pairs) => {
                for (from, to) in &pairs {
                    let _ = self.fs_m(to).and_then(|f| f.rename(to, from));
                    self.migrate(to, from);
                    self.index.remove_prefix(to);
                    self.touch_index(from);
                }
                Ok(format!("Moved {} back", plural(pairs.len(), "item", "items")))
            }
            Undo::Trashed(pairs) => {
                for (_, t) in &pairs {
                    let _ = self.restore(t);
                }
                Ok(format!("Restored {}", plural(pairs.len(), "item", "items")))
            }
        }
    }
}
