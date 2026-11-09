//! Plain data types shared by every layer. No UI, no IO.

pub type Path = String;
pub type R<T> = Result<T, String>;

// ───────────────────────── paths ─────────────────────────
pub fn parent(p: &str) -> String {
    if p == "/" {
        return "/".into();
    }
    match p.rfind('/') {
        Some(0) | None => "/".into(),
        Some(i) => p[..i].into(),
    }
}
pub fn join(a: &str, b: &str) -> String {
    if a == "/" {
        format!("/{b}")
    } else {
        format!("{}/{}", a.trim_end_matches('/'), b)
    }
}
pub fn name_of(p: &str) -> &str {
    if p == "/" {
        "/"
    } else {
        p.rsplit('/').next().unwrap_or(p)
    }
}
pub fn ext_of(name: &str) -> String {
    match name.rfind('.') {
        Some(i) if i > 0 && i + 1 < name.len() => name[i + 1..].to_lowercase(),
        _ => String::new(),
    }
}
pub fn stem_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(i) if i > 0 => &name[..i],
        _ => name,
    }
}
pub fn is_within(p: &str, root: &str) -> bool {
    root == "/" || p == root || (p.starts_with(root) && p.as_bytes().get(root.len()) == Some(&b'/'))
}

// ───────────────────────── kinds ─────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Folder,
    Image,
    Audio,
    Video,
    Document,
    Text,
    Code,
    Archive,
    App,
    Data,
    Other,
}

impl Kind {
    pub fn from_ext(e: &str) -> Kind {
        match e {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "heic" | "avif" | "tiff" | "ico" => Kind::Image,
            "wav" | "mp3" | "flac" | "ogg" | "m4a" | "aac" | "opus" | "aiff" => Kind::Audio,
            "mp4" | "mov" | "mkv" | "webm" | "avi" | "m4v" => Kind::Video,
            "pdf" | "doc" | "docx" | "odt" | "rtf" | "pages" | "ppt" | "pptx" | "key" | "xls" | "xlsx" | "ods" => Kind::Document,
            "txt" | "md" | "log" | "toml" | "yaml" | "yml" | "ini" | "cfg" | "csv" => Kind::Text,
            "rs" | "js" | "ts" | "py" | "c" | "h" | "cpp" | "go" | "java" | "html" | "css" | "json" | "sh" | "tsx" | "jsx" | "sql" => Kind::Code,
            "zip" | "tar" | "gz" | "tgz" | "xz" | "txz" | "bz2" | "tbz2" | "7z" | "rar" | "zst" => Kind::Archive,
            "appimage" | "exe" | "dmg" | "deb" | "rpm" | "apk" | "msi" | "app" => Kind::App,
            "flp" | "db" | "sqlite" | "bin" | "iso" | "dat" => Kind::Data,
            _ => Kind::Other,
        }
    }
    pub fn plural(self) -> &'static str {
        match self {
            Kind::Folder => "Folders",
            Kind::Image => "Images",
            Kind::Audio => "Audio",
            Kind::Video => "Videos",
            Kind::Document => "Documents",
            Kind::Text => "Text",
            Kind::Code => "Code",
            Kind::Archive => "Archives",
            Kind::App => "Apps",
            Kind::Data => "Data",
            Kind::Other => "Other",
        }
    }
    pub fn singular(self) -> &'static str {
        match self {
            Kind::Folder => "Folder",
            Kind::Image => "Image",
            Kind::Audio => "Audio",
            Kind::Video => "Video",
            Kind::Document => "Document",
            Kind::Text => "Text",
            Kind::Code => "Code",
            Kind::Archive => "Archive",
            Kind::App => "App",
            Kind::Data => "Data",
            Kind::Other => "File",
        }
    }
    pub fn icon(self) -> Icon {
        match self {
            Kind::Folder => Icon::Folder,
            Kind::Image => Icon::Image,
            Kind::Audio => Icon::Music,
            Kind::Video => Icon::Film,
            Kind::Document => Icon::Pdf,
            Kind::Text => Icon::Doc,
            Kind::Code => Icon::Code,
            Kind::Archive => Icon::Zip,
            Kind::App => Icon::App,
            Kind::Data => Icon::Cube,
            Kind::Other => Icon::File,
        }
    }
}

pub fn kind_label(ext: &str, is_dir: bool) -> String {
    if is_dir {
        return "Folder".into();
    }
    let s = match ext {
        "jpg" | "jpeg" => "JPEG image",
        "png" => "PNG image",
        "gif" => "GIF image",
        "webp" => "WebP image",
        "svg" => "SVG vector",
        "wav" => "WAV audio",
        "mp3" => "MP3 audio",
        "flac" => "FLAC audio",
        "mp4" => "MPEG-4 video",
        "pdf" => "PDF document",
        "txt" => "Plain text",
        "md" => "Markdown",
        "zip" => "ZIP archive",
        "tar" => "TAR archive",
        "gz" | "tgz" => "Gzip archive",
        "appimage" => "Linux app",
        "flp" => "FL Studio project",
        "" => "File",
        other => return format!("{} file", other.to_uppercase()),
    };
    s.into()
}

