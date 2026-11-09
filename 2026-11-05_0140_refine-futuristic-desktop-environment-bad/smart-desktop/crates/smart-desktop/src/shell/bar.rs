use super::*;

impl SmartApp {
    fn spark(&self, p: &egui::Painter, r: Rect, data: &[f32], col: Color32, max: f32) {
        if data.len() < 2 {
            return;
        }
        let n = data.len();
        let pts: Vec<Pos2> = data.iter().enumerate().map(|(i, v)| pos2(r.left() + r.width() * i as f32 / (n - 1) as f32, r.bottom() - r.height() * (v / max).clamp(0.0, 1.0))).collect();
        p.add(egui::Shape::line(pts, egui::Stroke::new(1.5, col)));
    }

    fn ago(&self, t: f64) -> String {
        let d = (self.shell.now - t).max(0.0);
        if d < 5.0 {
            "now".into()
        } else if d < 90.0 {
            format!("{}s", d as u32)
        } else {
            format!("{}m", (d / 60.0) as u32)
        }
    }

    pub(crate) fn draw_topbar(&mut self, ui: &mut Ui, screen: Rect, ai_t: f32) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let p = ui.painter().clone();
        let bar = Rect::from_min_size(screen.min, vec2(screen.width(), TOPBAR_H));
        let vis = 1.0 - ai_t * 0.85;
        let (top, bot) = if th.dark { (c(10, 14, 24, 0.62 * vis), c(10, 14, 24, 0.44 * vis)) } else { (c(255, 255, 255, 0.58 * vis), c(244, 248, 255, 0.40 * vis)) };
        grad_fill(&p, bar, 0.0, top, bot);
        p.line_segment([bar.left_bottom(), bar.right_bottom()], egui::Stroke::new(1.0, if th.dark { c(255, 255, 255, 0.07 * vis) } else { c(255, 255, 255, 0.7 * vis) }));
        p.line_segment([bar.left_bottom() + vec2(0.0, 1.0), bar.right_bottom() + vec2(0.0, 1.0)], egui::Stroke::new(1.0, alpha(th.shadow, (if th.dark { 0.3 } else { 0.06 }) * vis)));
        let cy = bar.center().y;
        let app = self.focused_app();
        let ink = th.ink1;
        let narrow = bar.width() < 900.0;

        // ── island (center): live state of the machine ──
        let busy = self.jobs.iter().find(|j| j.state == JobState::Running).map(|j| (j.title.clone(), j.done as f32 / j.total.max(1) as f32));
        let rec = self.shell.cap.recording_secs(self.shell.now);
        let ai_busy = self.shell.ai.busy;
        let speaking = self.shell.ai.speaking;
        let tw_target = if busy.is_some() { 270.0 } else if rec.is_some() { 190.0 } else if ai_busy || speaking { 210.0 } else { 178.0 };
        let iw = spring_to(&ctx, "island-w", tw_target, 220.0, 24.0);
        let island = Rect::from_center_size(pos2(bar.center().x, cy), vec2(iw, 24.0));
        let ir = self.hit(ui, island, "tb-island", Sense::click());
        shadow(&p, island, 12.0, 2.0, 6.0, -1.0, alpha(th.shadow, 0.35));
        grad_fill(&p, island, 12.0, hex("#1b2336"), hex("#05070d"));
        rim(&p, island, 12.0, 1.0, Color32::from_white_alpha(if ir.hovered() { 90 } else { 54 }), true, 14.0);
        let t = self.time as f32;
        if let Some((title, frac)) = &busy {
            let ring = pos2(island.left() + 14.0, cy);
            p.circle_stroke(ring, 6.0, egui::Stroke::new(2.0, Color32::from_white_alpha(40)));
            p.add(egui::Shape::line(crate::widgets::arc_points(ring, 6.0, -std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2 + frac * std::f32::consts::TAU), egui::Stroke::new(2.0, hex("#8fb4ff"))));
            let short = self.fit(&p, title, &f_medium(11.0), island.width() - 76.0);
            p.text(pos2(island.left() + 28.0, cy), Align2::LEFT_CENTER, short, f_medium(11.0), Color32::from_white_alpha(225));
            p.text(pos2(island.right() - 10.0, cy), Align2::RIGHT_CENTER, format!("{:.0}%", frac * 100.0), f_mono_b(11.0), Color32::WHITE);
        } else if let Some(s) = rec {
            let blink = 0.5 + 0.5 * (t * 4.0).sin();
            p.circle_filled(pos2(island.left() + 14.0, cy), 4.5, alpha(hex("#ff4d4d"), 0.5 + 0.5 * blink));
            p.circle_filled(pos2(island.left() + 14.0, cy), 9.0, alpha(hex("#ff4d4d"), 0.15 * blink));
            p.text(pos2(island.left() + 28.0, cy), Align2::LEFT_CENTER, "REC", f_mono_b(10.5), hex("#ff8a8a"));
            p.text(pos2(island.right() - 12.0, cy), Align2::RIGHT_CENTER, format!("{:02}:{:02}", (s / 60.0) as u32, (s % 60.0) as u32), f_mono_b(11.5), Color32::WHITE);
        } else if ai_busy || speaking {
            for i in 0..14 {
                let h = if speaking { 3.0 + 7.0 * (0.5 + 0.5 * (t * 9.0 + i as f32 * 0.9).sin()) } else { 3.0 + 3.0 * (0.5 + 0.5 * (t * 4.0 - i as f32 * 0.5).sin()) };
                let x = island.left() + 14.0 + i as f32 * 4.2;
                p.line_segment([pos2(x, cy - h / 2.0), pos2(x, cy + h / 2.0)], egui::Stroke::new(2.0, mix(hex("#a58bff"), hex("#ff7ac0"), i as f32 / 14.0)));
            }
            p.text(pos2(island.right() - 12.0, cy), Align2::RIGHT_CENTER, if speaking { "speaking" } else { "thinking" }, f_medium(10.5), Color32::from_white_alpha(200));
        } else {
            p.circle_filled(pos2(island.left() + 13.0, cy), 3.2, th.good);
            p.circle_filled(pos2(island.left() + 13.0, cy), 6.5, alpha(th.good, 0.18 + 0.1 * (t * 2.0).sin()));
            let lat = format!("{:.0} ms", self.shell.lat);
            p.text(pos2(island.left() + 26.0, cy), Align2::LEFT_CENTER, if self.shell.remote { "Remote" } else { "Local" }, f_semi(10.5), Color32::from_white_alpha(215));
            p.text(pos2(island.right() - 12.0, cy), Align2::RIGHT_CENTER, lat, f_mono_b(10.5), Color32::from_white_alpha(235));
            let sp = Rect::from_min_max(pos2(island.left() + 78.0, cy - 6.0), pos2(island.right() - 62.0, cy + 6.0));
            self.spark(&p, sp, &self.shell.lat_hist, hex("#6ee7ff"), 70.0);
        }
        if ir.clicked() {
            self.shell.sys_open = !self.shell.sys_open;
            self.shell.clip_open = false;
            self.shell.qs = false;
            self.shell.snd.play(Snd::Menu);
        }

