//! Depth-based drawing kit: anti-aliased gradient shapes, the elevation ladder, skeuomorphic
//! controls with a top light-reflection edge, typography helpers.
use crate::icons::*;
use crate::theme::*;
use egui::epaint::{Mesh, Shadow, Vertex};
use egui::*;
use smart_core::Icon;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::sync::Arc;

// ───────────────────────── shapes ─────────────────────────
pub fn rrect_pts(rect: Rect, r: [f32; 4]) -> Vec<Pos2> {
    // r = [nw, ne, se, sw]
    let seg = 9;
    let mut pts = Vec::with_capacity(4 * (seg + 1));
    let arcs = [
        (pos2(rect.left() + r[0], rect.top() + r[0]), PI, r[0], rect.left_top()),
        (pos2(rect.right() - r[1], rect.top() + r[1]), PI * 1.5, r[1], rect.right_top()),
        (pos2(rect.right() - r[2], rect.bottom() - r[2]), 0.0, r[2], rect.right_bottom()),
        (pos2(rect.left() + r[3], rect.bottom() - r[3]), FRAC_PI_2, r[3], rect.left_bottom()),
    ];
    for (c, a0, rad, corner) in arcs {
        if rad < 0.5 {
            pts.push(corner);
            continue;
        }
        for i in 0..=seg {
            let a = a0 + FRAC_PI_2 * i as f32 / seg as f32;
            pts.push(c + vec2(a.cos(), a.sin()) * rad);
        }
    }
    pts
}
/// Fill a convex polygon with per-vertex colours (and optional texture), with 1px anti-aliased feathering.
pub fn fill_poly(painter: &Painter, pts: &[Pos2], color_at: &dyn Fn(Pos2) -> Color32, tex: Option<(TextureId, &dyn Fn(Pos2) -> Pos2)>) {
    let n = pts.len();
    if n < 3 {
        return;
    }
    let mut mesh = Mesh::default();
    let center = pts.iter().fold(Vec2::ZERO, |a, p| a + p.to_vec2()) / n as f32;
    let center = center.to_pos2();
    let uv = |p: Pos2| tex.as_ref().map(|t| (t.1)(p)).unwrap_or(pos2(0.0, 0.0));
    if let Some((id, _)) = &tex {
        mesh.texture_id = *id;
    }
    for p in pts {
        mesh.vertices.push(Vertex { pos: *p, uv: uv(*p), color: color_at(*p) });
    }
    for i in 0..n {
        let (a, b) = (pts[(i + n - 1) % n], pts[(i + 1) % n]);
        let t = b - a;
        let nrm = vec2(t.y, -t.x).normalized();
        // outward normal for clockwise (screen) polygons
        let p = pts[i] + nrm * 0.9;
        mesh.vertices.push(Vertex { pos: p, uv: uv(pts[i]), color: Color32::TRANSPARENT });
    }
    mesh.vertices.push(Vertex { pos: center, uv: uv(center), color: color_at(center) });
    let c = (2 * n) as u32;
    for i in 0..n {
        let j = (i + 1) % n;
        mesh.indices.extend([c, i as u32, j as u32]);
        mesh.indices.extend([i as u32, (n + i) as u32, (n + j) as u32]);
        mesh.indices.extend([i as u32, (n + j) as u32, j as u32]);
    }
    painter.add(Shape::mesh(mesh));
}

pub fn lerpc(a: Color32, b: Color32, t: f32) -> Color32 {
    a.lerp_to_gamma(b, t.clamp(0.0, 1.0))
}

pub fn grad_rect(painter: &Painter, rect: Rect, r: f32, top: Color32, bottom: Color32) {
    let h = rect.height().max(1.0);
    fill_poly(painter, &rrect_pts(rect, [r.min(rect.height() / 2.0); 4]), &|p| lerpc(top, bottom, (p.y - rect.top()) / h), None);
}
pub fn grad_rect_r(painter: &Painter, rect: Rect, r: [f32; 4], top: Color32, bottom: Color32) {
    let h = rect.height().max(1.0);
    fill_poly(painter, &rrect_pts(rect, r), &|p| lerpc(top, bottom, (p.y - rect.top()) / h), None);
}

pub fn shadow(painter: &Painter, rect: Rect, r: f32, dx: f32, dy: f32, blur: f32, spread: f32, color: Color32) {
    let rr = rect.expand(spread);
    if rr.width() < 1.0 || rr.height() < 1.0 {
        return;
    }
    painter.add(Shadow { offset: [dx as i8, dy as i8], blur: blur.min(255.0) as u8, spread: 0, color }.as_shape(rr, (r + spread).max(0.0) as u8));
}

