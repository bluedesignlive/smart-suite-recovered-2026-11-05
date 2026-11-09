//! Audio preview playback: HTMLAudioElement on the web, rodio natively (feature `native-audio`).
use smart_core::{PlayerCmd, Store};

#[derive(Default)]
pub struct Audio {
    #[cfg(target_arch = "wasm32")]
    el: Option<web_sys::HtmlAudioElement>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "native-audio"))]
    out: Option<(rodio::OutputStream, rodio::Sink)>,
}

impl Audio {
    pub fn handle(&mut self, cmd: PlayerCmd, store: &Store) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            match cmd {
                PlayerCmd::Start(p) => {
                    if let Some(el) = self.el.take() {
                        let _ = el.pause();
                    }
                    let Ok(bytes) = store.eng.read_all(&p, 64_000_000) else { return };
                    let arr = js_sys::Uint8Array::from(bytes.as_slice());
                    let parts = js_sys::Array::new();
                    parts.push(&arr);
                    let opts = web_sys::BlobPropertyBag::new();
                    opts.set_type("audio/wav");
                    if let Ok(blob) = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts) {
                        if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                            if let Ok(el) = web_sys::HtmlAudioElement::new_with_src(&url) {
                                let _ = el.play();
                                self.el = Some(el);
                            }
                        }
                    }
                    let _ = wasm_bindgen::JsValue::NULL.dyn_ref::<web_sys::Window>();
                }
                PlayerCmd::Pause => {
                    if let Some(el) = &self.el {
                        let _ = el.pause();
                    }
                }
                PlayerCmd::Resume => {
                    if let Some(el) = &self.el {
                        let _ = el.play();
                    }
                }
                PlayerCmd::Stop => {
                    if let Some(el) = self.el.take() {
                        let _ = el.pause();
                    }
                }
            }
        }
        #[cfg(all(not(target_arch = "wasm32"), feature = "native-audio"))]
        {
            match cmd {
                PlayerCmd::Start(p) => {
                    self.out = None;
                    if let Ok(bytes) = store.eng.read_all(&p, 256_000_000) {
                        if let Ok((stream, handle)) = rodio::OutputStream::try_default() {
                            if let Ok(sink) = rodio::Sink::try_new(&handle) {
                                if let Ok(src) = rodio::Decoder::new(std::io::Cursor::new(bytes)) {
                                    sink.append(src);
                                    sink.play();
                                    self.out = Some((stream, sink));
                                }
                            }
                        }
                    }
                }
                PlayerCmd::Pause => {
                    if let Some((_, s)) = &self.out {
                        s.pause()
                    }
                }
                PlayerCmd::Resume => {
                    if let Some((_, s)) = &self.out {
                        s.play()
                    }
                }
                PlayerCmd::Stop => self.out = None,
            }
        }
        #[cfg(all(not(target_arch = "wasm32"), not(feature = "native-audio")))]
        {
            let _ = (cmd, store);
        }
    }
}
