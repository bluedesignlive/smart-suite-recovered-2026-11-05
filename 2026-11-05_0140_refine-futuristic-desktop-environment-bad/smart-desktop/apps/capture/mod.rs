//! Capture: screenshots (screen / window / region with countdown), screen video and audio-only recording.
//! Everything is driven from Rust: the screenshot comes from the renderer, video from the canvas stream and audio from the
//! microphone through MediaRecorder; screenshots land in Pictures, so Photos sees them immediately.
use crate::app::*;
use crate::appkit::Frame;
use crate::chrome::*;
use crate::icons::Ic;
use crate::paint::*;
use crate::shell::{AppId, NoteKind, TOPBAR_H};
use crate::sound::Snd;
use crate::theme::*;
use crate::widgets::*;
use egui::{pos2, vec2, Align2, Color32, Context, Id, Order, Pos2, Rect, Sense, Ui};
use smart_core::*;

#[derive(Default)]
pub struct Capture {
    pub shots: std::sync::Arc<std::sync::Mutex<Vec<std::sync::Arc<egui::ColorImage>>>>,
    pub mode: usize,
    pub area: usize,
    pub delay: usize,
    pub rec_t0: Option<f64>,
    pub recent: Vec<(String, f64, bool)>,
    pub pending: Option<(f64, Option<Rect>)>,
    pub awaiting: Option<Option<Rect>>,
    pub region_drag: Option<(Pos2, Pos2)>,
    pub selecting: bool,
    pub flash: f64,
}

impl Capture {
    pub fn recording_secs(&self, now: f64) -> Option<f32> {
        self.rec_t0.map(|t| (now - t) as f32)
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};
    thread_local! { static REC: RefCell<Option<web_sys::MediaRecorder>> = const { RefCell::new(None) }; }

    fn download(blob: &web_sys::Blob, name: &str) {
        let Ok(url) = web_sys::Url::create_object_url_with_blob(blob) else { return };
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        if let Ok(a) = doc.create_element("a").and_then(|e| e.dyn_into::<web_sys::HtmlAnchorElement>().map_err(|e| e.into())) {
            a.set_href(&url);
            a.set_download(name);
            a.click();
        }
    }

    fn attach(stream: &web_sys::MediaStream, name: String, mime: &'static str) -> bool {
        let Ok(rec) = web_sys::MediaRecorder::new_with_media_stream(stream) else { return false };
        let chunks: Rc<RefCell<Vec<web_sys::Blob>>> = Rc::new(RefCell::new(vec![]));
        let c2 = chunks.clone();
        let ondata = Closure::<dyn FnMut(web_sys::BlobEvent)>::new(move |e: web_sys::BlobEvent| {
            if let Some(b) = e.data() {
                c2.borrow_mut().push(b);
            }
        });
        rec.set_ondataavailable(Some(ondata.as_ref().unchecked_ref()));
        ondata.forget();
        let c3 = chunks.clone();
        let onstop = Closure::<dyn FnMut()>::new(move || {
            let arr = js_sys::Array::new();
            for b in c3.borrow().iter() {
                arr.push(b);
            }
            let opts = web_sys::BlobPropertyBag::new();
            opts.set_type(mime);
            if let Ok(blob) = web_sys::Blob::new_with_blob_sequence_and_options(&arr, &opts) {
                download(&blob, &name);
            }
        });
        rec.set_onstop(Some(onstop.as_ref().unchecked_ref()));
        onstop.forget();
        if rec.start().is_err() {
            return false;
        }
        REC.with(|r| *r.borrow_mut() = Some(rec));
        true
    }