// ───────────────────────── icons (frontend maps these to glyphs) ─────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Home, Clock, Monitor, Doc, Download, Image, Music, Film, Drive, Globe, Cloud, Folder, FolderOpen, File, Zip, Pdf,
    Code, Search, Sparkle, Grid, List, Columns, Sort, More, Back, Forward, Up, ChevronDown, ChevronRight, Sidebar, Copy,
    Cut, Paste, Trash, Rename, Share, Tag, Archive, Eye, Info, Plus, Close, Check, CheckCircle, Refresh, NewFolder,
    NewFile, Terminal, Play, Pause, Stop, Eject, Usb, Phone, Broom, Pie, Command, Link, Move, Star, Lock, Warning, Send,
    Moon, Sun, Gear, Keyboard, Upload, Wand, Undo, Restore, EyeOff, SelectAll, App, OpenExt, Duplicate, Wave, Cube,
    Rocket, Layers, Sliders, Hash, Bolt, Plug, ArrowDown, ArrowUp,
}

// ───────────────────────── entries ─────────────────────────
#[derive(Clone, Debug)]
pub struct Entry {
    pub path: Path,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: i64,
    pub kind: Kind,
    pub ext: String,
    pub hidden: bool,
    pub children: Option<u32>,
}

impl Entry {
    pub fn new(path: &str, is_dir: bool, size: u64, modified: i64) -> Entry {
        let name = name_of(path).to_string();
        let ext = if is_dir { String::new() } else { ext_of(&name) };
        Entry {
            kind: if is_dir { Kind::Folder } else { Kind::from_ext(&ext) },
            hidden: name.starts_with('.'),
            path: path.to_string(),
            name,
            is_dir,
            size,
            modified,
            ext,
            children: None,
        }
    }
    pub fn kind_label(&self) -> String {
        kind_label(&self.ext, self.is_dir)
    }
}

// ───────────────────────── tags ─────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct TagDef {
    pub id: &'static str,
    pub label: &'static str,
    pub rgb: [u8; 3],
}
pub const TAGS: [TagDef; 5] = [
    TagDef { id: "design", label: "Design", rgb: [47, 111, 235] },
    TagDef { id: "amapiano", label: "Amapiano", rgb: [201, 134, 26] },
    TagDef { id: "archive", label: "Archive", rgb: [43, 155, 109] },
    TagDef { id: "urgent", label: "Urgent", rgb: [211, 68, 80] },
    TagDef { id: "review", label: "Review", rgb: [138, 92, 214] },
];
pub fn tag_def(id: &str) -> Option<&'static TagDef> {
    TAGS.iter().find(|t| t.id == id)
}

// ───────────────────────── formatting ─────────────────────────
pub fn fmt_size(b: u64) -> String {
    let b = b as f64;
    if b < 1000.0 {
        format!("{} B", b as u64)
    } else if b < 1e6 {
        format!("{:.0} KB", b / 1e3)
    } else if b < 1e9 {
        let v = b / 1e6;
        if v < 10.0 { format!("{:.1} MB", v) } else { format!("{:.0} MB", v) }
    } else if b < 1e12 {
        format!("{:.1} GB", b / 1e9)
    } else {
        format!("{:.2} TB", b / 1e12)
    }
}
pub fn fmt_speed(bps: f64) -> String {
    format!("{}/s", fmt_size(bps.max(0.0) as u64))
}
pub fn fmt_eta(secs: f64) -> String {
    if !secs.is_finite() || secs < 0.0 {
        return "calculating…".into();
    }
    let s = secs.round() as u64;
    if s < 60 {
        format!("{s}s left")
    } else if s < 3600 {
        format!("{}m {:02}s left", s / 60, s % 60)
    } else {
        format!("{}h {:02}m left", s / 3600, (s / 60) % 60)
    }
}
pub fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{} {}", n, if n == 1 { one } else { many })
}
pub fn now_ts() -> i64 {
    chrono::Utc::now().timestamp()
}
pub fn fmt_date(ts: i64) -> String {
    use chrono::{Datelike, TimeZone};
    let Some(d) = chrono::Local.timestamp_opt(ts, 0).single() else { return "—".into() };
    let now = chrono::Local::now();
    let days = (now.date_naive() - d.date_naive()).num_days();
    let t = d.format("%H:%M");
    match days {
        0 => format!("Today, {t}"),
        1 => format!("Yesterday, {t}"),
        _ if d.year() == now.year() => format!("{}, {t}", d.format("%b %-d")),
        _ => format!("{}", d.format("%b %-d, %Y")),
    }
}
pub fn fmt_full_date(ts: i64) -> String {
    use chrono::TimeZone;
    chrono::Local.timestamp_opt(ts, 0).single().map(|d| d.format("%A, %-d %B %Y at %H:%M").to_string()).unwrap_or_default()
}
pub fn clock_strings() -> (String, String) {
    let n = chrono::Local::now();
    (n.format("%H:%M").to_string(), n.format("%a %-d %B").to_string())
}
