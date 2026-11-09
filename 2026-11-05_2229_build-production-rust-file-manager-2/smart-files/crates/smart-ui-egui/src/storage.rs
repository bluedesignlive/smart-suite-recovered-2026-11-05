//! Storage workspace — a real tab, not a dialog: usage overview, smart junk
//! cleaning (build caches, not just "large files"), a size breakdown and a
//! connected-dots graph of your folder tree.
use crate::app::*;
use crate::chrome::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Pos2, Rect, Sense, Ui};
use smart_core::*;

#[derive(Clone)]
pub struct JunkGroup {
    pub title: String,
    pub icon: &'static str,
    pub desc: String,
    pub paths: Vec<String>,
    pub bytes: u64,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn dir_size(p: &std::path::Path, depth: usize, files: &mut usize) -> u64 {
    if *files > 20_000 || depth == 0 {
        return 0;
    }
    let Ok(rd) = std::fs::read_dir(p) else { return 0 };
    let mut sum = 0u64;
    for e in rd.flatten() {
        let Ok(m) = e.metadata() else { continue };
        if m.is_file() {
            *files += 1;
            sum += m.len();
        } else if m.is_dir() {
            sum += dir_size(&e.path(), depth - 1, files);
        }
    }
    sum
}

#[cfg(not(target_arch = "wasm32"))]
fn walk_names(root: &std::path::Path, depth: usize, names: &[&str], out: &mut Vec<(String, u64)>) {
    let Ok(rd) = std::fs::read_dir(root) else { return };
    for e in rd.flatten() {
        let Ok(m) = e.metadata() else { continue };
        if !m.is_dir() {
            continue;
        }
        let n = e.file_name().to_string_lossy().to_string();
        if names.contains(&n.as_str()) {
            let mut f = 0;
            out.push((e.path().to_string_lossy().into_owned(), dir_size(&e.path(), 6, &mut f)));
        } else if depth > 0 {
            walk_names(&e.path(), depth - 1, names, out);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn walk_logs(root: &std::path::Path, depth: usize, out: &mut Vec<(String, u64)>) {
    let Ok(rd) = std::fs::read_dir(root) else { return };
    for e in rd.flatten() {
        let Ok(m) = e.metadata() else { continue };
        if m.is_file() {
            let n = e.file_name().to_string_lossy().to_lowercase();
            if n.ends_with(".log") || n.ends_with(".log.1") {
                out.push((e.path().to_string_lossy().into_owned(), m.len()));
            }
        } else if depth > 0 {
            walk_logs(&e.path(), depth - 1, out);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn walk_empty(root: &std::path::Path, depth: usize, out: &mut Vec<String>) {
    let Ok(rd) = std::fs::read_dir(root) else { return };
    let mut any = false;
    for e in rd.flatten() {
        any = true;
        if let Ok(m) = e.metadata() {
            if m.is_dir() && depth > 0 {
                walk_empty(&e.path(), depth - 1, out);
            }
        }
    }
    if !any {
        out.push(root.to_string_lossy().into_owned());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn walk_big(root: &std::path::Path, depth: usize, out: &mut Vec<(String, u64)>) {
    if out.len() >= 25 {
        return;
    }
    let Ok(rd) = std::fs::read_dir(root) else { return };
    for e in rd.flatten() {
        let Ok(m) = e.metadata() else { continue };
        if m.is_file() {
            if m.len() > 1 << 30 {
                out.push((e.path().to_string_lossy().into_owned(), m.len()));
            }
        } else if depth > 0 {
            walk_big(&e.path(), depth - 1, out);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn scan(home: &str) -> (Vec<JunkGroup>, Vec<(String, u64)>) {
    use std::path::Path;
    let hp = Path::new(home);
    let mut groups = vec![];
    let mut caches = vec![];
    walk_names(hp, 3, &["target", "node_modules", ".gradle", "__pycache__", ".cache", ".next", ".venv", "vendor"], &mut caches);
    if !caches.is_empty() {
        let bytes = caches.iter().map(|(_, b)| b).sum();
        groups.push(JunkGroup { title: "Build & package caches".into(), icon: "box", desc: "target/, node_modules/, .cache, __pycache__ and friends".into(), paths: caches.into_iter().map(|(p, _)| p).collect(), bytes });
    }
    let mut logs = vec![];
    walk_logs(hp, 3, &mut logs);
    if !logs.is_empty() {
        let bytes = logs.iter().map(|(_, b)| b).sum();
        groups.push(JunkGroup { title: "Log files".into(), icon: "document-text", desc: format!("{} .log files", logs.len()), paths: logs.into_iter().map(|(p, _)| p).collect(), bytes });
    }
    if let Some(trash) = std::env::var("HOME").ok().map(|h| std::path::PathBuf::from(h).join(".local/share/Trash/files")) {
        if trash.exists() {
            let mut f = 0;
            let b = dir_size(&trash, 8, &mut f);
            if b > 0 {
                groups.push(JunkGroup { title: "Trash".into(), icon: "trash-bin-minimalistic", desc: "Already deleted — reclaim the space".into(), paths: vec![trash.to_string_lossy().into_owned()], bytes: b });
            }
        }
    }
    let mut empties = vec![];
    walk_empty(hp, 3, &mut empties);
    if !empties.is_empty() {
        groups.push(JunkGroup { title: "Empty folders".into(), icon: "folder-open", desc: format!("{} folders with nothing inside", empties.len()), paths: empties, bytes: 0 });
    }
    let mut big = vec![];
    walk_big(hp, 4, &mut big);
    if !big.is_empty() {
        let bytes = big.iter().map(|(_, b)| b).sum();
        groups.push(JunkGroup { title: "Very large files".into(), icon: "database", desc: "Over 1 GB each — review before removing".into(), paths: big.into_iter().map(|(p, _)| p).collect(), bytes });
    }
    let mut bd: Vec<(String, u64)> = vec![];
    if let Ok(rd) = std::fs::read_dir(hp) {
        for e in rd.flatten() {
            let Ok(m) = e.metadata() else { continue };
            let b = if m.is_dir() { let mut f = 0; dir_size(&e.path(), 4, &mut f) } else { m.len() };
            bd.push((e.file_name().to_string_lossy().into_owned(), b));
        }
    }
    bd.sort_by(|a, b| b.1.cmp(&a.1));
    bd.truncate(10);
    (groups, bd)
}

pub fn usage_color(f: f32, th: &Theme) -> Color32 {
    if f > 0.9 { th.danger } else if f > 0.75 { th.warn } else { th.accent }
}

pub fn draw(app: &mut SmartApp, ui: &mut Ui, rect: Rect) {
    let p = ui.painter().clone();
    let th = app.th.clone();
    let dev = app.current_device();
    let (name, used, text) = match &dev {
        Some(d) => (d.name.clone(), d.used_fraction(), d.capacity_text()),
        None => ("Computer".into(), 0.0, String::new()),
    };
    let head = Rect::from_min_size(rect.min + vec2(4.0, 4.0), vec2(rect.width() - 8.0, 92.0));
    elevate(&p, &th, head, 16.0, 2);
    grad_fill(&p, head, 16.0, th.card_top, th.card_bottom);
    rim(&p, head, 16.0, 1.2, th.rim, true, 30.0);
    p.text(pos2(head.left() + 20.0, head.top() + 24.0), Align2::LEFT_CENTER, name, f_bold(T_TITLE), th.ink1);
    let col = usage_color(used, &th);
    p.text(pos2(head.right() - 20.0, head.top() + 24.0), Align2::RIGHT_CENTER, format!("{:.0}% used", used * 100.0), f_semi(T_BODY), col);
    let bar = Rect::from_min_size(pos2(head.left() + 20.0, head.top() + 44.0), vec2(head.width() - 40.0, 8.0));
    well(&p, &th, bar, 4.0);
    grad_fill(&p, Rect::from_min_size(bar.min, vec2((bar.width() * used).max(8.0), 8.0)), 4.0, col, col);
    p.text(pos2(head.left() + 20.0, head.top() + 70.0), Align2::LEFT_CENTER, text, f_medium(T_META), th.ink3);
    if used > 0.9 {
        p.text(pos2(head.right() - 20.0, head.top() + 70.0), Align2::RIGHT_CENTER, "Running low — review cleanup below", f_medium(T_META), th.danger);
    }
    let seg = Rect::from_min_size(pos2(head.left() + 20.0, head.bottom() + 14.0), vec2(330.0, 34.0));
    well(&p, &th, seg, 17.0);
    let views = ["Clean up", "Breakdown", "Graph"];
    for (i, v) in views.iter().enumerate() {
        let r = Rect::from_min_size(pos2(seg.left() + 4.0 + i as f32 * 108.0, seg.top() + 3.0), vec2(104.0, 28.0));
        let resp = app.hit(ui, r, ("stview", i), Sense::click());
        let on = app.storage_view == i as u8;
        if on {
            elevate(&p, &th, r, 14.0, 1);
            grad_fill(&p, r, 14.0, th.panel_top, th.panel_bottom);
        } else if resp.hovered() {
            p.rect_filled(r, cr(14.0), th.row_hover);
        }
        p.text(r.center(), Align2::CENTER_CENTER, *v, if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, if on { th.accent_ink } else { th.ink2 });
        if resp.clicked() {
            app.storage_view = i as u8;
        }
    }
    let body = Rect::from_min_max(pos2(rect.left() + 4.0, seg.bottom() + 14.0), pos2(rect.right() - 4.0, rect.bottom() - 4.0));
    let mut sui = ui.new_child(egui::UiBuilder::new().max_rect(body).layout(egui::Layout::top_down(egui::Align::Min)));
    sui.set_clip_rect(body.intersect(ui.clip_rect()));
    egui::ScrollArea::vertical().id_salt("storage-scroll").auto_shrink([false, false]).show(&mut sui, |ui| match app.storage_view {
        0 => draw_clean(app, ui, body),
        1 => draw_breakdown(app, ui, body, &th),
        _ => draw_graph(app, ui, body, &th),
    });
}

fn draw_clean(app: &mut SmartApp, ui: &mut Ui, body: Rect) {
    let p = ui.painter().clone();
    let th = app.th.clone();
    let mut y = ui.max_rect().top();
    if app.storage_scan.is_none() {
        let b = Rect::from_min_size(pos2(body.left(), y + 8.0), vec2(180.0, 38.0));
        if app.label_button(ui, b, Some("magnifer"), "Scan this device", "st-scan", Btn::Accent).clicked() {
            let home = app.be.home();
            #[cfg(not(target_arch = "wasm32"))]
            if !app.be.demo {
                let (g, bd) = scan(&home.path);
                app.storage_scan = Some(g);
                app.storage_breakdown = Some(bd);
            }
            if app.storage_scan.is_none() {
                app.storage_scan = Some(vec![]);
                app.toast("Storage scanning runs in the desktop build", false);
            }
        }
        p.text(pos2(body.left(), y + 62.0), Align2::LEFT_CENTER, "Finds build caches (target/, node_modules/…), logs, empty folders,", f_medium(T_META), th.ink3);
        p.text(pos2(body.left(), y + 78.0), Align2::LEFT_CENTER, "the Trash and oversized files. Nothing is touched until you confirm.", f_medium(T_META), th.ink3);
        return;
    }
    let groups = app.storage_scan.clone().unwrap_or_default();
    if groups.is_empty() {
        p.text(pos2(body.left(), y + 10.0), Align2::LEFT_CENTER, "Nothing to clean — this device is tidy.", f_semi(T_BODY), th.good);
        return;
    }
    let mut clean: Option<usize> = None;
    for (i, g) in groups.iter().enumerate() {
        let r = Rect::from_min_size(pos2(body.left(), y), vec2(body.width(), 56.0));
        elevate(&p, &th, r, 14.0, 1);
        grad_fill(&p, r, 14.0, th.card_top, th.card_bottom);
        rim(&p, r, 14.0, 1.0, th.rim, true, 24.0);
        let on = app.storage_sel.contains(&i);
        let cb = Rect::from_min_size(r.min + vec2(14.0, 17.0), vec2(22.0, 22.0));
        let cbr = app.hit(ui, cb, ("stsel", i), Sense::click());
        well(&p, &th, cb, 7.0);
        if on {
            let (a, b) = th.accent_grad();
            grad_fill(&p, cb, 7.0, a, b);
            app.ico(&p, "check", Ic::Filled, cb.center(), 13.0, th.on_accent());
        } else if cbr.hovered() {
            rim(&p, cb, 7.0, 1.4, th.accent, true, 10.0);
        }
        if cbr.clicked() {
            if on { app.storage_sel.remove(&i); } else { app.storage_sel.insert(i); }
        }
        app.ico(&p, g.icon, Ic::Duo, pos2(r.left() + 58.0, r.center().y), 20.0, [th.accent, th.warn, th.ink3, th.good, th.danger][i % 5]);
        p.text(pos2(r.left() + 80.0, r.center().y - 9.0), Align2::LEFT_CENTER, g.title.clone(), f_semi(T_BODY), th.ink1);
        p.text(pos2(r.left() + 80.0, r.center().y + 10.0), Align2::LEFT_CENTER, g.desc.clone(), f_medium(T_META), th.ink3);
        p.text(pos2(r.right() - 106.0, r.center().y), Align2::RIGHT_CENTER, format_size(g.bytes), f_semi(T_BODY), th.ink2);
        let bb = Rect::from_min_size(pos2(r.right() - 92.0, r.center().y - 15.0), vec2(78.0, 30.0));
        if app.label_button(ui, bb, Some("trash-bin-minimalistic"), "Clean", "stclean", Btn::Raised).clicked() {
            clean = Some(i);
        }
        y += 64.0;
    }
    if let Some(i) = clean {
        let g = &groups[i];
        let locs: Vec<Location> = g.paths.iter().map(|p| Location::new("file", p.clone())).collect();
        app.submit_job(JobKind::Trash { srcs: locs });
        app.toast(format!("Moving {} to Trash", g.title), true);
        app.storage_scan = None;
        app.storage_sel.clear();
    }
    if !app.storage_sel.is_empty() {
        let sel_bytes: u64 = app.storage_sel.iter().filter_map(|i| groups.get(*i)).map(|g| g.bytes).sum();
        let fb = Rect::from_min_size(pos2(body.left(), y + 6.0), vec2(320.0, 38.0));
        if app.label_button(ui, fb, Some("trash-bin-minimalistic"), &format!("Move {} selected to Trash · {}", app.storage_sel.len(), format_size(sel_bytes)), "stcleanall", Btn::Danger).clicked() {
            let mut locs = vec![];
            for i in &app.storage_sel {
                if let Some(g) = groups.get(*i) {
                    locs.extend(g.paths.iter().map(|p| Location::new("file", p.clone())));
                }
            }
            app.submit_job(JobKind::Trash { srcs: locs });
            app.toast("Moving selection to Trash", true);
            app.storage_scan = None;
            app.storage_sel.clear();
        }
    }
}

fn draw_breakdown(app: &mut SmartApp, ui: &mut Ui, body: Rect, th: &Theme) {
    let p = ui.painter().clone();
    let mut y = ui.max_rect().top();
    let bd = app.storage_breakdown.clone().unwrap_or_default();
    if bd.is_empty() {
        p.text(pos2(body.left(), y + 8.0), Align2::LEFT_CENTER, "Run a scan first — Clean up → Scan this device.", f_medium(T_META), th.ink3);
        return;
    }
    let max = bd.iter().map(|(_, b)| *b).max().unwrap_or(1).max(1);
    p.text(pos2(body.left(), y + 6.0), Align2::LEFT_CENTER, "WHERE THE SPACE GOES", f_semi(T_LABEL), th.ink4);
    y += 26.0;
    for (name, b) in &bd {
        let r = Rect::from_min_size(pos2(body.left(), y), vec2(body.width(), 34.0));
        let resp = app.hit(ui, r, ("bd", name.clone()), Sense::click());
        if resp.hovered() {
            p.rect_filled(r, cr(10.0), th.row_hover);
        }
        p.text(pos2(r.left() + 8.0, r.center().y), Align2::LEFT_CENTER, app.fit(&p, name, &f_medium(T_BODY), 220.0), f_medium(T_BODY), th.ink1);
        let bw = (r.width() - 340.0).max(60.0);
        let bar = Rect::from_min_size(pos2(r.left() + 240.0, r.center().y - 4.0), vec2(bw, 8.0));
        well(&p, th, bar, 4.0);
        grad_fill(&p, Rect::from_min_size(bar.min, vec2((bw * (*b as f32 / max as f32)).max(6.0), 8.0)), 4.0, th.accent, th.accent);
        p.text(pos2(r.right() - 8.0, r.center().y), Align2::RIGHT_CENTER, format_size(*b), f_semi(T_META), th.ink2);
        if resp.clicked() {
            app.push_act(Act::Nav(Route::Loc(app.be.home().join(name))));
        }
        y += 36.0;
    }
}

fn draw_graph(app: &mut SmartApp, ui: &mut Ui, body: Rect, th: &Theme) {
    let p = ui.painter().clone();
    let home = app.be.home();
    let mut nodes: Vec<(String, String, usize, u64, usize)> = vec![]; // label, path, depth, bytes, parent idx
    #[cfg(not(target_arch = "wasm32"))]
    if !app.be.demo {
        if let Ok(rd) = std::fs::read_dir(&home.path) {
            for e in rd.flatten() {
                let Ok(m) = e.metadata() else { continue };
                if !m.is_dir() {
                    continue;
                }
                let mut ff = 0;
                let b = dir_size(&e.path(), 3, &mut ff);
                let pi = nodes.len();
                nodes.push((e.file_name().to_string_lossy().to_string(), e.path().to_string_lossy().into_owned(), 1, b, 0));
                if pi < 6 {
                    if let Ok(rd2) = std::fs::read_dir(e.path()) {
                        for e2 in rd2.flatten().take(5) {
                            if e2.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                                let mut ff2 = 0;
                                let b2 = dir_size(&e2.path(), 2, &mut ff2);
                                nodes.push((e2.file_name().to_string_lossy().to_string(), e2.path().to_string_lossy().into_owned(), 2, b2, pi));
                            }
                        }
                    }
                }
            }
        }
    }
    let c = body.center();
    let maxb = nodes.iter().map(|n| n.3).max().unwrap_or(1).max(1);
    let pos_of = |i: usize| -> Pos2 {
        let n = &nodes[i];
        if n.2 == 1 {
            let ones = nodes.iter().filter(|m| m.2 == 1).count().max(1);
            let idx = nodes[..i].iter().filter(|m| m.2 == 1).count();
            let ang = (idx as f32 / ones as f32) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            let rad = body.width().min(body.height()) * 0.30;
            pos2(c.x + ang.cos() * rad, c.y + ang.sin() * rad * 0.82)
        } else {
            let pp = pos_of_parent(&nodes, i, c, body);
            pp
        }
    };
    fn pos_of_parent(nodes: &[(String, String, usize, u64, usize)], i: usize, c: Pos2, body: Rect) -> Pos2 {
        let n = &nodes[i];
        let pi = n.4;
        let ones = nodes.iter().filter(|m| m.2 == 1).count().max(1);
        let idx = nodes[..pi].iter().filter(|m| m.2 == 1).count();
        let ang = (idx as f32 / ones as f32) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        let rad = body.width().min(body.height()) * 0.30;
        let base = pos2(c.x + ang.cos() * rad, c.y + ang.sin() * rad * 0.82);
        let kids: Vec<usize> = nodes.iter().enumerate().filter(|(_, m)| m.2 == 2 && m.4 == pi).map(|(j, _)| j).collect();
        let ki = kids.iter().position(|&j| j == i).unwrap_or(0) as f32;
        let kn = kids.len().max(1) as f32;
        let spread = (ki - (kn - 1.0) / 2.0) * 0.22;
        pos2(base.x + (ang + spread).cos() * 74.0, base.y + (ang + spread).sin() * 74.0)
    }
    // edges
    for i in 0..nodes.len() {
        if nodes[i].2 == 2 {
            let a = pos_of_parent(&nodes, i, c, body);
            let pi = nodes[i].4;
            let ones = nodes.iter().filter(|m| m.2 == 1).count().max(1);
            let idx = nodes[..pi].iter().filter(|m| m.2 == 1).count();
            let ang = (idx as f32 / ones as f32) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            let rad = body.width().min(body.height()) * 0.30;
            let b = pos2(c.x + ang.cos() * rad, c.y + ang.sin() * rad * 0.82);
            p.line_segment([a, b], egui::Stroke::new(1.0, alpha(th.accent, 0.22)));
        }
    }
    for i in 0..nodes.len() {
        let n = nodes[i].clone();
        let pos = if n.2 == 1 { pos_of(i) } else { pos_of_parent(&nodes, i, c, body) };
        let r = (6.0 + 20.0 * (n.3 as f32 / maxb as f32).sqrt()).min(28.0);
        let resp = app.hit(ui, Rect::from_center_size(pos, vec2(r * 2.0, r * 2.0)), ("node", i), Sense::click());
        let col = if n.2 == 1 { th.accent } else { alpha(th.accent, 0.55) };
        p.circle_filled(pos, r, alpha(col, if resp.hovered() { 0.5 } else { 0.26 }));
        p.circle_stroke(pos, r, egui::Stroke::new(1.4, col));
        if n.2 == 1 {
            p.text(pos2(pos.x, pos.y + r + 11.0), Align2::CENTER_CENTER, app.fit(&p, &n.0, &f_medium(10.0), 90.0), f_medium(10.0), th.ink2);
        }
        if resp.hovered() {
            p.text(pos2(pos.x, pos.y - r - 9.0), Align2::CENTER_CENTER, format!("{} · {}", n.0, format_size(n.3)), f_semi(T_META), th.ink1);
            ui.ctx().request_repaint();
        }
        if resp.clicked() && n.2 == 1 {
            app.push_act(Act::Nav(Route::Loc(Location::new("file", n.1.clone()))));
        }
    }
    if nodes.is_empty() {
        p.text(c, Align2::CENTER_CENTER, "The folder graph runs on the desktop build.", f_medium(T_META), th.ink3);
    }
    let _ = (home, pos_of);
}
