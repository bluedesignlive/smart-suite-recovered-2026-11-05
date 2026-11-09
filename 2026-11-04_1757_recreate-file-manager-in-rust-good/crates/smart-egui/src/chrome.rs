//! Sidebar, context-aware appbar and status bar.
use crate::app::*;
use crate::paint::*;
use crate::theme::*;
use egui::*;
use smart_core::*;

pub fn spaced(p: &Painter, pos: Pos2, s: &str, size: f32, color: Color32, track: f32) {
    let mut x = pos.x;
    for ch in s.chars() {
        let r = text(p, pos2(x, pos.y), Align2::LEFT_CENTER, &ch.to_string(), size, W::Semi, color);
        x += r.width() + track;
    }
}

impl App {
    pub fn open_menu(&mut self, ctx: &Context, at: Option<Pos2>, entries: Vec<MenuEntry>) {
        let pos = at.or_else(|| ctx.input(|i| i.pointer.latest_pos())).unwrap_or(pos2(100.0, 100.0));
        self.menu = Some(MenuState { pos, entries, sub: None, t0: self.t });
    }

    fn traffic(&mut self, ui: &mut Ui, origin: Pos2) {
        let cols = [hex(0xff5f57), hex(0xfebc2e), hex(0x28c840)];
        let hover_any = ui.rect_contains_pointer(Rect::from_min_size(origin - vec2(10.0, 10.0), vec2(72.0, 20.0)));
        for (i, col) in cols.iter().enumerate() {
            let c = origin + vec2(i as f32 * 20.0, 0.0);
            let r = Rect::from_center_size(c, vec2(16.0, 16.0));
            let resp = ui.interact(r, Id::new(("tl", i)), Sense::click());
            let p = ui.painter();
            shadow(p, Rect::from_center_size(c, vec2(12.0, 12.0)), 6.0, 0.0, 1.0, 2.5, 0.0, rgba(0, 0, 0, 0.22));
            p.circle_filled(c, 6.0, lerpc(*col, Color32::WHITE, 0.0));
            fill_poly(p, &rrect_pts(Rect::from_center_size(c - vec2(0.0, 1.5), vec2(8.5, 5.0)), [2.5; 4]), &|q| rgba(255, 255, 255, 0.5 * (1.0 - (q.y - (c.y - 4.0)) / 5.0).clamp(0.0, 1.0)), None);
            if hover_any {
                let g = ["×", "–", "+"][i];
                text(p, c + vec2(0.0, -0.5), Align2::CENTER_CENTER, g, 11.0, W::Semi, rgba(0, 0, 0, 0.55));
            }
            if resp.clicked() {
                let ctx = ui.ctx().clone();
                match i {
                    0 | 1 => {
                        if self.web {
                            self.win_hidden = true;
                        } else {
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx.send_viewport_cmd(if i == 0 { ViewportCommand::Close } else { ViewportCommand::Minimized(true) });
                        }
                    }
                    _ => self.toggle_max(&ctx),
                }
            }
        }
    }

