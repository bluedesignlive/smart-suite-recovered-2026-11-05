//! Files as the system file picker: the same places, thumbnails, search and breadcrumb model as the manager, in a
//! chooser the desktop (or any app) can summon. Hands back a `Location` — no private paths.
use crate::app::*;
use crate::chrome::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::shell::TOPBAR_H;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Id, Rect, Sense, Ui};
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
    pub search: String,
    pub focus_search: bool,
}

impl SmartApp {
    pub fn open_picker(&mut self, purpose: PickPurpose) {
        let start = self.places.iter().find(|p| p.id == "pictures").or_else(|| self.places.first()).map(|p| p.loc.clone()).unwrap_or_else(|| self.be.home());
        let mut pk = Picker { purpose, dir: start, entries: vec![], sel: None, born: self.shell.now, search: String::new(), focus_search: false };
        self.picker_load(&mut pk);
        self.shell.desk.picker = Some(pk);
        self.shell.snd.play(crate::sound::Snd::Open);
    }

    fn picker_load(&self, pk: &mut Picker) {
        pk.entries.clear();
        if let Some(fs) = self.be.registry.get(&pk.dir.fs) {
            if let Ok(mut l) = fs.list(&pk.dir.path) {
                l.retain(|e| !e.hidden && (e.is_dir || e.kind == Kind::Image));
                if !pk.search.is_empty() {
                    let q = pk.search.to_lowercase();
                    l.retain(|e| e.name.to_lowercase().contains(&q));
                }
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
        let (w, h) = (860.0f32.min(screen.width() - 24.0), 560.0f32.min(screen.height() - TOPBAR_H - 40.0));
        let card = Rect::from_center_size(screen.center() + vec2(0.0, 10.0 + (1.0 - pop) * 18.0), vec2(w, h));
        let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(screen));
        cui.set_opacity((age * 8.0).clamp(0.0, 1.0));
        let p = cui.painter().clone();
        elevate(&p, &th, card, 24.0, 5);
        menu_surface(&p, &th, card, 24.0);
        cui.allocate_rect(card, Sense::click());
        let (a, b) = th.accent_grad();
        let tile = Rect::from_min_size(card.min + vec2(20.0, 16.0), vec2(34.0, 34.0));
        grad_fill(&p, tile, 11.0, a, b);
        rim(&p, tile, 11.0, 1.0, Color32::from_white_alpha(140), true, 16.0);
        self.ico(&p, "gallery-wide", Ic::Duo, tile.center(), 18.0, Color32::WHITE);
        p.text(pos2(tile.right() + 12.0, tile.center().y - 8.0), Align2::LEFT_CENTER, "Choose a picture", f_bold(T_TITLE), th.ink1);
        p.text(pos2(tile.right() + 12.0, tile.center().y + 9.0), Align2::LEFT_CENTER, "From Files", f_medium(T_META), th.ink3);
        let mut close = self.icon_button(&cui, Rect::from_center_size(pos2(card.right() - 28.0, tile.center().y), vec2(28.0, 28.0)), "close-circle", "pk-x", Btn::Quiet, true).clicked();
        // search field in the header
        let sr = Rect::from_min_size(pos2(card.right() - 268.0, tile.center().y - 15.0), vec2(200.0, 30.0));
        well(&p, &th, sr, 15.0);
        self.ico(&p, "magnifer", Ic::Outline, pos2(sr.left() + 14.0, sr.center().y), 14.0, th.ink3);
        let mut q = std::mem::take(&mut pk.search);
        let te = egui::TextEdit::singleline(&mut q).font(f_medium(12.5)).hint_text("Search this folder").frame(egui::Frame::NONE).text_color(th.ink1).desired_width(sr.width() - 40.0);
        let sresp = cui.put(Rect::from_min_size(pos2(sr.left() + 28.0, sr.center().y - 12.0), vec2(sr.width() - 36.0, 24.0)), te);
        pk.search = q;
        if pk.focus_search {
            sresp.request_focus();
            pk.focus_search = false;
        }
        if sresp.changed() {
            let mut tmp = pk.entries.clone();
            tmp.clear();
            self.picker_load(&mut pk);
        }
        // places rail
        let rail = Rect::from_min_size(pos2(card.left() + 14.0, card.top() + 66.0), vec2(172.0, h - 66.0 - 72.0));
        let mut go: Option<Location> = None;
        for (i, pl) in self.places.iter().take(10).enumerate() {
            let r = Rect::from_min_size(rail.min + vec2(0.0, i as f32 * 34.0), vec2(rail.width(), 30.0));
            let resp = self.hit(&cui, r, ("pk-pl", i), Sense::click());
            let on = pk.dir == pl.loc;
            if on {
                grad_fill(&p, r, 10.0, th.accent_tint, th.accent_tint2);
                rim(&p, r, 10.0, 1.0, alpha(Color32::WHITE, if th.dark { 0.1 } else { 0.7 }), true, 12.0);
            } else if resp.hovered() {
                p.rect_filled(r, cr(10.0), th.row_hover);
            }
            self.ico(&p, pl.icon, if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 18.0, r.center().y), 15.0, if on { th.accent_ink } else { th.ink3 });
            p.text(pos2(r.left() + 36.0, r.center().y), Align2::LEFT_CENTER, &pl.title, if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, if on { th.ink1 } else { th.ink2 });
            if resp.clicked() {
                go = Some(pl.loc.clone());
                pk.search.clear();
            }
        }
        // breadcrumb bar above the grid
        let grid = Rect::from_min_max(pos2(card.left() + 200.0, card.top() + 66.0), pos2(card.right() - 16.0, card.bottom() - 72.0));
        let crumb_y = grid.top() + 15.0;
        if let Some(up) = pk.dir.parent() {
            let ub = Rect::from_center_size(pos2(grid.left() + 17.0, crumb_y), vec2(34.0, 30.0));
            if self.icon_button(&cui, ub, "arrow-up", "pk-up", Btn::Raised, true).on_hover_text("Enclosing folder").clicked() {
                go = Some(up);
            }
        }
        let mut bx = grid.left() + 44.0;
        for anc in pk.dir.ancestors().iter().rev().chain(std::iter::once(&pk.dir)) {
            let name = if anc.is_root() { self.pretty_path(anc) } else { anc.name() };
            let is_last = anc == &pk.dir;
            let bw = self.tw(&p, &name, &if is_last { f_semi(T_BODY) } else { f_medium(T_BODY) }) + 16.0;
            if bx + bw > grid.right() - 60.0 {
                break;
            }
            let r = Rect::from_min_size(pos2(bx, crumb_y - 14.0), vec2(bw, 28.0));
            let resp = self.hit(&cui, r, ("pk-crumb", anc.path.clone()), Sense::click());
            if resp.hovered() && !is_last {
                p.rect_filled(r, cr(8.0), th.row_hover);
            }
            p.text(r.center(), Align2::CENTER_CENTER, &name, if is_last { f_semi(T_BODY) } else { f_medium(T_BODY) }, if is_last { th.ink1 } else { th.ink3 });
            if resp.clicked() && !is_last {
                go = Some(anc.clone());
            }
            bx += bw + 2.0;
            if !is_last {
                p.text(pos2(bx, crumb_y), Align2::CENTER_CENTER, "\u{203a}", f_medium(T_BODY), th.ink4);
                bx += 12.0;
            }
        }
        // count + size
        let total_size: u64 = pk.entries.iter().map(|e| e.size).sum();
        p.text(pos2(grid.right(), crumb_y), Align2::RIGHT_CENTER, format!("{} items \u{b7} {}", pk.entries.len(), format_size(total_size)), f_medium(T_META), th.ink3);
        p.line_segment([pos2(grid.left(), grid.top() + 30.0), pos2(grid.right(), grid.top() + 30.0)], egui::Stroke::new(1.0, th.seam));
        // grid
        let area = Rect::from_min_max(pos2(grid.left(), grid.top() + 40.0), grid.max);
        let cellw = 122.0;
        let cols = ((area.width() / cellw) as usize).max(1);
        let rows = pk.entries.len().div_ceil(cols);
        let off = self.vscroll(&cui, area, "picker", rows as f32 * 126.0 + 8.0);
        let pc = p.with_clip_rect(area);
        let mut budget = 4u32;
        let mut choose: Option<Location> = None;
        let mut sel_focus = false;
        for (i, e) in pk.entries.iter().enumerate() {
            let r = Rect::from_min_size(pos2(area.left() + (i % cols) as f32 * cellw, area.top() + (i / cols) as f32 * 126.0 - off), vec2(cellw - 10.0, 118.0));
            if r.bottom() < area.top() || r.top() > area.bottom() {
                continue;
            }
            let resp = cui.interact(r.intersect(area), Id::new(("pk-e", i)), Sense::click());
            let on = pk.sel.as_ref() == Some(&e.loc);
            let hv = self.anim(&cui, ("pk-h", i), resp.hovered());
            if on {
                grad_fill(&pc, r, 14.0, alpha(th.accent, 0.22), alpha(th.accent, 0.10));
                pc.rect_stroke(r, cr(14.0), egui::Stroke::new(2.0, th.accent), egui::StrokeKind::Inside);
            } else if hv > 0.01 {
                pc.rect_filled(r, cr(14.0), alpha(th.row_hover, hv));
            }
            let img = Rect::from_min_size(r.min + vec2(7.0, 7.0 - hv * 1.5), vec2(r.width() - 14.0, 76.0));
            if e.is_dir {
                crate::copyfx::folder_back(&pc, &th, img.center(), 52.0, 40.0, 0.0, 0.0, th.accent);
                crate::copyfx::folder_front(&pc, img.center(), 52.0, 40.0, 0.0, 0.0, th.accent);
            } else if let Some((tex, sz)) = self.photos_thumb(&ctx, &e.loc, &mut budget) {
                shadow(&pc, img, 9.0, 3.0, 7.0, -2.0, alpha(th.shadow, 0.25));
                textured_rrect(&pc, img, 10.0, tex, cover_uv(sz[0], sz[1], img), Color32::WHITE);
                rim(&pc, img, 10.0, 1.0, Color32::from_white_alpha(140), true, 20.0);
            } else {
                grad_fill(&pc, img, 10.0, alpha(th.ink3, 0.2), alpha(th.ink3, 0.1));
            }
            pc.text(pos2(r.center().x, r.bottom() - 15.0), Align2::CENTER_CENTER, self.fit(&pc, &e.name, &f_semi(11.0), r.width() - 8.0), f_semi(11.0), th.ink1);
            if resp.clicked() {
                if e.is_dir {
                    go = Some(e.loc.clone());
                } else {
                    pk.sel = Some(e.loc.clone());
                    sel_focus = true;
                }
            }
            if resp.double_clicked() && !e.is_dir {
                choose = Some(e.loc.clone());
            }
        }
        if pk.entries.is_empty() {
            p.text(area.center(), Align2::CENTER_CENTER, if pk.search.is_empty() { "No pictures in this folder" } else { "Nothing matches your search" }, f_medium(T_BODY), th.ink3);
        }
        let _ = sel_focus;
        // footer
        let by = card.bottom() - 18.0 - 34.0;
        let name = pk.sel.as_ref().map(|l| l.name()).unwrap_or_else(|| "Select a picture".into());
        let sel_size = pk.sel.as_ref().and_then(|l| self.be.stat(l).ok()).map(|e| format_size(e.size)).unwrap_or_default();
        p.text(pos2(card.left() + 22.0, by + 9.0), Align2::LEFT_CENTER, self.fit(&p, &name, &f_semi(T_BODY), 420.0), f_semi(T_BODY), th.ink1);
        if !sel_size.is_empty() {
            p.text(pos2(card.left() + 22.0, by + 26.0), Align2::LEFT_CENTER, sel_size, f_medium(T_META), th.ink3);
        }
        if self.label_button(&cui, Rect::from_min_size(pos2(card.right() - 22.0 - 96.0, by), vec2(96.0, 34.0)), None, "Choose", "pk-ok", Btn::Accent).clicked() {
            choose = pk.sel.clone();
        }
        if self.label_button(&cui, Rect::from_min_size(pos2(card.right() - 22.0 - 96.0 - 8.0 - 78.0, by), vec2(78.0, 34.0)), None, "Cancel", "pk-cancel", Btn::Raised).clicked() {
            close = true;
        }
        // Enter chooses the selection
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && pk.sel.is_some() {
            choose = pk.sel.clone();
        }
        if let Some(d) = go {
            pk.dir = d;
            pk.sel = None;
            let mut tmp = Picker { purpose: pk.purpose, dir: pk.dir.clone(), entries: vec![], sel: None, born: pk.born, search: pk.search.clone(), focus_search: false };
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