/// The 5-step elevation ladder from the design contract.
pub fn elevation(painter: &Painter, rect: Rect, r: f32, level: u8, dark: bool) {
    let k = if dark { 1.7 } else { 1.0 };
    let c = |a: f32| rgba(if dark { 0 } else { 16 }, if dark { 0 } else { 26 }, if dark { 0 } else { 44 }, (a * if dark { k } else { 1.0 }).min(0.95));
    match level {
        1 => shadow(painter, rect, r, 0.0, 1.0, 3.0, 0.0, c(0.09)),
        2 => {
            shadow(painter, rect, r, 0.0, 3.0, 10.0, -2.0, c(0.14));
            shadow(painter, rect, r, 0.0, 1.0, 2.0, 0.0, c(0.06));
        }
        3 => {
            shadow(painter, rect, r, 0.0, 14.0, 34.0, -12.0, c(0.26));
            shadow(painter, rect, r, 0.0, 3.0, 10.0, -2.0, c(0.09));
        }
        4 => {
            shadow(painter, rect, r, 0.0, 28.0, 72.0, -16.0, c(0.40));
            shadow(painter, rect, r, 0.0, 6.0, 18.0, -6.0, c(0.16));
        }
        _ => {
            shadow(painter, rect, r, 0.0, 54.0, 120.0, -26.0, c(0.55));
            shadow(painter, rect, r, 0.0, 18.0, 48.0, -18.0, c(0.30));
        }
    }
}

/// Top-edge light reflection: a 1px bright rim plus a soft gloss that fades over the upper half.
pub fn reflect(painter: &Painter, rect: Rect, r: f32, rim: f32, gloss: f32) {
    let r = r.min(rect.height() / 2.0);
    if gloss > 0.0 {
        let half = Rect::from_min_max(rect.min + vec2(0.5, 0.5), pos2(rect.right() - 0.5, rect.top() + (rect.height() * 0.58).max(r + 1.0)));
        let h = half.height();
        fill_poly(painter, &rrect_pts(half, [r, r, 0.0, 0.0]), &|p| rgba(255, 255, 255, gloss * (1.0 - ((p.y - half.top()) / h)).powf(1.4)), None);
    }
    if rim > 0.0 {
        let rr = rect.shrink(0.5);
        let mut pts = vec![];
        let seg = 8;
        for i in 0..=seg {
            let a = PI + FRAC_PI_2 * (0.35 + 0.65 * i as f32 / seg as f32);
            pts.push(pos2(rr.left() + r, rr.top() + r) + vec2(a.cos(), a.sin()) * (r - 0.5));
        }
        for i in 0..=seg {
            let a = PI * 1.5 + FRAC_PI_2 * (0.65 * i as f32 / seg as f32);
            pts.push(pos2(rr.right() - r, rr.top() + r) + vec2(a.cos(), a.sin()) * (r - 0.5));
        }
        painter.add(Shape::line(pts, Stroke::new(1.0, rgba(255, 255, 255, rim))));
    }
}

