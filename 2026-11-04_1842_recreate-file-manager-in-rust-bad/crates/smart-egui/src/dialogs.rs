//! Modal dialogs, the transfer experience, and Quick Look.
use crate::app::*;
use crate::paint::*;
use crate::theme::*;
use egui::*;
use smart_core::archive::{self, Format};
use smart_core::*;
use smart_core::Modal;

fn rot(c: Pos2, p: Vec2, a: f32) -> Pos2 {
    let (s, co) = a.sin_cos();
    c + vec2(p.x * co - p.y * s, p.x * s + p.y * co)
}
fn bez(a: Pos2, c: Pos2, b: Pos2, t: f32) -> Pos2 {
    let u = 1.0 - t;
    pos2(u * u * a.x + 2.0 * u * t * c.x + t * t * b.x, u * u * a.y + 2.0 * u * t * c.y + t * t * b.y)
}

impl App {
    fn center(&self, ctx: &Context) -> Pos2 {
        if self.web && self.win != Rect::NOTHING && !self.win_hidden { self.win.center() } else { ctx.screen_rect().center() }
    }

    /// Shell for every dialog: `content_h` is what the body actually needs — the panel wraps to it
    /// instead of reserving a fixed slab of screen.
    fn modal_shell(&mut self, ctx: &Context, id: &str, w: f32, content_h: f32, ic: Icon, title: &str, sub: &str, body: impl FnOnce(&mut App, &mut Ui, Rect)) {
        let pal = self.pal;
        let screen = ctx.screen_rect();
        let t = ctx.animate_bool_with_time(Id::new((id, "t")), true, 0.18);
        let t = spring(ctx, Id::new((id, "spr")), 1.0, ctx.input(|i| i.stable_dt), self.reduced);
        Area::new(Id::new((id, "scrim"))).order(Order::Foreground).fixed_pos(screen.min).show(ctx, |ui| {
            let (r, resp) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter().rect_filled(r, 0.0, pal.scrim.gamma_multiply(t));
            if resp.clicked() {
                self.pending.push(Action::CloseModal);
            }
        });
        let h = (content_h + 108.0).min(screen.height() - 30.0);
        let rect = Rect::from_center_size(self.center(ctx) + vec2(0.0, (1.0 - t) * 12.0), vec2(w.min(screen.width() - 30.0), h));
        Area::new(Id::new((id, "panel"))).order(Order::Foreground).fixed_pos(rect.min).show(ctx, |ui| {
            ui.set_opacity(t.min(1.0));
            let (rect, _) = ui.allocate_exact_size(rect.size(), Sense::click());
            let p = ui.painter().clone();
            elevation(&p, rect, 24.0, 5, pal.dark);
            grad_rect(&p, rect, 24.0, pal.panel_top, pal.panel_bottom);
            reflect(&p, rect, 24.0, if pal.dark { 0.12 } else { 0.95 }, 0.0);
            let tile = Rect::from_min_size(rect.min + vec2(22.0, 18.0), vec2(38.0, 38.0));
            shadow(&p, tile, 13.0, 0.0, 4.0, 10.0, -2.0, with_a(pal.accent, 0.4));
            grad_rect(&p, tile, 13.0, pal.accent_hover, pal.accent_press);
            reflect(&p, tile, 13.0, 0.55, 0.3);
            icon(&p, tile.center(), ic, 19.0, Color32::WHITE, true);
            text_fit(&p, rect.min + vec2(72.0, 29.0), Align2::LEFT_CENTER, title, 15.5, W::Semi, pal.ink1, rect.width() - 150.0);
            text_fit(&p, rect.min + vec2(72.0, 47.0), Align2::LEFT_CENTER, sub, 11.5, W::Regular, pal.ink3, rect.width() - 150.0);
            let cb = Rect::from_center_size(pos2(rect.right() - 32.0, rect.top() + 36.0), vec2(30.0, 30.0));
            if button(ui, &pal, cb, Id::new((id, "x")), Some(Icon::Close), "", Look::Ghost, false, true).clicked() {
                self.pending.push(Action::CloseModal);
            }
            body(self, ui, Rect::from_min_max(rect.min + vec2(22.0, 72.0), rect.max - vec2(22.0, 18.0)));
        });
    }

    /// Footer action row: sized from the label + padding, never from fixed widths.
    fn footer_btn(&mut self, ui: &mut Ui, right: f32, y: f32, id: &str, ic: Option<Icon>, label: &str, look: Look, enabled: bool) -> (bool, f32) {
        let w = btn_width(ui.painter(), ic, label);
        let r = Rect::from_min_size(pos2(right - w, y), vec2(w, btn_h(self.coarse)));
        let clicked = button(ui, &self.pal, r, Id::new(id), ic, label, look, false, enabled).clicked();
        (clicked, right - w - 8.0)
    }

    fn field(&mut self, ui: &mut Ui, r: Rect, id: &str, s: &mut String, hint: &str, focus: bool) -> Response {
        let pal = self.pal;
        let p = ui.painter();
        grad_rect(p, r, 14.0, if pal.dark { rgba(0, 0, 0, 0.3) } else { rgba(20, 30, 48, 0.07) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(20, 30, 48, 0.03) });
        let resp = ui.put(r.shrink2(vec2(10.0, 3.0)), TextEdit::singleline(s).id(Id::new(id)).frame(false).hint_text(RichText::new(hint).color(pal.ink3).font(font(13.5, W::Regular))).font(font(13.5, W::Medium)).text_color(pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(4, 0)));
        if focus {
            resp.request_focus();
        }
        if resp.has_focus() {
            ui.painter().rect_stroke(r.expand(1.0), 15.0, Stroke::new(2.0, with_a(pal.accent, 0.45)), StrokeKind::Outside);
        }
        resp
    }

