//! Storage workspace: opens as a tab inside Files. Overview, Cleanup (context-aware junk), Map (treemap),
//! Graph (an Obsidian-style force-directed view of the folder tree) and Biggest. Scanning runs off the UI thread on
//! desktop and is sliced across frames on the web.
use crate::app::*;
use crate::appkit::*;
use crate::chrome::*;
use crate::content::kind_color;
use crate::icons::Ic;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Pos2, Rect, Sense, Ui, Vec2};
use smart_core::analyze::*;
use smart_core::vfs::Vfs;
use smart_core::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub const VIEWS: [(&str, &str); 5] = [("pie-chart-2", "Overview"), ("trash-bin-minimalistic", "Cleanup"), ("widget-4", "Map"), ("graph-new", "Graph"), ("sort-vertical", "Biggest")];

/// Drive gauge colours: accent, then amber from 80 %, then red from 90 %.
pub fn usage_colors(th: &Theme, used: f32) -> (Color32, Color32) {
    if used >= 0.90 {
        (mix(th.danger, Color32::WHITE, 0.22), th.danger)
    } else if used >= 0.80 {
        (mix(th.warn, Color32::WHITE, 0.22), th.warn)
    } else {
        th.accent_grad()
    }
}

#[derive(Clone)]
pub struct GNode {
    pub idx: Option<usize>,
    pub label: String,
    pub path: String,
    pub size: u64,
    pub pos: Vec2,
    pub vel: Vec2,
    pub r: f32,
    pub depth: u8,
    pub pinned: bool,
    pub junk: bool,
    pub is_dir: bool,
    pub kind: Kind,
}

pub struct Graph {
    pub focus: usize,
    pub sig: (usize, usize),
    pub nodes: Vec<GNode>,
    pub edges: Vec<(usize, usize)>,
    pub zoom: f32,
    pub pan: Vec2,
    pub hover: Option<usize>,
    pub drag: Option<usize>,
    pub heat: f32,
}

pub struct Snap {
    pub at: f64,
    pub total: u64,
    pub buckets: [u64; 6],
    pub files: u64,
    pub dirs: u64,
    pub errors: u64,
    pub finished: bool,
    pub progress: f32,
    pub cands: Vec<Junk>,
    pub kinds: Vec<(JunkKind, u64, usize)>,
    pub safe: u64,
    pub all: u64,
    pub big_files: Vec<FileInfo>,
    pub big_dirs: Vec<(String, String, u64)>,
}

pub struct StorageState {
    pub scan: Arc<Mutex<Scan>>,
    pub vfs: Option<Arc<dyn Vfs>>,
    pub scope_home: bool,
    pub view: usize,
    pub sel: HashSet<String>,
    pub filter: Option<JunkKind>,
    pub map_focus: usize,
    pub graph: Graph,
    pub snap: Option<Snap>,
    pub name: String,
    pub fs: String,
    pub drive_total: u64,
    pub drive_free: u64,
    pub rescan_at: Option<f64>,
    pub born: f64,
}

impl StorageState {
    fn snapshot(&mut self, now: f64) {
        let fresh = self.snap.as_ref().map(|s| now - s.at < 0.25 && !s.finished).unwrap_or(false);
        let done = self.snap.as_ref().map(|s| s.finished).unwrap_or(false);
        if fresh || done {
            return;
        }
        let s = self.scan.lock().unwrap();
        let cands: Vec<Junk> = s.candidates().into_iter().take(400).cloned().collect();
        let big_dirs = s.biggest_dirs(14).into_iter().map(|i| (s.nodes[i].path.clone(), s.nodes[i].name.clone(), s.nodes[i].size)).collect();
        self.snap = Some(Snap { at: now, total: s.total(), buckets: s.buckets, files: s.files_seen, dirs: s.dirs_seen, errors: s.errors, finished: s.finished, progress: s.progress(), cands, kinds: s.by_kind(), safe: s.reclaimable(true), all: s.reclaimable(false), big_files: s.biggest_files(30).into_iter().cloned().collect(), big_dirs });
    }
}