        // ── left: logo + app name + menus (only what fits before the island) ──
        let mut x = bar.left() + 10.0;
        let logo = Rect::from_center_size(pos2(x + 13.0, cy), vec2(26.0, 22.0));
        let lr = self.hit(ui, logo, "tb-logo", Sense::click());
        if lr.hovered() || self.shell.menu == Some(0) {
            p.rect_filled(logo, cr(7.0), th.row_hover);
        }
        let lg = Rect::from_center_size(logo.center(), vec2(17.0, 17.0));
        let (a, b) = th.accent_grad();
        grad_fill(&p, lg, 6.0, a, b);
        rim(&p, lg, 6.0, 1.0, Color32::from_white_alpha(140), true, 8.0);
        p.text(lg.center() + vec2(0.0, 0.5), Align2::CENTER_CENTER, "S", f_bold(10.5), Color32::WHITE);
        let mut anchors: Vec<Rect> = vec![logo];
        if lr.clicked() {
            self.shell.menu = if self.shell.menu == Some(0) { None } else { Some(0) };
            self.shell.snd.play(Snd::Menu);
        }
        x = logo.right() + 8.0;
        let limit = island.left() - 14.0;
        let nm = app.name();
        if x + self.tw(&p, nm, &f_bold(12.5)) < limit {
            p.text(pos2(x, cy), Align2::LEFT_CENTER, nm, f_bold(12.5), ink);
            x += self.tw(&p, nm, &f_bold(12.5)) + 12.0;
            let menus = self.tb_menus(app);
            for (i, (label, _)) in menus.iter().enumerate() {
                let w = self.tw(&p, label, &f_medium(12.5)) + 18.0;
                if x + w > limit {
                    anchors.push(Rect::NOTHING);
                    continue;
                }
                let r = Rect::from_min_size(pos2(x, cy - 11.0), vec2(w, 22.0));
                let resp = self.hit(ui, r, ("tb-menu", i), Sense::click());
                let open = self.shell.menu == Some(i + 1);
                if resp.hovered() || open {
                    p.rect_filled(r, cr(7.0), if open { th.row_press } else { th.row_hover });
                }
                p.text(r.center(), Align2::CENTER_CENTER, *label, f_medium(12.5), if open || resp.hovered() { ink } else { th.ink2 });
                if resp.clicked() {
                    self.shell.menu = if open { None } else { Some(i + 1) };
                } else if resp.hovered() && self.shell.menu.is_some() && !open {
                    self.shell.menu = Some(i + 1);
                }
                anchors.push(r);
                x += w + 2.0;
            }
        }

