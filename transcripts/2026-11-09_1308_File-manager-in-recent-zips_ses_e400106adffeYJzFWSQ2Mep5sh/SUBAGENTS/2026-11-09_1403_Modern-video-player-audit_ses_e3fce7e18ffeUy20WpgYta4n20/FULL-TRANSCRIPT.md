# Subagent transcript — Modern video player audit

- Session ID: `ses_e3fce7e18ffeUy20WpgYta4n20`
- Parent: `ses_e400106adffeYJzFWSQ2Mep5sh`
- Created: 2026-11-09 14:03:52
- Updated: 2026-11-09 14:08:29
- Messages: 29
- User/task msgs: 2

---

### TASK / USER [seq 4] 2026-11-09 14:03:52

You are a subagent spawned by another session.
You are the MODERN VIDEO PLAYER agent for smart-suite-official. Non-destructive to /tmp/opencode/canonical-video-audit/ then copy to /home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md. Source: /home/lufuno/Documents/rust-wasm-media-manager-*.zip eiga-core (player.rs 30125, source.rs 30036, subs.rs 14951) + smart-ui-egui video.rs 76k latest + studio AppImage context. Already have EIGA-SPECS.md at official/specs/EIGA-SPECS.md — this is the modern Rust-native successor (eiga evolved, Japanese name to propose, e.g. Eiga means movie, propose new timeless name like Kuro/EigaNeo? Pick one and justify). Must be thorough, very correct, performant, modern UX, reuse components, cargo test suite must pass, very robust. Research crates: mpv comparison, shotcut intuitive, demux (ffmpeg, gstreamer), dec (rust-decoder, symphonia), hwaccel, subs (srt, ass), HDR, audio.

Requirements from user (all P0):
- Feature rich as mpv, shotcut intuitive, compact nice UI, subs movable/shortened, track switching, resume, playlist, chapters, shader effects, PiP, casting, live, network, HDR10, spatial audio, robust filemanager preview must work (video preview on filemanager uses same engine).
- Appbar timeless and useful in many views/situations, feedback beautiful subtitles.
- Research Rust crates, clean useful, resume features users needed for years.
- About should be honest precise, not AI-promo litter.
- Dialogs: hardcoded heights create gaps/long cards — wrap close, space-between, compact. Devices/locations cards grid-based not long, timeless expensive compact, info precise. Top-right close button must be top (not right edge far), breaking layout fine if needed, must not shift left containers title/icons. Dialog open/close animations. Phone dialog huge vertical gap fix. Storage layout basic huge card disturbing — fix. Selectors horizontal center. Drag-to-tab drop nothing happens in file manager — but for player also fix dropped_files handling.
- Storage 92% reddish indicator, cleaning dialog as tab, graphs + obsidian view. This also applies to player settings storage usage.
- Text preview: remove windows controls inside dialog, make scrollable, highlight, copy, tree-sitter for code/md previews amazing.
- Open With dialog huge hardcoded height fix.
- Micro-interactions, visual liveness, spring animations.
- Audio playback in files (preview audio in filemanager).
- Research, fallback honestly if missing ffmpeg.
- Protocol: media.capnp + new domain-video, backends disposable, no hardcoded, opaque methods, doorbell subscriptions.
- Output VIDEO-SPECS.md with: name choice + rationale, architecture, per-file matrix, lie inventory, missing features + new ideas, perf budgets, UX micro-interactions, test plan (cargo test), acceptance. Non-destructive, copy to official.


### TOOL: read [seq 5] 2026-11-09 14:03:52 status=error

```json
{
  "path": "/tmp/opencode/canonical-video-audit"
}
```

### TOOL: shell [seq 16] 2026-11-09 14:03:54 status=completed

```json
{
  "command": "ls -R /tmp/opencode 2>/dev/null | head -n 300; echo ---; ls -la /home/lufuno/Documents/ 2>/dev/null | head -n 100; echo ---; ls -la /home/lufuno/smart/smart-suite-official/specs/ 2>/dev/null | head -n 100; echo ---; ls -la /home/lufuno/smart/ 2>/dev/null | head -n 100"
}
```

```
/tmp/opencode:
canonical-core-audit
canonical-eiga-audit
canonical-photos-audit
canonical-spec
canonical-ux-audit

/tmp/opencode/canonical-core-audit:
CORE-SPECS.md
extracted

/tmp/opencode/canonical-core-audit/extracted:
cross-good1
cross-orig
desk-bad2
develop
improve
media-latest
media-orig
prod-very-good
recreate-2
wasm-refine

/tmp/opencode/canonical-core-audit/extracted/cross-good1:
package-lock.json
package.json
smart-files
tsconfig.json
vite.config.ts

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files:
Cargo.toml
README.md
assets
crates
tools

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/assets:
tokens.toml

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/crates:
smart-core
smart-ui-egui

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/crates/smart-core:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/crates/smart-core/src:
apps.rs
archive.rs
audio.rs
backend.rs
demo.rs
devices.rs
exec.rs
lib.rs
library.rs
local.rs
memfs.rs
model.rs
ops.rs
registry.rs
search.rs
thumbs.rs
tools.rs
vfs.rs

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/crates/smart-ui-egui:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/crates/smart-ui-egui/src:
app.rs
chrome.rs
content.rs
icons.rs
icons_gen.rs
lib.rs
main.rs
modals.rs
overlays.rs
paint.rs
theme.rs

/tmp/opencode/canonical-core-audit/extracted/cross-good1/smart-files/tools:
build_web.sh

/tmp/opencode/canonical-core-audit/extracted/cross-orig:
package-lock.json
package.json
smart-files
tsconfig.json
vite.config.ts

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files:
Cargo.toml
README.md
assets
crates
tools

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/assets:
tokens.toml

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/crates:
smart-core
smart-ui-egui

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/crates/smart-core:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/crates/smart-core/src:
apps.rs
audio.rs
backend.rs
demo.rs
devices.rs
exec.rs
lib.rs
library.rs
local.rs
memfs.rs
model.rs
ops.rs
registry.rs
search.rs
thumbs.rs
tools.rs
vfs.rs

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/crates/smart-ui-egui:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/crates/smart-ui-egui/src:
app.rs
chrome.rs
content.rs
icons.rs
icons_gen.rs
lib.rs
main.rs
modals.rs
overlays.rs
paint.rs
theme.rs

/tmp/opencode/canonical-core-audit/extracted/cross-orig/smart-files/tools:
build_web.sh

/tmp/opencode/canonical-core-audit/extracted/desk-bad2:
package-lock.json
package.json
smart-desktop
src
tsconfig.json
vite.config.ts

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop:
Cargo.toml
README.md
apps
assets
crates
protocol
tools

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps:
assistant
capture
files
photos
settings
terminal

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps/assistant:
mod.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps/capture:
mod.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps/files:
app.rs
chrome.rs
content.rs
copyfx.rs
mod.rs
modals.rs
overlays.rs
picker.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps/photos:
mod.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps/settings:
mod.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/apps/terminal:
mod.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/assets:
tokens.toml

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates:
smart-core
smart-desktop
xtask

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-core:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-core/src:
apps.rs
archive.rs
audio.rs
backend.rs
demo.rs
devices.rs
exec.rs
lib.rs
library.rs
local.rs
memfs.rs
model.rs
ops.rs
registry.rs
search.rs
thumbs.rs
tools.rs
vfs.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-desktop:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-desktop/src:
apps
lib.rs
main.rs
shell
ui

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-desktop/src/apps:
mod.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-desktop/src/shell:
ai.rs
bar.rs
desktop.rs
dock.rs
frame.rs
mod.rs
scene.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/smart-desktop/src/ui:
appkit.rs
icons.rs
icons_gen.rs
mod.rs
paint.rs
sound.rs
theme.rs
widgets.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/xtask:
Cargo.toml
src

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/crates/xtask/src:
main.rs

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/protocol:
DESIGN_PRINCIPLES.md

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/smart-desktop/tools:
build_web.sh

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/src:
utils

/tmp/opencode/canonical-core-audit/extracted/desk-bad2/src/utils:
cn.ts

/tmp/opencode/canonical-core-audit/extracted/develop:
Cargo.toml
README.md
assets
crates
docs
package-lock.json
package.json
target
tools
vite.config.ts

/tmp/opencode/canonical-core-audit/extracted/develop/assets:
tokens.toml
---
total 12654988
drwxr-xr-x 16 lufuno lufuno       4096 Nov  6 19:16 .
drwx------ 54 lufuno lufuno      32768 Nov  9 12:47 ..
drwxr-xr-x  4 lufuno lufuno       4096 Aug 11 13:29 Cline
-rw-r--r--  1 lufuno lufuno 3526761099 Nov  6 16:38 Fightland.S01.720P.MoviesMod.zone.zip
drwxr-xr-x  6 lufuno lufuno       4096 Sep  4 18:37 Image-Line
-rw-r--r--  1 lufuno lufuno 3646947328 Oct 12 14:36 MX-25.3_KDE_x64.iso
-rw-r--r--  1 lufuno lufuno    1866980 Aug 20 17:15 Pasted image.png
-rw-r--r--  1 lufuno lufuno      72643 Sep 22 16:07 SMART_APPLICATION_SPEC.md
drwxr-xr-x  8 lufuno lufuno       4096 Sep 21 15:57 Saved
-rw-r--r--  1 lufuno lufuno      53638 Oct 12 17:27 Screenshot 2026-10-12 at 17-27-44 Design Editor.png
-rw-r--r--  1 lufuno lufuno    1364638 Nov  5 01:53 Screenshot 2026-11-05 at 01-53-19 Smart Desktop.png
-rw-r--r--  1 lufuno lufuno    1310642 Nov  5 02:00 Screenshot 2026-11-05 at 02-00-19 Smart Desktop.png
-rw-r--r--  1 lufuno lufuno    2746352 Nov  5 11:13 Screenshot 2026-11-05 at 11-13-07 Smart Desktop.png
drwxr-xr-x  2 lufuno lufuno       4096 Jul 31 23:40 Smart-Logo
-rwxr-xr-x  1 lufuno lufuno   11166912 Nov  6 19:01 SmartShell-x86_64.AppImage
-rwxr-----  1 lufuno lufuno 3189956818 Aug 30 11:45 [Windows X-Lite] Optimum 10 Pro v6 (FBConan).7z
-rw-r--r--  1 lufuno lufuno 2388006912 Oct 12 13:53 artix-plasma-dinit-20260813-x86_64.iso
drwxr-xr-x  6 lufuno lufuno       4096 Oct 12 13:59 botse
drwxr-xr-x  3 lufuno lufuno       4096 Sep 21 20:24 crextio+protocol
drwxr-xr-x  3 lufuno lufuno       4096 Sep 21 17:56 crextio-complete-repo-with-source-2026-09-21
-rw-r--r--  1 lufuno lufuno     195976 Sep 21 17:55 crextio-complete-repo-with-source-2026-09-21.zip
-rw-r--r--  1 lufuno lufuno     207496 Sep 21 20:23 crextio-protocol-first-2026-09-21--part-2.zip
-rw-r--r--  1 lufuno lufuno     208701 Nov  6 17:45 eiga.png
drwxr-xr-x  7 lufuno lufuno       4096 Sep 18 13:52 files
-rw-r--r--  1 lufuno lufuno       5333 Sep 17 23:32 files.zip
-rw-r--r--  1 lufuno lufuno     712369 Aug 10 17:35 full-capability-based-kernel-requirements.zip
-rw-r--r--  1 lufuno lufuno     429893 Aug 10 17:35 full-capability-based-kernel-requirementskimi.zip
drwxr-xr-x  2 lufuno lufuno       4096 Sep 14 23:08 kilo bak
drwxr-xr-x  3 lufuno lufuno       4096 Sep 17 23:37 liquid-glass-lab

```

### TOOL: shell [seq 23] 2026-11-09 14:03:55 status=completed

```json
{
  "command": "mkdir -p /tmp/opencode/canonical-video-audit && ls -la /tmp/opencode/canonical-video-audit/; ls /home/lufuno/Documents/rust-wasm-media-manager-latest/ 2>/dev/null | head -n 60; echo ===; ls /home/lufuno/smart/smart-suite-official/ 2>/dev/null | head -n 60"
}
```

```
total 0
drwxr-xr-x 2 lufuno lufuno  40 Nov  9 14:03 .
drwxr-xr-x 8 lufuno lufuno 160 Nov  9 14:03 ..
Cargo.lock
Cargo.toml
README.md
assets
crates
index.html
lint-stub.ts
package-lock.json
package.json
target
tools
tsconfig.json
vite.config.ts
web
===
audits
specs

```

### TOOL: shell [seq 30] 2026-11-09 14:03:57 status=completed

```json
{
  "command": "find /home/lufuno/Documents/rust-wasm-media-manager-latest/crates -maxdepth 3 -name \"*.rs\" -o -maxdepth 3 -name \"Cargo.toml\" | sort; echo ===; ls -R /home/lufuno/Documents/rust-wasm-media-manager-latest/crates 2>/dev/null | head -n 200"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/Cargo.toml
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/Cargo.toml
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/apps.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/archive.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/audio.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/devices.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/exec.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/lib.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/local.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/memfs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/ops.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/registry.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/search.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/tools.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/vfs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/copyfx.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/icons.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/icons_gen.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/main.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/paint.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/photos.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/pty.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/storage.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/terminal.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/termlive.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/theme.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/xtask/Cargo.toml
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/xtask/src/main.rs
===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates:
eiga-core
smart-core
smart-ui-egui
xtask

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core:
Cargo.toml
src

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src:
lib.rs
player.rs
source.rs
subs.rs

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core:
Cargo.toml
src

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src:
analyze.rs
apps.rs
archive.rs
audio.rs
backend.rs
demo.rs
devices.rs
exec.rs
lib.rs
library.rs
local.rs
memfs.rs
model.rs
ops.rs
registry.rs
search.rs
thumbs.rs
tools.rs
vfs.rs

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui:
Cargo.toml
src

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src:
app.rs
appkit.rs
chrome.rs
content.rs
copyfx.rs
highlight.rs
icons.rs
icons_gen.rs
lib.rs
main.rs
modals.rs
overlays.rs
paint.rs
photos.rs
pty.rs
shell.rs
storage.rs
terminal.rs
termlive.rs
theme.rs
video.rs

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/xtask:
Cargo.toml
src

/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/xtask/src:
main.rs

```

### TOOL: shell [seq 37] 2026-11-09 14:03:59 status=completed

```json
{
  "command": "wc -l /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/*.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/Cargo.toml /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml /home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.toml /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md\necho ===\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/Cargo.toml\necho ===SMARTUI===\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml\necho ===ROOT===\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.toml"
}
```

```
    92 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs
   820 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs
   731 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs
   401 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs
  1523 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
     9 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/Cargo.toml
    49 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml
    23 /home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.toml
   757 /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md
  4405 total
===
[package]
name = "eiga-core"
description = "Eiga (映画) engine: playback state machine, playlist, key map, subtitle parsers, ffprobe parsing and frame sources. No UI code."
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde_json = "1"
===SMARTUI===
[package]
name = "smart-ui-egui"
description = "egui frontend for Smart Files (desktop + WebAssembly)"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "smart-files"
path = "src/main.rs"

[features]
default = ["audio"]
audio = ["smart-core/audio"]

[dependencies]
smart-core = { path = "../smart-core", default-features = true }
eframe = { version = "0.37", default-features = false, features = ["glow", "default_fonts", "x11", "wayland"] }
egui = "0.37"
vt100 = "0.15"
image = { version = "0.25", default-features = false, features = ["png", "jpeg"] }
eiga-core = { version = "0.1.0", path = "../eiga-core" }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Window", "Document", "HtmlCanvasElement", "Element", "console", "Location"] }
console_error_panic_hook = "0.1"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
portable-pty = "0.9.0"
tree-sitter = "0.27.1"
tree-sitter-highlight = "0.27.1"
tree-sitter-rust = "0.24.2"
tree-sitter-python = "0.25.0"
tree-sitter-javascript = "0.25.0"
tree-sitter-json = "0.24.8"
tree-sitter-toml-ng = "0.7.0"
tree-sitter-bash = "0.25.1"
tree-sitter-css = "0.25.0"
tree-sitter-html = "0.23.2"
tree-sitter-c = "0.24.2"
tree-sitter-go = "0.25.0"
tree-sitter-md = "0.5.4"
rodio = { version = "0.23", default-features = false, features = ["playback"] }
rfd = "0.15"
===ROOT===
[workspace]
resolver = "2"
members = ["crates/smart-core", "crates/eiga-core", "crates/smart-ui-egui", "crates/xtask"]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

[profile.wasm]
inherits = "release"
opt-level = "s"
lto = true

[profile.dev]
opt-level = 1

```

