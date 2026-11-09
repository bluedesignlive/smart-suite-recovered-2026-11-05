//! Demo world used in the browser (and for the optional native demo workspace).
use crate::archive::{Compressor, Format};
use crate::engine::*;
use crate::model::*;
use crate::vfs::*;

macro_rules! thumb {
    ($n:literal) => {
        include_bytes!(concat!("../../../assets/thumbs/", $n, ".jpg")) as &'static [u8]
    };
}
pub const MB: u64 = 1_000_000;
pub const KB: u64 = 1_000;
pub const GB: u64 = 1_000_000_000;

pub fn wav(secs: f32, f: impl Fn(f32) -> f32) -> Vec<u8> {
    let rate = 22050u32;
    let n = (secs * rate as f32) as u32;
    let mut d = Vec::with_capacity(44 + n as usize * 2);
    d.extend(b"RIFF");
    d.extend((36 + n * 2).to_le_bytes());
    d.extend(b"WAVEfmt ");
    d.extend(16u32.to_le_bytes());
    d.extend(1u16.to_le_bytes());
    d.extend(1u16.to_le_bytes());
    d.extend(rate.to_le_bytes());
    d.extend((rate * 2).to_le_bytes());
    d.extend(2u16.to_le_bytes());
    d.extend(16u16.to_le_bytes());
    d.extend(b"data");
    d.extend((n * 2).to_le_bytes());
    for i in 0..n {
        let t = i as f32 / rate as f32;
        d.extend((((f(t).clamp(-1.0, 1.0)) * 28000.0) as i16).to_le_bytes());
    }
    d
}
fn logdrum(t: f32) -> f32 {
    let beat = t % 0.476;
    let f = 62.0 + 90.0 * (-beat * 22.0).exp();
    (beat * f * std::f32::consts::TAU).sin() * (-beat * 5.5).exp() * 0.9
}
fn shaker(t: f32) -> f32 {
    let beat = t % 0.238;
    let noise = ((t * 91_733.0).sin() * 43_758.545).fract() * 2.0 - 1.0;
    noise * (-beat * 26.0).exp() * 0.5
}
fn pad(t: f32) -> f32 {
    [220.0f32, 277.2, 329.6].iter().map(|f| (t * f * std::f32::consts::TAU).sin()).sum::<f32>() / 4.0 * (1.0 - (t / 2.4 - 0.5).abs() * 1.4).max(0.1)
}

fn text(s: &str) -> Content {
    Content::Buf(s.as_bytes().to_vec())
}

fn zip_of(files: &[(&str, &[u8])]) -> Content {
    let mut c = Compressor::new(Format::Zip);
    for (n, d) in files {
        let _ = c.add_file(n, d, now_ts());
    }
    Content::Buf(c.finish().unwrap_or_default())
}

pub fn populate(fs: &mut dyn Vfs, home: &str, big_sparse: bool) {
    let mut mem = MemFs::default();
    seed(&mut mem, home);
    replay(&mem, fs, home, big_sparse);
}