    pub fn modals(&mut self, ctx: &Context) {
        let Some(m) = self.store.modal.clone() else { return };
        let pal = self.pal;
        match m {
            Modal::Terminal => self.terminal_ui(ctx),
            Modal::Shortcuts => {
                let rows = [("Command palette", "Ctrl K"), ("Search", "/  ·  Ctrl F"), ("Ask Smart", "Ctrl J"), ("Quick Look", "Space"), ("Open", "Enter"), ("Rename", "F2"), ("New folder", "Ctrl Shift N"), ("Copy · Cut · Paste", "Ctrl C / X / V"), ("Duplicate", "Ctrl D"), ("Select all", "Ctrl A"), ("Move to Trash", "Delete"), ("Undo", "Ctrl Z"), ("Back · Forward · Up", "Alt ← → ↑"), ("Toggle sidebar", "Ctrl \\"), ("Get info", "Ctrl I"), ("Copy instead of move (drag)", "hold Alt")];
                self.modal_shell(ctx, "m-keys", 596.0, (rows.len().div_ceil(2)) as f32 * 42.0 + 8.0, Icon::Keyboard, "Keyboard shortcuts", "Everything is reachable without the mouse", |app, ui, r| {
                    for (i, (l, k)) in rows.iter().enumerate() {
                        let col = i % 2;
                        let row = i / 2;
                        let cr = Rect::from_min_size(r.min + vec2(col as f32 * (r.width() / 2.0 + 4.0), row as f32 * 44.0), vec2(r.width() / 2.0 - 8.0, 38.0));
                        let p = ui.painter();
                        text_fit(p, cr.left_center() + vec2(4.0, 0.0), Align2::LEFT_CENTER, l, 12.5, W::Medium, app.pal.ink1, cr.width() - 120.0);
                        let k = if app.mac() { k.replace("Ctrl", "⌘") } else { k.to_string() };
                        let kw = text_w(p, &k, 10.5, W::Medium) + 16.0;
                        let kr = Rect::from_center_size(pos2(cr.right() - kw / 2.0 - 2.0, cr.center().y), vec2(kw, 24.0));
                        grad_rect(p, kr, 8.0, if app.pal.dark { rgba(255, 255, 255, 0.12) } else { rgba(255, 255, 255, 0.98) }, if app.pal.dark { rgba(255, 255, 255, 0.05) } else { rgba(244, 248, 254, 0.8) });
                        shadow(p, kr, 8.0, 0.0, 1.0, 2.0, 0.0, rgba(16, 26, 44, if app.pal.dark { 0.4 } else { 0.12 }));
                        reflect(p, kr, 8.0, 0.7, 0.0);
                        text(p, kr.center(), Align2::CENTER_CENTER, &k, 10.5, W::Medium, app.pal.ink2);
                    }
                });
            }
            Modal::About => {
                self.modal_shell(ctx, "m-about", 500.0, 214.0, Icon::Folder, "Smart Files", "Version 0.1 · native Rust · runs everywhere", |app, ui, r| {
                    let p = ui.painter().clone();
                    let lines = ["One Rust codebase: core engine + UI adapters.", "• smart-core — virtual filesystems, jobs, search, archives, assistant", "• smart-egui — this frontend (native + WebAssembly)", "", "The UI never touches storage directly: every pixel is rendered from", "a Store, every change is an Action. Swap the frontend, keep the app.", "", "Design: depth over strokes — tonal steps and an elevation ladder."];
                    for (i, l) in lines.iter().enumerate() {
                        text(&p, r.min + vec2(2.0, 12.0 + i as f32 * 24.0), Align2::LEFT_CENTER, l, 12.5, if i == 0 { W::Semi } else { W::Regular }, if i == 0 { app.pal.ink1 } else { app.pal.ink2 });
                    }
                });
            }
            Modal::Delete(paths) => {
                let n = paths.len();
                self.modal_shell(ctx, "m-del", 430.0, 86.0, Icon::Trash, "Delete permanently?", &format!("{} will be removed for good", plural(n, "item", "items")), |app, ui, r| {
                    let pal = app.pal;
                    text(ui.painter(), r.min + vec2(2.0, 14.0), Align2::LEFT_CENTER, "This can’t be undone.", 13.0, W::Medium, pal.ink2);
                    let b1 = Rect::from_min_size(pos2(r.right() - 130.0, r.bottom() - 40.0), vec2(130.0, 40.0));
                    let b0 = Rect::from_min_size(pos2(r.right() - 130.0 - 100.0, r.bottom() - 40.0), vec2(90.0, 40.0));
                    if button(ui, &pal, b0, Id::new("del-c"), None, "Cancel", Look::Plain, false, true).clicked() {
                        app.pending.push(Action::CloseModal);
                    }
                    if button(ui, &pal, b1, Id::new("del-ok"), Some(Icon::Trash), "Delete", Look::Danger, false, true).clicked() {
                        app.pending.push(Action::ConfirmDelete(paths.clone()));
                    }
                });
            }
            Modal::OpenWith(path) => {
                let e = self.store.eng.stat(&path).ok();
                let apps = e.as_ref().map(|e| self.store.eng.apps_for(e)).unwrap_or_default();
                let name = name_of(&path).to_string();
                self.modal_shell(ctx, "m-ow", 452.0, apps.len() as f32 * 52.0 + 6.0, Icon::App, "Open with…", &name, |app, ui, r| {
                    let pal = app.pal;
                    for (i, a) in apps.iter().enumerate() {
                        let row = Rect::from_min_size(r.min + vec2(0.0, i as f32 * 54.0), vec2(r.width(), 48.0));
                        let resp = ui.interact(row, Id::new(("ow", i)), Sense::click());
                        let p = ui.painter();
                        if resp.hovered() {
                            grad_rect(p, row, 14.0, with_a(pal.accent, 0.16), with_a(pal.accent, 0.09));
                            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                        }
                        let t = Rect::from_min_size(row.min + vec2(8.0, 6.0), vec2(36.0, 36.0));
                        grad_rect(p, t, 11.0, with_a(pal.accent, 0.22), with_a(pal.accent, 0.1));
                        icon(p, t.center(), a.icon, 18.0, pal.accent_ink, true);
                        text(p, row.min + vec2(56.0, 24.0), Align2::LEFT_CENTER, &a.name, 13.5, W::Medium, pal.ink1);
                        if a.default {
                            text(p, pos2(row.right() - 12.0, row.center().y), Align2::RIGHT_CENTER, "default", 11.5, W::Medium, pal.ink3);
                        }
                        if resp.clicked() {
                            app.pending.push(Action::OpenWith(path.clone(), a.id.clone()));
                        }
                    }
                });
            }
            Modal::Compress { paths, name, fmt } => {
                let total: u64 = paths.iter().filter_map(|p| self.store.eng.stat(p).ok()).map(|e| if e.is_dir { self.store.eng.dir_size(&e.path) } else { e.size }).sum();
                self.modal_shell(ctx, "m-zip", 512.0, 262.0, Icon::Zip, "Compress", &format!("{} · {}", plural(paths.len(), "item", "items"), fmt_size(total)), |app, ui, r| {
                    let pal = app.pal;
                    text(ui.painter(), r.min + vec2(2.0, 8.0), Align2::LEFT_CENTER, "NAME", 10.0, W::Semi, pal.ink4);
                    let mut nm = name.clone();
                    let fr = Rect::from_min_size(r.min + vec2(0.0, 24.0), vec2(r.width() - 90.0, 44.0));
                    let resp = app.field(ui, fr, "zip-name", &mut nm, "Archive name", !ui.ctx().memory(|m| m.has_focus(Id::new("zip-name"))) && ui.ctx().memory(|m| m.focused().is_none()));
                    text(ui.painter(), pos2(fr.right() + 12.0, fr.center().y), Align2::LEFT_CENTER, &format!(".{}", fmt.ext()), 13.5, W::Medium, pal.ink3);
                    text(ui.painter(), r.min + vec2(2.0, 96.0), Align2::LEFT_CENTER, "FORMAT", 10.0, W::Semi, pal.ink4);
                    let mut nf = fmt;
                    for (i, f) in Format::ALL.iter().enumerate() {
                        let w = (r.width() - 20.0) / 3.0;
                        let c = Rect::from_min_size(r.min + vec2(i as f32 * (w + 10.0), 112.0), vec2(w, 92.0));
                        let resp = ui.interact(c, Id::new(("fmt", i)), Sense::click());
                        let on = *f == fmt;
                        let p = ui.painter();
                        if on {
                            shadow(p, c, 16.0, 0.0, 4.0, 12.0, -3.0, with_a(pal.accent, 0.35));
                            grad_rect(p, c, 16.0, with_a(pal.accent, if pal.dark { 0.32 } else { 0.2 }), with_a(pal.accent, if pal.dark { 0.2 } else { 0.1 }));
                            p.rect_stroke(c, 16.0, Stroke::new(2.0, pal.accent), StrokeKind::Inside);
                        } else {
                            let (t, b) = card_colors(&pal);
                            surface(p, c, 16.0, &pal, 1, t, b);
                        }
                        icon(p, c.center_top() + vec2(0.0, 26.0), Icon::Zip, 24.0, if on { pal.accent_ink } else { pal.ink2 }, on);
                        text(p, c.center_top() + vec2(0.0, 52.0), Align2::CENTER_CENTER, f.label(), 13.0, W::Semi, if on { pal.accent_ink } else { pal.ink1 });
                        text(p, c.center_top() + vec2(0.0, 72.0), Align2::CENTER_CENTER, f.hint(), 10.5, W::Regular, pal.ink3);
                        if resp.clicked() {
                            nf = *f;
                        }
                    }
                    text(ui.painter(), r.min + vec2(2.0, 232.0), Align2::LEFT_CENTER, &format!("Pure-Rust encoders · reads zip, tar, tar.gz, tar.xz, tar.bz2, gz, xz, bz2 back", ), 11.5, W::Regular, pal.ink3);
                    if let Some(Modal::Compress { name: n2, fmt: f2, .. }) = &mut app.store.modal {
                        *n2 = nm.clone();
                        *f2 = nf;
                    }
                    let go = Rect::from_min_size(pos2(r.right() - 170.0, r.bottom() - 40.0), vec2(170.0, 40.0));
                    let c0 = Rect::from_min_size(pos2(r.right() - 170.0 - 100.0, r.bottom() - 40.0), vec2(90.0, 40.0));
                    let enter = resp.lost_focus() && ui.ctx().input(|i| i.key_pressed(Key::Enter));
                    if button(ui, &pal, c0, Id::new("zip-c"), None, "Cancel", Look::Plain, false, true).clicked() {
                        app.pending.push(Action::CloseModal);
                    }
                    if button(ui, &pal, go, Id::new("zip-go"), Some(Icon::Zip), "Create archive", Look::Accent, false, !nm.trim().is_empty()).clicked() || (enter && !nm.trim().is_empty()) {
                        app.pending.push(Action::CompressNow { name: nm.clone(), fmt: nf });
                    }
                });
            }
            Modal::AddCloud { provider, account } => {
                self.modal_shell(ctx, "m-cloud", 576.0, 360.0, Icon::Cloud, "Connect cloud storage", "Mounted like any disk — drag, copy, search and index across clouds", |app, ui, r| {
                    let pal = app.pal;
                    let icons = [Icon::Cloud, Icon::Globe, Icon::Layers, Icon::Cloud, Icon::Cube, Icon::Link];
                    let mut np = provider;
                    for (i, name) in CLOUD_PROVIDERS.iter().enumerate() {
                        let w = (r.width() - 20.0) / 3.0;
                        let c = Rect::from_min_size(r.min + vec2((i % 3) as f32 * (w + 10.0), (i / 3) as f32 * 94.0), vec2(w, 84.0));
                        let resp = ui.interact(c, Id::new(("prov", i)), Sense::click());
                        let on = i == provider;
                        let p = ui.painter();
                        if on {
                            shadow(p, c, 16.0, 0.0, 4.0, 12.0, -3.0, with_a(pal.accent, 0.35));
                            grad_rect(p, c, 16.0, with_a(pal.accent, if pal.dark { 0.3 } else { 0.18 }), with_a(pal.accent, if pal.dark { 0.18 } else { 0.09 }));
                            p.rect_stroke(c, 16.0, Stroke::new(2.0, pal.accent), StrokeKind::Inside);
                        } else {
                            let (t, b) = card_colors(&pal);
                            surface(p, c, 16.0, &pal, 1, t, b);
                        }
                        icon(p, c.center_top() + vec2(0.0, 28.0), icons[i], 24.0, if on { pal.accent_ink } else { pal.ink2 }, on);
                        text_fit(p, c.center_top() + vec2(0.0, 58.0), Align2::CENTER_CENTER, name, 12.0, W::Semi, if on { pal.accent_ink } else { pal.ink1 }, w - 16.0);
                        if resp.clicked() {
                            np = i;
                        }
                    }
                    text(ui.painter(), r.min + vec2(2.0, 214.0), Align2::LEFT_CENTER, "ACCOUNT", 10.0, W::Semi, pal.ink4);
                    let mut acc = account.clone();
                    let fr = Rect::from_min_size(r.min + vec2(0.0, 230.0), vec2(r.width(), 44.0));
                    app.field(ui, fr, "cloud-acct", &mut acc, "you@example.com  (optional in demo)", false);
                    if let Some(Modal::AddCloud { provider: p2, account: a2 }) = &mut app.store.modal {
                        *p2 = np;
                        *a2 = acc;
                    }
                    let lines = ["Connectors implement the same `Vfs` trait as your disk, so S3, WebDAV,", "Drive, Dropbox and OneDrive plug in without touching the UI. This build ships a", "simulated provider — swap in an OpenDAL/WebDAV backend in smart-core."];
                    for (i, l) in lines.iter().enumerate() {
                        text(ui.painter(), r.min + vec2(2.0, 300.0 + i as f32 * 18.0), Align2::LEFT_CENTER, l, 11.5, W::Regular, pal.ink3);
                    }
                    let go = Rect::from_min_size(pos2(r.right() - 150.0, r.bottom() - 40.0), vec2(150.0, 40.0));
                    if button(ui, &pal, go, Id::new("cloud-go"), Some(Icon::Plug), "Connect", Look::Accent, false, true).clicked() {
                        app.pending.push(Action::AddCloud);
                    }
                });
            }
            Modal::Properties(path) => self.properties(ctx, &path),
        }
        let _ = pal;
    }