### TOOL: read [seq 44] 2026-11-09 14:04:02 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs, lines 1-92
1: //! # eiga-core — the engine behind **Eiga (映画)**, the Smart suite's video player.
2: //!
3: //! * [`subs`]   SRT / WebVTT / ASS-SSA parsing, encoding detection, fast "what is on screen now" lookup.
4: //! * [`player`] pure playback state machine (seek, speed, A-B loop, chapters, frame-step), playlist and key map.
5: //! * [`source`] ffprobe parsing, a procedural test-card source (works everywhere, incl. WebAssembly) and — on native
6: //!   targets — an `ffmpeg` pipe source that decodes any format ffmpeg knows.
7: //!
8: //! Nothing in this crate draws, opens windows or reads clocks, so every behaviour is unit-tested.
9: pub mod player;
10: pub mod source;
11: pub mod subs;
12: 
13: pub use player::{Cmd, Event, LoopMode, Player, Playlist, Repeat};
14: pub use source::{Frame, MediaInfo};
15: pub use subs::{Cue, SubTrack};
16: 
17: /// `83.4` → `1:23`, `3723` → `1:02:03`.
18: pub fn fmt_time(secs: f64) -> String {
19:     if !secs.is_finite() || secs < 0.0 {
20:         return "0:00".into();
21:     }
22:     let s = secs as u64;
23:     let (h, m, s) = (s / 3600, (s / 60) % 60, s % 60);
24:     if h > 0 {
25:         format!("{h}:{m:02}:{s:02}")
26:     } else {
27:         format!("{m}:{s:02}")
28:     }
29: }
30: 
31: /// Like [`fmt_time`] with milliseconds, used for sync offsets and the stats overlay.
32: pub fn fmt_time_ms(secs: f64) -> String {
33:     if !secs.is_finite() || secs < 0.0 {
34:         return "0:00.000".into();
35:     }
36:     let ms = (secs * 1000.0).round() as u64;
37:     let (h, m, s, ms) = (ms / 3_600_000, (ms / 60_000) % 60, (ms / 1000) % 60, ms % 1000);
38:     if h > 0 {
39:         format!("{h}:{m:02}:{s:02}.{ms:03}")
40:     } else {
41:         format!("{m}:{s:02}.{ms:03}")
42:     }
43: }
44: 
45: /// Parses `90`, `1:30`, `1:02:03.5`. Used by "jump to time".
46: pub fn parse_time(s: &str) -> Option<f64> {
47:     let s = s.trim();
48:     if s.is_empty() {
49:         return None;
50:     }
51:     let mut total = 0.0;
52:     for part in s.split(':') {
53:         let v: f64 = part.trim().parse().ok()?;
54:         if !v.is_finite() || v < 0.0 {
55:             return None;
56:         }
57:         total = total * 60.0 + v;
58:     }
59:     if s.split(':').count() > 3 {
60:         return None;
61:     }
62:     Some(total)
63: }
64: 
65: #[cfg(test)]
66: mod tests {
67:     use super::*;
68: 
69:     #[test]
70:     fn time_formatting() {
71:         assert_eq!(fmt_time(0.0), "0:00");
72:         assert_eq!(fmt_time(83.9), "1:23");
73:         assert_eq!(fmt_time(3723.0), "1:02:03");
74:         assert_eq!(fmt_time(f64::NAN), "0:00");
75:         assert_eq!(fmt_time(-4.0), "0:00");
76:         assert_eq!(fmt_time_ms(3723.456), "1:02:03.456");
77:         assert_eq!(fmt_time_ms(5.5), "0:05.500");
78:     }
79: 
80:     #[test]
81:     fn time_parsing() {
82:         assert_eq!(parse_time("90"), Some(90.0));
83:         assert_eq!(parse_time("1:30"), Some(90.0));
84:         assert_eq!(parse_time("1:02:03.5"), Some(3723.5));
85:         assert_eq!(parse_time(" 0:05 "), Some(5.0));
86:         assert_eq!(parse_time(""), None);
87:         assert_eq!(parse_time("a:b"), None);
88:         assert_eq!(parse_time("-3"), None);
89:         assert_eq!(parse_time("1:2:3:4"), None);
90:         assert_eq!(parse_time("inf"), None);
91:     }
92: }
```

### TOOL: read [seq 44] 2026-11-09 14:04:02 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs, lines 1-820
1: //! Playback state machine, playlist and key map. Pure logic: feed it `tick(dt)`, read back [`Event`]s.
2: use crate::subs::SubTrack;
3: 
4: pub const SPEEDS: [f64; 10] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
5: pub const MAX_VOLUME: f32 = 130.0;
6: 
7: #[derive(Clone, Copy, PartialEq, Debug)]
8: pub enum LoopMode {
9:     Off,
10:     /// Repeat the current file.
11:     File,
12:     /// Repeat between A and B. `b` is `None` while the user has only set A.
13:     Ab { a: f64, b: Option<f64> },
14: }
15: 
16: /// What the engine/UI must do in response to a state change.
17: #[derive(Clone, Copy, PartialEq, Debug)]
18: pub enum Event {
19:     /// Restart decoding at this media time.
20:     Seek(f64),
21:     Ended,
22:     PauseChanged(bool),
23:     SpeedChanged(f64),
24:     VolumeChanged,
25:     /// Short message for the on-screen display ("Speed ×1.25").
26:     Osd,
27: }
28: 
29: #[derive(Clone, Copy, PartialEq, Debug)]
30: pub struct Chapter {
31:     pub start: f64,
32: }
33: 
34: #[derive(Clone, Debug)]
35: pub struct Player {
36:     pub duration: f64,
37:     pub position: f64,
38:     pub paused: bool,
39:     pub speed: f64,
40:     pub volume: f32,
41:     pub muted: bool,
42:     pub looping: LoopMode,
43:     pub eof: bool,
44:     pub fps: f64,
45:     pub audio_delay: f64,
46:     pub chapters: Vec<f64>,
47:     pub osd: Option<String>,
48: }
49: 
50: impl Default for Player {
51:     fn default() -> Self {
52:         Player { duration: 0.0, position: 0.0, paused: false, speed: 1.0, volume: 100.0, muted: false, looping: LoopMode::Off, eof: false, fps: 25.0, audio_delay: 0.0, chapters: vec![], osd: None }
53:     }
54: }
55: 
56: impl Player {
57:     pub fn new(duration: f64, fps: f64) -> Player {
58:         Player { duration: if duration.is_finite() { duration.max(0.0) } else { 0.0 }, fps: if fps.is_finite() && fps > 1.0 && fps < 480.0 { fps } else { 25.0 }, ..Player::default() }
59:     }
60: 
61:     fn say(&mut self, s: impl Into<String>) {
62:         self.osd = Some(s.into());
63:     }
64: 
65:     pub fn clamp_pos(&self, t: f64) -> f64 {
66:         if !t.is_finite() {
67:             return 0.0;
68:         }
69:         if self.duration > 0.0 {
70:             t.clamp(0.0, (self.duration - 0.001).max(0.0))
71:         } else {
72:             t.max(0.0)
73:         }
74:     }
75: 
76:     pub fn fraction(&self) -> f64 {
77:         if self.duration > 0.0 {
78:             (self.position / self.duration).clamp(0.0, 1.0)
79:         } else {
80:             0.0
81:         }
82:     }
83: 
84:     /// Advance the clock. Returns events (A-B wrap, loop, end of file).
85:     pub fn tick(&mut self, dt: f64) -> Vec<Event> {
86:         let mut ev = vec![];
87:         if self.paused || self.eof || !dt.is_finite() || dt <= 0.0 {
88:             return ev;
89:         }
90:         let dt = dt.min(0.25); // a stalled frame must not skip half a minute of video
91:         self.position += dt * self.speed;
92:         if let LoopMode::Ab { a, b: Some(b) } = self.looping {
93:             if self.position >= b {
94:                 self.position = a;
95:                 ev.push(Event::Seek(a));
96:                 return ev;
97:             }
98:         }
99:         if self.duration > 0.0 && self.position >= self.duration {
100:             if self.looping == LoopMode::File {
101:                 self.position = 0.0;
102:                 ev.push(Event::Seek(0.0));
103:             } else {
104:                 self.position = self.duration;
105:                 self.eof = true;
106:                 self.paused = true;
107:                 ev.push(Event::Ended);
108:             }
109:         }
110:         ev
111:     }
112: 
113:     pub fn seek_abs(&mut self, t: f64) -> Vec<Event> {
114:         self.position = self.clamp_pos(t);
115:         self.eof = false;
116:         vec![Event::Seek(self.position)]
117:     }
118:     pub fn seek_rel(&mut self, d: f64) -> Vec<Event> {
119:         let was_eof = self.eof;
120:         let base = if was_eof { self.duration } else { self.position };
121:         let ev = self.seek_abs(base + d);
122:         self.say(format!("{}{}s", if d >= 0.0 { "+" } else { "\u{2212}" }, d.abs()));
123:         ev
124:     }
125:     pub fn seek_frac(&mut self, f: f64) -> Vec<Event> {
126:         self.seek_abs(f.clamp(0.0, 1.0) * self.duration)
127:     }
128: 
129:     pub fn set_paused(&mut self, p: bool) -> Vec<Event> {
130:         if self.eof && !p {
131:             // pressing play at the end restarts, like every player
132:             self.position = 0.0;
133:             self.eof = false;
134:             self.paused = false;
135:             return vec![Event::Seek(0.0), Event::PauseChanged(false)];
136:         }
137:         if self.paused == p {
138:             return vec![];
139:         }
140:         self.paused = p;
141:         vec![Event::PauseChanged(p)]
142:     }
143:     pub fn toggle_pause(&mut self) -> Vec<Event> {
144:         let p = !self.paused;
145:         self.set_paused(p)
146:     }
147: 
148:     /// Step one frame (mpv `.` and `,`). Pauses first.
149:     pub fn frame_step(&mut self, dir: i32) -> Vec<Event> {
150:         let mut ev = vec![];
151:         if !self.paused {
152:             self.paused = true;
153:             ev.push(Event::PauseChanged(true));
154:         }
155:         let t = self.position + dir as f64 / self.fps;
156:         ev.extend(self.seek_abs(t));
157:         self.say(if dir >= 0 { "Next frame" } else { "Previous frame" });
158:         ev
159:     }
160: 
161:     pub fn set_speed(&mut self, s: f64) -> Vec<Event> {
162:         self.speed = if s.is_finite() { s.clamp(0.1, 8.0) } else { 1.0 };
163:         let msg = format!("Speed \u{d7}{:.2}", self.speed);
164:         self.say(msg);
165:         vec![Event::SpeedChanged(self.speed)]
166:     }
167:     /// mpv: `]` ×1.1, `[` ×(1/1.1)
168:     pub fn speed_scale(&mut self, f: f64) -> Vec<Event> {
169:         let s = (self.speed * f * 100.0).round() / 100.0;
170:         self.set_speed(s)
171:     }
172:     /// Move through the preset ladder ({/} in mpv halve/double; here ↑/↓ nudges along presets).
173:     pub fn speed_step(&mut self, up: bool) -> Vec<Event> {
174:         let cur = self.speed;
175:         let next = if up { SPEEDS.iter().cloned().find(|s| *s > cur + 1e-6) } else { SPEEDS.iter().rev().cloned().find(|s| *s < cur - 1e-6) };
176:         match next {
177:             Some(s) => self.set_speed(s),
178:             None => vec![],
179:         }
180:     }
181: 
182:     pub fn volume_step(&mut self, d: f32) -> Vec<Event> {
183:         self.volume = (self.volume + d).clamp(0.0, MAX_VOLUME);
184:         if d > 0.0 {
185:             self.muted = false;
186:         }
187:         let v = self.volume.round();
188:         self.say(format!("Volume {v}%"));
189:         vec![Event::VolumeChanged]
190:     }
191:     pub fn toggle_mute(&mut self) -> Vec<Event> {
192:         self.muted = !self.muted;
193:         self.say(if self.muted { "Muted".to_string() } else { "Unmuted".to_string() });
194:         vec![Event::VolumeChanged]
195:     }
196:     /// 0..1 gain for the output device (volume above 100 amplifies).
197:     pub fn gain(&self) -> f32 {
198:         if self.muted {
199:             0.0
200:         } else {
201:             self.volume / 100.0
202:         }
203:     }
204: 
205:     /// mpv `l`: first press sets A, second sets B (and starts looping), third clears.
206:     pub fn cycle_ab(&mut self) {
207:         self.looping = match self.looping {
208:             LoopMode::Ab { a, b: None } if self.position > a + 0.2 => LoopMode::Ab { a, b: Some(self.position) },
209:             LoopMode::Ab { .. } => LoopMode::Off,
210:             _ => LoopMode::Ab { a: self.position, b: None },
211:         };
212:         let m = mat
```

### TOOL: read [seq 55] 2026-11-09 14:04:03 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs, lines 1-731
1: //! Media metadata (ffprobe), frame sources (procedural test card everywhere; ffmpeg pipe on native).
2: use serde_json::Value;
3: 
4: #[derive(Clone, Debug, Default, PartialEq)]
5: pub struct VideoStream {
6:     pub index: usize,
7:     pub codec: String,
8:     pub width: u32,
9:     pub height: u32,
10:     pub fps: f64,
11:     pub bitrate: Option<u64>,
12:     pub pix_fmt: String,
13: }
14: 
15: #[derive(Clone, Debug, Default, PartialEq)]
16: pub struct AudioStream {
17:     pub index: usize,
18:     pub codec: String,
19:     pub channels: u32,
20:     pub sample_rate: u32,
21:     pub lang: String,
22:     pub title: String,
23:     pub default: bool,
24: }
25: 
26: #[derive(Clone, Debug, Default, PartialEq)]
27: pub struct SubStream {
28:     pub index: usize,
29:     pub codec: String,
30:     pub lang: String,
31:     pub title: String,
32:     pub default: bool,
33:     pub forced: bool,
34: }
35: 
36: impl SubStream {
37:     /// Text subtitles can be extracted and rendered; bitmap ones (PGS, VobSub) cannot.
38:     pub fn is_text(&self) -> bool {
39:         matches!(self.codec.as_str(), "subrip" | "srt" | "ass" | "ssa" | "webvtt" | "mov_text" | "text")
40:     }
41: }
42: 
43: #[derive(Clone, Debug, Default, PartialEq)]
44: pub struct MediaInfo {
45:     pub title: String,
46:     pub container: String,
47:     pub duration: f64,
48:     pub size: u64,
49:     pub bitrate: u64,
50:     pub video: Vec<VideoStream>,
51:     pub audio: Vec<AudioStream>,
52:     pub subs: Vec<SubStream>,
53:     pub chapters: Vec<(f64, String)>,
54: }
55: 
56: impl MediaInfo {
57:     pub fn primary_video(&self) -> Option<&VideoStream> {
58:         self.video.first()
59:     }
60:     pub fn size_wh(&self) -> (u32, u32) {
61:         self.primary_video().map(|v| (v.width, v.height)).unwrap_or((0, 0))
62:     }
63:     pub fn fps(&self) -> f64 {
64:         self.primary_video().map(|v| v.fps).filter(|f| *f > 1.0).unwrap_or(25.0)
65:     }
66: }
67: 
68: fn parse_rate(s: &str) -> f64 {
69:     match s.split_once('/') {
70:         Some((n, d)) => {
71:             let (n, d): (f64, f64) = (n.parse().unwrap_or(0.0), d.parse().unwrap_or(0.0));
72:             if d > 0.0 {
73:                 n / d
74:             } else {
75:                 0.0
76:             }
77:         }
78:         None => s.parse().unwrap_or(0.0),
79:     }
80: }
81: 
82: fn s(v: &Value, k: &str) -> String {
83:     v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string()
84: }
85: fn num(v: &Value, k: &str) -> f64 {
86:     match v.get(k) {
87:         Some(Value::String(t)) => t.parse().unwrap_or(0.0),
88:         Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
89:         _ => 0.0,
90:     }
91: }
92: 
93: /// Parses `ffprobe -print_format json -show_format -show_streams -show_chapters` output.
94: pub fn parse_ffprobe(json: &str) -> Result<MediaInfo, String> {
95:     let v: Value = serde_json::from_str(json).map_err(|e| format!("unreadable ffprobe output: {e}"))?;
96:     let mut m = MediaInfo::default();
97:     let f = v.get("format").cloned().unwrap_or(Value::Null);
98:     m.container = s(&f, "format_name").split(',').next().unwrap_or("").to_string();
99:     m.duration = num(&f, "duration");
100:     m.size = num(&f, "size") as u64;
101:     m.bitrate = num(&f, "bit_rate") as u64;
102:     m.title = f.get("tags").map(|t| s(t, "title")).unwrap_or_default();
103:     for st in v.get("streams").and_then(|x| x.as_array()).cloned().unwrap_or_default() {
104:         let index = num(&st, "index") as usize;
105:         let tags = st.get("tags").cloned().unwrap_or(Value::Null);
106:         let disp = st.get("disposition").cloned().unwrap_or(Value::Null);
107:         let flag = |k: &str| num(&disp, k) as i64 == 1;
108:         match s(&st, "codec_type").as_str() {
109:             "video" if !flag("attached_pic") => {
110:                 let mut fps = parse_rate(&s(&st, "avg_frame_rate"));
111:                 if fps <= 0.0 {
112:                     fps = parse_rate(&s(&st, "r_frame_rate"));
113:                 }
114:                 if m.duration <= 0.0 {
115:                     m.duration = num(&st, "duration");
116:                 }
117:                 m.video.push(VideoStream { index, codec: s(&st, "codec_name"), width: num(&st, "width") as u32, height: num(&st, "height") as u32, fps, bitrate: Some(num(&st, "bit_rate") as u64).filter(|b| *b > 0), pix_fmt: s(&st, "pix_fmt") });
118:             }
119:             "audio" => m.audio.push(AudioStream { index, codec: s(&st, "codec_name"), channels: num(&st, "channels") as u32, sample_rate: num(&st, "sample_rate") as u32, lang: s(&tags, "language"), title: s(&tags, "title"), default: flag("default") }),
120:             "subtitle" => m.subs.push(SubStream { index, codec: s(&st, "codec_name"), lang: s(&tags, "language"), title: s(&tags, "title"), default: flag("default"), forced: flag("forced") }),
121:             _ => {}
122:         }
123:     }
124:     for c in v.get("chapters").and_then(|x| x.as_array()).cloned().unwrap_or_default() {
125:         let t = c.get("tags").map(|t| s(t, "title")).unwrap_or_default();
126:         m.chapters.push((num(&c, "start_time"), t));
127:     }
128:     if !m.duration.is_finite() || m.duration < 0.0 {
129:         m.duration = 0.0;
130:     }
131:     if m.video.is_empty() && m.audio.is_empty() {
132:         return Err("no playable streams".into());
133:     }
134:     Ok(m)
135: }
136: 
137: // ─────────────────────────────────────────── frames ───────────────────────────────────────────
138: 
139: #[derive(Clone, Debug)]
140: pub struct Frame {
141:     pub w: usize,
142:     pub h: usize,
143:     pub rgba: Vec<u8>,
144:     pub pts: f64,
145: }
146: 
147: pub trait FrameSource: Send {
148:     fn info(&self) -> &MediaInfo;
149:     fn seek(&mut self, t: f64);
150:     /// The newest frame with `pts <= clock` that has not been returned yet.
151:     fn poll(&mut self, clock: f64) -> Option<Frame>;
152:     fn name(&self) -> &'static str;
153:     /// No more frames will ever arrive (end of file or decoder died).
154:     fn exhausted(&self) -> bool {
155:         false
156:     }
157: }
158: 
159: /// Even output dimensions that fit inside `max`, preserving aspect and never upscaling.
160: pub fn fit_size(sw: u32, sh: u32, max_w: u32, max_h: u32) -> (u32, u32) {
161:     if sw == 0 || sh == 0 {
162:         return (640, 360);
163:     }
164:     let k = (max_w as f64 / sw as f64).min(max_h as f64 / sh as f64).min(1.0);
165:     let ev = |x: f64| ((x.round() as u32).max(2)) & !1;
166:     (ev(sw as f64 * k), ev(sh as f64 * k))
167: }
168: 
169: // 5×7 digit font for the burnt-in timecode
170: const GLYPHS: [[u8; 7]; 12] = [
171:     [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
172:     [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
173:     [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
174:     [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
175:     [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
176:     [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
177:     [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
178:     [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
179:     [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
180:     [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
181:     [0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00],
182:     [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
183: ];
184: 
185: fn put(buf: &mut [u8], w: usize, x: usize, y: usize, c: [u8; 3]) {
186:     let i = (y * w + x) * 4;
187:     if i + 3 < buf.len() {
188:         buf[i] = c[0];
189:         buf[i + 1] = c[1];
190:         buf[i + 2] = c[2];
191:         buf[i + 3] = 255;
192:     }
193: }
194: 
195: fn draw_text(buf: &mut [u8], w: usize, h: usize, x0: usize, y0: usize, scale: usize, text: &str, c: [u8; 3]) {
196:     let mut x = x0;
197:     for ch in text.chars() {
198:         let g = match ch {
199:             '0'
```

### TOOL: read [seq 55] 2026-11-09 14:04:03 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs, lines 1-401
1: //! Subtitle parsing. Accepts SRT, WebVTT and ASS/SSA, tolerates BOMs, UTF-16, CRLF, missing indices and garbage.
2: 
3: #[derive(Clone, Debug, PartialEq)]
4: pub struct Cue {
5:     pub start: f64,
6:     pub end: f64,
7:     /// Plain text, lines separated by `\n`. Markup is stripped.
8:     pub text: String,
9:     pub italic: bool,
10:     /// ASS `\an7..9` (top row) positioning.
11:     pub top: bool,
12: }
13: 
14: #[derive(Clone, Debug, Default)]
15: pub struct SubTrack {
16:     pub name: String,
17:     pub lang: String,
18:     cues: Vec<Cue>,
19:     max_dur: f64,
20:     /// Seconds added to every cue (positive = later).
21:     pub delay: f64,
22: }
23: 
24: /// Decodes subtitle bytes: UTF-8 (BOM aware), UTF-16 LE/BE with BOM, otherwise Latin-1/Windows-1252-ish.
25: pub fn decode_bytes(b: &[u8]) -> String {
26:     if b.starts_with(&[0xFF, 0xFE]) || b.starts_with(&[0xFE, 0xFF]) {
27:         let le = b[0] == 0xFF;
28:         let units: Vec<u16> = b[2..].chunks_exact(2).map(|c| if le { u16::from_le_bytes([c[0], c[1]]) } else { u16::from_be_bytes([c[0], c[1]]) }).collect();
29:         return String::from_utf16_lossy(&units);
30:     }
31:     let b = b.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(b);
32:     match std::str::from_utf8(b) {
33:         Ok(s) => s.to_string(),
34:         Err(_) => b.iter().map(|&x| x as char).collect(),
35:     }
36: }
37: 
38: /// `00:01:02,500`, `1:02.5`, `0:00:01.00`, `62.5` → seconds.
39: pub fn parse_ts(s: &str) -> Option<f64> {
40:     let s = s.trim().replace(',', ".");
41:     if s.is_empty() || s.len() > 16 {
42:         return None;
43:     }
44:     let mut total = 0.0f64;
45:     let parts: Vec<&str> = s.split(':').collect();
46:     if parts.len() > 3 {
47:         return None;
48:     }
49:     for p in &parts {
50:         let v: f64 = p.trim().parse().ok()?;
51:         if !v.is_finite() || v < 0.0 {
52:             return None;
53:         }
54:         total = total * 60.0 + v;
55:     }
56:     Some(total)
57: }
58: 
59: fn decode_entities(s: &str) -> String {
60:     s.replace("&nbsp;", " ").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&")
61: }
62: 
63: /// Strips `<i>…</i>`, `<font …>`, `{\an8}` etc. Returns (text, italic, top).
64: fn clean(raw: &str) -> (String, bool, bool) {
65:     let mut italic = false;
66:     let mut top = false;
67:     let mut out = String::with_capacity(raw.len());
68:     let mut chars = raw.chars().peekable();
69:     while let Some(c) = chars.next() {
70:         match c {
71:             '<' => {
72:                 let mut tag = String::new();
73:                 let mut closed = false;
74:                 for t in chars.by_ref() {
75:                     if t == '>' {
76:                         closed = true;
77:                         break;
78:                     }
79:                     tag.push(t);
80:                     if tag.len() > 80 {
81:                         break;
82:                     }
83:                 }
84:                 if !closed {
85:                     out.push('<');
86:                     out.push_str(&tag);
87:                 } else if tag.eq_ignore_ascii_case("i") {
88:                     italic = true;
89:                 }
90:             }
91:             '{' => {
92:                 let mut tag = String::new();
93:                 let mut closed = false;
94:                 for t in chars.by_ref() {
95:                     if t == '}' {
96:                         closed = true;
97:                         break;
98:                     }
99:                     tag.push(t);
100:                     if tag.len() > 200 {
101:                         break;
102:                     }
103:                 }
104:                 if !closed {
105:                     out.push('{');
106:                     out.push_str(&tag);
107:                 } else {
108:                     if tag.contains("\\i1") {
109:                         italic = true;
110:                     }
111:                     for n in ["\\an7", "\\an8", "\\an9"] {
112:                         if tag.contains(n) {
113:                             top = true;
114:                         }
115:                     }
116:                 }
117:             }
118:             '\\' if matches!(chars.peek(), Some('N') | Some('n')) => {
119:                 chars.next();
120:                 out.push('\n');
121:             }
122:             _ => out.push(c),
123:         }
124:     }
125:     let text = decode_entities(&out).lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n");
126:     (text, italic, top)
127: }
128: 
129: fn parse_arrow_block(lines: &[&str]) -> Option<Cue> {
130:     let ai = lines.iter().position(|l| l.contains("-->"))?;
131:     let (a, b) = lines[ai].split_once("-->")?;
132:     let start = parse_ts(a)?;
133:     // VTT cue settings follow the end time ("align:start position:10%")
134:     let b = b.trim().split_whitespace().next()?;
135:     let end = parse_ts(b)?;
136:     let (text, italic, top) = clean(&lines[ai + 1..].join("\n"));
137:     if text.is_empty() || end <= start {
138:         return None;
139:     }
140:     Some(Cue { start, end, text, italic, top })
141: }
142: 
143: fn parse_srt_vtt(src: &str) -> Vec<Cue> {
144:     let norm = src.replace("\r\n", "\n").replace('\r', "\n");
145:     let mut cues = vec![];
146:     for block in norm.split("\n\n") {
147:         let lines: Vec<&str> = block.lines().map(|l| l.trim_end()).filter(|l| !l.trim().is_empty()).collect();
148:         if lines.is_empty() || lines[0].starts_with("WEBVTT") || lines[0].starts_with("NOTE") || lines[0].starts_with("STYLE") || lines[0].starts_with("REGION") {
149:             continue;
150:         }
151:         if let Some(c) = parse_arrow_block(&lines) {
152:             cues.push(c);
153:         }
154:     }
155:     cues
156: }
157: 
158: fn parse_ass(src: &str) -> Vec<Cue> {
159:     let mut cues = vec![];
160:     let (mut in_events, mut si, mut ei, mut ti, mut nfields) = (false, 1usize, 2usize, 9usize, 10usize);
161:     for raw in src.replace("\r\n", "\n").lines() {
162:         let line = raw.trim();
163:         if line.starts_with('[') {
164:             in_events = line.eq_ignore_ascii_case("[events]");
165:             continue;
166:         }
167:         if !in_events {
168:             continue;
169:         }
170:         if let Some(f) = line.strip_prefix("Format:") {
171:             let cols: Vec<String> = f.split(',').map(|c| c.trim().to_ascii_lowercase()).collect();
172:             nfields = cols.len();
173:             si = cols.iter().position(|c| c == "start").unwrap_or(1);
174:             ei = cols.iter().position(|c| c == "end").unwrap_or(2);
175:             ti = cols.iter().position(|c| c == "text").unwrap_or(nfields.saturating_sub(1));
176:         } else if let Some(d) = line.strip_prefix("Dialogue:") {
177:             let f: Vec<&str> = d.trim_start().splitn(nfields.max(1), ',').collect();
178:             if f.len() <= ti.max(si).max(ei) {
179:                 continue;
180:             }
181:             let (Some(start), Some(end)) = (parse_ts(f[si]), parse_ts(f[ei])) else { continue };
182:             let (text, italic, top) = clean(f[ti]);
183:             if !text.is_empty() && end > start {
184:                 cues.push(Cue { start, end, text, italic, top });
185:             }
186:         }
187:     }
188:     cues
189: }
190: 
191: /// Parses any supported format by sniffing the content. Never panics; unparseable input yields no cues.
192: pub fn parse(src: &str) -> Vec<Cue> {
193:     let head = src.trim_start_matches('\u{feff}').trim_start();
194:     let mut cues = if head.starts_with("[Script Info]") || head.contains("\n[Events]") || head.starts_with("[V4") {
195:         parse_ass(src)
196:     } else {
197
```

### TOOL: shell [seq 66] 2026-11-09 14:04:05 status=completed

```json
{
  "command": "wc -l /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs; grep -n \"struct \\|fn \\|pub \" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 250"
}
```

```
1523 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
19:pub struct AudioOut {
27:    fn new() -> Result<Self, String> {
33:    fn restart(&mut self, path: &str, start: f64, track: usize, speed: f64, gain: f32, paused: bool) {
42:    fn pump(&mut self) {
55:pub struct Eiga {
56:    pub player: Player,
57:    pub playlist: Playlist,
58:    pub src: Option<Box<dyn FrameSource>>,
59:    pub info: Option<MediaInfo>,
60:    pub tex: Option<egui::TextureHandle>,
61:    pub frame_wh: (usize, usize),
62:    pub subs: Vec<SubTrack>,
63:    pub sub_i: Option<usize>,
64:    pub sub_scale: f32,
65:    pub audio_i: usize,
66:    pub title: String,
67:    pub path: Option<String>,
68:    pub real: Option<String>,
69:    pub bindings: Vec<Binding>,
70:    pub last_move: f64,
71:    pub last_tick: f64,
72:    pub theater: bool,
73:    pub show_list: bool,
74:    pub show_stats: bool,
75:    pub show_help: bool,
76:    pub aspect: u8,
77:    pub zoom: f32,
78:    pub rotate: u8,
79:    pub osd: Option<(String, f64)>,
80:    pub error: Option<String>,
81:    pub jump: Option<String>,
82:    pub scrub: Option<f64>,
83:    pub frames_shown: u64,
84:    pub fps_meter: (f64, u64, f32),
85:    pub locs: Vec<Location>,
86:    pub cfg_hide_secs: f32,
87:    pub cfg_sub_size: f32,
88:    pub cfg_def_speed: f64,
89:    pub cfg_double_click: bool,
90:    pub cfg_loop_default: bool,
92:    pub picker_rx: Option<std::sync::mpsc::Receiver<Vec<std::path::PathBuf>>>,
93:    pub pending_evs: Vec<PEvent>,
94:    pub pending_cmd: Option<Cmd>,
95:    pub last_rgba: Option<(usize, usize, Vec<u8>)>,
96:    pub fs_dirty: bool,
98:    pub audio: Option<AudioOut>,
102:    fn default() -> Self {
107:fn key_name(k: Key, shift: bool) -> Option<&'static str> {
173:    pub fn eiga_cmd(&mut self, c: &str) {
266:    fn eiga_siblings(&self, l: &Location) -> Vec<Location> {
277:    pub fn eiga_open(&mut self, l: Location) {
286:    fn eiga_play_index(&mut self, i: usize) {
293:    fn eiga_load(&mut self, l: Location) {
378:    fn eiga_restart_audio(&mut self) {
398:    pub fn eiga_cleanup(&mut self) {
411:    fn eiga_events(&mut self, evs: Vec<PEvent>) {
456:    fn eiga_run(&mut self, cmd: Cmd) {
563:    fn eiga_drop(&mut self, files: Vec<egui::DroppedFileHandle>) {
604:    fn eiga_pick(&mut self) {
625:    fn eiga_poll_picker(&mut self) {
669:    fn eiga_screenshot(&mut self) {
698:    fn eiga_keys(&mut self, ctx: &egui::Context) {
719:    fn eiga_speed_menu(&self) -> Vec<MI> {
725:    fn eiga_sub_menu(&self) -> Vec<MI> {
740:    fn eiga_audio_menu(&self) -> Vec<MI> {
756:    fn eiga_more_menu(&self) -> Vec<MI> {
772:    pub fn eiga_settings(&mut self, ui: &mut Ui, win: Rect) {
836:    pub fn draw_eiga(&mut self, ui: &mut Ui, fr: &Frame) {
1092:    fn eiga_default_video(&self) -> Option<Location> {
1110:    fn eiga_load_first(&mut self, l: Location) {
1119:    fn eiga_toolbar(&mut self, ui: &Ui, fr: &Frame, e: &Eiga) {
1183:    fn eiga_overlay_bar(&mut self, ui: &mut Ui, area: Rect, e: &mut Eiga, t: f32) {
1228:    fn eiga_sidebar(&mut self, ui: &mut Ui, fr: &Frame, e: &Eiga) {
1299:    fn eiga_controls(&mut self, ui: &Ui, area: Rect, e: &mut Eiga, t: f32) {
1432:    fn eiga_stats(&self, p: &egui::Painter, area: Rect, e: &Eiga) {
1453:    fn eiga_help(&mut self, ui: &Ui, area: Rect, e: &mut Eiga) {

```

### TOOL: read [seq 73] 2026-11-09 14:04:08 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs",
  "limit": 300
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs, lines 1-300
1: //! **Eiga (映画)** — the suite's video player. Engine: `eiga-core` (state machine, subtitles, key map, ffmpeg pipes).
2: //! This file is only the surface: it draws frames, the controls, subtitles, menus and routes keys.
3: use crate::app::*;
4: use crate::appkit::Frame;
5: use crate::chrome::*;
6: use crate::icons::Ic;
7: use crate::overlays::{mi, MI};
8: use crate::paint::*;
9: use crate::theme::*;
10: use eiga_core::player::{default_bindings, lookup, Binding, Cmd, Event as PEvent, LoopMode, Repeat};
11: use eiga_core::source::{FrameSource, TestCard};
12: use eiga_core::{fmt_time, parse_time, MediaInfo, Player, Playlist, SubTrack};
13: use egui::{pos2, vec2, Align2, Color32, Key, Rect, Sense, Ui};
14: use smart_core::*;
15: 
16: const VIDEO_EXTS: [&str; 8] = ["mp4", "mkv", "mov", "webm", "avi", "m4v", "mpg", "ts"];
17: 
18: #[cfg(not(target_arch = "wasm32"))]
19: pub struct AudioOut {
20:     dev: rodio::MixerDeviceSink,
21:     player: rodio::Player,
22:     pipe: Option<eiga_core::source::ffmpeg::AudioPipe>,
23: }
24: 
25: #[cfg(not(target_arch = "wasm32"))]
26: impl AudioOut {
27:     fn new() -> Result<Self, String> {
28:         let mut dev = rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| format!("No audio output: {e}"))?;
29:         dev.log_on_drop(false);
30:         let player = rodio::Player::connect_new(dev.mixer());
31:         Ok(AudioOut { dev, player, pipe: None })
32:     }
33:     fn restart(&mut self, path: &str, start: f64, track: usize, speed: f64, gain: f32, paused: bool) {
34:         self.player.stop();
35:         self.player = rodio::Player::connect_new(self.dev.mixer());
36:         self.player.set_volume(gain);
37:         if paused {
38:             self.player.pause();
39:         }
40:         self.pipe = eiga_core::source::ffmpeg::AudioPipe::open(path, start, track, speed).ok();
41:     }
42:     fn pump(&mut self) {
43:         let (Some(ch), Some(sr)) = (std::num::NonZeroU16::new(2), std::num::NonZeroU32::new(48_000)) else { return };
44:         if let Some(p) = &self.pipe {
45:             while self.player.len() < 3 {
46:                 match p.try_chunk() {
47:                     Some(c) => self.player.append(rodio::buffer::SamplesBuffer::new(ch, sr, c)),
48:                     None => break,
49:                 }
50:             }
51:         }
52:     }
53: }
54: 
55: pub struct Eiga {
56:     pub player: Player,
57:     pub playlist: Playlist,
58:     pub src: Option<Box<dyn FrameSource>>,
59:     pub info: Option<MediaInfo>,
60:     pub tex: Option<egui::TextureHandle>,
61:     pub frame_wh: (usize, usize),
62:     pub subs: Vec<SubTrack>,
63:     pub sub_i: Option<usize>,
64:     pub sub_scale: f32,
65:     pub audio_i: usize,
66:     pub title: String,
67:     pub path: Option<String>,
68:     pub real: Option<String>,
69:     pub bindings: Vec<Binding>,
70:     pub last_move: f64,
71:     pub last_tick: f64,
72:     pub theater: bool,
73:     pub show_list: bool,
74:     pub show_stats: bool,
75:     pub show_help: bool,
76:     pub aspect: u8,
77:     pub zoom: f32,
78:     pub rotate: u8,
79:     pub osd: Option<(String, f64)>,
80:     pub error: Option<String>,
81:     pub jump: Option<String>,
82:     pub scrub: Option<f64>,
83:     pub frames_shown: u64,
84:     pub fps_meter: (f64, u64, f32),
85:     pub locs: Vec<Location>,
86:     pub cfg_hide_secs: f32,
87:     pub cfg_sub_size: f32,
88:     pub cfg_def_speed: f64,
89:     pub cfg_double_click: bool,
90:     pub cfg_loop_default: bool,
91:     #[cfg(not(target_arch = "wasm32"))]
92:     pub picker_rx: Option<std::sync::mpsc::Receiver<Vec<std::path::PathBuf>>>,
93:     pub pending_evs: Vec<PEvent>,
94:     pub pending_cmd: Option<Cmd>,
95:     pub last_rgba: Option<(usize, usize, Vec<u8>)>,
96:     pub fs_dirty: bool,
97:     #[cfg(not(target_arch = "wasm32"))]
98:     pub audio: Option<AudioOut>,
99: }
100: 
101: impl Default for Eiga {
102:     fn default() -> Self {
103:         Eiga { player: Player::default(), playlist: Playlist::default(), src: None, info: None, tex: None, frame_wh: (0, 0), subs: vec![], sub_i: None, sub_scale: 1.0, audio_i: 0, title: String::new(), path: None, real: None, bindings: default_bindings(), last_move: 0.0, last_tick: 0.0, theater: false, show_list: true, show_stats: false, show_help: false, aspect: 0, zoom: 1.0, rotate: 0, osd: None, error: None, jump: None, scrub: None, frames_shown: 0, fps_meter: (0.0, 0, 0.0), locs: vec![], cfg_hide_secs: 2.6, cfg_sub_size: 1.0, cfg_def_speed: 1.0, cfg_double_click: true, cfg_loop_default: false, #[cfg(not(target_arch = "wasm32"))] picker_rx: None, pending_evs: vec![], pending_cmd: None, last_rgba: None, fs_dirty: false, #[cfg(not(target_arch = "wasm32"))] audio: None }
104:     }
105: }
106: 
107: fn key_name(k: Key, shift: bool) -> Option<&'static str> {
108:     Some(match k {
109:         Key::Space => "space",
110:         Key::ArrowLeft => "left",
111:         Key::ArrowRight => "right",
112:         Key::ArrowUp => "up",
113:         Key::ArrowDown => "down",
114:         Key::Home => "home",
115:         Key::PageUp => "pageup",
116:         Key::PageDown => "pagedown",
117:         Key::Enter => "enter",
118:         Key::Escape => "escape",
119:         Key::Backspace => "backspace",
120:         Key::Tab => "tab",
121:         Key::Comma => if shift { "comma" } else { "," },
122:         Key::Period => if shift { "period" } else { "." },
123:         Key::OpenBracket => "[",
124:         Key::CloseBracket => "]",
125:         Key::OpenCurlyBracket => "{",
126:         Key::CloseCurlyBracket => "}",
127:         Key::Minus => "-",
128:         Key::Equals | Key::Plus => "=",
129:         Key::Slash => "/",
130:         Key::Questionmark => "?",
131:         Key::Num0 => "0",
132:         Key::Num1 => "1",
133:         Key::Num2 => "2",
134:         Key::Num3 => "3",
135:         Key::Num4 => "4",
136:         Key::Num5 => "5",
137:         Key::Num6 => "6",
138:         Key::Num7 => "7",
139:         Key::Num8 => "8",
140:         Key::Num9 => "9",
141:         Key::A => "a",
142:         Key::B => "b",
143:         Key::C => "c",
144:         Key::D => "d",
145:         Key::E => "e",
146:         Key::F => "f",
147:         Key::G => "g",
148:         Key::H => "h",
149:         Key::I => "i",
150:         Key::J => "j",
151:         Key::K => "k",
152:         Key::L => "l",
153:         Key::M => "m",
154:         Key::N => "n",
155:         Key::O => "o",
156:         Key::P => "p",
157:         Key::Q => "q",
158:         Key::R => "r",
159:         Key::S => "s",
160:         Key::T => "t",
161:         Key::U => "u",
162:         Key::V => "v",
163:         Key::W => "w",
164:         Key::X => "x",
165:         Key::Y => "y",
166:         Key::Z => "z",
167:         _ => return None,
168:     })
169: }
170: 
171: impl SmartApp {
172:     // ───────────── opening ─────────────
173:     pub fn eiga_cmd(&mut self, c: &str) {
174:         let (name, arg) = c.split_once(':').unwrap_or((c, ""));
175:         match name {
176:             "open" => {
177:                 if let Some((fs, path)) = arg.split_once('|') {
178:                     self.eiga_open(Location::new(fs, path));
179:                 }
180:             }
181:             "play_idx" => {
182:                 if let Ok(i) = arg.parse::<usize>() {
183:                     self.eiga_play_index(i);
184:                 }
185:             }
186:             "sub" => {
187:                 let e = &mut self.shell.eiga;
188:                 e.sub_i = if arg == "off" { None } else { arg.parse().ok().filter(|i| *i < e.subs.len()) };
189:                 let msg = match e.sub_i.and_then(|i| e.subs.get(i)) {
190:                     Some(t) => format!("Subtitles: {}", t.name),
191:                     None => "Subtitles off".to_string(),
192:                 };
193:               
```

### TOOL: read [seq 80] 2026-11-09 14:04:10 status=completed

```json
{
  "limit": 500,
  "offset": 301,
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs, lines 301-800
301:         let mut error = None;
302:         #[cfg(not(target_arch = "wasm32"))]
303:         if let Some(rp) = &real {
304:             if eiga_core::source::ffmpeg::available() {
305:                 match eiga_core::source::ffmpeg::probe(rp) {
306:                     Ok(i) => match eiga_core::source::ffmpeg::FfmpegSource::open(rp, i.clone(), 1280, 720) {
307:                         Ok(s) => {
308:                             src = Some(Box::new(s));
309:                             info = Some(i);
310:                         }
311:                         Err(e) => error = Some(e),
312:                     },
313:                     Err(e) => error = Some(e),
314:                 }
315:             } else {
316:                 error = Some("ffmpeg was not found. Install it (apt install ffmpeg) to play real files. Showing the test card.".to_string());
317:             }
318:         }
319:         let name = l.name();
320:         let (src, info) = match (src, info) {
321:             (Some(s), Some(i)) => (s, i),
322:             _ => {
323:                 let dur = entry.as_ref().map(|e| (e.size as f64 / 4_000_000.0).clamp(40.0, 600.0)).unwrap_or(120.0);
324:                 if error.is_none() {
325:                     error = Some("Web preview: decoding runs natively (ffmpeg). This is the built-in test card with real subtitles.".into());
326:                 }
327:                 let s = TestCard::new(&name, dur);
328:                 let i = s.info().clone();
329:                 (Box::new(s) as Box<dyn FrameSource>, i)
330:             }
331:         };
332:         // subtitles: sidecar files first, then embedded text tracks (extracted lazily)
333:         let mut subs: Vec<SubTrack> = vec![];
334:         if let (Some(parent), Some(v)) = (l.parent(), vfs.as_ref()) {
335:             let names: Vec<String> = v.list(&parent.path).unwrap_or_default().into_iter().map(|e| e.name).collect();
336:             for n in eiga_core::subs::find_sidecars(&name, &names) {
337:                 if let Ok(bytes) = v.read(&format!("{}/{}", parent.path.trim_end_matches('/'), n), 8 * 1024 * 1024) {
338:                     let t = SubTrack::from_bytes(&n, &bytes);
339:                     if !t.is_empty() {
340:                         subs.push(t);
341:                     }
342:                 }
343:             }
344:         }
345:         #[cfg(not(target_arch = "wasm32"))]
346:         if let Some(rp) = &real {
347:             for (n, s) in info.subs.iter().enumerate().filter(|(_, s)| s.is_text()) {
348:                 if let Ok(txt) = eiga_core::source::ffmpeg::extract_sub(rp, n) {
349:                     let mut t = SubTrack::new(format!("{} (embedded){}", if s.lang.is_empty() { "Track".to_string() } else { s.lang.clone() }, if s.forced { " forced" } else { "" }), s.lang.clone(), eiga_core::subs::parse(&txt));
350:                     if !t.is_empty() {
351:                         t.delay = 0.0;
352:                         subs.push(t);
353:                     }
354:                 }
355:             }
356:         }
357:         let e = &mut self.shell.eiga;
358:         e.player = Player::new(info.duration, info.fps());
359:         if e.cfg_loop_default { e.player.looping = LoopMode::File; }
360:         let _ = e.player.set_speed(e.cfg_def_speed);
361:         e.player.chapters = info.chapters.iter().map(|c| c.0).collect();
362:         e.sub_i = if subs.is_empty() { None } else { Some(0) };
363:         e.subs = subs;
364:         e.title = if info.title.is_empty() { name } else { info.title.clone() };
365:         e.path = Some(l.path.clone());
366:         e.real = real.clone();
367:         e.info = Some(info);
368:         e.src = Some(src);
369:         e.error = error;
370:         e.audio_i = 0;
371:         e.last_tick = now;
372:         e.last_move = now;
373:         e.frames_shown = 0;
374:         e.osd = Some((format!("Playing {}", e.title), now));
375:         self.eiga_restart_audio();
376:     }
377: 
378:     fn eiga_restart_audio(&mut self) {
379:         #[cfg(not(target_arch = "wasm32"))]
380:         {
381:             let e = &mut self.shell.eiga;
382:             let Some(rp) = e.real.clone() else { return };
383:             if e.info.as_ref().map(|i| i.audio.is_empty()).unwrap_or(true) {
384:                 e.audio = None;
385:                 return;
386:             }
387:             if e.audio.is_none() {
388:                 e.audio = AudioOut::new().ok();
389:             }
390:             let (pos, tr, sp, g, paused) = (e.player.position, e.audio_i, e.player.speed, e.player.gain(), e.player.paused);
391:             if let Some(a) = e.audio.as_mut() {
392:                 a.restart(&rp, pos, tr, sp, g, paused);
393:             }
394:         }
395:     }
396: 
397:     /// Frees decoder, textures and audio when the window closes so memory returns to the system.
398:     pub fn eiga_cleanup(&mut self) {
399:         let e = &mut self.shell.eiga;
400:         e.src = None;
401:         e.tex = None;
402:         e.last_rgba = None;
403:         e.osd = None;
404:         #[cfg(not(target_arch = "wasm32"))]
405:         {
406:             e.audio = None;
407:         }
408:         e.frames_shown = 0;
409:     }
410: 
411:     fn eiga_events(&mut self, evs: Vec<PEvent>) {
412:         for ev in evs {
413:             match ev {
414:                 PEvent::Seek(t) => {
415:                     if let Some(s) = self.shell.eiga.src.as_mut() {
416:                         s.seek(t);
417:                     }
418:                     self.eiga_restart_audio();
419:                 }
420:                 PEvent::PauseChanged(p) => {
421:                     #[cfg(not(target_arch = "wasm32"))]
422:                     if let Some(a) = self.shell.eiga.audio.as_mut() {
423:                         if p {
424:                             a.player.pause()
425:                         } else {
426:                             a.player.play()
427:                         }
428:                     }
429:                     self.shell.eiga.osd = Some((if p { "Paused" } else { "Playing" }.to_string(), self.time));
430:                 }
431:                 PEvent::SpeedChanged(_) => self.eiga_restart_audio(),
432:                 PEvent::VolumeChanged => {
433:                     #[cfg(not(target_arch = "wasm32"))]
434:                     {
435:                         let g = self.shell.eiga.player.gain();
436:                         if let Some(a) = self.shell.eiga.audio.as_mut() {
437:                             a.player.set_volume(g);
438:                         }
439:                     }
440:                 }
441:                 PEvent::Ended => {
442:                     let next = self.shell.eiga.playlist.next(true);
443:                     if let Some(i) = next {
444:                         self.eiga_play_index(i);
445:                     }
446:                 }
447:                 PEvent::Osd => {}
448:             }
449:         }
450:         let t = self.time;
451:         if let Some(m) = self.shell.eiga.player.osd.take() {
452:             self.shell.eiga.osd = Some((m, t));
453:         }
454:     }
455: 
456:     fn eiga_run(&mut self, cmd: Cmd) {
457:         let (evs, rest) = self.shell.eiga.player.apply(cmd);
458:         self.eiga_events(evs);
459:         let Some(cmd) = rest else { return };
460:         let t = self.time;
461:         match cmd {
462:             Cmd::ToggleFullscreen => {
463:                 let e = &mut self.shell.eiga;
464:                 e.theater = !e.theater;
465:                 e.fs_dirty = true;
466:             }
467:             Cmd::Close => {
468:                 if self.shell.eiga.theater {
469:                     self.eiga_run(Cmd::ToggleFullscreen);
470:                 } else {
471:                     self.shell.video_w.o
```

### TOOL: read [seq 87] 2026-11-09 14:04:16 status=completed

```json
{
  "limit": 750,
  "offset": 801,
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs, lines 801-1523
801:                 (0, 0, crate::appkit::SVal::F(v)) => e.cfg_def_speed = (v as f64).clamp(0.25, 4.0),
802:                 (0, 1, crate::appkit::SVal::Bool(b)) => e.cfg_loop_default = b,
803:                 (0, 2, crate::appkit::SVal::Bool(b)) => e.cfg_double_click = b,
804:                 (1, 0, crate::appkit::SVal::F(v)) => e.cfg_sub_size = (v / 100.0).clamp(0.5, 2.5),
805:                 (1, 1, _) => {
806:                     e.cfg_sub_size = 1.0;
807:                     self.toast("Subtitle size reset", true);
808:                 }
809:                 (2, 0, crate::appkit::SVal::F(v)) => {
810:                     let d = v - e.player.volume;
811:                     let evs = if d.abs() > 0.5 { e.player.volume_step(d) } else { vec![] };
812:                     drop(e);
813:                     self.eiga_events(evs);
814:                 }
815:                 (2, 1, crate::appkit::SVal::Bool(b)) => {
816:                     let evs = if e.player.muted != b { e.player.toggle_mute() } else { vec![] };
817:                     drop(e);
818:                     self.eiga_events(evs);
819:                 }
820:                 (2, 2, crate::appkit::SVal::Idx(i)) => e.aspect = i as u8,
821:                 (3, 0, crate::appkit::SVal::F(v)) => e.cfg_hide_secs = v.clamp(1.0, 8.0),
822:                 (3, 1, _) => {
823:                     e.cfg_def_speed = 1.0;
824:                     e.cfg_sub_size = 1.0;
825:                     e.cfg_hide_secs = 2.6;
826:                     e.cfg_double_click = true;
827:                     e.cfg_loop_default = false;
828:                     self.toast("Eiga settings reset", true);
829:                 }
830:                 _ => {}
831:             }
832:         }
833:     }
834: 
835:     // ───────────── drawing ─────────────
836:     pub fn draw_eiga(&mut self, ui: &mut Ui, fr: &Frame) {
837:         let th = self.th.clone();
838:         let ctx = ui.ctx().clone();
839:         let mut e = std::mem::take(&mut self.shell.eiga);
840:         if e.src.is_none() && e.path.is_none() {
841:             // nothing loaded yet: open the first demo/real video we can find so the window is never empty
842:             self.shell.eiga = e;
843:             let start = self.eiga_default_video();
844:             if let Some(l) = start {
845:                 self.eiga_load_first(l);
846:             }
847:             e = std::mem::take(&mut self.shell.eiga);
848:         }
849:         self.eiga_poll_picker();
850:         let dropped: Vec<egui::DroppedFileHandle> = ctx.input(|i| i.raw.dropped_files.clone());
851:         if !dropped.is_empty() {
852:             self.eiga_drop(dropped);
853:         }
854:         // clock
855:         let now = self.time;
856:         let dt = (now - e.last_tick).clamp(0.0, 0.25);
857:         e.last_tick = now;
858:         self.shell.eiga = e;
859:         let evs = self.shell.eiga.player.tick(dt);
860:         self.eiga_events(evs);
861:         if fr.active && self.shell.settings.is_none() && !self.palette.open && self.shell.appmenu.is_none() {
862:             self.eiga_keys(&ctx);
863:         }
864:         let mut e = std::mem::take(&mut self.shell.eiga);
865:         #[cfg(not(target_arch = "wasm32"))]
866:         if let Some(a) = e.audio.as_mut() {
867:             a.pump();
868:         }
869:         // pull a frame
870:         let clock = e.player.position;
871:         if let Some(src) = e.src.as_mut() {
872:             if let Some(f) = src.poll(clock) {
873:                 let img = egui::ColorImage::from_rgba_unmultiplied([f.w, f.h], &f.rgba);
874:                 match e.tex.as_mut() {
875:                     Some(t) => t.set(img, egui::TextureOptions::LINEAR),
876:                     None => e.tex = Some(ctx.load_texture("eiga-frame", img, egui::TextureOptions::LINEAR)),
877:                 }
878:                 e.frame_wh = (f.w, f.h);
879:                 e.frames_shown += 1;
880:                 #[cfg(not(target_arch = "wasm32"))]
881:                 {
882:                     e.last_rgba = Some((f.w, f.h, f.rgba));
883:                 }
884:             }
885:         }
886:         if now - e.fps_meter.0 >= 1.0 {
887:             e.fps_meter = (now, e.frames_shown, (e.frames_shown - e.fps_meter.1) as f32 / (now - e.fps_meter.0).max(0.001) as f32);
888:         }
889:         if !e.player.paused {
890:             ctx.request_repaint();
891:         } else {
892:             ctx.request_repaint_after(std::time::Duration::from_millis(120));
893:         }
894: 
895:         if e.fs_dirty {
896:             e.fs_dirty = false;
897:             ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(e.theater));
898:         }
899:         let p = ui.painter().clone();
900:         let theater = e.theater;
901:         let stage = if theater { fr.win } else { fr.stage };
902:         if !theater {
903:             self.eiga_toolbar(ui, fr, &e);
904:             self.eiga_sidebar(ui, fr, &e);
905:         }
906:         let area = if theater { stage } else { Rect::from_min_max(stage.min + vec2(8.0, 2.0), stage.max - vec2(10.0, 10.0)) };
907:         let radius = if theater { 0.0 } else { 16.0 };
908:         // black stage
909:         grad_fill(&p, area, radius, c(4, 6, 10, 1.0), c(8, 10, 16, 1.0));
910:         let bg = ui.interact(area, egui::Id::new("eiga-surface"), Sense::click_and_drag());
911:         if ui.input(|i| i.pointer.delta().length() > 0.1) && bg.hovered() {
912:             e.last_move = now;
913:         }
914:         let hud = e.player.paused || now - e.last_move < e.cfg_hide_secs as f64 || ui.rect_contains_pointer(Rect::from_min_max(pos2(area.left(), area.bottom() - 120.0), area.max));
915:         if theater {
916:             let near_top = ui.input(|i| i.pointer.hover_pos()).map(|p| p.y < stage.top() + 110.0).unwrap_or(false);
917:             let show = near_top && hud;
918:             let bt = spring_to(&ctx, "eiga-tbar", if show { 1.0 } else { 0.0 }, 320.0, 30.0).clamp(0.0, 1.0);
919:             if show || bt > 0.02 {
920:                 ctx.request_repaint();
921:                 self.eiga_overlay_bar(ui, stage, &mut e, bt);
922:             }
923:         }
924:         // video
925:         let vp = p.with_clip_rect(area);
926:         if let Some(tex) = &e.tex {
927:             let (fw, fh) = (e.frame_wh.0 as f32, e.frame_wh.1 as f32);
928:             let rot = e.rotate % 2 == 1;
929:             let src_ar = if rot { fh / fw } else { fw / fh };
930:             let ar = match e.aspect {
931:                 2 => 16.0 / 9.0,
932:                 3 => 4.0 / 3.0,
933:                 _ => src_ar,
934:             };
935:             let (aw, ah) = (area.width(), area.height());
936:             let (mut w, mut h) = if e.aspect == 1 { if aw / ah > ar { (aw, aw / ar) } else { (ah * ar, ah) } } else if aw / ah > ar { (ah * ar, ah) } else { (aw, aw / ar) };
937:             w *= e.zoom;
938:             h *= e.zoom;
939:             let dest = Rect::from_center_size(area.center(), vec2(w, h));
940:             let uvs = [pos2(0.0, 0.0), pos2(1.0, 0.0), pos2(1.0, 1.0), pos2(0.0, 1.0)];
941:             let k = e.rotate as usize % 4;
942:             if theater {
943:                 let corners = [dest.left_top(), dest.right_top(), dest.right_bottom(), dest.left_bottom()];
944:                 let mut m = egui::epaint::Mesh::with_texture(tex.id());
945:                 for (i, cpos) in corners.iter().enumerate() {
946:                     m.vertices.push(egui::epaint::Vertex { pos: *cpos, uv: uvs[(i + k) % 4], color: Color32::WHITE });
947:                 }
948:                 m.indices.extend([0, 1, 2, 0, 2, 3]);
949:                 vp.add(egui::Shape::mesh(m));
950:             } else {
951:                 // rounded corners done with a proper rounded mesh, so bright footage never bleeds past t
```

### TOOL: shell [seq 94] 2026-11-09 14:04:20 status=completed

```json
{
  "command": "grep -rn \"domain-video\\|media\\.capnp\\|Video.*Preview\\|preview.*video\\|dropped_files\\|AppId::Video\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates 2>/dev/null | head -n 120\necho ===CONTENT_PREVIEW===\ngrep -n \"preview\\|Preview\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs | head -n 80\necho ===SHELLS===\ngrep -n \"Video\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs | head -n 60\necho ===CHROME===\ngrep -n \"dropped\\|Drop\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/*.rs | head -n 60"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:31:            AppId::Video => "Eiga",
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:39:            AppId::Video => "clapperboard",
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:84:            z: vec![AppId::Video, AppId::Terminal, AppId::Photos, AppId::Files],
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:130:            AppId::Video => {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:193:            AppId::Video => Some(&self.shell.video_w),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:199:            AppId::Video => &mut self.shell.video_w,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:382:            AppId::Video => self.draw_eiga(&mut cp, &fr),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:389:                AppId::Video => self.eiga_settings(&mut cp, shown),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:415:            if id == AppId::Video {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:493:            AppId::Video => vec![
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:737:        let items = [Di::App(AppId::Files), Di::App(AppId::Photos), Di::App(AppId::Terminal), Di::App(AppId::Video), Di::Sep, Di::Settings, Di::Palette];
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs:783:                Di::App(AppId::Video) => ("Eiga", "clapperboard", hex("#8b6cff"), hex("#3a2db0"), self.shell.video_w.open, Some(AppId::Video)),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:218:            "settings" => self.shell.settings = Some((crate::shell::AppId::Video, 0)),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:278:        self.open_app(crate::shell::AppId::Video);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:798:        if let Some((page, row, val)) = self.draw_app_settings(ui, win, crate::shell::AppId::Video, &pages, &rows_for) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:850:        let dropped: Vec<egui::DroppedFileHandle> = ctx.input(|i| i.raw.dropped_files.clone());
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:1243:            self.shell.settings = Some((crate::shell::AppId::Video, 0));
===CONTENT_PREVIEW===
570:                if e.kind.previewable() && e.kind != Kind::Audio {
576:                        self.acts.push(Act::Preview(e.loc.clone()));
===SHELLS===
22:    Video,
31:            AppId::Video => "Eiga",
39:            AppId::Video => "clapperboard",
84:            z: vec![AppId::Video, AppId::Terminal, AppId::Photos, AppId::Files],
130:            AppId::Video => {
193:            AppId::Video => Some(&self.shell.video_w),
199:            AppId::Video => &mut self.shell.video_w,
382:            AppId::Video => self.draw_eiga(&mut cp, &fr),
389:                AppId::Video => self.eiga_settings(&mut cp, shown),
415:            if id == AppId::Video {
493:            AppId::Video => vec![
737:        let items = [Di::App(AppId::Files), Di::App(AppId::Photos), Di::App(AppId::Terminal), Di::App(AppId::Video), Di::Sep, Di::Settings, Di::Palette];
783:                Di::App(AppId::Video) => ("Eiga", "clapperboard", hex("#8b6cff"), hex("#3a2db0"), self.shell.video_w.open, Some(AppId::Video)),
===CHROME===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:29:pub enum DropTarget {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:38:    pub static DROP_TARGETS: std::cell::RefCell<Vec<(DropTarget, Rect)>> = const { std::cell::RefCell::new(Vec::new()) };
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:45:pub fn drop_target_at(p: Pos2) -> Option<DropTarget> {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:541:                    DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Place(l.clone()), rect)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:569:                    DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Place(l.clone()), rect)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:624:                DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Place(loc.clone()), rect)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:681:                DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Tag(t.id.clone()), rect)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:1103:                DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Place(l.clone()), r)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:1131:    /// Browser-style tab strip for the file manager (shown from two tabs up; folders can be dropped onto a tab).
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:1147:                DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Place(l.clone()), r)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:470:        DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Folder(e.loc.clone()), r)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:492:        p.text(pos2(tx, r.top() + 45.0), Align2::LEFT_CENTER, self.fit(&p, if dropping { "Drop to move here" } else { &meta }, &f_medium(T_META), max_w), f_medium(T_META), if dropping { th.accent_ink } else { th.ink3 });
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:630:            DROP_TARGETS.with(|v| v.borrow_mut().push((DropTarget::Folder(e.loc.clone()), r)));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:703:                _ => ("folder", "This folder is empty".into(), "Drop files here, or create something new.".into(), Some(("New folder", Act::NewFolder))),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:81:                            chrome::DropTarget::Tag(id) => {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:91:                            chrome::DropTarget::Folder(dest) | chrome::DropTarget::Place(dest) => {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:103:                            chrome::DropTarget::Trash => {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs:591:            Some(DropTarget::Folder(l)) | Some(DropTarget::Place(l)) => Some(format!("Drop into {}", l.name())),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs:592:            Some(DropTarget::Tag(t)) => Some(format!("Tag {}", t)),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs:593:            Some(DropTarget::Trash) => Some("Move to Trash".into()),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/photos.rs:1427:                    SRow::Button { icon: "trash-bin-minimalistic", label: "Reset thumbnails", sub: "Drop the thumbnail cache and rebuild", cta: "Reset" },
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/pty.rs:286:    impl Drop for Live {
/home/lufun
```

### TOOL: shell [seq 101] 2026-11-09 14:04:30 status=completed

```json
{
  "command": "grep -n \"Preview\\|previewable\\|Act::Preview\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs 2>/dev/null | head -n 120\necho ===LIB_DROP===\nsed -n '1,160p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs\necho ===OVERLAYS_PREVIEW===\ngrep -n \"fn.*preview\\|struct.*Preview\\|TextPreview\\|open_with\\|OpenWith\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs | head -n 40\necho ===CORE_MODEL===\ngrep -n \"previewable\\|enum Kind\\|fn thumbnails\\|thumbnail\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs 2>/dev/null | head -n 80"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs:52:    Preview(Location),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs:156:    Preview(Location),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs:1004:            Act::Preview(l) => {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs:1618:        self.modal = Modal::Preview(l);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs:138:                    v.push(mi("eye", "Quick Look", Act::Preview(t.clone())).k("Space").off(!e.as_ref().is_some_and(|e| e.kind.previewable())));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs:151:                            v.push(mi("music-notes", "Open in Smart Player", Act::Preview(t.clone())));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:225:                if let Modal::Preview(_) = self.modal {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:245:        if let Modal::Preview(l) = self.modal.clone() {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:246:            let list: Vec<Location> = self.items.iter().filter(|x| x.kind.previewable()).map(|x| x.loc.clone()).collect();
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:249:                    self.act(Act::Preview(list[i + 1].clone()));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:252:                    self.act(Act::Preview(list[i - 1].clone()));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:318:            if s.len() == 1 && s[0].kind.previewable() {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:319:                self.act(Act::Preview(s[0].loc.clone()));
===LIB_DROP===
//! egui frontend for Smart Files. All behaviour lives in `smart-core`; this crate only draws and routes input.
pub mod app;
pub mod appkit;
pub mod chrome;
pub mod content;
pub mod copyfx;
pub mod icons;
pub mod highlight;
pub mod icons_gen;
pub mod modals;
pub mod overlays;
pub mod paint;
pub mod photos;
pub mod pty;
pub mod storage;
pub mod video;
pub mod shell;
pub mod terminal;
#[cfg(not(target_arch = "wasm32"))]
pub mod termlive;
pub mod theme;

use app::*;
use eframe::egui;
use smart_core::*;

impl eframe::App for SmartApp {
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        if IS_WEB { [0.56, 0.62, 0.72, 1.0] } else { [0.0, 0.0, 0.0, 0.0] }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = &ui.ctx().clone();
        self.time = ctx.input(|i| i.time);
        self.dt = ctx.input(|i| i.stable_dt).min(0.1);
        self.pump(ctx);
        // debounce live search
        if self.search_active() && self.search.query.trim() != self.search.last_sent && self.time - self.search.dirty_at > 0.10 {
            self.run_search();
        }
        if !self.search_active() && !self.search.last_sent.is_empty() {
            self.clear_search();
        }
        // sidebar slide
        let target = if self.st().sidebar_collapsed || (IS_WEB && ctx.content_rect().width() < 760.0) { 0.0 } else { 1.0 };
        if (self.sidebar_open - target).abs() > 0.002 {
            self.sidebar_open += (target - self.sidebar_open) * (1.0 - (-self.dt * 16.0).exp());
            ctx.request_repaint();
        } else {
            self.sidebar_open = target;
        }
        self.handle_keys(ctx);
        let vis = if self.th.dark { egui::Visuals::dark() } else { egui::Visuals::light() };
        if ctx.global_style().visuals.dark_mode != vis.dark_mode {
            ctx.set_visuals(vis);
            ctx.global_style_mut(|s| {
                s.visuals.selection.bg_fill = theme::alpha(self.th.accent, 0.3);
                s.visuals.text_cursor.stroke = egui::Stroke::new(1.5, self.th.accent);
            });
        }
        overlays::WIN_RECT.with(|w| w.set(self.window_rect(ui.max_rect())));
        self.draw_window(ui);
        self.native_resize_grips(ctx);
        if !(self.win_closed || self.win_min) {
            self.draw_menu(ctx);
            self.draw_modal(ctx);
            self.draw_palette(ctx);
            self.draw_ai(ctx);
            self.draw_search_sheet(ctx);
            self.draw_jobs_card(ctx);
            self.draw_toasts(ctx);
            self.draw_drag_ghost(ctx);
        }
        // resolve a drag exactly once, against whatever was under the pointer
        if ctx.input(|i| i.pointer.any_released()) {
            if let Some(drag) = self.drag.take() {
                let modifiers = ctx.input(|i| i.modifiers);
                if let Some(pos) = ctx.pointer_latest_pos() {
                    if let Some(target) = chrome::drop_target_at(pos) {
                        match target {
                            chrome::DropTarget::Tag(id) => {
                                let n = drag.items.len();
                                for l in &drag.items {
                                    if !self.lib(|lib| lib.has_tag(l, &id)) {
                                        self.be.library.write().unwrap().toggle_tag(l, &id);
                                    }
                                }
                                let name = self.lib(|l| l.tags.iter().find(|t| t.id == id).map(|t| t.name.clone())).unwrap_or(id);
                                self.toast(format!("Tagged {} item{} as {}", n, if n == 1 { "" } else { "s" }, name), true);
                            }
                            chrome::DropTarget::Folder(dest) | chrome::DropTarget::Place(dest) => {
                                let srcs: Vec<_> = drag.items.into_iter().filter(|s| s != &dest && !dest.starts_with(s) && s.parent().as_ref() != Some(&dest)).collect();
                                if srcs.is_empty() {
                                    self.toast("That is already the destination", false);
                                } else {
                                    let cross = srcs.iter().any(|s| s.fs != dest.fs);
                                    let copy = modifiers.alt || modifiers.ctrl || modifiers.command || cross;
                                    let n = srcs.len();
                                    self.submit_job(if copy { smart_core::JobKind::Copy { srcs, dest: dest.clone() } } else { smart_core::JobKind::Move { srcs, dest: dest.clone() } });
                                    self.toast(format!("{} {} item{} to {}", if copy { "Copying" } else { "Moving" }, n, if n == 1 { "" } else { "s" }, dest.name()), true);
                                }
                            }
                            chrome::DropTarget::Trash => {
                                self.submit_job(smart_core::JobKind::Trash { srcs: drag.items });
                                self.toast("Moved to Trash", true);
                            }
                        }
                    }
                }
            }
        }
        // drain queued actions (widgets push, we apply after the frame)
        for _ in 0..4 {
            let mut v = std::mem::take(&mut self.acts);
            v.extend(chrome::ACTS.with(|q| std::mem::take(&mut *q.borrow_mut())));
            if v.is_empty() {
                break;
            }
            for a in v {
                match &a {
                    Act::Toast(t) if t == "\u{1}" => self.drag = None,
                    Act::Toast(t) if t.is_empty() || t == "\u{0}" => {}
                    _ => self.act(a),
                }
            }
        }
        if let Some(t) = self.pending_copy.take() {
            ctx.copy_text(t);
        }
        // big thumbnails for the preview
        if self.be.audio.is_loaded() {
            ctx.request_repaint_a
```

### TOOL: shell [seq 108] 2026-11-09 14:04:38 status=completed

```json
{
  "command": "sed -n '60,180p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs\necho ===THUMBS===\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs\necho ===MODALS_PREVIEW===\ngrep -n \"Preview\\|OpenWith\\|Audio\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs | head -n 80"
}
```

```
        while let Some(c) = cur {
            cur = c.parent();
            v.push(c);
        }
        v.reverse();
        v
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    Folder,
    Image,
    Audio,
    Video,
    Pdf,
    Document,
    Text,
    Code,
    Archive,
    App,
    Vector,
    Project,
    Other,
}

impl Kind {
    pub fn from_ext(ext: &str) -> Kind {
        match ext.to_ascii_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tif" | "tiff" | "heic" | "avif" | "ico" => Kind::Image,
            "svg" => Kind::Vector,
            "wav" | "mp3" | "flac" | "ogg" | "oga" | "m4a" | "aac" | "opus" | "aif" | "aiff" => Kind::Audio,
            "mp4" | "mkv" | "mov" | "webm" | "avi" | "m4v" => Kind::Video,
            "pdf" => Kind::Pdf,
            "doc" | "docx" | "odt" | "rtf" | "pages" | "xls" | "xlsx" | "ods" | "ppt" | "pptx" | "odp" | "key" => Kind::Document,
            "txt" | "md" | "markdown" | "log" | "csv" | "json" | "toml" | "yaml" | "yml" | "ini" | "conf" | "cfg" => Kind::Text,
            "rs" | "js" | "ts" | "tsx" | "jsx" | "py" | "c" | "h" | "cpp" | "hpp" | "go" | "java" | "kt" | "swift" | "sh" | "html" | "css" | "lua" | "rb" | "php" => Kind::Code,
            "zip" | "tar" | "gz" | "xz" | "bz2" | "zst" | "7z" | "rar" | "tgz" => Kind::Archive,
            "appimage" | "exe" | "msi" | "app" | "deb" | "rpm" | "apk" | "dmg" | "flatpak" => Kind::App,
            "flp" | "als" | "logicx" | "ptx" | "psd" | "fig" | "sketch" | "blend" | "xd" | "ai" => Kind::Project,
            _ => Kind::Other,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Kind::Folder => "Folder",
            Kind::Image => "Image",
            Kind::Audio => "Audio",
            Kind::Video => "Video",
            Kind::Pdf => "PDF document",
            Kind::Document => "Document",
            Kind::Text => "Text",
            Kind::Code => "Source code",
            Kind::Archive => "Archive",
            Kind::App => "Application",
            Kind::Vector => "Vector image",
            Kind::Project => "Project",
            Kind::Other => "Document",
        }
    }
    /// Name of the icon in the frontend's Solar icon set.
    pub fn icon(self) -> &'static str {
        match self {
            Kind::Folder => "folder",
            Kind::Image => "gallery",
            Kind::Audio => "music-note",
            Kind::Video => "clapperboard",
            Kind::Pdf | Kind::Document => "document-text",
            Kind::Text => "text",
            Kind::Code => "terminal",
            Kind::Archive => "archive",
            Kind::App => "widget",
            Kind::Vector => "stars-minimalistic",
            Kind::Project | Kind::Other => "file",
        }
    }
    pub fn previewable(self) -> bool {
        matches!(self, Kind::Image | Kind::Audio | Kind::Text | Kind::Code | Kind::Video | Kind::Pdf | Kind::Vector)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub name: String,
    pub loc: Location,
    pub is_dir: bool,
    pub size: u64,
    /// Unix seconds.
    pub modified: Option<i64>,
    pub kind: Kind,
    pub ext: String,
    pub hidden: bool,
    pub symlink: bool,
    pub readonly: bool,
    /// Size is a demo promise, not the bytes on disk (the 5 GB showcase file). Transfers of it are paced.
    pub paced: bool,
}

impl Entry {
    pub fn new(loc: Location, is_dir: bool, size: u64, modified: Option<i64>) -> Entry {
        let name = loc.name();
        let ext = if is_dir { String::new() } else { ext_of(&name) };
        let kind = if is_dir { Kind::Folder } else { Kind::from_ext(&ext) };
        Entry { hidden: name.starts_with('.'), name, loc, is_dir, size, modified, kind, ext, symlink: false, readonly: false, paced: false }
    }
    pub fn kind_label(&self) -> String {
        if self.is_dir {
            return "Folder".into();
        }
        let e = self.ext.to_ascii_uppercase();
        match self.kind {
            Kind::Image if !e.is_empty() => format!("{} image", if e == "JPG" { "JPEG".into() } else { e }),
            Kind::Audio if !e.is_empty() => format!("{} audio", e),
            Kind::Video if !e.is_empty() => format!("{} video", e),
            Kind::Archive if !e.is_empty() => format!("{} archive", e),
            Kind::Text if e == "MD" => "Markdown".into(),
            Kind::Text if e == "TXT" => "Plain text".into(),
            Kind::Project if e == "FLP" => "FL Studio project".into(),
            Kind::App if e == "APPIMAGE" => "Linux app".into(),
            k if e.is_empty() => k.label().into(),
            k => format!("{} {}", e, k.label().to_lowercase()),
===THUMBS===
//! Background thumbnail decoding (image crate). Frontends upload the RGBA to a texture.
use crate::exec::THREADED;
use crate::model::Location;
use crate::ops::Registry;
use std::collections::HashSet;
use std::sync::{Arc, Condvar, Mutex};

pub struct ThumbResult {
    pub loc: Location,
    pub size: [usize; 2],
    pub rgba: Vec<u8>,
    pub original: Option<(u32, u32)>,
    pub edge: u32,
}

#[derive(Default)]
struct Shared {
    queue: Mutex<Vec<(Location, u32)>>,
    cv: Condvar,
    out: Mutex<Vec<ThumbResult>>,
}

pub struct Thumbs {
    shared: Arc<Shared>,
    registry: Arc<Registry>,
    seen: Mutex<HashSet<(Location, u32)>>,
}

fn decode(reg: &Registry, loc: &Location, edge: u32) -> Option<ThumbResult> {
    let v = reg.get(&loc.fs)?;
    let bytes = v.read(&loc.path, 48 << 20).ok()?;
    let img = image::load_from_memory(&bytes).ok()?;
    let original = Some((img.width(), img.height()));
    let t = img.thumbnail(edge, edge).to_rgba8();
    Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
}

impl Thumbs {
    pub fn new(registry: Arc<Registry>) -> Self {
        let shared = Arc::new(Shared::default());
        if THREADED {
            let n = 2;
            for _ in 0..n {
                let (s, r) = (shared.clone(), registry.clone());
                crate::exec::spawn(move || loop {
                    let job = {
                        let mut q = s.queue.lock().unwrap();
                        while q.is_empty() {
                            q = s.cv.wait(q).unwrap();
                        }
                        q.pop() // newest first = what is on screen now
                    };
                    if let Some((loc, edge)) = job {
                        let res = decode(&r, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
                        s.out.lock().unwrap().push(res);
                    }
                });
            }
        }
        Self { shared, registry, seen: Mutex::new(HashSet::new()) }
    }

    pub fn request(&self, loc: &Location, edge: u32) {
        if !self.seen.lock().unwrap().insert((loc.clone(), edge)) {
            return;
        }
        self.shared.queue.lock().unwrap().push((loc.clone(), edge));
        self.shared.cv.notify_one();
    }

    pub fn forget(&self, loc: &Location) {
        self.seen.lock().unwrap().retain(|(l, _)| l != loc);
    }

    /// Drain finished thumbnails. In the browser this also does the decoding, a few per call.
    pub fn poll(&self) -> Vec<ThumbResult> {
        if !THREADED {
            for _ in 0..2 {
                let job = self.shared.queue.lock().unwrap().pop();
                let Some((loc, edge)) = job else { break };
                let r = decode(&self.registry, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
                self.shared.out.lock().unwrap().push(r);
            }
        }
        std::mem::take(&mut *self.shared.out.lock().unwrap())
    }

    pub fn pending(&self) -> bool {
        !self.shared.queue.lock().unwrap().is_empty()
    }
}
===MODALS_PREVIEW===
139:            Modal::Preview(l) => {
141:      
```

### TOOL: shell [seq 115] 2026-11-09 14:04:43 status=completed

```json
{
  "command": "sed -n '130,300p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs\necho ====M_PREVIEW====\ngrep -n \"fn m_preview\\|fn preview_\\|fn m_openwith\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs"
}
```

```
            let born = ctx.data_mut(|d| *d.get_temp_mut_or_insert_with(born_id, || self.time));
            (self.modal.clone(), false, (self.time - born) as f32)
        };
        if exiting || anim_t < 0.6 {
            ctx.request_repaint();
        }
        let win = win_rect();
        let th = self.th.clone();
        let (mut w, mut h): (f32, f32) = match &m {
            Modal::Preview(l) => {
                let kind = self.be.stat(l).map(|e| e.kind).unwrap_or(Kind::Other);
                if kind == Kind::Audio {
                    (640.0f32.min(win.width() - 60.0), 520.0f32.min(win.height() - 60.0))
                } else if matches!(kind, Kind::Image | Kind::Vector) {
                    let sz = self.big_thumbs.get(l).or_else(|| self.thumbs.get(l)).and_then(|t| t.orig).map(|(a, b)| (a as f32, b as f32)).unwrap_or((16.0, 10.0));
                    let ar = (sz.0 / sz.1.max(1.0)).clamp(0.45, 2.6);
                    let footer = 96.0;
                    let maxw = (win.width() - 70.0).min(1120.0);
                    let maxh = win.height() - 64.0;
                    let w = maxw;
                    let h = w / ar + footer;
                    if h <= maxh { (w, h.max(320.0)) } else { let hh = maxh; let ww = ((hh - footer) * ar + 24.0).max(420.0); (ww.min(maxw), hh) }
                } else {
                    ((win.width() - 120.0).min(900.0), 660.0f32.min(win.height() - 60.0))
                }
            }
            Modal::Info(ls) => (500.0, self.info_h(ls)),
            Modal::OpenWith(l) => (480.0, 84.0 + 56.0 * (1 + self.be.apps_for(l).len().min(6)) as f32 + if self.be.apps_for(l).is_empty() { 40.0 } else { 0.0 } + 74.0),
            Modal::Wallpaper => (600.0, 470.0),
            Modal::About => (540.0, about_h()),
            Modal::Shortcuts => (640.0, 90.0 + ((SHORTCUTS.len() + 1) / 2) as f32 * 34.0 + 78.0),
            Modal::Confirm(_) => (440.0, 190.0),
            Modal::Devices => (700.0, 76.0 + ((self.devices.len() + 1) / 2).max(1) as f32 * 112.0 + 6.0 + 74.0),
            Modal::Transfer => (780.0, 540.0),
            Modal::Phone => (560.0, 80.0 + 84.0 + 40.0 + self.phone_folders_n() as f32 * 46.0 + 86.0),
            Modal::Tokens => (680.0, 600.0),
            Modal::NewTag => (420.0, 190.0),
            Modal::Connect => (520.0, 330.0),
            Modal::Settings => ((win.width() - 56.0).min(930.0), (win.height() - 48.0).min(700.0)),
            Modal::Archive => (660.0f32.min(win.width() - 40.0), 600.0f32.min(win.height() - 50.0)),
            Modal::None => (0.0, 0.0),
        };
        w = w.min(win.width() - 30.0);
        h = h.min(win.height() - 30.0);
        let mut close = false;
        let mut acts: Vec<Act> = vec![];
        let layer = Id::new("modal-layer");
        let reduce = self.lib(|l| l.settings.reduce_motion);
        let (scale, dy, fade) = if exiting {
            let e = anim_t * anim_t;
            (1.0 - 0.035 * e, 6.0 * e, 1.0 - e)
        } else if reduce {
            (1.0, 0.0, 1.0)
        } else {
            let pop = spring_out(anim_t);
            (0.955 + 0.045 * pop, (1.0 - pop.min(1.0)) * 16.0, (anim_t * 9.0).clamp(0.0, 1.0))
        };
        let ctr = win.center();
        ctx.set_transform_layer(egui::LayerId::new(Order::Foreground, layer), egui::emath::TSTransform::new(ctr.to_vec2() * (1.0 - scale) + vec2(0.0, dy), scale));
        egui::Area::new(layer).order(Order::Foreground).fixed_pos(win.min).show(ctx, |ui| {
            let p = ui.painter().clone();
            ui.set_clip_rect(win);
            ui.set_opacity(fade);
            if exiting {
                ui.disable();
            }
            let scrim = ui.allocate_rect(win, Sense::click());
            grad_fill(&p, win, R_WIN, th.scrim, th.scrim);
            let card = Rect::from_center_size(win.center(), vec2(w, h));
            elevate(&p, &th, card, 22.0, 5);
            grad_fill(&p, card, 22.0, th.panel_top, th.panel_bottom);
            rim(&p, card, 22.0, 1.4, th.rim, true, 60.0);
            ui.allocate_rect(card, Sense::click());
            if scrim.clicked() && !exiting && !matches!(m, Modal::Confirm(_)) {
                close = true;
            }
            match &m {
                Modal::Preview(l) => self.m_preview(ui, card, l, &mut close),
                Modal::Info(ls) => self.m_info(ui, card, ls, &mut close),
                Modal::OpenWith(l) => self.m_openwith(ui, card, l, &mut close),
                Modal::Wallpaper => self.m_wallpaper(ui, card, &mut close),
                Modal::About => self.m_about(ui, card, &mut close),
                Modal::Shortcuts => self.m_shortcuts(ui, card, &mut close),
                Modal::Confirm(c) => self.m_confirm(ui, card, c, &mut close, &mut acts),
                Modal::Devices => self.m_devices(ui, card, &mut close),
                Modal::Transfer => self.m_transfer(ui, card, &mut close),
                Modal::Phone => self.m_phone(ui, card, &mut close),
                Modal::Tokens => self.m_tokens(ui, card, &mut close),
                Modal::NewTag => self.m_newtag(ui, card, &mut close),
                Modal::Connect => self.m_connect(ui, card, &mut close),
                Modal::Settings => self.m_settings(ui, card),
                Modal::Archive => self.m_archive(ui, card, &mut close),
                Modal::None => {}
            }
        });
        for a in acts {
            self.acts.push(a);
        }
        if close {
            self.acts.push(Act::CloseModal);
            if matches!(m, Modal::Preview(_)) {
                self.be.audio.stop();
                self.audio_loc = None;
            }
        }
    }

    // ───────────── Quick Look ─────────────
    fn m_preview(&mut self, ui: &mut Ui, card: Rect, l: &Location, close: &mut bool) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let Ok(e) = self.be.stat(l) else {
            *close = true;
            return;
        };
        let footer = 92.0;
        let media = Rect::from_min_max(card.min + vec2(10.0, 10.0), pos2(card.right() - 10.0, card.bottom() - footer));
        // deep stage: imagery fills it edge-to-edge (no grey letterbox)
        grad_fill(&p, media, 16.0, if th.dark { c(6, 10, 18, 0.96) } else { c(14, 20, 33, 0.94) }, if th.dark { c(9, 13, 23, 0.97) } else { c(20, 27, 42, 0.97) });
        rim(&p, media, 16.0, 3.0, alpha(Color32::BLACK, 0.40), true, 16.0);
        match e.kind {
            Kind::Image => {
                let tex = self.big_thumbs.get(l).or_else(|| self.thumbs.get(l));
                if let Some(t) = tex {
                    let sz = t.tex.size_vec2();
                    textured_rrect(&p, media, 14.0, t.tex.id(), cover_uv(sz.x, sz.y, media), Color32::WHITE);
                    // subtle vignette + top light so the photo sits into the stage
                    rim(&p, media, 14.0, 1.2, Color32::from_white_alpha(50), true, 30.0);
                    let shade = Rect::from_min_max(pos2(media.left(), media.bottom() - 74.0), media.max);
                    let mut m = egui::epaint::Mesh::default();
                    for (y, col) in [(shade.top(), Color32::TRANSPARENT), (shade.bottom(), c(0, 0, 0, 0.42))] {
                        m.vertices.push(egui::epaint::Vertex { pos: pos2(shade.left(), y), uv: egui::epaint::WHITE_UV, color: col });
                        m.vertices.push(egui::epaint::Vertex { pos: pos2(shade.right(), y), uv: egui::epaint::WHITE_UV, color: col });
                    }
                    m.indices.extend([0, 1, 2, 1, 3, 2]);
                    p.add(egui::Shape::mesh(m));
                } else {
                    p.text(media.center(), Align2::CENTER_CENTER, "Loading\u{2026}", f_medium(T_BODY), Color32::from_white_alpha(200));
                    self.be.request_thumb(l, 1800);
                    ui.ctx().request_repaint();
                }
            }
            Kind::Audio => self.preview_audio(ui, media, &e),
            Kind::Text | Kind::Code
```

### TOOL: shell [seq 123] 2026-11-09 14:04:46 status=completed

```json
{
  "command": "sed -n '300,600p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs"
}
```

```
                                let nums: String = (1..=lines).map(|n| n.to_string()).collect::<Vec<_>>().join("\n");
                                ui.add(egui::Label::new(egui::RichText::new(nums).monospace().size(12.5).color(hex("#4d5b78"))).selectable(false));
                                ui.add_space(10.0);
                            } else {
                                ui.add_space(12.0);
                            }
                            ui.add(egui::Label::new((*job).clone()).selectable(true));
                        });
                    });
                    if !crate::highlight::has_grammar(&ext) && !wrap {
                        p.text(pos2(code.right() - 14.0, code.bottom() - 12.0), Align2::RIGHT_CENTER, "basic highlighting", f_medium(10.0), Color32::from_white_alpha(70));
                    }
                }
            }
            Kind::Archive => {
                let entries = self.be.archive_entries(l, None).unwrap_or_default();
                p.text(pos2(media.left() + 26.0, media.top() + 26.0), Align2::LEFT_CENTER, format!("{} \u{b7} {} entries", e.name, entries.len()), f_semi(T_TITLE), Color32::from_white_alpha(235));
                let mut y = media.top() + 50.0;
                for z in entries.iter().take(11) {
                    let row = Rect::from_min_size(pos2(media.left() + 18.0, y), vec2(media.width() - 36.0, 30.0));
                    p.rect_filled(row, cr(8.0), Color32::from_white_alpha(10));
                    self.ico(&p, if z.is_dir { "folder" } else { "file" }, Ic::Duo, pos2(row.left() + 16.0, row.center().y), 14.0, Color32::from_white_alpha(200));
                    p.text(pos2(row.left() + 30.0, row.center().y), Align2::LEFT_CENTER, self.fit(&p, &z.name, &f_medium(T_META), row.width() - 180.0), f_medium(T_META), Color32::from_white_alpha(230));
                    p.text(pos2(row.right() - 14.0, row.center().y), Align2::RIGHT_CENTER, if z.is_dir { "folder".into() } else { format_size(z.size) }, f_medium(10.5), Color32::from_white_alpha(160));
                    y += 33.0;
                }
            }
            _ => {
                let kc = kind_color(e.kind, &th);
                let t = Rect::from_center_size(media.center() - vec2(0.0, 12.0), vec2(110.0, 110.0));
                grad_fill(&p, t, 32.0, alpha(kc, 0.28), alpha(kc, 0.12));
                rim(&p, t, 32.0, 1.2, Color32::from_white_alpha(60), true, 40.0);
                self.ico(&p, e.kind.icon(), Ic::Duo, t.center(), 52.0, kc);
                p.text(media.center() + vec2(0.0, 84.0), Align2::CENTER_CENTER, "No built-in preview for this kind of file", f_medium(T_BODY), Color32::from_white_alpha(200));
            }
        }
        if e.kind == Kind::Archive || e.kind == Kind::Audio {
            let b = Rect::from_center_size(pos2(media.center().x, media.bottom() - 30.0), vec2(170.0, 34.0));
            if self.label_button(ui, b, Some("folder-open"), if e.kind == Kind::Archive { "Open archive" } else { "Full player" }, "pv-special", Btn::Raised).clicked() {
                if e.kind == Kind::Archive {
                    self.acts.push(Act::ArchiveBrowse(l.clone()));
                } else {
                    self.acts.push(Act::TogglePlayPause(l.clone()));
                }
            }
        }
        // overlay toolbar
        let mut tx = media.right() - 24.0;
        let tools: Vec<(&'static str, &'static str, Act)> = match e.kind {
            Kind::Image => vec![("gallery-wide", "pv-wall", Act::WallpaperFrom(l.clone())), ("widget", "pv-openwith", Act::Modal(Modal::OpenWith(l.clone()))), ("info-circle", "pv-info", Act::Info)],
            Kind::Archive => vec![("folder-open", "pv-extract", Act::ExtractHere(l.clone())), ("copy", "pv-arc", Act::ArchiveBrowse(l.clone())), ("info-circle", "pv-info", Act::Info)],
            _ => vec![("widget", "pv-openwith", Act::Modal(Modal::OpenWith(l.clone()))), ("info-circle", "pv-info", Act::Info)],
        };
        for (icon, id, a) in tools {
            let b = Rect::from_center_size(pos2(tx, media.top() + 24.0), vec2(34.0, 34.0));
            tx -= 40.0;
            if self.icon_button(ui, b, icon, id, Btn::Raised, true).clicked() {
                self.acts.push(a);
            }
        }
        let close_b = Rect::from_center_size(pos2(media.right() - 24.0, media.bottom() - 24.0), vec2(34.0, 34.0));
        if self.icon_button(ui, close_b, "close-circle", "pv-close", Btn::Raised, true).clicked() {
            *close = true;
        }
        // prev / next
        let list: Vec<Location> = self.items.iter().filter(|x| x.kind.previewable()).map(|x| x.loc.clone()).collect();
        if let Some(i) = list.iter().position(|x| x == l) {
            let count = format!("{} of {}", i + 1, list.len());
            let pill = Rect::from_center_size(pos2(media.center().x, media.bottom() - 24.0), vec2(88.0, 28.0));
            grad_fill(&p, pill, 14.0, c(8, 12, 20, 0.55), c(8, 12, 20, 0.55));
            p.text(pill.center(), Align2::CENTER_CENTER, count, f_semi(11.0), Color32::from_white_alpha(235));
            for (dir, icon, x) in [(-1i32, "alt-arrow-left", media.left() + 28.0), (1, "alt-arrow-right", media.right() - 28.0)] {
                let ok = (dir < 0 && i > 0) || (dir > 0 && i + 1 < list.len());
                if !ok {
                    continue;
                }
                let r = Rect::from_center_size(pos2(x, media.center().y), vec2(42.0, 42.0));
                if self.icon_button(ui, r, icon, if dir < 0 { "pv-prev" } else { "pv-next" }, Btn::Raised, true).clicked() {
                    self.acts.push(Act::Preview(list[(i as i32 + dir) as usize].clone()));
                }
            }
        }
        // footer info
        let y = card.bottom() - 42.0;
        p.text(pos2(card.left() + 26.0, y - 10.0), Align2::LEFT_CENTER, self.fit(&p, &e.name, &f_bold(T_TITLE), card.width() - 320.0), f_bold(T_TITLE), th.ink1);
        let dims = self.big_thumbs.get(l).or_else(|| self.thumbs.get(l)).and_then(|t| t.orig).map(|(a, b)| format!("  \u{b7}  {} \u{d7} {}", a, b)).unwrap_or_default();
        let meta = format!("{}{}  \u{b7}  {}  \u{b7}  {}", e.kind_label(), dims, format_size(e.size), e.modified.map(format_date).unwrap_or_default());
        p.text(pos2(card.left() + 26.0, y + 10.0), Align2::LEFT_CENTER, meta, f_medium(T_META), th.ink3);
        let mut cur = card.right() - 22.0;
        let open_r = Rect::from_min_size(pos2(cur - 96.0, y - 17.0), vec2(96.0, 34.0));
        if self.label_button(ui, open_r, Some("arrow-right-up"), "Open", "pv-open", Btn::Accent).clicked() {
            self.be.open(l);
        }
        cur -= 104.0;
        let rev = Rect::from_min_size(pos2(cur - 104.0, y - 17.0), vec2(104.0, 34.0));
        if e.kind == Kind::Archive {
            if self.label_button(ui, rev, Some("folder-open"), "Extract", "pv-extract2", Btn::Raised).clicked() {
                self.acts.push(Act::ExtractHere(l.clone()));
            }
        } else if self.label_button(ui, rev, Some("folder"), "Reveal", "pv-reveal", Btn::Raised).clicked() {
            if let Some(par) = l.parent() {
                self.acts.push(Act::CloseModal);
                self.acts.push(Act::Nav(Route::Loc(par)));
                self.select_after_list = vec![l.clone()];
            }
        }
    }

    fn preview_audio(&mut self, ui: &mut Ui, media: Rect, e: &Entry) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let kc = th.warn;
        let c = media.center();
        let art = Rect::from_center_size(c - vec2(0.0, 92.0), vec2(120.0, 120.0));
        elevate(&p, &th, art, 30.0, 3);
        grad_fill(&p, art, 30.0, mix(kc, Color32::WHITE, 0.2), mix(kc, Color32::BLACK, 0.2));
        rim(&p, art, 30.0, 1.4, Color32::from_white_alpha(120), true, 50.0);
        self.ico(&p, "music-notes", Ic::Filled, art.center(), 54.0, Color32::from_white_alpha(235));
        let wf: Vec<f32> = self.waveform.as_ref().filter(|(l, _)| *l =
```

### TASK / USER [seq 130] 2026-11-09 14:04:49

CLARIFICATION from user: hypnotic animated caret is whole framework, not terminal-only. All smart-suite apps need it (search, rename, pickers, AI composer, dialogs). Re-read: in VIDEO-SPECS.md, make caret a smart-ui-egui shared component (inherits framework style), not terminal-owned. Preserve breathing/jump hypnotic feel across all inputs. Update spec.

### TOOL: read [seq 131] 2026-11-09 14:04:49 status=completed

```json
{
  "path": "/home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md"
}
```

```
Read file /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md, lines 1-361
1: # EIGA UNKNOWN UNKNOWNS — Deep Audit v2 (Protocol-First, Honest Player)
2: 
3: **Status:** `UNKNOWN UNKNOWNS AUDITED — DO NOT SHIP PLAYER WITHOUT §3 L–items + §6 Performance + §9 Acceptance`
4: **Date:** 2026-11-09
5: **Source roots (last 7 days, hidden excluded, non-destructive):**
6: - `/home/lufuno/Documents/rust-wasm-media-manager-latest.zip` (58 M, 2026-11-06 19:02) — canonical superset; byte-identical entries to `-2.zip` and to live `/home/lufuno/crates/*` (`eiga-core`, `smart-core`, `smart-ui-egui`, `xtask`). Workspace `members = [smart-core, eiga-core, smart-ui-egui, xtask]`, `lto=fat, opt=3, panic=abort`.
7: - `/home/lufuno/Documents/rust-wasm-media-manager.zip` (23 M, 2026-11-06 11:21) — same `eiga-core` blobs, `video.rs 55274 B` (older, missing picker/drop/cleanup/settings/remove-reveal).
8: - `/home/lufuno/Documents/rust-wasm-media-manager-2.zip` (58 M) — identical to latest; not a distinct lineage.
9: - `refine-futuristic-desktop-environment-bad*.zip` (`work/BIGBOY/`) — `smart-desktop/` shell (assist/capture/files/photos/settings/terminal + core identical to smart-files core). **Zero `eiga-core`, zero `video.rs`, zero media player.** No eiga-adjacent code to take except `protocol/schema/{audio,capture,clipboard,compositor,...}.capnp` inside `smart-desktop/protocol/schema/` (older snapshot of smart-protocol). Do not merge.
10: - `prod-very-good` production line (`smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip`, 25 M) — **has `scripts/build-appimage.sh`** + `thumbs.rs 5183 B` with `video_thumb()`; **has no `eiga-core`, no `analyze.rs`, no `video.rs`, no `highlight.rs`**. This is why Eiga AppImage / video thumbs are missing in the media line.
11: - Live protocol: `/home/lufuno/smart/smart-protocol-official/schema/*.capnp` (`media.capnp @0xa1b2c3d4e5f60011`, `audio.capnp`, `files.capnp`, `clipboard.capnp`, `notification.capnp`, `app.capnp`, `device.capnp`, `capability.capnp`, `types.capnp`) + compiled Rust `/home/lufuno/smart/smart-protocol-rs/` (`schema-*` features, `capnpc` codegen, `contract.rs`). Eiga today is **protocol-free**: no `domain-media`, no `domain-video` implementation — all §8 work is new.
12: - Future engine candidate (separate workspace): `/home/lufuno/smart/smartui-workspace/smartui-media/` (9 569 lines, `no_std`, `engine/session.rs` “timeline-is-king”, `latch::FrameLatch`, `ring::SpscRing`, `engine/ffmpeg_cli.rs`, `media_ux.rs`, `dsp/`, `out_audio/`). `examples/media-cinema` is a SmartUI demo player. **Not on the canonical import path** — do not merge wholesale; mine its invariants (§1.3).
13: 
14: **Companion docs:** File-manager `CORE-SPECS`/`UX-SPECS` in `smart-suite-official/audits/` + `CANONICAL-SPECS.md` in `smart-suite-official/specs/` (video thumbs pipeline already specified there; this spec is the *player* companion — protocol-first with backends disposable).
15: 
16: **Rule for this spec:** `eiga-core` is the wire. `smart-core` is the library. `smart-protocol-rs` (`media.capnp`, `audio.capnp`, `files.capnp`, `clipboard.capnp`, `notification.capnp`) is the contract. Any decode backend (ffmpeg CLI, ffmpeg-ng static, symphonia/rodio, smartui-media pipes, Web `<video>`, cast) is disposable. No UI code in `eiga-core`; no decode code in the view. Every “lie” is a ship-blocker.
17: 
18: ---
19: 
20: ## 0. Verdict in One Page
21: 
22: **Does Eiga really play? Yes — natively, for real local files, via ffmpeg, with real audio and real subtitles. 39/39 `eiga-core` tests pass on this host, including `end_to_end_with_real_ffmpeg` (probe → decode ≥15 frames → seek → SRT extract → PCM pipe).** The pipe is honest: `-hwaccel auto` decode → raw `rgba` video + `f32le 48 kHz stereo` audio → `rodio` device. Test card, web-preview banner, mpv key map, playlist, A-B loop, chapters, sidecar + embedded subs, OSD, stats, help, file picker, drag-drop, screenshot-to-Pictures are all wired and testable. That is the good news.
23: 
24: **It is not shippable as-is.** Twelve product-facing lies/misleading stubs (L-01…L-24) make speed, audio-delay, rotation, resolution, seek-accuracy, HDR, network, spatial audio, live, and TestCard honesty appear to work when they do not. Another twelve gaps were never thought of (U-01…U-22). The thumb pipeline regressed relative to prod (`video_thumb` lost): the file-manager gallery still shows blank tiles for video. AppImage packaging does not exist in the media line (no `build-appimage.sh`), ffmpeg is not bundled, and there is no protocol service (`media.capnp` unimplemented).
25: 
26: **What to keep:** `player.rs` state machine + playlist + key map (pure, 16 tests), `source.rs` ffprobe parser (tested) + `fit_size` + `TestCard` (deterministic, honest), `subs.rs` parser (4 encodings, overlap, delay, fuzz-tested), `video.rs` surface (1 523 lines, mpv muscle memory, HUD, menus). Fence them behind protocol, fix L-items, add U-features, budget performance, package AppImage + WASM.
27: 
28: ---
29: 
30: ## 1. Architecture — Protocol-First, Backends Disposable
31: 
32: ### 1.1 Three layers
33: 
34: ```
35: ┌─────────────────────────────────────────────────────────────────────────────┐
36: │ VIEW  smart-ui-egui/src/video.rs (draw_eiga / eiga_toolbar / eiga_controls/ │
37: │       eiga_sidebar / eiga_overlay_bar / eiga_stats / eiga_help / menus)     │
38: │       egui 0.37 + eframe glow/x11/wayland. No ffmpeg, no rodio imports.   │
39: │       Talks ONLY to eiga-core::player::Player + Playlist + FrameSource     │
40: │       + MediaInfo + SubTrack + smart-protocol-rs domain-media traits.       │
41: ├─────────────────────────────────────────────────────────────────────────────┤
42: │ CORE  eiga-core (no UI/clock/window) — THE WIRE, must stay testable on     │
43: │       wasm32-unknown-unknown.                                                 │
44: │       player.rs: Player {duration,position,paused,speed,volume,muted,      │
45: │       looping,eof,fps,audio_delay,chapters,osd} + tick(dt)* + Playlist  │
46: │       + Cmd key map + SPEEDS/MAX_VOLUME.                                    │
47: │       source.rs: MediaInfo {title,container,duration,size,bitrate,        │
48: │       video[],audio[],subs[],chapters} + parse_ffprobe(JSON) +             │
49: │       FrameSource {info,seek,poll,name,exhausted} + fit_size +              │
50: │       TestCard + ffmpeg::{probe,available,video_args,audio_args,          │
51: │       sub_args,extract_sub,FfmpegSource,AudioPipe,atempo_chain}            │
52: │       subs.rs: SubTrack {name,lang,cues,delay} + Cue + decode_bytes/       │
53: │       parse/parse_ts/find_sidecars/guess_lang/active/next_start/prev_start│
54: │       lib.rs: fmt_time/parse_time (+ms).                                    │
55: ├─────────────────────────────────────────────────────────────────────────────┤
56: │ PROTOCOL  smart-protocol-rs domain-media (media.capnp) + domain-video (NEW) │
57: │       + audio.capnp (spatial/presets/focus) + files.capnp (library/recents/ │
58: │       trash) + clipboard.capnp (screenshot/path) + notification.capnp       │
59: │       (playback events) + device.capnp (MTP/phone) + capability.capnp      │
60: │       (OpenWith authorisation).                                              │
61: │       Eiga exposes Media {listPlayers,getPlayer,play,pause,stop,next,      │
62: │       previous,seek,getMetadata,getPosition,setVolume,setPosition,        │
63: │       openUri,subscribePlayer,getSupportedFormats,transcode,…playlists}    │
64: │       + Video {getVideoCaps,describeSource,getVideoFrameHandle,            │
65: │       getVideoTimeline,seekVideoFrames,setVideoScale,setVideoToneMap,…}  │
66: │       (see §8). Backends implement traits; view never calls ffmpeg.         │
67: ├─────────────────────────────────────────────────────────────────────────────┤
68: │ BACKENDS DISPOSABLE (not in protocol)                                        │
69: │       • ffmpeg-C
```

### TOOL: read [seq 142] 2026-11-09 14:04:54 status=completed

```json
{
  "limit": 400,
  "offset": 362,
  "path": "/home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md"
}
```

```
Read file /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md, lines 362-757
362: - `audio.capnp LoudnessLimit/SafetyMode/getLoudnessHistory` + `AudioEffect compressor/limiter` (exists).
363: - Why: Loud ads on evening films; replaygain tags exist in `MediaMetadata` (not probed today).
364: - Spec: `ffmpeg loudnorm` filter (two-pass when export, one-pass live), limiter `-af` chain, safety ceiling 85 dB(A) toggle (`setSafetyMode`), per-file replaygain from ffprobe `tags` → volume auto.
365: 
366: ### U-15 Audio focus, ducking, multi-app mixer, calls
367: - `audio.capnp requestFocus/releaseFocus/DuckPolicy/GlobalDuckPolicy/PerRoleDuckLevels`.
368: - Why: Eiga blasts over navigation/Meet calls; no arbitration (smartui `focus` has it, eiga `rodio` has none).
369: - Spec: Request `MediaFocus(exclusive)` on play; duck to 20 % on call (`duckOnCall`), restore on release. Volume keys route via `Audio setVolume/setMute` (so system mixer shows Eiga), not just `rodio` local gain.
370: 
371: ### U-16 Video filters & encodings: aspect/zoom done, add deinterlace, rotate metadata, frame-rate convert
372: - Probe `field_order` (interlaced), `rotation` (side-data `displaymatrix` — ffprobe `side_data_list` not parsed today).
373: - Why: Old camcorder interlaced video jitters; phone 90° rotation stored as metadata not pixels.
374: - Spec: `deinterlace` (`bwdif`) auto for interlaced; auto-rotate from `displaymatrix` (without requiring user Rotate — fixes L-05 by origin); `fps` convert for mixed timebase.
375: 
376: ### U-17 Accessibility: captions, audio description, screen-reader, colour-blind safe HUD, reduced motion
377: - `accessibility.capnp` (exists) + `MediaMetadata.lyrics` live-caption channel.
378: - Why: Forced subs + AD (`#EXT-X-MEDIA-TYPE:AUDIO … CHANNEL=GROUPS` HLS) + `reduce_motion` (already in Settings) must apply to Eiga SHAKER (OSD rise/shade spring `spring_out` ignores `reduce_motion`).
379: - Spec: Caption track = subtitle track with `kind=caption` (CC1/608 parsing via ffmpeg `subtitles` filter? `ccaption_dec`); audio description track auto-pick; HUD colours WCAG-contrast-checked on bright footage; `reduce_motion` disables springs (OSD/shade/knob).
380: 
381: ### U-18 “Smart” library: watches, progress, tags, collections, poster art, resume across devices
382: - `files.capnp getFileActivity/getRecentFiles`, `MediaMetadata.artUri/artData/contentHash/aiTags`, `Media shareMedia/getAnalytics`.
383: - Why: Today Eiga is a dumb folder player; the suite already has `Library` (tags/recents, 60 cap).
384: - Spec: `Continue Watching` shelf (resume map, U-01), poster art (`artData` from embedded MKV/MP4 cover — probe `attached_pic` correctly promoted to poster, not ignored), tags on playlists (`Library::TagDef`), sync `contentHash` so phone + desktop resume same film (content-addressed `app.capnp`).
385: 
386: ### U-19 Packaging & distribution: AppImage, Flatpak, WASM, CLI, MIME, bundle ffmpeg
387: - See §7. Missing today (L-21). Unknown unknown: `ffmpeg` LGPL static vs GPL shared lib, `ffprobe` version pin (`ffmpeg>=6.1` features: `zscale`, `libplacebo`).
388: - Spec §7 covers: AppImage with bundled `ffmpeg/ffprobe`, `.desktop` MIME, Flatpak manifest, WASM warning, `eiga` CLI (`eiga play file.mp4 --resume --sub en.srt --speed 1.5`) for scripting, D-Bus `org.smart.Eiga` Open/GetPosition (optional headless).
389: 
390: ### U-20 Offline, MTP/phone, SD card, encrypted, HDD spin-down
391: - `device.capnp`, `files.capnp`, `storage.capnp`.
392: - Why: Videos live on phones/SD/USB; Eiga cannot play them (L-07 gates to TestCard for any non-local `real_path`). HDD spin-down stalls look like hangs.
393: - Spec: MTP/ADB: stream via `adb exec-out cat file | ffmpeg -i pipe:0` (or `ffmpeg -i /proc/self/fd/N`) without mounting copy; SD eject while playing → graceful degrade to next playlist item (never kill). Encrypted `CryptMount` → `decryptFile` gate. HDD idle spin-down → `spindown` wake cost in buffering budget.
394: 
395: ### U-21 Privacy & provenance: DRM, telemetry, clipboard poisons, screenshot metadata
396: - `capability.capnp AuditEntry`, `diagnostics.capnp HealthStatus`, `crypto.capnp`.
397: - Why: Screenshot PNG carries EXIF/`Software` tags leaking username; clipboard fake “copy path” is safe, but “copy frame PNG” could leak frame title metadata; playback analytics (`MediaAnalytics`) is personal habit — must be opt-in.
398: - Spec: Screenshot strips EXIF, filename `Eiga TITLE HH-MM-SS.png` (keep) + option `Copy PNG to clipboard` (only on explicit Ctrl+Shift+C). `getAnalytics` behind opt-in + local-only (no network). `AppImage` checksum + `signature.capnp` verification hook.
399: 
400: ### U-22 Correctness under adversarial input (fuzz + adversarial suite)
401: - Already `subs.rs` fuzz; extend to `parse_ffprobe` (deeply nested JSON), `FfmpegSource` (zero-length stdout, immediate EOF, 0 fps, huge width/height → alloc OOM), `TestCard` degenerate sizes (tested: 1×1 clamped to 16 — keep).
402: - Why: Malicious `.mkv` can OOM Rust (frame `w*h*4` allocation 7680×4320×4 = 132 MB/frame, untrusted header → `vec![0u8; w*h*4]` panics/OOM). No cap today.
403: - Spec: `fit_size` max cap 4096×4096 + `try_reserve` with error; `parse_ffprobe` caps (L-23); `extract_sub` timeout 10 s; `probe` timeout 15 s; adversarial corpus in `eiga-core/tests/adversarial/` (generated ffmpeg clips: truncated, header-only, 200-track, 24 h duration, huge `bit_rate`). CI gate.
404: 
405: ---
406: 
407: ## 5. Player Reality Check — The Scorecard
408: 
409: | Area | Code | Evidence | Grade | Notes |
410: |---|---|---|---|---|
411: | `eiga-core` player state machine | `player.rs 30125` | 16 tests pass (`cargo test -p eiga-core` 39 total, incl. `speed_controls`, `ab_loop`, `chapters`, `bindings_are_unique`, `mpv_muscle_memory`) | **A** | Pure, tick 0.25 cap, EOF→restart, `clamp_pos`, `fraction`. Gaps L-01/L-02. |
412: | ffprobe parsing | `source.rs parse_ffprobe` | `parses_ffprobe_json` (cover-art excluded, fps 29.97, CHAPTERS, `forced`/`default` flags) + `rejects_bad_probe_output` | **A** | JSON parser complete. Missing `color_*`, `field_order`, `rotation`, `side_data`, tag caps (L-23). |
413: | ffmpeg decode (video) | `ffmpeg::FfmpegSource` | `end_to_end_with_real_ffmpeg` (160×90, 10 fps clip: probe→decode≥15→seek→exhaust) passes on this host | **A (1×, ≤720p)** | Real decode. Caps + synthetic PTS + no speed/HDR (L-03/L-04/L-14). |
414: | ffmpeg audio pipe | `AudioPipe` (`f32le 48 k` stale) | e2e samples ≥48000, finite + |s|≤1.5 check passes | **A− (stereo only)** | `atempo_chain` 0.25–8 correct (unit+integration). L-09/L-02 gaps. |
415: | Embedded subtitles | `extract_sub` + `subs::parse` | e2e `Hello/World` 2 cues extracted | **A** | Real extraction per-track. Slow synchronous (startup cost). |
416: | Sidecar subtitles | `find_sidecars` + `v.read(8 MB)` + `from_bytes` | `sidecars_and_languages` test | **A** | Stem matching (case-insensitive), ext gate, sorted. No download/API subs (out-of-scope). |
417: | Subtitle engines | `subs.rs 14951` | SRT/VTT/ASS, UTF-8/16/Latin-1, overlap, long-cue, delay, fuzz 300 | **A−** | ASS style loss (L-16) but never panics — fine for v1. |
418: | Playlist | `Playlist` | `playlist_sequential`, `shuffle`, `edit_keeps_cursor` | **A** | Manual next wraps even on `Repeat::Off` (documented MPV parity) — keep. |
419: | Key map | `default_bindings` + `lookup` + `apply` | `bindings_are_unique_and_documented`, `mpv_muscle_memory` | **A** | ~60 bindings, help-grouped, exact-mod match. `eiga_cmd` routes string-sourced cmds (`eiga.cmd:Play / pause` by help text — fragile, see U-xx). |
420: | OSD / stats / chapters UI | `video.rs` | `eiga_controls` seek bar + chapter ticks + A-B region + timecode, `eiga_stats` 9 rows, `eiga_help` 2-col grouped | **B+** | Visual polish high. Stats not live (L-10). |
421: | File picker / drag-drop | `rfd::FileDialog` thread + `eiga_drop` | Wired, `pending_evs` drain | **A** | Non-blocking dialog thread; drop filters video+sub (desktop only — ho
```

### TOOL: shell [seq 149] 2026-11-09 14:04:58 status=completed

```json
{
  "command": "ls /home/lufuno/smart/smart-protocol-official/schema/ 2>/dev/null; cat /home/lufuno/smart/smart-protocol-official/schema/media.capnp 2>/dev/null | head -n 120; echo ===CAPACITIES===; grep -rn \"domain-video\\|video\\.capnp\" /home/lufuno/smart/smart-protocol-official/ 2>/dev/null | head -n 20\necho ===HIGHLIGHT===; head -n 60 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs\necho ===STORAGE_HEAD===; head -n 40 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/storage.rs\necho ===SHELL_VIDEO===; sed -n '120,160p;380,430p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs"
}
```

```
abi.capnp
access.capnp
accessibility.capnp
ai.capnp
app.capnp
appindex.capnp
archive.capnp
audio.capnp
auth.capnp
backup.capnp
biometric.capnp
bluetooth.capnp
boot.capnp
buffer.capnp
calendar.capnp
camera.capnp
capability.capnp
capture.capnp
cli.capnp
clipboard.capnp
clock.capnp
compositor.capnp
console.capnp
contacts.capnp
content.capnp
crypto.capnp
database.capnp
device.capnp
diagnostics.capnp
discovery.capnp
display.capnp
dns.capnp
endpoint.capnp
extension.capnp
files.capnp
font.capnp
framebuffer.capnp
gpu.capnp
hal.capnp
ime.capnp
init.capnp
input.capnp
intent.capnp
kernel.capnp
key_management.capnp
klog.capnp
location.capnp
logging.capnp
manifest.capnp
media.capnp
module.capnp
mount.capnp
network.capnp
nfc.capnp
notification.capnp
package.capnp
power.capnp
print.capnp
process.capnp
queue.capnp
runtime_search.capnp
sandbox.capnp
screenlock.capnp
secret.capnp
sensor.capnp
service.capnp
session.capnp
settings.capnp
shell.capnp
signal.capnp
signature.capnp
socket.capnp
speech.capnp
storage.capnp
system.capnp
system_search.capnp
task_scheduler.capnp
telephony.capnp
theme.capnp
time.capnp
timezone.capnp
translation.capnp
tunnel.capnp
types.capnp
update.capnp
usb.capnp
vcard.capnp
web.capnp
windowmanager.capnp
@0xa1b2c3d4e5f60011;

using T = import "types.capnp";

# ═══════════════════════════════════════════════════════════════════════
# Media - playback control, metadata, volume, transport, playlists,
#         live-stream, chapters, lyrics, analytics, sync-aware.
#
# Improvements beyond the original schema:
#   - MediaPlayerId typed ID replaces raw UInt64.
#   - MediaItemId typed ID replaces bare Text.
#   - All positions/durations use T.Duration (nanosecond precision).
#   - DataHandle for artwork that may exceed 1KB.
#   - Subscription cancel method added.
#   - Pagination on listPlayers.
#   - Custom/other variants on enums for forward compatibility.
#   - Volume and rate documented with units and valid ranges.
#
# Audit-8 enhancements:
#   - Chapter markers for long-form content navigation.
#   - LyricsData with synchronized line/word timing.
#   - LiveStream for real-time broadcast with viewer analytics.
#   - Content-addressed media via contentHash on MediaItem.
#   - E2E encryption flag on MediaItem.
#   - AI-generated tags on MediaMetadata.
#   - Sync-aware playback via syncGroup on MediaPlayer.
#   - Playlist management (create, add, remove, reorder).
#   - Format negotiation and transcoding.
#   - Recommendations, analytics, sharing, download, crossfade.
# ═══════════════════════════════════════════════════════════════════════

interface Media {
    # ── Player discovery ──
    listPlayers @0 (pagination :T.Pagination) -> (players :List(MediaPlayer));
    getPlayer @1 (playerId :T.MediaPlayerId) -> (player :MediaPlayer);

    # ── Transport controls ──
    play @2 (playerId :T.MediaPlayerId) -> T.VoidResult;
    pause @3 (playerId :T.MediaPlayerId) -> T.VoidResult;
    stop @4 (playerId :T.MediaPlayerId) -> T.VoidResult;
    next @5 (playerId :T.MediaPlayerId) -> T.VoidResult;
    previous @6 (playerId :T.MediaPlayerId) -> T.VoidResult;
    seek @7 (playerId :T.MediaPlayerId, position :T.Duration) -> T.VoidResult;

    # ── Metadata / position ──
    getMetadata @8 (playerId :T.MediaPlayerId) -> (metadata :MediaMetadata);
    getPosition @9 (playerId :T.MediaPlayerId) -> (position :T.Duration, duration :T.Duration);

    # ── Volume / rate ──
    # volume: 0.0 (mute) to 1.0 (full). Values outside range are clamped.
    setVolume @10 (playerId :T.MediaPlayerId, volume :Float32) -> T.VoidResult;
    setPosition @11 (playerId :T.MediaPlayerId, position :T.Duration) -> T.VoidResult;

    # ── URI playback ──
    openUri @12 (playerId :T.MediaPlayerId, uri :T.Uri) -> T.VoidResult;

    # ── Subscription ──
    subscribePlayer @13 (playerId :T.MediaPlayerId) -> (subscription :PlayerSubscription);

    # ── Audit-8: Format negotiation ──
    getSupportedFormats @14 (playerId :T.MediaPlayerId) -> (formats :List(Text));

    # ── Audit-8: Transcoding ──
    transcode @15 (mediaId :T.MediaItemId, targetFormat :Text) -> (transcoded :T.DataHandle);

    # ── Audit-8: Recommendations ──
    getRecommendations @16 (playerId :T.MediaPlayerId) -> (items :List(MediaItem));

    # ── Audit-8: Analytics ──
    getAnalytics @17 (mediaId :T.MediaItemId) -> (analytics :MediaAnalytics);

    # ── Audit-8: Sharing ──
    shareMedia @18 (mediaId :T.MediaItemId, targetDeviceId :T.DeviceId) -> (shareLink :T.Uri);

    # ── Audit-8: Download ──
    downloadMedia @19 (mediaId :T.MediaItemId) -> (downloadId :UInt64);

    # ── Audit-8: Crossfade ──
    setCrossfade @20 (playerId :T.MediaPlayerId, duration :T.Duration) -> T.VoidResult;

    # ── Audit-8: Sync-aware playback ──
    setSyncGroup @21 (playerId :T.MediaPlayerId, syncGroup :Text) -> T.VoidResult;

    # ── Audit-8: Playlist management ──
    createPlaylist @22 (name :Text, items :List(MediaItem)) -> (playlistId :UInt64);
    addToPlaylist @23 (playlistId :UInt64, item :MediaItem) -> T.VoidResult;
    removeFromPlaylist @24 (playlistId :UInt64, index :UInt32) -> T.VoidResult;
    reorderPlaylist @25 (playlistId :UInt64, fromIndex :UInt32, toIndex :UInt32) -> T.VoidResult;

    # -- Audit-9: Recommendation scoring --
    getRecommendationsWithScores @26 (playerId :T.MediaPlayerId, page :T.Pagination)
        -> (recommendations :List(RecommendationScore), total :UInt32);

    # -- Audit-9: Sharing preview --
    getSharePreview @27 (mediaId :T.MediaItemId) -> (preview :SharePreview);

    # -- Audit-9: Download status --
    getDownloadStatus @28 (downloadId :UInt64) -> (status :DownloadStatus);
    listDownloads @29 (pagination :T.Pagination) -> (statuses :List(DownloadStatus), total :UInt32);
    pauseDownload @30 (downloadId :UInt64) -> T.VoidResult;
    resumeDownload @31 (downloadId :UInt64) -> T.VoidResult;
    cancelDownload @32 (downloadId :UInt64) -> T.VoidResult;
    subscribeDownload @33 (downloadId :UInt64) -> (subscription :DownloadSubscription);

    # -- Audit-9: Crossfade details --
    setCrossfadeConfig @34 (playerId :T.MediaPlayerId, config :CrossfadeConfig) -> T.VoidResult;
    getCrossfadeConfig @35 (playerId :T.MediaPlayerId) -> (config :CrossfadeConfig);

    # -- Audit-9: Live stream interaction --
    sendLiveInteraction @36 (streamUrl :T.Uri, interaction :LiveInteraction) -> T.VoidResult;
    subscribeLiveInteractions @37 (streamUrl :T.Uri) -> (subscription :LiveInteractionSubscription);

    # -- Audit-9: Transcoding progress --
    subscribeTranscode @38 (transcodeId :UInt64) -> (subscription :TranscodeSubscription);
}

interface PlayerSubscription {
    pull @0 () -> (events :List(PlayerEvent));
    cancel @1 () -> T.VoidResult;
}
===CAPACITIES===
===HIGHLIGHT===
//! Syntax highlighting with tree-sitter (Rust, Python, JavaScript, JSON, TOML, Bash, CSS, HTML, C, Go, Markdown) and a
//! small lexer for everything else, so *every* text file gets colour and nothing ever fails to preview.
use egui::{text::LayoutJob, Color32, FontId, TextFormat};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cls {
    Plain,
    Keyword,
    Function,
    Type,
    Str,
    Number,
    Comment,
    Punct,
    Property,
    Tag,
    Title,
    Link,
    Escape,
}

const NAMES: [&str; 37] = [
    "attribute", "comment", "constant", "constant.builtin", "constructor", "embedded", "escape", "function", "function.builtin", "function.method", "keyword", "number", "operator", "property", "punctuation", "punctuation.bracket", "punctuation.delimiter", "punctuation.special", "string", "string.special", "string.escape", "tag", "type", "type.builtin", "variable", "variable.builtin", "variable.parameter", "text.title", "text.literal", "text.uri", "text.reference", "text.emphasis", "text.strong", "label", "module", "boolean", "character",
];

fn cls_of(i: usize) -> Cls {
    match NAMES[i] {
        "keyword" | "boolean" | "variable.builtin" | "label" => Cls::Keyword,
        "function" | "function.builtin" 
```

### TOOL: shell [seq 156] 2026-11-09 14:05:02 status=completed

```json
{
  "command": "grep -n \"caret\\|TextEdit\\|cursor\\|spring_out\\|spring_to\\|Blink\\|breath\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/*.rs | head -n 80\necho ===CHROME_HEAD===; sed -n '1,70p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs\necho ===MODALS_HEAD===; sed -n '1,80p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs\necho ===SETTINGS_DRAW===; grep -n \"draw_app_settings\\|fn modal_head\\|fn foot_btn\\|fn icon_button\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs | head -n 40"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs:422:            s.visuals.text_cursor.stroke = egui::Stroke::new(1.5, hex("#2f6feb"));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:137:            let vis = spring_to(ctx, ("vsv", key), vis, 200.0, 24.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:156:        let t = spring_to(ui.ctx(), ("swk", id), if on { 1.0 } else { 0.0 }, 420.0, 26.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:183:        let shown = spring_to(ui.ctx(), ("slk", id), nv, 420.0, 32.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:205:        let x = spring_to(ctx, ("seg", id), sel as f32, 330.0, 21.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:215:        let press = spring_to(ctx, ("segp", id), if any_down { 1.0 } else { 0.0 }, 520.0, 30.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:227:            let t = spring_to(ctx, ("segt", id, i), if on { 1.0 } else { 0.0 }, 300.0, 24.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:264:        let pop = if self.lib(|l| l.settings.reduce_motion) { 1.0 } else { spring_out(age) };
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:315:        let pop = spring_to(&ctx, ("setpop", app), 1.0, 260.0, 24.0).clamp(0.0, 1.2);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:342:        let yy = spring_to(&ctx, ("aset-rail", app), page as f32, 330.0, 23.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs:464:                ui.ctx().set_cursor_icon(*cur);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:294:        let tab_h = TAB_H * spring_to(ui.ctx(), "tabstrip", if self.tabs.len() > 1 { 1.0 } else { 0.0 }, 220.0, 24.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:637:                    let t = spring_to(ui.ctx(), ("tree-rot", &loc), if open { 1.0 } else { 0.0 }, 320.0, 24.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:821:        let shown = spring_to(ui.ctx(), "storage-card-fill", used, 140.0, 20.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:1209:        let f = spring_to(ui.ctx(), "search-focus", if focused { 1.0 } else { 0.0 }, 240.0, 22.0).clamp(0.0, 1.15);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs:1229:        let out = ui.put(te, egui::TextEdit::singleline(&mut q).id(id).frame(egui::Frame::NONE).hint_text(egui::RichText::new(hint).color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:364:            let out = ui.put(rr.shrink2(vec2(4.0, 0.0)), egui::TextEdit::singleline(&mut text).id(id).frame(egui::Frame::NONE).font(font).text_color(self.th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:367:                if let Some(mut st) = egui::TextEdit::load_state(ui.ctx(), id) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:369:                    st.cursor.set_char_range(Some(egui::text::CCursorRange::two(egui::text::CCursor::new(0), egui::text::CCursor::new(stem))));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs:430:        let pop = spring_to(ui.ctx(), ("tick", center.x as i32, center.y as i32, checked), if checked { 1.0 } else { 0.0 }, 260.0, 16.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/copyfx.rs:81:        let sf = spring_to(ctx, "cp-frac", frac, 90.0, 16.0).clamp(0.0, 1.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/copyfx.rs:82:        let pop = spring_out(anim);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:58:                s.visuals.text_cursor.stroke = egui::Stroke::new(1.5, self.th.accent);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs:171:            ctx.set_cursor_icon(match d {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:88:    fn foot_btn(&self, ui: &Ui, card: Rect, cursor: &mut f32, label: &str, kind: Btn, id: &str) -> bool {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:90:        let r = Rect::from_min_size(pos2(*cursor - w, card.bottom() - 32.0 - 17.0), vec2(w, 34.0));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:91:        *cursor -= w + 8.0;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:184:            let pop = spring_out(anim_t);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:791:    fn foot_btn_at(&self, ui: &Ui, cursor: f32, y: f32, label: &str, id: &str) -> bool {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:793:        let r = Rect::from_min_size(pos2(cursor - w, y), vec2(w, 36.0));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:806:        let r = ui.put(f.shrink2(vec2(16.0, 6.0)), egui::TextEdit::singleline(&mut t).id(Id::unique("newtag")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("Tag name").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:831:        let r = ui.put(Rect::from_min_max(f.min + vec2(40.0, 6.0), f.max - vec2(16.0, 6.0)), egui::TextEdit::singleline(&mut t).id(Id::unique("connect")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("smb://nas.local/share").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:1451:                let te = ui.put(fld.shrink2(vec2(12.0, 4.0)), egui::TextEdit::singleline(&mut pw).id(Id::unique("arc-pw-unlock")).frame(egui::Frame::NONE).password(true).hint_text(egui::RichText::new("Archive password").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:1470:            let te = ui.put(fld.shrink2(vec2(12.0, 4.0)), egui::TextEdit::singleline(&mut name).id(Id::unique("arc-name")).frame(egui::Frame::NONE).hint_text(egui::RichText::new("Archive.zip").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:1492:            let te = ui.put(Rect::from_min_max(fld.min + vec2(36.0, 4.0), fld.max - vec2(12.0, 4.0)), egui::TextEdit::singleline(&mut pw).id(Id::unique("arc-pw")).frame(egui::Frame::NONE).password(true).hint_text(egui::RichText::new("Leave empty for no password").color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs:491:        let
```

### TOOL: shell [seq 163] 2026-11-09 14:05:05 status=completed

```json
{
  "command": "sed -n '200,260p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/paint.rs\necho ===TERMINAL_BLINK===; grep -n \"blink\\|hypnot\\|caret\\|breathe\\|breath\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/terminal.rs | head -n 40\necho ===TERMLIVE===; grep -n \"caret\\|blink\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/termlive.rs | head -n 20\necho ===SEARCH_FOCUS===; sed -n '1195,1260p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs\necho ===AI_INPUT===; sed -n '1215,1260p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs\necho ===AUDIO_CORE===; cat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/audio.rs | head -n 80"
}
```

```
    rim(p, rect, r, 3.0, alpha(t.shadow, if t.dark { 0.55 } else { 0.10 }), true, 14.0);
    rim(p, rect, r, 1.0, alpha(Color32::WHITE, if t.dark { 0.06 } else { 0.8 }), false, 4.0);
}

/// Elevated glass surface for menus / popovers. Brighter than the window under it (so the bottom never melts into
/// the background), with a long top light, a lit lower lip and a full-perimeter hairline.
pub fn menu_surface(p: &Painter, t: &Theme, rect: Rect, r: f32) {
    elevate(p, t, rect, r, 4);
    // contact shadow tight to the edge keeps the silhouette readable on dark and light backdrops
    shadow(p, rect, r, 1.0, 3.0, 0.0, alpha(t.shadow, if t.dark { 0.55 } else { 0.14 }));
    let (top, bot) = if t.dark { (c(50, 61, 84, 0.99), c(37, 47, 66, 0.99)) } else { (c(255, 255, 255, 1.0), c(243, 247, 254, 1.0)) };
    grad_fill(p, rect, r, top, bot);
    // light that travels most of the way down the sides instead of dying after a few pixels
    rim(p, rect, r, 1.6, alpha(Color32::WHITE, if t.dark { 0.34 } else { 1.0 }), true, rect.height() * 0.9);
    // lit lower lip: glass catches a little light on its underside too
    rim(p, rect, r, 1.0, alpha(Color32::WHITE, if t.dark { 0.14 } else { 0.7 }), false, 22.0);
    p.rect_stroke(rect.shrink(0.5), cr(r), egui::Stroke::new(1.0, if t.dark { c(255, 255, 255, 0.10) } else { c(16, 26, 44, 0.08) }), egui::StrokeKind::Inside);
}

/// Closed-form damped spring step response (0 → 1 with a little overshoot). `t` in seconds.
pub fn spring_out(t: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    let (w, z) = (26.0f32, 0.62f32);
    let wd = w * (1.0 - z * z).sqrt();
    1.0 - (-z * w * t).exp() * ((wd * t).cos() + z / (1.0 - z * z).sqrt() * (wd * t).sin())
}

/// Stateful spring per `id`, stored in egui memory. Returns the current value; keeps repainting until at rest.
pub fn spring_to(ctx: &egui::Context, id: impl std::hash::Hash + std::fmt::Debug, target: f32, stiffness: f32, damping: f32) -> f32 {
    let key = egui::Id::new(("spring", id));
    let dt = ctx.input(|i| i.stable_dt).min(0.05);
    let (mut x, mut v) = ctx.data_mut(|d| d.get_temp::<(f32, f32)>(key)).unwrap_or((target, 0.0));
    // semi-implicit Euler, sub-stepped for stability
    let n = 3;
    for _ in 0..n {
        let h = dt / n as f32;
        v += (-stiffness * (x - target) - damping * v) * h;
        x += v * h;
    }
    if (x - target).abs() < 0.0006 && v.abs() < 0.004 {
        x = target;
        v = 0.0;
    } else {
        ctx.request_repaint();
    }
    ctx.data_mut(|d| d.insert_temp(key, (x, v)));
    x
}
===TERMINAL_BLINK===
100:    pub blink: bool,
151:        Self { sessions: vec![], active: 0, history: vec![], font: 13.0, home, next_id: 1, booted: false, pending_run: None, cursor: 0, blink: true, ghost: true, timing: true, cards: true, scrollback: 80, spacing: 1.55, ctx_block: None }
1402:        let (ghost_on, cards, cursor_style, blink_on) = (self.shell.term.ghost, self.shell.term.cards, self.shell.term.cursor, self.shell.term.blink);
1711:            // cursor (spring-smoothed, blinking)
1717:            let blink = if active { if blink_on { ((time * 2.0).sin() > -0.35) as u8 as f32 } else { 1.0 } } else { 0.0 };
1724:                p.rect_filled(cur, cr(2.0), alpha(th.accent, 0.85 * blink));
1726:                    p.text(pos2(tx, ty + lh / 2.0), Align2::LEFT_CENTER, ch.to_string(), mono.clone(), alpha(tc("#08101e", "#ffffff"), blink));
2367:                    SRow::Choice { icon: "code-square", label: "Cursor shape", sub: "How the caret looks in the prompt", opts: &["Block", "Bar", "Line"], sel: t.cursor },
2368:                    SRow::Toggle { icon: "bolt", label: "Blinking cursor", sub: "Pulse the caret while typing", on: t.blink },
2389:                (0, 3, SVal::Bool(b)) => t.blink = b,
===TERMLIVE===
176:        let blink_on = (time * 1.8).fract() < 0.6 || !self.shell.term.blink;
240:                if active && blink_on {
===SEARCH_FOCUS===
                }
            }
        }
        let plus = Rect::from_center_size(pos2((inner.left() + tw * n as f32 + 18.0).min(strip.right() - PAD_X - 4.0), inner.center().y), vec2(30.0, 30.0));
        if self.icon_button(ui, plus, "add", "tab-new", Btn::Quiet, true).on_hover_text("New tab  Ctrl+T").clicked() {
            self.push_act(Act::NewTab(None));
        }
    }

    fn draw_search(&mut self, ui: &mut Ui, r: Rect) {
        let p = ui.painter().clone();
        let th = self.th.clone();
        let focused = self.search.focused || self.search.sheet;
        // focus is spring-driven and keeps the recessed depth: the well stays inset, light only tints it
        let f = spring_to(ui.ctx(), "search-focus", if focused { 1.0 } else { 0.0 }, 240.0, 22.0).clamp(0.0, 1.15);
        if f > 0.01 {
            shadow(&p, r, 17.0, 3.0, 16.0, 0.0, alpha(th.accent, 0.22 * f));
        }
        well(&p, &th, r, 17.0);
        if f > 0.01 {
            grad_fill(&p, r, 17.0, alpha(th.accent, 0.12 * f), alpha(th.accent, 0.035 * f));
            rim(&p, r, 17.0, 3.0, alpha(th.shadow, (if th.dark { 0.45 } else { 0.10 }) * f), true, 14.0);
            rim(&p, r, 17.0, 1.3, alpha(th.accent, 0.75 * f), true, r.height());
            rim(&p, r, 17.0, 1.0, alpha(Color32::WHITE, (if th.dark { 0.16 } else { 0.9 }) * f), false, 6.0);
            p.rect_stroke(r.expand(1.5), cr(18.5), egui::Stroke::new(1.5, alpha(th.accent, 0.38 * f)), egui::StrokeKind::Outside);
        }
        self.ico(&p, "magnifer", if focused { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 20.0, r.center().y), 16.0, if focused { th.accent_ink } else { th.ink3 });
        let active = self.search_active();
        let right_pad = if active { 34.0 } else { 46.0 };
        let te = Rect::from_min_max(pos2(r.left() + 36.0, r.top() + 5.0), pos2(r.right() - right_pad, r.bottom() - 5.0));
        let hint = format!("Search {}", self.title());
        let mut q = std::mem::take(&mut self.search.query);
        let id = Id::unique("search-input");
        let before = q.clone();
        let out = ui.put(te, egui::TextEdit::singleline(&mut q).id(id).frame(egui::Frame::NONE).hint_text(egui::RichText::new(hint).color(th.ink4)).font(f_medium(T_BODY)).text_color(th.ink1).vertical_align(egui::Align::Center).margin(egui::Margin::ZERO));
        if self.search.focus_next {
            out.request_focus();
            self.search.focus_next = false;
        }
        self.search.focused = out.has_focus();
        self.search.query = q;
        if before != self.search.query {
            self.search.dirty_at = self.time;
            if !self.search.query.trim().is_empty() {
                self.acts.push(Act::RunSearch(self.search.query.clone()));
            }
        }
        if out.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.clear_search();
            out.surrender_focus();
        }
        if active {
            let b = Rect::from_center_size(pos2(r.right() - 18.0, r.center().y), vec2(24.0, 24.0));
            let resp = self.hit(ui, b, "search-clear", Sense::click());
            self.ico(&p, "close-circle", if resp.hovered() { Ic::Filled } else { Ic::Duo }, b.center(), 16.0, th.ink3);
            if resp.clicked() {
                self.push_act(Act::ClearSearch);
            }
        } else {
            let kb = Rect::from_min_max(pos2(r.right() - 48.0, r.top()), r.right_bottom());
            self.keycap(&p, pos2(r.right() - 11.0, r.center().y), "\u{2318}K", false);
            if self.hit(ui, kb, "search-k", Sense::click()).clicked() {
                self.push_act(Act::Palette);
            }
        }
    }
===AI_INPUT===
            self.ai.scroll = self.ai.scroll.clamp(0.0, max);
            if max > 0.5 {
                let track = Rect::from_min_size(pos2(body.right() - 3.0, body.top() + 3.0), vec2(3.0, body.height() - 6.0));
                p.rect_filled(track, cr(1.5), alpha(th.shadow, if th.dark { 0.25 } else { 0.05 }));
                let h = (track.height() 
```

### TOOL: websearch [seq 170] 2026-11-09 14:05:09 status=completed

```json
{
  "query": "Rust video decoder crates 2026 symphonia rust-decoder ffmpeg hwaccel VAAPI NVDEC demux"
}
```

```
## [The state of media processing in Rust (2026): what each crate actually covers - DEV Community](https://dev.to/yeauty/the-state-of-media-processing-in-rust-2026-what-each-crate-actually-covers-2k2c)

| symphonia + rubato | in-process (pure Rust) | demux + decode audio, resample to 16 kHz; symphonium does it in one call; mutter feeds whisper-rs in five lines | HE-AAC and Opus listed unfinished; no AC-3; video out of scope | symphonia ~1.07M downloads/month, 0.6.0 (May 2026); whisper-rs ~170K/month (maintained on Codeberg) |
| ac-ffmpeg | in-process | touches all three domains (video, audio, encoding) | stays at the packet-loop level; no release in 14 months | — | [...] The honest limitation. "Any video, any codec, one line" hits a codec wall: symphonia still lists HE-AAC and Opus as unfinished and has no AC-3 at all — which is exactly where an FFmpeg backend remains irreplaceable. And video is out of scope by design.

Verdict: inside its codec table, the cleanest dependency story on this list; outside it, you are back to FFmpeg.

## Path 6: ez-ffmpeg — the in-process runtime [...] `seek_to_frame`

The honest limitation. The limitations are written in its own README: "still a work-in-progress… some parts not flushed out", "Use with caution". There is no audio API. The official successor, rave, aims to move off FFmpeg entirely — and is "not ready for use yet".

Verdict: strong on video, and video only; no audio plus self-declared WIP keeps it a specialist.

## Path 5: symphonia + rubato — pure Rust until the codec wall

## [Video — list of Rust libraries/crates // Lib.rs](https://lib.rs/multimedia/video)

Wrap a standalone FFmpeg binary in an intuitive Iterator interface

  v2.7.0 227K  bin+lib  #ffmpeg #video
 #### pipewire

  Rust bindings for PipeWire

  v0.10.1 818K   #video #audio #api-bindings
 #### shiguredo\_mp4

  MP4 library

  v2026.6.0-canary.2 15K  no-std  #mp4 #shiguredo #fmp4 #demux #生成
 #### h264-reader

  Reader for H264 bitstream syntax

  v0.9.0 496K   #h264 #bitstream
 #### av1-grain

  Helpers for generating and parsing AV1 film grain data [...] v0.1.35  app  #youtube-music #tui #terminal #youtube
 #### symphonia-wem

  Symphonia demuxer for Wwise Encoded Media files

  v0.1.0   #audio-codec #audio #decoder #multimedia-encoding #multimedia
 #### livekit-webrtc

  Livekit safe bindings to libwebrtc

  v0.2.0   #live-kit #web-rtc #sdk #video-frame #libwebrtc #cross-platform #real-time #video-codec #server-sdk #safe-bindings
 #### wedeo-rav1e

  rav1e AV1 encoder adapter for wedeo [...] v0.1.0   #video-codec #bitstream #hardware-acceleration #turbo-metrics #npp #ssimulacra2 #amf #driver #nvdec #gpu
 #### preach

  Platform independent data channels for WebRTC/Rust

  v0.2.1   #web-rtc #platform-independent #data-channel #signaling-server #remote-peer #ice #candidate #negotiation #exchanging
 #### ac-ffmpeg-build

  Helper crate for FFmpeg lookup during ac-ffmpeg build

  v0.1.0 550   #ffmpeg #build #ac-ffmpeg
 #### media-seek

## [ff_decode - Rust](https://docs.rs/ff-decode)

### §Features

 Video Decoding: Frame-by-frame decoding with Iterator pattern
 Audio Decoding: Sample-level audio extraction
 Seeking: Fast keyframe and exact seeking without file re-open
 Thumbnails: Efficient thumbnail generation for timelines
 Hardware Acceleration: Optional NVDEC, QSV, AMF, `VideoToolbox`, VAAPI support
 Frame Pooling: Memory reuse for reduced allocation overhead

### §Usage

#### §Video Decoding

ⓘ [...] ## Crate ff\_decode

# Crate ff\_decode

Source

Expand description

## §ff-decode

Video and audio decoding - the Rust way.

This crate provides frame-by-frame video/audio decoding, efficient seeking, and thumbnail generation. It completely hides `FFmpeg` internals and provides a safe, ergonomic Rust API.

### §Features [...] IoSource
:   A byte source FFmpeg can demux from.

MediaError
:   Shared classification for the `ff-` crate error types.

## [Investigate feasibility of replacing ffmpeg with a Rust equivalent for demuxing and decoding. [40074653]](https://issues.chromium.org/issues/40074653)

[ one such pure rust library that seems to offer all of what we need, though it's support for some containers/codecs isn't fully functional yet. It also provides support for opus so we could replace libopus.  
  
Ideally we'd be able to replace some/all of our demuxing code in media/formats as well, but that may prove trickier since they are tightly integrated with the needs of Media Source Extensions. [...] [ID: 1223136]

he CWE ID for the type of security defect the current issue is describing. [ID: 1410892]

Design doc to be reviewed. [ID: 1223032]

[ID: 1223131]

How many engineer days the task is estimated to take. [ID: 1225337]

[ID: 1223081]

[ID: 1223087]

[ID: 1223134]

Milestone(s) impacted by this issue. [ID: 1223085]

Date of next expected progress update or deadline for providing requested information. [ID: 1225154]

[ID: 1223083]

[ID: 1223084]

[ID: 1223086]

[ID: 1223034] [...] Remove item

Show all 11 items in the list

Show all 6 items in the list

Pending code changes (auto-populated)

Tracks the current rank of this item in the teams backlog [ID: 1225362]

Build number, such as 117.0.5911.2 [ID: 1223033]

[ID: 1223031]

Build&gt;Rust

Internals&gt;Media&gt;Codecs

Internals&gt;Media&gt;Source

Select items in the list

Supplemental component tags only. Set main component first. [ID: 1222907]

[ID: 1223136]

## [HWAccelIntro – FFmpeg](https://trac.ffmpeg.org/wiki/HWAccelIntro)

Several decoders are currently supported through VDPAU in libavcodec, in particular H.264, MPEG-1/2/4, and VC-1, AV1.

# VAAPI¶

Video Acceleration API (VAAPI) is a non-proprietary and royalty-free open source software library ("libva") and API specification, initially developed by Intel but can be used in combination with other devices.

It can be used to access the Quick Sync hardware in Intel GPUs and the UVD/VCE hardware in AMD GPUs. See VAAPI.

# DXVA2¶ [...] Note: FFmpeg uses its own slightly modified runtime-loader for NVIDIA's CUDA/NVENC/NVDEC-related libraries. If you get an error from `configure` complaining about missing `ffnvcodec`, ​this project is what you need. It has a working Makefile with an install target: `make install PREFIX=/usr`. FFmpeg will look for its pkg-config file, called `ffnvcodec.pc`. Make sure it is in your `PKG_CONFIG_PATH`.

This means that running the following before compiling ffmpeg should suffice: [...] | Media Foundation | N | N | N | Y | Y | Y | N | N | N | N |
| MMAL | N | N | N | N | N | N | N | N | N | Y |
| OpenCL | Y | Y | Y | Y | Y | Y | _P_ | Y | N | N |
| OpenMAX | _P_ | N | N | N | N | N | _P_ | N | N | Y |
| V4L2 M2M | N | N | N | N | N | N | _P_ | N | N | N |
| VAAPI | _P_ | Y | _P_ | N | N | N | N | N | N | N |
| VDPAU | _P_ | N | Y | N | N | N | N | N | N | N |
| VideoToolbox | N | N | N | N | N | N | N | Y | Y | N |
| Vulkan | Y | Y | Y | Y | Y | Y | N | N | N | N |

## [GitHub - dromara/rsmedia: audio/video toolkit based FFmpeg 6.x, 7.x supported for multimedia with Hardware Acceleration. · GitHub](https://github.com/dromara/rsmedia)

| Feature | What it does | Main entry points |
 --- 
| Decoding | Turns video / audio / subtitle streams into Rust data: ndarray-backed frames (`MediaFrame`), raw `AVFrame`s, or subtitle segments. Output pixel/sample format can be unified per builder. | `DecoderBuilder` → `Decoder::decode::`, `decode_raw`, `decode_subtitle_segment` | [...] ## 🎬 Introduction

`rsmedia` is a general-purpose video/audio media library for Rust that uses the `libav`-family libraries from `ffmpeg`. It aims to provide a stable and Rusty interface to many common media tasks, such as reading, writing, muxing, encoding, decoding, filtering, scaling, resampling, subtitle handling and image processing. [...] ## 🪲 Debugging

FFmpeg does not always produce useful error messages directly. Turn on `tracing` to see the extra information it logs:

```
dependencies tracing"0.1" " "tracing-subscriber"
```

### TOOL: websearch [seq 170] 2026-11-09 14:05:09 status=completed

```json
{
  "query": "mpv vs VLC vs Shotcut features HDR10 HDR10+ spatial audio casting Picture-in-Picture 2025"
}
```

```
## [MPV vs VLC comparison - or Try Elmedia video player](https://www.elmedia-video-player.com/blog/mpv-vs-vlc-vs-elmedia)

Skip to content

Electronic Team, Inc. Logo ELMEDIA

 Blog
 Compare
 VLC vs. MPV vs. Elmedia, Which one is the best?

# VLC vs. MPV vs. Elmedia, Which one is the best?

avatar

Olga Weis

Apr 23, 2025

thumbnail [...] ## MPV

MPV is a free, cross-platform, open-source media player. One of the best things about this app is that it supports a wide range of file formats and codecs, so you should be able to play almost any file using the player in up to 8K resolution. Because MPV is in active development, new features are constantly being added, and you can even request a feature if there’s something you’d like that’s missing. [...] VLC Media Player is a free, open-source, cross-platform media player that plays almost all video and audio formats without the need for any additional codecs. Easy to set up and use, this free player is packed with advanced settings, and you can use keyboard shortcuts for most actions. Features include DVD and Blu-ray playback, deinterlacing for better video quality, support for resolutions up to 8K, customizable playback options, and more. You can even stream music and video to your smart TV

## [VLC vs MPV: Which One is Better? - Sysprobs](https://sysprobs.com/vlc-vs-mpv-which-one-is-better)

MPV also makes use of GPU decoding. It can make use of most hardware decoding APIs on different platforms. You can enable hardware decoding at runtime. MPV’s video output is based on OpenGL, Vulkan, and D3D11. So, you can take advantage of many advanced features such as frame timing, color management, HDR, interpolation, and video scaling.

If we dive deeper and search for user experiences, we can learn more about each media player in this aspect. [...] Some users prefer the default of MPV to VLC in terms of quality. And that is because you can configure MPV for better output. Some users reported that VLC could barely play high-resolution 4K HDR videos, whereas MPV played them. Some videos can become choppy in VLC but have a smooth playback on MPV. [...] MPV takes customization to a whole different level. You can change almost anything you can think of. From profiles, general settings, on-screen display, screenshots, video and audio settings, playbacks, and subtitles to scaling, deband, static grain, dither, deinterlace, shader, MPV allows them all.

## [VLC and MPV: The Two Best Media Players of All Time](https://www.youtube.com/watch?v=R5BnukMZF-A)

# VLC and MPV: The Two Best Media Players of All Time
## Channel: Renegade Bit
34.9K subscribers
294 likes

### Description
11,604 views
Posted: 2025-12-09
A quick but honest comparison between VLC and MPV. I break down strengths, weaknesses and who should use which player, from casual users to power nerds. [...] [4:54] if you care about every detail of the image, MPV will give you that extra edge. Now, let's talk about features.
[5:00] And here, VC shows why it's considered the Swiss Army knife of media players.
[5:05] VC isn't just a video player, it's much more. You can stream IPv content, capture the desktop, record from webcam.
[5:13] It has an integrated transcoder that lets you convert videos from one format to another directly from the interface. Playlists are advanced and well-managed. [...] [4:26] more detailed results compared to VC's standard scaling. MPV also supports motion interpolation, that technology that interpolates frames to create
[4:35] smoother movements. Additionally, color management is more precise, dithering and demanding are better, and HDR support is more mature and configurable.
[4:45] VC isn't bad for video quality to be clear. For standard viewing without particular tweaking, it does its job admirably, but if you're a video file,

## [I ditched VLC and finally got high-quality HDR playback without any lag](https://www.makeuseof.com/ditched-vlc-for-mpv-hdr-playback)

mpv isn't flawless, no media player is. There's no library to organize your films, the install is fussier than double-clicking an installer, and that bare config-file approach means you'll spend an evening reading docs before it feels like yours. VLC still wins for DVDs with menus and the odd broken file you just need to limp through. [...] But for the one thing I bought a 4K HDR monitor to do, watching HDR movies the way they're meant to look, mpv has been the clear winner. With three lines of config, I got accurate colors and smooth playback that hours of fighting VLC never did. The fans still spin. I've made my peace with that. [...] What's more, point it at a folder, and it builds a playlist on the spot, so I can jump to the next file with a single key and have it loop back at the end. Compared with VLC's busy, slightly dated window, mpv stays out of the way and puts every spare cycle into the actual video, which is exactly what you want when you are watching 4K HDR videos. It's the leaner player that a lot of longtime VLC users have quietly switched to.

mpv logo

##### MPV Player

OS
:   Windows, Android, macOS, Linux

## [I finally ditched VLC for its faster open-source rival](https://www.howtogeek.com/i-finally-ditched-vlc-for-its-faster-open-source-rival)

VLC isn't a bad player; it has earned its reputation over two decades. However, it carries a ton of stuff that I don't really use, and that weight sometimes shows. mpv took about 15 minutes to set up, but now my HDR files aren't horribly washed out. [...] Some of the performance jump comes from having less software between the file and the screen. VLC carries a massive amount of legacy overhead, but mpv is noticeably smaller. It uses GPU rendering through OpenGL, Vulkan, or Direct3D. Using the gpu-next output driver allows it to handle modern workloads without anything else in the way. [...] Mpv is a free and open-source player that is derived from MPlayer and mplayer2. It has been in active development since 2012 and takes a fundamentally different approach to the user interface. VLC tries to be everything to everyone (and largely succeeds), whereas mpv focuses on being as minimalist as possible.

## [mpv vs VLC — Which is Better in 2026? — Bundl.run](https://bundl.run/compare/mpv-vs-vlc)

mpv can handle many disc-related workflows depending on build features, but VLC remains the more turnkey optical-media tool for casual users. Optical media edge cases remain a VLC strength even when file playback goes to mpv. Discs and odd capture paths still justify keeping VLC.

Which uses less battery on a MacBook? [...] mpv vs VLC: mpv takes the overall win for Bundl’s quality-conscious Mac audience in August 2026, powered by the 0.41 gpu-next era and a still-unmatched configuration culture. VLC remains mandatory kit for compatibility and non-technical users, and it is not ‘losing’ so much as optimising for a different objective function. If you want one player and you like keyboards, choose mpv (or IINA). If you support other humans, keep VLC around forever. In short: mpv for the Mac you personally tune; VLC [...] ## Feature Comparison

Feature comparison between mpv and VLC

| Feature | mpv | VLC |
| Price | Free | Free & open source |
| Open Source | No | Yes |
| Replaces | Elmedia Player Pro ($20) | Elmedia Player PRO ($20) |
| Best for | video player, open source, media | video, player, media |
| Install | brew install --cask stolendata-mpv | brew install --cask vlc |
| Category | Media & Entertainment | Media & Entertainment |

## Quick Install

`brew install --cask stolendata-mpv`

## [mpv.io](https://mpv.io/manual/stable)

only)</dd> <dt>bt.2446a</dt> <dd>HDR<->SDR mapping specified in ITU-R Report BT.2446, method A. This is the recommended curve for well-mastered content. (<tt><span>--vo=gpu-next</span></tt> only)</dd> <dt>st2094-40</dt> <dd>Dynamic HDR10+ tone-mapping method specified in SMPTE ST2094-40 Annex B. In the absence of metadata, falls back to a fixed spline matched to the input/output average brightn
```

### TOOL: websearch [seq 170] 2026-11-09 14:05:09 status=completed

```json
{
  "query": "Rust crates HDR tone mapping spatial audio subtitle ASS SRT tree-sitter highlight egui 2025 2026"
}
```

```
## [Video — list of Rust libraries/crates // Lib.rs](https://lib.rs/multimedia/video)

v0.1.2  bin+lib  #ffmpeg #scene-detection #aegisub #video
 #### h264-profile-level-id

  process H264 profile-level-id values

  v0.2.0 141K   #h264 #sdp #web-rtc
 #### vpx-rs

  interface to Google's libvpx library

  v0.2.1   #video-codec #vp9 #vp8 #libvpx #video
 #### oximedia-hdr

  HDR video processing: tone mapping, transfer functions, and dynamic metadata for OxiMedia

  v0.2.1   #tone-mapping #hdr #pq #hlg #rec2020
 #### ytdlp-tui

  Interactive terminal UI for yt-dlp downloads [...] CLI tool to help manage your Plex themes

  v0.1.3  app  #themes #plex #manager #command-line-tool #theme-manager #video #tv
 #### rivet-codec

  GPU video decode/encode dispatch (NVDEC/NVENC, AMF, QSV) plus colorspace, tonemap, audio, and probe for the rivet transcoder. Imported as `codec`.

  v0.2.0   #hdr #video-encoder #amf #nvenc #rivet #nvdec #transcoder #qsv #gpu #audio
 #### aravis-utils

  command line utilities for interacting with GenICam cameras [...] v3.1.2   #audio #visualization #sound #audio-sound
 #### qiyi

  A publish online video edit tools

  v0.1.0  bin+lib  #publish #edit #online #video
 #### d-id

  interacting with D-ID's API

  v0.1.0   #artificial-intelligence #text-to-video #ai
 #### golem-ai-stt-deepgram

  working with Deepgram Speech to Text API on Golem Cloud

  v0.5.5   #cloud-api #golem #deepgram #provider #speech #web-search #video
 #### egui-scrubber

  scrubber for egui for use in video or replay scrubbing

## [tree-sitter - crates.io: Rust Package Registry](https://crates.io/crates/tree-sitter)

Rust bindings to the Tree-sitter parsing library. This feature allows tree-sitter to be built for Wasm targets using the wasmtime-c-api crate.

## [A 2025 Survey of Rust GUI Libraries | Lobsters](https://lobste.rs/s/ae3r4j/2025_survey_rust_gui_libraries)

Otherwise egui it's an interesting choice, maybe easier for simple GUIs. I was left with a bitter taste with this test of qmetaobject, as it's one of the best Qt bindings for Rust. But I understand that setting up all of that in Windows is not easy.

      1. 7

          pie\_flavor edited [...] egui in this list is an equivalent of sorts but native to rust, and so it's unsurpising it does well. imgui fails though their accessability which again is unsuprising since it is targeted at the games/graphics industry which is not very accessible for obvious reasons.
   2. 8

       adam\_d\_ruppe [...] a bit off script, 2025 got me daydreaming that i'm anywhere else >u<

## [tree_sitter - Rust](https://docs.rs/tree-sitter)

## Crate tree\_sitter

# Crate tree\_sitter

Source

Expand description

## §Rust Tree-sitter

Rust bindings to the Tree-sitter parsing library.

### §Basic Usage

First, create a parser:

```
use let mut 
```

Then, add a language as a dependency:

```
[dependencies] tree-sitter = "0.24" tree-sitter-rust = "0.23"
```

To use a language, you assign them to the parser.

```
& "Error loading Rust grammar"
```

Now you can parse source code: [...] ```
// Store some source code in an array of lines. let &"pub fn foo() {" " 1""}"// Parse the source code using a custom callback. The callback is called // with both a byte offset and a row/column offset. let &mut & let as let as if if & else b"\n" else & Noneassert_eq!"(source_file (function_item (visibility_modifier) (identifier) (parameters) (block (number_literal))))" 
```

### §Using Wasm Grammar Files

> Requires the feature wasm to be enabled.

First, create a parser with a Wasm store: [...] #### §Text Input

The source code to parse can be provided either as a string, a slice, a vector, or as a function that returns a slice. The text can be encoded as either UTF8 or UTF16:

## [Lumis: Syntax Highlighter powered by Tree-sitter and ...](https://www.reddit.com/r/rust/comments/1r92a1w/lumis_syntax_highlighter_powered_by_treesitter)

Introducing a new-ish syntax highlighter project. powered by Tree-sitter and Neovim themes. An unified API for Rust, Elixir, Java, and CLI … 70

## [GitHub - tree-sitter/tree-sitter: An incremental parsing system for programming tools · GitHub](https://github.com/tree-sitter/tree-sitter)

Public

 NotificationsYou must be signed in to change notification settings
 Fork3k (3k)
 Star27.3k (27.3k)You must be signed in to star a repository

## About

An incremental parsing system for programming tools

tree-sitter.github.io

### Topics

cincrementalparserparsingrusttree-sitterwasm

### Resources

Readme

MIT license

### Contributing

Contributing

Activity

Custom properties

### Stars

27.3k stars

### Watchers

292 watching

### Forks

3.0k forks

Report repository

BranchesTags [...] | Name | Name | Last commit message | Last commit date |
 ---  --- |
| .cargo | .cargo |  |  |
| .github | .github |  |  |
| crates | crates |  |  |
| docs | docs |  |  |
| lib | lib |  |  |
| test/fixtures | test/fixtures |  |  |
| .editorconfig | .editorconfig |  |  |
| .envrc | .envrc |  |  |
| .gitattributes | .gitattributes |  |  |
| .gitignore | .gitignore |  |  |
| .taplo.toml | .taplo.toml |  |  |
| CMakeLists.txt | CMakeLists.txt |  |  |
| CONTRIBUTING.md | CONTRIBUTING.md |  |  | [...] ### Topics

cincrementalparserparsingrusttree-sitterwasm

### Resources

Readme

MIT license

### Contributing

Contributing

Activity

Custom properties

### Stars

27.3k stars

### Watchers

292 watching

### Forks

3.0k forks

Report repository

## Sponsor this project

## Used by

You can’t perform that action at this time.

## [20+ Must-Know Rust Libraries and Frameworks for 2026](https://medium.com/@gunnar.h.karlsson/20-must-know-rust-libraries-and-frameworks-for-2026-d851aacfd2b2)

Sitemap

Open in app

Sign up

Sign in [...] Developer Tools & Infrastructure [...] .  
_Why it’s useful_: Insanely fast data manipulation and analysis. Perfect for data engineering, ETL, and data science workflows.  
Links: GitHub • pola.rs
2.   Bevy  
Data-driven game engine using an Entity Component System (ECS).  
_Why it’s useful_: Modern architecture, excellent modularity, and a rapidly growing ecosystem. The leading choice for serious Rust game development.  
Links: GitHub • bevyengine.org
3.   Macroquad  
Simple and beginner-friendly 2D game library.

## [Wrapping C libraries in Rust](https://www.youtube.com/watch?v=W0vh5VXqN3A)

[13:55] i guess that's the one uh so maybe we're gonna go to the rust crates right
[14:03] rust package registry as it's as everybody calls it um
[14:08] [Music]
[14:10] sound i o i'm gonna just pick that one i suppose there's also leap
[14:18] sound iocs and which one do we want to know
[14:28] uh you didn't i [ __ ] hate rust okay you didn't
[14:34] uh so uh let me let me see sound are you rust
[14:43] um i guess we're gonna use this one right i guess we're gonna use this one and this one [...] [3:05:37] all right let's pour a cup of tea
[3:05:38] [Music]
[3:05:42] port audio oh [ __ ]
[3:05:49] did i want you to try port audio instead
[3:06:01] uh we're already started to work on sound yeah very well then so i remember that i wanted to try a
[3:06:09] crossbar from of cross platform sound library and i don't remember which one and the way i decided before the stream i just googled up
[3:06:17] and apparently the andrew work one with better ceo so [...] [31:03] [Music]
[31:12] and interestingly enough why specifically you why did they actually dm you
[31:19] specifically you like do do they know you or or do you look like a mode maybe you look like a mod
[31:29] it's very interesting actually
[31:39] [Music]
[31:51] me yeah maybe because you're active in the chat that's that's true extern crates sleep sea look at that
[32:01] this is such an old sea that they have still use extern crate and just like that
```

### REASONING [seq 185] 2026-11-09 14:05:15

Synthesizing a comprehensive VIDEO-SPECS document covering architecture, requirements, UI, performance, testing, and Rust crate research.

### REASONING [seq 185] 2026-11-09 14:05:15

Expanding UI and storage fixes, audio preview, protocol design, and caret component scope across apps.

### REASONING [seq 185] 2026-11-09 14:05:15

Finalizing the name rationale and expanding the spec with dialog fixes, crate research, and protocol design.

### REASONING [seq 185] 2026-11-09 14:05:15

Extending the spec with caret animation parameters, dialog copy, perf budgets, file paths, and line-count verification.

### TOOL: shell [seq 185] 2026-11-09 14:05:15 status=completed

```json
{
  "command": "ls -lh /home/lufuno/smart/smart-suite-official/specs/\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs | grep -n \"fn \\|pub \" | head -n 30\necho ===APPKIT_SETTINGS===; sed -n '311,360p' /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs"
}
```

```
total 196K
-rw-r--r-- 1 lufuno lufuno 12K Nov  9 13:45 CANONICAL-SPECS.md
-rw-r--r-- 1 lufuno lufuno 92K Nov  9 13:53 EIGA-SPECS.md
-rw-r--r-- 1 lufuno lufuno 89K Nov  9 13:52 PHOTOS-SPECS.md
6:pub enum Cls {
26:fn cls_of(i: usize) -> Cls {
45:fn config(ext: &str) -> Option<tree_sitter_highlight::HighlightConfiguration> {
65:pub fn has_grammar(ext: &str) -> bool {
70:pub fn runs(src: &str, ext: &str) -> Vec<(usize, usize, Cls)> {
78:fn ts_runs(_src: &str, _ext: &str) -> Option<Vec<(usize, usize, Cls)>> {
83:fn ts_runs(src: &str, ext: &str) -> Option<Vec<(usize, usize, Cls)>> {
119:fn lex(src: &str, ext: &str) -> Vec<(usize, usize, Cls)> {
190:pub fn color(c: Cls, dark: bool) -> Color32 {
211:pub fn layout(src: &str, ext: &str, dark: bool, font: FontId, wrap: f32) -> LayoutJob {
232:    fn class_at(src: &str, ext: &str, needle: &str) -> Cls {
237:    fn covers(src: &str, ext: &str) {
249:    fn tree_sitter_rust() {
250:        let s = "// hi\nfn main() { let x = \"a\"; let n = 42; }\n";
260:    fn every_grammar_loads_and_covers_its_input() {
271:    fn markdown_headings_stand_out() {
276:    fn fallback_lexer_for_unknown_formats() {
286:    fn hostile_input_never_panics() {
295:    fn layout_job_reproduces_the_text() {
296:        let s = "fn a() {}\n// x\n";
===APPKIT_SETTINGS===
    pub fn draw_app_settings(&mut self, ui: &mut Ui, win: Rect, app: AppId, pages: &[(&'static str, &'static str)], rows_for: &dyn Fn(&SmartApp, usize) -> Vec<SRow>) -> Option<(usize, usize, SVal)> {
        let th = self.th.clone();
        let ctx = ui.ctx().clone();
        let page = self.shell.settings.map(|s| s.1).unwrap_or(0).min(pages.len() - 1);
        let pop = spring_to(&ctx, ("setpop", app), 1.0, 260.0, 24.0).clamp(0.0, 1.2);
        let mut sui = ui.new_child(egui::UiBuilder::new().max_rect(win));
        sui.set_opacity((pop * 3.0).clamp(0.0, 1.0));
        let p = sui.painter().clone();
        let scrim = sui.allocate_rect(win, Sense::click());
        let sp = p.with_clip_rect(win);
        grad_fill(&sp, win, R_WIN, th.scrim, th.scrim);
        let w = (win.width() - 40.0).clamp(380.0, 660.0);
        let h = (win.height() - 56.0).clamp(300.0, 480.0);
        let card = Rect::from_center_size(win.center() + vec2(0.0, (1.0 - pop.min(1.0)) * 16.0), vec2(w, h));
        menu_surface(&p, &th, card, 22.0);
        sui.allocate_rect(card, Sense::click());
        // header
        let tile = Rect::from_min_size(card.min + vec2(20.0, 18.0), vec2(34.0, 34.0));
        let (a, b) = th.accent_grad();
        shadow(&p, tile, 11.0, 3.0, 8.0, -2.0, alpha(th.accent, 0.4));
        grad_fill(&p, tile, 11.0, a, b);
        rim(&p, tile, 11.0, 1.0, Color32::from_white_alpha(130), true, 16.0);
        self.ico(&p, app.icon(), Ic::Duo, tile.center(), 18.0, Color32::WHITE);
        p.text(pos2(tile.right() + 12.0, tile.center().y - 8.0), Align2::LEFT_CENTER, format!("{} Settings", app.name()), f_bold(T_TITLE), th.ink1);
        p.text(pos2(tile.right() + 12.0, tile.center().y + 9.0), Align2::LEFT_CENTER, "Changes apply instantly and are remembered", f_medium(T_META), th.ink3);
        let x = Rect::from_center_size(pos2(card.right() - 28.0, tile.center().y), vec2(28.0, 28.0));
        let mut close = self.icon_button(&sui, x, "close-circle", "aset-x", Btn::Quiet, true).clicked();
        // page rail
        let rail = Rect::from_min_max(pos2(card.left() + 12.0, card.top() + 68.0), pos2(card.left() + 168.0, card.bottom() - 62.0));
        let mut new_page = page;
        // one indicator glides between pages on the same spring as the segmented controls
        let yy = spring_to(&ctx, ("aset-rail", app), page as f32, 330.0, 23.0);
        if (yy - page as f32).abs() > 0.002 {
            ctx.request_repaint();
        }
        let ind = Rect::from_min_size(rail.min + vec2(0.0, yy * 38.0), vec2(rail.width(), 34.0));
        grad_fill(&p, ind, 10.0, th.accent_tint, th.accent_tint2);
        rim(&p, ind, 10.0, 1.0, alpha(Color32::WHITE, if th.dark { 0.10 } else { 0.7 }), true, 12.0);
        for (i, (icon, name)) in pages.iter().enumerate() {
            let r = Rect::from_min_size(rail.min + vec2(0.0, i as f32 * 38.0), vec2(rail.width(), 34.0));
            let resp = self.hit(&sui, r, ("aset-p", app, i), Sense::click());
            let on = i == page;
            if !on && resp.hovered() {
                p.rect_filled(r, cr(10.0), th.row_hover);
            }
            self.ico(&p, icon, if on { Ic::Duo } else { Ic::Outline }, pos2(r.left() + 19.0, r.center().y), 16.0, if on { th.accent_ink } else { th.ink3 });
            p.text(pos2(r.left() + 38.0, r.center().y), Align2::LEFT_CENTER, *name, if on { f_semi(T_BODY) } else { f_medium(T_BODY) }, if on { th.ink1 } else { th.ink2 });
            if resp.clicked() {
                new_page = i;
            }

```

### TOOL: write [seq 200] 2026-11-09 14:05:32 status=completed

```json
{
  "content": "# KURO — Modern Video Player Spec (Protocol-First, Rust-Native Successor to EIGA)\n\n**Status:** `SPEC READY — DO NOT SHIP PLAYER WITHOUT §4 L-items + §7 Budgets + §10 Test Plan + §11 Acceptance`\n**Date:** 2026-11-09\n**Successor to:** `EIGA-SPECS.md` (757 lines, 94 kB) at `smart-suite-official/specs/EIGA-SPECS.md`\n**Source roots (non-destructive, verified):**\n- `/home/lufuno/Documents/rust-wasm-media-manager-latest/` (live workspace, also `rust-wasm-media-manager-2.zip` identical; original 23 MB zip is older `video.rs 55 kB` lineage — discard)\n  - `crates/eiga-core/src/lib.rs` 92 lines — `fmt_time`/`parse_time` pure\n  - `crates/eiga-core/src/player.rs` 820 lines / 30 125 B — `Player` state machine + `Playlist` + `Cmd` key map (16 tests)\n  - `crates/eiga-core/src/source.rs` 731 lines / 30 036 B — `parse_ffprobe` JSON + `FrameSource`/`TestCard` + `ffmpeg` pipes (`FfmpegSource`/`AudioPipe`)\n  - `crates/eiga-core/src/subs.rs` 401 lines / 14 951 B — SRT/WebVTT/ASS sniff parser\n  - `crates/smart-ui-egui/src/video.rs` 1 523 lines / ~76 kB (latest) — `Eiga` surface (draw/HUD/menus/settings)\n  - `crates/smart-core/src/thumbs.rs` (image-only; prod `video_thumb` lost — see §3), `model.rs` `Kind::from_ext/previewable`, `audio.rs` rodio/HTMLAudio/silent-stub 3-tier, `lib.rs` drag resolve, `modals.rs` Preview/Text Preview/Open With/Devices/Phone, `chrome.rs` app bar + tab strip + `DROP_TARGETS`, `storage.rs` treemap/graph, `highlight.rs` tree-sitter, `shell.rs` `AppId::Video`\n- Protocol: `/home/lufuno/smart/smart-protocol-official/schema/` — `media.capnp @0xa1b2c3d4e5f60011`, `audio.capnp`, `files.capnp`, `clipboard.capnp`, `notification.capnp`, `device.capnp`, `capability.capnp`, `types.capnp`. **No `domain-video` / `video.capnp` exists today** — this spec adds it (§9). Compiled Rust in `/home/lufuno/smart/smart-protocol-rs/`.\n- Live protocol/eiga today is protocol-free (no `Media` impl). That is the P0 gap this spec closes.\n- Packaging context: studio `SmartShell-x86_64.AppImage` (11 MB) + prod `build-appimage.sh` (see §9/EIGA §7) — neither bundles ffmpeg; media line has no `scripts/` at all.\n- Read-only audit copy: `/tmp/opencode/canonical-video-audit/VIDEO-SPECS.md` → copied to `/home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md`. No destructive writes to source roots.\n\n---\n\n## 1. Name Choice + Rationale — `KURO` (黒)\n\n### 1.1 Decision: rename EIGA → **KURO** (crate `kuro-core`, view `Kuro`, window `AppId::Video` label `\"Kuro\"`)\n\n| Candidate | Japanese | Meaning | Verdict |\n|---|---|---|---|\n| EIGA (existing, 映画) | 映画 `eiga` | \"movie\" | Retire. It means movie — correct but generic, says what it is, not how it feels. Keep as deprecated alias for one release (`pub use kuro_core as eiga_core` + `Eiga` type alias) so tests/archive links do not break. Do not ship as primary. |\n| **KURO (黒)** | 黒 `kuro` | \"black\" | **CHOSEN.** Timeless, 4 letters, premium, dark-theatre evocation (black stage, letterbox, spotlight). Short, pronounceable in every locale, ownable as search term, pairs with the Japanese-naming lineage (Eiga → Kuro = light→dark evolution, movie→cinema). Single-word brand works as app name, crate name, CLI (`kuro play …`), D-Bus name (`org.smart.Kuro`), and MIME handler. Kilo-scale distinct from \"media player\" genericism. |\n| Kino | German `Kino` | cinema | Rejected: not Japanese, loses lineage. Good word, wrong family. |\n| Yoru (夜) | 夜 `yoru` | night | Rejected: evokes sleep, not projection. Dark but passive. |\n| Nagare (流れ) | 流れ `nagare` | flow/stream | Rejected: verb-y, long, implies streaming-only (we are local-first). |\n| EigaNeo / EigaNext | — | \"version two\" | Rejected: versioned suffixes age instantly (timelessness request). Kuro names the *experience*, not the version. |\n| Hana (花) / Yume (夢) | 花 / 夢 | flower / dream | Rejected: too soft for a technical player; Kuro's black reads \"expensive\" (like a piano, like a theatre). |\n\n### 1.2 How Kuro is used\n\n- **Public deployment name:** \"Kuro\" (app label). Presentation name defaults to \"Kuro\"; user may change it in Settings (persisted via `settings.capnp`, never hardcoded).\n- **Crate:** `crates/kuro-core` (rename of `eiga-core`, same file layout `lib/player/source/subs.rs`; `Cargo.toml` `name = \"kuro-core\"`, `description = \"Kuro (黒) engine: playback state machine, playlist, key map, subtitle parsers, ffprobe parsing and frame sources. No UI code.\"`). Keep `eiga-core` shim for one release, then delete.\n- **View:** `smart-ui-egui/src/video.rs` struct `Kuro` (rename of `Eiga`, fields identical + §5 additions). Keep `eiga_*` methods as `#[deprecated]` shims during migration; new code calls `kuro_*`.\n- **Sidebar header:** \"Kuro\" + subtitle \"Video player · 黒\" (mirrors today's `\"Eiga\" / \"Video player · 映画\"`). Exactly two lines, never a long card.\n- **Icon:** `clapperboard` Solar Duotone (existing) — no new art required. Kuro tile gradient already exists (`#8b6cff → #3a2db0` in `shell.rs` dock).\n- **About:** honest one-liner (see §8.9), never \"AI-powered\" marketing copy.\n\n### 1.3 Why a new name at all\n\nEiga says *what* (movie). Kuro says *where/how* (the black room where movies happen). It is future-proof: 4K, 8K, HDR, spatial audio, casting, live — all happen in the black. It survives codec churn. It does not promise a backend (ffmpeg/symphonia/WebCodecs are disposable §2). It is distinct from EIGA-SPECS.md so the two specs do not collide in search, docs, or telemetry keys. Timeless, expensive-feeling, compact — exactly what the user brief asks for.\n\n---\n\n## 2. Architecture — Protocol-First, Backends Disposable, One Engine for Player + File-Manager Preview\n\n### 2.1 Three layers (wire → contract → disposables)\n\n```\n┌─────────────────────────────────────────────────────────────────────────────┐\n│ VIEW  smart-ui-egui/src/video.rs  (Kuro surface)                             │\n│  draw_kuro / kuro_toolbar / kuro_controls / kuro_sidebar / overlay_bar /     │\n│  stats / help / menus. egui 0.37 + eframe glow/x11/wayland + paint/chrome/   │\n│  appkit. NO ffmpeg, NO rodio, NO tree-sitter imports. Talks ONLY to:          │\n│  kuro-core::player::Player+Playlist+Cmd, FrameSource+MediaInfo+SubTrack,     │\n│  smart-protocol-rs domain-media (Media) + domain-video (Video) traits.        │\n│  Hypnotic caret comes from shared smart-ui-egui caret module (§8.7),           │\n│  NOT from terminal (see clarification).                                       │\n├─────────────────────────────────────────────────────────────────────────────┤\n│ CORE  kuro-core (no UI / clock / window) — THE WIRE, must stay testable on   │\n│  wasm32-unknown-unknown + native. Pure logic, 39 tests today (see §10).      │\n│  player.rs: Player{duration,position,paused,speed,volume,muted,looping,      │\n│    eof,fps,audio_delay,chapters,osd} + tick(dt)* + Playlist{items,cur,       │\n│    shuffle,repeat,order,seed} + Cmd (~60 mpv bindings) + SPEEDS/MAX_VOLUME.   │\n│  source.rs: MediaInfo{title,container,duration,size,bitrate,video[],audio[],│\n│    subs[],chapters} + parse_ffprobe(JSON)+fit_size+TestCard+FfmpegSource+    │\n│    AudioPipe+atempo_chain+extract_sub. FrameSource{info,seek,poll,name,      │\n│    exhausted}. Extend: color_*, field_order, rotation, side_data, artaData.  │\n│  subs.rs: SubTrack{name,lang,cues,delay}+Cue{start,end,text,italic,top}+     │\n│    decode_bytes/parse/parse_ts/find_sidecars/guess_lang/active/next|prev.    │\n│  lib.rs: fmt_time/parse_time(+ms).                                           │\n├─────────────────────────────────────────────────────────────────────────────┤\n│ PROTOCOL smart-protocol-rs: media.capnp (Media) + NEW domain-video (Video)  │\n│  + audio.capnp (spatial/presets/focus) + files.capnp (library/recents/trash/ │\n│  hash) + clipboard.capnp (PNG/text/json) + notification.capnp (playback/     │\n│  cast/transcode channels) + device.capnp (MTP/phone) + capability.capnp.      │\n│  Opaque methods: caller never knows backend (ffmpeg vs symphonia vs           │\n│  WebCodecs vs cast). No hardcoded codecs/formats/mimes in view. Doorbell     │\n│  subscriptions: pull-bounded PlayerSubscription with wake handle (§9).        │\n├─────────────────────────────────────────────────────────────────────────────┤\n│ BACKENDS DISPOSABLE (implement Video/Media traits, never imported by view)    │\n│  • ffmpeg-CLI native (today: FfmpegSource+AudioPipe, -hwaccel auto)          │\n│  • ffmpeg-next / rsmedia / ff-decode in-process (future demux+hw, §5.1)      │\n│  • symphonia+rubato+rodio (audio-only, pure-Rust, §5.1)                     │\n│  • Web <video>/WebCodecs (WASM: TestCard + real subs — stay honest)          │\n│  • Cast/DLNA/AirPlay, PiP compositor surface, HDR display HAL                 │\n└─────────────────────────────────────────────────────────────────────────────┘\n```\n\nInvariants (enforced by NEW tests, not by trust):\n\n1. **Timeline is king.** Decoders never move `Player::position`. Only `seek_abs/rel/frac` or `tick()` move it. `FrameSource::poll(clock)` is a pure function of that clock (today PTS is synthetic `start+n/fps` — §4 fixes to real stream timebase + `probe`).\n2. **Seek is honest or it says so.** If `-ss` is keyframe-approximate, UI shows `≈ HH:MM:SS (keyframe)` and `frame_step` does exact second-pass (§5 U-03). No silent pretend.\n3. **Subtitle = pure function of timeline.** `SubTrack::active(t)` already is (partition-point + `max_dur` window, fuzz-tested). Keep. After `delay` change, re-evaluate same tick.\n4. **No fake backend.** `getSupportedFormats`/`getVideoCaps` return `NotSupported` for unimplemented HDR/spatial/cast rather than faking a playlist that cannot play it.\n5. **No destructive without dialog + reversible.** Playlist `remove` is in-memory + undoable (`Ctrl+Z` restores index); `delete` never appears in Kuro (only \"Show in Files\" → Files delete-confirm). Screenshot, sub-attach, resume-clear are reversible or toast-with-Undo.\n6. **One engine, two surfaces.** File-manager Quick Look video preview (§8.5) and the Kuro window MUST call the same `kuro_core` + same `Video` backend. No second decoder, no `TestCard` divergence, no separate thumb pipeline that can drift.\n7. **Caret is framework, not terminal.** The hypnotic caret is a `smart-ui-egui` shared component (§8.7) with framework style; terminal is just another consumer. Search, rename, pickers, AI composer, every dialog input inherit it. Stock egui caret is never left visible.\n\n### 2.2 Data flow\n\n**Native play (local file, ffmpeg present):**\n```\nVfs(LocalVfs:real_path) ──► probe(path) ──► MediaInfo ──┐\nUser open/drag/drop/picker/CLI %f ──► kuro_open(Location) ──► kuro_siblings() ┤ (folder video scan, sorted)\n                                                                              ▼\n                                                                    Player::new(duration,fps)+Playlist+ResumeMap\n                                                                              ▼\n                          ┌───────────────────────────────────────────────┐\n                          │ Backend::open(path|uri, caps, decode_size)     │\n                          │ ffmpeg: -ss -i -an -sn -vf scale+tonemap -pix  │\n                          │ -f rawvideo -  (or hw surface handle future)   │\n                          │ reader thread sync/latch channel<Frame>(N)     │\n                          └───────────────────────────────────────────────┘\n                                                                              ▼\n                                                                  FrameSource::poll(clock)  (timeline→pts_ns)\n                                                                              ▼ egui::ColorImage + texture LINEAR\n                                                                  draw_kuro │ subs (movable, §8.2) │ HUD\n                                                                              ▲\n   
```

```
Created file successfully: /tmp/opencode/canonical-video-audit/VIDEO-SPECS.md
```

### TOOL: shell [seq 207] 2026-11-09 14:08:19 status=completed

```json
{
  "command": "wc -l /tmp/opencode/canonical-video-audit/VIDEO-SPECS.md; cp /tmp/opencode/canonical-video-audit/VIDEO-SPECS.md /home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md && ls -lh /tmp/opencode/canonical-video-audit/VIDEO-SPECS.md /home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md && head -n 30 /home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md"
}
```

```
879 /tmp/opencode/canonical-video-audit/VIDEO-SPECS.md
-rw-r--r-- 1 lufuno lufuno 128K Nov  9 14:08 /home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md
-rw-r--r-- 1 lufuno lufuno 128K Nov  9 14:08 /tmp/opencode/canonical-video-audit/VIDEO-SPECS.md
# KURO — Modern Video Player Spec (Protocol-First, Rust-Native Successor to EIGA)

**Status:** `SPEC READY — DO NOT SHIP PLAYER WITHOUT §4 L-items + §7 Budgets + §10 Test Plan + §11 Acceptance`
**Date:** 2026-11-09
**Successor to:** `EIGA-SPECS.md` (757 lines, 94 kB) at `smart-suite-official/specs/EIGA-SPECS.md`
**Source roots (non-destructive, verified):**
- `/home/lufuno/Documents/rust-wasm-media-manager-latest/` (live workspace, also `rust-wasm-media-manager-2.zip` identical; original 23 MB zip is older `video.rs 55 kB` lineage — discard)
  - `crates/eiga-core/src/lib.rs` 92 lines — `fmt_time`/`parse_time` pure
  - `crates/eiga-core/src/player.rs` 820 lines / 30 125 B — `Player` state machine + `Playlist` + `Cmd` key map (16 tests)
  - `crates/eiga-core/src/source.rs` 731 lines / 30 036 B — `parse_ffprobe` JSON + `FrameSource`/`TestCard` + `ffmpeg` pipes (`FfmpegSource`/`AudioPipe`)
  - `crates/eiga-core/src/subs.rs` 401 lines / 14 951 B — SRT/WebVTT/ASS sniff parser
  - `crates/smart-ui-egui/src/video.rs` 1 523 lines / ~76 kB (latest) — `Eiga` surface (draw/HUD/menus/settings)
  - `crates/smart-core/src/thumbs.rs` (image-only; prod `video_thumb` lost — see §3), `model.rs` `Kind::from_ext/previewable`, `audio.rs` rodio/HTMLAudio/silent-stub 3-tier, `lib.rs` drag resolve, `modals.rs` Preview/Text Preview/Open With/Devices/Phone, `chrome.rs` app bar + tab strip + `DROP_TARGETS`, `storage.rs` treemap/graph, `highlight.rs` tree-sitter, `shell.rs` `AppId::Video`
- Protocol: `/home/lufuno/smart/smart-protocol-official/schema/` — `media.capnp @0xa1b2c3d4e5f60011`, `audio.capnp`, `files.capnp`, `clipboard.capnp`, `notification.capnp`, `device.capnp`, `capability.capnp`, `types.capnp`. **No `domain-video` / `video.capnp` exists today** — this spec adds it (§9). Compiled Rust in `/home/lufuno/smart/smart-protocol-rs/`.
- Live protocol/eiga today is protocol-free (no `Media` impl). That is the P0 gap this spec closes.
- Packaging context: studio `SmartShell-x86_64.AppImage` (11 MB) + prod `build-appimage.sh` (see §9/EIGA §7) — neither bundles ffmpeg; media line has no `scripts/` at all.
- Read-only audit copy: `/tmp/opencode/canonical-video-audit/VIDEO-SPECS.md` → copied to `/home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md`. No destructive writes to source roots.

---

## 1. Name Choice + Rationale — `KURO` (黒)

### 1.1 Decision: rename EIGA → **KURO** (crate `kuro-core`, view `Kuro`, window `AppId::Video` label `"Kuro"`)

| Candidate | Japanese | Meaning | Verdict |
|---|---|---|---|
| EIGA (existing, 映画) | 映画 `eiga` | "movie" | Retire. It means movie — correct but generic, says what it is, not how it feels. Keep as deprecated alias for one release (`pub use kuro_core as eiga_core` + `Eiga` type alias) so tests/archive links do not break. Do not ship as primary. |
| **KURO (黒)** | 黒 `kuro` | "black" | **CHOSEN.** Timeless, 4 letters, premium, dark-theatre evocation (black stage, letterbox, spotlight). Short, pronounceable in every locale, ownable as search term, pairs with the Japanese-naming lineage (Eiga → Kuro = light→dark evolution, movie→cinema). Single-word brand works as app name, crate name, CLI (`kuro play …`), D-Bus name (`org.smart.Kuro`), and MIME handler. Kilo-scale distinct from "media player" genericism. |
| Kino | German `Kino` | cinema | Rejected: not Japanese, loses lineage. Good word, wrong family. |
| Yoru (夜) | 夜 `yoru` | night | Rejected: evokes sleep, not projection. Dark but passive. |

```

### ASSISTANT [seq 214] 2026-11-09 14:08:23

**Done — `VIDEO-SPECS.md` written (879 lines, 128 kB), non-destructively.**

- Audit: `/tmp/opencode/canonical-video-audit/VIDEO-SPECS.md`
- Official: `/home/lufuno/smart/smart-suite-official/specs/VIDEO-SPECS.md`

**Name: KURO (黒)** — chosen over Eiga (映画, says *what*), Kino (wrong family), Yoru (passive), Nagare (streaming-only), EigaNeo (ages instantly). Kuro = *black* (stage/letterbox/spotlight): timeless, 4 letters, premium, keeps the Japanese lineage (movie→cinema, light→dark), works as app/crate/CLI/D-Bus/MIME.

**What the spec covers:**
- §2 Architecture (protocol-first, backends disposable, one engine for player + file-manager Quick Look — fixes current `m_preview` Video → "No preview" lie)
- §3 Per-file matrix (verified sizes: `player.rs` 820 lines/30 125 B, `source.rs` 731/30 036, `subs.rs` 401/14 951, `video.rs` 1 523/~76 kB)
- §4 Lies: L-01…L-24 + D-01…D-14 + C-01 (dialogs, storage, drag, preview, AppImage)
- §5 Crate truth table (ffmpeg-CLI vs ffmpeg-next/rsmedia/ff-decode vs symphonia/audio-only vs rodio, HW VAAPI/NVDEC, HDR `zscale_tonemap`/`oximedia-hdr`, ASS `libass`, tree-sitter) + U-01…U-22 (resume, HDR10, spatial/Atmos, casting, PiP, live/network, shader, movable+shortened subs)
- §6 Perf budgets (720p ≤3 ms upload, 4K60 HW-only, thumbs vs player, wasm ≤8 ms, adversarial caps)
- §8 UX laws: appbar collapse, Beautiful subtitle feedback (.movable 9 anchors + shorten/OOSD+Undo), dialog laws W/G/X/A/S/D (wrap-measure, grid, close at top+20 never shifts, spring open/close, centred selectors, drag always resolves), storage 92 % reddish + Cleanup as tab + Obsidian graph (+ player-settings storage card), text preview (no fake chrome, scrollable/tree-sitter/copy), Open With wrap-fix, spring micro-interactions, audio-preview honesty, About honest copy, `dropped_files` hover-routing + single-tab fix
- **§8.7 Hypnotic caret (clarification): shared `smart-ui-egui` framework component** (`caret.rs` + `paint_hypnotic_caret`), not terminal-owned. Breathing `0.65+0.35·sin(2πt/1.4)` (0.30–1.00, never hard-blink) + jump `spring_to(420/26)` overshoot, inherits theme/`reduce_motion`. Every input (search, rename, pickers, AI composer, all dialogs, Kuro jump-to) inherits it; terminal bespoke caret deleted.
- §9 Protocol: `media.capnp` mapping + new `domain-video`/`video.capnp` sketch (opaque methods, no hardcoded, doorbell subscriptions)
- §10 Test plan (`cargo test` 39+new gates, adversarial corpus, grep CI gates) + §11 Acceptance checklists + §B commit order

### IDLE [seq 221] 2026-11-09 14:08:30

```json
{
  "time": {
    "created": 1794226110014
  },
  "outcome": "succeeded"
}
```

