//! Files as the system file picker: the same places, thumbnails and folder model, in a chooser the desktop (or any
//! app) can summon. Purpose-filtered, keyboard-light, and it hands back a `Location` — no private paths.
use crate::app::*;
use crate::chrome::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::shell::TOPBAR_H;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Rect, Sense, Ui};
use smart_core::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PickPurpose {
    Wallpaper,
}

pub struct Picker {
    pub purpose: PickPurpose,
    pub dir: Location,
    pub entries: Vec<Entry>,
    pub sel: Option<Location>,
    pub born: f64,
}

impl SmartApp {
    pub fn open_picker(&mut self, purpose: PickPurpose) {
        let start = self.places.iter().find(|p| p.id == "pictures").or_else(|| self.places.first()).map(|p| p.loc.clone()).unwrap_or_else(|| self.be.home());
        let mut pk = Picker { purpose, dir: start, entries: vec![], sel: None, born: self.shell.now };
        self.picker_load(&mut pk);
        self.shell.desk.picker = Some(pk);
        self.shell.snd.play(crate::sound::Snd::Open);
    }

    fn picker_load(&self, pk: &mut Picker) {
        pk.entries.clear();
        if let Some(fs) = self.be.registry.get(&pk.dir.fs) {
            if let Ok(mut l) = fs.list(&pk.dir.path) {
                l.retain(|e| !e.hidden && (e.is_dir || e.kind == Kind::Image));
                sort_entries(&mut l, SortKey::Name, true, true);
                pk.entries = l;
            }
        }
    }

