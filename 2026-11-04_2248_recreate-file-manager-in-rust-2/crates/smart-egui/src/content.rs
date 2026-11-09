//! Content plane: grid / list / search results, rubber-band selection, drag & drop, and the Storage view.
use crate::app::*;
use crate::paint::*;
use crate::theme::*;
use egui::*;
use smart_core::treemap;
use smart_core::*;

#[derive(Clone, Copy)]
enum Cell_ {
    FolderCard,
    Card,
    Row,
    SearchRow,
}
struct Placed {
    idx: usize,
    rect: Rect,
    kind: Cell_,
}
enum Item_ {
    Head(String, usize, f32),
    Entry(Placed),
}

pub fn checkbox(p: &Painter, c: Pos2, state: u8, pal: &Pal) {
    let r = Rect::from_center_size(c, vec2(18.0, 18.0));
    if state == 0 {
        grad_rect(p, r, 6.0, if pal.dark { rgba(0, 0, 0, 0.3) } else { rgba(20, 30, 48, 0.10) }, if pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(20, 30, 48, 0.04) });
    } else {
        shadow(p, r, 6.0, 0.0, 2.0, 5.0, -1.0, with_a(pal.accent, 0.4));
        grad_rect(p, r, 6.0, pal.accent_hover, pal.accent);
        reflect(p, r, 6.0, 0.5, 0.25);
        if state == 1 {
            icon(p, c, Icon::Check, 12.0, Color32::WHITE, false);
        } else {
            p.rect_filled(Rect::from_center_size(c, vec2(8.0, 2.0)), 1.0, Color32::WHITE);
        }
    }
}

impl App {
    fn page_header(&mut self, ui: &mut Ui, rect: Rect) -> f32 {
        let pal = self.pal;
        let p = ui.painter().clone();
        let x = rect.left() + 26.0;
        let title = match &self.store.route {
            Route::Search(q) => format!("“{}”", q),
            _ => self.store.title(),
        };
        text_fit(&p, pos2(x, rect.top() + 24.0), Align2::LEFT_CENTER, &title, 26.0, W::Semi, pal.ink1, rect.width() - 60.0);
        let n = self.store.items.len();
        let folders = self.store.items.iter().filter(|e| e.is_dir).count();
        let mut meta = format!("{}  ·  {}", plural(n, "item", "items"), fmt_size(self.store.total_size()));
        if folders > 0 {
            meta += &format!("  ·  {}", plural(folders, "folder", "folders"));
        }
        if matches!(self.store.route, Route::Search(_)) && !self.store.eng.index.ready {
            meta += "  ·  still indexing…";
        }
        text(&p, pos2(x, rect.top() + 56.0), Align2::LEFT_CENTER, &meta, 12.5, W::Medium, pal.ink3);
        let mut h = 78.0;
        if let Route::Search(_) = &self.store.route {
            // parsed query chips + scope switch
            let mut cx = x;
            let cy = rect.top() + 94.0;
            let chips = self.store.query.chips.clone();
            for c in chips {
                let w = text_w(&p, &c, 11.5, W::Medium) + 22.0;
                let r = Rect::from_center_size(pos2(cx + w / 2.0, cy), vec2(w, 26.0));
                grad_rect(&p, r, 13.0, with_a(pal.accent, if pal.dark { 0.25 } else { 0.16 }), with_a(pal.accent, if pal.dark { 0.15 } else { 0.08 }));
                reflect(&p, r, 13.0, 0.5, 0.0);
                text(&p, r.center(), Align2::CENTER_CENTER, &c, 11.5, W::Medium, pal.accent_ink);
                cx += w + 8.0;
            }
            let scoped = self.store.in_folder_origin();
            if scoped {
                let every = self.store.search_everywhere;
                for (i, (l, v)) in [("This folder", false), ("Everywhere", true)].into_iter().enumerate() {
                    let w = text_w(&p, l, 11.5, W::Medium) + 22.0;
                    let r = Rect::from_min_size(pos2(rect.right() - 26.0 - 190.0 + i as f32 * 96.0, cy - 13.0), vec2(w.max(86.0), 26.0));
                    let resp = ui.interact(r, Id::new(("scope", i)), Sense::click());
                    let on = every == v;
                    if on {
                        grad_rect(&p, r, 13.0, if pal.dark { rgba(255, 255, 255, 0.2) } else { rgba(255, 255, 255, 1.0) }, if pal.dark { rgba(255, 255, 255, 0.1) } else { rgba(244, 248, 254, 1.0) });
                        shadow(&p, r, 13.0, 0.0, 2.0, 5.0, 0.0, rgba(16, 26, 44, 0.16));
                        reflect(&p, r, 13.0, 0.8, 0.0);
                    }
                    text(&p, r.center(), Align2::CENTER_CENTER, l, 11.5, W::Medium, if on { pal.accent_ink } else { pal.ink3 });
                    if resp.clicked() {
                        self.act(Action::SearchEverywhere(v));
                    }
                }
            }
            h = 114.0;
        }
        h
    }