    fn properties(&mut self, ctx: &Context, path: &str) {
        let Ok(e) = self.store.eng.stat(path) else {
            self.store.modal = None;
            return;
        };
        let mount = self.store.eng.mounts.iter().find(|m| m.root == path).cloned();
        let size = if e.is_dir { self.store.eng.dir_size(path) } else { e.size };
        let is_arch = archive::is_archive(&e.name);
        let listing: Vec<(String, u64, bool)> = if is_arch && e.size < 64_000_000 {
            let key = Id::new(("arc", path));
            if let Some(v) = ctx.data(|d| d.get_temp::<Vec<(String, u64, bool)>>(key)) {
                v
            } else {
                let v: Vec<(String, u64, bool)> = self.store.eng.read_all(path, 64_000_000).ok().and_then(|b| archive::list(b, &e.name).ok()).map(|l| l.into_iter().map(|i| (i.path, i.size, i.is_dir)).collect()).unwrap_or_default();
                ctx.data_mut(|d| d.insert_temp(key, v.clone()));
                v
            }
        } else {
            vec![]
        };
        let tag = self.store.eng.tags.get(path).cloned();
        let dims = if e.kind == Kind::Image { self.store.eng.read_head(path, 4096).ok().and_then(|b| imagesize::blob_size(&b).ok()).map(|s| format!("{} × {}", s.width, s.height)) } else { None };
        let h = 322.0 + if mount.is_some() { 40.0 } else { 0.0 } + (listing.len().min(6) as f32) * 24.0 + if is_arch { 38.0 } else { 0.0 } + if dims.is_some() { 26.0 } else { 0.0 } + if e.kind == Kind::Image { 132.0 } else { 0.0 };
        let thumb = if e.kind == Kind::Image { self.thumbs.get(ctx, &self.store, path, 300) } else { None };
        let ename = e.name.clone();
        let sub = if let Some(m) = &mount { m.detail.clone() } else { e.kind_label() };
        let ic = mount.as_ref().map(|m| m.icon()).unwrap_or_else(|| e.kind.icon());
        let path_s = path.to_string();
        self.modal_shell(ctx, "m-info", 528.0, h, ic, &ename, &sub, |app, ui, r| {
            let pal = app.pal;
            let mut y = r.top();
            if let Some(t) = &thumb {
                let tr = Rect::from_min_size(pos2(r.left(), y), vec2(r.width(), 120.0));
                let uv = cover_uv(tr, t.size_vec2());
                elevation(ui.painter(), tr, 14.0, 2, pal.dark);
                fill_poly(ui.painter(), &rrect_pts(tr, [14.0; 4]), &|_| Color32::WHITE, Some((t.id(), &uv)));
                y += 134.0;
            }
            let mut rows: Vec<(&str, String)> = vec![("Kind", e.kind_label()), ("Size", if size == 0 && e.is_dir { "Calculating…".into() } else { format!("{}  ({} bytes)", fmt_size(size), size) })];
            if let Some(d) = dims {
                rows.push(("Dimensions", d));
            }
            if e.is_dir {
                rows.push(("Contains", e.children.map(|c| plural(c as usize, "item", "items")).unwrap_or_default()));
            }
            rows.push(("Where", parent(&path_s)));
            rows.push(("Modified", fmt_full_date(e.modified)));
            if let Some(m) = &mount {
                rows.push(("Capacity", if m.total > 0 { format!("{} free of {}", fmt_size(m.free), fmt_size(m.total)) } else { "—".into() }));
            }
            for (k, v) in rows {
                text(ui.painter(), pos2(r.left() + 2.0, y + 14.0), Align2::LEFT_CENTER, k, 12.0, W::Medium, pal.ink4);
                text_fit(ui.painter(), pos2(r.left() + 110.0, y + 14.0), Align2::LEFT_CENTER, &v, 12.5, W::Medium, pal.ink1, r.width() - 120.0);
                y += 28.0;
            }
            if let Some(m) = &mount {
                progress_bar(ui.painter(), Rect::from_min_size(pos2(r.left() + 110.0, y - 8.0), vec2(r.width() - 120.0, 6.0)), &pal, m.used_frac());
                y += 14.0;
            }
            if is_arch {
                text(ui.painter(), pos2(r.left() + 2.0, y + 14.0), Align2::LEFT_CENTER, &format!("CONTENTS · {}", plural(listing.len(), "entry", "entries")), 10.0, W::Semi, pal.ink4);
                y += 30.0;
                for (n, s, d) in listing.iter().take(6) {
                    icon(ui.painter(), pos2(r.left() + 14.0, y + 10.0), if *d { Icon::Folder } else { Icon::File }, 14.0, pal.ink3, false);
                    text_fit(ui.painter(), pos2(r.left() + 34.0, y + 10.0), Align2::LEFT_CENTER, n, 12.0, W::Medium, pal.ink2, r.width() - 130.0);
                    text(ui.painter(), pos2(r.right() - 4.0, y + 10.0), Align2::RIGHT_CENTER, &fmt_size(*s), 11.5, W::Regular, pal.ink3);
                    y += 24.0;
                }
            }
            // tags
            text(ui.painter(), pos2(r.left() + 2.0, y + 18.0), Align2::LEFT_CENTER, "Tag", 12.0, W::Medium, pal.ink4);
            for (i, t) in TAGS.iter().enumerate() {
                let c = pos2(r.left() + 122.0 + i as f32 * 34.0, y + 18.0);
                let rr = Rect::from_center_size(c, vec2(26.0, 26.0));
                let resp = ui.interact(rr, Id::new(("ptag", i)), Sense::click());
                let on = tag.as_deref() == Some(t.id);
                let col = Color32::from_rgb(t.rgb[0], t.rgb[1], t.rgb[2]);
                let p = ui.painter();
                if on {
                    p.circle_stroke(c, 11.5, Stroke::new(2.0, col));
                }
                shadow(p, Rect::from_center_size(c, vec2(14.0, 14.0)), 7.0, 0.0, 2.0, 5.0, -1.0, with_a(col, 0.5));
                p.circle_filled(c, 7.0, col);
                resp.clone().on_hover_text(t.label);
                if resp.clicked() {
                    app.store.eng.set_tag(&path_s, if on { None } else { Some(t.id) });
                }
            }
        });
    }