        // ── right: clock, battery, volume, link, clipboard, assistant ──
        let mut rx = bar.right() - 10.0;
        let time = clock_hm();
        let date = clock_date();
        let tf = f_mono_b(12.5);
        let tw_ = self.tw(&p, &time, &tf);
        let show_date = bar.width() > 1040.0;
        let dw = if show_date { self.tw(&p, &date, &f_medium(12.0)) } else { 0.0 };
        let cluster = Rect::from_min_max(pos2(rx - tw_ - dw - if show_date { 18.0 } else { 6.0 }, cy - 12.0), pos2(rx + 4.0, cy + 12.0));
        let cresp = self.hit(ui, cluster, "tb-clock", Sense::click());
        if cresp.hovered() || self.shell.qs {
            p.rect_filled(cluster, cr(8.0), if self.shell.qs { th.row_press } else { th.row_hover });
        }
        if cresp.clicked() {
            self.shell.qs = !self.shell.qs;
            self.shell.menu = None;
            self.shell.sys_open = false;
            self.shell.clip_open = false;
        }
        p.text(pos2(rx, cy), Align2::RIGHT_CENTER, &time, tf, ink);
        rx -= tw_ + 10.0;
        if show_date {
            p.text(pos2(rx, cy), Align2::RIGHT_CENTER, &date, f_medium(12.0), th.ink2);
            rx -= dw + 12.0;
        }
        // battery with level
        let bat = Rect::from_center_size(pos2(rx - 13.0, cy), vec2(26.0, 20.0));
        let br = self.hit(ui, bat, "tb-bat", Sense::click());
        let body = Rect::from_center_size(bat.center(), vec2(19.0, 10.0));
        p.rect_stroke(body, cr(3.0), egui::Stroke::new(1.2, alpha(th.ink2, 0.9)), egui::StrokeKind::Inside);
        p.rect_filled(Rect::from_min_size(body.min + vec2(1.8, 1.8), vec2((body.width() - 3.6) * 0.87, body.height() - 3.6)), cr(1.5), th.good);
        p.rect_filled(Rect::from_center_size(pos2(body.right() + 1.8, cy), vec2(1.8, 4.0)), cr(1.0), alpha(th.ink2, 0.8));
        p.text(pos2(body.center().x, body.center().y), Align2::CENTER_CENTER, "\u{26a1}", f_regular(6.5), Color32::from_white_alpha(230));
        if br.clicked() {
            self.shell.qs = true;
        }
        br.on_hover_text("87% \u{b7} charging");
        rx -= 30.0;
        // volume (wheel changes it, with a tick for every step)
        let vol = Rect::from_center_size(pos2(rx - 11.0, cy), vec2(24.0, 22.0));
        let vr = self.hit(ui, vol, "tb-vol", Sense::click());
        if vr.hovered() {
            p.rect_filled(vol, cr(7.0), th.row_hover);
            let d = ctx.input(|i| i.smooth_scroll_delta.y);
            if d != 0.0 {
                self.shell.volume = (self.shell.volume + d * 0.0015).clamp(0.0, 1.0);
                self.shell.scroll_vol += d.abs();
                if self.shell.scroll_vol > 14.0 {
                    self.shell.scroll_vol = 0.0;
                    self.shell.snd.vol = self.shell.volume;
                    self.shell.snd.play(Snd::Volume);
                }
            }
        }
        let vi = if self.shell.volume < 0.02 { "volume-small" } else { "volume-loud" };
        self.ico(&p, vi, Ic::Outline, vol.center(), 15.0, if self.shell.volume > 0.02 { th.ink2 } else { alpha(th.ink4, 0.7) });
        vr.on_hover_text(format!("Volume {:.0}%  (scroll)", self.shell.volume * 100.0));
        rx -= 28.0;
        if !narrow {
            let net = Rect::from_center_size(pos2(rx - 11.0, cy), vec2(24.0, 22.0));
            let nr = self.hit(ui, net, "tb-net", Sense::click());
            if nr.hovered() {
                p.rect_filled(net, cr(7.0), th.row_hover);
            }
            self.ico(&p, if self.shell.remote { "global" } else { "wi-fi" }, Ic::Outline, net.center(), 15.0, if self.shell.wifi { th.ink2 } else { alpha(th.ink4, 0.7) });
            if nr.clicked() {
                self.shell.sys_open = !self.shell.sys_open;
            }
            nr.on_hover_text(if self.shell.remote { "Remote session \u{b7} click for live state" } else { "Wi-Fi" });
            rx -= 28.0;
            // clipboard
            let clip = Rect::from_center_size(pos2(rx - 11.0, cy), vec2(24.0, 22.0));
            let cr_ = self.hit(ui, clip, "tb-clip", Sense::click());
            if cr_.hovered() || self.shell.clip_open {
                p.rect_filled(clip, cr(7.0), if self.shell.clip_open { th.row_press } else { th.row_hover });
            }
            self.ico(&p, "clipboard-list", Ic::Outline, clip.center(), 15.0, th.ink2);
            if !self.shell.clip.is_empty() {
                p.circle_filled(clip.center() + vec2(7.0, -6.0), 3.0, th.accent);
            }
            ctx.data_mut(|d| d.insert_temp(Id::new("clip-btn"), clip));
            if cr_.clicked() {
                self.shell.clip_open = !self.shell.clip_open;
                self.shell.sys_open = false;
                self.shell.qs = false;
                self.shell.snd.play(Snd::Menu);
            }
            cr_.on_hover_text("Clipboard history");
            rx -= 28.0;
        }
        if self.shell.focus {
            self.ico(&p, "moon-stars", Ic::Duo, pos2(rx - 11.0, cy), 14.0, th.warn);
            rx -= 26.0;
        }
        // assistant
        let ai = Rect::from_center_size(pos2(rx - 13.0, cy), vec2(24.0, 24.0));
        let air = self.hit(ui, ai, "tb-ai", Sense::click());
        let pulse = if self.shell.ai.busy || self.shell.ai.speaking { 0.5 + 0.5 * (t * 6.0).sin() } else { 0.0 };
        let (g0, g1) = (hex("#a58bff"), hex("#e0569c"));
        shadow(&p, ai, 12.0, 2.0, 6.0 + 4.0 * pulse, -1.0, alpha(g1, 0.45));
        grad_fill(&p, ai, 12.0, g0, g1);
        rim(&p, ai, 12.0, 1.0, Color32::from_white_alpha(150), true, 12.0);
        self.ico(&p, "stars-minimalistic", Ic::Filled, ai.center(), 13.0, Color32::WHITE);
        if air.clicked() {
            self.acts.push(Act::App("ai.toggle".into()));
        }
        air.on_hover_text(format!("Ask {}  Ctrl+Space", self.shell.ai.name));

