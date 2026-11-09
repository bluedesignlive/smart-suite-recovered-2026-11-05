//! Overlays: context menu, command palette (with rail), Ask Smart panel, toasts, drag ghost.
use crate::app::*;
use crate::paint::*;
use crate::theme::*;
use egui::*;
use smart_core::*;
use smart_core::Modal;

fn rich(p: &Painter, s: &str, size: f32, color: Color32, bold: Color32, wrap: f32) -> std::sync::Arc<Galley> {
    let mut job = text::LayoutJob::default();
    job.wrap.max_width = wrap;
    for (i, part) in s.split("**").enumerate() {
        if part.is_empty() {
            continue;
        }
        let b = i % 2 == 1;
        job.append(part, 0.0, text::TextFormat { font_id: font(size, if b { W::Semi } else { W::Regular }), color: if b { bold } else { color }, line_height: Some(size * 1.5), ..Default::default() });
    }
    p.layout_job(job)
}

struct MenuOut {
    rect: Rect,
    hover_sub: Option<(usize, Rect)>,
    clicked: Option<Action>,
}

impl App {
    pub fn overlays(&mut self, ctx: &Context) {
        self.context_menu(ctx);
        self.palette_ui(ctx);
        self.modals(ctx);
        self.transfer_dialog(ctx);
        self.quicklook_ui(ctx);
        self.toasts_ui(ctx);
        self.drag_ghost(ctx);
        self.drop_overlay(ctx);
    }

    // ───────────────────────── context menu ─────────────────────────
    fn menu_panel(&mut self, ctx: &Context, id: &str, pos: Pos2, entries: &[MenuEntry], w: f32, alpha: f32, active_sub: Option<usize>) -> MenuOut {
        let pal = self.pal;
        let h: f32 = entries.iter().map(|e| match e { MenuEntry::Sep => 11.0, _ => 33.0 }).sum::<f32>() + 14.0;
        let screen = ctx.screen_rect();
        let pos = pos2(pos.x.min(screen.right() - w - 8.0).max(screen.left() + 8.0), pos.y.min(screen.bottom() - h - 8.0).max(screen.top() + 8.0));
        let mut out = MenuOut { rect: Rect::from_min_size(pos, vec2(w, h)), hover_sub: None, clicked: None };
        Area::new(Id::new(id)).order(Order::Foreground).fixed_pos(pos + vec2(0.0, (1.0 - alpha) * -6.0)).interactable(true).show(ctx, |ui| {
            ui.set_opacity(alpha);
            let (rect, _) = ui.allocate_exact_size(vec2(w, h), Sense::hover());
            let p = ui.painter().clone();
            elevation(&p, rect, 18.0, 4, pal.dark);
            grad_rect(&p, rect, 18.0, pal.menu, lerpc(pal.menu, if pal.dark { hex(0x161e2d) } else { hex(0xf1f5fc) }, 0.5));
            reflect(&p, rect, 18.0, if pal.dark { 0.12 } else { 0.9 }, 0.0);
            let mut y = rect.top() + 7.0;
            for (i, e) in entries.iter().enumerate() {
                match e {
                    MenuEntry::Sep => {
                        p.rect_filled(Rect::from_min_size(pos2(rect.left() + 18.0, y + 5.0), vec2(w - 36.0, 1.0)), 0.0, with_a(pal.ink1, 0.07));
                        y += 11.0;
                    }
                    MenuEntry::Item(m) => {
                        let r = Rect::from_min_size(pos2(rect.left() + 7.0, y), vec2(w - 14.0, 33.0));
                        let resp = ui.interact(r, Id::new((id, i)), if m.enabled { Sense::click() } else { Sense::hover() });
                        let hov = resp.hovered() && m.enabled;
                        if hov {
                            if m.danger {
                                grad_rect(&p, r, 9.0, with_a(pal.danger, 0.16), with_a(pal.danger, 0.1));
                            } else {
                                grad_rect(&p, r, 9.0, with_a(pal.accent, if pal.dark { 0.26 } else { 0.16 }), with_a(pal.accent, if pal.dark { 0.18 } else { 0.10 }));
                            }
                            reflect(&p, r, 9.0, if pal.dark { 0.1 } else { 0.5 }, 0.0);
                        }
                        let fg = if !m.enabled { pal.ink4 } else if m.danger { pal.danger } else if hov { pal.accent_ink } else { pal.ink1 };
                        if let Some(c) = m.swatch {
                            dot(&p, pos2(r.left() + 20.0, r.center().y), 5.0, Color32::from_rgb(c[0], c[1], c[2]));
                        } else {
                            icon(&p, pos2(r.left() + 20.0, r.center().y), m.icon, 16.0, if m.danger || !m.enabled { fg } else if hov { pal.accent_ink } else { pal.ink2 }, hov);
                        }
                        text_fit(&p, pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, &m.label, 13.0, W::Medium, fg, w - 120.0);
                        if m.checked {
                            icon(&p, pos2(r.right() - 16.0, r.center().y), Icon::Check, 14.0, pal.accent_ink, false);
                        } else if !m.shortcut.is_empty() {
                            let sc = if self.mac() { m.shortcut.replace("Ctrl+", "⌘") } else { m.shortcut.to_string() };
                            text(&p, pos2(r.right() - 12.0, r.center().y), Align2::RIGHT_CENTER, &sc, 11.5, W::Regular, pal.ink4);
                        }
                        if resp.clicked() && m.enabled {
                            out.clicked = Some(m.action.clone());
                        }
                        if resp.hovered() {
                            ctx.set_cursor_icon(CursorIcon::PointingHand);
                        }
                        y += 33.0;
                    }
                    MenuEntry::Sub(label, ic, _) => {
                        let r = Rect::from_min_size(pos2(rect.left() + 7.0, y), vec2(w - 14.0, 33.0));
                        let resp = ui.interact(r, Id::new((id, i)), Sense::click());
                        let open = active_sub == Some(i) || resp.hovered();
                        if open {
                            grad_rect(&p, r, 9.0, with_a(pal.accent, if pal.dark { 0.2 } else { 0.12 }), with_a(pal.accent, if pal.dark { 0.14 } else { 0.07 }));
                        }
                        icon(&p, pos2(r.left() + 20.0, r.center().y), *ic, 16.0, if open { pal.accent_ink } else { pal.ink2 }, open);
                        text(&p, pos2(r.left() + 40.0, r.center().y), Align2::LEFT_CENTER, label, 13.0, W::Medium, if open { pal.accent_ink } else { pal.ink1 });
                        icon(&p, pos2(r.right() - 14.0, r.center().y), Icon::ChevronRight, 12.0, pal.ink3, false);
                        if resp.hovered() || resp.clicked() {
                            out.hover_sub = Some((i, r));
                        }
                        y += 33.0;
                    }
                }
            }
        });
        out
    }