    pub(crate) fn draw_picker(&mut self, ui: &mut Ui, screen: Rect) {
        let Some(mut pk) = self.shell.desk.picker.take() else { return };
        let ctx = ui.ctx().clone();
        let th = self.th.clone();
        let age = (self.shell.now - pk.born) as f32;
        let pop = spring_out(age).min(1.04);
        ctx.request_repaint();
        let (w, h) = (780.0f32.min(screen.width() - 24.0), 520.0f32.min(screen.height() - TOPBAR_H - 40.0));
        let card = Rect::from_center_size(screen.center() + vec2(0.0, 10.0 + (1.0 - pop) * 18.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((age * 8.0).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        menu_surface(&p, &th, card, 24.0);
        cui.allocate_rect(card, Sense::click());
        let (a, b) = th.accent_grad();
        let tile = Rect::from_min_size(card.min + vec2(20.0, 16.0), vec2(34.0, 34.0));
        grad_fill(&p, tile, 11.0, a, b);
        self.ico(&p, "gallery-wide", Ic::Duo, tile.center(), 18.0, Color32::WHITE);
        p.text(pos2(tile.right() + 12.0, tile.center().y - 8.0), Align2::LEFT_CENTER, "Choose a picture", f_bold(T_TITLE), th.ink1);
        p.text(pos2(tile.right() + 12.0, tile.center().y + 9.0), Align2::LEFT_CENTER, format!("From Files \u{b7} {}", self.pretty_path(&pk.dir)), f_medium(T_META), th.ink3);
        let mut close = self.icon_button(&cui, Rect::from_center_size(pos2(card.right() - 28.0, tile.center().y), vec2(28.0, 28.0)), "close-circle", "pk-x", Btn::Quiet, true).clicked();
        // places rail
        let rail = Rect::from_min_size(pos2(card.left() + 14.0, card.top() + 64.0), vec2(168.0, h - 64.0 - 70.0));
        let mut go: Option<Location> = None;
        for (i, pl) in self.places.iter().take(10).enumerate() {
            let r = Rect::from_min_size(rail.min + vec2(0.0, i as f32 * 34.0), vec2(rail.width(), 30.0));
            let resp = self.hit(&cui, r, ("pk-pl", i), Sense::click());
            let on = pk.dir == pl.loc;
            if on {
                grad_fill(&p, r, 10.0, th.accent_tint, th.accent_tint2);
            } else if resp.hovered() {
                p.rect_filled(r, cr(10.0), th.row_hover);
            }
            self.ico(&p, pl.icon, if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 18.0, r.center().y), 15.0, if on { th.accent_ink } else { th.ink3 });
            p.text(pos2(r.left() + 36.0, r.center().y), Align2::LEFT_CENTER, &pl.title, if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, th.ink1);
            if resp.clicked() {
                go = Some(pl.loc.clone());
            }
        }
        // grid
        let grid = Rect::from_min_max(pos2(card.left() + 194.0, card.top() + 64.0), pos2(card.right() - 16.0, card.bottom() - 70.0));
        let up = Rect::from_min_size(grid.min, vec2(34.0, 30.0));
        if self.icon_button(&cui, up, "arrow-up", "pk-up", Btn::Raised, pk.dir.parent().is_some()).on_hover_text("Enclosing folder").clicked() {
            go = pk.dir.parent();
        }
        let area = Rect::from_min_max(pos2(grid.left(), grid.top() + 40.0), grid.max);
        let cellw = 118.0;
        let cols = ((area.width() / cellw) as usize).max(1);
        let rows = pk.entries.len().div_ceil(cols);
        let off = self.vscroll(&cui, area, "picker", rows as f32 * 128.0 + 8.0);
        let pc = p.with_clip_rect(area);
        let mut budget = 3u32;
        let mut choose: Option<Location> = None;
        for (i, e) in pk.entries.iter().enumerate() {
            let r = Rect::from_min_size(pos2(area.left() + (i % cols) as f32 * cellw, area.top() + (i / cols) as f32 * 128.0 - off), vec2(cellw - 8.0, 120.0));
            if r.bottom() < area.top() || r.top() > area.bottom() {
                continue;
            }
            let resp = cui.interact(r.intersect(area), egui::Id::new(("pk-e", i)), Sense::click());
            let on = pk.sel.as_ref() == Some(&e.loc);
            if on {
                grad_fill(&pc, r, 14.0, alpha(th.accent, 0.22), alpha(th.accent, 0.10));
                pc.rect_stroke(r, cr(14.0), egui::Stroke::new(2.0, th.accent), egui::StrokeKind::Inside);
            } else if resp.hovered() {
                pc.rect_filled(r, cr(14.0), th.row_hover);
            }
            let img = Rect::from_min_size(r.min + vec2(6.0, 6.0), vec2(r.width() - 12.0, 78.0));
            if e.is_dir {
                crate::copyfx::folder_back(&pc, &th, img.center(), 60.0, 46.0, 0.0, 0.0, th.accent);
                crate::copyfx::folder_front(&pc, img.center(), 60.0, 46.0, 0.0, 0.0, th.accent);
            } else if let Some((tex, sz)) = self.photos_thumb(&ctx, &e.loc, &mut budget) {
                textured_rrect(&pc, img, 10.0, tex, cover_uv(sz[0], sz[1], img), Color32::WHITE);
                rim(&pc, img, 10.0, 1.0, Color32::from_white_alpha(140), true, 20.0);
            } else {
                grad_fill(&pc, img, 10.0, alpha(th.ink3, 0.2), alpha(th.ink3, 0.1));
            }
            pc.text(pos2(r.center().x, r.bottom() - 16.0), Align2::CENTER_CENTER, self.fit(&pc, &e.name, &f_semi(11.0), r.width() - 10.0), f_semi(11.0), th.ink1);
            if resp.clicked() {
                if e.is_dir {
                    pk.sel = None;
                } else {
                    pk.sel = Some(e.loc.clone());
                }
            }
            if resp.double_clicked() {
                if e.is_dir {
                    go = Some(e.loc.clone());
                } else {
                    choose = Some(e.loc.clone());
                }
            }
            if e.is_dir && resp.clicked() {
                go = Some(e.loc.clone());
            }
        }
        if pk.entries.is_empty() {
            p.text(area.center(), Align2::CENTER_CENTER, "No pictures here", f_medium(T_BODY), th.ink3);
        }
        p.text(pos2(grid.left() + 44.0, grid.top() + 15.0), Align2::LEFT_CENTER, format!("{} items", pk.entries.len()), f_medium(T_META), th.ink3);
        // footer
        let by = card.bottom() - 18.0 - 34.0;
        let name = pk.sel.as_ref().map(|l| l.name()).unwrap_or_else(|| "Select a picture".into());
        p.text(pos2(card.left() + 22.0, by + 17.0), Align2::LEFT_CENTER, self.fit(&p, &name, &f_semi(T_BODY), 360.0), f_semi(T_BODY), th.ink2);
        if self.label_button(&cui, Rect::from_min_size(pos2(card.right() - 22.0 - 110.0, by), vec2(110.0, 34.0)), None, "Choose", "pk-ok", Btn::Accent).clicked() {
            choose = pk.sel.clone();
        }
        if self.label_button(&cui, Rect::from_min_size(pos2(card.right() - 22.0 - 110.0 - 8.0 - 84.0, by), vec2(84.0, 34.0)), None, "Cancel", "pk-cancel", Btn::Raised).clicked() {
            close = true;
        }
        if let Some(d) = go {
            pk.dir = d;
            pk.sel = None;
            let mut tmp = Picker { purpose: pk.purpose, dir: pk.dir.clone(), entries: vec![], sel: None, born: pk.born };
            self.picker_load(&mut tmp);
            pk.entries = tmp.entries;
        }
        if let Some(l) = choose {
            self.acts.push(Act::WallpaperFrom(l));
            self.shell.desk.wall_open = false;
            return;
        }
        if !close {
            self.shell.desk.picker = Some(pk);
        }
    }
}