    pub fn content_ui(&mut self, ui: &mut Ui, rect: Rect) {
        if matches!(self.store.route, Route::Storage) {
            return self.storage_ui(ui, rect);
        }
        let pal = self.pal;
        let head_h = self.page_header(ui, rect);
        let body = Rect::from_min_max(pos2(rect.left(), rect.top() + head_h), rect.max);
        let items = self.store.items.clone();
        let search = matches!(self.store.route, Route::Search(_));
        let list = self.store.view == ViewMode::List && !search || search;
        let w = (body.width() - 52.0).max(200.0);
        // ── layout pre-pass
        let mut placed: Vec<Item_> = vec![];
        let mut y = 4.0;
        let gap = 16.0;
        let cols = if w >= 980.0 { 4 } else if w >= 640.0 { 3 } else { 2 };
        if list {
            y += if search { 0.0 } else { 34.0 };
            let rh = if search { 56.0 } else { 40.0 };
            for (i, _) in items.iter().enumerate() {
                placed.push(Item_::Entry(Placed { idx: i, rect: Rect::from_min_size(pos2(0.0, y), vec2(w, rh - 4.0)), kind: if search { Cell_::SearchRow } else { Cell_::Row } }));
                y += rh;
            }
            self.cols = 1;
        } else {
            self.cols = cols;
            let tw = (w - gap * (cols as f32 - 1.0)) / cols as f32;
            for g in self.store.groups() {
                placed.push(Item_::Head(g.title.clone(), g.idx.len(), y));
                y += 34.0;
                let is_f = g.idx.iter().all(|i| items[*i].is_dir);
                let ch = if is_f { 70.0 } else { tw * 10.0 / 16.0 + 54.0 };
                for (k, i) in g.idx.iter().enumerate() {
                    let (c, r) = (k % cols, k / cols);
                    placed.push(Item_::Entry(Placed { idx: *i, rect: Rect::from_min_size(pos2(c as f32 * (tw + gap), y + r as f32 * (ch + gap)), vec2(tw, ch)), kind: if is_f { Cell_::FolderCard } else { Cell_::Card } }));
                }
                let rows = (g.idx.len() + cols - 1) / cols;
                y += rows as f32 * (ch + gap) + 10.0;
            }
        }
        let total_h = y + 30.0;
        let mut cui = ui.new_child(UiBuilder::new().max_rect(body).layout(Layout::top_down(Align::Min)));
        cui.set_clip_rect(body.intersect(ui.clip_rect()));
        let out = ScrollArea::vertical().id_salt(("content", self.store.route_key())).auto_shrink([false, false]).show(&mut cui, |ui| {
            let (area, _) = ui.allocate_exact_size(vec2(body.width(), total_h.max(body.height() - 4.0)), Sense::hover());
            let origin = pos2(area.left() + 26.0, area.top());
            // background: rubber band + blank-space menu
            let bg = ui.interact(area.intersect(ui.clip_rect()), Id::new("content-bg"), Sense::click_and_drag());
            self.background_interactions(ui, &bg);
            if items.is_empty() {
                self.empty_state(ui, body);
            }
            if list && !search {
                self.list_header(ui, pos2(origin.x, origin.y + 4.0), w);
            }
            for it in &placed {
                match it {
                    Item_::Head(t, n, hy) => {
                        let p = ui.painter();
                        let at = pos2(origin.x, origin.y + hy + 12.0);
                        let tw = text(p, at, Align2::LEFT_CENTER, t, 12.5, W::Semi, pal.ink2).width();
                        text(p, at + vec2(tw + 8.0, 0.5), Align2::LEFT_CENTER, &n.to_string(), 11.5, W::Medium, pal.ink4);
                        p.rect_filled(Rect::from_min_max(pos2(at.x + tw + 38.0, at.y), pos2(origin.x + w, at.y + 1.0)), 0.0, with_a(pal.ink1, 0.055));
                    }
                    Item_::Entry(pl) => {
                        let r = pl.rect.translate(origin.to_vec2());
                        if !r.intersects(ui.clip_rect().expand(40.0)) {
                            // still register rect so rubber band works off-screen
                            self.item_rects.push((items[pl.idx].path.clone(), r));
                            continue;
                        }
                        self.item_rects.push((items[pl.idx].path.clone(), r));
                        let e = &items[pl.idx];
                        match pl.kind {
                            Cell_::FolderCard => self.folder_card(ui, e, r),
                            Cell_::Card => self.file_card(ui, e, r),
                            Cell_::Row => self.list_row(ui, e, r, w, false),
                            Cell_::SearchRow => self.list_row(ui, e, r, w, true),
                        }
                    }
                }
            }
            self.draw_rubber(ui);
        });
        let _ = out;
        // top/bottom fade of the scroll viewport keeps depth cues soft
        let p = ui.painter();
        if pal.dark {
            grad_rect_r(p, Rect::from_min_size(body.min, vec2(body.width(), 10.0)), [0.0; 4], rgba(0, 0, 0, 0.0), rgba(0, 0, 0, 0.0));
        }
    }

    fn empty_state(&self, ui: &mut Ui, body: Rect) {
        let pal = self.pal;
        let c = body.center() + vec2(0.0, -20.0);
        let p = ui.painter();
        let (ic, title, sub) = match &self.store.route {
            Route::Search(_) => (Icon::Search, "No matches".to_string(), "Try fewer words, or use kind:image size:>5mb in:Documents".to_string()),
            Route::Trash => (Icon::Trash, "Trash is empty".to_string(), "Deleted items land here until you empty it".to_string()),
            _ => match &self.store.load_error {
                Some(e) => (Icon::Warning, "Can’t open this folder".to_string(), e.clone()),
                None => (Icon::FolderOpen, "This folder is empty".to_string(), "Drop files here, or press Ctrl Shift N for a new folder".to_string()),
            },
        };
        let r = Rect::from_center_size(c - vec2(0.0, 36.0), vec2(76.0, 76.0));
        let (t, b) = card_colors(&pal);
        surface(p, r, 24.0, &pal, 2, t, b);
        icon(p, r.center(), ic, 34.0, pal.ink3, false);
        text(p, c + vec2(0.0, 30.0), Align2::CENTER_CENTER, &title, 16.0, W::Semi, pal.ink1);
        text(p, c + vec2(0.0, 54.0), Align2::CENTER_CENTER, &sub, 12.5, W::Regular, pal.ink3);
    }

