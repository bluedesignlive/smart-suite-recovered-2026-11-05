//! Transfer dialog. Every operation has its own scene, its own sound and a real-feeling disk:
//!   copy    — sheets arc from folder to folder, a ghost stays behind
//!   move    — cards ride a rail out of the source, the source drains
//!   extract — a zip crate unzips top to bottom and files burst out of the opening
//! Below the stage a drive unit flashes its LEDs and lights sectors as bytes land. Throughput (not a guessed
//! countdown) is the headline; the estimate only appears once it is long enough to be honest.
use crate::app::*;
use crate::chrome::*;
use crate::overlays::win_rect;
use crate::paint::*;
use crate::sound::Snd;
use crate::theme::*;
use crate::widgets::*;
use egui::{pos2, vec2, Align2, Color32, Context, Id, Order, Painter, Pos2, Rect, Sense, Shape, Stroke, Vec2};
use smart_core::*;

fn rot(c: Pos2, v: Vec2, ang: f32) -> Pos2 {
    let (s, co) = ang.sin_cos();
    c + vec2(v.x * co - v.y * s, v.x * s + v.y * co)
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A sheet of paper with a folded corner and ruled lines, rotated about `c`.
pub fn sheet(p: &Painter, c: Pos2, w: f32, h: f32, ang: f32, body: Color32, ink: Color32, a: f32) {
    if a <= 0.01 {
        return;
    }
    let (hw, hh, fold) = (w / 2.0, h / 2.0, w * 0.3);
    let pts = |dx: f32, dy: f32| -> Vec<Pos2> { [(-hw, -hh), (hw - fold, -hh), (hw, -hh + fold), (hw, hh), (-hw, hh)].iter().map(|(x, y)| rot(c, vec2(*x + dx, *y + dy), ang)).collect() };
    p.add(Shape::convex_polygon(pts(0.0, 2.0), alpha(Color32::BLACK, 0.20 * a), Stroke::NONE));
    p.add(Shape::convex_polygon(pts(0.0, 0.0), alpha(body, a), Stroke::new(1.0, alpha(ink, 0.35 * a))));
    let tri: Vec<Pos2> = [(hw - fold, -hh), (hw, -hh + fold), (hw - fold, -hh + fold)].iter().map(|(x, y)| rot(c, vec2(*x, *y), ang)).collect();
    p.add(Shape::convex_polygon(tri, alpha(mix(body, ink, 0.25), a), Stroke::new(1.0, alpha(ink, 0.3 * a))));
    for k in 0..3 {
        let y = -hh + h * (0.46 + 0.16 * k as f32);
        let x1 = hw - w * 0.18 - if k == 2 { w * 0.28 } else { 0.0 };
        p.line_segment([rot(c, vec2(-hw + w * 0.18, y), ang), rot(c, vec2(x1, y), ang)], Stroke::new(1.4, alpha(ink, 0.45 * a)));
    }
}

fn fgeo(ctr: Pos2, w: f32, h: f32, squash: f32) -> Rect {
    let sy = 1.0 - 0.06 * squash;
    let sx = 1.0 + 0.04 * squash;
    Rect::from_center_size(pos2(ctr.x, ctr.y + h * (1.0 - sy) / 2.0), vec2(w * sx, h * sy))
}

/// Back half of a tight 3D folder: contact shadow, tab, back panel and two paper edges peeking out.
pub fn folder_back(p: &Painter, th: &Theme, ctr: Pos2, w: f32, h: f32, open: f32, squash: f32, hue: Color32) {
    let r = fgeo(ctr, w, h, squash);
    shadow(p, r.translate(vec2(0.0, 3.0)), 9.0, 5.0, 11.0, -3.0, alpha(th.shadow, if th.dark { 0.65 } else { 0.30 }));
    let tab = Rect::from_min_size(r.min, vec2(r.width() * 0.40, h * 0.22));
    grad_fill(p, tab, 6.0, mix(hue, Color32::WHITE, 0.10), mix(hue, Color32::BLACK, 0.14));
    let back = Rect::from_min_max(pos2(r.left(), r.top() + h * 0.12), r.max);
    grad_fill(p, back, 9.0, mix(hue, Color32::BLACK, 0.16), mix(hue, Color32::BLACK, 0.36));
    rim(p, back, 9.0, 1.0, Color32::from_white_alpha(46), true, 10.0);
    for k in 0..2 {
        let pr = Rect::from_min_size(pos2(r.left() + 6.0 + k as f32 * 2.5, back.top() + 3.0 - open * 3.5 - k as f32 * 1.5), vec2(r.width() - 12.0 - k as f32 * 5.0, 6.0));
        p.rect_filled(pr, cr(2.0), alpha(Color32::WHITE, 0.88 - 0.22 * k as f32));
    }
}

pub fn folder_front(p: &Painter, ctr: Pos2, w: f32, h: f32, open: f32, squash: f32, hue: Color32) {
    let r = fgeo(ctr, w, h, squash);
    let top = r.top() + h * (0.38 + 0.08 * open);
    let front = Rect::from_min_max(pos2(r.left(), top), r.max);
    grad_fill(p, front, 9.0, mix(hue, Color32::WHITE, 0.34), mix(hue, Color32::WHITE, 0.02));
    rim(p, front, 9.0, 1.3, Color32::from_white_alpha(170), true, 22.0);
    rim(p, front, 9.0, 1.0, Color32::from_black_alpha(46), false, 6.0);
    p.line_segment([pos2(front.left() + 8.0, top + 2.0), pos2(front.right() - 8.0, top + 2.0)], Stroke::new(1.0, Color32::from_white_alpha(120)));
}

#[derive(Clone, Copy, PartialEq)]
enum Xfer {
    Copy,
    Move,
    Extract,
}

const FILE_COLS: [&str; 6] = ["#6e9bff", "#59c79a", "#e5b05a", "#d86fb0", "#8f7cf5", "#ff8a6b"];

#[allow(clippy::too_many_arguments)]
fn scene(sp: &Painter, th: &Theme, kind: Xfer, stage: Rect, anim: f32, sf: f32, src_c: Pos2, dst_c: Pos2, fw: f32, fh: f32) {
    let hue_src = mix(th.ink3, th.accent, 0.35);
    let hue_dst = th.accent;
    let cols: Vec<Color32> = FILE_COLS.iter().map(|h| hex(h)).collect();
    let (n, period) = match kind {
        Xfer::Copy => (5usize, 1.6f32),
        Xfer::Move => (6, 1.9),
        Xfer::Extract => (8, 1.35),
    };
    let speed_k = 0.75 + 0.25 * (sf * 6.0).min(1.0);
    let (mut dst_bump, mut src_bump, mut dst_open, mut src_open) = (0.0f32, 0.0f32, 0.25f32, 0.25f32);
    let mut flying: Vec<(f32, usize)> = vec![];
    for k in 0..n {
        let tt = ((anim * speed_k / period) + k as f32 / n as f32).fract();
        dst_open = dst_open.max(1.0 - ((tt - 0.86).abs() / 0.18).clamp(0.0, 1.0));
        src_open = src_open.max(1.0 - ((tt - 0.07).abs() / 0.14).clamp(0.0, 1.0));
        let since_land = ((tt - 0.90).rem_euclid(1.0)) * period;
        if since_land < 0.9 {
            dst_bump += (-since_land * 7.5).exp() * (since_land * 30.0).sin();
        }
        let since_lift = ((tt - 0.02).rem_euclid(1.0)) * period;
        if since_lift < 0.6 {
            src_bump -= (-since_lift * 9.0).exp() * (since_lift * 26.0).sin() * 0.6;
        }
        flying.push((tt, k));
    }
    let mouth_src = pos2(src_c.x, src_c.y - fh * 0.30);
    let mouth_dst = pos2(dst_c.x, dst_c.y - fh * 0.30);
    // source side: a folder, or the zip crate
    let pull_y = src_c.y - fh * 0.55 + fh * 1.1 * sf;
    if kind == Xfer::Extract {
        let r = Rect::from_center_size(src_c, vec2(fw * 0.92, fh * 1.18));
        shadow(sp, r.translate(vec2(0.0, 3.0)), 9.0, 5.0, 11.0, -3.0, alpha(th.shadow, if th.dark { 0.65 } else { 0.30 }));
        grad_fill(sp, r, 9.0, mix(hex("#8f7cf5"), Color32::WHITE, 0.25), mix(hex("#5b4fd6"), Color32::BLACK, 0.15));
        rim(sp, r, 9.0, 1.3, Color32::from_white_alpha(160), true, 26.0);
        // opened part of the crate darkens below the pull
        let open_r = Rect::from_min_max(pos2(r.center().x - 6.0, pull_y), pos2(r.center().x + 6.0, r.bottom() - 3.0));
        if open_r.height() > 1.0 {
            grad_fill(sp, open_r, 3.0, alpha(Color32::BLACK, 0.55), alpha(Color32::BLACK, 0.25));
        }
        let teeth = 16;
        for i in 0..teeth {
            let y = r.top() + 4.0 + (r.height() - 8.0) * i as f32 / teeth as f32;
            let spread = if y < pull_y { 0.0 } else { ((y - pull_y) * 0.18).min(6.0) };
            let sx = if i % 2 == 0 { -1.6 } else { 1.6 };
            sp.rect_filled(Rect::from_center_size(pos2(r.center().x + sx * (1.0 + spread * 0.6) + if sx > 0.0 { spread } else { -spread }, y), vec2(3.0, 2.2)), cr(1.0), Color32::from_white_alpha(if y < pull_y { 230 } else { 150 }));
        }
        let pull = Rect::from_center_size(pos2(r.center().x, pull_y), vec2(9.0, 13.0));
        shadow(sp, pull, 3.0, 1.5, 3.0, 0.0, alpha(th.shadow, 0.4));
        grad_fill(sp, pull, 3.0, hex("#fff3c4"), hex("#e6b84a"));
        sp.circle_filled(pull.center() + vec2(0.0, 2.5), 1.6, alpha(Color32::BLACK, 0.45));
    } else {
        folder_back(sp, th, src_c, fw, fh, src_open, src_bump, hue_src);
    }
    folder_back(sp, th, dst_c, fw, fh, dst_open, dst_bump, hue_dst);
    let arc_h = if kind == Xfer::Move { 10.0 } else { 34.0 };
    let (from, to) = if kind == Xfer::Extract { (pos2(src_c.x + 8.0, pull_y), mouth_dst) } else { (mouth_src, mouth_dst) };
    // guide: arc for copy/extract, dashed rail for move
    for i in 0..30 {
        let (t0, t1) = (i as f32 / 30.0, (i as f32 + 0.5) / 30.0);
        let pt = |t: f32| pos2(from.x + (to.x - from.x) * t, from.y - arc_h * 4.0 * t * (1.0 - t));
        if kind == Xfer::Move {
            let ph = (anim * 0.9).fract();
            let t0 = (t0 + ph / 30.0 * 2.0).min(1.0);
            let t1 = (t1 + ph / 30.0 * 2.0).min(1.0);
            sp.line_segment([pt(t0), pt(t1)], Stroke::new(1.6, alpha(th.accent, 0.28)));
        } else if i % 2 == 0 {
            sp.line_segment([pt(t0), pt(t1)], Stroke::new(1.0, alpha(th.accent, 0.14)));
        }
    }
    flying.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (t, k) in &flying {
        let t = *t;
        let e = smooth(t * 0.9 + 0.05);
        let x = from.x + (to.x - from.x) * e;
        let lift = (4.0 * t * (1.0 - t)).powf(0.8);
        let fall = if t > 0.84 { (t - 0.84) / 0.16 * 26.0 * smooth((t - 0.84) / 0.16) } else { 0.0 };
        let y = from.y + if kind == Xfer::Extract { 4.0 } else { 14.0 } - (arc_h + 14.0) * lift + fall;
        let col = cols[*k % cols.len()];
        let fade = smooth(t * 9.0) * smooth((1.0 - t) * 18.0 + 0.25);
        let sc = 0.62 + 0.38 * (std::f32::consts::PI * t).sin().powf(0.5);
        match kind {
            Xfer::Move => {
                // rounded cards riding the rail
                let cc = pos2(x, from.y + 6.0 - arc_h * lift * 0.4 + fall * 0.6 + (anim * 3.0 + *k as f32).sin() * 1.2);
                let r = Rect::from_center_size(cc, vec2(22.0, 26.0) * sc);
                shadow(sp, r, 5.0, 3.0, 6.0, -1.0, alpha(th.shadow, 0.35 * fade));
                grad_fill(sp, r, 5.0, alpha(mix(Color32::WHITE, col, 0.30), fade), alpha(mix(Color32::WHITE, col, 0.62), fade));
                rim(sp, r, 5.0, 1.0, Color32::from_white_alpha((200.0 * fade) as u8), true, 12.0);
                sp.circle_filled(r.center() - vec2(0.0, 2.0), 3.0 * sc, alpha(Color32::WHITE, 0.8 * fade));
            }
            _ => {
                let tumble = (t - 0.5) * (1.1 + (*k as f32 * 1.7).sin() * 0.5) + (*k as f32 * 2.3).sin() * 0.25;
                sheet(sp, pos2(x, y), 20.0 * sc, 26.0 * sc, tumble, mix(Color32::WHITE, col, 0.14), mix(th.ink1, col, 0.6), fade);
                for s in 1..=2 {
                    let tp = (t - s as f32 * 0.035).max(0.0);
                    let ep = smooth(tp * 0.9 + 0.05);
                    let px = from.x + (to.x - from.x) * ep;
                    let py = from.y + 4.0 - arc_h * (4.0 * tp * (1.0 - tp)).powf(0.8);
                    sp.circle_filled(pos2(px, py + 6.0), 2.2 - s as f32 * 0.6, alpha(col, 0.35 * fade / s as f32));
                }
            }
        }
        let since = ((t - 0.90).rem_euclid(1.0)) * period;
        if since < 0.45 {
            for d in 0..5 {
                let ang = -std::f32::consts::PI * (0.15 + 0.7 * d as f32 / 4.0);
                let rr = since * 70.0;
                sp.circle_filled(mouth_dst + vec2(ang.cos() * rr, ang.sin() * rr + 4.0), 1.9 * (1.0 - since / 0.45), alpha(cols[(*k + d) % cols.len()], 0.9 * (1.0 - since / 0.45)));
            }
        }
    }
    if kind != Xfer::Extract {
        folder_front(sp, src_c, fw, fh, src_open, src_bump, hue_src);
        if kind == Xfer::Move {
            // the source drains: a fading dashed ghost where it used to be full
            let g = Rect::from_center_size(src_c + vec2(0.0, 2.0), vec2(fw, fh));
            sp.rect_stroke(g.expand(4.0), cr(11.0), Stroke::new(1.0, alpha(th.ink3, 0.35 * smooth(sf * 2.0))), egui::StrokeKind::Outside);
        }
    }
    folder_front(sp, dst_c, fw, fh, dst_open, dst_bump, hue_dst);
    let _ = stage;
}

impl SmartApp {
    pub(crate) fn m_copy_progress(&mut self, ctx: &Context) {
        let Some(id) = self.copy_dialog else { return };
        let Some(job) = self.jobs.iter().find(|j| j.id == id).map(|j| j.clone_job_view()) else {
            self.copy_dialog = None;
            return;
        };
        ctx.request_repaint();
        let win = win_rect();
        let th = self.th.clone();
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        self.copy_dialog_anim += dt;
        let anim = self.copy_dialog_anim;
        let info = self.last_transfer.clone();
        let frac = (job.done as f32 / job.total.max(1) as f32).clamp(0.0, 1.0);
        let sf = spring_to(ctx, "cp-frac", frac, 90.0, 22.0).clamp(0.0, 1.0);
        let pop = spring_out(anim).min(1.04);
        let lower = job.title.to_lowercase();
        let kind = if lower.starts_with("extract") {
            Xfer::Extract
        } else if lower.starts_with("mov") {
            Xfer::Move
        } else {
            Xfer::Copy
        };
        let first = info.as_ref().and_then(|(s, _)| s.first()).cloned();
        let name = first.as_ref().map(|l| l.name()).unwrap_or_else(|| job.title.clone());
        let size = first.as_ref().and_then(|l| self.be.stat(l).ok()).map(|e| e.size).unwrap_or(5_000_000_000);
        let dest = info.as_ref().map(|(_, d)| d.clone()).unwrap_or_else(|| self.target_dir());
        let paced = first.as_ref().is_some_and(|l| self.be.stat(l).map(|e| e.paced).unwrap_or(false));

        // sounds: start, disk ticks while writing, finish
        let flag = |k: &str| -> bool { ctx.data_mut(|d| d.get_temp::<bool>(Id::new((k, id))).unwrap_or(false)) };
        let set = |k: &str| ctx.data_mut(|d| d.insert_temp(Id::new((k, id)), true));
        if !flag("cps") {
            set("cps");
            self.shell.snd.play(if kind == Xfer::Extract { Snd::Unzip } else { Snd::XferStart });
        }
        let tick_n = (anim * 8.0) as u32;
        let last_tick: u32 = ctx.data_mut(|d| d.get_temp(Id::new(("cpt", id))).unwrap_or(0));
        if tick_n != last_tick && frac < 0.98 {
            ctx.data_mut(|d| d.insert_temp(Id::new(("cpt", id)), tick_n));
            self.shell.snd.play_vol(Snd::Disk, 0.5 + 0.5 * hash01(tick_n));
        }
        if frac >= 0.985 && !flag("cpd") {
            set("cpd");
            self.shell.snd.play(if kind == Xfer::Extract { Snd::UnzipDone } else { Snd::XferDone });
            self.shell.notify_done(if kind == Xfer::Extract { "Extraction complete" } else { "Transfer complete" }, &name);
        }

        // throughput: smoothed (EMA) so the number is stable; the estimate only shows when it is long enough to matter
        let elapsed = job.age.max(0.1);
        let raw = if frac > 0.01 { size as f32 * frac / elapsed } else { 0.0 };
        let ema: f32 = ctx.data_mut(|d| d.get_temp(Id::new(("cpe", id))).unwrap_or(raw));
        let ema = ema + (raw - ema) * (1.0 - (-dt * 2.0).exp());
        ctx.data_mut(|d| d.insert_temp(Id::new(("cpe", id)), ema));
        let wob = 1.0 + 0.10 * (anim * 2.1).sin() + 0.05 * (anim * 5.3).sin();
        let speed = (ema * wob).max(1.0);
        let mut hist: Vec<f32> = ctx.data_mut(|d| d.get_temp(Id::new(("cph", id))).unwrap_or_default());
        if tick_n != last_tick {
            hist.push(speed);
            if hist.len() > 48 {
                hist.remove(0);
            }
            ctx.data_mut(|d| d.insert_temp(Id::new(("cph", id)), hist.clone()));
        }
        let remaining = if ema > 1.0 { size as f32 * (1.0 - frac) / ema } else { 0.0 };
        let eta = if frac < 0.06 {
            "estimating\u{2026}".to_string()
        } else if frac > 0.97 {
            "finishing\u{2026}".into()
        } else if remaining >= 90.0 {
            format!("about {} min left", (remaining / 60.0).round() as u32)
        } else {
            "under a minute left".into()
        };

        egui::Area::new(Id::new("copy-dialog")).order(Order::Foreground).fixed_pos(win.min).show(ctx, |ui| {
            let p = ui.painter().clone();
            ui.set_clip_rect(win);
            ui.set_opacity(smooth(anim * 6.0));
            let _scrim = ui.allocate_rect(win, Sense::click());
            grad_fill(&p, win, R_WIN, alpha(th.scrim, 0.45), alpha(th.scrim, 0.8));
            let w = 600.0f32.min(win.width() - 32.0);
            let h = 392.0f32.min(win.height() - 32.0);
            let card = Rect::from_center_size(win.center() + vec2(0.0, (1.0 - pop) * 18.0), vec2(w, h));
            menu_surface(&p, &th, card, 24.0);
            ui.allocate_rect(card, Sense::click());
            let pad = 22.0;

            // header: skeuomorphic ring + names
            let ctr = pos2(card.left() + pad + 27.0, card.top() + 44.0);
            ring_gauge(&p, &th, ctr, 27.0, sf, &format!("{}", (sf * 100.0).round() as u32), anim as f64);
            let tx = ctr.x + 27.0 + 16.0;
            let badge_w = if paced { 92.0 } else { 0.0 };
            p.text(pos2(tx, card.top() + 34.0), Align2::LEFT_CENTER, self.fit(&p, &name, &f_bold(T_TITLE), card.right() - tx - pad - badge_w - 8.0), f_bold(T_TITLE), th.ink1);
            let verb = match kind {
                Xfer::Extract => "Extracting",
                Xfer::Move => "Moving",
                Xfer::Copy => "Copying",
            };
            p.text(pos2(tx, card.top() + 56.0), Align2::LEFT_CENTER, self.fit(&p, &format!("{verb} {} to {}", format_size(size), self.pretty_path(&dest)), &f_medium(T_META), card.right() - tx - pad), f_medium(T_META), th.ink3);
            if paced {
                let badge = Rect::from_min_size(pos2(card.right() - pad - 84.0, card.top() + 24.0), vec2(84.0, 20.0));
                grad_fill(&p, badge, 10.0, alpha(th.accent, 0.20), alpha(th.accent, 0.10));
                p.text(badge.center(), Align2::CENTER_CENTER, format!("{} DEMO", format_size(size).to_uppercase()), f_semi(9.0), th.accent_ink);
            }

            // stage
            let stage = Rect::from_min_max(pos2(card.left() + pad, card.top() + 88.0), pos2(card.right() - pad, card.top() + 192.0));
            grad_fill(&p, stage, 16.0, alpha(th.accent, if th.dark { 0.10 } else { 0.07 }), alpha(th.accent, if th.dark { 0.03 } else { 0.02 }));
            rim(&p, stage, 16.0, 2.0, alpha(th.shadow, if th.dark { 0.35 } else { 0.07 }), true, 12.0);
            let sp = p.with_clip_rect(stage);
            let src_c = pos2(stage.left() + 62.0, stage.center().y + 6.0);
            let dst_c = pos2(stage.right() - 62.0, stage.center().y + 6.0);
            scene(&sp, &th, kind, stage, anim, sf, src_c, dst_c, 56.0, 42.0);
            let src_label = match kind {
                Xfer::Extract => name.clone(),
                _ => first.as_ref().map(|l| if l.fs == dest.fs { l.parent().map(|p| p.name()).unwrap_or_else(|| "Source".into()) } else { "Source".into() }).unwrap_or_else(|| "Source".into()),
            };
            p.text(pos2(src_c.x, stage.bottom() - 11.0), Align2::CENTER_CENTER, self.fit(&p, &src_label, &f_semi(10.5), 104.0), f_semi(10.5), th.ink3);
            p.text(pos2(dst_c.x, stage.bottom() - 11.0), Align2::CENTER_CENTER, self.fit(&p, &dest.name(), &f_semi(10.5), 104.0), f_semi(10.5), th.accent_ink);

            // disk row: drive unit with flashing LEDs, sector map, throughput sparkline
            let row = Rect::from_min_size(pos2(card.left() + pad, stage.bottom() + 12.0), vec2(w - 2.0 * pad, 50.0));
            let act = 0.55 + 0.45 * hash01((anim * 9.0) as u32);
            drive_unit(&p, &th, Rect::from_min_size(row.min, vec2(150.0, 50.0)), act, sf, anim as f64, &format!("{} \u{b7} WRITE", dest.name().to_uppercase().chars().take(10).collect::<String>()));
            let spark = Rect::from_min_max(pos2(row.left() + 162.0, row.top()), row.max);
            well(&p, &th, spark, 12.0);
            let sp2 = p.with_clip_rect(spark.shrink(1.0));
            if hist.len() > 1 {
                let mx = hist.iter().cloned().fold(1.0f32, f32::max) * 1.25;
                let pts: Vec<Pos2> = hist.iter().enumerate().map(|(i, v)| pos2(spark.left() + 8.0 + (spark.width() - 16.0) * i as f32 / 47.0, spark.bottom() - 8.0 - (spark.height() - 16.0) * (v / mx))).collect();
                let mut area = pts.clone();
                area.push(pos2(pts.last().unwrap().x, spark.bottom() - 6.0));
                area.push(pos2(pts[0].x, spark.bottom() - 6.0));
                sp2.add(Shape::convex_polygon(area, alpha(th.accent, 0.12), Stroke::NONE));
                sp2.add(Shape::line(pts, Stroke::new(1.8, th.accent)));
            }
            p.text(pos2(spark.left() + 12.0, spark.top() + 13.0), Align2::LEFT_CENTER, "THROUGHPUT", f_semi(8.5), th.ink4);
            p.text(pos2(spark.right() - 12.0, spark.top() + 14.0), Align2::RIGHT_CENTER, format!("{}/s", format_size(speed as u64)), f_mono_b(15.0), th.ink1);

            // progress
            let bar = Rect::from_min_size(pos2(card.left() + pad, row.bottom() + 14.0), vec2(w - 2.0 * pad, 10.0));
            well(&p, &th, bar, 5.0);
            let fill = Rect::from_min_size(bar.min, vec2((bar.width() * sf).max(10.0), 10.0));
            let (a2, b2) = th.accent_grad();
            grad_fill(&p, fill, 5.0, a2, b2);
            rim(&p, fill, 5.0, 1.0, Color32::from_white_alpha(120), true, 6.0);
            let band_x = bar.left() + ((anim * 170.0) % (bar.width() + 120.0)) - 60.0;
            let band = fill.intersect(Rect::from_min_size(pos2(band_x, bar.top()), vec2(60.0, 10.0)));
            if band.width() > 1.0 {
                grad_fill(&p, band, 4.0, Color32::from_white_alpha(80), Color32::from_white_alpha(0));
            }
            p.text(pos2(card.left() + pad, bar.bottom() + 15.0), Align2::LEFT_CENTER, format!("{} of {}", format_size((size as f32 * sf) as u64), format_size(size)), f_medium(T_META), th.ink3);
            p.text(pos2(card.right() - pad, bar.bottom() + 15.0), Align2::RIGHT_CENTER, &eta, f_medium(T_META), th.ink3);

            // footer
            let by = card.bottom() - 16.0 - 32.0;
            if self.label_button(ui, Rect::from_min_size(pos2(card.right() - pad - 140.0 - 8.0 - 82.0, by), vec2(82.0, 32.0)), None, "Cancel", "cp-cancel", Btn::Danger).clicked() {
                self.acts.push(Act::CopyDialogClose);
            }
            if self.label_button(ui, Rect::from_min_size(pos2(card.right() - pad - 140.0, by), vec2(140.0, 32.0)), None, "Run in background", "cp-bg", Btn::Accent).clicked() {
                self.copy_dialog = None;
            }
            p.text(pos2(card.left() + pad, by + 16.0), Align2::LEFT_CENTER, "Keep working \u{2014} it continues behind", f_medium(10.5), th.ink4);
        });
    }
}