    // ───────────────────────── the transfer experience ─────────────────────────
    pub fn transfer_dialog(&mut self, ctx: &Context) {
        let Some(j) = self.store.transfer_job() else { return };
        // snapshot — the dialog never mutates engine state directly, it dispatches actions
        let id = j.id;
        let state = j.state;
        let prog = j.progress();
        let (done_b, total_b, speed, eta) = (j.done_bytes, j.total_bytes, j.speed, j.eta());
        let verb = j.verb();
        let title = j.title.clone();
        let cur_name = j.current_name();
        let (items_done, items_total) = (j.items_done(), j.plan.len());
        let speeds = j.speeds.clone();
        let elapsed = j.elapsed;
        let src_dir = j.src_dir.clone();
        let dest = j.dest.clone();
        let kinds: Vec<Kind> = (0..7).map(|k| if j.plan.is_empty() { Kind::Other } else { j.plan[(j.cur + k) % j.plan.len()].kind }).collect();
        let conflict = j.conflict.clone();
        let err = j.error.clone();
        let kind = j.kind;
        let pal = self.pal;
        let screen = ctx.screen_rect();
        let done = state == JobState::Done;
        let failed = state == JobState::Failed;
        let dest_mount = self.store.eng.mount_of(&dest).cloned();
        let dest_icon = dest_mount.as_ref().filter(|m| m.root == dest || m.kind != MountKind::Internal).map(|m| m.icon()).unwrap_or(Icon::Folder);
        let t_in = ctx.animate_bool_with_time(Id::new(("tr-in", id)), true, 0.25);
        let w = 600.0f32.min(screen.width() - 30.0);
        let h = (392.0f32 + if self.tr_graph { 58.0 } else { 0.0 } + if conflict.is_some() { 112.0 } else { 0.0 }).min(screen.height() - 30.0);
        Area::new(Id::new("tr-scrim")).order(Order::Foreground).fixed_pos(screen.min).show(ctx, |ui| {
            let (r, _) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter().rect_filled(r, 0.0, pal.scrim.gamma_multiply(t_in * 0.8));
        });
        let rect = Rect::from_center_size(self.center(ctx) + vec2(0.0, (1.0 - t_in) * 24.0), vec2(w, h));
        let t = self.t as f32;
        Area::new(Id::new("tr-panel")).order(Order::Foreground).fixed_pos(rect.min).show(ctx, |ui| {
            ui.set_opacity(t_in);
            let (rect, _) = ui.allocate_exact_size(rect.size(), Sense::click());
            let p = ui.painter().clone();
            elevation(&p, rect, 28.0, 5, pal.dark);
            grad_rect(&p, rect, 28.0, pal.panel_top, pal.panel_bottom);
            reflect(&p, rect, 28.0, if pal.dark { 0.12 } else { 0.95 }, 0.0);
            // header
            let head = if done { format!("{} complete", match kind { JobKind::Copy => "Copy", JobKind::Move => "Move", JobKind::Compress => "Compression", JobKind::Extract => "Extraction", JobKind::Delete => "Delete" }) } else if failed { "Transfer failed".to_string() } else { format!("{} {}", verb, title) };
            text_fit(&p, rect.min + vec2(28.0, 34.0), Align2::LEFT_CENTER, &head, 16.5, W::Semi, pal.ink1, w - 120.0);
            let sub = if done { format!("{} in {:.0} s", fmt_size(total_b), elapsed.max(0.1)) } else if let Some(e) = &err { e.clone() } else if state == JobState::Paused { "Paused".to_string() } else { format!("{} of {} items  ·  {}", items_done.min(items_total), items_total, if cur_name.is_empty() { "preparing…".to_string() } else { cur_name.clone() }) };
            text_fit(&p, rect.min + vec2(28.0, 56.0), Align2::LEFT_CENTER, &sub, 12.0, W::Regular, if failed { pal.danger } else { pal.ink3 }, w - 120.0);
            // ── the scene
            let s = Rect::from_min_size(rect.min + vec2(20.0, 78.0), vec2(w - 40.0, 168.0));
            let sky_a = if pal.dark { rgba(111, 155, 255, 0.16) } else { rgba(47, 111, 235, 0.10) };
            let sky_b = if pal.dark { rgba(111, 155, 255, 0.04) } else { rgba(47, 111, 235, 0.025) };
            grad_rect(&p, s, 22.0, sky_a, sky_b);
            // speed streaks
            let sp = (speed / 300e6).clamp(0.0, 1.4) as f32;
            if state == JobState::Running {
                for k in 0..9 {
                    let ph = (t * (0.25 + sp * 0.55) + k as f32 * 0.137).fract();
                    let y = s.top() + 22.0 + ((k * 53) % 150) as f32;
                    let x = s.left() + 20.0 + ph * (s.width() - 40.0);
                    p.line_segment([pos2(x - 26.0 * (0.4 + sp), y), pos2(x, y)], Stroke::new(1.5, with_a(pal.accent, 0.10 * (1.0 - (ph - 0.5).abs() * 2.0))));
                }
            }
            let a = pos2(s.left() + 78.0, s.center().y + 6.0);
            let b = pos2(s.right() - 78.0, s.center().y + 6.0);
            // flight arc (soft dotted rail)
            let ctl = pos2((a.x + b.x) / 2.0, s.top() - 22.0);
            for i in 0..28 {
                let q = bez(a + vec2(34.0, -6.0), ctl, b - vec2(34.0, 6.0), i as f32 / 27.0);
                p.circle_filled(q, 1.6, with_a(pal.accent, 0.20));
            }
            // pulses from flight phases
            let rate = if state == JobState::Running { 0.30 + sp * 0.50 } else { 0.0 };
            let mut land: f32 = 0.0;
            let mut launch: f32 = 0.0;
            let n = 6;
            let phases: Vec<f32> = (0..n).map(|k| ((self.t as f32 * rate) + k as f32 / n as f32).fract()).collect();
            for ph in &phases {
                land = land.max((1.0 - ((*ph - 0.965).abs() * 22.0)).max(0.0));
                launch = launch.max((1.0 - ((*ph - 0.03).abs() * 22.0)).max(0.0));
            }
            // source dock
            let src = Rect::from_center_size(a, vec2(84.0 * (1.0 - 0.04 * launch), 84.0 * (1.0 - 0.04 * launch)));
            let (ct, cb) = card_colors(&pal);
            surface(&p, src, 26.0, &pal, 3, ct, cb);
            icon(&p, src.center(), Icon::Folder, 44.0, with_a(pal.accent, 0.9), true);
            text_fit(&p, pos2(a.x, src.bottom() + 16.0), Align2::CENTER_CENTER, if src_dir.is_empty() { "Source" } else { name_of(&src_dir) }, 12.0, W::Semi, pal.ink2, 120.0);
            // destination dock with liquid
            let dz = 1.0 + 0.05 * land;
            let dst = Rect::from_center_size(b, vec2(84.0 * dz, 84.0 * dz));
            surface(&p, dst, 26.0, &pal, 3, ct, cb);
            let inner = dst.shrink(5.0);
            let fh = inner.height() * if done { 1.0 } else { prog };
            if fh > 1.0 {
                let top = inner.bottom() - fh;
                let wave = |x: f32| (x * 0.16 + t * 3.0).sin() * 2.5 * (1.0 - prog).min(1.0).max(0.2);
                let mut pts: Vec<Pos2> = vec![];
                let steps = 16;
                for i in 0..=steps {
                    let x = inner.left() + inner.width() * i as f32 / steps as f32;
                    pts.push(pos2(x, (top + wave(x)).max(inner.top())));
                }
                pts.push(pos2(inner.right(), inner.bottom() - 12.0));
                for i in 0..=6 {
                    let a = std::f32::consts::FRAC_PI_2 * i as f32 / 6.0;
                    pts.push(pos2(inner.right() - 12.0 + a.cos() * 12.0, inner.bottom() - 12.0 + a.sin() * 12.0));
                }
                for i in 0..=6 {
                    let a = std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_2 * i as f32 / 6.0;
                    pts.push(pos2(inner.left() + 12.0 + a.cos() * 12.0, inner.bottom() - 12.0 + a.sin() * 12.0));
                }
                let (c1, c2) = (with_a(pal.accent_hover, 0.85), with_a(pal.accent, 0.95));
                let hh = inner.height();
                fill_poly(&p, &pts, &|q| lerpc(c1, c2, (q.y - inner.top()) / hh), None);
            }
            icon(&p, dst.center(), dest_icon, 40.0, if fh > inner.height() * 0.45 { Color32::WHITE } else { with_a(pal.accent, 0.85) }, true);
            let dname = dest_mount.as_ref().filter(|m| m.root == dest).map(|m| m.label.clone()).unwrap_or_else(|| name_of(&dest).to_string());
            text_fit(&p, pos2(b.x, dst.bottom() + 16.0), Align2::CENTER_CENTER, &dname, 12.0, W::Semi, pal.ink2, 120.0);
            // landing ripple
            if land > 0.0 && state == JobState::Running {
                p.circle_stroke(b, 44.0 + (1.0 - land) * 26.0, Stroke::new(2.0, with_a(pal.accent, 0.35 * land)));
            }
            // flights
            if state == JobState::Running || state == JobState::Paused {
                for (k, ph) in phases.iter().enumerate() {
                    let ph = if state == JobState::Paused { (k as f32 / n as f32 + 0.1).fract() } else { *ph };
                    let e = ph * ph * (3.0 - 2.0 * ph);
                    let pos = bez(a + vec2(34.0, -6.0), ctl, b - vec2(34.0, 6.0), e);
                    let ahead = bez(a + vec2(34.0, -6.0), ctl, b - vec2(34.0, 6.0), (e + 0.02).min(1.0));
                    let ang = (ahead.y - pos.y).atan2(ahead.x - pos.x) * 0.55;
                    let scale = 0.55 + 0.55 * (ph * std::f32::consts::PI).sin();
                    let alpha = (ph / 0.08).min(1.0).min(((1.0 - ph) / 0.1).min(1.0));
                    // trail
                    for tt in 1..=4 {
                        let q = bez(a + vec2(34.0, -6.0), ctl, b - vec2(34.0, 6.0), (e - tt as f32 * 0.03).max(0.0));
                        p.circle_filled(q, 2.6 - tt as f32 * 0.4, with_a(pal.accent, 0.25 * alpha * (1.0 - tt as f32 / 5.0)));
                    }
                    let kind = kinds[k % kinds.len()];
                    let kc = pal.kind_color(kind);
                    let half = vec2(13.0, 16.5) * scale;
                    let base = Rect::from_center_size(Pos2::ZERO, half * 2.0);
                    let pts: Vec<Pos2> = rrect_pts(base, [5.0 * scale; 4]).into_iter().map(|q| rot(pos, q.to_vec2(), ang)).collect();
                    let sh: Vec<Pos2> = pts.iter().map(|q| *q + vec2(0.0, 5.0 * scale)).collect();
                    fill_poly(&p, &sh, &|_| rgba(10, 20, 40, 0.16 * alpha), None);
                    fill_poly(&p, &pts, &|q| lerpc(rgba(255, 255, 255, alpha), rgba(236, 242, 252, alpha), ((q.y - pos.y) / (half.y * 2.0) + 0.5).clamp(0.0, 1.0)), None);
                    // folded corner
                    let fc = rot(pos, vec2(half.x, -half.y), ang);
                    p.add(Shape::convex_polygon(vec![fc, rot(pos, vec2(half.x - 7.0 * scale, -half.y), ang), rot(pos, vec2(half.x, -half.y + 7.0 * scale), ang)], rgba(150, 170, 200, 0.55 * alpha), Stroke::NONE));
                    icon(&p, pos + vec2(0.0, 1.0), kind.icon(), 13.0 * scale, with_a(kc, alpha), true);
                }
            }
            // done burst
            if done {
                let pop = ctx.animate_bool_with_time(Id::new(("done-pop", id)), true, 0.5);
                let c = b + vec2(30.0, -34.0);
                let rr = 16.0 * (0.4 + 0.6 * (1.0 - (1.0 - pop).powi(3)));
                shadow(&p, Rect::from_center_size(c, vec2(rr * 1.4, rr * 1.4)), rr, 0.0, 3.0, 10.0, -2.0, with_a(pal.good, 0.5));
                p.circle_filled(c, rr, pal.good);
                icon(&p, c, Icon::Check, rr * 1.1, Color32::WHITE, true);
                for k in 0..10 {
                    let ang = k as f32 / 10.0 * std::f32::consts::TAU + 0.3;
                    let d = 22.0 + 28.0 * pop;
                    p.circle_filled(c + vec2(ang.cos(), ang.sin()) * d, 2.4 * (1.0 - pop * 0.7), with_a(pal.good, 0.8 * (1.0 - pop)));
                }
            }
            // ── stats (throughput graph is folded away by default: the dialog stays short)
            let y0 = s.bottom() + 8.0;
            let pct = if done { 100.0 } else { prog * 100.0 };
            let pct_s = format!("{:.0}%", pct);
            text(&p, pos2(rect.left() + 30.0, y0 + 16.0), Align2::LEFT_CENTER, &pct_s, 30.0, W::Semi, pal.ink1);
            let pw = text_w(&p, &pct_s, 30.0, W::Semi);
            text(&p, pos2(rect.left() + 42.0 + pw, y0 + 22.0), Align2::LEFT_CENTER, &format!("{} of {}", fmt_size(done_b.min(total_b)), fmt_size(total_b)), 12.5, W::Medium, pal.ink3);
            if !done {
                text(&p, pos2(rect.right() - 30.0, y0 + 10.0), Align2::RIGHT_CENTER, &fmt_speed(speed), 14.0, W::Semi, pal.ink1);
                text(&p, pos2(rect.right() - 30.0, y0 + 28.0), Align2::RIGHT_CENTER, &if state == JobState::Paused { "paused".to_string() } else { fmt_eta(eta) }, 11.5, W::Medium, pal.ink3);
            }
            progress_bar(&p, Rect::from_min_size(pos2(rect.left() + 30.0, y0 + 40.0), vec2(w - 60.0, 8.0)), &pal, if done { 1.0 } else { prog });
            let gbtn = Rect::from_min_size(pos2(rect.left() + 22.0, y0 + 54.0), vec2(214.0, 24.0));
            let gresp = ui.interact(gbtn, Id::new(("tr-graph", id)), Sense::click());
            if gresp.hovered() {
                p.rect_filled(gbtn, 8.0, with_a(pal.ink1, 0.06));
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            icon(&p, pos2(gbtn.left() + 12.0, gbtn.center().y), Icon::Wave, 13.0, pal.ink3, false);
            text(&p, pos2(gbtn.left() + 26.0, gbtn.center().y), Align2::LEFT_CENTER, &format!("Throughput · {}", fmt_speed(speed)), 11.0, W::Medium, pal.ink3);
            icon(&p, pos2(gbtn.right() - 12.0, gbtn.center().y), if self.tr_graph { Icon::ChevronDown } else { Icon::ChevronRight }, 11.0, pal.ink3, false);
            if gresp.clicked() {
                self.tr_graph = !self.tr_graph;
            }
            if self.tr_graph {
                let sp_r = Rect::from_min_size(pos2(rect.left() + 30.0, y0 + 84.0), vec2(w - 60.0, 46.0));
                grad_rect(&p, sp_r, 12.0, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(20, 30, 48, 0.04) }, if pal.dark { rgba(255, 255, 255, 0.02) } else { rgba(20, 30, 48, 0.015) });
                if speeds.len() > 2 {
                    let mx = speeds.iter().cloned().fold(1.0f32, f32::max) * 1.15;
                    let pts: Vec<Pos2> = speeds
                        .iter()
                        .enumerate()
                        .map(|(i, v)| pos2(sp_r.left() + 8.0 + (sp_r.width() - 16.0) * i as f32 / 79.0, sp_r.bottom() - 6.0 - (sp_r.height() - 14.0) * (v / mx)))
                        .collect();
                    let sc = with_a(pal.accent, 0.26);
                    let sc2 = with_a(pal.accent, 0.02);
                    for pair in pts.windows(2) {
                        let (p0, p1) = (pair[0], pair[1]);
                        let mut m = epaint::Mesh::default();
                        let bot = sp_r.bottom() - 4.0;
                        m.colored_vertex(p0, sc);
                        m.colored_vertex(p1, sc);
                        m.colored_vertex(pos2(p1.x, bot), sc2);
                        m.colored_vertex(pos2(p0.x, bot), sc2);
                        m.add_triangle(0, 1, 2);
                        m.add_triangle(0, 2, 3);
                        p.add(Shape::mesh(m));
                    }
                    p.add(Shape::line(pts.clone(), Stroke::new(2.0, pal.accent)));
                    if let Some(l) = pts.last() {
                        p.circle_filled(*l, 3.5, pal.accent);
                        p.circle_stroke(*l, 6.5, Stroke::new(2.0, with_a(pal.accent, 0.25)));
                    }
                } else {
                    text(&p, sp_r.center(), Align2::CENTER_CENTER, "measuring throughput…", 11.0, W::Medium, pal.ink4);
                }
            }
            // ── actions (widths come from the label + padding, matching every other control)
            let bh = btn_h(self.coarse);
            let by = rect.bottom() - 20.0 - bh;
            if let Some(name) = &conflict {
                let cr = Rect::from_min_max(pos2(rect.left() + 20.0, rect.bottom() - 188.0), pos2(rect.right() - 20.0, rect.bottom() - 16.0));
                surface(&p, cr, 20.0, &pal, 4, pal.menu, lerpc(pal.menu, pal.warn, 0.07));
                icon(&p, cr.min + vec2(30.0, 32.0), Icon::Warning, 20.0, pal.warn, true);
                text_fit(&p, cr.min + vec2(52.0, 26.0), Align2::LEFT_CENTER, &format!("“{name}” already exists"), 14.0, W::Semi, pal.ink1, cr.width() - 70.0);
                text(&p, cr.min + vec2(52.0, 46.0), Align2::LEFT_CENTER, "Same-named files are waiting in the destination.", 11.5, W::Regular, pal.ink3);
                let all_id = Id::new("conflict-all");
                let mut all: bool = ctx.data(|d| d.get_temp(all_id)).unwrap_or(false);
                let cb = ui.interact(Rect::from_min_size(cr.min + vec2(18.0, 62.0), vec2(180.0, 24.0)), Id::new("c-all"), Sense::click());
                crate::content::checkbox(&p, cr.min + vec2(28.0, 74.0), if all { 1 } else { 0 }, &pal);
                text(&p, cr.min + vec2(44.0, 74.0), Align2::LEFT_CENTER, "Apply to all", 12.0, W::Medium, pal.ink2);
                if cb.clicked() {
                    all = !all;
                    ctx.data_mut(|d| d.insert_temp(all_id, all));
                }
                let mut x = cr.right() - 14.0;
                for (i, (l, r_, look)) in [("Replace", Resolution::Replace, Look::Danger), ("Skip", Resolution::Skip, Look::Plain), ("Keep both", Resolution::KeepBoth, Look::Accent)].into_iter().enumerate() {
                    let w_ = btn_width(&p, None, l);
                    let br = Rect::from_min_size(pos2(x - w_, cr.bottom() - 20.0 - bh), vec2(w_, bh));
                    x -= w_ + 8.0;
                    if button(ui, &pal, br, Id::new(("res", i)), None, l, look, false, true).clicked() {
                        self.pending.push(Action::JobResolve { id, r: r_, all });
                    }
                }
            } else if done || failed {
                let mut x = rect.right() - 26.0;
                let w1 = btn_width(&p, Some(Icon::Check), "Done");
                let b1 = Rect::from_min_size(pos2(x - w1, by), vec2(w1, bh));
                if button(ui, &pal, b1, Id::new("tr-close"), Some(Icon::Check), "Done", Look::Accent, false, true).clicked() {
                    self.pending.push(Action::JobMinimize(id));
                }
                if done {
                    x -= w1 + 8.0;
                    let w0 = btn_width(&p, Some(Icon::FolderOpen), "Show in folder");
                    let b0 = Rect::from_min_size(pos2(x - w0, by), vec2(w0, bh));
                    if button(ui, &pal, b0, Id::new("tr-show"), Some(Icon::FolderOpen), "Show in folder", Look::Plain, false, true).clicked() {
                        self.pending.push(Action::GoPath(dest.clone()));
                        self.pending.push(Action::JobMinimize(id));
                    }
                }
            } else {
                let paused = state == JobState::Paused;
                let mut x = rect.right() - 26.0;
                let wbg = btn_width(&p, Some(Icon::ChevronDown), "Run in background");
                let b2 = Rect::from_min_size(pos2(x - wbg, by), vec2(wbg, bh));
                if button(ui, &pal, b2, Id::new("tr-bg"), Some(Icon::ChevronDown), "Run in background", Look::Plain, false, true).clicked() {
                    self.pending.push(Action::JobMinimize(id));
                }
                x -= wbg + 8.0;
                let wc = btn_width(&p, Some(Icon::Close), "Cancel");
                let b1 = Rect::from_min_size(pos2(x - wc, by), vec2(wc, bh));
                if button(ui, &pal, b1, Id::new("tr-cancel"), Some(Icon::Close), "Cancel", Look::Danger, false, true).clicked() {
                    self.pending.push(Action::JobCancel(id));
                }
                x -= wc + 8.0;
                let lbl = if paused { "Resume" } else { "Pause" };
                let wp = btn_width(&p, Some(if paused { Icon::Play } else { Icon::Pause }), lbl);
                let b0 = Rect::from_min_size(pos2(x - wp, by), vec2(wp, bh));
                if button(ui, &pal, b0, Id::new("tr-pause"), Some(if paused { Icon::Play } else { Icon::Pause }), lbl, Look::Plain, false, true).clicked() {
                    self.pending.push(Action::JobPause(id));
                }
            }
        });
    }

