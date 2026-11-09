//! Scene-graph window manager: animated geometry, snapping and AI-drivable layouts.
use super::*;
use egui::Context;

impl SmartApp {
    /// Animated geometry of a window node. While the user drags, the node follows almost rigidly; for snaps, tiles and
    /// AI relayouts it glides on springs so you can watch the scene rearrange itself.
    pub(crate) fn scene_rect(&mut self, ctx: &Context, id: AppId, target: Rect, live: bool) -> Rect {
        let reduce = self.lib(|l| l.settings.reduce_motion);
        let (k, d) = if live || reduce { (1100.0, 66.0) } else { (self.shell.wm.stiff, self.shell.wm.damp) };
        let x0 = spring_to(ctx, ("sc", id, 0), target.min.x, k, d);
        let y0 = spring_to(ctx, ("sc", id, 1), target.min.y, k, d);
        let x1 = spring_to(ctx, ("sc", id, 2), target.max.x, k, d);
        let y1 = spring_to(ctx, ("sc", id, 3), target.max.y, k, d);
        Rect::from_min_max(pos2(x0, y0), pos2(x1.max(x0 + 160.0), y1.max(y0 + 120.0)))
    }

    pub(crate) fn avail_rect(&self, screen: Rect) -> Rect {
        Rect::from_min_max(pos2(screen.left() + 6.0, screen.top() + TOPBAR_H + 4.0), pos2(screen.right() - 6.0, screen.bottom() - DOCK_H * 0.62))
    }

    /// Where a window dropped with the pointer at `p` would snap (half / quarter / maximise).
    pub(crate) fn snap_zone(&self, p: Pos2, screen: Rect) -> Option<Rect> {
        if !self.shell.wm.snap {
            return None;
        }
        let a = self.avail_rect(screen);
        let m = 16.0;
        let (l, r, t) = (p.x <= a.left() + m, p.x >= a.right() - m, p.y <= a.top() + 4.0);
        let corner = a.height() * 0.24;
        let (tb, bb) = (p.y - a.top() < corner, a.bottom() - p.y < corner);
        let (hw, hh) = (a.width() / 2.0, a.height() / 2.0);
        let q = |x: f32, y: f32, w: f32, h: f32| Some(Rect::from_min_size(pos2(x, y), vec2(w, h)));
        if l && tb {
            q(a.left(), a.top(), hw, hh)
        } else if l && bb {
            q(a.left(), a.top() + hh, hw, hh)
        } else if r && tb {
            q(a.left() + hw, a.top(), hw, hh)
        } else if r && bb {
            q(a.left() + hw, a.top() + hh, hw, hh)
        } else if l {
            q(a.left(), a.top(), hw, a.height())
        } else if r {
            q(a.left() + hw, a.top(), hw, a.height())
        } else if t {
            Some(a)
        } else {
            None
        }
    }

    pub(crate) fn draw_snap_preview(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx().clone();
        let target = self.shell.snap_hint;
        if let Some(t) = target {
            ctx.data_mut(|d| d.insert_temp(Id::new("snap-r"), t));
        }
        let a = spring_to(&ctx, "snap-a", if target.is_some() { 1.0 } else { 0.0 }, 240.0, 26.0).clamp(0.0, 1.0);
        if a < 0.01 {
            return;
        }
        let Some(t) = ctx.data_mut(|d| d.get_temp::<Rect>(Id::new("snap-r"))) else { return };
        let (x0, y0) = (spring_to(&ctx, ("snp", 0), t.min.x, 260.0, 28.0), spring_to(&ctx, ("snp", 1), t.min.y, 260.0, 28.0));
        let (x1, y1) = (spring_to(&ctx, ("snp", 2), t.max.x, 260.0, 28.0), spring_to(&ctx, ("snp", 3), t.max.y, 260.0, 28.0));
        let r = Rect::from_min_max(pos2(x0, y0), pos2(x1, y1)).shrink(4.0);
        let th = self.th.clone();
        let p = ui.painter();
        shadow(p, r, R_WIN, 8.0, 24.0, -4.0, alpha(th.accent, 0.35 * a));
        grad_fill(p, r, R_WIN, alpha(th.accent, 0.26 * a), alpha(th.accent, 0.10 * a));
        rim(p, r, R_WIN, 1.6, alpha(Color32::WHITE, 0.55 * a), true, 60.0);
        p.rect_stroke(r, cr(R_WIN), egui::Stroke::new(2.0, alpha(th.accent, 0.8 * a)), egui::StrokeKind::Inside);
    }