    pub fn start_video(stamp: &str) -> bool {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return false };
        let Some(canvas) = doc.get_element_by_id("smart-canvas").and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok()) else { return false };
        let Ok(stream) = canvas.capture_stream() else { return false };
        attach(&stream, format!("Smart Recording {stamp}.webm"), "video/webm")
    }

    pub fn start_audio(stamp: String) {
        wasm_bindgen_futures::spawn_local(async move {
            let Some(win) = web_sys::window() else { return };
            let Ok(md) = win.navigator().media_devices() else { return };
            let c = web_sys::MediaStreamConstraints::new();
            c.set_audio(&JsValue::TRUE);
            let Ok(p) = md.get_user_media_with_constraints(&c) else { return };
            if let Ok(s) = wasm_bindgen_futures::JsFuture::from(p).await {
                if let Ok(stream) = s.dyn_into::<web_sys::MediaStream>() {
                    attach(&stream, format!("Smart Voice {stamp}.webm"), "audio/webm");
                }
            }
        });
    }

    pub fn stop() {
        REC.with(|r| {
            if let Some(rec) = r.borrow_mut().take() {
                let _ = rec.stop();
            }
        });
    }
}

impl SmartApp {
    fn stamp(&self) -> String {
        format!("{}-{:02}", clock_hm().replace(':', "-"), now_secs() % 60)
    }

    pub(crate) fn cap_cmd(&mut self, cmd: &str) -> bool {
        let now = self.shell.now;
        match cmd {
            "cap.shot" => {
                let delay: f32 = [0.0, 3.0, 5.0, 10.0][self.shell.cap.delay.min(3)];
                match self.shell.cap.area {
                    2 => {
                        self.shell.cap.selecting = true;
                        self.shell.cap.region_drag = None;
                    }
                    1 => {
                        let f = [AppId::Photos, AppId::Terminal, AppId::Files, AppId::Settings, AppId::Studio].into_iter().find(|a| self.is_visible(*a));
                        let r = f.and_then(|a| self.win_rect_of(a, self.shell.screen));
                        self.shell.cap.pending = Some((now + delay.max(0.25) as f64, r));
                    }
                    _ => self.shell.cap.pending = Some((now + delay.max(0.25) as f64, None)),
                }
            }
            "cap.record" => {
                if self.shell.cap.rec_t0.is_some() {
                    #[cfg(target_arch = "wasm32")]
                    web::stop();
                    self.shell.cap.rec_t0 = None;
                    self.shell.snd.play(Snd::RecStop);
                    self.shell.notify("Recording saved", "Your browser downloaded the file", NoteKind::Ok);
                } else {
                    let stamp = self.stamp();
                    #[cfg(target_arch = "wasm32")]
                    let ok = if self.shell.cap.mode == 2 {
                        web::start_audio(stamp.clone());
                        true
                    } else {
                        web::start_video(&stamp)
                    };
                    #[cfg(not(target_arch = "wasm32"))]
                    let ok = {
                        let _ = &stamp;
                        true
                    };
                    if ok {
                        self.shell.cap.rec_t0 = Some(now);
                        self.shell.snd.play(Snd::RecStart);
                    } else {
                        self.shell.notify("Recording isn\u{2019}t available here", "This browser blocked capture", NoteKind::Err);
                    }
                }
            }
            _ => {}
        }
        true
    }