    // ───────────────────────── sidebar ─────────────────────────
    pub fn sidebar_ui(&mut self, ui: &mut Ui, rect: Rect, win: Rect, _rad: f32) {
        if rect.width() < 40.0 {
            return;
        }
        let pal = self.pal;
        let alpha = ((rect.width() - 80.0) / 172.0).clamp(0.0, 1.0);
        let mut ui = ui.new_child(UiBuilder::new().max_rect(Rect::from_min_size(rect.min, vec2(252.0, rect.height()))).layout(Layout::top_down(Align::Min)));
        ui.set_clip_rect(rect.intersect(ui.clip_rect()));
        ui.set_opacity(alpha);
        let ui = &mut ui;
        let x0 = rect.left() + 12.0;
        let w = 228.0;
        self.traffic(ui, pos2(win.left() + 26.0, win.top() + 29.0));
        // workspace
        let ws = Rect::from_min_size(pos2(x0, win.top() + 56.0), vec2(w, 46.0));
        let r = ui.interact(ws, Id::new("ws"), Sense::click());
        if r.hovered() {
            ui.painter().rect_filled(ws, 12.0, with_a(pal.ink1, 0.05));
        }
        {
            let av = Rect::from_min_size(ws.min + vec2(6.0, 9.0), vec2(28.0, 28.0));
            let p = ui.painter();
            shadow(p, av, 9.0, 0.0, 3.0, 7.0, -2.0, with_a(pal.accent, 0.5));
            grad_rect(p, av, 9.0, pal.accent_hover, pal.accent_press);
            reflect(p, av, 9.0, 0.5, 0.3);
            text(p, av.center(), Align2::CENTER_CENTER, "N", 13.5, W::Semi, Color32::WHITE);
            text(p, ws.min + vec2(44.0, 16.0), Align2::LEFT_CENTER, "Naledzi", 13.0, W::Semi, pal.ink1);
            text(p, ws.min + vec2(44.0, 32.0), Align2::LEFT_CENTER, "Personal workspace", 11.0, W::Regular, pal.ink3);
            icon(p, pos2(ws.right() - 14.0, ws.center().y), Icon::ChevronDown, 12.0, pal.ink3, false);
        }
        if r.clicked() {
            let e = self.store.menu_workspace();
            let at = pos2(ws.left(), ws.bottom() + 4.0);
            self.open_menu(ui.ctx(), Some(at), e);
        }

        // bottom storage card + background transfers
        let card_h = 78.0;
        let card = Rect::from_min_size(pos2(x0, rect.bottom() - card_h - 12.0), vec2(w, card_h));
        let bg_jobs: Vec<(u64, String, f32)> = self.store.eng.jobs.iter().filter(|j| j.active() && j.minimized).map(|j| (j.id, format!("{} {}", j.verb(), j.title), j.progress())).collect();
        let list_bottom = card.top() - 8.0 - bg_jobs.len() as f32 * 46.0;
        let list = Rect::from_min_max(pos2(rect.left(), ws.bottom() + 6.0), pos2(rect.left() + 252.0, list_bottom));

        // list
        let mut lui = ui.new_child(UiBuilder::new().max_rect(list).layout(Layout::top_down(Align::Min)));
        lui.set_clip_rect(list.intersect(ui.clip_rect()));
        ScrollArea::vertical().id_salt("side-scroll").auto_shrink([false, false]).show(&mut lui, |ui| {
            ui.set_width(252.0);
            ui.add_space(6.0);
            self.side_label(ui, "FAVORITES");
            let home = self.store.eng.home.clone();
            let favs: Vec<(Icon, String, Route, Path)> = {
                let mut v = vec![(Icon::Home, "Home".to_string(), Route::Folder(home.clone()), home.clone()), (Icon::Clock, "Recents".to_string(), Route::Recents, String::new())];
                for (n, i) in [("Desktop", Icon::Monitor), ("Documents", Icon::Doc), ("Downloads", Icon::Download), ("Pictures", Icon::Image), ("Music", Icon::Music), ("Movies", Icon::Film)] {
                    let p = join(&home, n);
                    if self.store.eng.exists(&p) {
                        v.push((i, n.to_string(), Route::Folder(p.clone()), p));
                    }
                }
                v
            };
            for (i, label, route, path) in favs {
                let active = self.store.route == route;
                let badge = if active { Some(self.store.items.len().to_string()) } else { None };
                let (resp, rr) = self.side_row(ui, i, &label, active, badge, None, false);
                if !path.is_empty() {
                    self.drops.push((rr, path));
                }
                if resp.clicked() {
                    self.act(Action::Go(route.clone()));
                }
                if resp.secondary_clicked() {
                    let e = self.store.menu_for_sidebar_place(&route);
                    self.open_menu(ui.ctx(), None, e);
                }
            }
            ui.add_space(14.0);
            self.side_label(ui, "LOCATIONS");
            let mounts = self.store.eng.mounts.clone();
            for m in mounts {
                if m.id.starts_with("adb") || !m.root.is_empty() {
                    let active = matches!(&self.store.route, Route::Folder(p) if is_within(p, &m.root) && m.root != "/") || (m.root == "/" && matches!(&self.store.route, Route::Folder(p) if *p == "/"));
                    let (resp, rr) = self.side_row(ui, m.icon(), &m.label, active, None, None, !m.mounted);
                    let removable = matches!(m.kind, MountKind::Removable | MountKind::Phone | MountKind::Cloud | MountKind::Network) && m.root != "/";
                    if removable && m.mounted && (resp.hovered() || active) {
                        let er = Rect::from_center_size(pos2(rr.right() - 18.0, rr.center().y), vec2(24.0, 24.0));
                        let eresp = ui.interact(er, Id::new(("eject", &m.id)), Sense::click());
                        let p = ui.painter();
                        if eresp.hovered() {
                            p.rect_filled(er, 8.0, with_a(pal.ink1, 0.08));
                        }
                        icon(p, er.center(), Icon::Eject, 14.0, if eresp.hovered() { pal.accent_ink } else { pal.ink3 }, false);
                        eresp.clone().on_hover_text(if matches!(m.kind, MountKind::Cloud | MountKind::Network) { "Disconnect" } else { "Eject safely" });
                        if eresp.clicked() {
                            self.act(Action::Unmount(m.id.clone()));
                            continue;
                        }
                    } else if !m.mounted {
                        icon(ui.painter(), pos2(rr.right() - 18.0, rr.center().y), Icon::Plug, 14.0, pal.ink4, false);
                    }
                    if m.mounted {
                        self.drops.push((rr, m.root.clone()));
                    }
                    if resp.clicked() {
                        self.act(if m.mounted { Action::GoPath(m.root.clone()) } else { Action::Mount(m.id.clone()) });
                    }
                    if resp.secondary_clicked() {
                        let e = self.store.menu_for_mount(&m.id);
                        self.open_menu(ui.ctx(), None, e);
                    }
                }
            }
            let trash_active = self.store.route == Route::Trash;
            let (resp, rr) = self.side_row(ui, Icon::Trash, "Trash", trash_active, None, None, false);
            if let Some(td) = self.store.eng.trash_dir.clone() {
                self.drops.push((rr, td));
            }
            if resp.clicked() {
                self.act(Action::Go(Route::Trash));
            }
            ui.add_space(14.0);
            self.side_label(ui, "TAGS");
            for t in TAGS.iter() {
                let n = self.store.tag_count(t.id);
                let route = Route::Tag(t.id.into());
                let (resp, _) = self.side_row(ui, Icon::Tag, t.label, self.store.route == route, if n > 0 { Some(n.to_string()) } else { None }, Some(t.rgb), false);
                if resp.clicked() {
                    self.act(Action::Go(route));
                }
            }
            ui.add_space(10.0);
        });

        // background transfers
        let mut y = card.top() - 8.0 - bg_jobs.len() as f32 * 46.0;
        for (id, title, prog) in bg_jobs {
            let r = Rect::from_min_size(pos2(x0, y + 4.0), vec2(w, 40.0));
            y += 46.0;
            let resp = ui.interact(r, Id::new(("bgjob", id)), Sense::click());
            let (top, bot) = card_colors(&pal);
            surface(ui.painter(), r, 14.0, &pal, 1, top, bot);
            ring_progress(ui.painter(), pos2(r.left() + 22.0, r.center().y), 9.0, prog, with_a(pal.ink1, 0.1), pal.accent, 3.0);
            text_fit(ui.painter(), pos2(r.left() + 42.0, r.center().y - 6.0), Align2::LEFT_CENTER, &title, 12.0, W::Medium, pal.ink1, w - 90.0);
            text(ui.painter(), pos2(r.left() + 42.0, r.center().y + 9.0), Align2::LEFT_CENTER, &format!("{:.0}% · tap to expand", prog * 100.0), 10.5, W::Regular, pal.ink3);
            if resp.clicked() {
                if let Some(j) = self.store.eng.job_mut(id) {
                    j.minimized = false;
                }
            }
        }

        // storage card
        let m = self.store.eng.mount_of(&self.store.current_folder()).cloned().or_else(|| self.store.eng.mounts.first().cloned());
        if let Some(m) = m {
            let resp = ui.interact(card, Id::new("storage-card"), Sense::click());
            let (top, bot) = card_colors(&pal);
            surface(ui.painter(), card, 16.0, &pal, if resp.hovered() { 2 } else { 1 }, top, bot);
            let p = ui.painter();
            icon(p, pos2(card.left() + 18.0, card.top() + 20.0), m.icon(), 15.0, pal.ink2, false);
            text_fit(p, pos2(card.left() + 34.0, card.top() + 20.0), Align2::LEFT_CENTER, &m.label, 12.0, W::Semi, pal.ink1, 120.0);
            let used = m.used_frac();
            if m.total > 0 {
                text(p, pos2(card.right() - 14.0, card.top() + 20.0), Align2::RIGHT_CENTER, &format!("{:.0}%", used * 100.0), 11.5, W::Medium, pal.ink2);
            }
            progress_bar(p, Rect::from_min_size(card.min + vec2(14.0, 36.0), vec2(w - 28.0, 5.0)), &pal, used);
            let sub = if m.total > 0 { format!("{} free of {}", fmt_size(m.free), fmt_size(m.total)) } else { "Open Storage & Cleanup".to_string() };
            text(p, card.min + vec2(14.0, 56.0), Align2::LEFT_CENTER, &sub, 11.0, W::Regular, pal.ink3);
            ui.ctx().set_cursor_icon_if_hover(resp.hovered());
            if resp.clicked() {
                self.act(Action::Go(Route::Storage));
            }
        }
    }