fn seed(mem: &mut MemFs, home: &str) {
    let now = now_ts();
    let ago = |h: f32| now - (h * 3600.0) as i64;
    let h = |p: &str| join(home, p);
    // build into a MemFs first, then replay into any Vfs (keeps the seeding code storage-agnostic)
    let put = |m: &mut MemFs, p: &str, c: Content, hrs: f32| m.put_file(&h(p), c, ago(hrs));
    for d in ["Desktop", "Documents", "Downloads", "Pictures", "Music", "Movies"] {
        mem.put_dir(&h(d), ago(5.0));
    }
    // Desktop
    put(mem, "Desktop/Welcome to Smart Files.md", text("# Welcome to Smart Files\n\nThis is a **native Rust** file manager. The same code runs on Linux, macOS, Windows and in your browser through WebAssembly.\n\n- Press `Ctrl K` for the command palette\n- Press `Space` for Quick Look\n- Drag files onto folders or the sidebar to move them\n- Try copying `file transfer.txt` — it is a 5 GB demo file with a live transfer experience\n"), 1.0);
    put(mem, "Desktop/Photos.app", text("Smart Photos — every image in your workspace, newest first"), 2.0);
    put(mem, "Desktop/Terminal.app", text("Smart Terminal — runs on the same filesystem layer as the views"), 2.0);
    put(mem, "Desktop/Studio Notes.txt", text("Mix check list\n1. Log drum sits under the kick\n2. Shaker 128 BPM swing 54%\n3. Export stems at 24-bit\n4. Send to Thabo for review\n"), 3.0);
    // Documents
    put(mem, "Documents/Brief — Smart Shell.pdf", Content::Sized(b"%PDF-1.4\n", 2_400_000), 20.0);
    put(mem, "Documents/Invoice 0142.pdf", Content::Sized(b"%PDF-1.4\n", 186_000), 70.0);
    put(mem, "Documents/Roadmap Q4.md", text("# Roadmap Q4\n\n## Shell\n- Depth-based design system\n- Command palette rail\n\n## Files\n- Transfer visualisation\n- Cloud connectors\n- Phone bridge\n"), 40.0);
    put(mem, "Documents/Archive 2025.zip", zip_of(&[("2025/q1.csv", b"month,revenue\njan,10\nfeb,12\n"), ("2025/q2.csv", b"month,revenue\napr,14\nmay,16\n"), ("2025/readme.txt", b"Yearly archive")]), 2000.0);
    mem.put_dir(&h("Documents/Projects/Smart Shell"), ago(30.0));
    put(mem, "Documents/Projects/Smart Shell/spec.md", text("# Smart Shell spec\nTokens, elevation ladder, motion."), 30.0);
    // Downloads
    put(mem, "Downloads/smart-daw-0.4.2.AppImage", Content::Sized(b"\x7fELF", 22_400_000), 26.0);
    put(mem, "Downloads/Inter-4.0.zip", zip_of(&[("Inter-Regular.ttf", b"font"), ("LICENSE.txt", b"OFL")]), 200.0);
    put(mem, "Downloads/dataset-sales.csv", text("id,item,amount\n1,Log drum,24\n2,Shaker,12\n"), 90.0);
    put(mem, "Downloads/old-installer.dmg", Content::Synth(740 * MB), 24.0 * 140.0);
    put(mem, "Downloads/setup-tools.exe", Content::Synth(310 * MB), 24.0 * 95.0);
    put(mem, "Downloads/Sessions Backup.tar.gz", Content::Synth(1_300 * MB), 24.0 * 60.0);
    // Pictures (matches the design prototype)
    let pics: [(&str, &'static [u8], u64, f32, Option<&str>); 8] = [
        ("4K Frosted Glass.jpg", thumb!("frosted-glass"), 1_500_000, 0.4, Some("design")),
        ("Blossom Study.jpg", thumb!("texture-01"), 812_000, 960.0, Some("amapiano")),
        ("Cycling in Soweto.png", thumb!("reference-01"), 977_000, 940.0, Some("archive")),
        ("Green Fold.png", thumb!("design-scan"), 868_000, 31.0, Some("design")),
        ("Nixio Dashboard.png", thumb!("landing-draft"), 1_700_000, 1.2, Some("design")),
        ("Orbit Poster.png", thumb!("poster-01"), 1_200_000, 900.0, Some("amapiano")),
        ("Soweto Moto Ad.png", thumb!("package-b"), 1_100_000, 880.0, Some("amapiano")),
        ("Window Study.png", thumb!("layout-study"), 878_000, 14.0, Some("archive")),
    ];
    for (n, d, sz, hrs, _) in pics {
        put(mem, &format!("Pictures/{n}"), Content::Sized(d, sz), hrs);
    }
    put(mem, "Pictures/Amapiano Pack Vol. 1/Pack Cover.jpg", Content::Sized(thumb!("poster-01"), 2_100_000), 6.0);
    put(mem, "Pictures/Amapiano Pack Vol. 1/Pack Cover (1).jpg", Content::Sized(thumb!("poster-01"), 2_100_000), 5.0);
    put(mem, "Pictures/Amapiano Pack Vol. 1/Amapiano Pack Vol. 1.flp", Content::Sized(b"FLhd", 4_800_000), 17.0);
    put(mem, "Pictures/Amapiano Pack Vol. 1/01 Log Drum 126 BPM.wav", Content::Sized(&[], 18_400_000), 7.0);
    put(mem, "Pictures/Brand — Naledzi/Brand Board.jpg", Content::Sized(thumb!("brand-board"), 3_200_000), 50.0);
    put(mem, "Pictures/Brand — Naledzi/Talk Notes.jpg", Content::Sized(thumb!("talk-notes"), 1_100_000), 52.0);
    put(mem, "Pictures/Brand — Naledzi/Package A.jpg", Content::Sized(thumb!("package-a"), 1_400_000), 55.0);
    put(mem, "Pictures/Log Drum Sessions/Session Notes.txt", text("Log drum 126 vs 128: 126 sits better with the pad."), 20.0);
    put(mem, "Pictures/Log Drum Sessions/UI Exploration.jpg", Content::Sized(thumb!("ui-exploration"), 2_000_000), 21.0);
    put(mem, "Pictures/SMART-DAW Beta/Layout Study.png", Content::Sized(thumb!("layout-study"), 1_400_000), 30.0);
    put(mem, "Pictures/SMART-DAW Beta/beta-notes.md", text("# Beta notes\n- Piano roll snapping\n- Log drum bus"), 31.0);
    // Music
    for (n, c, sz, hrs) in [
        ("01 Log Drum 126 BPM.wav", wav(2.8, logdrum), 18_400_000, 7.0),
        ("02 Shaker Groove.wav", wav(2.4, shaker), 6_200_000, 7.2),
        ("Warm Pad C minor.wav", wav(2.4, pad), 9_800_000, 30.0),
    ] {
        put(mem, &format!("Music/{n}"), Content::Buf(c.clone()), hrs);
        let _ = sz;
    }
    // Movies
    put(mem, "Movies/Studio Tour.mp4", Content::Synth(1_240 * MB), 100.0);
    put(mem, "Movies/Teaser 4K.mov", Content::Synth(2_100 * MB), 300.0);
    // junk for cleanup
    put(mem, ".cache/thumbnails/large/cache.db", Content::Synth(640 * MB), 400.0);
    put(mem, ".cache/pip/wheels.bin", Content::Synth(280 * MB), 600.0);
    put(mem, ".cache/browser/Cache_Data", Content::Synth(420 * MB), 12.0);
    put(mem, ".Trash/Untitled draft.txt", text("scrap"), 3.0);
    put(mem, ".Trash/old-photo.png", Content::Sized(thumb!("texture-01"), 3_100_000), 80.0);
    // the star of the show
    let big = h("Desktop/file transfer.txt");
    mem.put_file(&big, Content::Synth(5 * GB), ago(0.2));
}

fn replay(mem: &MemFs, fs: &mut dyn Vfs, home: &str, sparse: bool) {
    fn walk(mem: &MemFs, fs: &mut dyn Vfs, dir: &str, sparse: bool) {
        for e in mem.read_dir(dir).unwrap_or_default() {
            if e.is_dir {
                let _ = fs.mkdir(&e.path);
                walk(mem, fs, &e.path, sparse);
            } else if e.size > 64 << 20 {
                if sparse {
                    let _ = fs.create_sparse(&e.path, e.size);
                } else {
                    let _ = fs.write_chunk(&e.path, 0, Chunk::Zeros(e.size));
                }
            } else if let Ok(c) = mem.read_at(&e.path, 0, e.size) {
                let _ = fs.write_chunk(&e.path, 0, c);
            }
        }
    }
    let _ = fs.mkdir(home);
    walk(mem, fs, home, sparse);
}

fn populate_mem(m: &mut MemFs, home: &str) {
    seed(m, home);
}

fn media_root(m: &mut MemFs, root: &str, files: &[(&str, Content, f32)]) {
    let now = now_ts();
    m.put_dir(root, now);
    for (p, c, hrs) in files {
        m.put_file(&join(root, p), c.clone(), now - (*hrs * 3600.0) as i64);
    }
}

pub fn build() -> Engine {
    let home = "/home/naledzi";
    let mut e = Engine::empty(home, false);
    let mut root = MemFs::new("/");
    populate_mem(&mut root, home);
    root.put_dir("/Applications", now_ts());
    e.add_mount(Mount { id: "drive".into(), label: "Smart Drive".into(), root: "/".into(), kind: MountKind::Internal, total: 512 * GB, free: 194 * GB, mounted: true, detail: "NVMe · APFS".into(), device: None }, Box::new(root));

    let mut usb = MemFs::default();
    media_root(&mut usb, "/media/stick", &[
        ("Backups/studio-2026-10.zip", Content::Synth(3_400 * MB), 70.0),
        ("Sample Packs/Kick Library.zip", zip_of(&[("kick.wav", b"RIFF")]), 400.0),
        ("Readme.txt", text("SMART STICK — 64 GB"), 900.0),
    ]);
    e.add_mount(Mount { id: "usb".into(), label: "SMART STICK".into(), root: "/media/stick".into(), kind: MountKind::Removable, total: 64 * GB, free: 41 * GB, mounted: true, detail: "exFAT · USB 3.2".into(), device: None }, Box::new(usb));

    let mut phone = MemFs::default();
    media_root(&mut phone, "/phone/pixel9", &[
        ("DCIM/Camera/IMG_2041.jpg", Content::Sized(thumb!("reference-01"), 4_200_000), 5.0),
        ("DCIM/Camera/IMG_2042.jpg", Content::Sized(thumb!("texture-01"), 3_900_000), 5.0),
        ("DCIM/Camera/VID_0007.mp4", Content::Synth(380 * MB), 9.0),
        ("Music/Voice memo 12.m4a", Content::Sized(&[], 2_200_000), 30.0),
        ("Download/boarding-pass.pdf", Content::Sized(b"%PDF-1.4", 210_000), 120.0),
    ]);
    e.add_mount(Mount { id: "phone".into(), label: "Pixel 9 Pro".into(), root: "/phone/pixel9".into(), kind: MountKind::Phone, total: 256 * GB, free: 88 * GB, mounted: true, detail: "Android · USB MTP".into(), device: None }, Box::new(phone));

    let mut cloud = MemFs::default();
    media_root(&mut cloud, "/cloud/smart", &[
        ("Shared/Brand Kit.zip", zip_of(&[("logo.svg", b"<svg/>")]), 200.0),
        ("Shared/Pitch Deck.pdf", Content::Sized(b"%PDF-1.4", 8_400_000), 90.0),
        ("Backups/phone-2026-10.tar.gz", Content::Synth(1_900 * MB), 700.0),
        ("Notes.md", text("# Cloud notes"), 55.0),
    ]);
    e.add_mount(Mount { id: "cloud".into(), label: "Smart Cloud".into(), root: "/cloud/smart".into(), kind: MountKind::Cloud, total: 2_000 * GB, free: 1_420 * GB, mounted: true, detail: "naledzi@smart.cloud · synced".into(), device: None }, Box::new(cloud));

    let mut nas = MemFs::default();
    media_root(&mut nas, "/net/studio", &[("Masters/Album Master.wav", Content::Synth(820 * MB), 800.0), ("Projects/readme.txt", text("Studio NAS"), 2000.0)]);
    e.add_mount(Mount { id: "nas".into(), label: "Studio NAS".into(), root: "/net/studio".into(), kind: MountKind::Network, total: 8_000 * GB, free: 3_100 * GB, mounted: true, detail: "smb://studio.local".into(), device: None }, Box::new(nas));

    for (p, t) in [
        ("Pictures/4K Frosted Glass.jpg", "design"), ("Pictures/Nixio Dashboard.png", "design"), ("Pictures/Green Fold.png", "design"), ("Pictures/Brand — Naledzi", "design"),
        ("Pictures/Blossom Study.jpg", "amapiano"), ("Pictures/Orbit Poster.png", "amapiano"), ("Pictures/Soweto Moto Ad.png", "amapiano"), ("Pictures/Amapiano Pack Vol. 1", "amapiano"), ("Pictures/Log Drum Sessions", "amapiano"),
        ("Pictures/Cycling in Soweto.png", "archive"), ("Pictures/Window Study.png", "archive"), ("Documents/Archive 2025.zip", "archive"), ("Pictures/SMART-DAW Beta", "design"),
    ] {
        e.tags.insert(format!("{home}/{p}"), t.into());
    }
    e.trash_dir = Some(format!("{home}/.Trash"));
    e.reindex();
    let roots = vec![home.to_string(), "/media/stick".into(), "/phone/pixel9".into(), "/cloud/smart".into(), "/net/studio".into()];
    e.index.reset(roots);
    e
}

/// Native only: create a small real demo workspace (with a sparse 5 GB `file transfer.txt`).
#[cfg(not(target_arch = "wasm32"))]
pub fn create_native_workspace(e: &mut Engine) -> R<String> {
    let base = join(&e.home, "Smart Files Demo");
    let mut fs = StdFs;
    let _ = fs.mkdir(&base);
    let mut sub = Subfs { fs: &mut fs };
    populate(&mut sub, &base, true);
    e.index.queue.push(base.clone());
    e.index.ready = false;
    Ok(base)
}
#[cfg(not(target_arch = "wasm32"))]
struct Subfs<'a> {
    fs: &'a mut StdFs,
}
#[cfg(not(target_arch = "wasm32"))]
impl Vfs for Subfs<'_> {
    fn read_dir(&self, p: &str) -> R<Vec<Entry>> { self.fs.read_dir(p) }
    fn stat(&self, p: &str) -> R<Entry> { self.fs.stat(p) }
    fn mkdir(&mut self, p: &str) -> R<()> { self.fs.mkdir(p).or(Ok(())) }
    fn remove(&mut self, p: &str) -> R<()> { self.fs.remove(p) }
    fn rename(&mut self, a: &str, b: &str) -> R<()> { self.fs.rename(a, b) }
    fn read_at(&self, p: &str, o: u64, l: u64) -> R<Chunk> { self.fs.read_at(p, o, l) }
    fn write_chunk(&mut self, p: &str, o: u64, c: Chunk) -> R<()> { self.fs.write_chunk(p, o, c) }
    fn create_sparse(&mut self, p: &str, s: u64) -> R<()> { self.fs.create_sparse(p, s) }
}
