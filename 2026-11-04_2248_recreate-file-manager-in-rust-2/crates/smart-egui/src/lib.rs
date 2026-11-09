//! Smart Files — egui frontend. The same `App` runs natively and in the browser (WebAssembly).
mod app;
mod apps;
mod audio;
mod chrome;
mod content;
mod dialogs;
mod icons;
mod overlays;
mod paint;
mod shell;
mod theme;
mod thumbs;

pub use app::App;

#[cfg(target_arch = "wasm32")]
mod web {
    use super::App;
    use smart_core::{Engine, Store};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    /// Keyboard chords observed in the DOM. Browsers own modifier keys (⌘/Ctrl + letter is
    /// often swallowed before egui sees it), so the loader-level listener forwards them here and
    /// the app drains the queue every frame.
    thread_local! {
        static CHORDS: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
    }
    fn push_chord(name: &str) {
        CHORDS.with(|c| c.borrow_mut().push(name.to_string()));
    }
    pub fn drain_chords() -> Vec<String> {
        CHORDS.with(|c| std::mem::take(&mut *c.borrow_mut()))
    }
    pub fn is_mac() -> bool {
        web_sys::window()
            .and_then(|w| w.navigator().platform().ok())
            .map(|p| p.to_lowercase().contains("mac"))
            .unwrap_or(false)
    }

    /// Receives a keyboard chord from the loader (`src/main.js`, capture phase — browsers and
    /// winit swallow modifier chords before they bubble to anything canvas-side).
    #[wasm_bindgen]
    pub fn chord(name: &str) {
        push_chord(name);
    }

    /// Called from index.html: mounts the app on a canvas. The browser build runs on the in-memory demo world.
    #[wasm_bindgen]
    pub async fn start(canvas_id: &str, reduced: bool, coarse: bool) -> Result<(), JsValue> {
        console_error_panic_hook::set_once();
        let win = web_sys::window().ok_or("no window")?;
        let doc = win.document().ok_or("no document")?;
        let canvas = doc.get_element_by_id(canvas_id).ok_or("canvas missing")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let search = win.location().search().unwrap_or_default();
        let dark = search.contains("dark");
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(move |cc| {
                    let mut app = App::new(cc, Store::new(Engine::demo(), true), true, dark);
                    app.reduced = reduced;
                    app.coarse = coarse;
                    Ok(Box::new(app))
                }),
            )
            .await
    }
}
