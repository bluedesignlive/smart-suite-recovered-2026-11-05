//! Virtual filesystem abstraction. Every storage (disk, memory, cloud, phone) is a `Vfs`.
use crate::model::*;
use std::collections::BTreeMap;

pub enum Chunk {
    Zeros(u64),
    Data(Vec<u8>),
}
impl Chunk {
    pub fn len(&self) -> u64 {
        match self {
            Chunk::Zeros(n) => *n,
            Chunk::Data(d) => d.len() as u64,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            Chunk::Zeros(n) => vec![0; n as usize],
            Chunk::Data(d) => d,
        }
    }
}

pub trait Vfs {
    fn read_dir(&self, p: &str) -> R<Vec<Entry>>;
    fn stat(&self, p: &str) -> R<Entry>;
    fn mkdir(&mut self, p: &str) -> R<()>;
    fn remove(&mut self, p: &str) -> R<()>;
    fn rename(&mut self, from: &str, to: &str) -> R<()>;
    fn read_at(&self, p: &str, off: u64, len: u64) -> R<Chunk>;
    /// `off == 0` creates / truncates the file, otherwise appends at `off`.
    fn write_chunk(&mut self, p: &str, off: u64, c: Chunk) -> R<()>;
    fn local_path(&self, _p: &str) -> Option<std::path::PathBuf> {
        None
    }
    fn exists(&self, p: &str) -> bool {
        self.stat(p).is_ok()
    }
    /// Create a file of `size` bytes without materialising it (sparse where supported).
    fn create_sparse(&mut self, p: &str, size: u64) -> R<()> {
        let mut left = size.min(1 << 20);
        self.write_chunk(p, 0, Chunk::Zeros(0))?;
        while left > 0 {
            self.write_chunk(p, 1, Chunk::Zeros(left))?;
            left = 0;
        }
        Ok(())
    }
}

