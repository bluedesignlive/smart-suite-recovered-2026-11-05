//! Shared skeuomorphic widgets (ring gauge, drive unit with flashing LEDs) and the universal responsive-action
//! planner: labels fall away first, then buttons collapse into the single ••• menu — nothing ever overlaps.
use crate::app::SmartApp;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Painter, Pos2, Rect, Shape, Stroke};
use std::f32::consts::{FRAC_PI_2, TAU};

pub fn hash01(n: u32) -> f32 {
    let mut x = n.wrapping_mul(0x9E37_79B1);
    x ^= x >> 15;
    x = x.wrapping_mul(0x85EB_CA77);
    x ^= x >> 13;
    (x & 0xFFFF) as f32 / 65535.0
}

pub fn arc_points(ctr: Pos2, r: f32, a0: f32, a1: f32) -> Vec<Pos2> {
    let n = (((a1 - a0).abs() * r / 3.0) as usize).clamp(4, 240);
    (0..=n)
        .map(|i| {
            let a = a0 + (a1 - a0) * i as f32 / n as f32;
            ctr + vec2(a.cos() * r, a.sin() * r)
        })
        .collect()
}

/// A bevelled disc: stacked, slightly offset discs from a dark rim up to a bright crown.
pub fn dome(p: &Painter, ctr: Pos2, r: f32, dark: Color32, light: Color32) {
    let n = 10;
    for i in 0..n {
        let k = i as f32 / (n - 1) as f32;
        p.circle_filled(ctr + vec2(0.0, -k * r * 0.10), r * (1.0 - k * 0.10), mix(dark, light, k));
    }
}

/// Skeuomorphic ring gauge: machined bezel, recessed groove, glowing arc with a lit head and a convex centre dome.
pub fn ring_gauge(p: &Painter, th: &Theme, ctr: Pos2, r: f32, frac: f32, label: &str, t: f64) {
    let f = frac.clamp(0.0, 1.0);
    for i in 0..7 {
        let k = i as f32;
        p.circle_filled(ctr + vec2(0.0, r * 0.10 + k * 1.2), r + k * 1.6, alpha(th.shadow, 0.045));
    }
    let (bd, bl) = if th.dark { (c(18, 24, 38, 1.0), c(96, 108, 136, 1.0)) } else { (c(146, 160, 188, 1.0), c(255, 255, 255, 1.0)) };
    dome(p, ctr, r, bd, bl);
    let gr = r * 0.84;
    p.circle_filled(ctr, gr, if th.dark { c(3, 6, 14, 1.0) } else { c(118, 132, 160, 1.0) });
    p.circle_filled(ctr + vec2(0.0, r * 0.025), gr - 1.6, if th.dark { c(10, 15, 27, 1.0) } else { c(204, 214, 234, 1.0) });
    for i in 0..60 {
        let a = -FRAC_PI_2 + i as f32 / 60.0 * TAU;
        let on = (i as f32 / 60.0) < f;
        let (s, co) = a.sin_cos();
        let d = vec2(co, s);
        p.line_segment([ctr + d * (gr * 0.90), ctr + d * (gr * 0.96)], Stroke::new(1.1, if on { alpha(th.accent, 0.55) } else { alpha(th.ink4, 0.20) }));
    }
    if f > 0.004 {
        let ra = gr * 0.68;
        let w = (r * 0.17).max(3.0);
        let pts = arc_points(ctr, ra, -FRAC_PI_2, -FRAC_PI_2 + f * TAU);
        let (a0, a1) = th.accent_grad();
        for (ww, al) in [(w * 2.3, 0.08), (w * 1.7, 0.14), (w * 1.25, 0.26)] {
            p.add(Shape::line(pts.clone(), Stroke::new(ww, alpha(th.accent, al))));
        }
        p.add(Shape::line(pts.clone(), Stroke::new(w, a1)));
        p.add(Shape::line(pts.iter().map(|q| *q + vec2(0.0, -w * 0.2)).collect(), Stroke::new(w * 0.42, alpha(Color32::WHITE, 0.5))));
        p.circle_filled(pts[0], w / 2.0, a1);
        let head = *pts.last().unwrap();
        let pulse = 0.5 + 0.5 * (t * 4.0).sin() as f32;
        p.circle_filled(head, w * (1.15 + 0.3 * pulse), alpha(Color32::WHITE, 0.10));
        p.circle_filled(head, w * 0.56, mix(a0, Color32::WHITE, 0.4));
        p.circle_filled(head, w * 0.24, Color32::WHITE);
    }
    let cr = gr * 0.50;
    p.circle_filled(ctr + vec2(0.0, 2.2), cr + 1.2, alpha(th.shadow, 0.28));
    let (dd, dl) = if th.dark { (c(22, 30, 48, 1.0), c(64, 80, 114, 1.0)) } else { (c(206, 218, 240, 1.0), c(255, 255, 255, 1.0)) };
    dome(p, ctr, cr, dd, dl);
    p.circle_filled(ctr + vec2(-cr * 0.22, -cr * 0.46), cr * 0.34, alpha(Color32::WHITE, 0.10));
    p.circle_filled(ctr + vec2(-cr * 0.22, -cr * 0.50), cr * 0.20, alpha(Color32::WHITE, 0.12));
    p.text(ctr + vec2(0.0, 0.5), Align2::CENTER_CENTER, label, f_bold(r * 0.40), th.ink1);
}

