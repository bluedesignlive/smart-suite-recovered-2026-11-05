# Smart Shell

An experimental desktop shell written in Rust: a file manager and a few apps that share one design system
(`assets/tokens.toml`) and one codebase. It runs natively (Linux first) and, as a preview, in the browser through
WebAssembly.

**What this is not:** a finished product, or an OS. Windows, macOS and FreeBSD builds are untested. The web preview
uses an in-memory demo file system, so nothing in it touches your disk.

## Is this "a smart suite"?

Yes, in the sense that GNOME Core, KDE Gear and the apps bundled with macOS are suites: one design language, shared
components, apps that hand work to each other (Files opens videos in Eiga, folders in Terminal, images in Photos).
The prototype's job is to prove that one Rust toolkit and one token file can drive all of them.

## Apps

| App | What it does | Desktop | Web preview |
|---|---|---|---|
| **Files** | Tabs, search, tags, archives, devices, undoable copy/move, Open With | real files | demo files |
| **Storage** (a tab inside Files) | Overview, Cleanup, Treemap, Folder graph, Biggest | scans your disk on a worker thread | scans the demo drive |
| **Terminal** | A real shell on a pty (vt100 screen, scrollback, selection, resize), or Smart command blocks | `$SHELL` | virtual shell on the demo file system |
| **Photos** | Library, viewer, light editing | yes | yes |
| **Eiga (映画)** | Video player: mpv-style keys, subtitles, chapters, A-B loop, playlists | decodes with `ffmpeg` | built-in test card |

## Storage workspace

Click the drive card at the bottom of the sidebar (it turns amber at 80 % and red at 90 %) to open Storage in a new tab.

* **Overview**: usage gauge, content breakdown, the biggest wins, a warning banner when the drive is nearly full.
* **Cleanup**: *context-aware* junk detection, not just "big files": `target/` next to a `Cargo.toml`, `node_modules`,
  `.venv`, `__pycache__`, framework caches, `~/.cache`, `~/.cargo/registry`, `~/.npm/_cacache`, the Trash, partial
  downloads, huge logs, old installers in Downloads, "Report (1).pdf" copies and large files nobody touched for months.
  Every item has a safety grade (Safe, Review, Careful) and, where it applies, how it comes back (`cargo build`,
  `npm install`). Nothing is deleted without a confirmation, and everything goes to the Trash.
* **Map**: squarified treemap with drill-down. **Graph**: force-directed, Obsidian-style view of the folder tree
  (drag nodes, zoom, pan, click to focus, double-click to open). **Biggest**: largest files and folders.

The scanner (`smart-core/src/analyze.rs`) keeps only folders plus each folder's ten biggest files, so memory stays small
even on huge disks, and it is a pure state machine with unit tests.

## Eiga (映画)

Engine: `crates/eiga-core` (no UI code, 39 tests). Surface: `crates/smart-ui-egui/src/video.rs`.

* Keys follow mpv: `Space` pause, `←/→` ±5 s, `Shift+←/→` ±1 s, `↑/↓` ±1 min, `,` `.` frame step, `[` `]` speed ×1.1,
  `{` `}` half/double, `Backspace` reset speed, `9/0` volume, `m` mute, `v` subtitles, `z/x` audio delay,
  `Shift+z/x` subtitle delay, `f` fullscreen, `i` statistics, `s` screenshot, `Shift+L` A-B loop, `Ctrl+G` jump to time,
  `1-8` seek to 10-80 %. Press `H` in the player for the full sheet.
* Subtitles: SRT, WebVTT, ASS/SSA; sidecar files (`Movie.srt`, `Movie.en.srt`) and embedded text tracks; UTF-8,
  UTF-16 and Latin-1; per-track delay and size.
* Decoding: `ffmpeg`/`ffprobe` as child processes (video as raw RGBA over a pipe, audio as f32 PCM into rodio). This keeps
  the Rust side small and supports every format ffmpeg does. Audio and video share a wall clock, not a hardware
  clock, so expect a few tens of milliseconds of drift; use `z`/`x` to trim. There is no hardware-accelerated
  presentation yet (decoding can use `-hwaccel auto`).
* The web preview cannot spawn processes, so it plays a procedural test card with the real subtitle and control stack.

## Terminal

On the desktop the Terminal runs your `$SHELL` on a pseudo-terminal (`portable-pty`) and renders it through a vt100
screen, so vim, htop, ssh, less, tmux and your prompt work. `Ctrl+Shift+C/V` copy/paste, wheel and `Shift+PgUp` scroll
the 5000-line scrollback, drag to select, double-click selects a word. Right-click to switch to the Smart command-block
view. On the web there are no processes, so the virtual shell remains.

## Build

```sh
cargo run -p xtask --release -- assets      # icons, audio, wallpapers (needs the Solar icon JSON, fonts-inter)
cargo run --release -p smart-ui-egui        # desktop
cargo test -p smart-core -p eiga-core -p smart-ui-egui
# web preview
cargo build -p smart-ui-egui --lib --target wasm32-unknown-unknown --profile wasm
cargo run -p xtask --release -- bundle      # writes ./index.html with the wasm embedded
```

Desktop needs `ffmpeg` for video and the ALSA development headers to build audio. tree-sitter highlighting (Rust,
Python, JavaScript, JSON, TOML, Bash, CSS, HTML, C, Go, Markdown) is compiled into the desktop build; other text files
use a small built-in lexer, which is also what the web preview uses because the grammar crates do not yet build for
`wasm32-unknown-unknown` with the current toolchain.

## Layout

```
crates/smart-core     engine: file systems, search, jobs, devices, storage analysis (no UI)
crates/eiga-core      video engine: state machine, playlist, key map, subtitles, ffprobe, frame sources
crates/smart-ui-egui  the apps and the design system components
crates/xtask          asset and bundle tooling (Rust, no JS)
```