    pub(crate) fn set_win_rect(&mut self, id: AppId, r: Rect) {
        match id {
            AppId::Files => {
                let s = self.shell.screen;
                let area = Rect::from_min_max(pos2(s.left(), s.top() + TOPBAR_H), pos2(s.right(), s.bottom() - DOCK_H * 0.62));
                self.win_zoom = false;
                self.win_size = Some(r.size());
                self.win_off = r.center() - area.center();
            }
            other => {
                let w = self.app_win_mut(other);
                w.restore = None;
                w.rect = r;
                w.placed = true;
            }
        }
    }

    /// Re-lay-out every visible window. Used by the context menu, shortcuts and the AI (`arrange_windows`).
    pub(crate) fn apply_layout(&mut self, mode: &str) -> String {
        let screen = self.shell.screen;
        let a = self.avail_rect(screen).shrink(8.0);
        let ids: Vec<AppId> = self.shell.z.iter().rev().copied().filter(|i| self.is_visible(*i)).collect();
        if ids.is_empty() {
            return "No windows are open yet".into();
        }
        let n = ids.len();
        let gap = 10.0;
        let rects: Vec<Rect> = match mode {
            "cascade" => (0..n).map(|i| Rect::from_min_size(a.min + vec2(i as f32 * 36.0, i as f32 * 32.0), vec2((a.width() * 0.62).min(940.0), (a.height() * 0.74).min(640.0)))).collect(),
            "focus" => (0..n).map(|_| a).collect(),
            "split" => {
                let hw = (a.width() - gap) / 2.0;
                (0..n)
                    .map(|i| {
                        if i == 0 {
                            Rect::from_min_size(a.min, vec2(hw, a.height()))
                        } else {
                            let rows = (n - 1) as f32;
                            let h = (a.height() - gap * (rows - 1.0)) / rows;
                            Rect::from_min_size(pos2(a.left() + hw + gap, a.top() + (i - 1) as f32 * (h + gap)), vec2(hw, h))
                        }
                    })
                    .collect()
            }
            _ => {
                let cols = (n as f32).sqrt().ceil() as usize;
                let rows = n.div_ceil(cols);
                let (cw, ch) = ((a.width() - gap * (cols - 1) as f32) / cols as f32, (a.height() - gap * (rows - 1) as f32) / rows as f32);
                (0..n).map(|i| Rect::from_min_size(pos2(a.left() + (i % cols) as f32 * (cw + gap), a.top() + (i / cols) as f32 * (ch + gap)), vec2(cw, ch))).collect()
            }
        };
        for (i, (id, r)) in ids.iter().zip(rects).enumerate() {
            if mode == "focus" && i > 0 {
                match id {
                    AppId::Files => self.win_min = true,
                    other => self.app_win_mut(*other).min = true,
                }
                continue;
            }
            let r = Rect::from_min_size(r.min, vec2(r.width().max(420.0), r.height().max(300.0)));
            self.set_win_rect(*id, r);
        }
        self.shell.snd.play(Snd::Snap);
        format!("Arranged {} window{} ({})", n, if n == 1 { "" } else { "s" }, mode)
    }

    pub(crate) fn snap_app(&mut self, id: AppId, zone: &str) -> String {
        let a = self.avail_rect(self.shell.screen);
        let (hw, hh) = (a.width() / 2.0, a.height() / 2.0);
        let r = match zone {
            "left" => Rect::from_min_size(a.min, vec2(hw, a.height())),
            "right" => Rect::from_min_size(pos2(a.left() + hw, a.top()), vec2(hw, a.height())),
            "tl" => Rect::from_min_size(a.min, vec2(hw, hh)),
            "tr" => Rect::from_min_size(pos2(a.left() + hw, a.top()), vec2(hw, hh)),
            "bl" => Rect::from_min_size(pos2(a.left(), a.top() + hh), vec2(hw, hh)),
            "br" => Rect::from_min_size(pos2(a.left() + hw, a.top() + hh), vec2(hw, hh)),
            _ => a,
        };
        self.open_app(id);
        self.set_win_rect(id, r);
        self.shell.snd.play(Snd::Snap);
        format!("Snapped {} {}", id.name(), zone)
    }
}