/// Sector map of a drive: cells light up as they are written, the write head flickers.
pub fn sector_map(p: &Painter, th: &Theme, r: Rect, written: f32, activity: f32, t: f64) {
    well(p, th, r, 7.0);
    let (cols, rows) = (((r.width() - 8.0) / 7.0) as usize, ((r.height() - 8.0) / 7.0).max(1.0) as usize);
    let n = cols * rows;
    let head = (written.clamp(0.0, 1.0) * n as f32) as usize;
    let tick = (t * 22.0) as u32;
    for i in 0..n {
        let (cx, cy) = (i % cols, i / cols);
        let cell = Rect::from_min_size(pos2(r.left() + 4.0 + cx as f32 * 7.0, r.top() + 4.0 + cy as f32 * 7.0), vec2(5.0, 5.0));
        let col = if i + 3 >= head && i < head + 3 && activity > 0.05 {
            let fl = hash01(tick.wrapping_add(i as u32 * 7919));
            mix(th.accent, Color32::WHITE, 0.3 + 0.7 * fl)
        } else if i < head {
            alpha(th.accent, 0.55 + 0.25 * hash01(i as u32 * 31))
        } else {
            alpha(th.ink4, 0.16)
        };
        p.rect_filled(cell, cr(1.5), col);
    }
}

/// A tiny desk-drive: machined body, sector window, power LED and an activity LED that flashes with throughput.
pub fn drive_unit(p: &Painter, th: &Theme, r: Rect, activity: f32, written: f32, t: f64, name: &str) {
    elevate(p, th, r, 12.0, 2);
    let (a, b) = if th.dark { (c(54, 64, 86, 1.0), c(24, 30, 44, 1.0)) } else { (c(250, 252, 255, 1.0), c(206, 216, 234, 1.0)) };
    grad_fill(p, r, 12.0, a, b);
    rim(p, r, 12.0, 1.2, Color32::from_white_alpha(if th.dark { 70 } else { 230 }), true, 22.0);
    rim(p, r, 12.0, 1.0, Color32::from_black_alpha(40), false, 8.0);
    let win = Rect::from_min_max(r.min + vec2(10.0, 9.0), pos2(r.right() - 38.0, r.bottom() - 17.0));
    sector_map(p, th, win, written, activity, t);
    p.text(pos2(r.left() + 12.0, r.bottom() - 8.0), Align2::LEFT_CENTER, name, f_semi(8.5), th.ink3);
    let flick = activity * (0.30 + 0.70 * hash01((t * 26.0) as u32));
    let leds = [(r.right() - 24.0, r.top() + 14.0, th.good, 0.9), (r.right() - 24.0, r.top() + 28.0, hex("#ffb23c"), flick)];
    for (x, y, col, lvl) in leds {
        let ctr = pos2(x, y);
        p.circle_filled(ctr, 5.0, alpha(th.shadow, if th.dark { 0.6 } else { 0.25 }));
        p.circle_filled(ctr, 3.6, mix(alpha(th.ink4, 0.5), col, lvl));
        if lvl > 0.15 {
            p.circle_filled(ctr, 8.0, alpha(col, 0.20 * lvl));
            p.circle_filled(ctr - vec2(0.8, 0.9), 1.2, Color32::from_white_alpha((120.0 * lvl) as u8));
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fit {
    Label,
    Icon,
    Hidden,
}

impl SmartApp {
    /// Universal responsive toolbar planner. `items` are labels ordered by priority (most important first).
    /// Labels collapse to icon buttons from the least important end; if it is still too wide, whole buttons move
    /// into the overflow (••• menu). The result never overlaps.
    pub fn plan_actions(&self, p: &Painter, avail: f32, items: &[&str]) -> Vec<Fit> {
        let gap = 8.0;
        let lw: Vec<f32> = items.iter().map(|s| self.tw(p, s, &f_semi(T_BODY)) + 50.0).collect();
        let mut m = vec![Fit::Label; items.len()];
        let total = |m: &Vec<Fit>| -> f32 {
            let (mut t, mut n) = (0.0, 0usize);
            for (i, f) in m.iter().enumerate() {
                match f {
                    Fit::Label => {
                        t += lw[i];
                        n += 1;
                    }
                    Fit::Icon => {
                        t += 34.0;
                        n += 1;
                    }
                    Fit::Hidden => {}
                }
            }
            t + gap * n.saturating_sub(1) as f32
        };
        for i in (0..items.len()).rev() {
            if total(&m) <= avail {
                break;
            }
            m[i] = Fit::Icon;
        }
        for i in (0..items.len()).rev() {
            if total(&m) <= avail {
                break;
            }
            m[i] = Fit::Hidden;
        }
        m
    }
}
