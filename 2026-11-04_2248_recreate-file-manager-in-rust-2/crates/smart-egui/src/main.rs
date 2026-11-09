//! Native entry point: a frameless, floating window on Linux / macOS / Windows.
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 820.0]).with_min_inner_size([760.0, 520.0]).with_decorations(false).with_transparent(true).with_title("Smart Files"),
        ..Default::default()
    };
    eframe::run_native(
        "Smart Files",
        opts,
        Box::new(|cc| {
            let mut eng = smart_core::Engine::native();
            if std::env::var("SMART_FILES_DEMO").is_ok() {
                let _ = smart_core::demo::create_native_workspace(&mut eng);
            }
            let dark = std::env::var("SMART_FILES_DARK").is_ok();
            Ok(Box::new(smart_egui::App::new(cc, smart_core::Store::new(eng, false), false, dark)))
        }),
    )
}
#[cfg(target_arch = "wasm32")]
fn main() {}
