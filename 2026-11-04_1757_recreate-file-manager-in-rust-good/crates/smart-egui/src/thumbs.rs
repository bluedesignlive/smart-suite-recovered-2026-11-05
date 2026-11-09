//! Thumbnail / preview texture cache. Decoding runs on worker threads natively, inline (budgeted) on wasm.
use egui::{ColorImage, Context, TextureHandle, TextureOptions};
use smart_core::Store;
use std::collections::{HashMap, HashSet};

pub fn decode(bytes: &[u8], max: u32) -> Option<ColorImage> {
    let img = image::load_from_memory(bytes).ok()?;
    let img = if img.width() > max || img.height() > max { img.thumbnail(max, max) } else { img };
    let rgba = img.to_rgba8();
    Some(ColorImage::from_rgba_unmultiplied([rgba.width() as usize, rgba.height() as usize], rgba.as_raw()))
}

#[derive(Default)]
pub struct Thumbs {
    tex: HashMap<String, Option<TextureHandle>>,
    inflight: HashSet<String>,
    text: HashMap<String, String>,
    sizes: HashMap<String, (u64, u64)>,
    budget: u32,
    #[cfg(not(target_arch = "wasm32"))]
    chan: Option<(std::sync::mpsc::Sender<(String, Option<ColorImage>)>, std::sync::mpsc::Receiver<(String, Option<ColorImage>)>)>,
    #[cfg(not(target_arch = "wasm32"))]
    running: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl Thumbs {
    pub fn begin_frame(&mut self, ctx: &Context) {
        self.budget = 2;
        #[cfg(not(target_arch = "wasm32"))]
        if let Some((_, rx)) = &self.chan {
            while let Ok((k, img)) = rx.try_recv() {
                self.inflight.remove(&k);
                let t = img.map(|i| ctx.load_texture(&k, i, TextureOptions::LINEAR));
                self.tex.insert(k, t);
                ctx.request_repaint();
            }
        }
        let _ = ctx;
    }

    pub fn get(&mut self, ctx: &Context, store: &Store, path: &str, max: u32) -> Option<TextureHandle> {
        let key = format!("{path}@{max}");
        if let Some(t) = self.tex.get(&key) {
            return t.clone();
        }
        if self.inflight.contains(&key) {
            return None;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if store.eng.is_local(path) {
            if self.chan.is_none() {
                self.chan = Some(std::sync::mpsc::channel());
            }
            if self.running.load(std::sync::atomic::Ordering::Relaxed) >= 3 {
                return None;
            }
            let real = store.eng.mounts.iter().find(|_| true).map(|_| smart_core::vfs::to_real(path));
            let tx = self.chan.as_ref().unwrap().0.clone();
            let running = self.running.clone();
            running.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.inflight.insert(key.clone());
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                let img = real.and_then(|p| std::fs::metadata(&p).ok().filter(|m| m.len() < 60_000_000).and_then(|_| std::fs::read(&p).ok())).and_then(|b| decode(&b, max));
                let _ = tx.send((key, img));
                running.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                ctx.request_repaint();
            });
            return None;
        }
        if self.budget == 0 {
            ctx.request_repaint();
            return None;
        }
        self.budget -= 1;
        let t = store.eng.read_all(path, 40_000_000).ok().and_then(|b| decode(&b, max)).map(|i| ctx.load_texture(&key, i, TextureOptions::LINEAR));
        self.tex.insert(key, t.clone());
        t
    }

    pub fn excerpt(&mut self, store: &Store, path: &str) -> String {
        if let Some(t) = self.text.get(path) {
            return t.clone();
        }
        let s = store.eng.read_head(path, 360).map(|b| String::from_utf8_lossy(&b).replace("**", "").replace('`', "").lines().filter(|l| !l.trim().is_empty()).take(5).map(|l| l.trim_start_matches('#').trim().to_string()).collect::<Vec<_>>().join("\n")).unwrap_or_default();
        let s = if s.contains('\0') { String::new() } else { s };
        self.text.insert(path.to_string(), s.clone());
        s
    }
    /// Folder size from the search index, cached per engine revision.
    pub fn dir_size(&mut self, store: &Store, path: &str) -> u64 {
        let rev = store.eng.revision + store.eng.index.entries.len() as u64 / 500;
        if let Some((r, s)) = self.sizes.get(path) {
            if *r == rev {
                return *s;
            }
        }
        let s = store.eng.dir_size(path);
        self.sizes.insert(path.to_string(), (rev, s));
        s
    }
    pub fn invalidate(&mut self) {
        self.text.clear();
    }
}