    fn side_label(&self, ui: &mut Ui, s: &str) {
        let (r, _) = ui.allocate_exact_size(vec2(252.0, 24.0), Sense::hover());
        spaced(ui.painter(), pos2(r.left() + 26.0, r.center().y + 2.0), s, 10.0, self.pal.ink4, 0.9);
    }

    fn side_row(&mut self, ui: &mut Ui, ic: Icon, label: &str, active: bool, badge: Option<String>, tag: Option<[u8; 3]>, dim: bool) -> (Response, Rect) {
        let pal = self.pal;
        let (full, _) = ui.allocate_exact_size(vec2(252.0, 33.0), Sense::hover());
        let rect = Rect::from_min_size(full.min + vec2(12.0, 0.0), vec2(228.0, 32.0));
        let resp = ui.interact(rect, ui.id().with(("row", label, ic as u8)), Sense::click());
        let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered() || self.drag.is_some() && rect.contains(ui.ctx().pointer_latest_pos().unwrap_or_default()), 0.12);
        let p = ui.painter();
        let dropping = self.drag.is_some() && hov > 0.5;
        if active {
            grad_rect(p, rect, 10.0, with_a(pal.accent, if pal.dark { 0.24 } else { 0.15 }), with_a(pal.accent, if pal.dark { 0.15 } else { 0.09 }));
            reflect(p, rect, 10.0, if pal.dark { 0.1 } else { 0.55 }, 0.0);
        } else if hov > 0.0 {
            p.rect_filled(rect, 10.0, with_a(if pal.dark { Color32::WHITE } else { pal.ink1 }, 0.055 * hov));
        }
        if dropping {
            p.rect_stroke(rect, 10.0, Stroke::new(2.0, pal.accent), StrokeKind::Inside);
        }
        let fg = if active { pal.accent_ink } else if dim { pal.ink4 } else { pal.ink2 };
        if let Some(c) = tag {
            dot(p, pos2(rect.left() + 20.0, rect.center().y), 4.5, Color32::from_rgb(c[0], c[1], c[2]));
        } else {
            icon(p, pos2(rect.left() + 20.0, rect.center().y), ic, 17.0, fg, active || hov > 0.6);
        }
        text_fit(p, pos2(rect.left() + 40.0, rect.center().y), Align2::LEFT_CENTER, label, 13.0, if active { W::Semi } else { W::Medium }, if active { pal.accent_ink } else if dim { pal.ink3 } else { pal.ink1 }, 140.0);
        if let Some(b) = badge {
            text(p, pos2(rect.right() - 12.0, rect.center().y), Align2::RIGHT_CENTER, &b, 11.5, W::Medium, if active { pal.accent_ink } else { pal.ink3 });
        }
        if resp.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }
        (resp, rect)
    }

    // ───────────────────────── appbar ─────────────────────────
    pub fn appbar_ui(&mut self, ui: &mut Ui, bar: Rect, sidebar_hidden: bool, _rad: f32) {
        let pal = self.pal;
        let cy = bar.center().y;
        let mut x = bar.left() + 18.0;
        if sidebar_hidden {
            self.traffic(ui, pos2(bar.left() + 26.0, cy));
            x = bar.left() + 90.0;
        }
        let ctx = ui.ctx().clone();
        let b = |x: f32, w: f32| Rect::from_min_size(pos2(x, cy - 17.0), vec2(w, 34.0));
        if button(ui, &pal, b(x, 34.0), Id::new("tg-sidebar"), Some(Icon::Sidebar), "", Look::Plain, !self.store.sidebar_open, true).on_hover_text("Toggle sidebar  Ctrl+\\").clicked() {
            self.act(Action::ToggleSidebar);
        }
        x += 46.0;
        let (cb, cf) = (self.store.can_back(), self.store.can_forward());
        if button(ui, &pal, b(x, 34.0), Id::new("nav-b"), Some(Icon::Back), "", Look::Ghost, false, cb).on_hover_text("Back").clicked() {
            self.act(Action::Back);
        }
        x += 36.0;
        if button(ui, &pal, b(x, 34.0), Id::new("nav-f"), Some(Icon::Forward), "", Look::Ghost, false, cf).on_hover_text("Forward").clicked() {
            self.act(Action::Forward);
        }
        x += 44.0;

        // right cluster (right-to-left)
        let tools = self.store.tools();
        let has_sel = !self.store.sel.is_empty();
        let ctx_mode = !tools.is_empty() && (has_sel || matches!(self.store.route, Route::Trash | Route::Storage));
        let t_ctx = ctx.animate_bool_with_time(Id::new("ctx-mode"), ctx_mode, 0.22);
        let mut rx = bar.right() - 18.0;
        rx -= 34.0;
        if button(ui, &pal, b(rx, 34.0), Id::new("more"), Some(Icon::More), "", Look::Plain, false, true).on_hover_text("More").clicked() {
            let e = self.store.menu_more();
            self.open_menu(&ctx, Some(pos2(rx + 34.0 - 252.0, bar.bottom() - 6.0)), e);
        }
        rx -= 8.0;
        let compact = bar.width() < 900.0 || ctx_mode;
        if !ctx_mode {
            let label = self.store.sort.label();
            let sw = btn_width(ui.painter(), Some(Icon::Sort), label);
            rx -= sw;
            if button(ui, &pal, b(rx, sw), Id::new("sort"), Some(Icon::Sort), label, Look::Plain, false, true).on_hover_text("Sort & order").clicked() {
                let e = self.store.menu_view_options();
                self.open_menu(&ctx, Some(pos2(rx, bar.bottom() - 6.0)), e);
            }
            rx -= 8.0;
            // segmented grid/list
            let seg = Rect::from_min_size(pos2(rx - 72.0, cy - 17.0), vec2(72.0, 34.0));
            rx -= 80.0;
            let p = ui.painter();
            let (t0, t1) = if pal.dark { (rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.09)) } else { (rgba(20, 30, 48, 0.07), rgba(20, 30, 48, 0.035)) };
            grad_rect(p, seg, 17.0, t0, t1);
            for (i, (ic, mode)) in [(Icon::Grid, ViewMode::Grid), (Icon::List, ViewMode::List)].into_iter().enumerate() {
                let cell = Rect::from_min_size(seg.min + vec2(3.0 + i as f32 * 33.0, 3.0), vec2(33.0, 28.0));
                let active = self.store.view == mode;
                let resp = ui.interact(cell, Id::new(("seg", i)), Sense::click());
                let a = ctx.animate_bool_with_time(resp.id.with("a"), active, 0.16);
                let p = ui.painter();
                if a > 0.01 {
                    shadow(p, cell, 14.0, 0.0, 2.0, 5.0, 0.0, rgba(16, 26, 44, if pal.dark { 0.5 } else { 0.18 } * a));
                    grad_rect(p, cell, 14.0, if pal.dark { rgba(255, 255, 255, 0.2 * a) } else { rgba(255, 255, 255, a) }, if pal.dark { rgba(255, 255, 255, 0.1 * a) } else { rgba(246, 249, 255, a) });
                    reflect(p, cell, 14.0, 0.8 * a, 0.0);
                }
                icon(p, cell.center(), ic, 16.0, if active { pal.accent_ink } else { pal.ink3 }, active);
                if resp.clicked() {
                    self.act(Action::SetView(mode));
                }
                resp.on_hover_text(if i == 0 { "Grid" } else { "List" });
            }
        }
        // Ask Smart
        let ask_w = if compact && !ctx_mode { 34.0 } else { btn_width(ui.painter(), Some(Icon::Sparkle), "Ask Smart") };
        rx -= ask_w;
        let ai = self.store.ai_open;
        if button(ui, &pal, b(rx, ask_w), Id::new("ask"), Some(Icon::Sparkle), if ask_w > 40.0 { "Ask Smart" } else { "" }, if ai { Look::Accent } else { Look::Tint }, false, true).on_hover_text("Ask Smart  Ctrl+J").clicked() {
            self.act(Action::ToggleAi);
        }
        rx -= 8.0;
        // search
        let focused = ctx.memory(|m| m.has_focus(Id::new("search-field")));
        let sw_open = ctx.animate_value_with_time(Id::new("search-w"), if ctx_mode { 34.0 } else if focused || matches!(self.store.route, Route::Search(_)) { 300.0 } else if compact { 170.0 } else { 214.0 }, 0.18);
        let s_rect = Rect::from_min_size(pos2(rx - sw_open, cy - 17.0), vec2(sw_open, 34.0));
        let mid_right = s_rect.left() - 12.0;
        self.search_field(ui, s_rect, ctx_mode);

        // middle: breadcrumb or contextual tools
        let avail = mid_right - x;
        let mut ui2 = ui.new_child(UiBuilder::new().max_rect(Rect::from_min_max(pos2(x, bar.top()), pos2(mid_right, bar.bottom()))));
        ui2.set_clip_rect(Rect::from_min_max(pos2(x - 4.0, bar.top()), pos2(mid_right, bar.bottom())).intersect(ui.clip_rect()));
        if t_ctx < 0.99 {
            ui2.set_opacity((1.0 - t_ctx * 1.4).clamp(0.0, 1.0));
            self.breadcrumb(&mut ui2, x, cy, avail, &tools);
        }
        if t_ctx > 0.01 {
            ui2.set_opacity(t_ctx);
            self.tools_island(&mut ui2, x, cy, avail, tools);
        }
    }

    fn breadcrumb(&mut self, ui: &mut Ui, x: f32, cy: f32, avail: f32, tools: &[Tool]) {
        let pal = self.pal;
        let crumbs = self.store.breadcrumb();
        let mut shown: Vec<(String, Option<Route>)> = crumbs.iter().map(|(l, r)| (l.clone(), Some(r.clone()))).collect();
        if shown.len() > 4 {
            let tail = shown.split_off(shown.len() - 2);
            shown.truncate(1);
            shown.push(("…".into(), None));
            shown.extend(tail);
        }
        let mut cx = x;
        let n = shown.len();
        for (i, (label, route)) in shown.into_iter().enumerate() {
            let last = i == n - 1;
            let used = cx - x;
            let w = text_w(ui.painter(), &label, 13.0, W::Medium).min(if last { (avail - used - 8.0).max(60.0) } else { avail * 0.3 });
            let r = Rect::from_min_size(pos2(cx - 6.0, cy - 15.0), vec2(w + 12.0, 30.0));
            if cx + w > x + avail {
                break;
            }
            let resp = ui.interact(r, Id::new(("crumb", i)), Sense::click());
            let p = ui.painter();
            if resp.hovered() && !last && route.is_some() {
                p.rect_filled(r, 10.0, with_a(pal.ink1, 0.06));
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            text_fit(p, pos2(cx, cy), Align2::LEFT_CENTER, &label, 13.0, if last { W::Semi } else { W::Medium }, if last { pal.ink1 } else { pal.ink3 }, w);
            if let Some(Route::Folder(path)) = &route {
                self.drops.push((r, path.clone()));
            }
            if let (true, Some(rt)) = (resp.clicked() && !last, route) {
                self.act(Action::Go(rt));
            }
            cx += w + 8.0;
            if !last {
                icon(ui.painter(), pos2(cx, cy + 0.5), Icon::ChevronRight, 11.0, pal.ink4, false);
                cx += 14.0;
            }
        }
        // quiet, non-selection tools (Paste / New folder) trail the breadcrumb when there's room
        let mut tx = cx + 14.0;
        for t in tools.iter().filter(|t| t.tone == Tone::Accent) {
            let w = btn_width(ui.painter(), Some(t.icon), &t.label);
            if tx + w < x + avail {
                let r = Rect::from_min_size(pos2(tx, cy - 15.0), vec2(w, 30.0));
                if button(ui, &pal, r, Id::new(("bc-tool", &t.label)), Some(t.icon), &t.label, Look::Accent, false, true).on_hover_text(&t.tip).clicked() {
                    self.act(t.action.clone());
                }
                tx += w + 8.0;
            }
        }
    }

    fn tools_island(&mut self, ui: &mut Ui, x: f32, cy: f32, avail: f32, tools: Vec<Tool>) {
        let pal = self.pal;
        let mut cx = x;
        let n = self.store.sel.len();
        if n > 0 {
            let label = if n == 1 { "1 selected".to_string() } else { format!("{n} selected") };
            let w = btn_width(ui.painter(), Some(Icon::Close), &label);
            let r = Rect::from_min_size(pos2(cx, cy - 17.0), vec2(w, 34.0));
            if button(ui, &pal, r, Id::new("sel-chip"), Some(Icon::Close), &label, Look::Tint, false, true).on_hover_text("Clear selection  Esc").clicked() {
                self.act(Action::SelectNone);
            }
            cx += w + 10.0;
        }
        let full: f32 = tools.iter().map(|t| btn_width(ui.painter(), Some(t.icon), &t.label) + 6.0).sum();
        let labels_all = cx + full < x + avail;
        let mut overflow: Vec<Tool> = vec![];
        for (i, t) in tools.iter().enumerate() {
            let with_label = i == 0 || labels_all;
            let w = if with_label { btn_width(ui.painter(), Some(t.icon), &t.label) } else { 34.0 };
            if cx + w > x + avail - 40.0 && i + 1 < tools.len() {
                overflow.push(t.clone());
                continue;
            }
            let look = match t.tone {
                Tone::Accent => Look::Accent,
                Tone::Danger => Look::Danger,
                Tone::Plain => Look::Plain,
            };
            let r = Rect::from_min_size(pos2(cx, cy - 17.0), vec2(w, 34.0));
            let resp = button(ui, &pal, r, Id::new(("tool", &t.label)), Some(t.icon), if with_label { &t.label } else { "" }, look, false, true);
            let resp = resp.on_hover_text(format!("{}{}", t.label, if t.tip.is_empty() { String::new() } else { format!("  ·  {}", t.tip) }));
            if resp.clicked() {
                self.act(t.action.clone());
            }
            cx += w + 6.0;
        }
        if !overflow.is_empty() {
            let r = Rect::from_min_size(pos2(cx, cy - 17.0), vec2(34.0, 34.0));
            if button(ui, &pal, r, Id::new("tool-more"), Some(Icon::More), "", Look::Plain, false, true).clicked() {
                let entries: Vec<MenuEntry> = overflow.iter().map(|t| MenuEntry::Item(MenuItem { label: t.label.clone(), icon: t.icon, action: t.action.clone(), shortcut: "", danger: t.tone == Tone::Danger, enabled: true, checked: false, swatch: None })).collect();
                let at = pos2(r.left(), r.bottom() + 6.0);
                self.open_menu(&ui.ctx().clone(), Some(at), entries);
            }
        }
    }

    fn search_field(&mut self, ui: &mut Ui, rect: Rect, icon_only: bool) {
        let pal = self.pal;
        let ctx = ui.ctx().clone();
        let id = Id::new("search-field");
        let focused = ctx.memory(|m| m.has_focus(id));
        let p = ui.painter().clone();
        let r = rect.height() / 2.0;
        // recessed field: darker inset with a lit bottom lip
        let hov = ui.rect_contains_pointer(rect);
        if icon_only {
            if button(ui, &pal, rect, Id::new("search-btn"), Some(Icon::Search), "", Look::Plain, false, true).on_hover_text("Search  /").clicked() {
                self.act(Action::FocusSearch);
            }
            return;
        }
        shadow(&p, rect, r, 0.0, 1.0, 2.0, 0.0, if pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(255, 255, 255, 0.9) });
        grad_rect(&p, rect, r, if pal.dark { rgba(0, 0, 0, 0.30) } else { rgba(20, 30, 48, 0.085 + if hov { 0.02 } else { 0.0 }) }, if pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(20, 30, 48, 0.035) });
        if focused {
            p.rect_stroke(rect.expand(1.0), r + 1.0, Stroke::new(2.0, with_a(pal.accent, 0.45)), StrokeKind::Outside);
        }
        icon(&p, pos2(rect.left() + 18.0, rect.center().y), Icon::Search, 16.0, if focused { pal.accent_ink } else { pal.ink3 }, false);
        let mut text_s = self.store.search_text.clone();
        let hint = match &self.store.route {
            Route::Folder(_) => format!("Search {}", self.store.title()),
            _ => "Search files".to_string(),
        };
        let field = Rect::from_min_max(pos2(rect.left() + 34.0, rect.top() + 3.0), pos2(rect.right() - if focused { 38.0 } else { 52.0 }, rect.bottom() - 3.0));
        let resp = ui.put(field, TextEdit::singleline(&mut text_s).id(id).frame(false).hint_text(RichText::new(hint).color(pal.ink3).font(font(13.0, W::Regular))).font(font(13.0, W::Medium)).text_color(pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(2, 0)));
        if self.store.focus_search {
            resp.request_focus();
            self.store.focus_search = false;
        }
        if resp.changed() {
            self.act(Action::Search(text_s.clone()));
        }
        if resp.has_focus() && ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.act(Action::Search(String::new()));
            resp.surrender_focus();
        }
        if resp.has_focus() && ctx.input(|i| i.key_pressed(Key::Enter)) {
            resp.surrender_focus();
        }
        if !focused && text_s.is_empty() {
            let k = if self.mac() { "⌘K" } else { "Ctrl K" };
            let kw = text_w(&p, k, 10.5, W::Medium) + 12.0;
            let kr = Rect::from_center_size(pos2(rect.right() - 12.0 - kw / 2.0, rect.center().y), vec2(kw, 20.0));
            let kresp = ui.interact(kr, Id::new("kbd-k"), Sense::click());
            grad_rect(&p, kr, 7.0, if pal.dark { rgba(255, 255, 255, 0.12) } else { rgba(255, 255, 255, 0.95) }, if pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(244, 248, 254, 0.8) });
            shadow(&p, kr, 7.0, 0.0, 1.0, 2.0, 0.0, rgba(16, 26, 44, if pal.dark { 0.4 } else { 0.1 }));
            text(&p, kr.center(), Align2::CENTER_CENTER, k, 10.5, W::Medium, pal.ink3);
            if kresp.clicked() {
                self.act(Action::Palette(Some(Rail::All)));
            }
        } else if !text_s.is_empty() {
            let cr = Rect::from_center_size(pos2(rect.right() - 18.0, rect.center().y), vec2(22.0, 22.0));
            let cresp = ui.interact(cr, Id::new("search-clear"), Sense::click());
            icon(&p, cr.center(), Icon::Close, 13.0, if cresp.hovered() { pal.ink1 } else { pal.ink3 }, true);
            if cresp.clicked() {
                self.act(Action::Search(String::new()));
            }
        }
    }

    // ───────────────────────── status bar ─────────────────────────
    pub fn status_ui(&mut self, ui: &mut Ui, bar: Rect, rad: f32) {
        let pal = self.pal;
        let p = ui.painter().clone();
        grad_rect_r(&p, bar, [0.0, 0.0, rad, if bar.left() > self.win.left() + 4.0 { 0.0 } else { rad }], with_a(pal.ink1, 0.0), with_a(pal.ink1, if pal.dark { 0.0 } else { 0.025 }));
        let cy = bar.center().y;
        let sel = self.store.selected_entries();
        let label = if sel.is_empty() {
            let n = self.store.items.len();
            if matches!(self.store.route, Route::Storage) { format!("{} items indexed", self.store.eng.index.entries.len()) } else { format!("{}  ·  {}", plural(n, "item", "items"), fmt_size(self.store.total_size())) }
        } else {
            format!("{} selected  ·  {}", sel.len(), fmt_size(sel.iter().map(|e| e.size).sum()))
        };
        text(&p, pos2(bar.left() + 22.0, cy), Align2::LEFT_CENTER, &label, 11.5, W::Medium, pal.ink2);
        let mut rx = bar.right() - 22.0;
        // keyboard hints
        for (k, l) in [("Space", "preview"), ("Ctrl K", "palette")].into_iter().rev() {
            let k = if self.mac() && k == "Ctrl K" { "⌘K" } else { k };
            let lw = text_w(&p, l, 11.0, W::Regular);
            text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, l, 11.0, W::Regular, pal.ink4);
            rx -= lw + 6.0;
            let kw = text_w(&p, k, 10.0, W::Medium) + 10.0;
            let kr = Rect::from_center_size(pos2(rx - kw / 2.0, cy), vec2(kw, 17.0));
            grad_rect(&p, kr, 6.0, if pal.dark { rgba(255, 255, 255, 0.1) } else { rgba(255, 255, 255, 0.95) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(244, 248, 254, 0.7) });
            shadow(&p, kr, 6.0, 0.0, 1.0, 1.5, 0.0, rgba(16, 26, 44, if pal.dark { 0.35 } else { 0.1 }));
            text(&p, kr.center(), Align2::CENTER_CENTER, k, 10.0, W::Medium, pal.ink3);
            rx -= kw + 16.0;
        }
        // index state
        let idx = &self.store.eng.index;
        let (st, col) = if idx.ready { (format!("{} indexed", idx.entries.len()), pal.ink3) } else { (format!("Indexing… {}", idx.entries.len()), pal.accent_ink) };
        if !idx.ready {
            let a = self.t as f32 * 5.0;
            for i in 0..3 {
                let k = ((a + i as f32 * 1.1).sin() * 0.5 + 0.5).clamp(0.2, 1.0);
                dot(&p, pos2(rx - 4.0 - i as f32 * 8.0, cy), 2.2, with_a(pal.accent, k));
            }
            rx -= 34.0;
        }
        text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, &st, 11.0, W::Medium, col);
        rx -= text_w(&p, &st, 11.0, W::Medium) + 18.0;
        if let Some(m) = self.store.eng.mount_of(&self.store.current_folder()) {
            let s = format!("{}  ·  {}", m.label, if m.kind == MountKind::Cloud { "synced" } else { "mounted" });
            icon(&p, pos2(rx - text_w(&p, &s, 11.0, W::Medium) - 12.0, cy), Icon::CheckCircle, 13.0, pal.good, false);
            text(&p, pos2(rx, cy), Align2::RIGHT_CENTER, &s, 11.0, W::Medium, pal.ink3);
        }
    }
}

trait HoverCursor {
    fn set_cursor_icon_if_hover(&self, h: bool);
}
impl HoverCursor for Context {
    fn set_cursor_icon_if_hover(&self, h: bool) {
        if h {
            self.set_cursor_icon(CursorIcon::PointingHand);
        }
    }
}