// ───────────────────────── text ─────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum W {
    Regular,
    Medium,
    Semi,
}
pub fn font(size: f32, w: W) -> FontId {
    FontId::new(
        size,
        match w {
            W::Regular => FontFamily::Proportional,
            W::Medium => medium(),
            W::Semi => semi(),
        },
    )
}
pub fn text(painter: &Painter, pos: Pos2, anchor: Align2, s: &str, size: f32, w: W, color: Color32) -> Rect {
    painter.text(pos, anchor, s, font(size, w), color)
}
pub fn text_w(painter: &Painter, s: &str, size: f32, w: W) -> f32 {
    painter.layout_no_wrap(s.to_string(), font(size, w), Color32::WHITE).size().x
}
pub fn icon(painter: &Painter, center: Pos2, i: Icon, size: f32, color: Color32, fill: bool) {
    painter.text(center, Align2::CENTER_CENTER, glyph(i, fill), FontId::new(size, icons_family(fill)), color);
}
pub fn ellipsized(painter: &Painter, s: &str, size: f32, w: W, color: Color32, max_w: f32) -> Arc<Galley> {
    let mut job = text::LayoutJob::simple_singleline(s.to_string(), font(size, w), color);
    job.wrap = text::TextWrapping { max_width: max_w.max(8.0) + 1.5, max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
    painter.layout_job(job)
}
pub fn text_fit(painter: &Painter, pos: Pos2, anchor: Align2, s: &str, size: f32, w: W, color: Color32, max_w: f32) -> Rect {
    let g = ellipsized(painter, s, size, w, color, max_w);
    let rect = anchor.anchor_size(pos, g.size());
    painter.galley(rect.min, g, color);
    rect
}
/// Text with highlighted match positions (fuzzy search).
pub fn text_hl(painter: &Painter, pos: Pos2, s: &str, size: f32, color: Color32, hl: Color32, idx: &[u32], max_w: f32) -> Rect {
    let mut job = text::LayoutJob::default();
    job.wrap = text::TextWrapping { max_width: max_w.max(8.0), max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
    for (i, ch) in s.chars().enumerate() {
        let m = idx.contains(&(i as u32));
        job.append(&ch.to_string(), 0.0, text::TextFormat { font_id: font(size, if m { W::Semi } else { W::Medium }), color: if m { hl } else { color }, ..Default::default() });
    }
    let g = painter.layout_job(job);
    let r = Rect::from_min_size(pos, g.size());
    painter.galley(pos, g, color);
    r
}

// ───────────────────────── controls ─────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum Look {
    Plain,
    Accent,
    Tint,
    Danger,
    Ghost,
}

pub fn btn_width(painter: &Painter, i: Option<Icon>, label: &str) -> f32 {
    if label.is_empty() {
        return 34.0;
    }
    let tw = text_w(painter, label, 12.5, W::Medium);
    14.0 + if i.is_some() { 16.0 + 6.0 } else { 0.0 } + tw + 14.0
}

/// A depth-based pill control. Plain buttons have a lit top edge, soft gloss and elevation; accent buttons glow.
pub fn button(ui: &mut Ui, pal: &Pal, rect: Rect, id: Id, ic: Option<Icon>, label: &str, look: Look, active: bool, enabled: bool) -> Response {
    let resp = ui.interact(rect, id, if enabled { Sense::click() } else { Sense::hover() });
    let hov = ui.ctx().animate_bool_with_time(id.with("hv"), resp.hovered() && enabled, 0.12);
    let down = resp.is_pointer_button_down_on() && enabled;
    let p = ui.painter();
    let r = rect.height() / 2.0;
    let rect = if down { rect.shrink(0.8) } else { rect };
    let dark = pal.dark;
    let (top, bot, fg, rim, gloss) = match look {
        Look::Plain => (
            lerpc(if dark { rgba(255, 255, 255, 0.115) } else { rgba(255, 255, 255, 0.97) }, pal.control_hover, hov * 0.0),
            if dark { rgba(255, 255, 255, 0.05 + 0.03 * hov) } else { rgba(244, 248, 254, 0.78 + 0.12 * hov) },
            if active { pal.accent_ink } else { pal.ink1 },
            if dark { 0.16 } else { 0.95 },
            if dark { 0.08 } else { 0.55 },
        ),
        Look::Accent => (lerpc(pal.accent_hover, pal.accent_hover, hov), lerpc(pal.accent, pal.accent_press, 0.25 + 0.2 * hov), if dark { hex(0x08142e) } else { Color32::WHITE }, 0.55, 0.30),
        Look::Tint => (with_a(pal.accent, if dark { 0.22 } else { 0.15 + 0.04 * hov }), with_a(pal.accent, if dark { 0.14 } else { 0.08 + 0.04 * hov }), pal.accent_ink, if dark { 0.2 } else { 0.7 }, if dark { 0.06 } else { 0.35 }),
        Look::Danger => (with_a(pal.danger, 0.17 + 0.05 * hov), with_a(pal.danger, 0.09 + 0.05 * hov), pal.danger, if dark { 0.15 } else { 0.6 }, 0.25),
        Look::Ghost => (with_a(pal.ink1, 0.0 + 0.07 * hov), with_a(pal.ink1, 0.0 + 0.04 * hov), pal.ink2, 0.0, 0.0),
    };
    let fg = if enabled { fg } else { with_a(fg, 0.35) };
    if look != Look::Ghost {
        if !down && enabled {
            match look {
                Look::Accent => shadow(p, rect, r, 0.0, 5.0, 12.0, -3.0, with_a(pal.accent, if dark { 0.42 } else { 0.38 })),
                _ => {
                    shadow(p, rect, r, 0.0, 2.0 + hov, 5.0 + 2.0 * hov, 0.0, rgba(if dark { 0 } else { 16 }, if dark { 0 } else { 26 }, if dark { 0 } else { 44 }, if dark { 0.45 } else { 0.13 }));
                    shadow(p, rect, r, 0.0, 1.0, 1.5, 0.0, rgba(if dark { 0 } else { 16 }, if dark { 0 } else { 26 }, if dark { 0 } else { 44 }, if dark { 0.35 } else { 0.07 }));
                }
            }
        }
        // light mode plain buttons sit on an opaque base so they read as raised from glass
        if matches!(look, Look::Plain | Look::Tint | Look::Danger) && !dark {
            p.rect_filled(rect, r, rgba(255, 255, 255, if look == Look::Tint { 0.75 } else { 0.55 }));
        }
        grad_rect(p, rect, r, top, bot);
        reflect(p, rect, r, rim, gloss);
    } else if hov > 0.0 {
        grad_rect(p, rect, r, top, bot);
    }
    let c = rect.center();
    if label.is_empty() {
        if let Some(i) = ic {
            icon(p, c, i, 17.0, fg, active);
        }
    } else {
        let tw = text_w(p, label, 12.5, W::Medium);
        let iw = if ic.is_some() { 22.0 } else { 0.0 };
        let x0 = c.x - (iw + tw) / 2.0;
        if let Some(i) = ic {
            icon(p, pos2(x0 + 8.0, c.y), i, 16.0, fg, active);
        }
        text(p, pos2(x0 + iw, c.y), Align2::LEFT_CENTER, label, 12.5, W::Medium, fg);
    }
    if enabled {
        ui.ctx().set_cursor_icon_if(resp.hovered(), CursorIcon::PointingHand);
    }
    resp
}

trait CursorExt {
    fn set_cursor_icon_if(&self, c: bool, i: CursorIcon);
}
impl CursorExt for Context {
    fn set_cursor_icon_if(&self, c: bool, i: CursorIcon) {
        if c {
            self.set_cursor_icon(i)
        }
    }
}

/// Raised surface (cards, panels): elevation + vertical tonal gradient + lit top edge.
pub fn surface(p: &Painter, rect: Rect, r: f32, pal: &Pal, level: u8, top: Color32, bottom: Color32) {
    elevation(p, rect, r, level, pal.dark);
    grad_rect(p, rect, r, top, bottom);
    reflect(p, rect, r, if pal.dark { 0.10 } else { 0.8 }, 0.0);
}

pub fn card_colors(pal: &Pal) -> (Color32, Color32) {
    if pal.dark { (rgba(255, 255, 255, 0.075), rgba(255, 255, 255, 0.04)) } else { (rgba(255, 255, 255, 0.98), rgba(251, 253, 255, 0.90)) }
}

pub fn dot(p: &Painter, c: Pos2, r: f32, col: Color32) {
    p.circle_filled(c, r, col);
}

pub fn ring_progress(p: &Painter, c: Pos2, r: f32, frac: f32, track: Color32, fill: Color32, w: f32) {
    p.circle_stroke(c, r, Stroke::new(w, track));
    let n = 40;
    let end = (frac.clamp(0.0, 1.0) * n as f32) as usize;
    let pts: Vec<Pos2> = (0..=end).map(|i| {
        let a = -FRAC_PI_2 + TAU * (i as f32 / n as f32);
        c + vec2(a.cos(), a.sin()) * r
    }).collect();
    if pts.len() > 1 {
        p.add(Shape::line(pts, Stroke::new(w, fill)));
    }
}

pub fn progress_bar(p: &Painter, rect: Rect, pal: &Pal, frac: f32) {
    let r = rect.height() / 2.0;
    p.rect_filled(rect, r, if pal.dark { rgba(255, 255, 255, 0.09) } else { rgba(20, 30, 48, 0.08) });
    let w = (rect.width() * frac.clamp(0.0, 1.0)).max(if frac > 0.0 { rect.height() } else { 0.0 });
    if w > 0.0 {
        let f = Rect::from_min_size(rect.min, vec2(w, rect.height()));
        grad_rect(p, f, r, pal.accent_hover, pal.accent);
        reflect(p, f, r, 0.5, 0.25);
    }
}

pub fn switch_view_rect(avail: Rect) -> Rect {
    avail
}