    fn background_interactions(&mut self, ui: &mut Ui, bg: &Response) {
        let ctx = ui.ctx().clone();
        if bg.clicked() && !ctx.input(|i| i.modifiers.command || i.modifiers.ctrl || i.modifiers.shift) {
            self.act(Action::SelectNone);
        }
        if bg.secondary_clicked() {
            self.act(Action::SelectNone);
            self.store.sel.clear();
            let e = self.store.menu_background();
            self.open_menu(&ctx, None, e);
        }
        if bg.drag_started() && self.drag.is_none() {
            self.rubber = ctx.input(|i| i.pointer.press_origin());
            self.rubber_base = if ctx.input(|i| i.modifiers.command || i.modifiers.ctrl) { self.store.sel.clone() } else { vec![] };
        }
        if bg.dragged() && self.drag.is_none() {
            if let (Some(s), Some(c)) = (self.rubber, ctx.input(|i| i.pointer.latest_pos())) {
                let r = Rect::from_two_pos(s, c);
                let mut sel = self.rubber_base.clone();
                for (p, ir) in &self.item_rects_prev {
                    if ir.intersects(r) && !sel.contains(p) {
                        sel.push(p.clone());
                    }
                }
                self.store.sel = sel;
            }
        }
        if bg.drag_stopped() {
            self.rubber = None;
        }
    }
    fn draw_rubber(&mut self, ui: &mut Ui) {
        if let (Some(s), Some(c)) = (self.rubber, ui.ctx().input(|i| i.pointer.latest_pos())) {
            let r = Rect::from_two_pos(s, c);
            let p = ui.painter();
            p.rect_filled(r, 6.0, with_a(self.pal.accent, 0.12));
            p.rect_stroke(r, 6.0, Stroke::new(1.0, with_a(self.pal.accent, 0.7)), StrokeKind::Inside);
        }
    }

    // ───────────── item interactions ─────────────
    fn item_interact(&mut self, ui: &mut Ui, e: &Entry, r: Rect) -> Response {
        let ctx = ui.ctx().clone();
        let resp = ui.interact(r, Id::new(("item", &e.path)), Sense::click_and_drag());
        let (ctrl, shift) = ctx.input(|i| (i.modifiers.command || i.modifiers.ctrl, i.modifiers.shift));
        let selected = self.store.is_selected(&e.path);
        if resp.clicked() || (resp.drag_started() && !selected) {
            self.act(Action::Select { path: e.path.clone(), ctrl, shift });
        }
        if resp.double_clicked() {
            self.act(Action::Open(e.path.clone()));
        }
        if resp.secondary_clicked() {
            if !selected {
                self.store.dispatch(Action::Select { path: e.path.clone(), ctrl: false, shift: false });
            }
            let m = self.store.menu_for_items();
            self.open_menu(&ctx, None, m);
        }
        if resp.drag_started() && self.rubber.is_none() {
            let paths = if selected { self.store.sel.clone() } else { vec![e.path.clone()] };
            self.drag = Some(DragState { paths });
        }
        // folders are drop targets
        if e.is_dir {
            self.drops.push((r, e.path.clone()));
        }
        resp
    }
    fn drop_highlight(&self, ui: &Ui, e: &Entry, r: Rect, rad: f32) {
        if let (Some(d), true) = (&self.drag, e.is_dir) {
            if !d.paths.contains(&e.path) && ui.ctx().input(|i| i.pointer.latest_pos()).map(|p| r.contains(p)).unwrap_or(false) {
                let p = ui.painter();
                p.rect_filled(r, rad, with_a(self.pal.accent, 0.14));
                p.rect_stroke(r, rad, Stroke::new(2.0, self.pal.accent), StrokeKind::Inside);
            }
        }
    }

