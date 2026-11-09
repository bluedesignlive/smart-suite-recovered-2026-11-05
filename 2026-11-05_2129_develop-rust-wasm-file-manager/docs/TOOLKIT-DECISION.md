# Why egui (and not iced or gpui)?

The brief: *efficient, portable (Linux, Windows, macOS, FreeBSD, wasm), premium look, never glitch.*
I did not take "egui is wasteful" on faith — I checked what the claim actually means for **this** app.

## 1. The performance claim
Immediate-mode GUIs *can* burn CPU if they repaint 60 times a second forever. egui does not do that: eframe
runs **reactively** — a frame is drawn only after input, a timer, or an explicit `request_repaint*`.
This code base follows that rule: the only unconditional repaints are while an animation is actually moving
(sidebar slide, toast fade, transfer animation); polling uses `request_repaint_after(30–500 ms)` and only while
audio plays, a search is running, or a terminal is open. An idle window therefore costs ~0 % CPU — measured in
[docs/SAFETY.md](SAFETY.md#measured). Rendering is a single glow batch per frame; the file list is virtualised.

## 2. The alternatives

| | egui 0.37 (kept) | iced 0.13 | gpui |
|---|---|---|---|
| Linux Wayland+X11 | yes (winit) | yes (winit) | yes, but young; needs Vulkan (blade) |
| Windows / macOS | yes | yes | yes |
| **FreeBSD** | yes | yes | **no** |
| **WebAssembly** | **yes, production** | yes (WebGL / wgpu) | **no** |
| Stable release on crates.io | yes | yes | **no** — git dependency on Zed's monorepo, API churn |
| Software-GL fallback (VMs, old GPUs) | yes (glow/GLES2) | wgpu needs Vulkan/GL 3.3+ | **no** |
| Cost to adopt here | 0 | rewrite ≈ 18 k lines of custom-painted UI | rewrite + lose wasm preview |

* **gpui** fails three stated requirements outright (wasm, FreeBSD, stable dependency) and has no GL fallback.
* **iced** is a fine toolkit, but its retained Elm model means re-implementing every custom-painted widget of the
  design system (depth primitives, glass, spring animations, the copy animation, terminal, photo editor) from
  scratch. That is weeks of work with a high regression risk, for a performance gain that does not exist for an
  event-driven app. Not a responsible trade for "the most used app in the OS".

**Decision: keep egui**, invest the time in correctness, safety, packaging and the missing file-manager features
instead. The core is toolkit-free (`smart-core` has no UI code), so a future iced front-end remains possible.

## 3. "Zero cost" in practice
* All file-system work runs on worker threads (rayon / std threads); the UI thread only draws.
* `lto="fat"`, `codegen-units=1`, `opt-level=3`, stripped (native); `opt-level="s"` + LTO (wasm).
* Thumbnails and listings are cached and de-duplicated; search is incremental and cancellable.
