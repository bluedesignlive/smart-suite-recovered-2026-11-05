use super::*;

impl SmartApp {
    // ───────────── app window frame ─────────────
    /// Unified macOS-style window: the sidebar runs the full height (traffic lights live in it), the toolbar shares the
    /// surface with the content (no title bar, no divider) and the whole thing resizes from every edge.
    pub(crate) fn draw_app(&mut self, ui: &mut Ui, id: AppId, screen: Rect) {
        let (open, min) = {
            let w = self.app_win(id).unwrap();
            (w.open, w.min)
        };
        if !open || min {
            return;
        }
        let avail = Rect::from_min_max(pos2(screen.left(), screen.top() + TOPBAR_H), pos2(screen.right(), screen.bottom() - 6.0));
        let n_open = self.shell.z.iter().position(|x| *x == id).unwrap_or(0) as f32;
        {
            let w = self.app_win_mut(id);
            if !w.placed {
                let size = vec2(w.rect.width().min(avail.width() - 40.0), w.rect.height().min(avail.height() - DOCK_H * 0.6));
                let c = avail.center() + vec2(40.0 + n_open * 18.0, 10.0 + n_open * 14.0 - 30.0);
                w.rect = Rect::from_center_size(c, size);
                w.placed = true;
            }
        }
        let (rect, born, restore, side_pref) = {
            let w = self.app_win(id).unwrap();
            (w.rect, w.born, w.restore, w.side)
        };
        let active = self.focused_app() == id;
        let th = self.th.clone();
        let ctx = ui.ctx().clone();
        let age = (self.time - born) as f32;
        let pop = if self.lib(|l| l.settings.reduce_motion) { 1.0 } else { spring_out(age) };
        if age < 0.7 {
            ctx.request_repaint();
        }
        let off = vec2(0.0, (1.0 - pop) * 28.0);
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((age * 7.0).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        let live = self.shell.drag_win == Some(id);
        let shown = self.scene_rect(&ctx, id, rect, live).translate(off);
        elevate(&p, &th, shown, R_WIN, if active { 5 } else { 3 });
        // glass body (same recipe as the Files window)
        let (_, sz, blur) = self.active_wall();
        if let Some(b) = blur {
            let suv = cover_uv(sz[0], sz[1], screen);
            let f = |pt: Pos2| pos2(suv.min.x + (pt.x - screen.min.x) / screen.width() * suv.width(), suv.min.y + (pt.y - screen.min.y) / screen.height() * suv.height());
            textured_rrect(&p, shown, R_WIN, b, Rect::from_min_max(f(shown.min), f(shown.max)), Color32::from_white_alpha(if th.dark { 210 } else { 255 }));
        }
        grad_fill(&p, shown, R_WIN, th.window_top, th.window_bottom);
        // responsive: the sidebar folds away on narrow windows and comes back with a spring
        let want_side = side_pref && shown.width() >= 720.0;
        let sf = spring_to(&ctx, ("appside", id), if want_side { 1.0 } else { 0.0 }, 260.0, 26.0).clamp(0.0, 1.0);
        let side_w = SIDEBAR_W * sf;
        let sb = Rect::from_min_size(shown.min, vec2(side_w, shown.height()));
        let stage = Rect::from_min_max(pos2(shown.min.x + side_w, shown.min.y), shown.max);
        if side_w > 1.0 {
            let pp = p.with_clip_rect(sb);
            grad_fill(&pp, Rect::from_min_max(shown.min, pos2(shown.min.x + side_w + R_WIN + 4.0, shown.max.y)), R_WIN, th.sidebar_top, th.sidebar_bottom);
        }
        {
            let pp = p.with_clip_rect(stage);
            grad_fill(&pp, Rect::from_min_max(pos2(stage.min.x - R_WIN - 4.0, shown.min.y), shown.max), R_WIN, th.content_top, th.content_bottom);
        }
        // toolbar drag region (created first so controls on top win)
        let bar = Rect::from_min_size(stage.min, vec2(stage.width(), APPBAR_H));
        let drag = Rect::from_min_size(shown.min, vec2(shown.width(), APPBAR_H));
        let tresp = cui.interact(drag, Id::new(("apptitle", id)), Sense::click_and_drag());
        let mut maximise = tresp.double_clicked();
        if tresp.dragged() {
            self.shell.drag_win = Some(id);
            let ptr = ctx.pointer_latest_pos().unwrap_or(shown.center());
            if restore.is_some() {
                let w = self.app_win_mut(id);
                if let Some(r0) = w.restore.take() {
                    w.rect = Rect::from_min_size(pos2(ptr.x - r0.width() * 0.5, ptr.y - 16.0), r0.size());
                }
            } else {
                let d = tresp.drag_delta();
                let w = self.app_win_mut(id);
                w.rect = w.rect.translate(d);
                w.rect = w.rect.translate(vec2(0.0, (avail.top() - w.rect.top()).max(0.0)));
            }
            self.shell.snap_hint = self.snap_zone(ptr, screen);
        }
        if tresp.drag_stopped() {
            self.shell.drag_win = None;
            if let Some(z) = self.shell.snap_hint.take() {
                let w = self.app_win_mut(id);
                w.restore = Some(w.rect);
                w.rect = z;
                self.shell.snd.play(Snd::Snap);
            }
        }
        // traffic lights: in the sidebar when it is there, in the toolbar when it folds
        let cy = shown.top() + APPBAR_H / 2.0;
        let tl_x = shown.left() + 20.0;
        let (c1, c2, c3) = self.app_traffic(&cui, pos2(tl_x, cy), id, active);
        let mut do_close = c1;
        let mut do_min = c2;
        if c3 {
            maximise = true;
        }
        // sidebar toggle: owned by the toolbar, right after the traffic lights when the sidebar is folded
        let tx = bar.left() + 14.0 + (1.0 - sf) * 78.0 - if sf > 0.5 { 0.0 } else { 0.0 };
        let tx = if side_w > 40.0 { tx.max(bar.left() + 14.0) } else { shown.left() + 20.0 + 78.0 };
        let toggle = Rect::from_min_size(pos2(tx, cy - 17.0), vec2(34.0, 34.0));
        if self.icon_button(&cui, toggle, "sidebar-minimalistic", &format!("tb-side-{:?}", id), if side_pref { Btn::Raised } else { Btn::Tonal }, true).on_hover_text("Toggle sidebar  Ctrl+\\").clicked() {
            self.app_win_mut(id).side = !side_pref;
        }
        let side_body = Rect::from_min_max(pos2(shown.left(), shown.top() + 54.0), pos2(shown.left() + SIDEBAR_W, shown.bottom() - 10.0));
        let content = Rect::from_min_max(pos2(stage.left(), bar.bottom()), shown.max);
        let fr = crate::appkit::Frame { win: shown, side: sb, side_body, bar, stage: content, lead_x: toggle.right() + 12.0, right_x: bar.right() - 14.0, active };
        let mut cp = cui.new_child(egui::UiBuilder::new().max_rect(shown));
        cp.set_clip_rect(shown.intersect(screen));
        match id {
            AppId::Terminal => self.draw_terminal(&mut cp, &fr),
            AppId::Photos => self.draw_photos(&mut cp, &fr),
            AppId::Capture => self.draw_capture(&mut cp, &fr),
            AppId::Settings => self.draw_settings(&mut cp, &fr),
            AppId::Studio => self.draw_studio(&mut cp, &fr),
            AppId::Files => {}
        }
        if self.shell.settings.map(|s| s.0) == Some(id) {
            match id {
                AppId::Terminal => self.term_settings(&mut cp, shown),
                AppId::Photos => self.photos_settings(&mut cp, shown),
                _ => {}
            }
        }
        // edge + corner resize (all eight handles)
        let min_sz = vec2(520.0, 340.0);
        if let Some((dir, d)) = self.resize_handles(&cui, id, shown) {
            if restore.is_none() {
                let w = self.app_win_mut(id);
                if dir.0 > 0.0 {
                    w.rect.max.x = (w.rect.max.x + d.x).max(w.rect.min.x + min_sz.x);
                } else if dir.0 < 0.0 {
                    w.rect.min.x = (w.rect.min.x + d.x).min(w.rect.max.x - min_sz.x);
                }
                if dir.1 > 0.0 {
                    w.rect.max.y = (w.rect.max.y + d.y).max(w.rect.min.y + min_sz.y);
                } else if dir.1 < 0.0 {
                    w.rect.min.y = (w.rect.min.y + d.y).min(w.rect.max.y - min_sz.y).max(avail.top());
                }
            }
        }
        rim(&p, shown, R_WIN, 1.4, if th.dark { c(255, 255, 255, if active { 0.14 } else { 0.07 }) } else { c(255, 255, 255, 0.9) }, true, 42.0);
        {
            // visible resize affordances: a grip in the corner and a glow on the edge you are about to grab
            let pp = ctx.pointer_hover_pos();
            let hov = pp.is_some_and(|q| shown.expand(8.0).contains(q));
            let g = spring_to(&ctx, ("grip", id), if hov && restore.is_none() { 1.0 } else { 0.0 }, 220.0, 26.0).clamp(0.0, 1.0);
            let c0 = shown.right_bottom() - vec2(10.0, 10.0);
            for k in 0..3 {
                let o = 3.5 + k as f32 * 4.0;
                p.line_segment([c0 + vec2(-o, 0.0), c0 + vec2(0.0, -o)], egui::Stroke::new(1.6, alpha(th.ink3, 0.28 + 0.55 * g)));
            }
            if let Some(q) = pp {
                let e = 6.0;
                let sx = (q.x - shown.right()).abs() < e || (q.x - shown.left()).abs() < e;
                let sy = (q.y - shown.bottom()).abs() < e;
                if sy && q.x > shown.left() && q.x < shown.right() {
                    p.line_segment([pos2(shown.left() + 20.0, shown.bottom()), pos2(shown.right() - 20.0, shown.bottom())], egui::Stroke::new(2.5, alpha(th.accent, 0.8)));
                } else if sx && q.y > shown.top() && q.y < shown.bottom() {
                    let x = if (q.x - shown.right()).abs() < e { shown.right() } else { shown.left() };
                    p.line_segment([pos2(x, shown.top() + 20.0), pos2(x, shown.bottom() - 20.0)], egui::Stroke::new(2.5, alpha(th.accent, 0.8)));
                }
            }
        }
        // apply
        if do_close {
            self.shell.snd.play(Snd::Close);
            self.app_win_mut(id).open = false;
            self.shell.settings = None;
            do_close = false;
        }
        if do_min {
            self.shell.snd.play(Snd::Minimize);
            self.app_win_mut(id).min = true;
            do_min = false;
        }
        let _ = (do_close, do_min);
        if maximise {
            let w = self.app_win_mut(id);
            if let Some(r) = w.restore.take() {
                w.rect = r;
            } else {
                w.restore = Some(w.rect);
                w.rect = avail.shrink2(vec2(8.0, 4.0));
            }
        }
    }

}