        // ── menu popovers ──
        if let Some(mi_idx) = self.shell.menu {
            let (items, anchor) = if mi_idx == 0 {
                (
                    vec![
                        mi("info-circle", "About Smart Desktop", Act::App("sys.about".into())),
                        mi("settings-minimalistic", "Settings\u{2026}", Act::App("sys.settings".into())).k("Ctrl+,").gap(),
                        mi("stars-minimalistic", format!("Ask {}", self.shell.ai.name), Act::App("ai.toggle".into())).k("Ctrl+Space"),
                        mi("widget-4", "AI mode", Act::App("ai.mode".into())),
                        mi("gallery-wide", "Wallpaper\u{2026}", Act::App("desktop.wallpaper".into())).gap(),
                        mi("window-frame", "Tile windows", Act::App("wm.tile".into())),
                        mi("layers-minimalistic", "Cascade windows", Act::App("wm.cascade".into())),
                        mi("command", "Command palette", Act::Seq(vec![Act::OpenApp(AppId::Files), Act::Palette])).k("Ctrl+K").gap(),
                        mi("restart", "Restart session\u{2026}", Act::App("sys.restart".into())),
                    ],
                    anchors[0],
                )
            } else {
                let menus = self.tb_menus(app);
                (menus.get(mi_idx - 1).map(|m| m.1.clone()).unwrap_or_default(), anchors.get(mi_idx).copied().unwrap_or(anchors[0]))
            };
            if !items.is_empty() && anchor.is_positive() {
                let w = self.menu_width(&p, &items, 220.0);
                let mut mp = ui.new_child(egui::UiBuilder::new().max_rect(screen));
                let pop = spring_to(&ctx, ("tbpop", mi_idx), 1.0, 300.0, 24.0);
                mp.set_opacity(pop.clamp(0.0, 1.0));
                let (r, _, act) = self.paint_menu(&mut mp, pos2(anchor.left(), bar.bottom() + 4.0 - (1.0 - pop.min(1.0)) * 6.0), &items, w, "tbm", None);
                if let Some(a) = act {
                    self.shell.menu = None;
                    self.shell.snd.play(Snd::Click);
                    self.acts.push(a);
                } else if ctx.input(|i| i.pointer.primary_pressed()) && ctx.pointer_interact_pos().is_some_and(|pp| !r.contains(pp) && !bar.contains(pp)) {
                    self.shell.menu = None;
                }
            }
        } else {
            ctx.data_mut(|d| {
                for i in 0..8 {
                    d.remove::<(f32, f32)>(Id::new(("spring", ("tbpop", i))));
                }
            });
        }
        self.draw_quick_settings(ui, screen, bar);
    }

    pub(crate) fn draw_clip_popover(&mut self, ui: &mut Ui, screen: Rect) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let s = spring_to(&ctx, "clip-pop", if self.shell.clip_open { 1.0 } else { 0.0 }, 260.0, 24.0);
        if s < 0.01 {
            return;
        }
        let btn = ctx.data_mut(|d| d.get_temp::<Rect>(Id::new("clip-btn"))).unwrap_or(Rect::from_min_size(pos2(screen.right() - 200.0, 4.0), vec2(24.0, 22.0)));
        let w = 352.0;
        let n = self.shell.clip.len().min(7);
        let h = 86.0 + (n.max(1) as f32) * 50.0;
        let panel = Rect::from_min_size(pos2((btn.center().x + 24.0 - w).max(screen.left() + 8.0), TOPBAR_H + 8.0 - (1.0 - s.min(1.0)) * 12.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity(s.clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, panel, 20.0);
        cui.allocate_rect(panel, Sense::click());
        p.text(pos2(panel.left() + 18.0, panel.top() + 24.0), Align2::LEFT_CENTER, "Clipboard", f_bold(T_TITLE), th.ink1);
        p.text(pos2(panel.left() + 108.0, panel.top() + 25.0), Align2::LEFT_CENTER, format!("{} item{}", self.shell.clip.len(), if self.shell.clip.len() == 1 { "" } else { "s" }), f_medium(T_META), th.ink3);
        let clr = Rect::from_min_size(pos2(panel.right() - 78.0, panel.top() + 11.0), vec2(62.0, 26.0));
        if self.label_button(&cui, clr, None, "Clear", "clip-clear", Btn::Quiet).clicked() {
            self.shell.clip.retain(|c| c.pinned);
        }
        if self.shell.clip.is_empty() {
            p.text(panel.center() + vec2(0.0, 6.0), Align2::CENTER_CENTER, "Copy text or files and they appear here", f_medium(T_BODY), th.ink3);
        }
        let mut act: Option<(usize, u8)> = None;
        let mut order: Vec<usize> = (0..self.shell.clip.len()).collect();
        order.sort_by_key(|i| (!self.shell.clip[*i].pinned, std::cmp::Reverse((self.shell.clip[*i].t * 100.0) as i64)));
        for (row, idx) in order.iter().take(n).enumerate() {
            let it = &self.shell.clip[*idx];
            let r = Rect::from_min_size(pos2(panel.left() + 10.0, panel.top() + 52.0 + row as f32 * 50.0), vec2(w - 20.0, 46.0));
            let resp = self.hit(&cui, r, ("clip-row", *idx), Sense::click());
            if resp.hovered() {
                grad_fill(&p, r, 12.0, th.accent_tint, th.accent_tint2);
            }
            let ic = Rect::from_center_size(pos2(r.left() + 22.0, r.center().y), vec2(30.0, 30.0));
            grad_fill(&p, ic, 9.0, th.card_top, th.card_bottom);
            rim(&p, ic, 9.0, 1.0, th.rim, true, 12.0);
            self.ico(&p, if it.files > 0 { "folder-with-files" } else { "text" }, Ic::Duo, ic.center(), 15.0, th.accent_ink);
            let text = if it.files > 0 { format!("{} file{}", it.files, if it.files == 1 { "" } else { "s" }) } else { it.text.replace('\n', " \u{21b5} ") };
            p.text(pos2(r.left() + 44.0, r.center().y - 7.0), Align2::LEFT_CENTER, self.fit(&p, &text, &f_semi(T_BODY), r.width() - 130.0), f_semi(T_BODY), th.ink1);
            p.text(pos2(r.left() + 44.0, r.center().y + 9.0), Align2::LEFT_CENTER, format!("{} \u{b7} {} chars", self.ago(it.t), it.text.chars().count()), f_mono(9.5), th.ink4);
            let pin = Rect::from_center_size(pos2(r.right() - 58.0, r.center().y), vec2(26.0, 26.0));
            let ask = Rect::from_center_size(pos2(r.right() - 28.0, r.center().y), vec2(26.0, 26.0));
            if self.icon_button(&cui, pin, "bookmark", &format!("clip-pin-{idx}"), if it.pinned { Btn::Tonal } else { Btn::Quiet }, true).on_hover_text("Pin").clicked() {
                act = Some((*idx, 1));
            }
            if self.icon_button(&cui, ask, "stars-minimalistic", &format!("clip-ask-{idx}"), Btn::Quiet, it.files == 0).on_hover_text("Ask about this").clicked() {
                act = Some((*idx, 2));
            }
            if resp.clicked() && act.is_none() {
                act = Some((*idx, 0));
            }
        }
        match act {
            Some((i, 0)) => {
                let t = self.shell.clip[i].text.clone();
                self.pending_copy = Some(t);
                self.shell.snd.play(Snd::Paste);
                self.shell.clip_open = false;
                self.shell.notify("Copied back to the clipboard", "", NoteKind::Info);
            }
            Some((i, 1)) => self.shell.clip[i].pinned = !self.shell.clip[i].pinned,
            Some((i, _)) => {
                let t = self.shell.clip[i].text.clone();
                self.shell.clip_open = false;
                self.ai_open_with(format!("Explain this: {}", t.chars().take(400).collect::<String>()), Some(self.focused_app()));
            }
            None => {}
        }
        if ctx.input(|i| i.pointer.primary_pressed()) && ctx.pointer_interact_pos().is_some_and(|pp| !panel.contains(pp) && !btn.expand(6.0).contains(pp)) {
            self.shell.clip_open = false;
        }
    }

    pub(crate) fn draw_sys_popover(&mut self, ui: &mut Ui, screen: Rect) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let s = spring_to(&ctx, "sys-pop", if self.shell.sys_open { 1.0 } else { 0.0 }, 260.0, 24.0);
        if s < 0.01 {
            return;
        }
        let (w, h) = (372.0, 392.0);
        let panel = Rect::from_min_size(pos2(screen.center().x - w / 2.0, TOPBAR_H + 8.0 - (1.0 - s.min(1.0)) * 12.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity(s.clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, panel, 22.0);
        cui.allocate_rect(panel, Sense::click());
        p.text(pos2(panel.left() + 20.0, panel.top() + 26.0), Align2::LEFT_CENTER, "Live state", f_bold(T_TITLE), th.ink1);
        p.text(pos2(panel.right() - 20.0, panel.top() + 27.0), Align2::RIGHT_CENTER, "scene graph \u{b7} compositor \u{b7} link", f_mono(9.5), th.ink4);
        let open_n = AppId::ALL.iter().filter(|a| self.is_open(**a)).count();
        let nodes = open_n + self.shell.desk.items.len() + 9;
        let fps = self.shell.fps_hist.last().copied().unwrap_or(60.0);
        let cards: [(&str, String, &str, &[f32], f32, Color32); 4] = [
            ("Link", format!("{:.0} ms", self.shell.lat), if self.shell.remote { "Lagos \u{2192} Shenzhen \u{b7} smart-delta codec" } else { "local compositor" }, &self.shell.lat_hist, 70.0, hex("#6ee7ff")),
            ("Frame", format!("{:.0} fps", fps), "depth-sorted scene \u{b7} glow renderer", &self.shell.fps_hist, 144.0, th.good),
            ("Scene", format!("{} nodes", nodes), "windows, dock items, desktop icons", &[], 1.0, th.accent),
            ("Memory", format!("{} facts", self.shell.ai.memory.len()), "global, shared by every app", &[], 1.0, hex("#a58bff")),
        ];
        for (i, (name, val, sub, data, max, col)) in cards.iter().enumerate() {
            let r = Rect::from_min_size(panel.min + vec2(14.0 + (i % 2) as f32 * 176.0, 48.0 + (i / 2) as f32 * 96.0), vec2(170.0, 90.0));
            grad_fill(&p, r, 16.0, th.card_top, th.card_bottom);
            rim(&p, r, 16.0, 1.2, th.rim, true, 30.0);
            p.text(pos2(r.left() + 14.0, r.top() + 16.0), Align2::LEFT_CENTER, *name, f_semi(T_META), th.ink3);
            p.text(pos2(r.left() + 14.0, r.top() + 38.0), Align2::LEFT_CENTER, val, f_mono_b(18.0), th.ink1);
            p.text(pos2(r.left() + 14.0, r.bottom() - 12.0), Align2::LEFT_CENTER, self.fit(&p, sub, &f_medium(9.5), r.width() - 24.0), f_medium(9.5), th.ink4);
            if !data.is_empty() {
                self.spark(&p, Rect::from_min_size(pos2(r.left() + 14.0, r.top() + 50.0), vec2(r.width() - 28.0, 16.0)), data, *col, *max);
            } else {
                let w = (0.35 + 0.35 * (self.time as f32 * 0.6 + i as f32).sin().abs()) * (r.width() - 28.0);
                p.rect_filled(Rect::from_min_size(pos2(r.left() + 14.0, r.top() + 58.0), vec2(r.width() - 28.0, 4.0)), cr(2.0), alpha(th.ink4, 0.2));
                p.rect_filled(Rect::from_min_size(pos2(r.left() + 14.0, r.top() + 58.0), vec2(w, 4.0)), cr(2.0), *col);
            }
        }
        // window list = the scene graph, live
        p.text(pos2(panel.left() + 20.0, panel.top() + 250.0), Align2::LEFT_CENTER, "WINDOWS", f_semi(9.5), th.ink4);
        let mut y = panel.top() + 266.0;
        for id in self.shell.z.clone().iter().rev() {
            if !self.is_open(*id) {
                continue;
            }
            if y > panel.bottom() - 46.0 {
                break;
            }
            let r = self.win_rect_of(*id, self.shell.screen).or_else(|| self.app_win(*id).map(|w| w.rect));
            let r = r.unwrap_or(Rect::ZERO);
            let z = self.shell.z.iter().position(|x| x == id).unwrap_or(0);
            self.ico(&p, id.icon(), Ic::Duo, pos2(panel.left() + 30.0, y + 9.0), 14.0, th.accent_ink);
            p.text(pos2(panel.left() + 48.0, y + 9.0), Align2::LEFT_CENTER, id.name(), f_semi(T_BODY), th.ink1);
            p.text(pos2(panel.right() - 20.0, y + 9.0), Align2::RIGHT_CENTER, format!("z{} \u{b7} {}\u{d7}{} @ {:.0},{:.0}", z, r.width() as i32, r.height() as i32, r.min.x, r.min.y), f_mono(10.0), th.ink3);
            y += 22.0;
        }
        if y < panel.top() + 270.0 {
            p.text(pos2(panel.left() + 20.0, y + 8.0), Align2::LEFT_CENTER, "No windows open", f_medium(T_META), th.ink4);
        }
        let b = Rect::from_min_size(pos2(panel.right() - 128.0, panel.bottom() - 46.0), vec2(112.0, 30.0));
        if self.label_button(&cui, b, Some("window-frame"), "Tile all", "sys-tile", Btn::Raised).clicked() {
            self.apply_layout("grid");
        }
        let b2 = Rect::from_min_size(pos2(panel.right() - 248.0, panel.bottom() - 46.0), vec2(112.0, 30.0));
        let rem = if self.shell.remote { "Go local" } else { "Go remote" };
        if self.label_button(&cui, b2, Some("global"), rem, "sys-remote", Btn::Raised).clicked() {
            self.shell.remote = !self.shell.remote;
            self.shell.snd.play(Snd::Toggle);
        }
        if ctx.input(|i| i.pointer.primary_pressed()) && ctx.pointer_interact_pos().is_some_and(|pp| !panel.contains(pp) && pp.y > TOPBAR_H) {
            self.shell.sys_open = false;
        }
    }

    // ───────────── status bar ─────────────
    pub(crate) fn tb_menus(&self, app: AppId) -> Vec<(&'static str, Vec<MI>)> {
        let st = self.st();
        let sel_n = self.sel.len();
        match app {
            AppId::Files => vec![
                (
                    "File",
                    vec![
                        mi("add-folder", "New folder", Act::NewFolder).k("Ctrl+Shift+N"),
                        mi("add", "New tab", Act::NewTab(None)).k("Ctrl+T"),
                        mi("close-circle", "Close tab", Act::CloseTab(self.tab_i)).off(self.tabs.len() < 2),
                        mi("info-circle", "Get Info", Act::Info).k("Ctrl+I").gap().off(sel_n == 0),
                        mi("archive", "Compress to ZIP\u{2026}", Act::ArchiveCompress).off(sel_n == 0),
                        mi("terminal", "Open terminal here", Act::OpenTerminal).gap(),
                        mi("maximize", "Close window", Act::WinClose).gap(),
                    ],
                ),
                (
                    "Edit",
                    vec![
                        mi("copy", "Copy", Act::Copy).k("Ctrl+C").off(sel_n == 0),
                        mi("scissors", "Cut", Act::Cut).k("Ctrl+X").off(sel_n == 0),
                        mi("clipboard", "Paste", Act::Paste).k("Ctrl+V").off(self.clipboard.is_none()),
                        mi("checklist-minimalistic", "Select all", Act::SelectAll).k("Ctrl+A").gap(),
                        mi("check-circle", "Selection mode", Act::ToggleSelectMode).on(self.select_mode),
                    ],
                ),
                (
                    "View",
                    vec![
                        mi("widget", "As grid", Act::SetGrid(true)).on(st.grid).k("G"),
                        mi("list-check", "As list", Act::SetGrid(false)).on(!st.grid).k("L"),
                        mi("eye", "Show hidden files", Act::ToggleHidden).on(st.show_hidden).k("Ctrl+H").gap(),
                        mi("folder", "Folders first", Act::ToggleFoldersFirst).on(st.folders_first),
                        mi("sidebar-minimalistic", "Sidebar", Act::ToggleSidebar).k("Ctrl+\\").gap(),
                    ],
                ),
                (
                    "Go",
                    {
                        let mut v = vec![mi("alt-arrow-left", "Back", Act::Back).k("Alt+\u{2190}").off(!self.can_back()), mi("alt-arrow-right", "Forward", Act::Forward).k("Alt+\u{2192}").off(!self.can_fwd()), mi("arrow-up", "Enclosing folder", Act::Up).k("Alt+\u{2191}")];
                        for (i, pl) in self.places.iter().take(6).enumerate() {
                            let mut m = mi(pl.icon, pl.title.clone(), Act::Nav(Route::Loc(pl.loc.clone())));
                            if i == 0 {
                                m = m.gap();
                            }
                            v.push(m);
                        }
                        v
                    },
                ),
            ],
            AppId::Capture | AppId::Settings | AppId::Studio => vec![
                ("Window", vec![mi("maximize", "Maximize", Act::App("wm.max".into())), mi("minimize", "Minimize", Act::App("wm.min".into())), mi("close-circle", "Close", Act::App("wm.close".into())).gap()]),
                ("View", vec![mi("stars-minimalistic", if st.dark { "Light theme" } else { "Dark theme" }, Act::ToggleTheme), mi("window-frame", "Tile windows", Act::App("wm.tile".into()))]),
            ],
            AppId::Terminal => vec![
                ("Shell", vec![mi("add", "New session", Act::App("term.new".into())).k("Ctrl+T"), mi("close-circle", "Close session", Act::App("term.close".into())).k("Ctrl+W"), mi("trash-bin-minimalistic", "Clear", Act::App("term.clear".into())).k("Ctrl+L").gap()]),
                ("View", vec![mi("magnifer-zoom-in", "Bigger text", Act::App("term.bigger".into())).k("Ctrl +"), mi("magnifer", "Smaller text", Act::App("term.smaller".into())).k("Ctrl -"), mi("stars-minimalistic", if st.dark { "Light theme" } else { "Dark theme" }, Act::ToggleTheme).gap()]),
            ],
            AppId::Photos => vec![
                ("File", vec![mi("folder-open", "Show in Files", Act::App("photos.reveal".into())), mi("gallery-wide", "Set as wallpaper", Act::App("photos.wallpaper".into())), mi("download-minimalistic", "Save edited copy", Act::App("photos.save".into())).gap()]),
                (
                    "View",
                    vec![
                        mi("full-screen", "Fit to window", Act::App("photos.fit".into())).k("0"),
                        mi("maximize", "Actual size", Act::App("photos.actual".into())).k("1"),
                        mi("play", "Slideshow", Act::App("photos.slideshow".into())).k("S").gap(),
                        mi("info-circle", "Info panel", Act::App("photos.info".into())).k("I").on(self.shell.photos.info),
                        mi("tuning-2", "Edit panel", Act::App("photos.edit".into())).k("E").on(self.shell.photos.edit),
                    ],
                ),
            ],
        }
    }


    pub(crate) fn draw_quick_settings(&mut self, ui: &mut Ui, screen: Rect, bar: Rect) {
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let s = spring_to(&ctx, "qs", if self.shell.qs { 1.0 } else { 0.0 }, 260.0, 24.0);
        if s < 0.01 {
            return;
        }
        let w = 320.0;
        let panel = Rect::from_min_size(pos2(screen.right() - w - 10.0, bar.bottom() + 8.0 - (1.0 - s.min(1.0)) * 12.0), vec2(w, 326.0));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity(s.clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, panel, 20.0);
        cui.allocate_rect(panel, Sense::click());
        let pad = 14.0;
        // toggle tiles
        let tiles: [(&'static str, &str, String, bool, u8); 4] = [
            ("wi-fi", "Wi-Fi", if self.shell.wifi { "Smart-5G".into() } else { "Off".into() }, self.shell.wifi, 0),
            ("bluetooth", "Bluetooth", if self.shell.bt { "On".into() } else { "Off".into() }, self.shell.bt, 1),
            ("moon-stars", "Dark mode", if th.dark { "On".into() } else { "Off".into() }, th.dark, 2),
            ("bell", "Focus", if self.shell.focus { "Do not disturb".into() } else { "Off".into() }, self.shell.focus, 3),
        ];
        let tw_ = (w - pad * 2.0 - 10.0) / 2.0;
        for (i, (icon, label, sub, on, id)) in tiles.iter().enumerate() {
            let r = Rect::from_min_size(panel.min + vec2(pad + (i % 2) as f32 * (tw_ + 10.0), pad + (i / 2) as f32 * 66.0), vec2(tw_, 58.0));
            let resp = self.hit(&cui, r, ("qs-tile", i), Sense::click());
            let a = self.anim(&cui, ("qs-t", i), resp.hovered());
            if *on {
                raised_accent(&p, &th, r, 16.0, resp.hovered(), resp.is_pointer_button_down_on());
            } else {
                raised(&p, &th, r, 16.0, resp.hovered(), resp.is_pointer_button_down_on());
            }
            let fg = if *on { th.on_accent() } else { th.ink1 };
            let orb = Rect::from_center_size(pos2(r.left() + 26.0, r.center().y), vec2(30.0, 30.0));
            grad_fill(&p, orb, 15.0, alpha(fg, if *on { 0.22 } else { 0.07 }), alpha(fg, if *on { 0.10 } else { 0.03 }));
            self.ico(&p, icon, if *on || a > 0.5 { Ic::Duo } else { Ic::Outline }, orb.center(), 17.0, fg);
            p.text(pos2(r.left() + 48.0, r.center().y - 8.0), Align2::LEFT_CENTER, *label, f_semi(T_BODY), fg);
            p.text(pos2(r.left() + 48.0, r.center().y + 9.0), Align2::LEFT_CENTER, self.fit(&p, sub, &f_medium(T_META), r.width() - 56.0), f_medium(T_META), alpha(fg, 0.72));
            if resp.clicked() {
                match id {
                    0 => self.shell.wifi = !self.shell.wifi,
                    1 => self.shell.bt = !self.shell.bt,
                    2 => self.acts.push(Act::ToggleTheme),
                    _ => self.shell.focus = !self.shell.focus,
                }
            }
        }
        // sliders
        let mut y = panel.top() + pad + 2.0 * 66.0 + 6.0;
        for k in 0..2 {
            let (icon, label) = if k == 0 { ("sun-2", "Display") } else { ("volume-loud", "Sound") };
            p.text(pos2(panel.left() + pad + 2.0, y + 2.0), Align2::LEFT_CENTER, label, f_semi(T_META), th.ink3);
            let track = Rect::from_min_size(pos2(panel.left() + pad + 30.0, y + 16.0), vec2(w - pad * 2.0 - 30.0, 12.0));
            let resp = self.hit(&cui, track.expand2(vec2(6.0, 8.0)), ("qs-slider", k), Sense::click_and_drag());
            let v = if k == 0 { self.shell.brightness } else { self.shell.volume };
            let mut nv = v;
            if resp.dragged() || resp.clicked() {
                if let Some(pp) = resp.interact_pointer_pos() {
                    nv = ((pp.x - track.left()) / track.width()).clamp(0.0, 1.0);
                }
            }
            if k == 0 {
                self.shell.brightness = nv.max(0.25);
            } else {
                self.shell.volume = nv;
            }
            let shown = spring_to(&ctx, ("qs-sl", k), nv, 400.0, 30.0).clamp(0.0, 1.0);
            well(&p, &th, track, 6.0);
            let fill = Rect::from_min_size(track.min, vec2((track.width() * shown).max(12.0), track.height()));
            let (c0, c1) = th.accent_grad();
            grad_fill(&p, fill, 6.0, c0, c1);
            rim(&p, fill, 6.0, 1.0, Color32::from_white_alpha(120), true, 6.0);
            let knob = Rect::from_center_size(pos2(track.left() + track.width() * shown, track.center().y), vec2(20.0, 20.0) * if resp.dragged() { 1.12 } else { 1.0 });
            raised(&p, &th, knob, 10.0, resp.hovered(), resp.dragged());
            self.ico(&p, icon, Ic::Outline, pos2(panel.left() + pad + 11.0, track.center().y), 16.0, th.ink2);
            y += 44.0;
        }
        // battery + actions
        let bat = Rect::from_min_size(pos2(panel.left() + pad, panel.bottom() - pad - 44.0), vec2(w - pad * 2.0 - 96.0, 44.0));
        self.ico(&p, "battery-charge-minimalistic", Ic::Duo, pos2(bat.left() + 12.0, bat.center().y), 20.0, th.good);
        p.text(pos2(bat.left() + 32.0, bat.center().y - 8.0), Align2::LEFT_CENTER, "87%  \u{b7}  charging", f_semi(T_BODY), th.ink1);
        p.text(pos2(bat.left() + 32.0, bat.center().y + 9.0), Align2::LEFT_CENTER, "About 2 h 10 min to full", f_medium(T_META), th.ink3);
        let sr = Rect::from_min_size(pos2(panel.right() - pad - 88.0, panel.bottom() - pad - 36.0), vec2(88.0, 32.0));
        if self.label_button(&cui, sr, Some("settings-minimalistic"), "Settings", "qs-settings", Btn::Raised).clicked() {
            self.shell.qs = false;
            self.acts.push(Act::OpenApp(AppId::Files));
            self.acts.push(Act::Settings);
        }
        if ctx.input(|i| i.pointer.primary_pressed()) && ctx.pointer_interact_pos().is_some_and(|pp| !panel.contains(pp) && !bar.contains(pp)) {
            self.shell.qs = false;
        }
    }

}
