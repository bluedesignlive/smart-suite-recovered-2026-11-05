//! Photos and Terminal — the same window anatomy as Files: full-height sidebar with the window
//! controls at its top, a 56 px toolbar, a tonal status bar, and the same rows, cards and buttons.
use crate::app::*;
use crate::paint::*;
use crate::shell::Focus;
use crate::theme::*;
use egui::*;
use smart_core::*;

pub struct FrameRects {
    pub sidebar: Rect,
    pub toolbar: Rect,
    pub content: Rect,
    pub status: Rect,
}

#[derive(Clone, Debug)]
pub enum SideAct {
    AppRow(usize),
    Folder(Path),
    Route(Route),
    Tag(String),
}

impl App {
    /// The ecosystem sidebar. Every app shows the same workspace header, favourites, locations,
    /// tags and storage card; an app only contributes its own section at the top.
    pub fn app_sidebar(&mut self, ui: &mut Ui, fr: &FrameRects, width: f32, section: &str, rows: &[(Icon, String, usize)]) -> Option<SideAct> {
        let pal = self.pal;
        let p = ui.painter().clone();
        let x = fr.sidebar.left() + 12.0;
        let w = width - 24.0;
        let card = Rect::from_min_size(pos2(x, fr.sidebar.bottom() - 74.0), vec2(w, 62.0));
        let bottom_limit = card.top() - 8.0;
        // workspace header — identical to Files
        let ws = Rect::from_min_size(pos2(x, fr.sidebar.top() + 50.0), vec2(w, 44.0));
        let wsr = ui.interact(ws, Id::new("app-ws"), Sense::click());
        if wsr.hovered() {
            p.rect_filled(ws, 11.0, with_a(pal.ink1, 0.05));
        }
        let av = Rect::from_min_size(ws.min + vec2(4.0, 8.0), vec2(28.0, 28.0));
        shadow(&p, av, 9.0, 0.0, 3.0, 7.0, -2.0, with_a(pal.accent, 0.45));
        grad_rect(&p, av, 9.0, pal.accent_hover, pal.accent_press);
        reflect(&p, av, 9.0, 0.5, 0.3);
        text(&p, av.center(), Align2::CENTER_CENTER, "N", 13.0, W::Semi, Color32::WHITE);
        text(&p, ws.min + vec2(42.0, 15.0), Align2::LEFT_CENTER, "Naledzi", 12.5, W::Semi, pal.ink1);
        text(&p, ws.min + vec2(42.0, 30.0), Align2::LEFT_CENTER, "Personal workspace", 10.5, W::Regular, pal.ink3);
        icon(&p, pos2(ws.right() - 12.0, ws.center().y), Icon::ChevronDown, 11.0, pal.ink3, false);
        let mut y = ws.bottom() + 10.0;
        let mut act: Option<SideAct> = None;
        self.app_section(ui, x, y, section);
        y += 23.0;
        for (i, (ic, label, n)) in rows.iter().enumerate() {
            if y + 32.0 > bottom_limit {
                break;
            }
            let (r, _) = self.app_nav(ui, x, w, y, *ic, label, false, (n > &0).then(|| n.to_string()));
            if r.clicked() {
                act = Some(SideAct::AppRow(i));
            }
            y += 33.0;
        }
        y += 8.0;
        // favourites (Files places)
        if y + 24.0 < bottom_limit {
            self.app_section(ui, x, y, "FAVORITES");
            y += 23.0;
            let home = self.store.eng.home.clone();
            let mut favs: Vec<(Icon, String, Path)> = vec![(Icon::Home, "Home".into(), home.clone()), (Icon::Clock, "Recents".into(), String::new())];
            for (n, i) in [("Desktop", Icon::Monitor), ("Documents", Icon::Doc), ("Downloads", Icon::Download), ("Pictures", Icon::Image), ("Music", Icon::Music), ("Movies", Icon::Film)] {
                let pth = join(&home, n);
                if self.store.eng.exists(&pth) {
                    favs.push((i, n.into(), pth));
                }
            }
            for (i, label, pth) in favs {
                if y + 32.0 > bottom_limit {
                    break;
                }
                let (r, _) = self.app_nav(ui, x, w, y, i, &label, false, None);
                if r.clicked() {
                    act = Some(if pth.is_empty() { SideAct::Route(Route::Recents) } else { SideAct::Folder(pth) });
                }
                y += 33.0;
            }
            y += 6.0;
        }
        // locations (mounts)
        if y + 24.0 < bottom_limit {
            self.app_section(ui, x, y, "LOCATIONS");
            y += 23.0;
            let mounts: Vec<(Icon, String, Path, bool)> = self.store.eng.mounts.iter().map(|m| (m.icon(), m.label.clone(), m.root.clone(), m.mounted)).collect();
            for (i, label, root, mounted) in mounts {
                if y + 32.0 > bottom_limit {
                    break;
                }
                let (r, _) = self.app_nav(ui, x, w, y, i, &label, false, if mounted { None } else { Some("ejected".into()) });
                if r.clicked() && mounted {
                    act = Some(SideAct::Folder(root));
                }
                y += 33.0;
            }
            y += 6.0;
        }
        if y + 24.0 < bottom_limit {
            self.app_section(ui, x, y, "TAGS");
            y += 23.0;
            for t in TAGS.iter() {
                if y + 32.0 > bottom_limit {
                    break;
                }
                let n = self.store.tag_count(t.id);
                let (r, rr) = self.app_nav(ui, x, w, y, Icon::Tag, t.label, false, (n > 0).then(|| n.to_string()));
                dot(&p, pos2(rr.left() + 20.0, rr.center().y), 4.0, Color32::from_rgb(t.rgb[0], t.rgb[1], t.rgb[2]));
                if r.clicked() {
                    act = Some(SideAct::Tag(t.id.to_string()));
                }
                y += 33.0;
            }
        }
        // storage card
        let m = self.store.eng.mount_of(&self.store.eng.home).cloned();
        let (frac, line) = m.as_ref().map(|m| (m.used_frac(), format!("{} free of {}", fmt_size(m.free), fmt_size(m.total)))).unwrap_or((0.0, "Local workspace".into()));
        self.app_side_card(ui, card, Icon::Drive, "Smart Drive", &line, frac);
        let _ = (p, m);
        act
    }
}