// ───────────────────────── in-memory fs ─────────────────────────
#[derive(Clone)]
pub enum Content {
    Static(&'static [u8]),
    /// Real bytes padded with zeros up to a declared size (lets demo images look like 1.5 MB originals).
    Sized(&'static [u8], u64),
    Buf(Vec<u8>),
    /// Sparse content of the given size (reads as zeros). Lets the demo hold a 5 GB file in a browser.
    Synth(u64),
}
impl Content {
    fn len(&self) -> u64 {
        match self {
            Content::Static(s) => s.len() as u64,
            Content::Sized(_, n) => *n,
            Content::Buf(b) => b.len() as u64,
            Content::Synth(n) => *n,
        }
    }
}
#[derive(Clone)]
struct Node {
    dir: bool,
    content: Content,
    modified: i64,
}

#[derive(Default)]
pub struct MemFs {
    nodes: BTreeMap<String, Node>,
}

impl MemFs {
    pub fn new(root: &str) -> Self {
        let mut m = MemFs::default();
        m.put_dir(root, now_ts());
        m
    }
    pub fn put_dir(&mut self, p: &str, modified: i64) {
        if p != "/" && !self.nodes.contains_key(p) {
            self.put_dir(&parent(p), modified);
        }
        self.nodes.entry(p.to_string()).or_insert(Node { dir: true, content: Content::Buf(vec![]), modified });
    }
    pub fn put_file(&mut self, p: &str, content: Content, modified: i64) {
        self.put_dir(&parent(p), modified);
        self.nodes.insert(p.to_string(), Node { dir: false, content, modified });
    }
    fn prefix(p: &str) -> String {
        if p == "/" { "/".into() } else { format!("{p}/") }
    }
    fn descendants(&self, p: &str) -> Vec<String> {
        let pre = Self::prefix(p);
        self.nodes.range(pre.clone()..).take_while(|(k, _)| k.starts_with(&pre)).map(|(k, _)| k.clone()).collect()
    }
    fn entry_of(&self, p: &str, n: &Node) -> Entry {
        let mut e = Entry::new(p, n.dir, if n.dir { 0 } else { n.content.len() }, n.modified);
        if n.dir {
            e.children = Some(self.count_children(p));
        }
        e
    }
    fn count_children(&self, p: &str) -> u32 {
        let pre = Self::prefix(p);
        self.nodes.range(pre.clone()..).take_while(|(k, _)| k.starts_with(&pre)).filter(|(k, _)| !k[pre.len()..].contains('/')).count() as u32
    }
}

impl Vfs for MemFs {
    fn read_dir(&self, p: &str) -> R<Vec<Entry>> {
        match self.nodes.get(p) {
            Some(n) if n.dir => {}
            Some(_) => return Err("Not a folder".into()),
            None => return Err(format!("“{}” doesn’t exist", name_of(p))),
        }
        let pre = Self::prefix(p);
        Ok(self
            .nodes
            .range(pre.clone()..)
            .take_while(|(k, _)| k.starts_with(&pre))
            .filter(|(k, _)| k.len() > pre.len() && !k[pre.len()..].contains('/'))
            .map(|(k, n)| self.entry_of(k, n))
            .collect())
    }
    fn stat(&self, p: &str) -> R<Entry> {
        self.nodes.get(p).map(|n| self.entry_of(p, n)).ok_or_else(|| format!("“{}” doesn’t exist", name_of(p)))
    }
    fn mkdir(&mut self, p: &str) -> R<()> {
        if self.nodes.contains_key(p) {
            return Err(format!("“{}” already exists", name_of(p)));
        }
        if !self.nodes.contains_key(&parent(p)) {
            return Err("Parent folder is missing".into());
        }
        self.nodes.insert(p.to_string(), Node { dir: true, content: Content::Buf(vec![]), modified: now_ts() });
        Ok(())
    }
    fn remove(&mut self, p: &str) -> R<()> {
        for k in self.descendants(p) {
            self.nodes.remove(&k);
        }
        self.nodes.remove(p).map(|_| ()).ok_or_else(|| "Nothing to remove".into())
    }
    fn rename(&mut self, from: &str, to: &str) -> R<()> {
        if self.nodes.contains_key(to) {
            return Err(format!("“{}” already exists", name_of(to)));
        }
        if !self.nodes.contains_key(&parent(to)) {
            return Err("Destination folder is missing".into());
        }
        let mut keys = self.descendants(from);
        keys.push(from.to_string());
        for k in keys {
            if let Some(n) = self.nodes.remove(&k) {
                self.nodes.insert(format!("{}{}", to, &k[from.len()..]), n);
            }
        }
        Ok(())
    }
    fn read_at(&self, p: &str, off: u64, len: u64) -> R<Chunk> {
        let n = self.nodes.get(p).ok_or("File not found")?;
        let total = n.content.len();
        let end = (off + len).min(total);
        let start = off.min(end);
        Ok(match &n.content {
            Content::Synth(_) => Chunk::Zeros(end - start),
            Content::Static(s) => Chunk::Data(s[start as usize..end as usize].to_vec()),
            Content::Sized(s, _) => {
                let mut v = vec![0u8; (end - start) as usize];
                let real = s.len() as u64;
                if start < real {
                    let e = end.min(real);
                    v[..(e - start) as usize].copy_from_slice(&s[start as usize..e as usize]);
                }
                Chunk::Data(v)
            }
            Content::Buf(b) => Chunk::Data(b[start as usize..end as usize].to_vec()),
        })
    }
    fn write_chunk(&mut self, p: &str, off: u64, c: Chunk) -> R<()> {
        if off == 0 {
            if !self.nodes.contains_key(&parent(p)) {
                return Err("Destination folder is missing".into());
            }
            let content = match c {
                Chunk::Zeros(n) if n > (64 << 20) => Content::Synth(n),
                Chunk::Zeros(n) => Content::Buf(vec![0; n as usize]),
                Chunk::Data(d) => Content::Buf(d),
            };
            self.nodes.insert(p.to_string(), Node { dir: false, content, modified: now_ts() });
            return Ok(());
        }
        let n = self.nodes.get_mut(p).ok_or("Partial file vanished")?;
        match (&mut n.content, c) {
            (Content::Synth(s), c) => *s += c.len(),
            (Content::Buf(b), Chunk::Data(d)) => b.extend_from_slice(&d),
            (Content::Buf(b), Chunk::Zeros(z)) => {
                if b.len() as u64 + z > (64 << 20) {
                    n.content = Content::Synth(b.len() as u64 + z);
                } else {
                    b.resize(b.len() + z as usize, 0);
                }
            }
            (Content::Static(_) | Content::Sized(..), c) => {
                let mut v = match &n.content {
                    Content::Static(s) | Content::Sized(s, _) => s.to_vec(),
                    _ => vec![],
                };
                v.extend(c.into_vec());
                n.content = Content::Buf(v);
            }
        }
        Ok(())
    }
}

// ───────────────────────── real filesystem ─────────────────────────
#[cfg(not(target_arch = "wasm32"))]
pub use std_fs::{from_real, to_real, StdFs};

#[cfg(not(target_arch = "wasm32"))]
mod std_fs {
    use super::*;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::PathBuf;

    #[derive(Default)]
    pub struct StdFs;

    pub fn to_real(p: &str) -> PathBuf {
        #[cfg(windows)]
        {
            // "/C/Users/x" → "C:\Users\x"
            let t = p.trim_start_matches('/');
            let (d, rest) = t.split_once('/').unwrap_or((t, ""));
            return PathBuf::from(format!("{d}:\\{}", rest.replace('/', "\\")));
        }
        #[cfg(not(windows))]
        PathBuf::from(p)
    }
    pub fn from_real(p: &std::path::Path) -> String {
        let s = p.to_string_lossy().replace('\\', "/");
        #[cfg(windows)]
        {
            if let Some((d, rest)) = s.split_once(':') {
                return format!("/{d}{rest}");
            }
        }
        s
    }
    fn secs(t: std::io::Result<std::time::SystemTime>) -> i64 {
        t.ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64).unwrap_or(0)
    }
    fn entry(path: &str, md: &std::fs::Metadata) -> Entry {
        Entry::new(path, md.is_dir(), if md.is_dir() { 0 } else { md.len() }, secs(md.modified()))
    }
    fn err(e: std::io::Error, p: &str) -> String {
        match e.kind() {
            std::io::ErrorKind::NotFound => format!("“{}” doesn’t exist", name_of(p)),
            std::io::ErrorKind::PermissionDenied => format!("No permission to access “{}”", name_of(p)),
            _ => e.to_string(),
        }
    }

    impl Vfs for StdFs {
        fn read_dir(&self, p: &str) -> R<Vec<Entry>> {
            let rd = std::fs::read_dir(to_real(p)).map_err(|e| err(e, p))?;
            let mut out = Vec::new();
            for e in rd.flatten() {
                let full = join(p, &e.file_name().to_string_lossy());
                // follow symlinks for type, fall back to the link itself
                if let Ok(md) = std::fs::metadata(e.path()).or_else(|_| e.metadata()) {
                    out.push(entry(&full, &md));
                }
            }
            Ok(out)
        }
        fn stat(&self, p: &str) -> R<Entry> {
            std::fs::metadata(to_real(p)).map(|m| entry(p, &m)).map_err(|e| err(e, p))
        }
        fn mkdir(&mut self, p: &str) -> R<()> {
            std::fs::create_dir(to_real(p)).map_err(|e| err(e, p))
        }
        fn remove(&mut self, p: &str) -> R<()> {
            let r = to_real(p);
            let md = std::fs::symlink_metadata(&r).map_err(|e| err(e, p))?;
            (if md.is_dir() { std::fs::remove_dir_all(&r) } else { std::fs::remove_file(&r) }).map_err(|e| err(e, p))
        }
        fn rename(&mut self, from: &str, to: &str) -> R<()> {
            if to_real(to).exists() {
                return Err(format!("“{}” already exists", name_of(to)));
            }
            std::fs::rename(to_real(from), to_real(to)).map_err(|e| err(e, from))
        }
        fn read_at(&self, p: &str, off: u64, len: u64) -> R<Chunk> {
            let mut f = std::fs::File::open(to_real(p)).map_err(|e| err(e, p))?;
            f.seek(SeekFrom::Start(off)).map_err(|e| e.to_string())?;
            let mut buf = Vec::with_capacity(len.min(1 << 24) as usize);
            f.take(len).read_to_end(&mut buf).map_err(|e| e.to_string())?;
            Ok(Chunk::Data(buf))
        }
        fn write_chunk(&mut self, p: &str, off: u64, c: Chunk) -> R<()> {
            let r = to_real(p);
            let mut f = if off == 0 {
                std::fs::File::create(&r)
            } else {
                std::fs::OpenOptions::new().append(true).open(&r)
            }
            .map_err(|e| err(e, p))?;
            match c {
                Chunk::Data(d) => f.write_all(&d),
                Chunk::Zeros(n) => f.write_all(&vec![0u8; n as usize]),
            }
            .map_err(|e| err(e, p))
        }
        fn local_path(&self, p: &str) -> Option<PathBuf> {
            Some(to_real(p))
        }
        fn create_sparse(&mut self, p: &str, size: u64) -> R<()> {
            let f = std::fs::File::create(to_real(p)).map_err(|e| err(e, p))?;
            f.set_len(size).map_err(|e| e.to_string())
        }
    }
}