    fn name_or_rename(&mut self, ui: &mut Ui, e: &Entry, at: Pos2, max_w: f32, size: f32, color: Color32, anchor_center: bool) -> f32 {
        let ctx = ui.ctx().clone();
        if self.store.rename.as_ref().map(|r| r.path == e.path).unwrap_or(false) {
            let rect = Rect::from_min_size(at - vec2(4.0, size * 0.75), vec2(max_w + 8.0, size * 1.5 + 2.0));
            let id = Id::new(("rename", &e.path));
            let (mut txt, fresh) = {
                let r = self.store.rename.as_ref().unwrap();
                (r.text.clone(), r.fresh)
            };
            let p = ui.painter();
            shadow(p, rect, 8.0, 0.0, 2.0, 8.0, 0.0, with_a(self.pal.accent, 0.3));
            p.rect_filled(rect, 8.0, if self.pal.dark { hex(0x1c2536) } else { Color32::WHITE });
            p.rect_stroke(rect, 8.0, Stroke::new(1.5, self.pal.accent), StrokeKind::Inside);
            let resp = ui.put(rect.shrink2(vec2(2.0, 0.0)), TextEdit::singleline(&mut txt).id(id).frame(false).font(font(size, W::Semi)).text_color(self.pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(4, 0)));
            if fresh {
                resp.request_focus();
                let stem = if e.is_dir { e.name.chars().count() } else { stem_of(&e.name).chars().count() };
                let mut st = text_edit::TextEditState::load(&ctx, id).unwrap_or_default();
                st.cursor.set_char_range(Some(text::CCursorRange::two(text::CCursor::new(0), text::CCursor::new(stem))));
                st.store(&ctx, id);
                if let Some(r) = self.store.rename.as_mut() {
                    r.fresh = false;
                }
            }
            if let Some(r) = self.store.rename.as_mut() {
                r.text = txt;
            }
            if resp.lost_focus() {
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    self.act(Action::CancelRename);
                } else {
                    self.act(Action::CommitRename);
                }
            }
            return max_w;
        }
        let g = ellipsized(ui.painter(), &e.name, size, W::Semi, color, max_w);
        let w = g.size().x;
        let pos = if anchor_center { pos2(at.x, at.y - g.size().y / 2.0) } else { pos2(at.x, at.y - g.size().y / 2.0) };
        ui.painter().galley(pos, g, color);
        w
    }

    fn tag_dot(&self, ui: &Ui, e: &Entry, c: Pos2) {
        if let Some(t) = self.store.tag_of(&e.path) {
            dot(ui.painter(), c, 3.6, Color32::from_rgb(t.rgb[0], t.rgb[1], t.rgb[2]));
        }
    }

    fn folder_card(&mut self, ui: &mut Ui, e: &Entry, r: Rect) {
        let pal = self.pal;
        let resp = self.item_interact(ui, e, r);
        let sel = self.store.is_selected(&e.path);
        let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.14);
        let rr = r.translate(vec2(0.0, -2.0 * hov));
        let p = ui.painter().clone();
        let (t, b) = card_colors(&pal);
        surface(&p, rr, 16.0, &pal, if hov > 0.5 || sel { 3 } else { 2 }, t, b);
        if sel {
            grad_rect(&p, rr, 16.0, with_a(pal.accent, 0.13), with_a(pal.accent, 0.08));
            p.rect_stroke(rr, 16.0, Stroke::new(2.0, pal.accent), StrokeKind::Inside);
        }
        let tile = Rect::from_min_size(rr.min + vec2(12.0, 14.0), vec2(42.0, 42.0));
        shadow(&p, tile, 12.0, 0.0, 3.0, 8.0, -2.0, with_a(pal.accent, if pal.dark { 0.35 } else { 0.28 }));
        grad_rect(&p, tile, 12.0, with_a(pal.accent, if pal.dark { 0.38 } else { 0.22 }), with_a(pal.accent, if pal.dark { 0.22 } else { 0.11 }));
        reflect(&p, tile, 12.0, if pal.dark { 0.15 } else { 0.7 }, 0.2);
        icon(&p, tile.center(), Icon::Folder, 23.0, if pal.dark { pal.accent_ink } else { pal.accent }, true);
        let maxw = rr.width() - 54.0 - 40.0;
        let name_w = self.name_or_rename(ui, e, rr.min + vec2(66.0, 25.0), maxw, 13.0, pal.ink1, false);
        let n = e.children.map(|c| plural(c as usize, "item", "items")).unwrap_or_default();
        let sz = self.thumbs.dir_size(&self.store, &e.path);
        text(&p, rr.min + vec2(66.0, 45.0), Align2::LEFT_CENTER, &format!("{n}{}", if sz > 0 { format!("  ·  {}", fmt_size(sz)) } else { String::new() }), 11.5, W::Regular, pal.ink3);
        if sel || hov > 0.35 {
            let c = pos2(rr.right() - 40.0, rr.center().y);
            let tr = ui.interact(Rect::from_center_size(c, vec2(26.0, 26.0)), Id::new(("fold-sel", &e.path)), Sense::click());
            checkbox(&p, c, if sel { 1 } else { 0 }, &pal);
            if tr.clicked() {
                self.act(Action::Select { path: e.path.clone(), ctrl: true, shift: false });
            }
        } else {
            self.tag_dot(ui, e, pos2(rr.right() - 16.0, rr.center().y));
        }
        let _ = name_w;
        self.drop_highlight(ui, e, rr, 16.0);
    }

    fn file_card(&mut self, ui: &mut Ui, e: &Entry, r: Rect) {
        let pal = self.pal;
        let resp = self.item_interact(ui, e, r);
        let sel = self.store.is_selected(&e.path);
        let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.14);
        let media_h = r.width() * 10.0 / 16.0;
        let media = Rect::from_min_size(r.min + vec2(0.0, -2.0 * hov), vec2(r.width(), media_h));
        let p = ui.painter().clone();
        elevation(&p, media, 14.0, if hov > 0.5 || sel { 3 } else { 2 }, pal.dark);
        let col = pal.kind_color(e.kind);
        let ctx = ui.ctx().clone();
        let tex = if e.kind == Kind::Image { self.thumbs.get(&ctx, &self.store, &e.path, 460) } else { None };
        if let Some(t) = tex {
            let sz = t.size_vec2();
            let uv = cover_uv(media, sz);
            fill_poly(&p, &rrect_pts(media, [14.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
            // lit top edge over imagery
            reflect(&p, media, 14.0, if pal.dark { 0.14 } else { 0.55 }, 0.0);
        } else {
            grad_rect(&p, media, 14.0, with_a(col, if pal.dark { 0.30 } else { 0.20 }), with_a(col, if pal.dark { 0.14 } else { 0.07 }));
            reflect(&p, media, 14.0, if pal.dark { 0.12 } else { 0.75 }, 0.1);
            match e.kind {
                Kind::Audio => {
                    let peaks = self.store.wave_peaks(&e.path, 36);
                    let n = peaks.len().max(1);
                    let bw = (media.width() - 48.0) / n as f32;
                    for (i, pk) in peaks.iter().enumerate() {
                        let h = 8.0 + pk * (media.height() * 0.55);
                        let x = media.left() + 24.0 + i as f32 * bw;
                        p.rect_filled(Rect::from_center_size(pos2(x + bw / 2.0, media.center().y), vec2((bw - 2.0).max(1.5), h)), 2.0, with_a(col, 0.8));
                    }
                }
                Kind::Text | Kind::Code => {
                    let ex = self.thumbs.excerpt(&self.store, &e.path);
                    if ex.is_empty() {
                        icon(&p, media.center(), e.kind.icon(), 42.0, col, true);
                    }
                    let mut y = media.top() + 16.0;
                    for (i, l) in ex.lines().take(5).enumerate() {
                        text_fit(&p, pos2(media.left() + 16.0, y), Align2::LEFT_TOP, l, 11.0, if i == 0 { W::Semi } else { W::Regular }, if i == 0 { pal.ink1 } else { pal.ink3 }, media.width() - 32.0);
                        y += 17.0;
                    }
                }
                _ => icon(&p, media.center(), e.kind.icon(), 42.0, col, true),
            }
            let ext = if e.ext.is_empty() { e.kind.singular().to_uppercase() } else { e.ext.to_uppercase() };
            let w = text_w(&p, &ext, 10.0, W::Semi) + 14.0;
            let pill = Rect::from_min_size(pos2(media.left() + 10.0, media.bottom() - 28.0), vec2(w, 18.0));
            grad_rect(&p, pill, 9.0, with_a(col, 0.95), with_a(col, 0.75));
            reflect(&p, pill, 9.0, 0.5, 0.2);
            text(&p, pill.center(), Align2::CENTER_CENTER, &ext, 10.0, W::Semi, Color32::WHITE);
        }
        if sel {
            p.rect_filled(media, 14.0, with_a(pal.accent, 0.14));
            p.rect_stroke(media.expand(2.0), 16.0, Stroke::new(2.5, pal.accent), StrokeKind::Outside);
        }
        if sel || hov > 0.35 {
            let c = media.left_top() + vec2(20.0, 20.0);
            if !pal.dark {
                p.circle_filled(c, 11.5, rgba(255, 255, 255, 0.86));
            }
            let t = ui.interact(Rect::from_center_size(c, vec2(26.0, 26.0)), Id::new(("card-sel", &e.path)), Sense::click());
            checkbox(&p, c, if sel { 1 } else { 0 }, &pal);
            if t.clicked() {
                self.act(Action::Select { path: e.path.clone(), ctrl: true, shift: false });
            }
            if t.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
        }
        // quick look on hover
        if hov > 0.02 && self.drag.is_none() {
            let qr = Rect::from_center_size(media.right_top() + vec2(-22.0, 22.0), vec2(30.0, 30.0));
            let qresp = ui.interact(qr, Id::new(("ql", &e.path)), Sense::click());
            ui.set_opacity(hov);
            let (t, b) = card_colors(&pal);
            surface(ui.painter(), qr, 15.0, &pal, 2, t, b);
            icon(ui.painter(), qr.center(), Icon::Eye, 15.0, if qresp.hovered() { pal.accent_ink } else { pal.ink2 }, qresp.hovered());
            ui.set_opacity(1.0);
            if qresp.clicked() {
                self.act(Action::QuickLook(Some(e.path.clone())));
            }
        }
        // labels
        let ly = media.bottom() + 18.0;
        let maxw = r.width() - 24.0;
        let nw = self.name_or_rename(ui, e, pos2(r.left() + 3.0, ly), maxw, 13.0, pal.ink1, false);
        self.tag_dot(ui, e, pos2(r.left() + 3.0 + nw + 10.0, ly));
        text(&p, pos2(r.left() + 3.0, ly + 19.0), Align2::LEFT_CENTER, &format!("{}  ·  {}", fmt_size(e.size), fmt_date(e.modified)), 11.5, W::Regular, pal.ink3);
    }

    fn list_header(&mut self, ui: &mut Ui, at: Pos2, w: f32) {
        let pal = self.pal;
        let cols = [("Name", SortKey::Name, 0.0), ("Modified", SortKey::Modified, w - 330.0), ("Size", SortKey::Size, w - 190.0), ("Kind", SortKey::Kind, w - 110.0)];
        for (l, k, dx) in cols {
            let r = Rect::from_min_size(at + vec2(dx + if dx == 0.0 { 40.0 } else { 0.0 }, 0.0), vec2(90.0, 26.0));
            if w < 700.0 && (k == SortKey::Kind) {
                continue;
            }
            let resp = ui.interact(r, Id::new(("lh", l)), Sense::click());
            let active = self.store.sort == k;
            let p = ui.painter();
            let tw = text(p, r.left_center(), Align2::LEFT_CENTER, l, 11.5, W::Semi, if active { pal.accent_ink } else if resp.hovered() { pal.ink2 } else { pal.ink3 }).width();
            if active {
                icon(p, r.left_center() + vec2(tw + 10.0, 0.0), if self.store.sort_asc { Icon::ArrowUp } else { Icon::ArrowDown }, 11.0, pal.accent_ink, false);
            }
            if resp.clicked() {
                self.act(Action::SetSort(k));
            }
        }
    }

    fn list_row(&mut self, ui: &mut Ui, e: &Entry, r: Rect, w: f32, search: bool) {
        let pal = self.pal;
        let resp = self.item_interact(ui, e, r);
        let sel = self.store.is_selected(&e.path);
        let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.1);
        let p = ui.painter().clone();
        if sel {
            grad_rect(&p, r, 11.0, with_a(pal.accent, if pal.dark { 0.24 } else { 0.15 }), with_a(pal.accent, if pal.dark { 0.17 } else { 0.10 }));
            reflect(&p, r, 11.0, if pal.dark { 0.1 } else { 0.5 }, 0.0);
        } else if hov > 0.0 {
            p.rect_filled(r, 11.0, with_a(if pal.dark { Color32::WHITE } else { pal.ink1 }, 0.05 * hov));
        }
        let col = pal.kind_color(e.kind);
        let show_tick = sel || hov > 0.3;
        if show_tick {
            let c = pos2(r.left() + 20.0, r.center().y);
            let tr = ui.interact(Rect::from_center_size(c, vec2(26.0, 26.0)), Id::new(("row-sel", &e.path)), Sense::click());
            checkbox(&p, c, if sel { 1 } else { 0 }, &pal);
            if tr.clicked() {
                self.act(Action::Select { path: e.path.clone(), ctrl: true, shift: false });
            }
        }
        let ix = if show_tick { 44.0 } else { 24.0 };
        let ic = Rect::from_center_size(pos2(r.left() + ix, r.center().y), vec2(32.0, 32.0));
        let ctx = ui.ctx().clone();
        let tex = if e.kind == Kind::Image { self.thumbs.get(&ctx, &self.store, &e.path, 96) } else { None };
        if let Some(t) = tex {
            let uv = cover_uv(ic, t.size_vec2());
            elevation(&p, ic, 8.0, 1, pal.dark);
            fill_poly(&p, &rrect_pts(ic, [8.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
        } else {
            grad_rect(&p, ic, 9.0, with_a(col, if pal.dark { 0.32 } else { 0.2 }), with_a(col, if pal.dark { 0.18 } else { 0.09 }));
            reflect(&p, ic, 9.0, if pal.dark { 0.12 } else { 0.7 }, 0.0);
            icon(&p, ic.center(), e.kind.icon(), 17.0, col, true);
        }
        let name_x = r.left() + ix + 24.0;
        let right_cols = if search { 210.0 } else if w < 700.0 { 230.0 } else { 340.0 };
        let maxw = w - 48.0 - right_cols - 30.0;
        let cy = if search { r.center().y - 8.0 } else { r.center().y };
        let nw = if search {
            let idx = self.store.match_idx.get(&e.path).cloned().unwrap_or_default();
            if self.store.rename.as_ref().map(|x| x.path == e.path).unwrap_or(false) {
                self.name_or_rename(ui, e, pos2(name_x, cy), maxw, 13.0, pal.ink1, false)
            } else {
                text_hl(&p, pos2(name_x, cy - 8.0), &e.name, 13.0, pal.ink1, pal.accent_ink, &idx, maxw).width()
            }
        } else {
            self.name_or_rename(ui, e, pos2(name_x, cy), maxw, 13.0, pal.ink1, false)
        };
        self.tag_dot(ui, e, pos2(name_x + nw + 10.0, cy));
        if search {
            text_fit(&p, pos2(name_x, r.center().y + 11.0), Align2::LEFT_CENTER, &parent(&e.path), 11.5, W::Regular, pal.ink3, maxw);
            text(&p, pos2(r.right() - 16.0, r.center().y - 8.0), Align2::RIGHT_CENTER, &if e.is_dir { "Folder".to_string() } else { fmt_size(e.size) }, 12.0, W::Medium, pal.ink2);
            text(&p, pos2(r.right() - 16.0, r.center().y + 10.0), Align2::RIGHT_CENTER, &fmt_date(e.modified), 11.5, W::Regular, pal.ink3);
        } else {
            let x = r.left();
            text(&p, pos2(x + w - 330.0, cy), Align2::LEFT_CENTER, &fmt_date(e.modified), 12.0, W::Regular, pal.ink2);
            text(&p, pos2(x + w - 190.0, cy), Align2::LEFT_CENTER, &if e.is_dir { e.children.map(|c| plural(c as usize, "item", "items")).unwrap_or_default() } else { fmt_size(e.size) }, 12.0, W::Regular, pal.ink2);
            if w >= 700.0 {
                text_fit(&p, pos2(x + w - 110.0, cy), Align2::LEFT_CENTER, &e.kind_label(), 12.0, W::Regular, pal.ink3, 104.0);
            }
        }
        self.drop_highlight(ui, e, r, 11.0);
    }

    // ───────────────────────── storage ─────────────────────────
    pub fn storage_ui(&mut self, ui: &mut Ui, rect: Rect) {
        let pal = self.pal;
        let p = ui.painter().clone();
        let x = rect.left() + 26.0;
        text(&p, pos2(x, rect.top() + 24.0), Align2::LEFT_CENTER, "Storage", 26.0, W::Semi, pal.ink1);
        let idx_n = self.store.eng.index.entries.len();
        let scanning = !self.store.eng.index.ready;
        let m = self.store.eng.mount_of(&self.store.eng.home).cloned();
        let sub = match &m {
            Some(m) if m.total > 0 => format!("{} · {} free of {}  ·  {} items {}", m.label, fmt_size(m.free), fmt_size(m.total), idx_n, if scanning { "scanned so far…" } else { "analysed" }),
            _ => format!("{idx_n} items {}", if scanning { "scanned so far…" } else { "analysed" }),
        };
        text(&p, pos2(x, rect.top() + 56.0), Align2::LEFT_CENTER, &sub, 12.5, W::Medium, pal.ink3);
        // usage by kind
        let mut by: std::collections::BTreeMap<Kind, u64> = Default::default();
        for e in self.store.eng.index.entries.iter().filter(|e| !e.is_dir) {
            *by.entry(e.kind).or_default() += e.size;
        }
        let total: u64 = by.values().sum::<u64>().max(1);
        let bar = Rect::from_min_size(pos2(x, rect.top() + 80.0), vec2(rect.width() - 52.0, 14.0));
        p.rect_filled(bar, 7.0, with_a(pal.ink1, 0.07));
        let mut v: Vec<_> = by.iter().collect();
        v.sort_by_key(|(_, s)| std::cmp::Reverse(**s));
        let mut cx = bar.left();
        for (k, s) in v.iter().take(7) {
            let wseg = bar.width() * **s as f32 / total as f32;
            if wseg < 1.0 {
                continue;
            }
            let seg = Rect::from_min_size(pos2(cx, bar.top()), vec2((wseg - 2.0).max(1.0), bar.height()));
            let c = pal.kind_color(**k);
            grad_rect(&p, seg, 7.0, lerpc(c, Color32::WHITE, 0.25), c);
            reflect(&p, seg, 7.0, 0.4, 0.2);
            cx += wseg;
        }
        let mut lx = bar.left();
        for (k, s) in v.iter().take(6) {
            let l = format!("{}  {}", k.plural(), fmt_size(**s));
            dot(&p, pos2(lx + 4.0, bar.bottom() + 16.0), 4.0, pal.kind_color(**k));
            let w = text(&p, pos2(lx + 14.0, bar.bottom() + 16.0), Align2::LEFT_CENTER, &l, 11.5, W::Medium, pal.ink2).width();
            lx += w + 34.0;
        }
        let top = bar.bottom() + 38.0;
        let area = Rect::from_min_max(pos2(x, top), pos2(rect.right() - 26.0, rect.bottom() - 10.0));
        let gap = 18.0;
        let lw = ((area.width() - gap) * 0.56).floor();
        let lens_r = Rect::from_min_size(area.min, vec2(lw, area.height()));
        let clean_r = Rect::from_min_max(pos2(lens_r.right() + gap, area.top()), area.max);
        self.lens_card(ui, lens_r);
        self.clean_card(ui, clean_r);
    }

    fn lens_card(&mut self, ui: &mut Ui, r: Rect) {
        let pal = self.pal;
        let p = ui.painter().clone();
        let (t, b) = card_colors(&pal);
        surface(&p, r, 20.0, &pal, 2, t, b);
        icon(&p, pos2(r.left() + 26.0, r.top() + 28.0), Icon::Pie, 18.0, pal.accent_ink, true);
        text(&p, pos2(r.left() + 44.0, r.top() + 28.0), Align2::LEFT_CENTER, "Space Lens", 14.0, W::Semi, pal.ink1);
        let root = self.store.storage.lens_root.clone();
        let rel = if is_within(&root, &self.store.eng.home) { format!("~{}", &root[self.store.eng.home.len()..]) } else { root.clone() };
        let at_home = root == self.store.eng.home;
        if !at_home {
            let br = Rect::from_center_size(pos2(r.right() - 28.0, r.top() + 28.0), vec2(30.0, 30.0));
            if button(ui, &pal, br, Id::new("lens-up"), Some(Icon::Up), "", Look::Plain, false, true).on_hover_text("Up one level").clicked() {
                self.act(Action::LensGo(parent(&root)));
            }
        }
        text_fit(&p, pos2(r.right() - if at_home { 24.0 } else { 52.0 }, r.top() + 28.0), Align2::RIGHT_CENTER, &rel, 11.5, W::Medium, pal.ink3, r.width() * 0.4);
        let tm = Rect::from_min_max(r.min + vec2(16.0, 54.0), r.max - vec2(16.0, 16.0));
        let items: Vec<LensItem> = self.store.storage.lens.iter().take(48).cloned().collect();
        if items.is_empty() {
            text(&p, tm.center(), Align2::CENTER_CENTER, if self.store.eng.index.ready { "Nothing to show here" } else { "Scanning…" }, 13.0, W::Medium, pal.ink3);
            return;
        }
        let sizes: Vec<u64> = items.iter().map(|i| i.size).collect();
        let cells = treemap::layout(&sizes, tm.width(), tm.height());
        let total: u64 = sizes.iter().sum();
        let mut hovered = None;
        for c in &cells {
            let it = &items[c.idx];
            let cr = Rect::from_min_size(tm.min + vec2(c.x, c.y), vec2(c.w, c.h)).shrink(1.5);
            if cr.width() < 3.0 || cr.height() < 3.0 {
                continue;
            }
            let resp = ui.interact(cr, Id::new(("lens", &it.path)), Sense::click());
            let hv = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.12);
            let col = if it.is_dir { lerpc(pal.kind_color(Kind::Folder), pal.kind_color(items_kind(&it.name)), 0.5) } else { pal.kind_color(it.kind) };
            let col = lerpc(col, Color32::WHITE, 0.12 * hv);
            grad_rect(&p, cr, 9.0, lerpc(col, Color32::WHITE, 0.22), lerpc(col, Color32::BLACK, 0.12));
            reflect(&p, cr, 9.0, 0.45, 0.18);
            if cr.width() > 74.0 && cr.height() > 38.0 {
                text_fit(&p, cr.min + vec2(10.0, 15.0), Align2::LEFT_CENTER, &it.name, 12.0, W::Semi, Color32::WHITE, cr.width() - 20.0);
                text(&p, cr.min + vec2(10.0, 31.0), Align2::LEFT_CENTER, &format!("{}  ·  {:.0}%", fmt_size(it.size), it.size as f32 * 100.0 / total as f32), 10.5, W::Medium, rgba(255, 255, 255, 0.82));
            }
            if resp.hovered() {
                hovered = Some(c.idx);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            if resp.clicked() {
                if it.is_dir {
                    self.act(Action::LensGo(it.path.clone()));
                } else {
                    self.act(Action::Properties(Some(it.path.clone())));
                }
            }
            if resp.secondary_clicked() {
                self.act(Action::Properties(Some(it.path.clone())));
            }
        }
        self.lens_hover = hovered;
        if let Some(h) = hovered {
            let it = &items[h];
            egui::show_tooltip_text(ui.ctx(), ui.layer_id(), Id::new("lens-tip"), format!("{} — {}{}", it.name, fmt_size(it.size), if it.is_dir { "  (click to explore)" } else { "" }));
        }
    }

    fn clean_card(&mut self, ui: &mut Ui, r: Rect) {
        let pal = self.pal;
        let p = ui.painter().clone();
        let (t, b) = card_colors(&pal);
        surface(&p, r, 20.0, &pal, 2, t, b);
        icon(&p, pos2(r.left() + 26.0, r.top() + 28.0), Icon::Broom, 18.0, pal.accent_ink, true);
        text(&p, pos2(r.left() + 44.0, r.top() + 28.0), Align2::LEFT_CENTER, "Cleanup", 14.0, W::Semi, pal.ink1);
        let all: u64 = self.store.storage.report.cats.iter().map(|c| c.size()).sum();
        text(&p, pos2(r.right() - 24.0, r.top() + 28.0), Align2::RIGHT_CENTER, &format!("{} reclaimable", fmt_size(all)), 11.5, W::Medium, pal.ink3);
        let sel_total = self.store.storage.report.selected_total();
        let foot_h = 70.0;
        let list = Rect::from_min_max(r.min + vec2(10.0, 52.0), pos2(r.right() - 10.0, r.bottom() - foot_h));
        let mut lui = ui.new_child(UiBuilder::new().max_rect(list).layout(Layout::top_down(Align::Min)));
        lui.set_clip_rect(list.intersect(ui.clip_rect()));
        let cats = self.store.storage.report.cats.clone();
        ScrollArea::vertical().id_salt("clean-scroll").auto_shrink([false, false]).show(&mut lui, |ui| {
            ui.set_width(list.width());
            for (ci, cat) in cats.iter().enumerate() {
                if cat.items.is_empty() {
                    continue;
                }
                let (row, _) = ui.allocate_exact_size(vec2(list.width(), 60.0), Sense::hover());
                let resp = ui.interact(row, Id::new(("cat", ci)), Sense::click());
                let p = ui.painter();
                if resp.hovered() {
                    p.rect_filled(row, 14.0, with_a(if pal.dark { Color32::WHITE } else { pal.ink1 }, 0.045));
                }
                let tile = Rect::from_min_size(row.min + vec2(8.0, 10.0), vec2(40.0, 40.0));
                grad_rect(p, tile, 12.0, with_a(pal.accent, if pal.dark { 0.3 } else { 0.18 }), with_a(pal.accent, if pal.dark { 0.16 } else { 0.08 }));
                reflect(p, tile, 12.0, if pal.dark { 0.12 } else { 0.7 }, 0.15);
                icon(p, tile.center(), cat.icon, 20.0, pal.accent_ink, true);
                text(p, row.min + vec2(58.0, 22.0), Align2::LEFT_CENTER, &cat.title, 13.0, W::Semi, pal.ink1);
                text_fit(p, row.min + vec2(58.0, 40.0), Align2::LEFT_CENTER, &cat.desc, 11.5, W::Regular, pal.ink3, list.width() - 190.0);
                text(p, pos2(row.right() - 64.0, row.center().y), Align2::RIGHT_CENTER, &fmt_size(cat.size()), 12.5, W::Semi, pal.ink2);
                let sel_n = cat.items.iter().filter(|i| i.selected).count();
                let state = if sel_n == 0 { 0 } else if sel_n == cat.items.len() { 1 } else { 2 };
                let cb = pos2(row.right() - 36.0, row.center().y);
                checkbox(p, cb, state, &pal);
                icon(p, pos2(row.right() - 14.0, row.center().y), if cat.open { Icon::ChevronDown } else { Icon::ChevronRight }, 12.0, pal.ink3, false);
                let cbr = ui.interact(Rect::from_center_size(cb, vec2(26.0, 26.0)), Id::new(("catcb", ci)), Sense::click());
                if cbr.clicked() {
                    self.act(Action::CleanToggleCat(ci));
                } else if resp.clicked() {
                    self.act(Action::CleanExpand(ci));
                }
                if cat.open {
                    for (ii, it) in cat.items.iter().enumerate().take(12) {
                        let (ir, _) = ui.allocate_exact_size(vec2(list.width(), 34.0), Sense::hover());
                        let iresp = ui.interact(ir, Id::new(("cit", ci, ii)), Sense::click());
                        let p = ui.painter();
                        if iresp.hovered() {
                            p.rect_filled(ir, 10.0, with_a(if pal.dark { Color32::WHITE } else { pal.ink1 }, 0.045));
                        }
                        checkbox(p, pos2(ir.left() + 30.0, ir.center().y), if it.selected { 1 } else { 0 }, &pal);
                        text_fit(p, pos2(ir.left() + 50.0, ir.center().y - 6.0), Align2::LEFT_CENTER, name_of(&it.path), 12.0, W::Medium, pal.ink1, list.width() - 170.0);
                        text_fit(p, pos2(ir.left() + 50.0, ir.center().y + 8.0), Align2::LEFT_CENTER, &it.note, 10.5, W::Regular, pal.ink3, list.width() - 170.0);
                        text(p, pos2(ir.right() - 14.0, ir.center().y), Align2::RIGHT_CENTER, &fmt_size(it.size), 11.5, W::Medium, pal.ink2);
                        if iresp.clicked() {
                            self.act(Action::CleanToggle(ci, ii));
                        }
                    }
                    ui.add_space(6.0);
                }
            }
        });
        // footer: the one big action
        let fr = Rect::from_min_max(pos2(r.left() + 16.0, r.bottom() - foot_h + 12.0), pos2(r.right() - 16.0, r.bottom() - 14.0));
        if let Some(f) = self.store.storage.last_freed {
            text(&p, pos2(fr.left(), fr.center().y - 8.0), Align2::LEFT_CENTER, &format!("Freed {}", fmt_size(f)), 13.0, W::Semi, pal.good);
            text(&p, pos2(fr.left(), fr.center().y + 9.0), Align2::LEFT_CENTER, "Moved to Trash or deleted — Undo is available", 11.0, W::Regular, pal.ink3);
        } else {
            text(&p, pos2(fr.left(), fr.center().y - 8.0), Align2::LEFT_CENTER, &format!("{} selected", fmt_size(sel_total)), 13.0, W::Semi, pal.ink1);
            text(&p, pos2(fr.left(), fr.center().y + 9.0), Align2::LEFT_CENTER, "Safe items are pre-selected", 11.0, W::Regular, pal.ink3);
        }
        let bw = 150.0;
        let br = Rect::from_min_size(pos2(fr.right() - bw, fr.center().y - 20.0), vec2(bw, 40.0));
        if button(ui, &pal, br, Id::new("clean-go"), Some(Icon::Broom), &format!("Clean {}", fmt_size(sel_total)), Look::Accent, false, sel_total > 0).clicked() {
            self.act(Action::CleanApply);
        }
    }
}

fn items_kind(name: &str) -> Kind {
    let h: u32 = name.bytes().fold(7, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
    [Kind::Image, Kind::Audio, Kind::Video, Kind::Archive, Kind::Document, Kind::Code, Kind::App][(h % 7) as usize]
}

trait StoreExt {
    fn route_key(&self) -> String;
    fn in_folder_origin(&self) -> bool;
}
impl StoreExt for Store {
    fn route_key(&self) -> String {
        format!("{:?}", self.route)
    }
    fn in_folder_origin(&self) -> bool {
        !self.search_origin_is_none()
    }
}