    /// In-app terminal: same `Vfs` the file views use, so it sees the real filesystem (or the demo world).
    fn terminal_ui(&mut self, ctx: &Context) {
        if self.term.cwd.is_empty() {
            self.term.cwd = self.store.current_folder();
            if self.term.lines.is_empty() {
                self.term.lines.push((false, "Smart Terminal — the shell talks to the same storage layer as the views.".into()));
                self.term.lines.push((false, "Type `help` for commands.".into()));
            }
        }
        let lines = self.term.lines.clone();
        let cwd = self.term.cwd.clone();
        let mut input = std::mem::take(&mut self.term.input);
        let mut run: Option<String> = None;
        self.modal_shell(ctx, "m-term", 620.0, 292.0, Icon::Terminal, "Terminal", &cwd, |app, ui, r| {
            let pal = app.pal;
            let out = Rect::from_min_max(r.min, pos2(r.right(), r.bottom() - 40.0));
            let mut lui = ui.new_child(UiBuilder::new().max_rect(out).layout(Layout::top_down(Align::Min)));
            lui.set_clip_rect(out.intersect(ui.clip_rect()));
            ScrollArea::vertical().id_salt("term-scroll").auto_shrink([false, false]).stick_to_bottom(true).show(&mut lui, |ui| {
                for (cmd, l) in &lines {
                    let g = ui.painter().layout(l.clone(), FontId::new(11.5, FontFamily::Monospace), if *cmd { pal.accent_ink } else { pal.ink2 }, (out.width() - 12.0).max(80.0));
                    let (rr, _) = ui.allocate_exact_size(vec2(out.width() - 12.0, g.size().y + 3.0), Sense::hover());
                    ui.painter().galley(rr.min, g, if *cmd { pal.accent_ink } else { pal.ink2 });
                }
            });
            let fr = Rect::from_min_size(pos2(r.left(), r.bottom() - 34.0), vec2(r.width(), 32.0));
            grad_rect(ui.painter(), fr, 10.0, if pal.dark { rgba(0, 0, 0, 0.3) } else { rgba(20, 30, 48, 0.07) }, if pal.dark { rgba(255, 255, 255, 0.04) } else { rgba(20, 30, 48, 0.03) });
            text(ui.painter(), pos2(fr.left() + 12.0, fr.center().y), Align2::LEFT_CENTER, "$", 12.5, W::Semi, pal.accent_ink);
            let resp = ui.put(
                Rect::from_min_max(fr.min + vec2(24.0, 2.0), fr.max - vec2(8.0, 2.0)),
                TextEdit::singleline(&mut input).frame(false).font(FontId::new(11.5, FontFamily::Monospace)).text_color(pal.ink1).vertical_align(Align::Center).margin(Margin::symmetric(4, 0)),
            );
            resp.request_focus();
            if resp.has_focus() && ui.ctx().input(|i| i.key_pressed(Key::Enter)) {
                run = Some(input.clone());
            }
        });
        if let Some(line) = run {
            self.term_exec(&line);
            self.term.input.clear();
        } else {
            self.term.input = input;
        }
    }