fn hue_for(name: &str, th: &Theme) -> Color32 {
    let pal = [th.accent, hex("#14b8a6"), hex("#8b5cf6"), hex("#f59e0b"), hex("#ec4899"), hex("#22c55e"), hex("#06b6d4")];
    let h = name.bytes().fold(7u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
    pal[h as usize % pal.len()]
}

fn arc_mesh(p: &egui::Painter, ctr: Pos2, r_out: f32, r_in: f32, a0: f32, a1: f32, col: Color32) {
    if a1 - a0 < 0.002 {
        return;
    }
    let n = (((a1 - a0) * r_out / 5.0).ceil() as usize).clamp(2, 160);
    let mut m = egui::epaint::Mesh::default();
    for i in 0..=n {
        let a = a0 + (a1 - a0) * i as f32 / n as f32;
        let d = vec2(a.cos(), a.sin());
        m.vertices.push(egui::epaint::Vertex { pos: ctr + d * r_out, uv: egui::epaint::WHITE_UV, color: col });
        m.vertices.push(egui::epaint::Vertex { pos: ctr + d * r_in, uv: egui::epaint::WHITE_UV, color: col });
        if i > 0 {
            let b = (i * 2) as u32;
            m.indices.extend([b - 2, b - 1, b, b - 1, b + 1, b]);
        }
    }
    p.add(egui::Shape::mesh(m));
}

impl SmartApp {
    // ───────────── lifecycle ─────────────
    pub fn storage_start(&mut self, home_scope: bool) {
        let dev = self.current_device();
        let (fs, name, total, free) = match &dev {
            Some(d) => (d.loc.as_ref().map(|l| l.fs.clone()).unwrap_or_else(|| self.be.home().fs), d.name.clone(), d.total, d.free),
            None => (self.be.home().fs, "Computer".to_string(), 0, 0),
        };
        let home = self.be.home();
        let root = if home_scope || home.fs != fs { home.path.clone() } else { "/".to_string() };
        let vfs = self.be.registry.get(&fs);
        let mut sc = Scan::new(&fs, &root, now_secs());
        if !IS_WEB && fs == "file" {
            sc.skip_pseudo();
        }
        let scan = Arc::new(Mutex::new(sc));
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(v) = vfs.clone() {
            let s2 = scan.clone();
            std::thread::Builder::new()
                .name("storage-scan".into())
                .spawn(move || loop {
                    let more = {
                        let mut g = match s2.lock() {
                            Ok(g) => g,
                            Err(p) => p.into_inner(),
                        };
                        g.step(v.as_ref(), 6)
                    };
                    if !more || Arc::strong_count(&s2) == 1 {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                })
                .ok();
        }
        let old_view = self.storage.as_ref().map(|s| s.view).unwrap_or(0);
        self.storage = Some(StorageState {
            scan,
            vfs,
            scope_home: root == home.path,
            view: old_view,
            sel: HashSet::new(),
            filter: None,
            map_focus: 0,
            graph: Graph { focus: usize::MAX, sig: (0, 0), nodes: vec![], edges: vec![], zoom: 1.0, pan: Vec2::ZERO, hover: None, drag: None, heat: 1.0 },
            snap: None,
            name,
            fs,
            drive_total: total,
            drive_free: free,
            rescan_at: None,
            born: self.time,
        });
    }

    pub fn storage_cmd(&mut self, c: &str) {
        let (name, arg) = c.split_once(':').unwrap_or((c, ""));
        match name {
            "open" => {
                if self.storage.is_none() {
                    self.storage_start(false);
                }
                if let Some(i) = self.tabs.iter().position(|t| t.route == Route::Storage).or_else(|| (self.route == Route::Storage).then_some(self.tab_i)) {
                    self.switch_tab(i);
                } else {
                    self.new_tab(Some(Route::Storage));
                }
            }
            "rescan" => {
                let home = self.storage.as_ref().map(|s| s.scope_home).unwrap_or(false);
                self.storage_start(home);
            }
            "scope" => {
                let home = arg == "home";
                self.storage_start(home);
            }
            "view" => {
                if let (Some(s), Ok(v)) = (self.storage.as_mut(), arg.parse::<usize>()) {
                    s.view = v.min(VIEWS.len() - 1);
                }
            }
            "select_safe" => {
                if let Some(s) = self.storage.as_mut() {
                    if let Some(sn) = &s.snap {
                        s.sel = sn.cands.iter().filter(|j| j.safety == Safety::Safe).map(|j| j.path.clone()).collect();
                    }
                    s.view = 1;
                }
            }
            "clear_sel" => {
                if let Some(s) = self.storage.as_mut() {
                    s.sel.clear();
                }
            }
            "ask_clean" => {
                let Some(s) = self.storage.as_ref() else { return };
                let Some(sn) = &s.snap else { return };
                let chosen: Vec<&Junk> = sn.cands.iter().filter(|j| s.sel.contains(&j.path)).collect();
                if chosen.is_empty() {
                    return;
                }
                let bytes: u64 = chosen.iter().map(|j| j.size()).sum();
                let risky = chosen.iter().any(|j| j.safety != Safety::Safe);
                self.modal = Modal::Confirm(Confirm {
                    title: format!("Move {} item{} to Trash?", chosen.len(), if chosen.len() == 1 { "" } else { "s" }),
                    body: format!("Frees {}.{}", format_size(bytes), if risky { " Some items need your judgement. Review them first." } else { " Everything selected is regenerated automatically." }),
                    ok: "Move to Trash".into(),
                    danger: true,
                    act: Box::new(Act::App("storage.clean".into())),
                });
            }
            "clean" => {
                let Some(s) = self.storage.as_mut() else { return };
                let fs = s.fs.clone();
                let srcs: Vec<Location> = s.sel.drain().map(|p| Location::new(fs.clone(), p)).collect();
                s.rescan_at = Some(self.time + 2.0);
                if let Some(sn) = s.snap.as_mut() {
                    sn.finished = false;
                }
                if !srcs.is_empty() {
                    self.submit_job(JobKind::Trash { srcs });
                }
            }
            _ => {}
        }
    }

    fn storage_pump(&mut self, ui: &Ui, st: &mut StorageState) {
        #[cfg(target_arch = "wasm32")]
        if let Some(v) = st.vfs.clone() {
            let mut s = st.scan.lock().unwrap();
            if !s.finished {
                s.step(v.as_ref(), 45);
                ui.ctx().request_repaint();
            }
        }
        let running = st.snap.as_ref().map(|s| !s.finished).unwrap_or(true);
        if running {
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(120));
        }
        st.snapshot(self.time);
        if let Some(t) = st.rescan_at {
            if self.time >= t {
                st.rescan_at = None;
                self.acts.push(Act::App("storage.rescan".into()));
            }
        }
    }

    // ───────────── contextual app bar ─────────────
    pub fn storage_appbar(&mut self, ui: &mut Ui, left: f32, right: f32, cy: f32) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let Some(st) = self.storage.as_ref() else { return };
        let (view, nsel) = (st.view, st.sel.len());
        let sel_bytes: u64 = st.snap.as_ref().map(|sn| sn.cands.iter().filter(|j| st.sel.contains(&j.path)).map(|j| j.size()).sum()).unwrap_or(0);
        let scanning = st.snap.as_ref().map(|s| !s.finished).unwrap_or(true);
        let prog = st.snap.as_ref().map(|s| s.progress).unwrap_or(0.0);
        let home = st.scope_home;
        let mut rx = right;
        let tight = (rx - left) < 620.0;
        if nsel > 0 {
            if tight {
                let r = Rect::from_min_size(pos2(rx - 34.0, cy - 17.0), vec2(34.0, 34.0));
                if self.icon_button(ui, r, "trash-bin-minimalistic", "st-clean", Btn::Danger, true).on_hover_text(&format!("Trash {nsel} · frees {}", format_size(sel_bytes))).clicked() {
                    self.push_act(Act::App("storage.ask_clean".into()));
                }
                rx = r.left() - 8.0;
            } else {
                let label = format!("Trash {} \u{b7} {}", nsel, format_size(sel_bytes));
                let w = self.tw(&p, &label, &f_semi(T_BODY)) + 56.0;
                let r = Rect::from_min_size(pos2(rx - w, cy - 17.0), vec2(w, 34.0));
                if self.label_button(ui, r, Some("trash-bin-minimalistic"), &label, "st-clean", Btn::Danger).clicked() {
                    self.push_act(Act::App("storage.ask_clean".into()));
                }
                rx = r.left() - 8.0;
            }
        } else {
            let (rr, nrx) = self.adaptive_button(ui, rx, cy, "refresh", "Rescan", "st-rescan", Btn::Raised, rx - left > 560.0);
            if rr.clicked() {
                self.push_act(Act::App("storage.rescan".into()));
            }
            rx = nrx;
        }
        // scope switch
        let scope = Rect::from_min_size(pos2(rx - 108.0, cy - 17.0), vec2(108.0, 34.0));
        if rx - left > 640.0 {
            if let Some(i) = self.segment(ui, scope, &[("", "Home"), ("", "Drive")], false, if home { 0 } else { 1 }, "st-scope") {
                self.push_act(Act::App(format!("storage.scope:{}", if i == 0 { "home" } else { "drive" })));
            }
            rx = scope.left() - 8.0;
        }
        // view switcher
        let wide = rx - left > 700.0;
        let sw = if wide { 5.0 * 82.0 + 6.0 } else { 5.0 * 38.0 + 6.0 };
        let seg = Rect::from_min_size(pos2((rx - sw).max(left + 120.0), cy - 17.0), vec2(sw, 34.0));
        let opts: Vec<(&'static str, &str)> = VIEWS.iter().map(|(i, l)| if wide { ("", *l) } else { (*i, *l) }).collect();
        if let Some(i) = self.segment(ui, seg, &opts, !wide, view, "st-views") {
            self.push_act(Act::App(format!("storage.view:{i}")));
        }
        // title / progress
        let name = self.storage.as_ref().map(|s| s.name.clone()).unwrap_or_default();
        let x = left;
        let title_w = (rx - x - 210.0).clamp(60.0, 420.0);
        self.ico(&p, "pie-chart-2", Ic::Duo, pos2(x + 10.0, cy), 18.0, th.accent_ink);
        p.text(pos2(x + 28.0, cy - if scanning { 6.0 } else { 0.0 }), Align2::LEFT_CENTER, self.fit(&p, &format!("Storage \u{b7} {name}"), &f_semi(T_BODY), title_w), f_semi(T_BODY), th.ink1);
        if scanning {
            let bar = Rect::from_min_size(pos2(x + 28.0, cy + 9.0), vec2(120.0, 4.0));
            well(&p, &th, bar, 2.0);
            let f = (prog.max(0.04)).min(1.0);
            let (a, b) = th.accent_grad();
            grad_fill(&p, Rect::from_min_size(bar.min, vec2(bar.width() * f, 4.0)), 2.0, a, b);
        }
    }

    // ───────────── content ─────────────
    pub fn draw_storage(&mut self, ui: &mut Ui, rect: Rect) {
        if self.storage.is_none() {
            self.storage_start(false);
        }
        let mut st = self.storage.take().unwrap();
        self.storage_pump(ui, &mut st);
        let th = self.th.clone();
        let p = ui.painter().with_clip_rect(rect);
        let Some(sn) = st.snap.take() else {
            self.storage = Some(st);
            return;
        };
        let used = if st.drive_total > 0 { 1.0 - st.drive_free as f32 / st.drive_total as f32 } else { 0.0 };
        // header band
        let head = Rect::from_min_size(rect.min + vec2(24.0, 6.0), vec2(rect.width() - 48.0, 74.0));
        self.storage_header(ui, &p, head, &st, &sn, used);
        let body = Rect::from_min_max(pos2(rect.left() + 24.0, head.bottom() + 12.0), pos2(rect.right() - 24.0, rect.bottom() - 8.0));
        match st.view {
            0 => self.storage_overview(ui, &p, body, &mut st, &sn, used),
            1 => self.storage_cleanup(ui, &p, body, &mut st, &sn),
            2 => self.storage_map(ui, &p, body, &mut st),
            3 => self.storage_graph(ui, &p, body, &mut st),
            _ => self.storage_biggest(ui, &p, body, &mut st, &sn),
        }
        st.snap = Some(sn);
        self.storage = Some(st);
    }

    fn storage_header(&mut self, ui: &Ui, p: &egui::Painter, r: Rect, st: &StorageState, sn: &Snap, used: f32) {
        let th = self.th.clone();
        elevate(p, &th, r, 16.0, 1);
        grad_fill(p, r, 16.0, th.card_top, th.card_bottom);
        rim(p, r, 16.0, 1.2, th.rim, true, 30.0);
        let (a, b) = usage_colors(&th, used);
        let warn = used >= 0.90;
        let ico = Rect::from_min_size(r.min + vec2(16.0, 16.0), vec2(42.0, 42.0));
        grad_fill(p, ico, 21.0, alpha(a, 0.28), alpha(b, 0.14));
        self.ico(p, if warn { "danger-triangle" } else { "ssd-square" }, Ic::Filled, ico.center(), 22.0, b);
        let free = st.drive_free;
        let title = if st.drive_total > 0 { format!("{} used of {}", format_size(st.drive_total - st.drive_free), format_size(st.drive_total)) } else { format!("{} scanned", format_size(sn.total)) };
        p.text(pos2(ico.right() + 14.0, r.top() + 24.0), Align2::LEFT_CENTER, title, f_bold(T_TITLE), th.ink1);
        let sub = if st.drive_total > 0 {
            if warn {
                format!("Only {} free ({:.0}% full). Clean up soon so the system keeps room to work.", format_size(free), used * 100.0)
            } else {
                format!("{} free \u{b7} {:.0}% full", format_size(free), used * 100.0)
            }
        } else {
            format!("{} files \u{b7} {} folders", sn.files, sn.dirs)
        };
        p.text(pos2(ico.right() + 14.0, r.top() + 44.0), Align2::LEFT_CENTER, self.fit(p, &sub, &f_medium(T_META), r.width() - 340.0), f_medium(T_META), if warn { th.danger } else { th.ink3 });
        // gauge
        let bar = Rect::from_min_max(pos2(ico.right() + 14.0, r.bottom() - 18.0), pos2(r.right() - 300.0_f32.min(r.width() * 0.4), r.bottom() - 11.0));
        if bar.width() > 40.0 {
            well(p, &th, bar, 3.5);
            let grow = spring_to(ui.ctx(), "st-gauge", used, 160.0, 22.0).clamp(0.0, 1.0);
            let fill = Rect::from_min_size(bar.min, vec2((bar.width() * grow).max(7.0), 7.0));
            grad_fill(p, fill, 3.5, a, b);
            rim(p, fill, 3.5, 1.0, Color32::from_white_alpha(90), true, 6.0);
        }
        // reclaimable chip
        if sn.all > 0 {
            let t = format!("{} can be reclaimed", format_size(sn.safe.max(sn.all / 3)));
            let w = self.tw(p, &t, &f_semi(T_META)) + 40.0;
            let chip = Rect::from_min_size(pos2(r.right() - w - 16.0, r.top() + 14.0), vec2(w, 30.0));
            let resp = self.hit(ui, chip, "st-chip", Sense::click());
            grad_fill(p, chip, 15.0, alpha(th.good, if resp.hovered() { 0.28 } else { 0.18 }), alpha(th.good, 0.10));
            self.ico(p, "shield-check", Ic::Duo, pos2(chip.left() + 18.0, chip.center().y), 15.0, th.good);
            p.text(pos2(chip.left() + 32.0, chip.center().y), Align2::LEFT_CENTER, t, f_semi(T_META), th.good);
            if resp.clicked() {
                self.push_act(Act::App("storage.view:1".into()));
            }
        }
        if !sn.finished {
            p.text(pos2(r.right() - 16.0, r.bottom() - 28.0), Align2::RIGHT_CENTER, format!("Scanning\u{2026} {} files", sn.files), f_medium(T_META), th.ink3);
        }
    }

    // ───────────── overview ─────────────
    fn storage_overview(&mut self, ui: &Ui, p: &egui::Painter, body: Rect, st: &mut StorageState, sn: &Snap, used: f32) {
        let th = self.th.clone();
        let two = body.width() > 760.0;
        let left_w = if two { 300.0 } else { body.width() };
        let mut y = body.top();
        // warning banner
        if used >= 0.90 {
            let r = Rect::from_min_size(pos2(body.left(), y), vec2(body.width(), 58.0));
            grad_fill(p, r, 16.0, alpha(th.danger, 0.20), alpha(th.danger, 0.10));
            rim(p, r, 16.0, 1.0, alpha(th.danger, 0.4), true, 20.0);
            self.ico(p, "danger-triangle", Ic::Filled, pos2(r.left() + 28.0, r.center().y), 22.0, th.danger);
            let after = if st.drive_total > 0 { ((st.drive_free + sn.safe) as f32 / st.drive_total as f32 * 100.0).min(100.0) } else { 0.0 };
            p.text(pos2(r.left() + 52.0, r.center().y - 9.0), Align2::LEFT_CENTER, format!("Your drive is {:.0}% full", used * 100.0), f_bold(T_BODY), th.ink1);
            let msg = format!("{} of build output and caches can be removed safely, taking free space to {:.0}%.", format_size(sn.safe), after);
            p.text(pos2(r.left() + 52.0, r.center().y + 10.0), Align2::LEFT_CENTER, self.fit(p, &msg, &f_medium(T_META), r.width() - 250.0), f_medium(T_META), th.ink2);
            if sn.safe > 0 {
                let b = Rect::from_min_size(pos2(r.right() - 168.0, r.center().y - 17.0), vec2(152.0, 34.0));
                if self.label_button(ui, b, Some("shield-check"), "Review safe items", "st-safe", Btn::Accent).clicked() {
                    self.push_act(Act::App("storage.select_safe".into()));
                }
            }
            y = r.bottom() + 14.0;
        }
        // donut
        let donut_box = Rect::from_min_size(pos2(body.left(), y), vec2(left_w, 264.0));
        elevate(p, &th, donut_box, 18.0, 1);
        grad_fill(p, donut_box, 18.0, th.card_top, th.card_bottom);
        rim(p, donut_box, 18.0, 1.2, th.rim, true, 30.0);
        let ctr = donut_box.center() - vec2(0.0, 6.0);
        let (ro, ri) = (92.0, 66.0);
        let total_b: u64 = sn.buckets.iter().sum::<u64>().max(1);
        let sweep = spring_to(ui.ctx(), ("st-donut", st.born as i64), 1.0, 90.0, 16.0).clamp(0.0, 1.0);
        let cols = [th.accent, hex("#8b5cf6"), hex("#14b8a6"), hex("#f59e0b"), hex("#ec4899"), th.ink4];
        let mut a0 = -std::f32::consts::FRAC_PI_2;
        arc_mesh(p, ctr, ro, ri, 0.0, std::f32::consts::TAU, alpha(th.ink4, 0.12));
        for (i, v) in sn.buckets.iter().enumerate() {
            let sweep_a = *v as f32 / total_b as f32 * std::f32::consts::TAU * sweep;
            arc_mesh(p, ctr, ro, ri, a0 + 0.012, a0 + sweep_a - 0.012, cols[i]);
            a0 += sweep_a;
        }
        p.text(ctr - vec2(0.0, 8.0), Align2::CENTER_CENTER, format_size(sn.total), f_bold(22.0), th.ink1);
        p.text(ctr + vec2(0.0, 14.0), Align2::CENTER_CENTER, if sn.finished { "scanned".to_string() } else { format!("{:.0}%", sn.progress * 100.0) }, f_medium(T_META), th.ink3);
        p.text(pos2(donut_box.center().x, donut_box.bottom() - 28.0), Align2::CENTER_CENTER, format!("{} files in {} folders", sn.files, sn.dirs), f_medium(T_META), th.ink3);
        // legend + wins
        let right = if two { Rect::from_min_max(pos2(donut_box.right() + 16.0, y), pos2(body.right(), body.bottom())) } else { Rect::from_min_max(pos2(body.left(), donut_box.bottom() + 14.0), body.max) };
        let mut cy = right.top();
        let leg = Rect::from_min_size(pos2(right.left(), cy), vec2(right.width(), 12.0 + 6.0 * 30.0 + 8.0));
        elevate(p, &th, leg, 18.0, 1);
        grad_fill(p, leg, 18.0, th.card_top, th.card_bottom);
        rim(p, leg, 18.0, 1.2, th.rim, true, 30.0);
        for (i, bk) in Bucket::ALL.iter().enumerate() {
            let row = Rect::from_min_size(pos2(leg.left() + 16.0, leg.top() + 12.0 + i as f32 * 30.0), vec2(leg.width() - 32.0, 28.0));
            p.circle_filled(pos2(row.left() + 6.0, row.center().y), 5.0, cols[i]);
            p.text(pos2(row.left() + 20.0, row.center().y), Align2::LEFT_CENTER, bk.label(), f_medium(T_BODY), th.ink2);
            let sz = format_size(sn.buckets[i]);
            p.text(pos2(row.right(), row.center().y), Align2::RIGHT_CENTER, sz, f_semi(T_BODY), th.ink1);
            let bar = Rect::from_min_max(pos2(row.left() + 190.0, row.center().y - 3.0), pos2(row.right() - 76.0, row.center().y + 3.0));
            if bar.width() > 30.0 {
                well(p, &th, bar, 3.0);
                let f = sn.buckets[i] as f32 / total_b as f32;
                grad_fill(p, Rect::from_min_size(bar.min, vec2((bar.width() * f).max(6.0), 6.0)), 3.0, mix(cols[i], Color32::WHITE, 0.2), cols[i]);
            }
        }
        cy = leg.bottom() + 14.0;
        // top wins
        let wins = Rect::from_min_max(pos2(right.left(), cy), pos2(right.right(), body.bottom()));
        if wins.height() > 70.0 {
            p.text(pos2(wins.left() + 4.0, wins.top() + 8.0), Align2::LEFT_CENTER, "BIGGEST WINS", f_semi(T_LABEL), th.ink4);
            let n = ((wins.height() - 24.0) / 62.0).floor().clamp(0.0, 3.0) as usize;
            for (i, j) in sn.cands.iter().take(n).enumerate() {
                let r = Rect::from_min_size(pos2(wins.left(), wins.top() + 22.0 + i as f32 * 62.0), vec2(wins.width(), 54.0));
                self.junk_row(ui, p, r, j, false, "ov", None);
            }
            if sn.cands.is_empty() && sn.finished {
                p.text(pos2(wins.left() + 6.0, wins.top() + 44.0), Align2::LEFT_CENTER, "Nothing obviously wasteful found. Nice and tidy.", f_medium(T_BODY), th.ink3);
            }
        }
    }

    fn safety_color(&self, s: Safety) -> Color32 {
        match s {
            Safety::Safe => self.th.good,
            Safety::Review => self.th.warn,
            Safety::Careful => self.th.danger,
        }
    }

    /// One junk candidate row. `check` = Some(checked) draws a checkbox. Returns true when the row body was clicked.
    fn junk_row(&mut self, ui: &Ui, p: &egui::Painter, r: Rect, j: &Junk, with_check: bool, key: &str, checked: Option<bool>) -> bool {
        let th = self.th.clone();
        let resp = self.hit(ui, r, (key, &j.path), Sense::click());
        let hv = self.anim(ui, (key, "h", &j.path), resp.hovered());
        elevate(p, &th, r, 14.0, 1);
        grad_fill(p, r, 14.0, th.card_top, th.card_bottom);
        if hv > 0.01 {
            p.rect_filled(r, cr(14.0), alpha(th.accent, 0.06 * hv));
        }
        rim(p, r, 14.0, 1.0, th.rim, true, 24.0);
        let mut x = r.left() + 14.0;
        if with_check {
            let on = checked.unwrap_or(false);
            let bx = Rect::from_center_size(pos2(x + 9.0, r.center().y), vec2(18.0, 18.0));
            let t = spring_to(ui.ctx(), ("stck", &j.path), if on { 1.0 } else { 0.0 }, 420.0, 28.0).clamp(0.0, 1.0);
            well(p, &th, bx, 6.0);
            if t > 0.02 {
                let s = bx.shrink((1.0 - t) * 8.0);
                let (a, b) = th.accent_grad();
                grad_fill(p, s, 6.0, alpha(a, t), alpha(b, t));
                self.ico(p, "check-read", Ic::Filled, bx.center(), 12.0 * t, Color32::from_white_alpha((255.0 * t) as u8));
            }
            x += 30.0;
        }
        let ic = Rect::from_center_size(pos2(x + 17.0, r.center().y), vec2(34.0, 34.0));
        let kc = self.safety_color(j.safety);
        grad_fill(p, ic, 11.0, alpha(kc, 0.20), alpha(kc, 0.08));
        self.ico(p, j.kind.icon(), Ic::Duo, ic.center(), 17.0, kc);
        let tx = ic.right() + 12.0;
        let right_w = 150.0;
        let max_w = (r.right() - tx - right_w - 12.0).max(60.0);
        p.text(pos2(tx, r.center().y - 9.0), Align2::LEFT_CENTER, self.fit(p, &j.name, &f_semi(T_BODY), max_w), f_semi(T_BODY), th.ink1);
        let sub = format!("{} \u{b7} {}", self.pretty_path(&Location::new(self.storage.as_ref().map(|s| s.fs.clone()).unwrap_or_default(), j.path.clone()).parent().unwrap_or_else(|| self.be.home())), j.reason);
        p.text(pos2(tx, r.center().y + 10.0), Align2::LEFT_CENTER, self.fit(p, &sub, &f_medium(T_META), max_w), f_medium(T_META), th.ink3);
        p.text(pos2(r.right() - 14.0, r.center().y - 9.0), Align2::RIGHT_CENTER, format_size(j.size()), f_bold(T_BODY), th.ink1);
        let badge = j.safety.label();
        let bw = self.tw(p, badge, &f_semi(10.0)) + 18.0;
        let br = Rect::from_min_size(pos2(r.right() - 14.0 - bw, r.center().y + 2.0), vec2(bw, 18.0));
        grad_fill(p, br, 9.0, alpha(kc, 0.22), alpha(kc, 0.12));
        p.text(br.center(), Align2::CENTER_CENTER, badge, f_semi(10.0), kc);
        if resp.hovered() {
            if let Some(cmd) = j.restore {
                resp.clone().on_hover_text(format!("Comes back with: {cmd}"));
            }
        }
        if resp.secondary_clicked() {
            if let Some(loc) = Location::new(self.storage.as_ref().map(|s| s.fs.clone()).unwrap_or_default(), j.path.clone()).parent() {
                self.push_act(Act::NewTab(Some(Route::Loc(loc))));
            }
        }
        resp.clicked()
    }

    // ───────────── cleanup ─────────────
    fn storage_cleanup(&mut self, ui: &Ui, p: &egui::Painter, body: Rect, st: &mut StorageState, sn: &Snap) {
        let th = self.th.clone();
        // category chips
        let mut x = body.left();
        let mut y = body.top();
        let mut chips: Vec<(Option<JunkKind>, String, &'static str)> = vec![(None, format!("All \u{b7} {}", format_size(sn.all)), "layers-minimalistic")];
        for (k, s, n) in &sn.kinds {
            chips.push((Some(*k), format!("{} \u{b7} {} ({n})", k.label(), format_size(*s)), k.icon()));
        }
        for (k, label, icon) in chips {
            let w = self.tw(p, &label, &f_semi(T_META)) + 46.0;
            if x + w > body.right() {
                x = body.left();
                y += 38.0;
            }
            let r = Rect::from_min_size(pos2(x, y), vec2(w, 30.0));
            let on = st.filter == k;
            let resp = self.hit(ui, r, ("st-chip", format!("{:?}", k)), Sense::click());
            let a = self.anim(ui, ("st-chip-h", format!("{:?}", k)), resp.hovered());
            if on {
                grad_fill(p, r, 15.0, th.accent_tint, th.accent_tint2);
                rim(p, r, 15.0, 1.0, alpha(Color32::WHITE, if th.dark { 0.12 } else { 0.7 }), true, 14.0);
            } else {
                raised(p, &th, r, 15.0, resp.hovered(), resp.is_pointer_button_down_on());
            }
            self.ico(p, icon, if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 18.0, r.center().y), 14.0, if on { th.accent_ink } else { mix(th.ink3, th.ink1, a) });
            p.text(pos2(r.left() + 32.0, r.center().y), Align2::LEFT_CENTER, label, f_semi(T_META), if on { th.accent_ink } else { th.ink2 });
            if resp.clicked() {
                st.filter = k;
            }
            x += w + 8.0;
        }
        y += 42.0;
        let foot_h = 54.0;
        let list = Rect::from_min_max(pos2(body.left(), y), pos2(body.right(), body.bottom() - foot_h - 8.0));
        let rows: Vec<&Junk> = sn.cands.iter().filter(|j| st.filter.map(|k| j.kind == k).unwrap_or(true)).collect();
        let row_h = 64.0;
        let content_h = rows.len() as f32 * row_h + 8.0;
        let off = self.vscroll(ui, list, "st-cleanup", content_h);
        let lp = p.with_clip_rect(list.expand2(vec2(2.0, 0.0)));
        if rows.is_empty() {
            let msg = if sn.finished { "Nothing to clean in this category." } else { "Looking for things you can clean\u{2026}" };
            lp.text(list.center(), Align2::CENTER_CENTER, msg, f_medium(T_BODY), th.ink3);
        }
        for (i, j) in rows.iter().enumerate() {
            let r = Rect::from_min_size(pos2(list.left(), list.top() + i as f32 * row_h - off), vec2(list.width() - 14.0, 56.0));
            if r.bottom() < list.top() - 4.0 || r.top() > list.bottom() + 4.0 {
                continue;
            }
            let on = st.sel.contains(&j.path);
            if self.junk_row(ui, &lp, r, j, true, "st-row", Some(on)) {
                if on {
                    st.sel.remove(&j.path);
                } else {
                    st.sel.insert(j.path.clone());
                }
            }
        }
        // sticky action bar
        let bar = Rect::from_min_size(pos2(body.left(), body.bottom() - foot_h), vec2(body.width(), foot_h));
        elevate(p, &th, bar, 18.0, 3);
        grad_fill(p, bar, 18.0, th.panel_top, th.panel_bottom);
        rim(p, bar, 18.0, 1.2, th.rim, true, 30.0);
        let sel_b: u64 = sn.cands.iter().filter(|j| st.sel.contains(&j.path)).map(|j| j.size()).sum();
        let msg = if st.sel.is_empty() { "Tick items to move them to the Trash. Nothing is deleted until you confirm.".to_string() } else { format!("{} selected \u{b7} frees {}", st.sel.len(), format_size(sel_b)) };
        p.text(pos2(bar.left() + 20.0, bar.center().y), Align2::LEFT_CENTER, self.fit(p, &msg, &f_semi(T_BODY), bar.width() - 420.0), f_semi(T_BODY), if st.sel.is_empty() { th.ink3 } else { th.ink1 });
        let mut rx = bar.right() - 14.0;
        let b3 = Rect::from_min_size(pos2(rx - 134.0, bar.center().y - 17.0), vec2(134.0, 34.0));
        if self.label_button(ui, b3, Some("trash-bin-minimalistic"), "Move to Trash", "st-trash", if st.sel.is_empty() { Btn::Raised } else { Btn::Danger }).clicked() && !st.sel.is_empty() {
            self.push_act(Act::App("storage.ask_clean".into()));
        }
        rx = b3.left() - 8.0;
        let b2 = Rect::from_min_size(pos2(rx - 112.0, bar.center().y - 17.0), vec2(112.0, 34.0));
        if self.label_button(ui, b2, Some("shield-check"), "Select safe", "st-selsafe", Btn::Raised).clicked() {
            st.sel = sn.cands.iter().filter(|j| j.safety == Safety::Safe).map(|j| j.path.clone()).collect();
        }
        if !st.sel.is_empty() {
            let b1 = Rect::from_min_size(pos2(b2.left() - 8.0 - 76.0, bar.center().y - 17.0), vec2(76.0, 34.0));
            if self.label_button(ui, b1, None, "Clear", "st-clear", Btn::Quiet).clicked() {
                st.sel.clear();
            }
        }
    }

    // ───────────── treemap ─────────────
    fn storage_map(&mut self, ui: &Ui, p: &egui::Painter, body: Rect, st: &mut StorageState) {
        let th = self.th.clone();
        let scan = st.scan.lock().unwrap();
        let focus = st.map_focus.min(scan.nodes.len() - 1);
        // breadcrumb
        let chain = scan.chain(focus);
        let mut x = body.left();
        let mut jump = None;
        for (i, n) in chain.iter().enumerate() {
            let label = if i == 0 { "Everything".to_string() } else { scan.nodes[*n].name.clone() };
            let w = self.tw(p, &label, &f_semi(T_META)) + 22.0;
            let r = Rect::from_min_size(pos2(x, body.top()), vec2(w, 28.0));
            let last = i + 1 == chain.len();
            let resp = self.hit(ui, r, ("st-mapcrumb", i), Sense::click());
            if last {
                grad_fill(p, r, 14.0, th.accent_tint, th.accent_tint2);
            } else if resp.hovered() {
                p.rect_filled(r, cr(14.0), th.row_hover);
            }
            p.text(r.center(), Align2::CENTER_CENTER, label, f_semi(T_META), if last { th.accent_ink } else { th.ink3 });
            if resp.clicked() && !last {
                jump = Some(*n);
            }
            x += w + 4.0;
            if !last {
                p.text(pos2(x, r.center().y), Align2::LEFT_CENTER, "\u{203a}", f_regular(T_BODY), th.ink4);
                x += 14.0;
            }
        }
        p.text(pos2(body.right(), body.top() + 14.0), Align2::RIGHT_CENTER, format!("{} \u{b7} click a folder to zoom in, double-click to open it", format_size(scan.nodes[focus].size)), f_medium(T_META), th.ink4);
        let area = Rect::from_min_max(pos2(body.left(), body.top() + 38.0), body.max);
        let mut items = scan.items(focus);
        let total: u64 = items.iter().map(|i| i.size).sum::<u64>().max(1);
        // fold the long tail into one tile so the map never turns to dust
        let mut tail = 0u64;
        let mut keep = vec![];
        for it in items.drain(..) {
            if keep.len() >= 48 || (it.size as f64 / total as f64) < 0.004 {
                tail += it.size;
            } else {
                keep.push(it);
            }
        }
        if tail > 0 {
            keep.push(Item { label: "smaller items".into(), size: tail, node: None, path: String::new(), is_dir: false, junk: false, kind: Kind::Other, modified: None });
        }
        let sizes: Vec<f64> = keep.iter().map(|i| i.size as f64).collect();
        let rects = squarify(&sizes, area.left() as f64, area.top() as f64, area.width() as f64, area.height() as f64);
        let pointer = ui.input(|i| i.pointer.hover_pos());
        let mut open: Option<usize> = None;
        let mut tip: Option<(String, Pos2)> = None;
        for (i, it) in keep.iter().enumerate() {
            let (x, y, w, h) = rects[i];
            let r = Rect::from_min_size(pos2(x as f32, y as f32), vec2(w as f32, h as f32)).shrink(1.5);
            if r.width() < 3.0 || r.height() < 3.0 {
                continue;
            }
            let resp = self.hit(ui, r, ("st-tile", &it.path, i), Sense::click());
            let a = self.anim(ui, ("st-tile-h", &it.path, i), resp.hovered());
            let base = if it.is_dir { hue_for(&it.label, &th) } else if it.path.is_empty() { th.ink4 } else { kind_color(it.kind, &th) };
            let base = if it.junk { mix(base, th.danger, 0.55) } else { base };
            let (c0, c1) = (alpha(mix(base, Color32::WHITE, 0.16 + 0.1 * a), if th.dark { 0.78 } else { 0.90 }), alpha(mix(base, Color32::BLACK, 0.12), if th.dark { 0.72 } else { 0.86 }));
            grad_fill(p, r, 9.0, c0, c1);
            rim(p, r, 9.0, 1.0, Color32::from_white_alpha(if a > 0.5 { 110 } else { 50 }), true, 10.0);
            if it.junk {
                p.rect_filled(Rect::from_min_size(r.min + vec2(8.0, 6.0), vec2(r.width().min(40.0) - 8.0, 3.0)), cr(1.5), th.danger);
            }
            if r.width() > 74.0 && r.height() > 36.0 {
                let tp = p.with_clip_rect(r.shrink(4.0));
                tp.text(r.min + vec2(10.0, 20.0), Align2::LEFT_CENTER, &it.label, f_semi(T_META), Color32::WHITE);
                tp.text(r.min + vec2(10.0, 38.0), Align2::LEFT_CENTER, format!("{} \u{b7} {:.0}%", format_size(it.size), it.size as f64 / total as f64 * 100.0), f_medium(10.5), Color32::from_white_alpha(210));
            }
            if resp.hovered() {
                tip = Some((format!("{}  \u{b7}  {}", it.label, format_size(it.size)), pointer.unwrap_or(r.center())));
            }
            if resp.double_clicked() {
                let path = if it.is_dir { it.path.clone() } else { it.path.rsplit_once('/').map(|x| x.0.to_string()).unwrap_or_default() };
                if !path.is_empty() {
                    open = Some(1);
                    let l = Location::new(scan.fs.clone(), if path.is_empty() { "/".into() } else { path });
                    self.push_act(Act::NewTab(Some(Route::Loc(l))));
                }
            } else if resp.clicked() {
                if let (true, Some(n)) = (it.is_dir, it.node) {
                    jump = Some(n);
                }
            }
        }
        let _ = open;
        if let Some((t, pos)) = tip {
            let w = self.tw(p, &t, &f_semi(T_META)) + 24.0;
            let r = Rect::from_min_size(pos2((pos.x + 14.0).min(body.right() - w), (pos.y + 18.0).min(body.bottom() - 30.0)), vec2(w, 26.0));
            let tp = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("st-tip")));
            elevate(&tp, &th, r, 13.0, 3);
            grad_fill(&tp, r, 13.0, th.panel_top, th.panel_bottom);
            tp.text(r.center(), Align2::CENTER_CENTER, t, f_semi(T_META), th.ink1);
        }
        drop(scan);
        if let Some(n) = jump {
            st.map_focus = n;
        }
    }

    // ───────────── graph ─────────────
    fn build_graph(scan: &Scan, focus: usize, th: &Theme) -> (Vec<GNode>, Vec<(usize, usize)>) {
        let root_size = scan.nodes[focus].size.max(1) as f32;
        let mut nodes: Vec<GNode> = vec![];
        let mut edges = vec![];
        let r_of = |s: u64| (3.0 + 30.0 * (s as f32 / root_size).sqrt()).clamp(3.0, 34.0);
        let seed = |i: usize| -> f32 {
            let mut h = (i as u32).wrapping_mul(2654435761);
            h ^= h >> 15;
            (h % 1000) as f32 / 1000.0
        };
        nodes.push(GNode { idx: Some(focus), label: scan.nodes[focus].name.clone(), path: scan.nodes[focus].path.clone(), size: scan.nodes[focus].size, pos: Vec2::ZERO, vel: Vec2::ZERO, r: 22.0, depth: 0, pinned: false, junk: false, is_dir: true, kind: Kind::Folder });
        let mut frontier = vec![(focus, 0usize)];
        while let Some((n, gi)) = frontier.first().cloned() {
            frontier.remove(0);
            let depth = nodes[gi].depth;
            if depth >= 4 || nodes.len() >= 190 {
                continue;
            }
            let mut kids: Vec<usize> = scan.nodes[n].children.clone();
            kids.sort_by_key(|c| std::cmp::Reverse(scan.nodes[*c].size));
            for c in kids.into_iter().take(if depth == 0 { 12 } else { 6 }) {
                let s = scan.nodes[c].size;
                if (s as f32 / root_size) < 0.004 && depth > 0 {
                    continue;
                }
                let ang = seed(nodes.len()) * std::f32::consts::TAU;
                let rad = 60.0 + 70.0 * (depth as f32 + 1.0);
                let id = nodes.len();
                nodes.push(GNode { idx: Some(c), label: scan.nodes[c].name.clone(), path: scan.nodes[c].path.clone(), size: s, pos: nodes[gi].pos + vec2(ang.cos(), ang.sin()) * rad * 0.5, vel: Vec2::ZERO, r: r_of(s), depth: depth + 1, pinned: false, junk: scan.nodes[c].junk.is_some(), is_dir: true, kind: Kind::Folder });
                edges.push((gi, id));
                frontier.push((c, id));
            }
            for f in scan.nodes[n].top_files.iter().take(2) {
                if (f.size as f32 / root_size) < 0.01 || nodes.len() >= 190 {
                    continue;
                }
                let ang = seed(nodes.len() + 77) * std::f32::consts::TAU;
                let id = nodes.len();
                nodes.push(GNode { idx: None, label: f.name.clone(), path: f.path.clone(), size: f.size, pos: nodes[gi].pos + vec2(ang.cos(), ang.sin()) * 50.0, vel: Vec2::ZERO, r: r_of(f.size).min(14.0), depth: depth + 1, pinned: false, junk: false, is_dir: false, kind: f.kind });
                edges.push((gi, id));
            }
        }
        let _ = th;
        (nodes, edges)
    }

    fn graph_physics(g: &mut Graph) {
        let n = g.nodes.len();
        let mut force = vec![Vec2::ZERO; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let d = g.nodes[i].pos - g.nodes[j].pos;
                let dist2 = d.length_sq().max(36.0);
                let k = 2600.0 * (g.nodes[i].r + g.nodes[j].r) / 40.0;
                let f = d / dist2.sqrt() * (k / dist2);
                force[i] += f;
                force[j] -= f;
            }
        }
        for (a, b) in &g.edges {
            let d = g.nodes[*b].pos - g.nodes[*a].pos;
            let dist = d.length().max(0.01);
            let rest = 46.0 + g.nodes[*a].r + g.nodes[*b].r;
            let f = d / dist * ((dist - rest) * 0.06);
            force[*a] += f;
            force[*b] -= f;
        }
        let mut energy = 0.0;
        for i in 0..n {
            let node = &mut g.nodes[i];
            if node.pinned {
                node.vel = Vec2::ZERO;
                continue;
            }
            let f = force[i] - node.pos * 0.012;
            node.vel = (node.vel + f * 0.9) * 0.82;
            if node.vel.length() > 14.0 {
                node.vel = node.vel.normalized() * 14.0;
            }
            node.pos += node.vel;
            energy += node.vel.length_sq();
        }
        g.heat = (energy / n.max(1) as f32).sqrt();
    }

    fn storage_graph(&mut self, ui: &Ui, p: &egui::Painter, body: Rect, st: &mut StorageState) {
        let th = self.th.clone();
        let scan = st.scan.lock().unwrap();
        let focus = st.graph.focus.min(scan.nodes.len() - 1);
        let sig = (focus, scan.nodes.len() / 8);
        if st.graph.sig != sig || st.graph.focus == usize::MAX {
            let (nodes, edges) = Self::build_graph(&scan, focus, &th);
            let g = &mut st.graph;
            g.focus = focus;
            g.sig = sig;
            g.nodes = nodes;
            g.edges = edges;
            g.heat = 10.0;
            g.pan = Vec2::ZERO;
            g.zoom = 1.0;
        }
        let g = &mut st.graph;
        // canvas
        elevate(p, &th, body, 18.0, 1);
        grad_fill(p, body, 18.0, if th.dark { c(10, 14, 24, 0.9) } else { c(246, 248, 253, 1.0) }, if th.dark { c(14, 19, 32, 0.92) } else { c(236, 241, 250, 1.0) });
        rim(p, body, 18.0, 1.2, th.rim, true, 30.0);
        let gp = p.with_clip_rect(body.shrink(1.0));
        let resp = self.hit(ui, body, "st-graph", Sense::click_and_drag());
        let center = body.center();
        let to_screen = |pos: Vec2, g: &Graph| center + pos * g.zoom + g.pan;
        let pointer = ui.input(|i| i.pointer.hover_pos());
        // zoom with the wheel around the pointer
        if resp.hovered() {
            let sd = ui.input(|i| i.smooth_scroll_delta.y);
            if sd != 0.0 {
                let old = g.zoom;
                g.zoom = (g.zoom * (1.0 + sd * 0.0015)).clamp(0.35, 3.5);
                if let Some(pp) = pointer {
                    let rel = pp - center - g.pan;
                    g.pan -= rel * (g.zoom / old - 1.0);
                }
            }
        }
        // hover & drag
        let under = pointer.and_then(|pp| g.nodes.iter().enumerate().filter(|(_, n)| (to_screen(n.pos, g) - pp).length() <= n.r * g.zoom + 5.0).min_by(|a, b| (to_screen(a.1.pos, g) - pp).length().partial_cmp(&(to_screen(b.1.pos, g) - pp).length()).unwrap()).map(|x| x.0));
        if resp.drag_started() {
            g.drag = under;
        }
        if resp.dragged() {
            match g.drag {
                Some(i) => {
                    if let Some(pp) = pointer {
                        g.nodes[i].pos = (pp - center - g.pan) / g.zoom;
                        g.nodes[i].pinned = true;
                        g.heat = g.heat.max(2.0);
                    }
                }
                None => g.pan += resp.drag_delta(),
            }
        }
        if resp.drag_stopped() {
            if let Some(i) = g.drag.take() {
                g.nodes[i].pinned = false;
            }
        }
        g.hover = if g.drag.is_some() { g.drag } else { under };
        if g.heat > 0.05 || g.drag.is_some() {
            for _ in 0..2 {
                Self::graph_physics(g);
            }
            ui.ctx().request_repaint();
        }
        let neighbours: HashSet<usize> = match g.hover {
            Some(h) => g.edges.iter().filter_map(|(a, b)| if *a == h { Some(*b) } else if *b == h { Some(*a) } else { None }).chain(std::iter::once(h)).collect(),
            None => HashSet::new(),
        };
        let dim = |i: usize, g: &Graph| g.hover.is_some() && !neighbours.contains(&i);
        for (a, b) in &g.edges {
            let (pa, pb) = (to_screen(g.nodes[*a].pos, g), to_screen(g.nodes[*b].pos, g));
            let faded = g.hover.is_some() && !(neighbours.contains(a) && neighbours.contains(b) && (Some(*a) == g.hover || Some(*b) == g.hover));
            gp.line_segment([pa, pb], egui::Stroke::new(1.2, alpha(th.ink3, if faded { 0.10 } else { 0.5 })));
        }
        for (i, n) in g.nodes.iter().enumerate() {
            let pos = to_screen(n.pos, g);
            let r = n.r * g.zoom;
            let base = if n.junk { th.danger } else if n.is_dir { hue_for(&n.label, &th) } else { kind_color(n.kind, &th) };
            let fade = if dim(i, g) { 0.25 } else { 1.0 };
            if g.hover == Some(i) || n.junk {
                gp.circle_filled(pos, r + 7.0, alpha(base, 0.16 * fade));
            }
            gp.circle_filled(pos, r, alpha(mix(base, Color32::WHITE, if n.is_dir { 0.0 } else { 0.25 }), 0.95 * fade));
            gp.circle_stroke(pos, r, egui::Stroke::new(1.2, Color32::from_white_alpha((90.0 * fade) as u8)));
            let show = g.hover == Some(i) || (r > 11.0 && !dim(i, g)) || (g.zoom > 1.6 && r > 6.0);
            if show {
                let t = self.fit(&gp, &n.label, &f_semi(T_META), 150.0);
                gp.text(pos + vec2(0.0, r + 10.0), Align2::CENTER_CENTER, t, f_semi(T_META), alpha(th.ink1, fade));
            }
        }
        if let Some(h) = g.hover {
            let n = &g.nodes[h];
            let t = format!("{}  \u{b7}  {}{}", n.label, format_size(n.size), if n.junk { "  \u{b7}  cleanable" } else { "" });
            let w = self.tw(p, &t, &f_semi(T_META)) + 24.0;
            let pp = to_screen(n.pos, g);
            let r = Rect::from_min_size(pos2((pp.x - w / 2.0).clamp(body.left() + 8.0, body.right() - w - 8.0), (pp.y - n.r * g.zoom - 38.0).max(body.top() + 8.0)), vec2(w, 26.0));
            elevate(&gp, &th, r, 13.0, 3);
            grad_fill(&gp, r, 13.0, th.panel_top, th.panel_bottom);
            gp.text(r.center(), Align2::CENTER_CENTER, t, f_semi(T_META), th.ink1);
        }
        gp.text(pos2(body.left() + 18.0, body.bottom() - 18.0), Align2::LEFT_CENTER, "Drag nodes \u{b7} scroll to zoom \u{b7} drag the background to pan \u{b7} click a folder to focus it \u{b7} double-click to open", f_medium(T_META), th.ink4);
        if g.focus != 0 {
            let b = Rect::from_min_size(pos2(body.right() - 118.0, body.top() + 14.0), vec2(104.0, 32.0));
            let parent = scan.nodes[g.focus].parent;
            if self.label_button(ui, b, Some("alt-arrow-left"), "Up one", "st-graph-up", Btn::Raised).clicked() {
                if let Some(pn) = parent {
                    g.focus = pn;
                    g.sig = (usize::MAX, 0);
                }
            }
        }
        // clicks
        if resp.clicked() {
            if let Some(i) = under {
                let n = &g.nodes[i];
                if let (true, Some(ix)) = (n.is_dir, n.idx) {
                    if ix != g.focus {
                        g.focus = ix;
                        g.sig = (usize::MAX, 0);
                    }
                }
            }
        }
        if resp.double_clicked() {
            if let Some(i) = under {
                let n = &g.nodes[i];
                let path = if n.is_dir { n.path.clone() } else { n.path.rsplit_once('/').map(|x| x.0.to_string()).unwrap_or_else(|| "/".into()) };
                self.push_act(Act::NewTab(Some(Route::Loc(Location::new(scan.fs.clone(), path)))));
            }
        }
    }

    // ───────────── biggest ─────────────
    fn storage_biggest(&mut self, ui: &Ui, p: &egui::Painter, body: Rect, st: &mut StorageState, sn: &Snap) {
        let th = self.th.clone();
        let two = body.width() > 820.0;
        let cols: Vec<Rect> = if two {
            let w = (body.width() - 16.0) / 2.0;
            vec![Rect::from_min_size(body.min, vec2(w, body.height())), Rect::from_min_size(pos2(body.left() + w + 16.0, body.top()), vec2(w, body.height()))]
        } else {
            vec![body]
        };
        let fs = st.fs.clone();
        let max_f = sn.big_files.first().map(|f| f.size).unwrap_or(1).max(1);
        let max_d = sn.big_dirs.first().map(|d| d.2).unwrap_or(1).max(1);
        let sets: [(&str, usize); 2] = [("LARGEST FILES", sn.big_files.len()), ("LARGEST FOLDERS", sn.big_dirs.len())];
        for (ci, (title, n)) in sets.iter().enumerate() {
            if !two && ci == 1 {
                break;
            }
            let area = cols[ci.min(cols.len() - 1)];
            p.text(pos2(area.left() + 4.0, area.top() + 8.0), Align2::LEFT_CENTER, *title, f_semi(T_LABEL), th.ink4);
            let list = Rect::from_min_max(pos2(area.left(), area.top() + 22.0), area.max);
            let row_h = 52.0;
            let total_n = if two { *n } else { sn.big_files.len() + sn.big_dirs.len() };
            let off = self.vscroll(ui, list, &format!("st-big-{ci}"), total_n as f32 * row_h + 8.0);
            let lp = p.with_clip_rect(list);
            for i in 0..total_n {
                let (name, path, size, is_dir, kind, max) = if (two && ci == 0) || (!two && i < sn.big_files.len()) {
                    let f = &sn.big_files[i];
                    (f.name.clone(), f.path.clone(), f.size, false, f.kind, max_f)
                } else {
                    let d = &sn.big_dirs[if two { i } else { i - sn.big_files.len() }];
                    (d.1.clone(), d.0.clone(), d.2, true, Kind::Folder, max_d)
                };
                let r = Rect::from_min_size(pos2(list.left(), list.top() + i as f32 * row_h - off), vec2(list.width() - 14.0, 46.0));
                if r.bottom() < list.top() || r.top() > list.bottom() {
                    continue;
                }
                let resp = self.hit(ui, r, ("st-bigrow", ci, i), Sense::click());
                let a = self.anim(ui, ("st-bigrow-h", ci, i), resp.hovered());
                elevate(&lp, &th, r, 12.0, 1);
                grad_fill(&lp, r, 12.0, th.card_top, th.card_bottom);
                rim(&lp, r, 12.0, 1.0, th.rim, true, 20.0);
                let col = if is_dir { hue_for(&name, &th) } else { kind_color(kind, &th) };
                let frac = size as f32 / max as f32;
                let bar = Rect::from_min_size(pos2(r.left() + 12.0, r.bottom() - 9.0), vec2((r.width() - 24.0) * frac, 3.0));
                lp.rect_filled(bar, cr(1.5), alpha(col, 0.55 + 0.3 * a));
                self.ico(&lp, if is_dir { "folder" } else { kind.icon() }, Ic::Duo, pos2(r.left() + 26.0, r.center().y - 4.0), 18.0, col);
                lp.text(pos2(r.left() + 48.0, r.center().y - 8.0), Align2::LEFT_CENTER, self.fit(&lp, &name, &f_semi(T_BODY), r.width() - 160.0), f_semi(T_BODY), th.ink1);
                let parent = Location::new(fs.clone(), path.clone()).parent().map(|l| self.pretty_path(&l)).unwrap_or_default();
                lp.text(pos2(r.left() + 48.0, r.center().y + 7.0), Align2::LEFT_CENTER, self.fit(&lp, &parent, &f_medium(10.5), r.width() - 160.0), f_medium(10.5), th.ink3);
                lp.text(pos2(r.right() - 14.0, r.center().y - 4.0), Align2::RIGHT_CENTER, format_size(size), f_bold(T_BODY), th.ink1);
                if resp.clicked() {
                    let l = Location::new(fs.clone(), path.clone());
                    let target = if is_dir { l } else { l.parent().unwrap_or_else(|| self.be.home()) };
                    self.push_act(Act::NewTab(Some(Route::Loc(target))));
                }
            }
        }
    }
}
