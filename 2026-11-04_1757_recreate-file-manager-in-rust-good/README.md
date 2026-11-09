# Smart Files — a native Rust file manager (egui · WebAssembly)

One Rust codebase: a real filesystem engine that runs on Linux / macOS / Windows **and** in the
browser, where the app appears as a floating window over a wallpaper.

## Layers (the frontend is swappable)

| crate | role |
|---|---|
| `crates/smart-core` | **Backend, zero UI.** `Vfs` storage trait (real disk `StdFs`, in-memory `MemFs`, Android over `adb`, cloud connectors), job engine (copy / move / delete / compress / extract with pause, cancel, conflict resolution, undo), pure-Rust archives (zip, tar, tar.gz, tar.xz, tar.bz2, gz, xz, bz2 — read + write), incremental search index + natural-language query parser + `nucleo` fuzzy ranking, on-device assistant with previewable plans, disk cleaner + Space Lens treemap, mount/unmount (`udisksctl` / `diskutil`), default apps (`.desktop` parsing, `open`), Trash (`trash` crate). |
| `Store` + `Action` (smart-core) | UI-agnostic app model. The context-aware appbar, context menus and the command palette are *computed here*, so every frontend gets identical behaviour. |
| `crates/smart-egui` | egui frontend: native frameless window + WebAssembly floating window. Writing a gpui/other frontend means rendering the same `Store`. |

## Build

**Web (single self-contained file):** `./scripts/build-web.sh`
→ `target/web` (wasm-bindgen + `wasm-opt`), `src/glue.js` + `src/wasm.js` (gzip+base64 of the wasm,
packed by `scripts/pack.mjs`), then `npm run build` → **`dist/index.html`** with everything inlined.
The host only serves that one file, so there is no runtime fetching at all.

**Native:** `cargo run -p smart-egui --release`
(`SMART_FILES_DEMO=1` creates `~/Smart Files Demo` including a sparse 5 GB `file transfer.txt`;
`SMART_FILES_DARK=1` starts in dark mode; `--features native-audio` adds rodio playback.)

**Tests:** `cargo test -p smart-core` — real-filesystem round-trips (copy, move, zip/tar.gz
compress + extract, keep-both conflict resolution, index + fuzzy search) and a paced 5 GB transfer.

## Keyboard

Browsers swallow ⌘/Ctrl + letter before any canvas library can see it, so the web build listens in
the capture phase (`src/main.js`) and hands chords to Rust, which applies them according to the
current UI state (e.g. `Ctrl+V` is ignored while you are typing). Native uses egui's key handling.
`⌘`/`Ctrl` is detected per platform, including on the web.

`Ctrl/⌘ K` (or `Ctrl/⌘ Shift P`, `F1`) command palette · `Ctrl/⌘ C/X/V` copy/cut/paste ·
`Ctrl/⌘ A` select all · `Ctrl/⌘ D` duplicate · `Ctrl/⌘ Z` undo · `Ctrl/⌘ I` get info ·
`Ctrl/⌘ Shift N` new folder · `Ctrl/⌘ F` or `/` search · `Ctrl/⌘ J` Ask Smart ·
`Ctrl/⌘ \` sidebar · `Space` Quick Look · `F2` rename · `Delete` trash · `F5` refresh ·
`Alt ←/→/↑` back/forward/up · `Tab` switch palette rail.

## Notable behaviour
- **Transfers are paced** for large copies so the transfer experience is real: paper files stream
  along an arc into a destination that fills like liquid, with live throughput, ETA, pause, cancel,
  run-in-background, per-file conflict resolution (replace / skip / keep both) and undo.
  `…` menu → *Transfer pace* switches to real speed or instant.
- **Drag & drop** works onto folders, sidebar places, devices and the Trash; hold `Alt` to copy.
- **Ask Smart** answers in plain language, plans organisation/dedupe/renames as previews you apply.
- **Storage** view: Space Lens treemap (click to drill in) + cleanup categories with safe defaults.

Screenshots of every major surface are in `docs/screenshots`.