    fn term_exec(&mut self, line: &str) {
        let line = line.trim().to_string();
        self.term.lines.push((true, format!("{} $ {}", name_of(&self.term.cwd), line)));
        if self.term.lines.len() > 400 {
            let n = self.term.lines.len() - 400;
            self.term.lines.drain(0..n);
        }
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap_or("").to_lowercase();
        let arg = parts.collect::<Vec<_>>().join(" ");
        let cwd = self.term.cwd.clone();
        let home = self.store.eng.home.clone();
        let resolve = |arg: &str| -> Path {
            if arg.is_empty() { cwd.clone() }
            else if arg == "~" { home.clone() }
            else if arg.starts_with('/') { arg.to_string() }
            else { join(&cwd, arg) }
        };
        let mut out: Vec<String> = vec![];
        match cmd.as_str() {
            "" => {}
            "help" => out.push("help  pwd  ls [path]  cd <dir>  cat <file>  du  df  open <name>  clear  date  whoami".into()),
            "pwd" => out.push(cwd.clone()),
            "date" => out.push(fmt_full_date(now_ts())),
            "whoami" => out.push("naledzi".into()),
            "uname" => out.push(format!("Smart Files · {} host", if self.store.eng.native { "native" } else { "wasm" })),
            "clear" => { self.term.lines.clear(); return; }
            "ls" => match self.store.eng.list(&resolve(&arg)) {
                Ok(l) => {
                    let mut names: Vec<String> = l.iter().map(|e| if e.is_dir { format!("{}/", e.name) } else { e.name.clone() }).collect();
                    names.sort();
                    out.push(names.join("   "));
                }
                Err(e) => out.push(e),
            },
            "cd" => match self.store.eng.stat(&resolve(&arg)) {
                Ok(e) if e.is_dir => self.term.cwd = e.path,
                Ok(_) => out.push("not a directory".into()),
                Err(e) => out.push(e),
            },
            "cat" => match self.store.eng.read_head(&resolve(&arg), 600) {
                Ok(b) => out.push(String::from_utf8_lossy(&b).replace('\0', "·")),
                Err(e) => out.push(e),
            },
            "du" => out.push(format!("{} in {}", fmt_size(self.store.eng.dir_size(&cwd)), cwd)),
            "df" => {
                for m in self.store.eng.mounts.iter().filter(|m| m.mounted) {
                    out.push(format!("{:<14} {:>10} free of {}", m.label, fmt_size(m.free), fmt_size(m.total)));
                }
            }
            "open" => {
                let target = resolve(&arg);
                out.push(format!("opening {}", name_of(&target)));
                self.pending.push(Action::Open(target));
            }
            other => out.push(format!("{other}: command not found (try `help`)")),
        }
        for l in out {
            self.term.lines.push((false, l));
        }
    }