    fn context_menu(&mut self, ctx: &Context) {
        let Some(m) = &self.menu else { return };
        let (entries, pos, t0, sub) = (m.entries.clone(), m.pos, m.t0, m.sub);
        let a = ((self.t - t0) / 0.14).clamp(0.0, 1.0) as f32;
        let a = 1.0 - (1.0 - a).powi(3);
        let main = self.menu_panel(ctx, "ctx-main", pos, &entries, 252.0, a, sub.map(|s| s.0));
        let mut hover_sub = main.hover_sub;
        let mut sub_rect: Option<Rect> = None;
        let mut clicked = main.clicked;
        let cur_sub = hover_sub.or(sub);
        if let Some((i, r)) = cur_sub {
            if let MenuEntry::Sub(_, _, kids) = &entries[i] {
                let out = self.menu_panel(ctx, "ctx-sub", pos2(r.right() + 4.0, r.top() - 7.0), kids, 214.0, 1.0, None);
                sub_rect = Some(out.rect);
                if out.clicked.is_some() {
                    clicked = out.clicked;
                }
                if hover_sub.is_none() {
                    let ptr = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
                    // keep the submenu open while moving across the gap into it
                    if r.expand2(vec2(10.0, 0.0)).contains(ptr) || out.rect.expand(6.0).contains(ptr) {
                        hover_sub = Some((i, r));
                    }
                }
            }
        }
        if let Some(menu) = self.menu.as_mut() {
            menu.sub = hover_sub;
        }
        if let Some(act) = clicked {
            self.menu = None;
            self.act(act);
            return;
        }
        if ctx.input(|i| i.pointer.any_pressed()) {
            let ptr = ctx.input(|i| i.pointer.latest_pos()).unwrap_or_default();
            if !main.rect.contains(ptr) && !sub_rect.map(|r| r.contains(ptr)).unwrap_or(false) {
                self.menu = None;
            }
        }
    }

