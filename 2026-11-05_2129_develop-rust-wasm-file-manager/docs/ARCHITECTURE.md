# Architecture

```
            ┌────────────────────────────── smart-ui-egui ──────────────────────────────┐
 input ───▶ │ handle_keys → Act  ──▶ SmartApp::act ──▶ Backend (smart-core)              │
            │ chrome / content / overlays / modals  (pure painting + hit-testing)        │
            │ guarded_frame: catch_unwind · native window commands · crash log           │
            └───────────────────────────────▲───────────────────────────────────────────┘
                                            │ Event (mpsc): Listed, Job, Thumb, Devices, Changed…
            ┌────────────────────────────── smart-core ─────────────────────────────────┐
            │ Registry ─ Vfs trait ─┬ LocalFs (std::fs, trash)                           │
            │                       ├ MemFs   (demo / wasm)                              │
            │                       └ AdbFs   (phone)                                    │
            │ ops (copy/move/trash/delete jobs, cancel, panic-safe) · search · thumbs    │
            │ devices (udisks/gio) · apps (.desktop, xdg-mime) · archive · library (JSON)│
            └────────────────────────────────────────────────────────────────────────────┘
```
* **UI never blocks**: it sends a request and receives an `Event`; stale results are dropped by request id.
* **Actions are data** (`enum Act`): menus, the palette, shortcuts, and toolbar buttons all push the same values,
  so every command is reachable from everywhere and testable.
* **Undo** is a bounded stack of `UndoOp` (`Restore`, `Discard`, `Rename`); nothing in it deletes data.
* **Design system**: `assets/tokens.toml` is the contract; `theme.rs` mirrors it; `paint.rs` holds depth primitives.