    pub fn quicklook_ui(&mut self, ctx: &Context) {
        let Some(path) = self.store.quicklook.clone() else { return };
        let Ok(e) = self.store.eng.stat(&path) else {
            self.store.quicklook = None;
            return;
        };
        let pal = self.pal;
        let screen = ctx.screen_rect();
        let t = ctx.animate_bool_with_time(Id::new("ql-t"), true, 0.18);
        Area::new(Id::new("ql-scrim")).order(Order::Foreground).fixed_pos(screen.min).show(ctx, |ui| {
            let (r, resp) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter().rect_filled(r, 0.0, with_a(hex(0x050a14), 0.55 * t));
            if resp.clicked() {
                self.pending.push(Action::QuickLook(None));
            }
        });
        let w = (screen.width() - 80.0).min(940.0);
        let h = (screen.height() - 80.0).min(660.0);
        let rect = Rect::from_center_size(self.center(ctx) + vec2(0.0, (1.0 - t) * 14.0), vec2(w, h));
        let tex = if e.kind == Kind::Image { self.thumbs.get(ctx, &self.store, &path, 1800) } else { None };
        let ex = if matches!(e.kind, Kind::Text | Kind::Code) { self.store.eng.read_head(&path, 24_000).map(|b| String::from_utf8_lossy(&b).to_string()).ok() } else { None };
        let arc: Vec<(String, u64, bool)> = if archive::is_archive(&e.name) && e.size < 64_000_000 {
            let key = Id::new(("arcq", &path));
            ctx.data(|d| d.get_temp::<Vec<(String, u64, bool)>>(key)).unwrap_or_else(|| {
                let v: Vec<(String, u64, bool)> = self.store.eng.read_all(&path, 64_000_000).ok().and_then(|b| archive::list(b, &e.name).ok()).map(|l| l.into_iter().map(|i| (i.path, i.size, i.is_dir)).collect()).unwrap_or_default();
                ctx.data_mut(|d| d.insert_temp(key, v.clone()));
                v
            })
        } else {
            vec![]
        };
        let player = self.store.player.clone();
        let peaks = if e.kind == Kind::Audio { self.store.wave_peaks(&path, 90) } else { vec![] };
        let dur = if e.kind == Kind::Audio { self.store.audio_duration(&path) } else { 0.0 };
        let tag = self.store.tag_of(&path);
        Area::new(Id::new("ql-panel")).order(Order::Foreground).fixed_pos(rect.min).show(ctx, |ui| {
            ui.set_opacity(t);
            let (rect, _) = ui.allocate_exact_size(rect.size(), Sense::click());
            let p = ui.painter().clone();
            elevation(&p, rect, 26.0, 5, pal.dark);
            grad_rect(&p, rect, 26.0, pal.panel_top, pal.panel_bottom);
            reflect(&p, rect, 26.0, if pal.dark { 0.12 } else { 0.95 }, 0.0);
            // header
            let kc = pal.kind_color(e.kind);
            let tile = Rect::from_min_size(rect.min + vec2(22.0, 18.0), vec2(40.0, 40.0));
            grad_rect(&p, tile, 13.0, with_a(kc, 0.3), with_a(kc, 0.14));
            reflect(&p, tile, 13.0, 0.6, 0.15);
            icon(&p, tile.center(), e.kind.icon(), 20.0, kc, true);
            text_fit(&p, rect.min + vec2(74.0, 29.0), Align2::LEFT_CENTER, &e.name, 15.5, W::Semi, pal.ink1, w - 360.0);
            text_fit(&p, rect.min + vec2(74.0, 48.0), Align2::LEFT_CENTER, &format!("{}  ·  {}  ·  {}", e.kind_label(), fmt_size(e.size), fmt_date(e.modified)), 11.5, W::Regular, pal.ink3, w - 360.0);
            if let Some(tg) = tag {
                dot(&p, rect.min + vec2(74.0 + text_w(&p, &e.name, 15.5, W::Semi).min(w - 360.0) + 14.0, 29.0), 4.0, Color32::from_rgb(tg.rgb[0], tg.rgb[1], tg.rgb[2]));
            }
            let mut x = rect.right() - 22.0;
            let cb = Rect::from_min_size(pos2(x - 34.0, rect.top() + 21.0), vec2(34.0, 34.0));
            x -= 44.0;
            if button(ui, &pal, cb, Id::new("ql-x"), Some(Icon::Close), "", Look::Plain, false, true).clicked() {
                self.pending.push(Action::QuickLook(None));
            }
            let ow = btn_width(&p, Some(Icon::OpenExt), "Open");
            let ob = Rect::from_min_size(pos2(x - ow, rect.top() + 21.0), vec2(ow, 34.0));
            x -= ow + 8.0;
            if button(ui, &pal, ob, Id::new("ql-open"), Some(Icon::OpenExt), "Open", Look::Accent, false, true).clicked() {
                self.store.quicklook = None;
                self.pending.push(Action::Open(path.clone()));
                if !self.store.eng.native {
                    self.store.quicklook = Some(path.clone());
                }
            }
            let ow = btn_width(&p, Some(Icon::App), "Open with…");
            let ob = Rect::from_min_size(pos2(x - ow, rect.top() + 21.0), vec2(ow, 34.0));
            x -= ow + 8.0;
            if button(ui, &pal, ob, Id::new("ql-ow"), Some(Icon::App), "Open with…", Look::Plain, false, true).clicked() {
                self.store.quicklook = None;
                self.pending.push(Action::OpenWithDialog(path.clone()));
            }
            for (i, (ic, d)) in [(Icon::Back, -1), (Icon::Forward, 1)].into_iter().enumerate() {
                let br = Rect::from_min_size(pos2(x - 34.0, rect.top() + 21.0), vec2(34.0, 34.0));
                x -= 40.0;
                if button(ui, &pal, br, Id::new(("ql-step", i)), Some(ic), "", Look::Plain, false, true).clicked() {
                    self.step_quicklook(d);
                }
            }
            // stage
            let stage = Rect::from_min_max(rect.min + vec2(22.0, 76.0), rect.max - vec2(22.0, 22.0));
            grad_rect(&p, stage, 20.0, if pal.dark { rgba(0, 0, 0, 0.28) } else { rgba(20, 30, 48, 0.06) }, if pal.dark { rgba(0, 0, 0, 0.18) } else { rgba(20, 30, 48, 0.03) });
            if let Some(tx) = &tex {
                let sz = tx.size_vec2();
                let k = ((stage.width() - 24.0) / sz.x).min((stage.height() - 24.0) / sz.y).min(3.0);
                let ir = Rect::from_center_size(stage.center(), sz * k);
                elevation(&p, ir, 12.0, 3, pal.dark);
                let mut m = epaint::Mesh::with_texture(tx.id());
                m.add_rect_with_uv(ir, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                p.add(Shape::mesh(m));
            } else if e.kind == Kind::Image {
                text(&p, stage.center(), Align2::CENTER_CENTER, "Decoding…", 13.0, W::Medium, pal.ink3);
            } else if e.kind == Kind::Audio {
                let wf = Rect::from_center_size(stage.center() - vec2(0.0, 24.0), vec2(stage.width() - 120.0, 130.0));
                let n = peaks.len().max(1);
                let bw = wf.width() / n as f32;
                let frac = if player.path.as_ref() == Some(&path) && dur > 0.0 { (player.pos / dur) as f32 } else { 0.0 };
                for (i, pk) in peaks.iter().enumerate() {
                    let hh = 8.0 + pk * wf.height();
                    let played = (i as f32 / n as f32) < frac;
                    p.rect_filled(Rect::from_center_size(pos2(wf.left() + i as f32 * bw + bw / 2.0, wf.center().y), vec2((bw - 3.0).max(1.5), hh)), 2.0, if played { pal.accent } else { with_a(pal.ink1, 0.18) });
                }
                let playing = player.playing && player.path.as_ref() == Some(&path);
                let pb = Rect::from_center_size(pos2(stage.center().x, wf.bottom() + 52.0), vec2(56.0, 56.0));
                if button(ui, &pal, pb, Id::new("ql-play"), Some(if playing { Icon::Pause } else { Icon::Play }), "", Look::Accent, false, true).clicked() {
                    self.pending.push(Action::PlayToggle(path.clone()));
                }
                let pos = if player.path.as_ref() == Some(&path) { player.pos } else { 0.0 };
                let fmt_t = |s: f64| format!("{}:{:02}", (s as u64) / 60, (s as u64) % 60);
                text(&p, pos2(wf.left(), wf.bottom() + 20.0), Align2::LEFT_CENTER, &fmt_t(pos), 11.5, W::Medium, pal.ink3);
                text(&p, pos2(wf.right(), wf.bottom() + 20.0), Align2::RIGHT_CENTER, &fmt_t(dur), 11.5, W::Medium, pal.ink3);
            } else if let Some(txt) = &ex {
                let mut tui = ui.new_child(UiBuilder::new().max_rect(stage.shrink(18.0)).layout(Layout::top_down(Align::Min)));
                tui.set_clip_rect(stage.shrink(14.0).intersect(ui.clip_rect()));
                ScrollArea::vertical().id_salt("ql-text").auto_shrink([false, false]).show(&mut tui, |ui| {
                    for l in txt.lines() {
                        let heading = l.starts_with('#');
                        let g = ui.painter().layout(l.to_string(), FontId::new(if heading { 15.0 } else { 12.5 }, if heading { semi_family() } else { FontFamily::Monospace }), if heading { pal.ink1 } else { pal.ink2 }, stage.width() - 60.0);
                        let (r, _) = ui.allocate_exact_size(vec2(stage.width() - 50.0, g.size().y + 4.0), Sense::hover());
                        ui.painter().galley(r.min, g, pal.ink2);
                    }
                });
            } else if !arc.is_empty() {
                let mut y = stage.top() + 18.0;
                text(&p, pos2(stage.left() + 24.0, y), Align2::LEFT_CENTER, &format!("{} entries", arc.len()), 12.0, W::Semi, pal.ink2);
                let eb = Rect::from_min_size(pos2(stage.right() - 160.0, y - 17.0), vec2(136.0, 34.0));
                if button(ui, &pal, eb, Id::new("ql-extract"), Some(Icon::Archive), "Extract here", Look::Accent, false, true).clicked() {
                    self.store.quicklook = None;
                    self.store.sel = vec![path.clone()];
                    self.pending.push(Action::ExtractHere);
                }
                y += 30.0;
                for (n, s, d) in arc.iter().take(((stage.bottom() - y) / 30.0) as usize) {
                    let depth = n.matches('/').count() as f32;
                    icon(&p, pos2(stage.left() + 34.0 + depth * 18.0, y + 12.0), if *d { Icon::Folder } else { Icon::from_name(n) }, 15.0, if *d { pal.accent } else { pal.ink3 }, *d);
                    text_fit(&p, pos2(stage.left() + 54.0 + depth * 18.0, y + 12.0), Align2::LEFT_CENTER, name_of(n.trim_end_matches('/')), 12.5, W::Medium, pal.ink1, stage.width() - 220.0);
                    text(&p, pos2(stage.right() - 28.0, y + 12.0), Align2::RIGHT_CENTER, &if *d { String::new() } else { fmt_size(*s) }, 11.5, W::Regular, pal.ink3);
                    y += 30.0;
                }
            } else {
                icon(&p, stage.center() - vec2(0.0, 20.0), e.kind.icon(), 72.0, kc, true);
                text(&p, stage.center() + vec2(0.0, 50.0), Align2::CENTER_CENTER, "No preview available", 14.0, W::Semi, pal.ink2);
                text(&p, stage.center() + vec2(0.0, 74.0), Align2::CENTER_CENTER, "Press Enter to open with the default app", 12.0, W::Regular, pal.ink3);
            }
        });
    }
}

fn semi_family() -> FontFamily {
    crate::icons::semi()
}

trait IconExt {
    fn from_name(n: &str) -> Icon;
    #[allow(non_snake_case)]
    fn Down() -> Icon;
}
impl IconExt for Icon {
    fn from_name(n: &str) -> Icon {
        Kind::from_ext(&ext_of(name_of(n))).icon()
    }
    fn Down() -> Icon {
        Icon::ChevronDown
    }
}