    // ───────────────────────── command palette ─────────────────────────
    fn palette_ui(&mut self, ctx: &Context) {
        if !self.store.palette.open {
            return;
        }
        let pal = self.pal;
        let screen = ctx.screen_rect();
        let t = ctx.animate_bool_with_time(Id::new("pal-t"), true, 0.16);
        // scrim
        Area::new(Id::new("pal-scrim")).order(Order::Foreground).fixed_pos(screen.min).show(ctx, |ui| {
            let (r, resp) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter().rect_filled(r, 0.0, pal.scrim.gamma_multiply(t));
            if resp.clicked() {
                self.pending.push(Action::Palette(None));
            }
        });
        // keyboard
        let last_q: String = ctx.data(|d| d.get_temp(Id::new("pal-last"))).unwrap_or_default();
        if last_q != self.store.palette.query {
            self.store.palette.sel = 0;
            ctx.data_mut(|d| d.insert_temp(Id::new("pal-last"), self.store.palette.query.clone()));
        }
        let items = self.store.palette_items();
        let n = items.len();
        let mut moved = false;
        ctx.input_mut(|i| {
            if i.consume_key(Modifiers::NONE, Key::ArrowDown) {
                self.store.palette.sel = (self.store.palette.sel + 1).min(n.saturating_sub(1));
                moved = true;
            }
            if i.consume_key(Modifiers::NONE, Key::ArrowUp) {
                self.store.palette.sel = self.store.palette.sel.saturating_sub(1);
                moved = true;
            }
            if i.consume_key(Modifiers::NONE, Key::Tab) {
                let k = Rail::ALL.iter().position(|r| *r == self.store.palette.rail).unwrap_or(0);
                self.store.palette.rail = Rail::ALL[(k + 1) % Rail::ALL.len()];
                self.store.palette.sel = 0;
            }
        });
        let sel = self.store.palette.sel.min(n.saturating_sub(1));
        let enter = ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter));
        let center = ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Enter) || i.consume_key(Modifiers::CTRL, Key::Enter));
        if center && !self.store.palette.query.trim().is_empty() {
            let q = self.store.palette.query.clone();
            self.store.palette.open = false;
            self.act(Action::AiAsk(q));
        } else if enter {
            if let Some(c) = items.get(sel) {
                let a = c.action.clone();
                self.store.palette.open = false;
                self.act(a);
            }
        }

        let w = (screen.width() - 40.0).min(860.0);
        let h = (screen.height() - 80.0).min(520.0);
        let rect = Rect::from_center_size(pos2(screen.center().x, screen.top() + 60.0 + h / 2.0 + (screen.height() - h - 120.0).max(0.0) * 0.25), vec2(w, h));
        let off = (1.0 - t) * 8.0;
        Area::new(Id::new("palette")).order(Order::Foreground).fixed_pos(rect.min + vec2(0.0, off)).show(ctx, |ui| {
            ui.set_opacity(t);
            let (rect, _) = ui.allocate_exact_size(rect.size(), Sense::hover());
            let p = ui.painter().clone();
            elevation(&p, rect, 24.0, 5, pal.dark);
            grad_rect(&p, rect, 24.0, pal.panel_top, pal.panel_bottom);
            reflect(&p, rect, 24.0, if pal.dark { 0.12 } else { 0.95 }, 0.0);
            // rail ------------------------------------------------------
            let rail_w = 78.0;
            let rail = Rect::from_min_size(rect.min, vec2(rail_w, rect.height()));
            grad_rect_r(&p, rail, [24.0, 0.0, 0.0, 24.0], if pal.dark { rgba(255, 255, 255, 0.03) } else { rgba(47, 111, 235, 0.045) }, if pal.dark { rgba(255, 255, 255, 0.015) } else { rgba(47, 111, 235, 0.02) });
            for (i, r) in Rail::ALL.iter().enumerate() {
                let cell = Rect::from_min_size(rail.min + vec2(10.0, 18.0 + i as f32 * 62.0), vec2(rail_w - 20.0, 54.0));
                let resp = ui.interact(cell, Id::new(("rail", i)), Sense::click());
                let active = self.store.palette.rail == *r;
                let a = ctx.animate_bool_with_time(resp.id.with("a"), active, 0.16);
                if a > 0.01 {
                    shadow(&p, cell, 16.0, 0.0, 3.0, 9.0, -2.0, with_a(pal.accent, 0.3 * a));
                    grad_rect(&p, cell, 16.0, with_a(pal.accent, (if pal.dark { 0.3 } else { 0.18 }) * a), with_a(pal.accent, (if pal.dark { 0.2 } else { 0.1 }) * a));
                    reflect(&p, cell, 16.0, 0.55 * a, 0.0);
                } else if resp.hovered() {
                    p.rect_filled(cell, 16.0, with_a(pal.ink1, 0.05));
                }
                icon(&p, cell.center() - vec2(0.0, 8.0), r.icon(), 20.0, if active { pal.accent_ink } else { pal.ink3 }, active);
                text(&p, cell.center() + vec2(0.0, 14.0), Align2::CENTER_CENTER, r.label(), 10.5, if active { W::Semi } else { W::Medium }, if active { pal.accent_ink } else { pal.ink3 });
                if resp.clicked() {
                    self.store.palette.rail = *r;
                    self.store.palette.sel = 0;
                }
            }
            // search header -----------------------------------------------
            let stage_w = if rect.width() > 760.0 { 270.0 } else { 0.0 };
            let main = Rect::from_min_max(pos2(rail.right() + 8.0, rect.top()), pos2(rect.right() - stage_w, rect.bottom()));
            let head = Rect::from_min_size(main.min + vec2(14.0, 14.0), vec2(main.width() - 28.0, 52.0));
            grad_rect(&p, head, 18.0, if pal.dark { rgba(0, 0, 0, 0.3) } else { rgba(20, 30, 48, 0.07) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(20, 30, 48, 0.03) });
            icon(&p, pos2(head.left() + 24.0, head.center().y), self.store.palette.rail.icon(), 19.0, pal.accent_ink, true);
            let hint = match self.store.palette.rail {
                Rail::All => "Search files, run commands, or ask Smart…",
                Rail::Go => "Go to a place, device or tag…",
                Rail::Files => "Find files or act on the selection…",
                Rail::Make => "Create something new…",
                Rail::Look => "Appearance & view…",
                Rail::Ask => "Ask Smart anything about your files…",
            };
            let mut q = self.store.palette.query.clone();
            let tid = Id::new("pal-input");
            let resp = ui.put(Rect::from_min_max(head.min + vec2(46.0, 4.0), head.max - vec2(60.0, 4.0)), TextEdit::singleline(&mut q).id(tid).frame(false).hint_text(RichText::new(hint).color(pal.ink3).font(font(16.0, W::Regular))).font(font(16.0, W::Medium)).text_color(pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(2, 0)));
            if self.palette_fresh {
                resp.request_focus();
                self.palette_fresh = false;
            }
            if !resp.has_focus() && ctx.memory(|m| m.focused().is_none()) {
                resp.request_focus();
            }
            self.store.palette.query = q;
            let esc = Rect::from_center_size(pos2(head.right() - 26.0, head.center().y), vec2(36.0, 22.0));
            grad_rect(&p, esc, 8.0, if pal.dark { rgba(255, 255, 255, 0.12) } else { rgba(255, 255, 255, 0.95) }, if pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(244, 248, 254, 0.8) });
            shadow(&p, esc, 8.0, 0.0, 1.0, 2.0, 0.0, rgba(16, 26, 44, 0.12));
            text(&p, esc.center(), Align2::CENTER_CENTER, "esc", 10.5, W::Medium, pal.ink3);

            // list -------------------------------------------------------
            let foot = 46.0;
            let list = Rect::from_min_max(pos2(main.left() + 8.0, head.bottom() + 8.0), pos2(main.right() - 8.0, rect.bottom() - foot - 14.0));
            let mut lui = ui.new_child(UiBuilder::new().max_rect(list).layout(Layout::top_down(Align::Min)));
            lui.set_clip_rect(list.intersect(ui.clip_rect()));
            let mut last_group = "";
            let mut clicked: Option<usize> = None;
            ScrollArea::vertical().id_salt("pal-scroll").auto_shrink([false, false]).show(&mut lui, |ui| {
                ui.set_width(list.width());
                if items.is_empty() {
                    ui.add_space(60.0);
                    let (r, _) = ui.allocate_exact_size(vec2(list.width(), 60.0), Sense::hover());
                    text(ui.painter(), r.center(), Align2::CENTER_CENTER, "Nothing matches — press Ctrl ↵ to ask Smart", 13.0, W::Medium, pal.ink3);
                }
                for (i, c) in items.iter().enumerate() {
                    if c.group != last_group {
                        last_group = c.group;
                        let (r, _) = ui.allocate_exact_size(vec2(list.width(), 30.0), Sense::hover());
                        crate::chrome::spaced(ui.painter(), pos2(r.left() + 14.0, r.center().y + 4.0), &c.group.to_uppercase(), 10.0, pal.ink4, 0.9);
                    }
                    let (r, _) = ui.allocate_exact_size(vec2(list.width(), 48.0), Sense::hover());
                    let resp = ui.interact(r, Id::new(("pc", i)), Sense::click());
                    let is_sel = i == sel;
                    if resp.hovered() && ctx.input(|i| i.pointer.delta().length() > 0.5) {
                        self.store.palette.sel = i;
                    }
                    let p = ui.painter();
                    if is_sel {
                        grad_rect(p, r, 14.0, with_a(pal.accent, if pal.dark { 0.24 } else { 0.14 }), with_a(pal.accent, if pal.dark { 0.16 } else { 0.08 }));
                        reflect(p, r, 14.0, if pal.dark { 0.1 } else { 0.55 }, 0.0);
                    }
                    let tile = Rect::from_min_size(r.min + vec2(8.0, 8.0), vec2(32.0, 32.0));
                    let kc = if let Some(path) = &c.path { pal.kind_color(Kind::from_ext(&ext_of(name_of(path)))) } else { pal.accent };
                    grad_rect(p, tile, 10.0, with_a(kc, if pal.dark { 0.32 } else { 0.2 }), with_a(kc, if pal.dark { 0.18 } else { 0.09 }));
                    reflect(p, tile, 10.0, if pal.dark { 0.12 } else { 0.7 }, 0.0);
                    icon(p, tile.center(), c.icon, 17.0, if c.path.is_some() { kc } else { pal.accent_ink }, is_sel);
                    let mw = r.width() - 64.0 - 120.0;
                    if c.indices.is_empty() {
                        text_fit(p, pos2(r.left() + 52.0, r.center().y - 7.0), Align2::LEFT_CENTER, &c.title, 13.5, W::Medium, pal.ink1, mw);
                    } else {
                        text_hl(p, pos2(r.left() + 52.0, r.center().y - 15.0), &c.title, 13.5, pal.ink1, pal.accent_ink, &c.indices, mw);
                    }
                    if !c.sub.is_empty() {
                        text_fit(p, pos2(r.left() + 52.0, r.center().y + 10.0), Align2::LEFT_CENTER, &c.sub, 11.5, W::Regular, pal.ink3, mw);
                    }
                    if is_sel || !c.shortcut.is_empty() {
                        let sc = if is_sel && c.shortcut.is_empty() { "↵".to_string() } else if self.mac() { c.shortcut.replace("Ctrl+", "⌘").replace("Ctrl", "⌘") } else { c.shortcut.to_string() };
                        let kw = text_w(p, &sc, 10.5, W::Medium) + 14.0;
                        let kr = Rect::from_center_size(pos2(r.right() - 14.0 - kw / 2.0, r.center().y), vec2(kw, 22.0));
                        grad_rect(p, kr, 8.0, if pal.dark { rgba(255, 255, 255, 0.12) } else { rgba(255, 255, 255, 0.98) }, if pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(244, 248, 254, 0.8) });
                        shadow(p, kr, 8.0, 0.0, 1.0, 2.0, 0.0, rgba(16, 26, 44, if pal.dark { 0.4 } else { 0.12 }));
                        text(p, kr.center(), Align2::CENTER_CENTER, &sc, 10.5, W::Medium, if is_sel { pal.accent_ink } else { pal.ink3 });
                    }
                    if is_sel && moved {
                        ui.scroll_to_rect(r, None);
                    }
                    if resp.clicked() {
                        clicked = Some(i);
                    }
                }
            });
            if let Some(i) = clicked {
                let a = items[i].action.clone();
                self.store.palette.open = false;
                self.pending.push(a);
            }
            // footer hints
            let fy = rect.bottom() - 20.0;
            p.line_segment([pos2(main.left() + 16.0, rect.bottom() - 40.0), pos2(main.right() - 16.0, rect.bottom() - 40.0)], Stroke::new(1.0, with_a(pal.ink1, 0.06)));
            let mut hx = main.left() + 22.0;
            for (k, l) in [("↑↓", "navigate"), ("↵", "run"), ("Tab", "switch rail"), ("esc", "close")] {
                let kw = text_w(&p, k, 10.0, W::Medium) + 10.0;
                let kr = Rect::from_min_size(pos2(hx, fy - 9.0), vec2(kw, 18.0));
                grad_rect(&p, kr, 6.0, if pal.dark { rgba(255, 255, 255, 0.1) } else { rgba(255, 255, 255, 0.95) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(244, 248, 254, 0.7) });
                shadow(&p, kr, 6.0, 0.0, 1.0, 1.5, 0.0, rgba(16, 26, 44, if pal.dark { 0.35 } else { 0.1 }));
                text(&p, kr.center(), Align2::CENTER_CENTER, k, 10.0, W::Medium, pal.ink3);
                let lw = text(&p, pos2(hx + kw + 6.0, fy), Align2::LEFT_CENTER, l, 11.0, W::Regular, pal.ink4).width();
                hx += kw + lw + 22.0;
            }
            // stage ------------------------------------------------------
            if stage_w > 0.0 {
                let stage = Rect::from_min_max(pos2(rect.right() - stage_w + 6.0, rect.top() + 14.0), pos2(rect.right() - 14.0, rect.bottom() - 14.0));
                grad_rect(&p, stage, 18.0, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(47, 111, 235, 0.05) }, if pal.dark { rgba(255, 255, 255, 0.02) } else { rgba(47, 111, 235, 0.02) });
                if let Some(c) = items.get(sel) {
                    self.palette_stage(ui, stage, c);
                }
            }
        });
    }

    fn palette_stage(&mut self, ui: &mut Ui, r: Rect, c: &Cmd) {
        let pal = self.pal;
        let ctx = ui.ctx().clone();
        let p = ui.painter().clone();
        let mut y = r.top() + 20.0;
        if let Some(path) = &c.path {
            let e = self.store.eng.stat(path).ok();
            let media = Rect::from_min_size(pos2(r.left() + 16.0, y), vec2(r.width() - 32.0, 130.0));
            let tex = if e.as_ref().map(|e| e.kind == Kind::Image).unwrap_or(false) { self.thumbs.get(&ctx, &self.store, path, 420) } else { None };
            elevation(&p, media, 14.0, 2, pal.dark);
            if let Some(t) = tex {
                let uv = cover_uv(media, t.size_vec2());
                fill_poly(&p, &rrect_pts(media, [14.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
            } else {
                let kc = pal.kind_color(e.as_ref().map(|e| e.kind).unwrap_or(Kind::Other));
                grad_rect(&p, media, 14.0, with_a(kc, 0.28), with_a(kc, 0.1));
                icon(&p, media.center(), c.icon, 40.0, kc, true);
            }
            reflect(&p, media, 14.0, 0.5, 0.0);
            y = media.bottom() + 16.0;
            text_fit(&p, pos2(r.left() + 18.0, y + 8.0), Align2::LEFT_CENTER, &c.title, 14.0, W::Semi, pal.ink1, r.width() - 36.0);
            y += 26.0;
            if let Some(e) = e {
                for (k, v) in [("Kind", e.kind_label()), ("Size", if e.is_dir { "—".into() } else { fmt_size(e.size) }), ("Modified", fmt_date(e.modified)), ("Where", parent(&e.path))] {
                    text(&p, pos2(r.left() + 18.0, y + 8.0), Align2::LEFT_CENTER, k, 11.5, W::Medium, pal.ink4);
                    text_fit(&p, pos2(r.left() + 78.0, y + 8.0), Align2::LEFT_CENTER, &v, 11.5, W::Medium, pal.ink2, r.width() - 96.0);
                    y += 22.0;
                }
            }
        } else {
            let tile = Rect::from_min_size(pos2(r.left() + 18.0, y), vec2(56.0, 56.0));
            shadow(&p, tile, 18.0, 0.0, 6.0, 14.0, -3.0, with_a(pal.accent, 0.4));
            grad_rect(&p, tile, 18.0, pal.accent_hover, pal.accent_press);
            reflect(&p, tile, 18.0, 0.55, 0.3);
            icon(&p, tile.center(), c.icon, 26.0, Color32::WHITE, true);
            y += 74.0;
            let g = ellipsized(&p, &c.title, 15.0, W::Semi, pal.ink1, r.width() - 36.0);
            p.galley(pos2(r.left() + 18.0, y), g, pal.ink1);
            y += 26.0;
            let desc = match &c.action {
                Action::AiAsk(_) => "Runs on-device. Results are previews — nothing changes until you apply them.".to_string(),
                Action::GoPath(_) | Action::Go(_) => "Jump straight there. Your back history keeps the way home.".to_string(),
                Action::Search(_) => "Searches the live index with fuzzy matching. Operators: kind:  size:  in:  tag:  content:".to_string(),
                _ if !c.sub.is_empty() => c.sub.clone(),
                _ => "Runs on the current selection or folder.".to_string(),
            };
            let g = rich(&p, &desc, 12.0, pal.ink3, pal.ink1, r.width() - 36.0);
            p.galley(pos2(r.left() + 18.0, y), g, pal.ink3);
        }
        // run hint
        let br = Rect::from_min_size(pos2(r.left() + 16.0, r.bottom() - 52.0), vec2(r.width() - 32.0, 36.0));
        let resp = button(ui, &pal, br, Id::new("stage-run"), Some(Icon::Forward), if matches!(c.action, Action::AiAsk(_)) { "Ask Smart  ↵" } else { "Run  ↵" }, Look::Accent, false, true);
        if resp.clicked() {
            self.store.palette.open = false;
            self.pending.push(c.action.clone());
        }
    }

    // ───────────────────────── Ask Smart panel ─────────────────────────
    pub fn ai_panel(&mut self, ui: &mut Ui, win: Rect) {
        let pal = self.pal;
        let ctx = ui.ctx().clone();
        let t = ctx.animate_bool_with_time_and_easing(Id::new("ai-t"), self.store.ai_open, 0.32, emath::easing::cubic_out);
        if t < 0.01 {
            return;
        }
        let w = 394.0f32.min(win.width() - 24.0);
        let full = Rect::from_min_max(pos2(win.right() - w - 12.0, win.top() + 68.0), pos2(win.right() - 12.0, win.bottom() - 44.0));
        let rect = full.translate(vec2((1.0 - t) * 36.0, 0.0));
        let mut pui = ui.new_child(UiBuilder::new().max_rect(rect).layout(Layout::top_down(Align::Min)));
        pui.set_opacity(t);
        let ui = &mut pui;
        let p = ui.painter().clone();
        let _ = ui.interact(rect, Id::new("ai-bg"), Sense::click_and_drag());
        elevation(&p, rect, 22.0, 4, pal.dark);
        grad_rect(&p, rect, 22.0, pal.panel_top, pal.panel_bottom);
        reflect(&p, rect, 22.0, if pal.dark { 0.12 } else { 0.95 }, 0.0);
        // header
        let tile = Rect::from_min_size(rect.min + vec2(16.0, 16.0), vec2(36.0, 36.0));
        shadow(&p, tile, 12.0, 0.0, 4.0, 10.0, -2.0, with_a(pal.accent, 0.45));
        grad_rect(&p, tile, 12.0, pal.accent_hover, pal.accent_press);
        reflect(&p, tile, 12.0, 0.55, 0.3);
        icon(&p, tile.center(), Icon::Sparkle, 19.0, Color32::WHITE, true);
        text(&p, rect.min + vec2(62.0, 26.0), Align2::LEFT_CENTER, "Ask Smart", 15.0, W::Semi, pal.ink1);
        dot(&p, rect.min + vec2(63.0, 45.0), 3.0, pal.good);
        text(&p, rect.min + vec2(72.0, 45.0), Align2::LEFT_CENTER, &format!("On-device · aware of {}", self.store.title()), 11.0, W::Regular, pal.ink3);
        let cb = Rect::from_center_size(pos2(rect.right() - 30.0, rect.top() + 34.0), vec2(30.0, 30.0));
        if button(ui, &pal, cb, Id::new("ai-close"), Some(Icon::Close), "", Look::Ghost, false, true).clicked() {
            self.act(Action::ToggleAi);
        }
        // composer
        let comp_h = 92.0;
        let comp = Rect::from_min_max(pos2(rect.left() + 14.0, rect.bottom() - comp_h - 34.0), pos2(rect.right() - 14.0, rect.bottom() - 34.0));
        grad_rect(&p, comp, 18.0, if pal.dark { rgba(0, 0, 0, 0.28) } else { rgba(20, 30, 48, 0.07) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(20, 30, 48, 0.03) });
        let id = Id::new("ai-input");
        let focused = ctx.memory(|m| m.has_focus(id));
        if focused {
            p.rect_stroke(comp.expand(1.0), 19.0, Stroke::new(2.0, with_a(pal.accent, 0.4)), StrokeKind::Outside);
        }
        let mut input = std::mem::take(&mut self.ai_input);
        let resp = ui.put(Rect::from_min_size(comp.min + vec2(10.0, 8.0), vec2(comp.width() - 20.0, 34.0)), TextEdit::singleline(&mut input).id(id).frame(false).hint_text(RichText::new("Ask about these files…").color(pal.ink3).font(font(13.0, W::Regular))).font(font(13.0, W::Medium)).text_color(pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(6, 0)));
        if self.compose_focus {
            resp.request_focus();
            self.compose_focus = false;
        }
        let send = (resp.lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter))) && !input.trim().is_empty();
        let chip = Rect::from_min_size(comp.left_bottom() + vec2(12.0, -30.0), vec2(124.0, 22.0));
        grad_rect(&p, chip, 8.0, if pal.dark { rgba(255, 255, 255, 0.1) } else { rgba(255, 255, 255, 0.95) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(244, 248, 254, 0.8) });
        shadow(&p, chip, 8.0, 0.0, 1.0, 2.0, 0.0, rgba(16, 26, 44, if pal.dark { 0.35 } else { 0.1 }));
        icon(&p, chip.left_center() + vec2(12.0, 0.0), Icon::Cube, 12.0, pal.ink3, false);
        text(&p, chip.left_center() + vec2(24.0, 0.0), Align2::LEFT_CENTER, "Smart Neura · local", 10.5, W::Medium, pal.ink3);
        let sb = Rect::from_center_size(comp.right_bottom() - vec2(24.0, 22.0), vec2(34.0, 34.0));
        let sresp = button(ui, &pal, sb, Id::new("ai-send"), Some(Icon::Send), "", Look::Accent, false, !input.trim().is_empty());
        text(&p, pos2(rect.center().x, rect.bottom() - 17.0), Align2::CENTER_CENTER, "AI actions are previews · nothing leaves this device", 10.5, W::Regular, pal.ink4);
        if send || sresp.clicked() {
            let q = input.trim().to_string();
            input.clear();
            self.act(Action::AiAsk(q));
            self.compose_focus = true;
        }
        self.ai_input = input;

        // messages
        let list = Rect::from_min_max(pos2(rect.left() + 4.0, rect.top() + 66.0), pos2(rect.right() - 4.0, comp.top() - 8.0));
        let mut mui = ui.new_child(UiBuilder::new().max_rect(list).layout(Layout::top_down(Align::Min)));
        mui.set_clip_rect(list.intersect(ui.clip_rect()));
        let msgs = self.store.msgs.clone();
        let pending = self.store.ai_pending.is_some();
        let n_msgs = msgs.len();
        ScrollArea::vertical().id_salt("ai-scroll").auto_shrink([false, false]).stick_to_bottom(true).show(&mut mui, |ui| {
            ui.set_width(list.width());
            ui.add_space(4.0);
            for (mi, m) in msgs.iter().enumerate() {
                self.ai_message(ui, mi, m, list.width());
            }
            if pending {
                let (r, _) = ui.allocate_exact_size(vec2(list.width(), 40.0), Sense::hover());
                for i in 0..3 {
                    let k = ((self.t as f32 * 6.0 - i as f32 * 0.9).sin() * 0.5 + 0.5).clamp(0.2, 1.0);
                    dot(ui.painter(), pos2(r.left() + 52.0 + i as f32 * 12.0, r.center().y), 3.5, with_a(pal.accent, k));
                }
            }
            ui.add_space(8.0);
        });
        let _ = n_msgs;
    }

    fn ai_message(&mut self, ui: &mut Ui, mi: usize, m: &smart_core::ai::AiMsg, w: f32) {
        let pal = self.pal;
        let ctx = ui.ctx().clone();
        let px = 16.0;
        if m.user {
            let g = rich(ui.painter(), &m.text, 13.0, Color32::WHITE, Color32::WHITE, w - 110.0);
            let size = g.size() + vec2(28.0, 22.0);
            let (r, _) = ui.allocate_exact_size(vec2(w, size.y + 10.0), Sense::hover());
            let b = Rect::from_min_size(pos2(r.right() - px - size.x, r.top() + 4.0), size);
            let p = ui.painter();
            shadow(p, b, 16.0, 0.0, 4.0, 10.0, -2.0, with_a(pal.accent, 0.35));
            grad_rect(p, b, 16.0, pal.accent_hover, pal.accent_press);
            reflect(p, b, 16.0, 0.45, 0.2);
            p.galley(b.min + vec2(14.0, 11.0), g, Color32::WHITE);
            return;
        }
        // assistant text
        let g = rich(ui.painter(), &m.text, 13.0, pal.ink1, pal.ink1, w - 32.0 - 36.0);
        let (r, _) = ui.allocate_exact_size(vec2(w, g.size().y + 12.0), Sense::hover());
        let p = ui.painter();
        let av = Rect::from_min_size(pos2(r.left() + px, r.top() + 4.0), vec2(22.0, 22.0));
        grad_rect(p, av, 8.0, with_a(pal.accent, if pal.dark { 0.3 } else { 0.18 }), with_a(pal.accent, if pal.dark { 0.18 } else { 0.08 }));
        icon(p, av.center(), Icon::Sparkle, 12.0, pal.accent_ink, true);
        p.galley(pos2(r.left() + px + 32.0, r.top() + 4.0), g, pal.ink1);
        // stats card
        if !m.stats.is_empty() {
            let thumbs: Vec<Path> = if mi == 0 { self.store.items.iter().filter(|e| e.kind == Kind::Image).take(3).map(|e| e.path.clone()).collect() } else { vec![] };
            let h = 20.0 + if thumbs.is_empty() { 0.0 } else { 56.0 } + m.stats.len() as f32 * 22.0 + 14.0;
            let (r, _) = ui.allocate_exact_size(vec2(w, h + 8.0), Sense::hover());
            let c = Rect::from_min_size(pos2(r.left() + px + 32.0, r.top()), vec2(w - px * 2.0 - 32.0, h));
            let (t, b) = card_colors(&pal);
            surface(ui.painter(), c, 14.0, &pal, 1, t, b);
            let mut y = c.top() + 14.0;
            if !thumbs.is_empty() {
                text(ui.painter(), pos2(c.left() + 12.0, y), Align2::LEFT_CENTER, "Awareness · current folder", 10.5, W::Medium, pal.ink3);
                y += 14.0;
                for (i, tp) in thumbs.iter().enumerate() {
                    let tr = Rect::from_min_size(pos2(c.left() + 12.0 + i as f32 * 54.0, y), vec2(50.0, 38.0));
                    if let Some(t) = self.thumbs.get(&ctx, &self.store, tp, 120) {
                        let uv = cover_uv(tr, t.size_vec2());
                        fill_poly(ui.painter(), &rrect_pts(tr, [8.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
                    }
                }
                y += 48.0;
            }
            for (ic, s) in &m.stats {
                icon(ui.painter(), pos2(c.left() + 20.0, y + 6.0), *ic, 14.0, pal.accent_ink, false);
                text_fit(ui.painter(), pos2(c.left() + 38.0, y + 6.0), Align2::LEFT_CENTER, s, 11.5, W::Medium, pal.ink2, c.width() - 52.0);
                y += 22.0;
            }
        }
        // result list
        if !m.results.is_empty() {
            for path in m.results.iter().take(6) {
                let (r, _) = ui.allocate_exact_size(vec2(w, 40.0), Sense::hover());
                let row = Rect::from_min_size(pos2(r.left() + px + 32.0, r.top()), vec2(w - px * 2.0 - 32.0, 36.0));
                let resp = ui.interact(row, Id::new(("air", mi, path)), Sense::click());
                let p = ui.painter();
                if resp.hovered() {
                    p.rect_filled(row, 10.0, with_a(pal.accent, 0.1));
                    ctx.set_cursor_icon(CursorIcon::PointingHand);
                }
                let k = Kind::from_ext(&ext_of(name_of(path)));
                let kc = pal.kind_color(k);
                let ic = Rect::from_min_size(row.min + vec2(4.0, 4.0), vec2(28.0, 28.0));
                let tex = if k == Kind::Image { self.thumbs.get(&ctx, &self.store, path, 80) } else { None };
                if let Some(t) = tex {
                    let uv = cover_uv(ic, t.size_vec2());
                    fill_poly(ui.painter(), &rrect_pts(ic, [7.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
                } else {
                    grad_rect(ui.painter(), ic, 8.0, with_a(kc, 0.25), with_a(kc, 0.1));
                    icon(ui.painter(), ic.center(), k.icon(), 14.0, kc, true);
                }
                text_fit(ui.painter(), row.min + vec2(40.0, 12.0), Align2::LEFT_CENTER, name_of(path), 12.0, W::Medium, pal.ink1, row.width() - 50.0);
                text_fit(ui.painter(), row.min + vec2(40.0, 26.0), Align2::LEFT_CENTER, &parent(path), 10.5, W::Regular, pal.ink3, row.width() - 50.0);
                if resp.clicked() {
                    self.act(Action::Open(path.clone()));
                }
            }
        }
        // plan
        if let Some(plan) = &m.plan {
            let shown = plan.ops.len().min(5);
            let h = 50.0 + shown as f32 * 22.0 + if plan.ops.len() > shown { 20.0 } else { 0.0 } + 52.0;
            let (r, _) = ui.allocate_exact_size(vec2(w, h + 10.0), Sense::hover());
            let c = Rect::from_min_size(pos2(r.left() + px + 32.0, r.top()), vec2(w - px * 2.0 - 32.0, h));
            let (t, b) = card_colors(&pal);
            surface(ui.painter(), c, 16.0, &pal, 2, t, b);
            let p = ui.painter().clone();
            icon(&p, pos2(c.left() + 20.0, c.top() + 22.0), Icon::Wand, 15.0, pal.accent_ink, true);
            text_fit(&p, pos2(c.left() + 38.0, c.top() + 22.0), Align2::LEFT_CENTER, &plan.title, 12.5, W::Semi, pal.ink1, c.width() - 50.0);
            text(&p, pos2(c.left() + 14.0, c.top() + 40.0), Align2::LEFT_CENTER, "PREVIEW", 9.5, W::Semi, pal.ink4);
            let mut y = c.top() + 62.0;
            for op in plan.ops.iter().take(shown) {
                use smart_core::ai::PlanOp::*;
                let (ic, s) = match op {
                    MkDir(d) => (Icon::NewFolder, format!("Create folder {}", name_of(d))),
                    Move { from, to_dir } => (Icon::Move, format!("{}  →  {}/", name_of(from), name_of(to_dir))),
                    Rename { from, to } => (Icon::Rename, format!("{}  →  {}", name_of(from), to)),
                    Trash(pth) => (Icon::Trash, format!("Trash {}", name_of(pth))),
                    Compress { paths, name } => (Icon::Zip, format!("Zip {} items → {}.zip", paths.len(), name)),
                };
                icon(&p, pos2(c.left() + 20.0, y), ic, 13.0, pal.ink3, false);
                text_fit(&p, pos2(c.left() + 36.0, y), Align2::LEFT_CENTER, &s, 11.5, W::Regular, pal.ink2, c.width() - 50.0);
                y += 22.0;
            }
            if plan.ops.len() > shown {
                text(&p, pos2(c.left() + 36.0, y), Align2::LEFT_CENTER, &format!("+ {} more", plan.ops.len() - shown), 11.0, W::Medium, pal.ink4);
            }
            let br = Rect::from_min_size(pos2(c.left() + 14.0, c.bottom() - 46.0), vec2(c.width() - 28.0, 34.0));
            if m.applied {
                text(&p, br.center(), Align2::CENTER_CENTER, "✓  Applied — Ctrl Z to undo", 12.5, W::Semi, pal.good);
            } else if button(ui, &pal, br, Id::new(("apply", mi)), Some(Icon::Check), "Apply changes", Look::Accent, false, true).clicked() {
                self.act(Action::AiApply(mi));
            }
        }
        // actions
        if !m.actions.is_empty() {
            let (r, _) = ui.allocate_exact_size(vec2(w, 44.0), Sense::hover());
            let mut x = r.left() + px + 32.0;
            for (i, (l, a)) in m.actions.iter().enumerate() {
                let bw = btn_width(ui.painter(), Some(Icon::Forward), l);
                let br = Rect::from_min_size(pos2(x, r.top() + 4.0), vec2(bw, 32.0));
                if button(ui, &pal, br, Id::new(("aia", mi, i)), Some(Icon::Forward), l, Look::Tint, false, true).clicked() {
                    self.act(a.clone());
                }
                x += bw + 8.0;
            }
        }
        // suggestions
        if !m.suggestions.is_empty() {
            let mut x = r_left(ui) + px + 32.0;
            let right = r_left(ui) + w - px;
            let mut y = ui.cursor().top() + 2.0;
            let start = y;
            for (i, s) in m.suggestions.iter().enumerate() {
                let bw = text_w(ui.painter(), s, 11.5, W::Medium) + 26.0;
                if x + bw > right {
                    x = r_left(ui) + px + 32.0;
                    y += 36.0;
                }
                let br = Rect::from_min_size(pos2(x, y), vec2(bw, 28.0));
                if button(ui, &pal, br, Id::new(("ais", mi, i)), None, s, Look::Plain, false, true).clicked() {
                    self.act(Action::AiAsk(s.clone()));
                }
                x += bw + 8.0;
            }
            ui.allocate_space(vec2(w, y + 40.0 - start));
        }
        ui.add_space(6.0);
    }

    // ───────────────────────── toasts / drag ─────────────────────────
    fn toasts_ui(&mut self, ctx: &Context) {
        let pal = self.pal;
        let toasts = self.store.toasts.clone();
        let screen = ctx.screen_rect();
        let base = if self.web && self.win != Rect::NOTHING { self.win.center().x } else { screen.center().x };
        let bottom = if self.web && self.win != Rect::NOTHING { self.win.bottom() - 52.0 } else { screen.bottom() - 52.0 };
        for (i, t) in toasts.iter().rev().enumerate() {
            let a_in = (t.age / 0.22).clamp(0.0, 1.0);
            let a_out = ((t.ttl - t.age) / 0.25).clamp(0.0, 1.0);
            let a = a_in.min(a_out);
            let eased = 1.0 - (1.0 - a_in).powi(3);
            let tw = text_w(&ctx.layer_painter(LayerId::background()), &t.text, 13.0, W::Medium);
            let aw = t.action.as_ref().map(|(l, _)| text_w(&ctx.layer_painter(LayerId::background()), l, 12.5, W::Semi) + 28.0).unwrap_or(0.0);
            let w = (tw + 64.0 + aw).min(560.0);
            let y = bottom - i as f32 * 52.0 + (1.0 - eased) * 16.0;
            let r = Rect::from_center_size(pos2(base, y), vec2(w, 44.0));
            Area::new(Id::new(("toast", t.id))).order(Order::Tooltip).fixed_pos(r.min).show(ctx, |ui| {
                ui.set_opacity(a);
                let (rr, _) = ui.allocate_exact_size(r.size(), Sense::hover());
                let p = ui.painter().clone();
                elevation(&p, rr, 22.0, 4, pal.dark);
                grad_rect(&p, rr, 22.0, pal.menu, lerpc(pal.menu, if pal.dark { hex(0x161e2d) } else { hex(0xf1f5fc) }, 0.6));
                reflect(&p, rr, 22.0, if pal.dark { 0.14 } else { 0.9 }, 0.0);
                icon(&p, pos2(rr.left() + 24.0, rr.center().y), t.icon, 18.0, if t.good { pal.good } else if matches!(t.icon, Icon::Warning) { pal.warn } else { pal.ink2 }, true);
                text_fit(&p, pos2(rr.left() + 44.0, rr.center().y), Align2::LEFT_CENTER, &t.text, 13.0, W::Medium, pal.ink1, w - 64.0 - aw);
                if let Some((l, act)) = &t.action {
                    let br = Rect::from_min_size(pos2(rr.right() - aw - 4.0, rr.center().y - 15.0), vec2(aw - 8.0, 30.0));
                    if button(ui, &pal, br, Id::new(("toast-a", t.id)), None, l, Look::Tint, false, true).clicked() {
                        self.pending.push(act.clone());
                    }
                }
            });
        }
    }

    fn drag_ghost(&mut self, ctx: &Context) {
        let Some(d) = &self.drag else { return };
        let Some(ptr) = ctx.input(|i| i.pointer.latest_pos()) else { return };
        let pal = self.pal;
        let paths = d.paths.clone();
        let target = self.drops.iter().rev().find(|(r, _)| r.contains(ptr)).map(|(_, t)| t.clone());
        let copy = ctx.input(|i| i.modifiers.alt) || paths.first().and_then(|p| target.as_ref().map(|t| self.store.eng.route(p) != self.store.eng.route(t))).unwrap_or(false);
        let label = match &target {
            Some(t) if !paths.iter().any(|p| is_within(t, p)) => format!("{} to {}", if copy { "Copy" } else { "Move" }, name_of(t)),
            _ => format!("{}", plural(paths.len(), "item", "items")),
        };
        ctx.set_cursor_icon(if target.is_some() { CursorIcon::Grabbing } else { CursorIcon::Grabbing });
        let first = paths.first().cloned().unwrap_or_default();
        let kind = self.store.entry(&first).map(|e| e.kind).unwrap_or(Kind::File_());
        Area::new(Id::new("ghost")).order(Order::Tooltip).fixed_pos(ptr + vec2(14.0, 12.0)).interactable(false).show(ctx, |ui| {
            let tw = text_w(ui.painter(), &label, 12.5, W::Semi);
            let (r, _) = ui.allocate_exact_size(vec2(tw + 70.0, 40.0), Sense::hover());
            let p = ui.painter().clone();
            elevation(&p, r, 20.0, 4, pal.dark);
            grad_rect(&p, r, 20.0, pal.menu, lerpc(pal.menu, pal.accent, 0.08));
            reflect(&p, r, 20.0, 0.8, 0.0);
            let kc = pal.kind_color(kind);
            for k in (0..paths.len().min(3)).rev() {
                let c = Rect::from_center_size(r.left_center() + vec2(24.0 + k as f32 * 3.0, -(k as f32) * 2.0), vec2(24.0, 28.0));
                grad_rect(&p, c, 7.0, with_a(kc, 0.35), with_a(kc, 0.15));
                if k == 0 {
                    icon(&p, c.center(), kind.icon(), 14.0, kc, true);
                }
            }
            text(&p, pos2(r.left() + 50.0, r.center().y), Align2::LEFT_CENTER, &label, 12.5, W::Semi, pal.ink1);
            if paths.len() > 1 {
                let b = Rect::from_center_size(r.left_top() + vec2(10.0, 6.0), vec2(20.0, 20.0));
                p.circle_filled(b.center(), 10.0, pal.accent);
                text(&p, b.center(), Align2::CENTER_CENTER, &paths.len().to_string(), 10.5, W::Semi, Color32::WHITE);
            }
        });
    }

    fn drop_overlay(&mut self, ctx: &Context) {
        if ctx.input(|i| i.raw.hovered_files.is_empty()) || !self.store.in_folder() {
            return;
        }
        let pal = self.pal;
        let win = if self.win == Rect::NOTHING { ctx.screen_rect() } else { self.win };
        let name = self.store.title();
        Area::new(Id::new("drop-ov")).order(Order::Tooltip).fixed_pos(win.min).interactable(false).show(ctx, |ui| {
            let (r, _) = ui.allocate_exact_size(win.size(), Sense::hover());
            let p = ui.painter().clone();
            p.rect_filled(r, 22.0, with_a(pal.accent, 0.12));
            p.rect_stroke(r.shrink(10.0), 18.0, Stroke::new(2.5, pal.accent), StrokeKind::Inside);
            let pill = Rect::from_center_size(r.center(), vec2(300.0, 56.0));
            surface(&p, pill, 28.0, &pal, 4, pal.menu, pal.menu);
            icon(&p, pill.left_center() + vec2(30.0, 0.0), Icon::Upload, 22.0, pal.accent_ink, true);
            text(&p, pill.left_center() + vec2(54.0, 0.0), Align2::LEFT_CENTER, &format!("Drop to add to “{name}”"), 14.0, W::Semi, pal.ink1);
        });
    }
}

fn r_left(ui: &Ui) -> f32 {
    ui.max_rect().left()
}

trait KindExt {
    #[allow(non_snake_case)]
    fn File_() -> Kind;
}
impl KindExt for Kind {
    fn File_() -> Kind {
        Kind::Other
    }
}