    /// Per-frame capture driver: region picker overlay, countdown, screenshot request and result.
    pub(crate) fn cap_pump(&mut self, ctx: &Context) {
        let now = self.shell.now;
        let screen = self.shell.screen;
        let lp = ctx.layer_painter(egui::LayerId::new(Order::Tooltip, Id::new("cap-ov")));
        if self.shell.cap.selecting {
            ctx.request_repaint();
            ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
            let (pressed, down, rel, pos) = ctx.input(|i| (i.pointer.primary_pressed(), i.pointer.primary_down(), i.pointer.primary_released(), i.pointer.latest_pos()));
            if let Some(pp) = pos {
                if pressed {
                    self.shell.cap.region_drag = Some((pp, pp));
                } else if down {
                    if let Some((a, _)) = self.shell.cap.region_drag {
                        self.shell.cap.region_drag = Some((a, pp));
                    }
                }
            }
            let sel = self.shell.cap.region_drag.map(|(a, b)| Rect::from_two_pos(a, b));
            let dim = Color32::from_black_alpha(110);
            match sel {
                Some(r) => {
                    lp.rect_filled(Rect::from_min_max(screen.min, pos2(screen.max.x, r.top())), 0.0, dim);
                    lp.rect_filled(Rect::from_min_max(pos2(screen.min.x, r.bottom()), screen.max), 0.0, dim);
                    lp.rect_filled(Rect::from_min_max(pos2(screen.min.x, r.top()), pos2(r.left(), r.bottom())), 0.0, dim);
                    lp.rect_filled(Rect::from_min_max(pos2(r.right(), r.top()), pos2(screen.max.x, r.bottom())), 0.0, dim);
                    lp.rect_stroke(r, 0.0, egui::Stroke::new(1.5, Color32::WHITE), egui::StrokeKind::Outside);
                    lp.text(r.right_bottom() + vec2(-4.0, 14.0), Align2::RIGHT_CENTER, format!("{}\u{d7}{}", r.width() as i32, r.height() as i32), f_mono_b(11.0), Color32::WHITE);
                }
                None => {
                    lp.rect_filled(screen, 0.0, Color32::from_black_alpha(70));
                    lp.text(screen.center(), Align2::CENTER_CENTER, "Drag to choose an area  \u{b7}  Esc to cancel", f_semi(16.0), Color32::WHITE);
                }
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.shell.cap.selecting = false;
            } else if rel {
                self.shell.cap.selecting = false;
                if let Some(r) = sel.filter(|r| r.width() > 8.0 && r.height() > 8.0) {
                    self.shell.cap.pending = Some((now + 0.2, Some(r)));
                }
            }
        }
        if let Some((t, r)) = self.shell.cap.pending {
            let left = t - now;
            if left > 0.3 {
                ctx.request_repaint();
                lp.text(screen.center(), Align2::CENTER_CENTER, format!("{}", left.ceil() as u32), f_mono_b(120.0), alpha(Color32::WHITE, 0.9));
                let n = left.ceil() as u32;
                let last: u32 = ctx.data_mut(|d| d.get_temp(Id::new("cap-n")).unwrap_or(0));
                if last != n {
                    ctx.data_mut(|d| d.insert_temp(Id::new("cap-n"), n));
                    self.shell.snd.play(Snd::Tick);
                }
            } else if left <= 0.0 {
                self.shell.cap.pending = None;
                self.shell.cap.awaiting = Some(r);
                let slot = self.shell.cap.shots.clone();
                ctx.request_screenshot(move |img| slot.lock().unwrap().push(img));
                ctx.request_repaint();
            }
        }
        let shot = self.shell.cap.shots.lock().unwrap().pop();
        if let Some(img) = shot {
            let region = self.shell.cap.awaiting.take().flatten();
            let (w, h) = (img.size[0] as u32, img.size[1] as u32);
            let mut buf = Vec::with_capacity((w * h * 4) as usize);
            for px in &img.pixels {
                buf.extend_from_slice(&px.to_array());
            }
            if let Some(im) = image::RgbaImage::from_raw(w, h, buf) {
                let k = w as f32 / screen.width().max(1.0);
                let im = match region {
                    Some(r) => {
                        let x = (r.left() * k).max(0.0) as u32;
                        let y = (r.top() * k).max(0.0) as u32;
                        let cw = ((r.width() * k) as u32).min(w.saturating_sub(x)).max(1);
                        let ch = ((r.height() * k) as u32).min(h.saturating_sub(y)).max(1);
                        image::imageops::crop_imm(&im, x, y, cw, ch).to_image()
                    }
                    None => im,
                };
                let mut out = vec![];
                let _ = image::DynamicImage::ImageRgba8(im).write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png);
                let name = format!("Screenshot {}.png", self.stamp());
                if let Some(pl) = self.places.iter().find(|p| p.id == "pictures").map(|p| p.loc.clone()) {
                    if let Some(fs) = self.be.registry.get(&pl.fs) {
                        let _ = fs.write(&pl.join(&name).path, &out);
                    }
                }
                self.shell.cap.flash = now;
                self.shell.cap.recent.insert(0, (name.clone(), now, false));
                self.shell.snd.play(Snd::Shutter);
                self.shell.notify("Screenshot saved", &format!("{name} \u{b7} Pictures"), NoteKind::Ok);
                self.acts.push(Act::Refresh);
            }
        }
        let fl = (1.0 - (now - self.shell.cap.flash) as f32 / 0.35).clamp(0.0, 1.0);
        if fl > 0.0 {
            ctx.request_repaint();
            lp.rect_filled(screen, 0.0, Color32::from_white_alpha((fl * 150.0) as u8));
        }
        let _ = TOPBAR_H;
    }

    pub fn draw_capture(&mut self, ui: &mut Ui, fr: &Frame) {
        let th = self.th.clone();
        let p = ui.painter().clone();
        let t = self.time as f32;
        let rec = self.shell.cap.rec_t0.is_some();
        if fr.has_side() {
            p.text(pos2(fr.side_body.left() + 20.0, fr.side_body.top() + 6.0), Align2::LEFT_CENTER, "RECENT", f_semi(9.5), th.ink4);
            if self.shell.cap.recent.is_empty() {
                p.text(pos2(fr.side_body.left() + 20.0, fr.side_body.top() + 34.0), Align2::LEFT_CENTER, "Captures you take show up here", f_medium(T_META), th.ink4);
            }
            for (i, (n, _, _)) in self.shell.cap.recent.clone().iter().take(10).enumerate() {
                let r = Rect::from_min_size(pos2(fr.side_body.left() + 10.0, fr.side_body.top() + 22.0 + i as f32 * 36.0), vec2(fr.side_body.width() - 20.0, 32.0));
                let resp = self.hit(ui, r, ("cap-r", i), Sense::click());
                if resp.hovered() {
                    p.rect_filled(r, cr(10.0), th.row_hover);
                }
                self.ico(&p, "gallery", Ic::Duo, pos2(r.left() + 18.0, r.center().y), 15.0, th.accent_ink);
                p.text(pos2(r.left() + 36.0, r.center().y), Align2::LEFT_CENTER, self.fit(&p, n, &f_medium(T_META), r.width() - 44.0), f_medium(T_META), th.ink2);
                if resp.clicked() {
                    self.open_app(AppId::Photos);
                }
            }
        }
        let st = fr.stage;
        let cx = st.center().x;
        // mode switch
        let seg = Rect::from_center_size(pos2(cx, st.top() + 34.0), vec2(330.0f32.min(st.width() - 40.0), 34.0));
        if !rec {
            if let Some(i) = self.segment(ui, seg, &[("", "Screenshot"), ("", "Screen video"), ("", "Audio")], false, self.shell.cap.mode, "cap-mode") {
                self.shell.cap.mode = i;
                self.shell.snd.play(Snd::Toggle);
            }
        }
        // options
        let mode = self.shell.cap.mode;
        let oy = st.top() + 84.0;
        if mode == 0 && !rec {
            let a = Rect::from_center_size(pos2(cx - 120.0, oy), vec2(230.0, 30.0));
            if let Some(i) = self.segment(ui, a, &[("", "Screen"), ("", "Window"), ("", "Region")], false, self.shell.cap.area, "cap-area") {
                self.shell.cap.area = i;
            }
            let d = Rect::from_center_size(pos2(cx + 140.0, oy), vec2(190.0, 30.0));
            if let Some(i) = self.segment(ui, d, &[("", "Now"), ("", "3s"), ("", "5s"), ("", "10s")], false, self.shell.cap.delay, "cap-delay") {
                self.shell.cap.delay = i;
            }
        } else {
            let msg = match (mode, rec) {
                (1, false) => "Records everything on the desktop at the display\u{2019}s frame rate",
                (2, false) => "Records your microphone \u{2014} nothing else leaves the machine",
                (_, true) => "Recording \u{2014} press stop when you are done",
                _ => "",
            };
            p.text(pos2(cx, oy), Align2::CENTER_CENTER, msg, f_medium(T_BODY), th.ink3);
        }
        // the shutter
        let ctr = pos2(cx, st.center().y + 22.0);
        let r = 46.0;
        let resp = self.hit(ui, Rect::from_center_size(ctr, vec2(r * 2.3, r * 2.3)), "cap-shutter", Sense::click());
        let down = resp.is_pointer_button_down_on();
        for i in 0..8 {
            p.circle_filled(ctr + vec2(0.0, 6.0 + i as f32 * 1.4), r + 4.0 + i as f32 * 1.6, alpha(th.shadow, 0.045));
        }
        dome(&p, ctr, r + 5.0, if th.dark { c(20, 26, 40, 1.0) } else { c(150, 164, 190, 1.0) }, if th.dark { c(104, 116, 144, 1.0) } else { c(255, 255, 255, 1.0) });
        p.circle_filled(ctr, r - 4.0, if th.dark { c(6, 9, 18, 1.0) } else { c(196, 207, 228, 1.0) });
        let (c0, c1) = if mode == 0 { (hex("#ffffff"), hex("#c8d3ea")) } else { (hex("#ff7a7a"), hex("#c8283c")) };
        let pulse = if rec { 0.5 + 0.5 * (t * 4.0).sin() } else { 0.0 };
        let inner = (r - 11.0) * if down { 0.92 } else { 1.0 } * if rec { 0.62 } else { 1.0 };
        if rec {
            p.circle_filled(ctr, r + 8.0 + 6.0 * pulse, alpha(hex("#ff4d4d"), 0.10 + 0.10 * pulse));
            let sq = Rect::from_center_size(ctr, vec2(inner * 1.2, inner * 1.2));
            grad_fill(&p, sq, 9.0, c0, c1);
            rim(&p, sq, 9.0, 1.0, Color32::from_white_alpha(140), true, 20.0);
        } else {
            dome(&p, ctr, inner, c1, c0);
            p.circle_filled(ctr + vec2(-inner * 0.25, -inner * 0.42), inner * 0.30, Color32::from_white_alpha(70));
            if mode == 0 {
                self.ico(&p, "camera", Ic::Duo, ctr, 26.0, hex("#5a6684"));
            } else {
                self.ico(&p, if mode == 1 { "videocamera-record" } else { "microphone" }, Ic::Filled, ctr, 26.0, Color32::WHITE);
            }
        }
        if resp.clicked() {
            self.acts.push(Act::App(if mode == 0 { "cap.shot" } else { "cap.record" }.into()));
        }
        // timer + visualiser
        let label = match (mode, rec) {
            (0, _) => "Capture".to_string(),
            (_, true) => {
                let s = self.shell.cap.recording_secs(self.shell.now).unwrap_or(0.0);
                format!("{:02}:{:02}", (s / 60.0) as u32, (s % 60.0) as u32)
            }
            (1, false) => "Record screen".into(),
            _ => "Record audio".into(),
        };
        p.text(pos2(cx, ctr.y + r + 34.0), Align2::CENTER_CENTER, label, if rec { f_mono_b(24.0) } else { f_semi(T_TITLE) }, if rec { th.danger } else { th.ink1 });
        if mode == 2 || rec {
            let n = 44;
            let w = (st.width() - 120.0).min(460.0);
            for i in 0..n {
                let k = i as f32 / n as f32;
                let env = (k * std::f32::consts::PI).sin();
                let amp = if rec { 0.25 + 0.75 * hash01(i as u32 * 17 + (t * 16.0) as u32) } else { 0.12 + 0.1 * (t * 2.0 + i as f32 * 0.4).sin().abs() };
                let h = 4.0 + 46.0 * amp * env;
                let x = cx - w / 2.0 + k * w;
                let y = st.bottom() - 52.0;
                p.line_segment([pos2(x, y - h / 2.0), pos2(x, y + h / 2.0)], egui::Stroke::new(3.0, alpha(if rec { hex("#ff5a5a") } else { th.accent }, 0.55 + 0.4 * amp)));
            }
        }
        if rec {
            ui.ctx().request_repaint();
        }
    }
}