impl App {
    /// One window anatomy for every app in the ecosystem.
    pub fn app_shell(&mut self, ui: &mut Ui, win: Rect, title: &str, subtitle: &str, ic: Icon, close: Action, sidebar_w: f32, status_left: &str, status_right: &str) -> FrameRects {
        let pal = self.pal;
        let focused = self.focused_app_is_focused();
        let p = ui.painter().clone();
        let rad = 18.0;
        elevation(&p, win, rad, if focused { 4 } else { 3 }, pal.dark);
        if let Some(w) = &self.wall {
            let uv = cover_uv(self.notify_screen, w.size);
            fill_poly(&p, &rrect_pts(win, [rad; 4]), &|_| Color32::WHITE, Some((w.blur.id(), &uv)));
        }
        // opaque base first: stacked windows must never show each other through
        p.rect_filled(win, rad, pal.solid_top);
        grad_rect(&p, win, rad, pal.solid_top, pal.solid_bottom);
        reflect(&p, win, rad, if pal.dark { 0.14 } else { 0.95 }, 0.0);

        let sw = sidebar_w.min(win.width() * 0.45);
        let sidebar = Rect::from_min_size(win.min, vec2(sw, win.height()));
        let main = Rect::from_min_max(pos2(win.left() + sw, win.top()), win.max);
        grad_rect_r(&p, sidebar, [rad, 0.0, 0.0, rad], if pal.dark { hex(0x161f2f) } else { hex(0xe8f0fa) }, if pal.dark { hex(0x111827) } else { hex(0xf0f6fd) });
        fill_poly(&p, &rrect_pts(main, [0.0, rad, rad, 0.0]), &|_| pal.solid_top, None);
        // tonal seam instead of a stroke
        p.rect_filled(Rect::from_min_size(pos2(main.left(), main.top() + rad), vec2(1.0, main.height() - rad * 2.0)), 0.0, with_a(if pal.dark { Color32::WHITE } else { pal.ink1 }, 0.06));

        // window controls — identical placement in every app
        for (i, col) in [hex(0xff5f57), hex(0xfebc2e), hex(0x28c840)].into_iter().enumerate() {
            let c = pos2(win.left() + 26.0 + i as f32 * 20.0, win.top() + 29.0);
            let resp = ui.interact(Rect::from_center_size(c, vec2(17.0, 17.0)), Id::new(("wc", title, i)), Sense::click());
            p.circle_filled(c, 6.0, if focused { col } else { with_a(col, 0.55) });
            if !pal.dark {
                p.circle_filled(c - vec2(0.0, 2.0), 3.2, rgba(255, 255, 255, 0.30));
            }
            if resp.clicked() {
                match i {
                    0 => self.act(close.clone()),
                    1 => self.minimize(self.focused()),
                    _ => {}
                }
            }
        }

        // toolbar
        let toolbar = Rect::from_min_size(pos2(main.left(), main.top()), vec2(main.width(), 56.0));
        let tile = Rect::from_min_size(pos2(toolbar.left() + 18.0, toolbar.center().y - 16.0), vec2(32.0, 32.0));
        let kc = pal.accent;
        shadow(&p, tile, 10.0, 0.0, 3.0, 8.0, -2.0, with_a(kc, 0.34));
        grad_rect(&p, tile, 10.0, with_a(kc, if pal.dark { 0.40 } else { 0.24 }), with_a(kc, if pal.dark { 0.22 } else { 0.12 }));
        reflect(&p, tile, 10.0, if pal.dark { 0.14 } else { 0.65 }, 0.16);
        icon(&p, tile.center(), ic, 17.0, if pal.dark { pal.accent_ink } else { pal.accent }, true);
        text_fit(&p, pos2(tile.right() + 12.0, toolbar.center().y - 7.0), Align2::LEFT_CENTER, title, 13.5, W::Semi, pal.ink1, toolbar.width() * 0.4);
        if !subtitle.is_empty() {
            text_fit(&p, pos2(tile.right() + 12.0, toolbar.center().y + 9.0), Align2::LEFT_CENTER, subtitle, 11.0, W::Regular, pal.ink3, toolbar.width() * 0.4);
        }
        p.rect_filled(Rect::from_min_size(pos2(main.left() + 12.0, toolbar.bottom() - 1.0), vec2(main.width() - 24.0, 1.0)), 0.0, with_a(pal.ink1, 0.055));

        let status = Rect::from_min_max(pos2(main.left(), main.bottom() - 30.0), main.max);
        p.rect_filled(Rect::from_min_size(pos2(main.left() + 12.0, status.top()), vec2(main.width() - 24.0, 1.0)), 0.0, with_a(pal.ink1, 0.05));
        text(&p, pos2(status.left() + 18.0, status.center().y), Align2::LEFT_CENTER, status_left, 11.0, W::Medium, pal.ink2);
        let mut rx = status.right() - 18.0;
        text(&p, pos2(rx, status.center().y), Align2::RIGHT_CENTER, status_right, 10.5, W::Medium, pal.ink4);

        let content = Rect::from_min_max(pos2(main.left(), toolbar.bottom()), pos2(main.right(), status.top()));
        // titlebar drag
        let drag = Rect::from_min_size(win.min, vec2(win.width(), 56.0));
        let dresp = ui.interact(drag, Id::new(("appdrag", title)), Sense::drag());
        if dresp.dragged() {
            let d = dresp.drag_delta();
            match title {
                "Photos" => self.photo_win = self.photo_win.translate(d),
                "Terminal" => self.term_win = self.term_win.translate(d),
                _ => {}
            }
        }
        if ui.input(|i| i.pointer.any_pressed()) {
            if let Some(pos) = ui.input(|i| i.pointer.latest_pos()) {
                if win.contains(pos) {
                    match title {
                        "Photos" => {
                            if self.focused() != Focus::Photos {
                                self.focus(Focus::Photos);
                            }
                        }
                        "Terminal" => {
                            if self.focused() != Focus::Terminal {
                                self.focus(Focus::Terminal);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        FrameRects { sidebar, toolbar, content, status }
    }

    pub fn focused_app_is_focused(&self) -> bool {
        true
    }

    /// Sidebar section label — same typography and insets as Files.
    pub fn app_section(&self, ui: &Ui, x: f32, y: f32, label: &str) {
        crate::chrome::spaced(ui.painter(), pos2(x + 14.0, y + 4.0), label, 10.0, self.pal.ink4, 0.9);
    }

    /// Sidebar row — the exact Files row anatomy (32 px, 10 radius, icon 20 in, text 40 in).
    pub fn app_nav(&mut self, ui: &mut Ui, left: f32, width: f32, y: f32, ic: Icon, label: &str, active: bool, badge: Option<String>) -> (Response, Rect) {
        let pal = self.pal;
        let rect = Rect::from_min_size(pos2(left, y), vec2(width, 32.0));
        let resp = ui.interact(rect, Id::new(("appnav", label)), Sense::click());
        let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.12);
        let p = ui.painter();
        if active {
            grad_rect(p, rect, 10.0, with_a(pal.accent, if pal.dark { 0.24 } else { 0.15 }), with_a(pal.accent, if pal.dark { 0.15 } else { 0.09 }));
            reflect(p, rect, 10.0, if pal.dark { 0.1 } else { 0.55 }, 0.0);
        } else if hov > 0.0 {
            p.rect_filled(rect, 10.0, with_a(if pal.dark { Color32::WHITE } else { pal.ink1 }, 0.055 * hov));
        }
        let fg = if active { pal.accent_ink } else { pal.ink1 };
        icon(p, pos2(rect.left() + 20.0, rect.center().y), ic, 16.0, if active { pal.accent_ink } else { pal.ink2 }, active || hov > 0.6);
        text_fit(p, pos2(rect.left() + 40.0, rect.center().y), Align2::LEFT_CENTER, label, 12.5, if active { W::Semi } else { W::Medium }, fg, width - 64.0);
        if let Some(b) = badge {
            text(p, pos2(rect.right() - 10.0, rect.center().y), Align2::RIGHT_CENTER, &b, 10.5, W::Medium, pal.ink4);
        }
        if resp.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }
        (resp, rect)
    }

    /// The small storage card used at the bottom of every sidebar.
    pub fn app_side_card(&self, ui: &Ui, rect: Rect, ic: Icon, title: &str, line: &str, frac: f32) {
        let pal = self.pal;
        let (t, b) = card_colors(&pal);
        surface(ui.painter(), rect, 14.0, &pal, 1, t, b);
        let p = ui.painter();
        icon(p, pos2(rect.left() + 16.0, rect.top() + 18.0), ic, 14.0, pal.ink2, false);
        text_fit(p, pos2(rect.left() + 32.0, rect.top() + 18.0), Align2::LEFT_CENTER, title, 11.5, W::Semi, pal.ink1, rect.width() - 44.0);
        progress_bar(p, Rect::from_min_size(pos2(rect.left() + 12.0, rect.top() + 33.0), vec2(rect.width() - 24.0, 4.0)), &pal, frac);
        text_fit(p, pos2(rect.left() + 12.0, rect.top() + 50.0), Align2::LEFT_CENTER, line, 10.0, W::Regular, pal.ink3, rect.width() - 24.0);
    }

    /// Toolbar action, anchored from the right edge; measured like every other control.
    pub fn toolbar_action(&mut self, ui: &mut Ui, toolbar: Rect, right: &mut f32, id: &str, label: &str, ic: Icon, look: Look) -> bool {
        let w = btn_width(ui.painter(), Some(ic), label);
        *right -= w;
        let r = Rect::from_min_size(pos2(*right, toolbar.center().y - 15.0), vec2(w, 30.0));
        let clicked = button(ui, &self.pal, r, Id::new(id), Some(ic), label, look, false, true).clicked();
        *right -= 6.0;
        clicked
    }

    // ───────────────────────── Photos ─────────────────────────
    pub fn photos_window(&mut self, ui: &mut Ui, win: Rect) {
        let pal = self.pal;
        let entries: Vec<search::IndexEntry> = self.store.eng.index.entries.iter().filter(|e| e.kind == Kind::Image && !e.is_dir).cloned().collect();
        let library_size: u64 = entries.iter().map(|e| e.size).sum();
        let fav_count = entries.iter().filter(|e| self.favs.contains(&e.path)).count();
        let recent = entries.iter().filter(|e| e.modified > now_ts() - 7 * 86400).count();
        let section_title = match self.photo_album {
            0 => "All photos",
            1 => "Favourites",
            2 => "Recent",
            i => TAGS.get(i as usize - 3).map(|t| t.label).unwrap_or("Album"),
        };
        let d = self.store.eng.mount_of(&self.store.eng.home).cloned();
        let status_left = format!("{} · {}", plural(entries.len(), "photo", "photos"), fmt_size(library_size));
        let status_right = format!("{}  ·  {}", if d.as_ref().map(|m| m.total > 0).unwrap_or(false) { fmt_size(d.as_ref().unwrap().free) } else { "—".into() }, "Photos 1.0");
        let fr = self.app_shell(ui, win, "Photos", "Library · on-device", Icon::Image, Action::ClosePhotos, 236.0, &status_left, &status_right);
        // one shared ecosystem sidebar
        let rows = vec![
            (Icon::Image, "All photos".to_string(), entries.len()),
            (Icon::Star, "Favourites".to_string(), fav_count),
            (Icon::Clock, "Recent".to_string(), recent),
        ];
        if let Some(act) = self.app_sidebar(ui, &fr, 236.0, "LIBRARY", &rows) {
            match act {
                SideAct::AppRow(i) => {
                    self.photo_album = i as u8;
                    self.photo_sel = None;
                }
                SideAct::Tag(t) => {
                    self.photo_album = TAGS.iter().position(|x| x.id == t).map(|i| (i + 3) as u8).unwrap_or(0);
                    self.photo_sel = None;
                }
                SideAct::Folder(_) | SideAct::Route(_) => {
                    self.photo_album = 0;
                    self.photo_sel = None;
                }
            }
        }
        // toolbar actions
        let mut right = fr.toolbar.right() - 18.0;
        if self.toolbar_action(ui, fr.toolbar, &mut right, "ph-files", "Open in Files", Icon::FolderOpen, Look::Plain) {
            self.act(Action::Go(Route::Photos));
            self.focus(Focus::Files);
            self.win_hidden = false;
        }
        if let Some(sel) = self.photo_sel.clone() {
            if self.toolbar_action(ui, fr.toolbar, &mut right, "ph-info", "Info", Icon::Info, Look::Plain) {
                self.act(Action::Properties(Some(sel)));
            }
            if self.toolbar_action(ui, fr.toolbar, &mut right, "ph-view", "View", Icon::Eye, Look::Accent) {
                self.photo_viewer = true;
            }
        }
        // content header + grid
        let mut shown: Vec<&search::IndexEntry> = entries
            .iter()
            .filter(|e| match self.photo_album {
                0 => true,
                1 => self.favs.contains(&e.path),
                2 => e.modified > now_ts() - 7 * 86400,
                i => TAGS.get(i as usize - 3).map(|t| self.store.eng.tags.get(&e.path).map(|x| x == t.id).unwrap_or(false)).unwrap_or(false),
            })
            .collect();
        shown.sort_by_key(|e| std::cmp::Reverse(e.modified));
        let head = Rect::from_min_size(fr.content.min, vec2(fr.content.width(), 58.0));
        text(&ui.painter(), pos2(head.left() + 22.0, head.top() + 24.0), Align2::LEFT_CENTER, section_title, 20.0, W::Semi, pal.ink1);
        text(&ui.painter(), pos2(head.left() + 22.0, head.top() + 44.0), Align2::LEFT_CENTER, &format!("{}  ·  {}", plural(shown.len(), "photo", "photos"), fmt_size(shown.iter().map(|e| e.size).sum::<u64>())), 11.5, W::Medium, pal.ink3);
        let grid = Rect::from_min_max(pos2(fr.content.left(), head.bottom()), fr.content.max);
        ui.painter().rect_filled(grid, 0.0, pal.solid_top);
        if shown.is_empty() {
            text(ui.painter(), grid.center(), Align2::CENTER_CENTER, "No photos here yet", 13.0, W::Medium, pal.ink3);
        } else {
            let cols = ((grid.width() - 40.0) / 190.0).floor().max(2.0) as usize;
            let gap = 14.0;
            let tw = (grid.width() - 40.0 - gap * (cols as f32 - 1.0)) / cols as f32;
            let th = tw * 0.68;
            let rows = shown.len().div_ceil(cols);
            let total_h = rows as f32 * (th + 46.0 + gap);
            let mut gui = ui.new_child(UiBuilder::new().max_rect(grid).layout(Layout::top_down(Align::Min)));
            gui.set_clip_rect(grid.intersect(ui.clip_rect()));
            ScrollArea::vertical().id_salt("photos-grid").auto_shrink([false, false]).show(&mut gui, |ui| {
                let (area, _) = ui.allocate_exact_size(vec2(grid.width(), total_h.max(grid.height())), Sense::hover());
                let o = area.min + vec2(20.0, 4.0);
                for (i, e) in shown.iter().enumerate() {
                    let media = Rect::from_min_size(o + vec2((i % cols) as f32 * (tw + gap), (i / cols) as f32 * (th + 46.0 + gap)), vec2(tw, th));
                    let resp = ui.interact(media, Id::new(("ph", &e.path)), Sense::click_and_drag());
                    let sel = self.photo_sel.as_ref() == Some(&e.path);
                    let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.13);
                    let rr = media.translate(vec2(0.0, -2.0 * hov));
                    let p = ui.painter().clone();
                    elevation(&p, rr, 14.0, if hov > 0.5 || sel { 3 } else { 2 }, pal.dark);
                    if let Some(t) = self.thumbs.get(ui.ctx(), &self.store, &e.path, 420) {
                        let uv = cover_uv(rr, t.size_vec2());
                        fill_poly(&p, &rrect_pts(rr, [14.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
                    } else {
                        grad_rect(&p, rr, 14.0, with_a(pal.kind_color(Kind::Image), 0.22), with_a(pal.kind_color(Kind::Image), 0.08));
                        icon(&p, rr.center(), Icon::Image, 28.0, pal.ink3, false);
                    }
                    reflect(&p, rr, 14.0, if pal.dark { 0.12 } else { 0.5 }, 0.0);
                    if sel {
                        p.rect_filled(rr, 14.0, with_a(pal.accent, 0.14));
                        p.rect_stroke(rr.expand(2.0), 16.0, Stroke::new(2.5, pal.accent), StrokeKind::Outside);
                    }
                    if sel || hov > 0.35 {
                        let c = rr.left_top() + vec2(20.0, 20.0);
                        if !pal.dark {
                            p.circle_filled(c, 11.5, rgba(255, 255, 255, 0.86));
                        }
                        crate::content::checkbox(&p, c, if sel { 1 } else { 0 }, &pal);
                    }
                    if self.favs.contains(&e.path) {
                        icon(&p, rr.right_top() + vec2(-20.0, 20.0), Icon::Star, 15.0, Color32::WHITE, true);
                    }
                    crate::paint::text_fit(&p, pos2(media.left() + 2.0, media.bottom() + 17.0), Align2::LEFT_CENTER, &e.name, 12.5, W::Semi, pal.ink1, tw - 6.0);
                    crate::paint::text(&p, pos2(media.left() + 2.0, media.bottom() + 33.0), Align2::LEFT_CENTER, &format!("{}  ·  {}", fmt_date(e.modified), fmt_size(e.size)), 10.5, W::Regular, pal.ink3);
                    if resp.clicked() {
                        self.photo_sel = Some(e.path.clone());
                    }
                    if resp.double_clicked() {
                        self.photo_sel = Some(e.path.clone());
                        self.photo_viewer = true;
                    }
                }
            });
        }
        if self.photo_viewer {
            if let Some(path) = self.photo_sel.clone() {
                let ov = fr.content;
                let p = ui.painter().clone();
                p.rect_filled(ov, 0.0, if pal.dark { hex(0x0a0f18) } else { hex(0xfbfdff) });
                let bx = ov.shrink2(vec2(20.0, 16.0));
                let stage = Rect::from_min_max(pos2(bx.left(), bx.top() + 30.0), pos2(bx.right(), bx.bottom() - 46.0));
                let meta = self.store.eng.stat(&path).ok();
                if let Some(t) = self.thumbs.get(ui.ctx(), &self.store, &path, 1600) {
                    let sz = t.size_vec2();
                    let k = ((stage.width() - 20.0) / sz.x).min((stage.height() - 20.0) / sz.y).max(0.1);
                    let ir = Rect::from_center_size(stage.center(), sz * k);
                    elevation(&p, ir, 10.0, 3, pal.dark);
                    let mut m = epaint::Mesh::with_texture(t.id());
                    m.add_rect_with_uv(ir, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                    p.add(Shape::mesh(m));
                }
                text_fit(&p, pos2(bx.left() + 2.0, bx.top() + 10.0), Align2::LEFT_CENTER, name_of(&path), 13.5, W::Semi, pal.ink1, bx.width() - 220.0);
                if let Some(m) = &meta {
                    text(&p, pos2(bx.right() - 2.0, bx.top() + 10.0), Align2::RIGHT_CENTER, &format!("{}  ·  {}", fmt_size(m.size), fmt_date(m.modified)), 10.5, W::Regular, pal.ink3);
                }
                let mut x = bx.left();
                let actions: [(&str, Icon, Look, Action); 4] = [
                    ("Open", Icon::OpenExt, Look::Accent, Action::Open(path.clone())),
                    ("Show in Files", Icon::FolderOpen, Look::Plain, Action::GoPath(parent(&path))),
                    ("Info", Icon::Info, Look::Plain, Action::Properties(Some(path.clone()))),
                    ("Trash", Icon::Trash, Look::Danger, Action::Trash),
                ];
                for (label, ic, look, act) in actions {
                    let w = btn_width(ui.painter(), Some(ic), label);
                    if button(ui, &pal, Rect::from_min_size(pos2(x, bx.bottom() - 34.0), vec2(w, 30.0)), Id::new(("phv", label)), Some(ic), label, look, false, true).clicked() {
                        if label == "Trash" || label == "Show in Files" {
                            self.store.sel = vec![path.clone()];
                        }
                        self.photo_viewer = false;
                        self.act(act);
                    }
                    x += w + 6.0;
                }
                let fav = self.favs.contains(&path);
                let fw = btn_width(ui.painter(), Some(Icon::Star), "Favourite");
                if button(ui, &pal, Rect::from_min_size(pos2(bx.right() - fw - 38.0, bx.bottom() - 34.0), vec2(fw, 30.0)), Id::new("phv-fav"), Some(Icon::Star), if fav { "Favourited" } else { "Favourite" }, Look::Tint, fav, true).clicked() {
                    if fav {
                        self.favs.remove(&path);
                    } else {
                        self.favs.insert(path.clone());
                    }
                }
                if button(ui, &pal, Rect::from_min_size(pos2(bx.right() - 30.0, bx.bottom() - 34.0), vec2(30.0, 30.0)), Id::new("phv-close"), Some(Icon::Close), "", Look::Plain, false, true).clicked() {
                    self.photo_viewer = false;
                }
            } else {
                self.photo_viewer = false;
            }
        }
        if ui.ctx().input(|i| i.key_pressed(Key::Escape)) && self.photo_viewer {
            self.photo_viewer = false;
        }
    }

    // ───────────────────────── Terminal ─────────────────────────
    pub fn terminal_window(&mut self, ui: &mut Ui, win: Rect) {
        let pal = self.pal;
        let places: [(&str, Icon); 5] = [("Home", Icon::Home), ("Desktop", Icon::Monitor), ("Documents", Icon::Doc), ("Downloads", Icon::Download), ("Pictures", Icon::Image)];
        let home = self.store.eng.home.clone();
        let cwd_name = if self.term.cwd == home { "Home".to_string() } else { name_of(&self.term.cwd).to_string() };
        let host = if self.store.eng.native { "Native host" } else { "WebAssembly host" };
        let mounts = self.store.eng.mounts.iter().filter(|m| m.mounted).count();
        let cwd_full = self.term.cwd.clone();
        let fr = self.app_shell(ui, win, "Terminal", &cwd_full, Icon::Terminal, Action::CloseTerminal, 236.0, &format!("$ {}", cwd_name), &format!("{}  ·  {} mounts  ·  zsh subset", host, mounts));
        // the same sidebar every app uses
        let rows = vec![
            (Icon::Terminal, "List folder".to_string(), 0),
            (Icon::Layers, "Disk usage".to_string(), 0),
            (Icon::Drive, "Mounted disks".to_string(), 0),
            (Icon::Broom, "Clear buffer".to_string(), 0),
        ];
        if let Some(act) = self.app_sidebar(ui, &fr, 236.0, "SHELL", &rows) {
            match act {
                SideAct::AppRow(0) => self.term_command("ls -l"),
                SideAct::AppRow(1) => self.term_command("du"),
                SideAct::AppRow(2) => self.term_command("df"),
                SideAct::AppRow(3) => self.term.lines.clear(),
                SideAct::Folder(p) => self.term.cwd = p,
                _ => {}
            }
        }
        // toolbar
        let mut right = fr.toolbar.right() - 18.0;
        if self.toolbar_action(ui, fr.toolbar, &mut right, "term-clear", "Clear", Icon::Broom, Look::Plain) {
            self.term.lines.clear();
        }
        if self.toolbar_action(ui, fr.toolbar, &mut right, "term-terminal", "Host shell", Icon::Terminal, Look::Plain) {
            if let Err(e) = self.store.eng.terminal_at(&self.term.cwd) {
                self.store.toast(e, Icon::Warning, false, None);
            }
        }
        if self.toolbar_action(ui, fr.toolbar, &mut right, "term-reveal", "Reveal in Files", Icon::FolderOpen, Look::Plain) {
            self.act(Action::GoPath(self.term.cwd.clone()));
            self.focus(Focus::Files);
            self.win_hidden = false;
        }
        // content
        let p = ui.painter().clone();
        let pad = 20.0;
        let input_h = 46.0;
        p.rect_filled(fr.content, 0.0, if pal.dark { hex(0x0d1420) } else { hex(0xfbfdff) });
        let out = Rect::from_min_max(pos2(fr.content.left() + pad, fr.content.top() + 10.0), pos2(fr.content.right() - pad, fr.content.bottom() - input_h - 8.0));
        let mut lui = ui.new_child(UiBuilder::new().max_rect(out).layout(Layout::top_down(Align::Min)));
        lui.set_clip_rect(out.intersect(ui.clip_rect()));
        let lines = self.term.lines.clone();
        ScrollArea::vertical().id_salt("term-app").auto_shrink([false, false]).stick_to_bottom(true).show(&mut lui, |ui| {
            ui.set_width(out.width());
            for (is_cmd, l) in &lines {
                let col = if *is_cmd { pal.accent_ink } else if pal.dark { hex(0xd7e2f4) } else { hex(0x25324a) };
                let g = ui.painter().layout(l.clone(), FontId::new(11.5, FontFamily::Monospace), col, (out.width() - 4.0).max(120.0));
                let (rr, _) = ui.allocate_exact_size(vec2(out.width() - 4.0, g.size().y + 2.0), Sense::hover());
                ui.painter().galley(rr.min, g, col);
            }
        });
        let prompt = Rect::from_min_size(pos2(fr.content.left() + pad, fr.content.bottom() - input_h), vec2(fr.content.width() - pad * 2.0, 34.0));
        grad_rect(&p, prompt, 9.0, if pal.dark { rgba(0, 0, 0, 0.3) } else { rgba(20, 30, 48, 0.07) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(20, 30, 48, 0.03) });
        let label = if self.term.cwd == home { "~".to_string() } else { cwd_name.clone() };
        text(&p, pos2(prompt.left() + 11.0, prompt.center().y), Align2::LEFT_CENTER, &format!("{label} $"), 11.5, W::Semi, pal.good);
        let pw = text_w(&p, &format!("{label} $"), 11.5, W::Semi) + 20.0;
        let mut input = std::mem::take(&mut self.term.input);
        let resp = ui.put(
            Rect::from_min_max(prompt.min + vec2(pw, 2.0), prompt.max - vec2(8.0, 2.0)),
            TextEdit::singleline(&mut input).frame(false).hint_text(RichText::new("type a command · help").color(pal.ink4).font(FontId::new(11.5, FontFamily::Monospace))).font(FontId::new(11.5, FontFamily::Monospace)).text_color(pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(4, 0)),
        );
        if self.focused() == Focus::Terminal {
            resp.request_focus();
        }
        let events = ui.ctx().input(|i| i.events.clone());
        let mut run = None;
        for e in events {
            match e {
                Event::Key { key: Key::Enter, pressed: true, .. } => run = Some(input.clone()),
                Event::Key { key: Key::ArrowUp, pressed: true, .. } => {
                    if let Some(h) = self.term.history.last() {
                        input = h.clone();
                    }
                }
                Event::Text(t) if t == "\t" => {
                    let dir = self.term.cwd.clone();
                    if let Ok(list) = self.store.eng.list(&dir) {
                        if let Some(h) = list.iter().find(|e| e.name.to_lowercase().starts_with(&input.to_lowercase()) && e.name != input) {
                            input = if h.is_dir { format!("{}/", h.name) } else { h.name.clone() };
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(line) = run {
            if !line.trim().is_empty() {
                self.term.history.push(line.clone());
                self.term_command(&line);
            }
            input.clear();
            resp.request_focus();
        }
        self.term.input = input;
    }

    fn term_command(&mut self, line: &str) {
        let line = line.trim().to_string();
        let cwd = self.term.cwd.clone();
        let home = self.store.eng.home.clone();
        self.term.lines.push((true, format!("{} $ {}", if cwd == home { "~".into() } else { name_of(&cwd).to_string() }, line)));
        if self.term.lines.len() > 500 {
            let n = self.term.lines.len() - 500;
            self.term.lines.drain(0..n);
        }
        let resolve = |arg: &str| -> Path {
            if arg.is_empty() { cwd.clone() } else if arg == "~" { home.clone() } else if arg == ".." { parent(&cwd) } else if arg.starts_with('/') { arg.to_string() } else { join(&cwd, arg) }
        };
        let words: Vec<String> = line.split_whitespace().map(|s| s.replace(['\'', '"'], "")).collect();
        let cmd = words.first().map(|s| s.to_lowercase()).unwrap_or_default();
        let arg = words.get(1).cloned().unwrap_or_default();
        let mut out: Vec<String> = vec![];
        match cmd.as_str() {
            "" => {}
            "help" => out.extend([
                "nav      pwd · cd <dir> · ls [-l] [dir] · find <name>".into(),
                "files    cat · touch · mkdir · rm · mv · cp · open".into(),
                "archive  zip <path> · unzip <archive>".into(),
                "system   du · df · date · whoami · uname · clear".into(),
                "↑ history · Tab completes names".into(),
            ]),
            "pwd" => out.push(cwd.clone()),
            "clear" => {
                self.term.lines.clear();
                return;
            }
            "date" => out.push(fmt_full_date(now_ts())),
            "whoami" => out.push("naledzi".into()),
            "uname" => out.push(format!("Smart Terminal 0.1 · {}", if self.store.eng.native { "native host" } else { "wasm host" })),
            "ls" | "ll" => {
                let long = cmd == "ll" || arg == "-l";
                let target = if arg.starts_with('-') || arg.is_empty() { resolve("") } else { resolve(&arg) };
                match self.store.eng.list(&target) {
                    Ok(mut l) => {
                        l.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));
                        if long {
                            for e in l.iter().take(60) {
                                out.push(format!("{:<34} {:>10}  {}", if e.is_dir { format!("{}/", e.name) } else { e.name.clone() }, if e.is_dir { "—".into() } else { fmt_size(e.size) }, fmt_date(e.modified)));
                            }
                        } else {
                            out.push(l.iter().map(|e| if e.is_dir { format!("{}/", e.name) } else { e.name.clone() }).collect::<Vec<_>>().join("   "));
                        }
                    }
                    Err(e) => out.push(e),
                }
            }
            "find" => {
                if arg.is_empty() {
                    out.push("usage: find <name>".into());
                } else {
                    let hits: Vec<String> = self.store.eng.index.entries.iter().filter(|e| e.name.to_lowercase().contains(&arg.to_lowercase()) && is_within(&e.path, &cwd)).take(20).map(|e| e.path.clone()).collect();
                    if hits.is_empty() { out.push(format!("no matches for “{arg}”")) } else { out.extend(hits) }
                }
            }
            "cd" => match self.store.eng.stat(&resolve(&arg)) {
                Ok(e) if e.is_dir => self.term.cwd = e.path,
                Ok(_) => out.push("not a directory".into()),
                Err(e) => out.push(e),
            },
            "cat" => match self.store.eng.read_head(&resolve(&arg), 4000) {
                Ok(b) => out.push(String::from_utf8_lossy(&b).replace('\0', "·")),
                Err(e) => out.push(e),
            },
            "touch" => match self.store.eng.create_file(&resolve(&arg), b"") {
                Ok(()) => out.push(format!("created {arg}")),
                Err(e) => out.push(e),
            },
            "mkdir" => match self.store.eng.mkdir(&resolve(&arg)) {
                Ok(()) => out.push(format!("created {arg}/")),
                Err(e) => out.push(e),
            },
            "rm" => {
                let p = resolve(&arg);
                let r = if self.store.eng.trash(&p).is_ok() { self.store.refresh(); Ok(()) } else { self.store.eng.delete_permanent(&p) };
                match r {
                    Ok(()) => out.push(format!("moved “{arg}” to Trash")),
                    Err(e) => out.push(e),
                }
            }
            "mv" | "cp" => {
                let (a, b) = (words.get(1).cloned().unwrap_or_default(), words.get(2).cloned().unwrap_or_default());
                if a.is_empty() || b.is_empty() {
                    out.push(format!("usage: {cmd} <source> <destination>"));
                } else {
                    let dest = resolve(&b);
                    let dest = if self.store.eng.stat(&dest).map(|e| e.is_dir).unwrap_or(false) { dest } else { parent(&dest) };
                    match self.store.eng.start_transfer(&[resolve(&a)], &dest, cmd == "mv") {
                        Ok(_) => out.push(format!("{cmd}: {a} → {b}")),
                        Err(e) => out.push(e),
                    }
                }
            }
            "zip" => {
                let src = resolve(&arg);
                match self.store.eng.stat(&src) {
                    Ok(e) => {
                        let name = stem_of(&e.name).to_string();
                        match self.store.eng.start_compress(&[src], &cwd, smart_core::archive::Format::Zip, &name) {
                            Ok(_) => out.push(format!("compressing “{name}.zip”")),
                            Err(er) => out.push(er),
                        }
                    }
                    Err(_) => out.push("usage: zip <file-or-folder>".into()),
                }
            }
            "unzip" => match self.store.eng.start_extract(&resolve(&arg), &cwd) {
                Ok(_) => out.push(format!("extracting {arg}")),
                Err(e) => out.push(e),
            },
            "du" => {
                let target = if arg.is_empty() { cwd.clone() } else { resolve(&arg) };
                out.push(format!("{}  {}", fmt_size(self.store.eng.dir_size(&target)), target));
            }
            "df" => {
                for m in self.store.eng.mounts.iter().filter(|m| m.mounted) {
                    out.push(format!("{:<16} {:>10} free of {:>10}   {}", m.label, fmt_size(m.free), fmt_size(m.total), m.detail));
                }
            }
            "open" => {
                let target = resolve(if arg.is_empty() || arg == "." { "" } else { &arg });
                out.push(format!("opening {target}"));
                self.pending.push(Action::GoPath(target));
                self.focus(Focus::Files);
                self.win_hidden = false;
            }
            other => out.push(format!("{other}: command not found — try `help`")),
        }
        for l in out {
            self.term.lines.push((false, l));
        }
    }
}
