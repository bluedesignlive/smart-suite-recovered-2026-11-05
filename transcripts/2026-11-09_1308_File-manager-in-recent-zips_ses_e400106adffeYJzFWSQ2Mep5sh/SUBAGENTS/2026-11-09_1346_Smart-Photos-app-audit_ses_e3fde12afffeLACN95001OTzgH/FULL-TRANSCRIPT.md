# Subagent transcript — Smart Photos app audit

- Session ID: `ses_e3fde12afffeLACN95001OTzgH`
- Parent: `ses_e400106adffeYJzFWSQ2Mep5sh`
- Created: 2026-11-09 13:46:51
- Updated: 2026-11-09 13:52:55
- Messages: 44
- User/task msgs: 1

---

### TASK / USER [seq 4] 2026-11-09 13:46:51

You are a subagent spawned by another session.
You are the SMART PHOTOS APP agent. Source is really complete if honest — make it real too. Source in /home/lufuno last 7 days, ignore hidden: /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip, /home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip, /home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip for smart-ui-egui/src/photos.rs 74978 (1467 lines), plus /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip smart-desktop/apps/photos/mod.rs 75586, plus smart-core library.rs 6223, thumbs.rs 5183, analyze.rs 31515, storage.rs, highlight.rs. Do non-destructive work to /tmp/opencode/canonical-photos-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md.

Focus:
- Full photos app reality: library, viewer, filmstrip, editor, capture, picker integration. Per-file best, lie hunt (fake zoom, fake editor, fake metadata), performance (thumb cache LRU 500MB, big_thumbs 8, mtime key), memory.
- Eiga thumb pipeline reuse for blazing previews, but for photos: RAW/HEIC/AVIF, EXIF, face detection, object search, smart albums, AI grouping (reversible), editor (crop/rotate/filter/history, all reversible).
- Missing features the user hasn't thought of: find them (unknown unknowns: duplicate finder, offline sync, backup, RAW+ HDR stack, palette, locations/on-map, Recents tombstones, MTP/phone import bug like file manager phone never mounts).
- Protocol-first: depend on smart-protocol-rs domain-storage/domain-media/domain-files, backends disposable (Linux/Android/iOS).
- Accessibility, drag-drop, picker (with New Folder/search), trash integration, destructive dialogs, settings file-manager-only style.
- Output PHOTOS-SPECS.md with matrix, lies, new ideas, perf budget, UX, integration, acceptance. Non-destructive, thorough.

### TOOL: read [seq 5] 2026-11-09 13:46:51 status=completed

```json
{
  "path": "/home/lufuno"
}
```

```
Read directory /home/lufuno, entries 1-105
.aider/
.bun/
.cache/
.cargo/
.cline/
.config/
.cua-driver/
.dbus/
.dsh/
.gnupg/
.herdr/
.hermes/
.hiddel/
.jcode/
.kilo/
.local/
.mozilla/
.npm/
.opencode/
.pki/
.rustup/
.var/
.vscode-oss-shared/
.vscode-oss/
.wine.bak/
.wine/
.zcode/
assets/
crates/
Desktop/
Documents/
donghua/
Downloads/
Lemon-Studio/
mirui/
models/
Music/
Notes/
Pictures/
Projects/
Public/
smart-interface/
smart/
target/
Templates/
tests/
tools/
untitled folder/
Videos/
web/
wine-staging/
work/
.aider.chat.history.md
.aider.input.history
.bash_history
.bash_logout
.bash_profile
.bashrc
.dmrc
.git-credentials
.gitconfig
.ICEauthority
.inputrc
.netrc
.profile
.wget-hsts
.Xauthority
.xsession-errors
.xsession-errors.old
.zshenv
.zshrc
62 - 《牧神记》第58话：虫军【玄幻 ｜ 修仙 ｜ 小说改 ｜ Made By Bilibili】.f251.webm.part
bonsai-ternary.html
Cargo.lock
Cargo.toml
direct_usb
direct_usb.c
downloaded.txt
enable_keyboard.bat
file-de-cluade.txt
fl-studio.sh
index.html
kilo_audit_005_chat_context.txt
kilo_chats_export.txt
last-de-cluade-2.txt
last-de-cluade-prompt.txt
last-de-cluade.txt
lint-stub.ts
package-lock.json
package.json
probe_usb
probe_usb.c
qwen-last.txt
qwen.txt
README.md
run_palera1n.sh
tsconfig.json
Untitled.flp
usb_fix
usb_fix.c
usb_inject.c
usb_inject2.c
usb_pre.c
veryl-x86_64-linux.zip
vite.config.ts
```

### TOOL: read [seq 16] 2026-11-09 13:46:53 status=completed

```json
{
  "path": "/home/lufuno/work/BIGBOY"
}
```

```
Read directory /home/lufuno/work/BIGBOY, entries 1-73
bigboy-soweto-commercial-website/
bigboy-sveltekit/
build-cross-platform-rust-file-manager-last/
improve-app-consistency-and-completeness-good/
rust-wasm-ui-refinements2/
shots/
01a17324-3baf-743e-a1c0-637b2f3f78ea.patch
1793698466656-29156505-7dc0-4fe0-8ebe-70a264c527d9-01a1a0c1-3e34-7c0d-8edb-74cc950f032f.jpeg
1793698688326-c77aaa75-17e1-4f22-9b36-f7f1f3f14e5b-01a1a0c5-55b8-7274-8641-0f73b6b59452.png
1793699139844-73986115-906c-4458-8744-be1b105fee51-01a1a0cc-3fef-79ef-94a4-02c273d9e26b.png
ADCB84696DF6CE6CE69C71486A245BC19D9E3047.torrent
bigboy-soweto-commercial-website--k2.7.zip
bigboy-soweto-commercial-website--minimax.zip
bigboy-soweto-commercial-website.zip
bigboysoweto_co_za.zone
build-cross-platform-rust-file-manager-good1.zip
build-cross-platform-rust-file-manager-last.zip
build-cross-platform-rust-file-manager.zip
build-skeuomorphic-amapiano-logdrum-plugin-3.zip
build-skeuomorphic-amapiano-logdrum-plugin-good.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush2.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush4.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush5 broken.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush5 broken2.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush5.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush6.zip
build-skeuomorphic-amapiano-logdrum-plugin-rush7.zip
build-skeuomorphic-amapiano-logdrum-plugin.zip
build-skeuomorphic-amapiano-logdrum-plugin2.zip
build-skeuomorphic-amapiano-logdrum-rush3-plugin.zip
develop-amapiano-logdrum-plugin-05.zip
develop-amapiano-logdrum-plugin-4.zip
develop-amapiano-logdrum-plugin-rush.zip
develop-amapiano-logdrum-plugin-rush4.zip
develop-amapiano-logdrum-plugin-rush5-dx10 needs fix.zip
develop-amapiano-logdrum-plugin-rush5.zip
develop-amapiano-logdrum-plugin.zip
gpt-image-2.5-sunburst_a_add_a_piano_and_a_pr.png
gpt-image-2.5-sunburst_b_I_am_planning_to_bui.png
gpt-image-2.5-sunburst_b_remove_the_sounds_fr.png
high-fidelity-skeuomorphic-daw-prototype.zip
i-logdrum.jpeg
improve-app-consistency-and-completeness-good.zip
improve-app-consistency-and-completeness-last.zip
improve-app-consistency-and-completeness.zip
initial-user-greeting.2.1.zip
initial-user-greeting.2.zip
initial-user-greeting.zip
Latest Price List 14 July 2026.pdf
premium-app-component-kit.zip
premium-skeuomorphic-daw-prototype-2-different.zip
premium-skeuomorphic-daw-prototype-bad.zip
premium-skeuomorphic-daw-prototype-bad2.zip
premium-skeuomorphic-daw-prototype.zip
recreate-file-manager-in-rust-2.zip
recreate-file-manager-in-rust-bad.zip
recreate-file-manager-in-rust-good.zip
recreate-file-manager-in-rust.zip
refine-futuristic-desktop-environment-bad.zip
refine-futuristic-desktop-environment-bad2.zip
run-egui-wasm-showcase-.zip
run-egui-wasm-showcase-e-log.zip
run-egui-wasm-showcase-rush.zip
run-egui-wasm-showcase-rush2.zip
run-egui-wasm-showcase-rush3.zip
run-egui-wasm-showcase.zip
rust-wasm-ui-refinements.zip
rust-wasm-ui-refinements2.zip
seedream-5.0-pro_a_create_a_premium_ui_.jpeg
Smart Recording 02-02-24.webm
Smart Voice 01-14-37.ogg
WhatsApp Image 2026-10-09 at 15.03.43.jpeg
```

### TOOL: read [seq 16] 2026-11-09 13:46:53 status=completed

```json
{
  "path": "/home/lufuno/smart/smart-suite"
}
```

```
Read directory /home/lufuno/smart/smart-suite, entries 1-2
apps/
prompt1.txt
```

### TOOL: shell [seq 16] 2026-11-09 13:46:53 status=completed

```json
{
  "command": "ls -R /home/lufuno/smart 2>&1 | head -n 300; echo ---; ls /tmp/opencode/canonical-photos-audit/ 2>&1; echo ---; ls /home/lufuno/smart/smart-suite-official/specs/ 2>&1 | head -n 50"
}
```

```
/home/lufuno/smart:
HQ
PLAN-001.md
Space_UI
Space_UI (Copy)
backups
bantu-labs
brand
cluade.md
crextio
crextio-complete-repo-with-source-2026-09-21.zip
crextio-engineering-complete
crextio-engineering-complete.zip
crextio-notes-stash
demos
draft.md
draft.txt
kukan
kukan-slint
modern-premium-os-design-system.zip
naledzi
naledzi_ui
plan_sealed_use_agents.md
prototypes
qwen-1
sa4-protocol-full
sa4-smart-protocol-official-stash
session-ses_e7cd-now.md
smart-agent-site
smart-cli-final.md
smart-cli-ng
smart-design
smart-foundation
smart-lib-ng
smart-os
smart-protocol
smart-protocol-official
smart-protocol-pal
smart-protocol-rs
smart-shell-spec
smart-suite
smart-suite-official
smart-ui-gpui
smartui-workspace
strange_fake
trash.md
wall-lockscreen

/home/lufuno/smart/HQ:
LOG.md
PROJECTS.md
TASKS.md

/home/lufuno/smart/Space_UI:
46554.zip
AGENTS.md
CHANGELOG.md
CONTRIBUTING.md
Cargo.lock
Cargo.toml
LICENSE-APACHE
LICENSE-MIT
Makefile
README.md
crates
docs
justfile
lucide
material-1.0
modern-premium-os-design-system.zip
os-.zip
os-04.zip
os-1.zip
os-2.zip
os-3.zip
space-ui-design-system
space-ui-design-system.zip
target
tools

/home/lufuno/smart/Space_UI/crates:
apps
gallery
space-compositor
space-core
space-de
space-host
space-protocol
space-shell
space-ui
space-wm

/home/lufuno/smart/Space_UI/crates/apps:
Cargo.toml
build.rs
src
ui

/home/lufuno/smart/Space_UI/crates/apps/src:
bin

/home/lufuno/smart/Space_UI/crates/apps/src/bin:
calculator.rs

/home/lufuno/smart/Space_UI/crates/apps/ui:
calculator.slint

/home/lufuno/smart/Space_UI/crates/gallery:
Cargo.toml
build.rs
src
ui

/home/lufuno/smart/Space_UI/crates/gallery/src:
demo_models.rs
main.rs

/home/lufuno/smart/Space_UI/crates/gallery/ui:
assets
gallery.slint
sections.slint
sections2.slint

/home/lufuno/smart/Space_UI/crates/gallery/ui/assets:
bell.svg
gear.svg
heart.svg
play.svg
search.svg
star.svg
trash.svg
wifi.svg

/home/lufuno/smart/Space_UI/crates/space-compositor:
Cargo.toml
src
tests

/home/lufuno/smart/Space_UI/crates/space-compositor/src:
buffers.rs
engine.rs
lib.rs
service.rs

/home/lufuno/smart/Space_UI/crates/space-compositor/tests:
compositor_contract.rs

/home/lufuno/smart/Space_UI/crates/space-core:
Cargo.toml
src

/home/lufuno/smart/Space_UI/crates/space-core/src:
lib.rs

/home/lufuno/smart/Space_UI/crates/space-de:
Cargo.toml
src

/home/lufuno/smart/Space_UI/crates/space-de/src:
host.rs
main.rs

/home/lufuno/smart/Space_UI/crates/space-host:
Cargo.toml
src

/home/lufuno/smart/Space_UI/crates/space-host/src:
lib.rs

/home/lufuno/smart/Space_UI/crates/space-protocol:
Cargo.toml
build.rs
src
tests

/home/lufuno/smart/Space_UI/crates/space-protocol/src:
bootstrap.rs
handshake.rs
lib.rs
transport.rs

/home/lufuno/smart/Space_UI/crates/space-protocol/tests:
handshake_roundtrip.rs

/home/lufuno/smart/Space_UI/crates/space-shell:
Cargo.toml
build.rs
src
ui

/home/lufuno/smart/Space_UI/crates/space-shell/src:
chrome.rs
engine.rs
lib.rs
service.rs

/home/lufuno/smart/Space_UI/crates/space-shell/ui:
root.slint

/home/lufuno/smart/Space_UI/crates/space-ui:
Cargo.toml
assets
build.rs
src
ui

/home/lufuno/smart/Space_UI/crates/space-ui/assets:
wallpaper-default.png
wallpaper-green-liquid.jpg

/home/lufuno/smart/Space_UI/crates/space-ui/src:
lib.rs

/home/lufuno/smart/Space_UI/crates/space-ui/ui:
assets
composites
icons
index.slint
primitives
shell
theme
types.slint

/home/lufuno/smart/Space_UI/crates/space-ui/ui/assets:
empty.png
fonts
icons

/home/lufuno/smart/Space_UI/crates/space-ui/ui/assets/fonts:
Inter-Variable.ttf
JetBrainsMono-Variable.ttf

/home/lufuno/smart/Space_UI/crates/space-ui/ui/assets/icons:
lucide

/home/lufuno/smart/Space_UI/crates/space-ui/ui/assets/icons/lucide:
a-arrow-down.svg
a-arrow-up.svg
a-large-small.svg
accessibility.svg
activity.svg
air-vent.svg
airplay.svg
alarm-clock-check.svg
alarm-clock-minus.svg
alarm-clock-off.svg
alarm-clock-plus.svg
alarm-clock.svg
alarm-smoke.svg
album.svg
align-center-horizontal.svg
align-center-vertical.svg
align-end-horizontal.svg
align-end-vertical.svg
align-horizontal-distribute-center.svg
align-horizontal-distribute-end.svg
align-horizontal-distribute-start.svg
align-horizontal-justify-center.svg
align-horizontal-justify-end.svg
align-horizontal-justify-start.svg
align-horizontal-space-around.svg
align-horizontal-space-between.svg
align-start-horizontal.svg
align-start-vertical.svg
align-vertical-distribute-center.svg
align-vertical-distribute-end.svg
align-vertical-distribute-start.svg
align-vertical-justify-center.svg
align-vertical-justify-end.svg
align-vertical-justify-start.svg
align-vertical-space-around.svg
align-vertical-space-between.svg
ambulance.svg
ampersand.svg
ampersands.svg
amphora.svg
anchor.svg
angry.svg
annoyed.svg
antenna.svg
anvil.svg
aperture.svg
app-window-mac.svg
app-window.svg
apple.svg
archive-restore.svg
archive-x.svg
archive.svg
armchair.svg
arrow-big-down-dash.svg
arrow-big-down.svg
arrow-big-left-dash.svg
arrow-big-left.svg
arrow-big-right-dash.svg
arrow-big-right.svg
arrow-big-up-dash.svg
arrow-big-up.svg
arrow-down-0-1.svg
arrow-down-1-0.svg
arrow-down-a-z.svg
---
ls: cannot access '/tmp/opencode/canonical-photos-audit/': No such file or directory
---
CANONICAL-SPECS.md

```

### TOOL: shell [seq 33] 2026-11-09 13:46:54 status=completed

```json
{
  "command": "ls -R /home/lufuno/smart/smart-suite-official 2>&1 | head -n 150\necho ===SMART-SUITE-APPS===\nls -R /home/lufuno/smart/smart-suite/apps 2>&1 | head -n 400\necho ===SMART-PROTOCOL-RS===\nls -R /home/lufuno/smart/smart-protocol-rs 2>&1 | head -n 200"
}
```

```
/home/lufuno/smart/smart-suite-official:
audits
specs

/home/lufuno/smart/smart-suite-official/audits:
CORE-SPECS.md
UX-SPECS.md

/home/lufuno/smart/smart-suite-official/specs:
CANONICAL-SPECS.md
===SMART-SUITE-APPS===
/home/lufuno/smart/smart-suite/apps:
build-production-rust-file-manager
build-production-rust-file-manager-2-very-good.zip
build-production-rust-file-manager-2.zip
build-production-rust-file-manager.zip
develop-rust-wasm-file-manager.zip
smart-clipboard

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager:
README.md
index.html
package-lock.json
package.json
scripts
smart-files
vite.config.ts

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager/scripts:
build-appimage.sh
gen_thumbs.py

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager/smart-files:
Cargo.lock
Cargo.toml
README.md
Smart-Files-x86_64.AppImage
assets
crates
target
tools
web

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager/smart-files/assets:
audio
fonts
icon-256.png
icon_names.txt
icons
thumbs
tokens.toml
wallpaper.jpg
wallpaper_blur.jpg
wallpaper_dusk.jpg
wallpaper_dusk_blur.jpg
wallpaper_graphite.jpg
wallpaper_graphite_blur.jpg
wallpaper_meadow.jpg
wallpaper_meadow_blur.jpg

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager/smart-files/assets/audio:
bass_deep.wav
kick_logdrum.wav
logdrum_conga.wav
pad_warm.wav
piano_lead.wav
shaker.wav

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager/smart-files/assets/fonts:
Inter-bold.otf
Inter-medium.otf
Inter-regular.otf
Inter-semibold.otf

/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager/smart-files/assets/icons:
add-folder_d.png
add-folder_f.png
add-folder_o.png
add_d.png
add_f.png
add_o.png
alt-arrow-down_d.png
alt-arrow-down_f.png
alt-arrow-down_o.png
alt-arrow-left_d.png
alt-arrow-left_f.png
alt-arrow-left_o.png
alt-arrow-right_d.png
alt-arrow-right_f.png
alt-arrow-right_o.png
archive_d.png
archive_f.png
archive_o.png
arrow-right-up_d.png
arrow-right-up_f.png
arrow-right-up_o.png
arrow-up_d.png
arrow-up_f.png
arrow-up_o.png
battery-charge-minimalistic_d.png
battery-charge-minimalistic_f.png
battery-charge-minimalistic_o.png
battery-full-minimalistic_d.png
battery-full-minimalistic_f.png
battery-full-minimalistic_o.png
bell_d.png
bell_f.png
bell_o.png
bluetooth_d.png
bluetooth_f.png
bluetooth_o.png
bolt_d.png
bolt_f.png
bolt_o.png
bookmark_d.png
bookmark_f.png
bookmark_o.png
calendar-date_d.png
calendar-date_f.png
calendar-date_o.png
calendar_d.png
calendar_f.png
calendar_o.png
check-circle_d.png
check-circle_f.png
check-circle_o.png
check-read_d.png
check-read_f.png
check-read_o.png
checklist-minimalistic_d.png
checklist-minimalistic_f.png
checklist-minimalistic_o.png
clapperboard_d.png
clapperboard_f.png
clapperboard_o.png
clipboard_d.png
clipboard_f.png
clipboard_o.png
clock-circle_d.png
clock-circle_f.png
clock-circle_o.png
close-circle_d.png
close-circle_f.png
close-circle_o.png
close-square_d.png
close-square_f.png
close-square_o.png
cloud-download_d.png
cloud-download_f.png
cloud-download_o.png
cloud-storage_d.png
cloud-storage_f.png
cloud-storage_o.png
cloud-upload_d.png
cloud-upload_f.png
cloud-upload_o.png
cloud_d.png
cloud_f.png
cloud_o.png
code-square_d.png
code-square_f.png
code-square_o.png
command_d.png
command_f.png
command_o.png
copy_d.png
copy_f.png
copy_o.png
crop-minimalistic_d.png
crop-minimalistic_f.png
crop-minimalistic_o.png
database_d.png
database_f.png
database_o.png
document-text_d.png
document-text_f.png
document-text_o.png
download-minimalistic_d.png
download-minimalistic_f.png
download-minimalistic_o.png
eject_d.png
eject_f.png
eject_o.png
eye_d.png
eye_f.png
eye_o.png
file-check_d.png
file-check_f.png
file-check_o.png
file_d.png
file_f.png
file_o.png
filter_d.png
filter_f.png
filter_o.png
flash-drive_d.png
flash-drive_f.png
flash-drive_o.png
flip-horizontal_d.png
flip-horizontal_f.png
flip-horizontal_o.png
flip-vertical_d.png
flip-vertical_f.png
flip-vertical_o.png
folder-check_d.png
folder-check_f.png
folder-check_o.png
folder-open_d.png
folder-open_f.png
folder-open_o.png
folder-with-files_d.png
folder-with-files_f.png
folder-with-files_o.png
folder_d.png
folder_f.png
folder_o.png
full-screen_d.png
full-screen_f.png
full-screen_o.png
gallery-wide_d.png
gallery-wide_f.png
gallery-wide_o.png
gallery_d.png
gallery_f.png
gallery_o.png
ghost_d.png
ghost_f.png
ghost_o.png
global_d.png
global_f.png
global_o.png
hamburger-menu_d.png
hamburger-menu_f.png
hamburger-menu_o.png
heart_d.png
heart_f.png
heart_o.png
history_d.png
history_f.png
history_o.png
home_d.png
home_f.png
home_o.png
inbox-in_d.png
inbox-in_f.png
inbox-in_o.png
info-circle_d.png
info-circle_f.png
info-circle_o.png
keyboard_d.png
keyboard_f.png
keyboard_o.png
laptop_d.png
laptop_f.png
laptop_o.png
layers-minimalistic_d.png
layers-minimalistic_f.png
layers-minimalistic_o.png
layout_d.png
layout_f.png
layout_o.png
link_d.png
link_f.png
link_o.png
list-check_d.png
list-check_f.png
list-check_o.png
lock_d.png
lock_f.png
lock_o.png
magic-stick-3_d.png
magic-stick-3_f.png
magic-stick-3_o.png
magnifer-zoom-in_d.png
magnifer-zoom-in_f.png
magnifer-zoom-in_o.png
magnifer_d.png
magnifer_f.png
magnifer_o.png
maximize-square-minimalistic_d.png
maximize-square-minimalistic_f.png
maximize-square-minimalistic_o.png
maximize_d.png
maximize_f.png
maximize_o.png
menu-dots_d.png
menu-dots_f.png
menu-dots_o.png
minimize-square-minimalistic_d.png
minimize-square-minimalistic_f.png
minimize-square-minimalistic_o.png
minimize_d.png
minimize_f.png
minimize_o.png
monitor_d.png
monitor_f.png
monitor_o.png
moon-stars_d.png
moon-stars_f.png
moon-stars_o.png
music-note_d.png
music-note_f.png
music-note_o.png
music-notes_d.png
music-notes_f.png
music-notes_o.png
palette_d.png
palette_f.png
palette_o.png
pause_d.png
pause_f.png
pause_o.png
pen_d.png
pen_f.png
pen_o.png
pip_d.png
pip_f.png
pip_o.png
play_d.png
play_f.png
play_o.png
plug-circle_d.png
plug-circle_f.png
plug-circle_o.png
posts-carousel-horizontal_d.png
posts-carousel-horizontal_f.png
posts-carousel-horizontal_o.png
power_d.png
power_f.png
power_o.png
refresh_d.png
refresh_f.png
refresh_o.png
restart_d.png
restart_f.png
restart_o.png
rotate-left_d.png
rotate-left_f.png
rotate-left_o.png
round-alt-arrow-left_d.png
round-alt-arrow-left_f.png
round-alt-arrow-left_o.png
scissors_d.png
scissors_f.png
scissors_o.png
sd-card_d.png
sd-card_f.png
sd-card_o.png
server-square_d.png
server-square_f.png
server-square_o.png
server_d.png
server_f.png
server_o.png
settings-minimalistic_d.png
settings-minimalistic_f.png
settings-minimalistic_o.png
share_d.png
share_f.png
share_o.png
sidebar-minimalistic_d.png
sidebar-minimalistic_f.png
sidebar-minimalistic_o.png
slider-horizontal_d.png
slider-horizontal_f.png
slider-horizontal_o.png
slider-vertical_d.png
slider-vertical_f.png
slider-vertical_o.png
smartphone_d.png
smartphone_f.png
smartphone_o.png
sort-from-top-to-bottom_d.png
sort-from-top-to-bottom_f.png
sort-from-top-to-bottom_o.png
sort-vertical_d.png
sort-vertical_f.png
sort-vertical_o.png
square-2_d.png
square-2_f.png
square-2_o.png
ssd-square_d.png
ssd-square_f.png
ssd-square_o.png
star_d.png
star_f.png
star_o.png
stars-minimalistic_d.png
stars-minimalistic_f.png
stars-minimalistic_o.png
sun-2_d.png
sun-2_f.png
sun-2_o.png
tag_d.png
tag_f.png
tag_o.png
terminal_d.png
terminal_f.png
terminal_o.png
text_d.png
text_f.png
text_o.png
trash-bin-minimalistic_d.png
trash-bin-minimalistic_f.png
trash-bin-minimalistic_o.png
trash-bin-trash_d.png
trash-bin-trash_f.png
trash-bin-trash_o.png
tuning-2_d.png
tuning-2_f.png
tuning-2_o.png
===SMART-PROTOCOL-RS===
/home/lufuno/smart/smart-protocol-rs:
Cargo.lock
Cargo.toml
LICENSE-APACHE
LICENSE-MIT
README.md
build.rs
codegen
docs
schema
scripts
src
target

/home/lufuno/smart/smart-protocol-rs/codegen:
abi_capnp.bin
access_capnp.bin
accessibility_capnp.bin
ai_capnp.bin
app_capnp.bin
appindex_capnp.bin
archive_capnp.bin
audio_capnp.bin
auth_
```

### TOOL: shell [seq 40] 2026-11-09 13:46:56 status=completed

```json
{
  "command": "mkdir -p /tmp/opencode/canonical-photos-audit && ls /home/lufuno/smart/smart-suite/apps/*.zip\nunzip -l /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip | head -n 100\necho ===2===\nunzip -l /home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip | head -n 100\necho ===3===\nunzip -l /home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip | head -n 100\necho ===4===\nunzip -l /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip | head -n 150"
}
```

```
/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip
/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2.zip
/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager.zip
/home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip
Archive:  /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
 14547419  11-04-2026 20:44   index.html
    85602  11-04-2026 20:44   package-lock.json
      629  11-04-2026 20:44   package.json
        0  11-04-2026 20:44   smart-files/
   102378  11-04-2026 20:44   smart-files/Cargo.lock
      356  11-04-2026 20:44   smart-files/Cargo.toml
     1724  11-04-2026 20:44   smart-files/README.md
        0  11-04-2026 20:44   smart-files/assets/
     1807  11-04-2026 20:44   smart-files/assets/icon_names.txt
     6254  11-04-2026 20:44   smart-files/assets/tokens.toml
        0  11-04-2026 20:44   smart-files/crates/
        0  11-04-2026 20:44   smart-files/crates/smart-core/
     1311  11-04-2026 20:44   smart-files/crates/smart-core/Cargo.toml
        0  11-04-2026 20:44   smart-files/crates/smart-core/src/
     8981  11-04-2026 20:44   smart-files/crates/smart-core/src/apps.rs
    18331  11-04-2026 20:44   smart-files/crates/smart-core/src/archive.rs
     6596  11-04-2026 20:44   smart-files/crates/smart-core/src/audio.rs
    26241  11-04-2026 20:44   smart-files/crates/smart-core/src/backend.rs
    10697  11-04-2026 20:44   smart-files/crates/smart-core/src/demo.rs
    17378  11-04-2026 20:44   smart-files/crates/smart-core/src/devices.rs
      428  11-04-2026 20:44   smart-files/crates/smart-core/src/exec.rs
     1146  11-04-2026 20:44   smart-files/crates/smart-core/src/lib.rs
     6223  11-04-2026 20:44   smart-files/crates/smart-core/src/library.rs
     3559  11-04-2026 20:44   smart-files/crates/smart-core/src/local.rs
     6592  11-04-2026 20:44   smart-files/crates/smart-core/src/memfs.rs
    11555  11-04-2026 20:44   smart-files/crates/smart-core/src/model.rs
     8838  11-04-2026 20:44   smart-files/crates/smart-core/src/ops.rs
      783  11-04-2026 20:44   smart-files/crates/smart-core/src/registry.rs
    21582  11-04-2026 20:44   smart-files/crates/smart-core/src/search.rs
     3184  11-04-2026 20:44   smart-files/crates/smart-core/src/thumbs.rs
     2296  11-04-2026 20:44   smart-files/crates/smart-core/src/tools.rs
     2841  11-04-2026 20:44   smart-files/crates/smart-core/src/vfs.rs
        0  11-04-2026 20:44   smart-files/crates/smart-ui-egui/
      890  11-04-2026 20:44   smart-files/crates/smart-ui-egui/Cargo.toml
        0  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/
    65641  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/app.rs
    33547  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/appkit.rs
    69045  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/chrome.rs
    46532  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/content.rs
    15711  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/copyfx.rs
     2002  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/icons.rs
    34208  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/icons_gen.rs
    20442  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/lib.rs
       64  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/main.rs
    95271  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/modals.rs
    75426  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/overlays.rs
    11031  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/paint.rs
    74978  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/photos.rs
    39324  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/shell.rs
   120099  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/terminal.rs
     8173  11-04-2026 20:44   smart-files/crates/smart-ui-egui/src/theme.rs
        0  11-04-2026 20:44   smart-files/crates/xtask/
      412  11-04-2026 20:44   smart-files/crates/xtask/Cargo.toml
        0  11-04-2026 20:44   smart-files/crates/xtask/src/
    13629  11-04-2026 20:44   smart-files/crates/xtask/src/main.rs
        0  11-04-2026 20:44   smart-files/target/
     2479  11-04-2026 20:44   smart-files/target/.rustc_info.json
      177  11-04-2026 20:44   smart-files/target/CACHEDIR.TAG
        0  11-04-2026 20:44   smart-files/target/debug/
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/ahash-deb52ff0833c972a/
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/ahash-deb52ff0833c972a/build-script-build-script-build
      607  11-04-2026 20:44   smart-files/target/debug/.fingerprint/ahash-deb52ff0833c972a/build-script-build-script-build.json
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/ahash-deb52ff0833c972a/invoked.timestamp
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/autocfg-223609aed98a3323/
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/autocfg-223609aed98a3323/invoked.timestamp
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/autocfg-223609aed98a3323/lib-autocfg
      349  11-04-2026 20:44   smart-files/target/debug/.fingerprint/autocfg-223609aed98a3323/lib-autocfg.json
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bumpalo-f686dafad574c62e/
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bumpalo-f686dafad574c62e/invoked.timestamp
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bumpalo-f686dafad574c62e/lib-bumpalo
      483  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bumpalo-f686dafad574c62e/lib-bumpalo.json
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bytemuck_derive-e3761598712ef4a1/
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bytemuck_derive-e3761598712ef4a1/invoked.timestamp
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bytemuck_derive-e3761598712ef4a1/lib-bytemuck_derive
      537  11-04-2026 20:44   smart-files/target/debug/.fingerprint/bytemuck_derive-e3761598712ef4a1/lib-bytemuck_derive.json
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/crc32fast-bff11abd8bcaf3d7/
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/crc32fast-bff11abd8bcaf3d7/build-script-build-script-build
      424  11-04-2026 20:44   smart-files/target/debug/.fingerprint/crc32fast-bff11abd8bcaf3d7/build-script-build-script-build.json
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/crc32fast-bff11abd8bcaf3d7/invoked.timestamp
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/document-features-d8d57450e3c44a4d/
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/document-features-d8d57450e3c44a4d/invoked.timestamp
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/document-features-d8d57450e3c44a4d/lib-document_features
      461  11-04-2026 20:44   smart-files/target/debug/.fingerprint/document-features-d8d57450e3c44a4d/lib-document_features.json
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/litrs-ee8bf3f4d07d921a/
       48  11-04-2026 20:44   smart-files/target/debug/.fingerprint/litrs-ee8bf3f4d07d921a/invoked.timestamp
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/litrs-ee8bf3f4d07d921a/lib-litrs
      396  11-04-2026 20:44   smart-files/target/debug/.fingerprint/litrs-ee8bf3f4d07d921a/lib-litrs.json
        0  11-04-2026 20:44   smart-files/target/debug/.fingerprint/mime_guess-ed1b151bbe5e09b0/
       16  11-04-2026 20:44   smart-files/target/debug/.fingerprint/mime_guess-ed1b151bbe5e09b0/build-script-build-script-build
      488  11-04-2026 20:44   smart-files/target/debug/.fingerprint/mime_guess-ed1b151bbe5e09b0/build-script-build-script-build.json
       
```

### TOOL: shell [seq 47] 2026-11-09 13:46:58 status=completed

```json
{
  "command": "mkdir -p /tmp/opencode/canonical-photos-audit\nfind /home/lufuno -maxdepth 6 -type f -name \"*.rs\" -newermt \"2026-11-02\" 2>/dev/null | head -n 100\necho ===LIBRARY===\nfind /home/lufuno -name \"library.rs\" -o -name \"analyze.rs\" -o -name \"thumbs.rs\" -o -name \"highlight.rs\" -o -name \"storage.rs\" 2>/dev/null | head -n 80\necho ===PHOTOS===\nfind /home/lufuno -name \"*photos*\" 2>/dev/null | grep -v \".cache\\|.cargo\\|target/\" | head -n 80"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/xtask/src/main.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/audio.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/apps.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/devices.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/search.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/tools.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/memfs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/archive.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/registry.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/vfs.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/ops.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/lib.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/local.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/exec.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/storage.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/terminal.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/copyfx.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/photos.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/icons_gen.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/pty.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/main.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/icons.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/termlive.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/theme.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/paint.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/fx.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/voice.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/params.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/engine.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/sequencer.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/dsp.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/presets.rs
/home/lufuno/smart/bantu-labs/crates/dugout-core/src/lib.rs
/home/lufuno/smart/bantu-labs/crates/dugout-cli/src/main.rs
/home/lufuno/target/debug/build/serde_core-f46aea374eb01030/out/private.rs
/home/lufuno/target/debug/build/serde_core-19f1974a5068c3a9/out/private.rs
/home/lufuno/target/debug/build/khronos_api-2d0fbb49bb8cff1c/out/webgl_exts.rs
/home/lufuno/target/debug/build/unicode-general-category-bb3ee87d9d80cd3e/out/category.rs
/home/lufuno/target/debug/build/serde_core-db1005b1ecbefd29/out/private.rs
/home/lufuno/target/debug/build/glutin_egl_sys-4f845f1e0f61fa34/out/egl_bindings.rs
/home/lufuno/target/debug/build/serde-21774e4fcadce38f/out/private.rs
/home/lufuno/target/debug/build/serde-ee8c85693be09282/out/private.rs
/home/lufuno/target/debug/build/glutin_glx_sys-9a8456d5f5f65740/out/glx_extra_bindings.rs
/home/lufuno/target/debug/build/glutin_glx_sys-9a8456d5f5f65740/out/glx_bindings.rs
/home/lufuno/target/debug/build/x11-dl-4b557a979bb91750/out/config.rs
/home/lufuno/target/debug/build/thiserror-ad88972956ebfdb6/out/private.rs
/home/lufuno/target/debug/build/mime_guess-7f73ec8606f950c7/out/mime_types_generated.rs
/home/lufuno/target/debug/build/serde_core-0142cd4fb8f47b8c/out/private.rs
/home/lufuno/target/release/build/serde_core-986d1363f1e96356/out/private.rs
/home/lufuno/target/release/build/serde_core-1df099a35e2c2130/out/private.rs
/home/lufuno/target/wasm/build/serde_core-2f3b3e1ecb71d414/out/private.rs
/home/lufuno/crates/xtask/src/main.rs
/home/lufuno/crates/smart-core/src/audio.rs
/home/lufuno/crates/smart-core/src/apps.rs
/home/lufuno/crates/smart-core/src/backend.rs
/home/lufuno/crates/smart-core/src/analyze.rs
/home/lufuno/crates/smart-core/src/devices.rs
/home/lufuno/crates/smart-core/src/search.rs
/home/lufuno/crates/smart-core/src/tools.rs
/home/lufuno/crates/smart-core/src/memfs.rs
/home/lufuno/crates/smart-core/src/demo.rs
/home/lufuno/crates/smart-core/src/archive.rs
/home/lufuno/crates/smart-core/src/registry.rs
/home/lufuno/crates/smart-core/src/library.rs
/home/lufuno/crates/smart-core/src/thumbs.rs
/home/lufuno/crates/smart-core/src/vfs.rs
/home/lufuno/crates/smart-core/src/model.rs
/home/lufuno/crates/smart-core/src/ops.rs
/home/lufuno/crates/smart-core/src/lib.rs
/home/lufuno/crates/smart-core/src/local.rs
/home/lufuno/crates/smart-core/src/exec.rs
/home/lufuno/crates/smart-ui-egui/src/shell.rs
/home/lufuno/crates/smart-ui-egui/src/storage.rs
/home/lufuno/crates/smart-ui-egui/src/video.rs
/home/lufuno/crates/smart-ui-egui/src/terminal.rs
/home/lufuno/crates/smart-ui-egui/src/copyfx.rs
/home/lufuno/crates/smart-ui-egui/src/chrome.rs
/home/lufuno/crates/smart-ui-egui/src/app.rs
/home/lufuno/crates/smart-ui-egui/src/photos.rs
/home/lufuno/crates/smart-ui-egui/src/icons_gen.rs
===LIBRARY===
/home/lufuno/Lemon-Studio/crates/os-native/src/library.rs
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/smart-files/crates/smart-core/src/library.rs
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/smart-files/crates/smart-core/src/thumbs.rs
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good/smart-files/crates/smart-core/src/library.rs
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good/smart-files/crates/smart-core/src/thumbs.rs
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/smart-files/crates/smart-core/src/library.rs
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/smart-files/crates/smart-core/src/thumbs.rs
/home/lufuno/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.40.0/src/outline/glyf/hint/storage.rs
/home/lufuno/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/s
```

### TOOL: shell [seq 56] 2026-11-09 13:47:05 status=completed

```json
{
  "command": "ls -la /home/lufuno/crates/smart-core/src/ /home/lufuno/crates/smart-ui-egui/src/ /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/ 2>&1\necho ===WC===\nwc -l /home/lufuno/crates/smart-core/src/*.rs /home/lufuno/crates/smart-ui-egui/src/*.rs 2>&1 | sort -n\necho ===GIT-LOG-OR-MTIME===\nls -lt /home/lufuno/crates/smart-core/src/ /home/lufuno/crates/smart-ui-egui/src/ | head -n 60\necho ===PROTOCOL===\nls /home/lufuno/smart/smart-protocol-rs/schema/*media* /home/lufuno/smart/smart-protocol-rs/schema/*storage* /home/lufuno/smart/smart-protocol-rs/schema/*files* 2>&1\necho ===EIGA===\nfind /home/lufuno -path \"*eiga*\" -name \"*.rs\" 2>/dev/null | grep -v \".cache\\|target\" | head -n 20\nfind /home/lufuno/Documents/rust-wasm-media-manager-latest -type f | head -n 60"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/:
total 240
drwxrwxr-x 2 lufuno lufuno  4096 Nov  6 17:02 .
drwxrwxr-x 3 lufuno lufuno  4096 Nov  6 17:02 ..
-rw-rw-r-- 1 lufuno lufuno 31515 Nov  6 17:02 analyze.rs
-rw-rw-r-- 1 lufuno lufuno  8981 Nov  6 17:02 apps.rs
-rw-rw-r-- 1 lufuno lufuno 18331 Nov  6 17:02 archive.rs
-rw-rw-r-- 1 lufuno lufuno  6596 Nov  6 17:02 audio.rs
-rw-rw-r-- 1 lufuno lufuno 30667 Nov  6 17:02 backend.rs
-rw-rw-r-- 1 lufuno lufuno 13303 Nov  6 17:02 demo.rs
-rw-rw-r-- 1 lufuno lufuno 17378 Nov  6 17:02 devices.rs
-rw-rw-r-- 1 lufuno lufuno   428 Nov  6 17:02 exec.rs
-rw-rw-r-- 1 lufuno lufuno  1163 Nov  6 17:02 lib.rs
-rw-rw-r-- 1 lufuno lufuno  6223 Nov  6 17:02 library.rs
-rw-rw-r-- 1 lufuno lufuno  3559 Nov  6 17:02 local.rs
-rw-rw-r-- 1 lufuno lufuno  6592 Nov  6 17:02 memfs.rs
-rw-rw-r-- 1 lufuno lufuno 11555 Nov  6 17:02 model.rs
-rw-rw-r-- 1 lufuno lufuno  8838 Nov  6 17:02 ops.rs
-rw-rw-r-- 1 lufuno lufuno   783 Nov  6 17:02 registry.rs
-rw-rw-r-- 1 lufuno lufuno 21582 Nov  6 17:02 search.rs
-rw-rw-r-- 1 lufuno lufuno  3184 Nov  6 17:02 thumbs.rs
-rw-rw-r-- 1 lufuno lufuno  2296 Nov  6 17:02 tools.rs
-rw-rw-r-- 1 lufuno lufuno  2841 Nov  6 17:02 vfs.rs

/home/lufuno/crates/smart-core/src/:
total 240
drwxr-xr-x 2 lufuno lufuno  4096 Nov  6 19:14 .
drwxr-xr-x 3 lufuno lufuno  4096 Nov  6 19:14 ..
-rw-r--r-- 1 lufuno lufuno 31515 Nov  6 19:14 analyze.rs
-rw-r--r-- 1 lufuno lufuno  8981 Nov  6 19:14 apps.rs
-rw-r--r-- 1 lufuno lufuno 18331 Nov  6 19:14 archive.rs
-rw-r--r-- 1 lufuno lufuno  6596 Nov  6 19:14 audio.rs
-rw-r--r-- 1 lufuno lufuno 30667 Nov  6 19:14 backend.rs
-rw-r--r-- 1 lufuno lufuno 13303 Nov  6 19:14 demo.rs
-rw-r--r-- 1 lufuno lufuno 17378 Nov  6 19:14 devices.rs
-rw-r--r-- 1 lufuno lufuno   428 Nov  6 19:14 exec.rs
-rw-r--r-- 1 lufuno lufuno  1163 Nov  6 19:14 lib.rs
-rw-r--r-- 1 lufuno lufuno  6223 Nov  6 19:14 library.rs
-rw-r--r-- 1 lufuno lufuno  3559 Nov  6 19:14 local.rs
-rw-r--r-- 1 lufuno lufuno  6592 Nov  6 19:14 memfs.rs
-rw-r--r-- 1 lufuno lufuno 11555 Nov  6 19:14 model.rs
-rw-r--r-- 1 lufuno lufuno  8838 Nov  6 19:14 ops.rs
-rw-r--r-- 1 lufuno lufuno   783 Nov  6 19:14 registry.rs
-rw-r--r-- 1 lufuno lufuno 21582 Nov  6 19:14 search.rs
-rw-r--r-- 1 lufuno lufuno  3184 Nov  6 19:14 thumbs.rs
-rw-r--r-- 1 lufuno lufuno  2296 Nov  6 19:14 tools.rs
-rw-r--r-- 1 lufuno lufuno  2841 Nov  6 19:14 vfs.rs

/home/lufuno/crates/smart-ui-egui/src/:
total 936
drwxr-xr-x 2 lufuno lufuno   4096 Nov  6 19:14 .
drwxr-xr-x 3 lufuno lufuno   4096 Nov  6 19:14 ..
-rw-r--r-- 1 lufuno lufuno  67728 Nov  6 19:14 app.rs
-rw-r--r-- 1 lufuno lufuno  34922 Nov  6 19:14 appkit.rs
-rw-r--r-- 1 lufuno lufuno  72457 Nov  6 19:14 chrome.rs
-rw-r--r-- 1 lufuno lufuno  46686 Nov  6 19:14 content.rs
-rw-r--r-- 1 lufuno lufuno  15711 Nov  6 19:14 copyfx.rs
-rw-r--r-- 1 lufuno lufuno  13912 Nov  6 19:14 highlight.rs
-rw-r--r-- 1 lufuno lufuno   2002 Nov  6 19:14 icons.rs
-rw-r--r-- 1 lufuno lufuno  40466 Nov  6 19:14 icons_gen.rs
-rw-r--r-- 1 lufuno lufuno  20550 Nov  6 19:14 lib.rs
-rw-r--r-- 1 lufuno lufuno     64 Nov  6 19:14 main.rs
-rw-r--r-- 1 lufuno lufuno 101498 Nov  6 19:14 modals.rs
-rw-r--r-- 1 lufuno lufuno  75723 Nov  6 19:14 overlays.rs
-rw-r--r-- 1 lufuno lufuno  11031 Nov  6 19:14 paint.rs
-rw-r--r-- 1 lufuno lufuno  74978 Nov  6 19:14 photos.rs
-rw-r--r-- 1 lufuno lufuno  15739 Nov  6 19:14 pty.rs
-rw-r--r-- 1 lufuno lufuno  42020 Nov  6 19:14 shell.rs
-rw-r--r-- 1 lufuno lufuno  54375 Nov  6 19:14 storage.rs
-rw-r--r-- 1 lufuno lufuno 121900 Nov  6 19:14 terminal.rs
-rw-r--r-- 1 lufuno lufuno  15941 Nov  6 19:14 termlive.rs
-rw-r--r-- 1 lufuno lufuno   8173 Nov  6 19:14 theme.rs
-rw-r--r-- 1 lufuno lufuno  76313 Nov  6 19:14 video.rs
===WC===
      3 /home/lufuno/crates/smart-ui-egui/src/main.rs
     17 /home/lufuno/crates/smart-core/src/exec.rs
     27 /home/lufuno/crates/smart-core/src/registry.rs
     38 /home/lufuno/crates/smart-core/src/lib.rs
     52 /home/lufuno/crates/smart-ui-egui/src/icons.rs
     60 /home/lufuno/crates/smart-core/src/tools.rs
     90 /home/lufuno/crates/smart-core/src/vfs.rs
     91 /home/lufuno/crates/smart-core/src/thumbs.rs
     98 /home/lufuno/crates/smart-core/src/local.rs
    190 /home/lufuno/crates/smart-core/src/audio.rs
    191 /home/lufuno/crates/smart-core/src/memfs.rs
    192 /home/lufuno/crates/smart-core/src/library.rs
    215 /home/lufuno/crates/smart-core/src/demo.rs
    221 /home/lufuno/crates/smart-ui-egui/src/theme.rs
    222 /home/lufuno/crates/smart-core/src/apps.rs
    237 /home/lufuno/crates/smart-core/src/ops.rs
    243 /home/lufuno/crates/smart-ui-egui/src/copyfx.rs
    249 /home/lufuno/crates/smart-ui-egui/src/paint.rs
    300 /home/lufuno/crates/smart-ui-egui/src/highlight.rs
    309 /home/lufuno/crates/smart-ui-egui/src/termlive.rs
    322 /home/lufuno/crates/smart-core/src/model.rs
    391 /home/lufuno/crates/smart-core/src/devices.rs
    393 /home/lufuno/crates/smart-ui-egui/src/pty.rs
    444 /home/lufuno/crates/smart-ui-egui/src/icons_gen.rs
    460 /home/lufuno/crates/smart-core/src/archive.rs
    509 /home/lufuno/crates/smart-ui-egui/src/lib.rs
    537 /home/lufuno/crates/smart-core/src/search.rs
    610 /home/lufuno/crates/smart-ui-egui/src/appkit.rs
    714 /home/lufuno/crates/smart-core/src/backend.rs
    746 /home/lufuno/crates/smart-core/src/analyze.rs
    845 /home/lufuno/crates/smart-ui-egui/src/shell.rs
    901 /home/lufuno/crates/smart-ui-egui/src/content.rs
   1039 /home/lufuno/crates/smart-ui-egui/src/storage.rs
   1279 /home/lufuno/crates/smart-ui-egui/src/overlays.rs
   1370 /home/lufuno/crates/smart-ui-egui/src/chrome.rs
   1467 /home/lufuno/crates/smart-ui-egui/src/photos.rs
   1523 /home/lufuno/crates/smart-ui-egui/src/video.rs
   1591 /home/lufuno/crates/smart-ui-egui/src/modals.rs
   1794 /home/lufuno/crates/smart-ui-egui/src/app.rs
   2401 /home/lufuno/crates/smart-ui-egui/src/terminal.rs
  22381 total
===GIT-LOG-OR-MTIME===
/home/lufuno/crates/smart-ui-egui/src/:
total 928
-rw-r--r-- 1 lufuno lufuno  76313 Nov  6 19:14 video.rs
-rw-r--r-- 1 lufuno lufuno  74978 Nov  6 19:14 photos.rs
-rw-r--r-- 1 lufuno lufuno  15739 Nov  6 19:14 pty.rs
-rw-r--r-- 1 lufuno lufuno  42020 Nov  6 19:14 shell.rs
-rw-r--r-- 1 lufuno lufuno  54375 Nov  6 19:14 storage.rs
-rw-r--r-- 1 lufuno lufuno 121900 Nov  6 19:14 terminal.rs
-rw-r--r-- 1 lufuno lufuno  15941 Nov  6 19:14 termlive.rs
-rw-r--r-- 1 lufuno lufuno   8173 Nov  6 19:14 theme.rs
-rw-r--r-- 1 lufuno lufuno  13912 Nov  6 19:14 highlight.rs
-rw-r--r-- 1 lufuno lufuno   2002 Nov  6 19:14 icons.rs
-rw-r--r-- 1 lufuno lufuno  40466 Nov  6 19:14 icons_gen.rs
-rw-r--r-- 1 lufuno lufuno  20550 Nov  6 19:14 lib.rs
-rw-r--r-- 1 lufuno lufuno     64 Nov  6 19:14 main.rs
-rw-r--r-- 1 lufuno lufuno 101498 Nov  6 19:14 modals.rs
-rw-r--r-- 1 lufuno lufuno  75723 Nov  6 19:14 overlays.rs
-rw-r--r-- 1 lufuno lufuno  11031 Nov  6 19:14 paint.rs
-rw-r--r-- 1 lufuno lufuno  72457 Nov  6 19:14 chrome.rs
-rw-r--r-- 1 lufuno lufuno  46686 Nov  6 19:14 content.rs
-rw-r--r-- 1 lufuno lufuno  15711 Nov  6 19:14 copyfx.rs
-rw-r--r-- 1 lufuno lufuno  67728 Nov  6 19:14 app.rs
-rw-r--r-- 1 lufuno lufuno  34922 Nov  6 19:14 appkit.rs

/home/lufuno/crates/smart-core/src/:
total 232
-rw-r--r-- 1 lufuno lufuno  3559 Nov  6 19:14 local.rs
-rw-r--r-- 1 lufuno lufuno  6592 Nov  6 19:14 memfs.rs
-rw-r--r-- 1 lufuno lufuno 11555 Nov  6 19:14 model.rs
-rw-r--r-- 1 lufuno lufuno  8838 Nov  6 19:14 ops.rs
-rw-r--r-- 1 lufuno lufuno   783 Nov  6 19:14 registry.rs
-rw-r--r-- 1 lufuno lufuno 21582 Nov  6 19:14 search.rs
-rw-r--r-- 1 lufuno lufuno  3184 Nov  6 19:14 thumbs.rs
-rw-r--r-- 1 lufuno lufuno  2296 Nov  6 19:14 tools.rs
-rw-r--r-- 1 lufuno lufuno  2841 Nov  6 19:14 vfs.rs
-rw-r--r-- 1 lufuno lufuno 31515 Nov  6 19:14 analyze.rs
-rw-r--r-- 1 lufuno lufuno  8981 Nov  6 19:14 apps.rs
-rw-r--r-- 1 lufuno lufuno 18331 Nov  6 19:14 archive.rs
-rw-r--r-- 1 lufuno lufuno  6596 Nov  6 19:14 audio.
```

### REASONING [seq 63] 2026-11-09 13:47:10

Mapping canonical crate paths to a /tmp audit workspace and comparing file versions across zips and local directories.

### TOOL: shell [seq 63] 2026-11-09 13:47:10 status=completed

```json
{
  "command": "mkdir -p /tmp/opencode/canonical-photos-audit/{zip-a1,zip-verygood,zip-wasm,zip-desktop,home-crates,proc}\ncd /tmp/opencode/canonical-photos-audit\nunzip -o -q /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip \"smart-files/crates/smart-ui-egui/src/photos.rs\" \"smart-files/crates/smart-core/src/library.rs\" \"smart-files/crates/smart-core/src/thumbs.rs\" -d zip-a1\nunzip -o -q /home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip \"smart-files/crates/smart-ui-egui/src/photos.rs\" \"smart-files/crates/smart-core/src/library.rs\" \"smart-files/crates/smart-core/src/thumbs.rs\" -d zip-verygood\nunzip -o -q /home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip \"crates/smart-ui-egui/src/photos.rs\" \"crates/smart-core/src/library.rs\" \"crates/smart-core/src/thumbs.rs\" -d zip-wasm\nunzip -o -q /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip \"smart-desktop/apps/photos/mod.rs\" \"smart-desktop/crates/smart-core/src/library.rs\" \"smart-desktop/crates/smart-core/src/thumbs.rs\" \"smart-desktop/apps/files/picker.rs\" -d zip-desktop\ncp /home/lufuno/crates/smart-ui-egui/src/photos.rs home-crates/photos.home.rs\ncp /home/lufuno/crates/smart-ui-egui/src/storage.rs home-crates/storage.home.rs\ncp /home/lufuno/crates/smart-ui-egui/src/highlight.rs home-crates/highlight.home.rs\ncp /home/lufuno/crates/smart-core/src/library.rs home-crates/library.home.rs\ncp /home/lufuno/crates/smart-core/src/thumbs.rs home-crates/thumbs.home.rs\ncp /home/lufuno/crates/smart-core/src/analyze.rs home-crates/analyze.home.rs\ncp /home/lufuno/crates/eiga-core/src/*.rs home-crates/ 2>&1\nls -R | head -n 60\necho ===MD5===\nfind . -type f -exec md5sum {} \\; | sort -k2\necho ===WC===\nwc -c -l /home/lufuno/crates/smart-ui-egui/src/photos.rs /home/lufuno/crates/smart-ui-egui/src/storage.rs /home/lufuno/crates/smart-ui-egui/src/highlight.rs /home/lufuno/crates/smart-core/src/library.rs /home/lufuno/crates/smart-core/src/thumbs.rs /home/lufuno/crates/smart-core/src/analyze.rs /home/lufuno/crates/eiga-core/src/*.rs 2>&1"
}
```

```
.:
home-crates
proc
zip-a1
zip-desktop
zip-verygood
zip-wasm

./home-crates:
analyze.home.rs
highlight.home.rs
lib.rs
library.home.rs
photos.home.rs
player.rs
source.rs
storage.home.rs
subs.rs
thumbs.home.rs

./proc:

./zip-a1:
smart-files

./zip-a1/smart-files:
crates

./zip-a1/smart-files/crates:
smart-core
smart-ui-egui

./zip-a1/smart-files/crates/smart-core:
src

./zip-a1/smart-files/crates/smart-core/src:
library.rs
thumbs.rs

./zip-a1/smart-files/crates/smart-ui-egui:
src

./zip-a1/smart-files/crates/smart-ui-egui/src:
photos.rs

./zip-desktop:
smart-desktop

./zip-desktop/smart-desktop:
apps
crates

./zip-desktop/smart-desktop/apps:
files
photos

./zip-desktop/smart-desktop/apps/files:
picker.rs

./zip-desktop/smart-desktop/apps/photos:
===MD5===
6336b4af39123b640f0ddb7968641974  ./home-crates/analyze.home.rs
d0f77f332751fc6993d052fe3379de70  ./home-crates/highlight.home.rs
1fcb3e7e6b54a70881ed6ba5541cbf57  ./home-crates/lib.rs
18e59884bb1458df96c17dbc24c94b2c  ./home-crates/library.home.rs
8d7738deb67839512a5b18093b9d48db  ./home-crates/photos.home.rs
29d2538ace43c0ad50460f515a5a3584  ./home-crates/player.rs
070e0a4d3f4e7b7ffe964448785f47ba  ./home-crates/source.rs
963f9d660ebf7cef107c1b80442b44ba  ./home-crates/storage.home.rs
a33b47d73dd7703d5fa3a4afef85cca1  ./home-crates/subs.rs
cfd4f201cbbc603bd0c44d0338f11acd  ./home-crates/thumbs.home.rs
18e59884bb1458df96c17dbc24c94b2c  ./zip-a1/smart-files/crates/smart-core/src/library.rs
cfd4f201cbbc603bd0c44d0338f11acd  ./zip-a1/smart-files/crates/smart-core/src/thumbs.rs
8d7738deb67839512a5b18093b9d48db  ./zip-a1/smart-files/crates/smart-ui-egui/src/photos.rs
95aae532407b6ab2ad395f01ee1f95d2  ./zip-desktop/smart-desktop/apps/files/picker.rs
19415a53d0002d65f3ed98ad4fadfc0c  ./zip-desktop/smart-desktop/apps/photos/mod.rs
18e59884bb1458df96c17dbc24c94b2c  ./zip-desktop/smart-desktop/crates/smart-core/src/library.rs
cfd4f201cbbc603bd0c44d0338f11acd  ./zip-desktop/smart-desktop/crates/smart-core/src/thumbs.rs
32be75f53bfda37218a255f29f865b3e  ./zip-verygood/smart-files/crates/smart-core/src/library.rs
44755767c1085d44af02fa2134e70966  ./zip-verygood/smart-files/crates/smart-core/src/thumbs.rs
8d7738deb67839512a5b18093b9d48db  ./zip-verygood/smart-files/crates/smart-ui-egui/src/photos.rs
18e59884bb1458df96c17dbc24c94b2c  ./zip-wasm/crates/smart-core/src/library.rs
cfd4f201cbbc603bd0c44d0338f11acd  ./zip-wasm/crates/smart-core/src/thumbs.rs
8d7738deb67839512a5b18093b9d48db  ./zip-wasm/crates/smart-ui-egui/src/photos.rs
===WC===
  1467  74978 /home/lufuno/crates/smart-ui-egui/src/photos.rs
  1039  54375 /home/lufuno/crates/smart-ui-egui/src/storage.rs
   300  13912 /home/lufuno/crates/smart-ui-egui/src/highlight.rs
   192   6223 /home/lufuno/crates/smart-core/src/library.rs
    91   3184 /home/lufuno/crates/smart-core/src/thumbs.rs
   746  31515 /home/lufuno/crates/smart-core/src/analyze.rs
    92   2991 /home/lufuno/crates/eiga-core/src/lib.rs
   820  30125 /home/lufuno/crates/eiga-core/src/player.rs
   731  30036 /home/lufuno/crates/eiga-core/src/source.rs
   401  14951 /home/lufuno/crates/eiga-core/src/subs.rs
  5879 262290 total

```

### TOOL: shell [seq 72] 2026-11-09 13:47:16 status=completed

```json
{
  "command": "cd /tmp/opencode/canonical-photos-audit\necho ===LIB-VERYGOOD-DIFF===\ndiff -u zip-a1/smart-files/crates/smart-core/src/library.rs zip-verygood/smart-files/crates/smart-core/src/library.rs | head -n 200\necho ===THUMBS-DIFF===\ndiff -u home-crates/thumbs.home.rs zip-verygood/smart-files/crates/smart-core/src/thumbs.rs | head -n 300\necho ===DESKTOP-PHOTOS-STAT===\ndiff --stat home-crates/photos.home.rs zip-desktop/smart-desktop/apps/photos/mod.rs 2>&1 | head -n 20\nwc -l -c zip-desktop/smart-desktop/apps/photos/mod.rs\necho ===PICKER-STAT===\nwc -l -c zip-desktop/smart-desktop/apps/files/picker.rs; head -n 50 zip-desktop/smart-desktop/apps/files/picker.rs"
}
```

```
===LIB-VERYGOOD-DIFF===
--- zip-a1/smart-files/crates/smart-core/src/library.rs	2026-11-04 20:44:48.000000000 +0200
+++ zip-verygood/smart-files/crates/smart-core/src/library.rs	2026-11-05 19:06:16.000000000 +0200
@@ -38,6 +38,28 @@
     pub reduce_motion: bool,
     pub autoplay: bool,
     pub accent: String,
+    /// Single click opens items (double-click otherwise).
+    #[serde(default)]
+    pub single_click: bool,
+    /// Human/natural ordering: file2 before file10.
+    #[serde(default = "default_true")]
+    pub natural_sort: bool,
+    /// Context-aware defaults (e.g. Downloads sorts by newest first).
+    #[serde(default = "default_true")]
+    pub smart_sort: bool,
+    /// Type-ahead accumulates characters to jump through the list.
+    #[serde(default = "default_true")]
+    pub typeahead: bool,
+    /// 0 = ask on conflict, 1 = skip existing, 2 = replace existing.
+    #[serde(default)]
+    pub overwrite: u8,
+    /// Toast when a transfer or extraction finishes.
+    #[serde(default = "default_true")]
+    pub notify_done: bool,
+}
+
+fn default_true() -> bool {
+    true
 }
 
 impl Default for Settings {
@@ -59,6 +81,12 @@
             reduce_motion: false,
             autoplay: false,
             accent: "#2f6feb".into(),
+            single_click: false,
+            natural_sort: true,
+            smart_sort: true,
+            typeahead: true,
+            overwrite: 0,
+            notify_done: true,
         }
     }
 }
===THUMBS-DIFF===
--- home-crates/thumbs.home.rs	2026-11-09 13:47:16.451386227 +0200
+++ zip-verygood/smart-files/crates/smart-core/src/thumbs.rs	2026-11-05 19:06:16.000000000 +0200
@@ -29,12 +29,66 @@
 fn decode(reg: &Registry, loc: &Location, edge: u32) -> Option<ThumbResult> {
     let v = reg.get(&loc.fs)?;
     let bytes = v.read(&loc.path, 48 << 20).ok()?;
-    let img = image::load_from_memory(&bytes).ok()?;
+    let img = match image::load_from_memory(&bytes) {
+        Ok(i) => i,
+        Err(_) => {
+            #[cfg(not(target_arch = "wasm32"))]
+            {
+                use crate::model::{ext_of, Kind};
+                if matches!(Kind::from_ext(&ext_of(&loc.name())), Kind::Video) {
+                    return video_thumb(v.as_ref(), loc, edge);
+                }
+            }
+            return None;
+        }
+    };
     let original = Some((img.width(), img.height()));
     let t = img.thumbnail(edge, edge).to_rgba8();
     Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
 }
 
+/// Pull a frame out of a video with ffmpeg (best-effort) and cache it on disk.
+#[cfg(not(target_arch = "wasm32"))]
+fn video_thumb(v: &dyn crate::vfs::Vfs, loc: &Location, edge: u32) -> Option<ThumbResult> {
+    use std::io::Write;
+    let real = v.real_path(&loc.path)?;
+    let cache_dir = dirs::cache_dir()?.join("smart-files/thumbs");
+    let _ = std::fs::create_dir_all(&cache_dir);
+    let key = format!("{:x}", smart_hash(&real.to_string_lossy()));
+    let cache = cache_dir.join(format!("{key}.png"));
+    let png = if cache.exists() {
+        std::fs::read(&cache).ok()?
+    } else {
+        let out = std::process::Command::new("ffmpeg")
+            .args(["-loglevel", "error", "-ss", "0.7", "-i"])
+            .arg(&real)
+            .args(["-frames:v", "1", "-vf", "scale='min(384,iw)':-2", "-f", "image2pipe", "-vcodec", "png", "-"])
+            .output()
+            .ok()?;
+        if !out.status.success() || out.stdout.is_empty() {
+            return None;
+        }
+        if let mut f = std::fs::File::create(&cache).ok() {
+            let _ = f.write_all(&out.stdout);
+        }
+        out.stdout
+    };
+    let img = image::load_from_memory(&png).ok()?;
+    let original = Some((img.width(), img.height()));
+    let t = img.thumbnail(edge, edge).to_rgba8();
+    Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
+}
+
+#[cfg(not(target_arch = "wasm32"))]
+fn smart_hash(s: &str) -> u64 {
+    let mut h: u64 = 0xcbf29ce484222325;
+    for b in s.bytes() {
+        h ^= b as u64;
+        h = h.wrapping_mul(0x100000001b3);
+    }
+    h
+}
+
 impl Thumbs {
     pub fn new(registry: Arc<Registry>) -> Self {
         let shared = Arc::new(Shared::default());
===DESKTOP-PHOTOS-STAT===
diff: unrecognized option '--stat'
diff: Try 'diff --help' for more information.
 1476 75586 zip-desktop/smart-desktop/apps/photos/mod.rs
===PICKER-STAT===
  233 12482 zip-desktop/smart-desktop/apps/files/picker.rs
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


```

### TOOL: read [seq 83] 2026-11-09 13:47:19 status=completed

```json
{
  "path": "/tmp/opencode/canonical-photos-audit/home-crates/library.home.rs"
}
```

```
Read file /tmp/opencode/canonical-photos-audit/home-crates/library.home.rs, lines 1-192
1: //! User-owned metadata: tags, recents, pinned places and preferences. Serialised as JSON.
2: use crate::model::{now_secs, Location, SortKey};
3: use serde::{Deserialize, Serialize};
4: use std::collections::BTreeMap;
5: use std::sync::{Arc, RwLock};
6: 
7: pub type SharedLibrary = Arc<RwLock<Library>>;
8: 
9: #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
10: pub struct TagDef {
11:     pub id: String,
12:     pub name: String,
13:     pub color: String, // #rrggbb
14: }
15: 
16: #[derive(Clone, Debug, Serialize, Deserialize)]
17: pub struct Recent {
18:     pub loc: Location,
19:     pub at: i64,
20: }
21: 
22: #[derive(Clone, Debug, Serialize, Deserialize)]
23: #[serde(default)]
24: pub struct Settings {
25:     pub dark: bool,
26:     pub grid: bool,
27:     pub sort: SortKey,
28:     pub ascending: bool,
29:     pub show_hidden: bool,
30:     pub folders_first: bool,
31:     pub sidebar_collapsed: bool,
32:     pub wallpaper: usize,
33:     pub search_contents: bool,
34:     pub thumb_scale: f32,
35:     pub show_extensions: bool,
36:     pub confirm_delete: bool,
37:     pub copy_animation_secs: f32,
38:     pub reduce_motion: bool,
39:     pub autoplay: bool,
40:     pub accent: String,
41: }
42: 
43: impl Default for Settings {
44:     fn default() -> Self {
45:         Self {
46:             dark: false,
47:             grid: true,
48:             sort: SortKey::Name,
49:             ascending: true,
50:             show_hidden: false,
51:             folders_first: true,
52:             sidebar_collapsed: false,
53:             wallpaper: 0,
54:             search_contents: false,
55:             thumb_scale: 1.0,
56:             show_extensions: true,
57:             confirm_delete: true,
58:             copy_animation_secs: 5.0,
59:             reduce_motion: false,
60:             autoplay: false,
61:             accent: "#2f6feb".into(),
62:         }
63:     }
64: }
65: 
66: #[derive(Clone, Debug, Serialize, Deserialize)]
67: #[serde(default)]
68: pub struct Library {
69:     pub tags: Vec<TagDef>,
70:     pub assignments: BTreeMap<String, Vec<String>>,
71:     pub recents: Vec<Recent>,
72:     pub pins: Vec<Location>,
73:     pub settings: Settings,
74: }
75: 
76: impl Default for Library {
77:     fn default() -> Self {
78:         Self {
79:             tags: vec![
80:                 TagDef { id: "design".into(), name: "Design".into(), color: "#2f6feb".into() },
81:                 TagDef { id: "amapiano".into(), name: "Amapiano".into(), color: "#c9861a".into() },
82:                 TagDef { id: "archive".into(), name: "Archive".into(), color: "#2b9b6d".into() },
83:             ],
84:             assignments: BTreeMap::new(),
85:             recents: vec![],
86:             pins: vec![],
87:             settings: Settings::default(),
88:         }
89:     }
90: }
91: 
92: fn key(l: &Location) -> String {
93:     format!("{}:{}", l.fs, l.path)
94: }
95: 
96: impl Library {
97:     pub fn shared(self) -> SharedLibrary {
98:         Arc::new(RwLock::new(self))
99:     }
100:     pub fn tags_of(&self, l: &Location) -> Vec<&TagDef> {
101:         self.assignments
102:             .get(&key(l))
103:             .map(|ids| ids.iter().filter_map(|i| self.tags.iter().find(|t| &t.id == i)).collect())
104:             .unwrap_or_default()
105:     }
106:     pub fn tag_ids_of(&self, l: &Location) -> Vec<String> {
107:         self.assignments.get(&key(l)).cloned().unwrap_or_default()
108:     }
109:     pub fn has_tag(&self, l: &Location, id: &str) -> bool {
110:         self.assignments.get(&key(l)).is_some_and(|v| v.iter().any(|t| t == id))
111:     }
112:     pub fn toggle_tag(&mut self, l: &Location, id: &str) {
113:         let e = self.assignments.entry(key(l)).or_default();
114:         if let Some(i) = e.iter().position(|t| t == id) {
115:             e.remove(i);
116:         } else {
117:             e.push(id.to_string());
118:         }
119:         if e.is_empty() {
120:             self.assignments.remove(&key(l));
121:         }
122:     }
123:     pub fn tag_count(&self, id: &str) -> usize {
124:         self.assignments.values().filter(|v| v.iter().any(|t| t == id)).count()
125:     }
126:     pub fn tagged(&self, id: &str) -> Vec<Location> {
127:         self.assignments
128:             .iter()
129:             .filter(|(_, v)| v.iter().any(|t| t == id))
130:             .filter_map(|(k, _)| k.split_once(':').map(|(fs, p)| Location::new(fs, p)))
131:             .collect()
132:     }
133:     pub fn add_tag(&mut self, name: &str, color: &str) -> String {
134:         let id = name.trim().to_lowercase().replace(' ', "-");
135:         if !self.tags.iter().any(|t| t.id == id) {
136:             self.tags.push(TagDef { id: id.clone(), name: name.trim().to_string(), color: color.to_string() });
137:         }
138:         id
139:     }
140:     /// Keep tags attached when files move or are renamed.
141:     pub fn rekey(&mut self, from: &Location, to: &Location) {
142:         let prefix = format!("{}/", key(from));
143:         let moves: Vec<(String, String)> = self
144:             .assignments
145:             .keys()
146:             .filter(|k| **k == key(from) || k.starts_with(&prefix))
147:             .map(|k| (k.clone(), format!("{}{}", key(to), &k[key(from).len()..])))
148:             .collect();
149:         for (a, b) in moves {
150:             if let Some(v) = self.assignments.remove(&a) {
151:                 self.assignments.insert(b, v);
152:             }
153:         }
154:     }
155:     pub fn touch_recent(&mut self, l: &Location) {
156:         self.recents.retain(|r| &r.loc != l);
157:         self.recents.insert(0, Recent { loc: l.clone(), at: now_secs() });
158:         self.recents.truncate(60);
159:     }
160:     pub fn is_pinned(&self, l: &Location) -> bool {
161:         self.pins.contains(l)
162:     }
163:     pub fn toggle_pin(&mut self, l: &Location) {
164:         if let Some(i) = self.pins.iter().position(|p| p == l) {
165:             self.pins.remove(i);
166:         } else {
167:             self.pins.push(l.clone());
168:         }
169:     }
170: 
171:     pub fn to_json(&self) -> String {
172:         serde_json::to_string_pretty(self).unwrap_or_default()
173:     }
174:     pub fn from_json(s: &str) -> Library {
175:         serde_json::from_str(s).unwrap_or_default()
176:     }
177:     #[cfg(not(target_arch = "wasm32"))]
178:     pub fn path() -> Option<std::path::PathBuf> {
179:         dirs::config_dir().map(|d| d.join("smart-files").join("library.json"))
180:     }
181:     #[cfg(not(target_arch = "wasm32"))]
182:     pub fn load() -> Library {
183:         Self::path().and_then(|p| std::fs::read_to_string(p).ok()).map(|s| Self::from_json(&s)).unwrap_or_default()
184:     }
185:     #[cfg(not(target_arch = "wasm32"))]
186:     pub fn save(&self) {
187:         if let Some(p) = Self::path() {
188:             let _ = std::fs::create_dir_all(p.parent().unwrap());
189:             let _ = std::fs::write(p, self.to_json());
190:         }
191:     }
192: }
```

### TOOL: read [seq 83] 2026-11-09 13:47:19 status=completed

```json
{
  "path": "/tmp/opencode/canonical-photos-audit/home-crates/thumbs.home.rs"
}
```

```
Read file /tmp/opencode/canonical-photos-audit/home-crates/thumbs.home.rs, lines 1-91
1: //! Background thumbnail decoding (image crate). Frontends upload the RGBA to a texture.
2: use crate::exec::THREADED;
3: use crate::model::Location;
4: use crate::ops::Registry;
5: use std::collections::HashSet;
6: use std::sync::{Arc, Condvar, Mutex};
7: 
8: pub struct ThumbResult {
9:     pub loc: Location,
10:     pub size: [usize; 2],
11:     pub rgba: Vec<u8>,
12:     pub original: Option<(u32, u32)>,
13:     pub edge: u32,
14: }
15: 
16: #[derive(Default)]
17: struct Shared {
18:     queue: Mutex<Vec<(Location, u32)>>,
19:     cv: Condvar,
20:     out: Mutex<Vec<ThumbResult>>,
21: }
22: 
23: pub struct Thumbs {
24:     shared: Arc<Shared>,
25:     registry: Arc<Registry>,
26:     seen: Mutex<HashSet<(Location, u32)>>,
27: }
28: 
29: fn decode(reg: &Registry, loc: &Location, edge: u32) -> Option<ThumbResult> {
30:     let v = reg.get(&loc.fs)?;
31:     let bytes = v.read(&loc.path, 48 << 20).ok()?;
32:     let img = image::load_from_memory(&bytes).ok()?;
33:     let original = Some((img.width(), img.height()));
34:     let t = img.thumbnail(edge, edge).to_rgba8();
35:     Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
36: }
37: 
38: impl Thumbs {
39:     pub fn new(registry: Arc<Registry>) -> Self {
40:         let shared = Arc::new(Shared::default());
41:         if THREADED {
42:             let n = 2;
43:             for _ in 0..n {
44:                 let (s, r) = (shared.clone(), registry.clone());
45:                 crate::exec::spawn(move || loop {
46:                     let job = {
47:                         let mut q = s.queue.lock().unwrap();
48:                         while q.is_empty() {
49:                             q = s.cv.wait(q).unwrap();
50:                         }
51:                         q.pop() // newest first = what is on screen now
52:                     };
53:                     if let Some((loc, edge)) = job {
54:                         let res = decode(&r, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
55:                         s.out.lock().unwrap().push(res);
56:                     }
57:                 });
58:             }
59:         }
60:         Self { shared, registry, seen: Mutex::new(HashSet::new()) }
61:     }
62: 
63:     pub fn request(&self, loc: &Location, edge: u32) {
64:         if !self.seen.lock().unwrap().insert((loc.clone(), edge)) {
65:             return;
66:         }
67:         self.shared.queue.lock().unwrap().push((loc.clone(), edge));
68:         self.shared.cv.notify_one();
69:     }
70: 
71:     pub fn forget(&self, loc: &Location) {
72:         self.seen.lock().unwrap().retain(|(l, _)| l != loc);
73:     }
74: 
75:     /// Drain finished thumbnails. In the browser this also does the decoding, a few per call.
76:     pub fn poll(&self) -> Vec<ThumbResult> {
77:         if !THREADED {
78:             for _ in 0..2 {
79:                 let job = self.shared.queue.lock().unwrap().pop();
80:                 let Some((loc, edge)) = job else { break };
81:                 let r = decode(&self.registry, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
82:                 self.shared.out.lock().unwrap().push(r);
83:             }
84:         }
85:         std::mem::take(&mut *self.shared.out.lock().unwrap())
86:     }
87: 
88:     pub fn pending(&self) -> bool {
89:         !self.shared.queue.lock().unwrap().is_empty()
90:     }
91: }
```

### TOOL: shell [seq 83] 2026-11-09 13:47:19 status=completed

```json
{
  "command": "cat /tmp/opencode/canonical-photos-audit/zip-verygood/smart-files/crates/smart-core/src/thumbs.rs\necho ===LIB-VERYGOOD-FULL===\ncat /tmp/opencode/canonical-photos-audit/zip-verygood/smart-files/crates/smart-core/src/library.rs"
}
```

```
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
    let img = match image::load_from_memory(&bytes) {
        Ok(i) => i,
        Err(_) => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                use crate::model::{ext_of, Kind};
                if matches!(Kind::from_ext(&ext_of(&loc.name())), Kind::Video) {
                    return video_thumb(v.as_ref(), loc, edge);
                }
            }
            return None;
        }
    };
    let original = Some((img.width(), img.height()));
    let t = img.thumbnail(edge, edge).to_rgba8();
    Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
}

/// Pull a frame out of a video with ffmpeg (best-effort) and cache it on disk.
#[cfg(not(target_arch = "wasm32"))]
fn video_thumb(v: &dyn crate::vfs::Vfs, loc: &Location, edge: u32) -> Option<ThumbResult> {
    use std::io::Write;
    let real = v.real_path(&loc.path)?;
    let cache_dir = dirs::cache_dir()?.join("smart-files/thumbs");
    let _ = std::fs::create_dir_all(&cache_dir);
    let key = format!("{:x}", smart_hash(&real.to_string_lossy()));
    let cache = cache_dir.join(format!("{key}.png"));
    let png = if cache.exists() {
        std::fs::read(&cache).ok()?
    } else {
        let out = std::process::Command::new("ffmpeg")
            .args(["-loglevel", "error", "-ss", "0.7", "-i"])
            .arg(&real)
            .args(["-frames:v", "1", "-vf", "scale='min(384,iw)':-2", "-f", "image2pipe", "-vcodec", "png", "-"])
            .output()
            .ok()?;
        if !out.status.success() || out.stdout.is_empty() {
            return None;
        }
        if let mut f = std::fs::File::create(&cache).ok() {
            let _ = f.write_all(&out.stdout);
        }
        out.stdout
    };
    let img = image::load_from_memory(&png).ok()?;
    let original = Some((img.width(), img.height()));
    let t = img.thumbnail(edge, edge).to_rgba8();
    Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
}

#[cfg(not(target_arch = "wasm32"))]
fn smart_hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
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
===LIB-VERYGOOD-FULL===
//! User-owned metadata: tags, recents, pinned places and preferences. Serialised as JSON.
use crate::model::{now_secs, Location, SortKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub type SharedLibrary = Arc<RwLock<Library>>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TagDef {
    pub id: String,
    pub name: String,
    pub color: String, // #rrggbb
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recent {
    pub loc: Location,
    pub at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub dark: bool,
    pub grid: bool,
    pub sort: SortKey,
    pub ascending: bool,
    pub show_hidden: bool,
    pub folders_first: bool,
    pub sidebar_collapsed: bool,
    pub wallpaper: usize,
    pub search_contents: bool,
    pub thumb_scale: f32,
    pub show_extensions: bool,
    pub confirm_delete: bool,
    pub copy_animation_secs: f32,
    pub reduce_motion: bool,
    pub autoplay: bool,
    pub accent: String,
    /// Single click opens items (double-click otherwise).
    #[serde(default)]
    pub single_click: bool,
    /// Human/natural ordering: file2 before file10.
    #[serde(default = "default_true")]
    pub natural_sort: bool,
    /// Context-aware defaults (e.g. Downloads sorts by newest first).
    #[serde(default = "default_true")]
    pub smart_sort: bool,
    /// Type-ahead accumulates characters to jump through the list.
    #[serde(default = "default_true")]
    pub typeahead: bool,
    /// 0 = ask on conflict, 1 = skip existing, 2 = replace existing.
    #[serde(default)]
    pub overwrite: u8,
    /// Toast when a transfer or extraction finishes.
    #[serde(default = "default_true")]
    pub notify_done: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dark: false,
            grid: true,
            sort: SortKey::Name,
            ascending: true,
            show_hidden: false,
            folders_first: true,
            sidebar_collapsed: false,
            wallpaper: 0,
            search_contents: false,
            thumb_scale: 1.0,
            show_extensions: true,
            confirm_delete: true,
            copy_animation_secs: 5.0,
            reduce_motion: false,
            autoplay: false,
            accent: "#2f6feb".into(),
            single_click: false,
            natural_sort: true,
            smart_sort: true,
            typeahead: true,
            overwrite: 0,
            notify_done: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Library {
    pub tags: Vec<TagDef>,
    pub assignments: BTreeMap<String, Vec<S
```

### TOOL: shell [seq 98] 2026-11-09 13:47:21 status=completed

```json
{
  "command": "cat /home/lufuno/crates/smart-ui-egui/src/photos.rs > /tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs\nwc -l /tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs\ngrep -n \"^pub struct\\|^pub enum\\|^struct\\|^enum\\|^impl\\|^ fn \\|^pub fn \\|^    pub fn \\|^    fn \" /home/lufuno/crates/smart-ui-egui/src/photos.rs | head -n 150"
}
```

```
1467 /tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs
16:pub struct Adjust {
24:impl Default for Adjust {
25:    fn default() -> Self {
42:pub enum Album {
48:pub struct Loaded {
60:pub struct Photos {
104:impl Default for Photos {
105:    fn default() -> Self {
222:impl SmartApp {
223:    fn photos_scan(&mut self) {
250:    pub fn photos_first(&mut self) -> Option<Location> {
257:    pub fn photos_show(&mut self, l: Location) {
265:    fn photos_reset_view(&mut self) {
278:    fn photos_target(&self) -> Option<Location> {
282:    fn photos_toggle_fav(&mut self, l: Location) {
288:    pub fn photos_cmd(&mut self, c: &str) {
402:    pub fn photos_list(&self) -> Vec<Entry> {
425:    fn photos_save(&mut self) {
466:    fn photos_thumb(&mut self, ctx: &egui::Context, l: &Location, budget: &mut u32) -> Option<(egui::TextureId, [f32; 2])> {
487:    fn photos_load_cur(&mut self, ctx: &egui::Context) {
526:    fn photos_reprocess(&mut self, ctx: &egui::Context) {
534:    fn photos_nav(&mut self, list: &[Entry], delta: i32) {
554:    fn photos_item_menu(&self, loc: &Location) -> Vec<MI> {
610:    pub fn draw_photos(&mut self, ui: &mut Ui, fr: &Frame) {
641:    fn photos_sidebar(&mut self, ui: &mut Ui, fr: &Frame, folders: &[(Location, usize)]) {
722:    fn photos_toolbar(&mut self, ui: &mut Ui, fr: &Frame, list: &[Entry], vs: f32) {
854:    fn photos_grid(&mut self, ui: &mut Ui, fr: &Frame, list: &[Entry], viewer: bool) {
970:    fn draw_photo_viewer(&mut self, ui: &mut Ui, fr: &Frame, vs: f32, list: &[Entry]) {
1294:    fn draw_photo_info(&mut self, ui: &mut Ui, r: Rect, p: &egui::Painter) {
1326:    fn draw_photo_edit(&mut self, ui: &mut Ui, r: Rect, p: &egui::Painter) {
1406:    pub fn photos_settings(&mut self, ui: &mut Ui, win: Rect) {

```

### TOOL: read [seq 107] 2026-11-09 13:47:24 status=completed

```json
{
  "path": "/tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs"
}
```

```
Read file /tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs, lines 1-1058
1: //! Smart Photos — library grid, albums, a viewer with spring zoom/pan, filmstrip, crossfading slideshow and a
2: //! non-destructive-feeling editor (live adjustments, filter presets with previews, compare, histogram, save a copy).
3: //! Pure Rust: images are decoded with the `image` crate and edited on the CPU.
4: use crate::app::*;
5: use crate::appkit::{Frame, SRow, SVal};
6: use crate::chrome::*;
7: use crate::overlays::{mi, MI};
8: use crate::icons::Ic;
9: use crate::paint::*;
10: use crate::theme::*;
11: use egui::{pos2, vec2, Align2, Color32, Id, Pos2, Rect, Sense, TextureHandle, TextureOptions, Ui, Vec2};
12: use smart_core::*;
13: use std::collections::{HashMap, HashSet};
14: 
15: #[derive(Clone, Copy, PartialEq, Debug)]
16: pub struct Adjust {
17:     pub exposure: f32,
18:     pub contrast: f32,
19:     pub saturation: f32,
20:     pub warmth: f32,
21:     pub vignette: f32,
22: }
23: 
24: impl Default for Adjust {
25:     fn default() -> Self {
26:         Self { exposure: 0.0, contrast: 0.0, saturation: 0.0, warmth: 0.0, vignette: 0.0 }
27:     }
28: }
29: 
30: const PRESETS: [(&str, Adjust); 8] = [
31:     ("Original", Adjust { exposure: 0.0, contrast: 0.0, saturation: 0.0, warmth: 0.0, vignette: 0.0 }),
32:     ("Vivid", Adjust { exposure: 0.05, contrast: 0.18, saturation: 0.45, warmth: 0.05, vignette: 0.0 }),
33:     ("Warm", Adjust { exposure: 0.05, contrast: 0.05, saturation: 0.10, warmth: 0.55, vignette: 0.10 }),
34:     ("Cool", Adjust { exposure: 0.0, contrast: 0.08, saturation: 0.0, warmth: -0.55, vignette: 0.10 }),
35:     ("Mono", Adjust { exposure: 0.0, contrast: 0.12, saturation: -1.0, warmth: 0.0, vignette: 0.0 }),
36:     ("Noir", Adjust { exposure: -0.10, contrast: 0.40, saturation: -1.0, warmth: 0.0, vignette: 0.55 }),
37:     ("Fade", Adjust { exposure: 0.15, contrast: -0.25, saturation: -0.30, warmth: 0.10, vignette: 0.0 }),
38:     ("Dusk", Adjust { exposure: -0.10, contrast: 0.15, saturation: 0.20, warmth: -0.25, vignette: 0.40 }),
39: ];
40: 
41: #[derive(Clone, PartialEq)]
42: pub enum Album {
43:     All,
44:     Favorites,
45:     Folder(Location),
46: }
47: 
48: pub struct Loaded {
49:     pub loc: Location,
50:     pub rgba: Vec<u8>,
51:     pub w: usize,
52:     pub h: usize,
53:     pub tex: TextureHandle,
54:     pub orig: TextureHandle,
55:     pub hist: [f32; 48],
56:     pub presets: Vec<TextureHandle>,
57:     pub bytes: u64,
58: }
59: 
60: pub struct Photos {
61:     pub scanned: bool,
62:     pub all: Vec<Entry>,
63:     pub album: Album,
64:     pub thumbs: HashMap<Location, (TextureHandle, [f32; 2])>,
65:     pub failed: HashSet<Location>,
66:     pub favs: HashSet<Location>,
67:     pub open: Option<Location>,
68:     pub cur: Option<Loaded>,
69:     pub prev: Option<(TextureHandle, f64)>,
70:     pub info: bool,
71:     pub edit: bool,
72:     pub zoom: f32,
73:     pub pan: Vec2,
74:     pub rot: i32,
75:     pub flip_h: bool,
76:     pub flip_v: bool,
77:     pub adj: Adjust,
78:     pub preset: usize,
79:     pub dirty: bool,
80:     pub tile: f32,
81:     pub slideshow: bool,
82:     pub slide_t: f64,
83:     pub compare: bool,
84:     pub grid_scroll_to: Option<f32>,
85:     pub sort: usize,
86:     pub fill: bool,
87:     pub captions: bool,
88:     pub gap: f32,
89:     pub slide_secs: f32,
90:     pub filmstrip: bool,
91:     pub tools: bool,
92:     pub wheel_zoom: bool,
93:     pub search: String,
94:     pub search_open: bool,
95:     pub search_focus: bool,
96:     pub ctx: Option<Location>,
97:     pub folders: Vec<(Location, usize)>,
98:     pub strip_x: f32,
99:     pub strip_v: f32,
100:     pub strip_goal: f32,
101:     pub strip_idx: usize,
102: }
103: 
104: impl Default for Photos {
105:     fn default() -> Self {
106:         Self {
107:             scanned: false,
108:             all: vec![],
109:             album: Album::All,
110:             thumbs: HashMap::new(),
111:             failed: HashSet::new(),
112:             favs: HashSet::new(),
113:             open: None,
114:             cur: None,
115:             prev: None,
116:             info: false,
117:             edit: false,
118:             zoom: 1.0,
119:             pan: Vec2::ZERO,
120:             rot: 0,
121:             flip_h: false,
122:             flip_v: false,
123:             adj: Adjust::default(),
124:             preset: 0,
125:             dirty: false,
126:             tile: 210.0,
127:             slideshow: false,
128:             slide_t: 0.0,
129:             compare: false,
130:             grid_scroll_to: None,
131:             sort: 0,
132:             fill: true,
133:             captions: false,
134:             gap: 12.0,
135:             slide_secs: 4.0,
136:             filmstrip: true,
137:             tools: true,
138:             wheel_zoom: true,
139:             search: String::new(),
140:             search_open: false,
141:             search_focus: false,
142:             ctx: None,
143:             folders: vec![],
144:             strip_x: 0.0,
145:             strip_v: 0.0,
146:             strip_goal: 0.0,
147:             strip_idx: usize::MAX,
148:         }
149:     }
150: }
151: 
152: fn process(src: &[u8], w: usize, h: usize, a: &Adjust) -> Vec<u8> {
153:     let mut out = vec![255u8; src.len()];
154:     let ex = 2f32.powf(a.exposure);
155:     let k = 1.0 + a.contrast * 1.1;
156:     let sat = 1.0 + a.saturation;
157:     let warm = a.warmth * 0.10;
158:     let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
159:     let maxd = (cx * cx + cy * cy).sqrt().max(1.0);
160:     let vig = a.vignette;
161:     for y in 0..h {
162:         let dy = (y as f32 - cy) / maxd;
163:         for x in 0..w {
164:             let i = (y * w + x) * 4;
165:             let (mut r, mut g, mut b) = (src[i] as f32 / 255.0, src[i + 1] as f32 / 255.0, src[i + 2] as f32 / 255.0);
166:             r *= ex;
167:             g *= ex;
168:             b *= ex;
169:             r = (r - 0.5) * k + 0.5;
170:             g = (g - 0.5) * k + 0.5;
171:             b = (b - 0.5) * k + 0.5;
172:             let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
173:             r = l + (r - l) * sat + warm;
174:             g = l + (g - l) * sat + warm * 0.15;
175:             b = l + (b - l) * sat - warm;
176:             if vig > 0.0 {
177:                 let dx = (x as f32 - cx) / maxd;
178:                 let d = (dx * dx + dy * dy).sqrt();
179:                 let t = ((d - 0.30) / 0.70).clamp(0.0, 1.0);
180:                 let v = 1.0 - vig * t * t * (3.0 - 2.0 * t);
181:                 r *= v;
182:                 g *= v;
183:                 b *= v;
184:             }
185:             out[i] = (r.clamp(0.0, 1.0) * 255.0) as u8;
186:             out[i + 1] = (g.clamp(0.0, 1.0) * 255.0) as u8;
187:             out[i + 2] = (b.clamp(0.0, 1.0) * 255.0) as u8;
188:         }
189:     }
190:     out
191: }
192: 
193: fn uvs(rot: i32, fh: bool, fv: bool) -> [egui::Pos2; 4] {
194:     let base = [pos2(0.0, 0.0), pos2(1.0, 0.0), pos2(1.0, 1.0), pos2(0.0, 1.0)];
195:     let k = rot.rem_euclid(4) as usize;
196:     let mut u = [base[0], base[0], base[0], base[0]];
197:     for i in 0..4 {
198:         u[i] = base[(i + 4 - k) % 4];
199:     }
200:     if fh {
201:         u.swap(0, 1);
202:         u.swap(2, 3);
203:     }
204:     if fv {
205:         u.swap(0, 3);
206:         u.swap(1, 2);
207:     }
208:     u
209: }
210: 
211: fn draw_tex(p: &egui::Painter, tex: egui::TextureId, r: Rect, rot: i32, fh: bool, fv: bool, tint: Color32) {
212:     let u = uvs(rot, fh, fv);
213:     let mut m = egui::epaint::Mesh::with_texture(tex);
214:     let c = [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom()];
215:     for i in 0..4 {
216:         m.vertices.push(egui::epaint::Vertex { pos: c[i], uv: u[i], color: tint });
217:     }
218:     m.indices.extend([0, 1, 2, 0, 2, 3]);
219:     p.add(egui::Shape::mesh(m
```

### TOOL: read [seq 114] 2026-11-09 13:47:28 status=completed

```json
{
  "limit": 450,
  "offset": 1059,
  "path": "/tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs"
}
```

```
Read file /tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs, lines 1059-1467
1059:                 if let Some((pt, _)) = &self.shell.photos.prev {
1060:                     let pr = Rect::from_center_size(canvas.center(), vec2(canvas.width() * 0.9, canvas.height() * 0.9));
1061:                     draw_tex(&sp, pt.id(), pr, 0, false, false, alpha(Color32::WHITE, 1.0 - t));
1062:                 }
1063:             }
1064:             shadow(&sp, r, 4.0, 14.0, 40.0, -6.0, alpha(Color32::BLACK, (if th.dark { 0.6 } else { 0.32 }) * smooth01(t)));
1065:             let tex = if compare { cur.orig.id() } else { cur.tex.id() };
1066:             draw_tex(&sp, tex, r, rot, fh, fv, alpha(Color32::WHITE, smooth01(t)));
1067:             if compare {
1068:                 let b = Rect::from_center_size(pos2(r.center().x, r.top() + 22.0), vec2(86.0, 26.0));
1069:                 grad_fill(&p, b, 13.0, alpha(Color32::BLACK, 0.6), alpha(Color32::BLACK, 0.5));
1070:                 p.text(b.center(), Align2::CENTER_CENTER, "ORIGINAL", f_bold(10.0), Color32::WHITE);
1071:             }
1072:             let resp = ui.interact(canvas, Id::new("ph-stage"), Sense::click_and_drag());
1073:             if resp.dragged() && self.shell.photos.zoom > 1.0 {
1074:                 self.shell.photos.pan += resp.drag_delta();
1075:                 ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
1076:             } else if resp.hovered() && self.shell.photos.zoom > 1.0 {
1077:                 ctx.set_cursor_icon(egui::CursorIcon::Grab);
1078:             }
1079:             if resp.double_clicked() {
1080:                 let z = if self.shell.photos.zoom > 1.05 { 1.0 } else { 2.4 };
1081:                 self.shell.photos.zoom = z;
1082:                 if z == 1.0 {
1083:                     self.shell.photos.pan = Vec2::ZERO;
1084:                 }
1085:             }
1086:             if resp.secondary_clicked() {
1087:                 if let Some(pos) = ctx.pointer_latest_pos() {
1088:                     self.shell.photos.ctx = None;
1089:                     let items = self.photos_viewer_menu();
1090:                     self.open_app_menu(pos, items);
1091:                 }
1092:             }
1093:             if resp.hovered() && self.shell.photos.wheel_zoom {
1094:                 let (sy, pinch) = ctx.input(|i| (i.smooth_scroll_delta.y, i.zoom_delta()));
1095:                 if sy.abs() > 0.1 || (pinch - 1.0).abs() > 0.001 {
1096:                     let nz = (self.shell.photos.zoom * (sy * 0.0035).exp() * pinch).clamp(0.4, 10.0);
1097:                     self.shell.photos.zoom = nz;
1098:                     if nz <= 1.0 {
1099:                         self.shell.photos.pan *= 0.8;
1100:                     }
1101:                 }
1102:             }
1103:         } else {
1104:             let t = self.time as f32;
1105:             for k in 0..3 {
1106:                 p.circle_filled(canvas.center() + vec2((k as f32 - 1.0) * 18.0, 0.0), 4.0 + 2.0 * (t * 6.0 + k as f32).sin(), alpha(th.ink3, 0.6));
1107:             }
1108:             ctx.request_repaint();
1109:         }
1110: 
1111:         // arrows
1112:         for (dir, icon) in [(-1, "alt-arrow-left"), (1, "alt-arrow-right")] {
1113:             let c0 = pos2(if dir < 0 { canvas.left() + 34.0 } else { canvas.right() - 34.0 }, canvas.center().y);
1114:             let r = Rect::from_center_size(c0, vec2(44.0, 44.0));
1115:             let near = ctx.pointer_hover_pos().is_some_and(|pp| (pp.x - c0.x).abs() < 120.0 && canvas.contains(pp));
1116:             let a = spring_to(&ctx, ("ph-arrow", dir), if near { 1.0 } else { 0.0 }, 260.0, 24.0).clamp(0.0, 1.0);
1117:             if a > 0.02 {
1118:                 let resp = self.hit(ui, r, ("ph-nav", dir), Sense::click());
1119:                 shadow(&p, r, 22.0, 3.0, 10.0, -2.0, alpha(Color32::BLACK, 0.3 * a));
1120:                 p.circle_filled(r.center(), 22.0 * (0.9 + 0.1 * a), alpha(Color32::BLACK, 0.5 * a));
1121:                 self.ico(&p, icon, if resp.hovered() { Ic::Duo } else { Ic::Outline }, r.center(), 20.0, alpha(Color32::WHITE, a));
1122:                 if resp.clicked() {
1123:                     self.photos_nav(list, dir);
1124:                 }
1125:             }
1126:         }
1127: 
1128:         // tool pill (raised, theme-aware)
1129:         let idx = list.iter().position(|e| Some(&e.loc) == self.shell.photos.open.as_ref()).map(|i| i + 1).unwrap_or(0);
1130:         if self.shell.photos.tools {
1131:             let tools: [(&'static str, &str, &str); 7] = [("rotate-left", "rotl", "Rotate left"), ("restart", "rotr", "Rotate right"), ("flip-horizontal", "fliph", "Flip horizontally"), ("flip-vertical", "flipv", "Flip vertically"), ("magnifer-zoom-in", "zoomin", "Zoom in"), ("full-screen", "fit", "Fit to window"), (if self.shell.photos.slideshow { "pause" } else { "play" }, "slideshow", "Slideshow  S")];
1132:             let pw = tools.len() as f32 * 40.0 + 14.0;
1133:             let pill = Rect::from_center_size(pos2(canvas.center().x, canvas.bottom() - 34.0), vec2(pw.min(canvas.width() - 20.0), 44.0));
1134:             raised(&p, &th, pill, 22.0, false, false);
1135:             for (i, (icon, id, tip)) in tools.iter().enumerate() {
1136:                 let r = Rect::from_center_size(pill.left_center() + vec2(27.0 + i as f32 * 40.0, 0.0), vec2(34.0, 34.0));
1137:                 if r.right() > pill.right() {
1138:                     break;
1139:                 }
1140:                 let resp = self.hit(ui, r, ("ph-tool", *id), Sense::click());
1141:                 let a = self.anim(ui, ("ph-tool-h", *id), resp.hovered());
1142:                 if resp.is_pointer_button_down_on() {
1143:                     grad_fill(&p, r, 17.0, alpha(th.shadow, if th.dark { 0.3 } else { 0.10 }), alpha(th.shadow, 0.04));
1144:                 } else if a > 0.01 {
1145:                     grad_fill(&p, r, 17.0, alpha(th.accent, 0.16 * a), alpha(th.accent, 0.07 * a));
1146:                     rim(&p, r, 17.0, 1.0, alpha(Color32::WHITE, (if th.dark { 0.14 } else { 0.9 }) * a), true, 12.0);
1147:                 }
1148:                 let on = (*id == "fliph" && fh) || (*id == "flipv" && fv);
1149:                 self.ico(&p, icon, if a > 0.5 || on { Ic::Duo } else { Ic::Outline }, r.center(), 18.0, if on { th.accent } else { mix(th.ink2, th.accent_ink, a) });
1150:                 resp.clone().on_hover_text(*tip);
1151:                 if resp.clicked() {
1152:                     self.push_act(Act::App(format!("photos.{id}")));
1153:                 }
1154:             }
1155:             if self.shell.photos.slideshow {
1156:                 let f = (((self.time - self.shell.photos.slide_t) / self.shell.photos.slide_secs as f64) as f32).clamp(0.0, 1.0);
1157:                 let b = Rect::from_min_size(pill.left_bottom() + vec2(16.0, 6.0), vec2(pill.width() - 32.0, 3.0));
1158:                 p.rect_filled(b, cr(1.5), alpha(th.ink3, 0.25));
1159:                 p.rect_filled(Rect::from_min_size(b.min, vec2(b.width() * f, 3.0)), cr(1.5), th.accent);
1160:             }
1161:         }
1162: 
1163:         // filmstrip: wheel + drag scrollable, follows the current photo with a spring
1164:         if strip_h > 0.0 {
1165:             let strip = Rect::from_min_max(pos2(outer.left(), outer.bottom() - strip_h), outer.max);
1166:             let (tw_, gap) = (62.0f32, 8.0f32);
1167:             let total = list.len() as f32 * (tw_ + gap) - gap;
1168:             let max_x = (total - strip.width()).max(0.0);
1169:             let cur_i = idx.saturating_sub(1);
1170:             if self.shell.photos.strip_idx != cur_i {
1171:                 self.shell.photos.strip_idx = cur_i;
1172:                 self.shell.photos.strip_goal = (cur_i as f32 * (tw_ + gap) + tw_ / 2.0 - strip.width() / 2.0).clamp(0.0, max_x);
1173:             }
1174:             let resp = ui.interact(strip, Id::new("ph-strip"), Sense::click_an
```

### TOOL: read [seq 121] 2026-11-09 13:47:30 status=completed

```json
{
  "path": "/tmp/opencode/canonical-photos-audit/home-crates/analyze.home.rs"
}
```

```
Read file /tmp/opencode/canonical-photos-audit/home-crates/analyze.home.rs, lines 1-746
1: //! Storage analysis: an incremental, bounded-memory directory scanner plus a *context-aware* junk detector.
2: //!
3: //! Design notes
4: //! * Only **directories** become nodes; per directory we keep the ten biggest files and aggregate the rest, so a
5: //!   home folder with millions of files stays in the tens of megabytes.
6: //! * The scanner is a state machine (`Scan::step`). Desktop runs it on a worker thread, the web build slices it
7: //!   across frames. Nothing here touches threads or clocks, so it behaves identically (and is unit-tested) everywhere.
8: //! * Junk is not "big files". A `target/` next to a `Cargo.toml`, a `node_modules/` next to a `package.json`, a
9: //!   `.venv`, package-manager caches, the Trash, stale installers and "Report (1).pdf" copies are all recognised from
10: //!   their *surroundings*, graded by how safe they are to remove, and annotated with how to get them back.
11: use crate::model::{Entry, Kind, Location};
12: use crate::vfs::Vfs;
13: use std::collections::HashSet;
14: 
15: /// How risky it is to remove something.
16: #[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
17: pub enum Safety {
18:     /// Regenerated automatically (build output, caches). Removing loses nothing.
19:     Safe,
20:     /// Probably unwanted, but it is *your* call (old installers, copies, logs).
21:     Review,
22:     /// Looks like personal data that is merely large or old. Never pre-selected.
23:     Careful,
24: }
25: 
26: impl Safety {
27:     pub fn label(self) -> &'static str {
28:         match self {
29:             Safety::Safe => "Safe",
30:             Safety::Review => "Review",
31:             Safety::Careful => "Careful",
32:         }
33:     }
34: }
35: 
36: #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
37: pub enum JunkKind {
38:     BuildArtifacts,
39:     Dependencies,
40:     Environments,
41:     Caches,
42:     Temp,
43:     Logs,
44:     Trash,
45:     Installers,
46:     Duplicates,
47:     LargeStale,
48: }
49: 
50: impl JunkKind {
51:     pub const ALL: [JunkKind; 10] = [
52:         JunkKind::BuildArtifacts,
53:         JunkKind::Dependencies,
54:         JunkKind::Environments,
55:         JunkKind::Caches,
56:         JunkKind::Temp,
57:         JunkKind::Logs,
58:         JunkKind::Trash,
59:         JunkKind::Installers,
60:         JunkKind::Duplicates,
61:         JunkKind::LargeStale,
62:     ];
63:     pub fn label(self) -> &'static str {
64:         match self {
65:             JunkKind::BuildArtifacts => "Build artifacts",
66:             JunkKind::Dependencies => "Dependencies",
67:             JunkKind::Environments => "Virtual environments",
68:             JunkKind::Caches => "Caches",
69:             JunkKind::Temp => "Temporary files",
70:             JunkKind::Logs => "Large logs",
71:             JunkKind::Trash => "Trash",
72:             JunkKind::Installers => "Old installers",
73:             JunkKind::Duplicates => "Likely copies",
74:             JunkKind::LargeStale => "Large & untouched",
75:         }
76:     }
77:     pub fn icon(self) -> &'static str {
78:         match self {
79:             JunkKind::BuildArtifacts => "code-square",
80:             JunkKind::Dependencies => "box-minimalistic",
81:             JunkKind::Environments => "box-minimalistic",
82:             JunkKind::Caches => "refresh",
83:             JunkKind::Temp => "clock-circle",
84:             JunkKind::Logs => "text",
85:             JunkKind::Trash => "trash-bin-minimalistic",
86:             JunkKind::Installers => "download-minimalistic",
87:             JunkKind::Duplicates => "copy",
88:             JunkKind::LargeStale => "history",
89:         }
90:     }
91:     pub fn blurb(self) -> &'static str {
92:         match self {
93:             JunkKind::BuildArtifacts => "Compiler output that your next build recreates",
94:             JunkKind::Dependencies => "Downloaded packages that the package manager restores",
95:             JunkKind::Environments => "Language environments you can rebuild from a lockfile",
96:             JunkKind::Caches => "Data apps keep to be faster. They rebuild it on demand",
97:             JunkKind::Temp => "Leftovers: partial downloads, backups, editor swap files",
98:             JunkKind::Logs => "Log files that grew far beyond usefulness",
99:             JunkKind::Trash => "Already deleted. Emptying frees the space for real",
100:             JunkKind::Installers => "Setup files you already ran",
101:             JunkKind::Duplicates => "Files named like a copy of a sibling",
102:             JunkKind::LargeStale => "Big files nobody has touched for a long time",
103:         }
104:     }
105: }
106: 
107: #[derive(Clone, Debug)]
108: pub struct FileInfo {
109:     pub name: String,
110:     pub path: String,
111:     pub size: u64,
112:     pub modified: Option<i64>,
113:     pub kind: Kind,
114: }
115: 
116: #[derive(Clone, Debug)]
117: pub struct DirNode {
118:     pub name: String,
119:     pub path: String,
120:     pub parent: Option<usize>,
121:     /// Recursive size in bytes (grows while scanning).
122:     pub size: u64,
123:     /// Bytes of files directly inside this folder.
124:     pub own: u64,
125:     /// Files directly inside this folder.
126:     pub files: u64,
127:     pub children: Vec<usize>,
128:     /// The biggest files directly inside (at most `TOP_FILES`).
129:     pub top_files: Vec<FileInfo>,
130:     pub newest: i64,
131:     pub junk: Option<usize>,
132:     pub in_junk: bool,
133:     pub done: bool,
134: }
135: 
136: #[derive(Clone, Debug)]
137: pub struct Junk {
138:     pub path: String,
139:     pub name: String,
140:     pub is_dir: bool,
141:     pub kind: JunkKind,
142:     pub safety: Safety,
143:     pub reason: String,
144:     /// How the data comes back, when it does.
145:     pub restore: Option<&'static str>,
146:     pub modified: Option<i64>,
147:     size: u64,
148:     node: Option<usize>,
149: }
150: 
151: impl Junk {
152:     pub fn size(&self) -> u64 {
153:         self.size
154:     }
155: }
156: 
157: pub const TOP_FILES: usize = 10;
158: pub const MIN_JUNK_BYTES: u64 = 512 * 1024;
159: const MAX_FILE_JUNK: usize = 1500;
160: const DAY: i64 = 86_400;
161: 
162: /// Coarse content buckets for the overview chart.
163: #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
164: pub enum Bucket {
165:     Media,
166:     Documents,
167:     Code,
168:     Archives,
169:     Apps,
170:     Other,
171: }
172: 
173: impl Bucket {
174:     pub const ALL: [Bucket; 6] = [Bucket::Media, Bucket::Documents, Bucket::Code, Bucket::Archives, Bucket::Apps, Bucket::Other];
175:     pub fn label(self) -> &'static str {
176:         match self {
177:             Bucket::Media => "Photos, audio & video",
178:             Bucket::Documents => "Documents",
179:             Bucket::Code => "Code & text",
180:             Bucket::Archives => "Archives",
181:             Bucket::Apps => "Apps & installers",
182:             Bucket::Other => "Other",
183:         }
184:     }
185:     pub fn of(k: Kind) -> Bucket {
186:         match k {
187:             Kind::Image | Kind::Vector | Kind::Audio | Kind::Video | Kind::Project => Bucket::Media,
188:             Kind::Pdf | Kind::Document => Bucket::Documents,
189:             Kind::Text | Kind::Code => Bucket::Code,
190:             Kind::Archive => Bucket::Archives,
191:             Kind::App => Bucket::Apps,
192:             _ => Bucket::Other,
193:         }
194:     }
195:     fn idx(self) -> usize {
196:         Bucket::ALL.iter().position(|b| *b == self).unwrap_or(5)
197:     }
198: }
199: 
200: pub struct Scan {
201:     pub fs: String,
202:     pub root: String,
203:     pub nodes: Vec<DirNode>,
204:     pub junk: Vec<Junk>,
205:     pub buckets: [u64; 6],
206:     pub files_seen: u64,
207:     pub dirs_seen: u64,
208:     pub errors: u64,
209:     pu
```

### TOOL: read [seq 121] 2026-11-09 13:47:30 status=completed

```json
{
  "path": "/tmp/opencode/canonical-photos-audit/home-crates/highlight.home.rs"
}
```

```
Read file /tmp/opencode/canonical-photos-audit/home-crates/highlight.home.rs, lines 1-300
1: //! Syntax highlighting with tree-sitter (Rust, Python, JavaScript, JSON, TOML, Bash, CSS, HTML, C, Go, Markdown) and a
2: //! small lexer for everything else, so *every* text file gets colour and nothing ever fails to preview.
3: use egui::{text::LayoutJob, Color32, FontId, TextFormat};
4: 
5: #[derive(Clone, Copy, PartialEq, Eq, Debug)]
6: pub enum Cls {
7:     Plain,
8:     Keyword,
9:     Function,
10:     Type,
11:     Str,
12:     Number,
13:     Comment,
14:     Punct,
15:     Property,
16:     Tag,
17:     Title,
18:     Link,
19:     Escape,
20: }
21: 
22: const NAMES: [&str; 37] = [
23:     "attribute", "comment", "constant", "constant.builtin", "constructor", "embedded", "escape", "function", "function.builtin", "function.method", "keyword", "number", "operator", "property", "punctuation", "punctuation.bracket", "punctuation.delimiter", "punctuation.special", "string", "string.special", "string.escape", "tag", "type", "type.builtin", "variable", "variable.builtin", "variable.parameter", "text.title", "text.literal", "text.uri", "text.reference", "text.emphasis", "text.strong", "label", "module", "boolean", "character",
24: ];
25: 
26: fn cls_of(i: usize) -> Cls {
27:     match NAMES[i] {
28:         "keyword" | "boolean" | "variable.builtin" | "label" => Cls::Keyword,
29:         "function" | "function.builtin" | "function.method" | "constructor" => Cls::Function,
30:         "type" | "type.builtin" | "module" | "attribute" => Cls::Type,
31:         "string" | "string.special" | "text.literal" | "character" | "embedded" => Cls::Str,
32:         "number" | "constant" | "constant.builtin" => Cls::Number,
33:         "comment" => Cls::Comment,
34:         "operator" | "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => Cls::Punct,
35:         "property" | "variable.parameter" => Cls::Property,
36:         "tag" => Cls::Tag,
37:         "text.title" | "text.strong" => Cls::Title,
38:         "text.uri" | "text.reference" | "text.emphasis" => Cls::Link,
39:         "escape" | "string.escape" => Cls::Escape,
40:         _ => Cls::Plain,
41:     }
42: }
43: 
44: #[cfg(not(target_arch = "wasm32"))]
45: fn config(ext: &str) -> Option<tree_sitter_highlight::HighlightConfiguration> {
46:     let (lang, name, q): (tree_sitter::Language, &str, &str) = match ext {
47:         "rs" => (tree_sitter_rust::LANGUAGE.into(), "rust", tree_sitter_rust::HIGHLIGHTS_QUERY),
48:         "py" | "pyw" => (tree_sitter_python::LANGUAGE.into(), "python", tree_sitter_python::HIGHLIGHTS_QUERY),
49:         "js" | "mjs" | "cjs" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), "javascript", tree_sitter_javascript::HIGHLIGHT_QUERY),
50:         "json" | "jsonc" => (tree_sitter_json::LANGUAGE.into(), "json", tree_sitter_json::HIGHLIGHTS_QUERY),
51:         "toml" => (tree_sitter_toml_ng::LANGUAGE.into(), "toml", tree_sitter_toml_ng::HIGHLIGHTS_QUERY),
52:         "sh" | "bash" | "zsh" => (tree_sitter_bash::LANGUAGE.into(), "bash", tree_sitter_bash::HIGHLIGHT_QUERY),
53:         "css" => (tree_sitter_css::LANGUAGE.into(), "css", tree_sitter_css::HIGHLIGHTS_QUERY),
54:         "html" | "htm" => (tree_sitter_html::LANGUAGE.into(), "html", tree_sitter_html::HIGHLIGHTS_QUERY),
55:         "c" | "h" => (tree_sitter_c::LANGUAGE.into(), "c", tree_sitter_c::HIGHLIGHT_QUERY),
56:         "go" => (tree_sitter_go::LANGUAGE.into(), "go", tree_sitter_go::HIGHLIGHTS_QUERY),
57:         "md" | "markdown" => (tree_sitter_md::LANGUAGE.into(), "markdown", tree_sitter_md::HIGHLIGHT_QUERY_BLOCK),
58:         _ => return None,
59:     };
60:     let mut c = tree_sitter_highlight::HighlightConfiguration::new(lang, name, q, "", "").ok()?;
61:     c.configure(&NAMES);
62:     Some(c)
63: }
64: 
65: pub fn has_grammar(ext: &str) -> bool {
66:     !cfg!(target_arch = "wasm32") && matches!(ext, "rs" | "py" | "pyw" | "js" | "mjs" | "cjs" | "jsx" | "json" | "jsonc" | "toml" | "sh" | "bash" | "zsh" | "css" | "html" | "htm" | "c" | "h" | "go" | "md" | "markdown")
67: }
68: 
69: /// Classified byte ranges covering the whole of `src`, in order.
70: pub fn runs(src: &str, ext: &str) -> Vec<(usize, usize, Cls)> {
71:     let ext = ext.to_ascii_lowercase();
72:     ts_runs(src, &ext).unwrap_or_else(|| lex(src, &ext))
73: }
74: 
75: /// tree-sitter path. The C grammars do not build for `wasm32-unknown-unknown` with the current crates, so the web
76: /// preview uses the lexer below; the desktop app (and AppImage) gets full parse-tree highlighting.
77: #[cfg(target_arch = "wasm32")]
78: fn ts_runs(_src: &str, _ext: &str) -> Option<Vec<(usize, usize, Cls)>> {
79:     None
80: }
81: 
82: #[cfg(not(target_arch = "wasm32"))]
83: fn ts_runs(src: &str, ext: &str) -> Option<Vec<(usize, usize, Cls)>> {
84:     use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};
85:     let cfg = config(ext)?;
86:     let mut h = Highlighter::new();
87:     let it = h.highlight(&cfg, src.as_bytes(), None, None, |_| None).ok()?;
88:     let mut out: Vec<(usize, usize, Cls)> = vec![];
89:     let mut stack: Vec<Cls> = vec![];
90:     for ev in it {
91:         match ev {
92:             Ok(HighlightEvent::HighlightStart(Highlight(i))) => stack.push(cls_of(i)),
93:             Ok(HighlightEvent::HighlightEnd) => {
94:                 stack.pop();
95:             }
96:             Ok(HighlightEvent::Source { start, end }) if end > start && end <= src.len() => {
97:                 let c = stack.iter().rev().find(|c| **c != Cls::Plain).copied().unwrap_or(Cls::Plain);
98:                 match out.last_mut() {
99:                     Some(l) if l.2 == c && l.1 == start => l.1 = end,
100:                     _ => out.push((start, end, c)),
101:                 }
102:             }
103:             Ok(_) => {}
104:             Err(_) => break,
105:         }
106:     }
107:     if out.is_empty() {
108:         None
109:     } else {
110:         Some(out)
111:     }
112: }
113: 
114: const KEYWORDS: &[&str] = &[
115:     "fn", "let", "const", "var", "if", "else", "elif", "for", "while", "do", "return", "class", "struct", "enum", "impl", "trait", "interface", "import", "export", "from", "package", "use", "mod", "pub", "static", "public", "private", "protected", "void", "int", "new", "try", "catch", "finally", "throw", "switch", "case", "break", "continue", "true", "false", "null", "nil", "None", "True", "False", "this", "self", "super", "async", "await", "match", "in", "is", "not", "and", "or", "def", "lambda", "type", "func", "extends", "implements", "namespace", "using", "end", "then", "fi", "done", "function", "echo",
116: ];
117: 
118: /// A tiny lexer: comments, strings, numbers, a generic keyword list. Good enough to make unknown formats readable.
119: fn lex(src: &str, ext: &str) -> Vec<(usize, usize, Cls)> {
120:     let hash_comment = matches!(ext, "yaml" | "yml" | "conf" | "ini" | "cfg" | "env" | "rb" | "pl" | "mk" | "dockerfile" | "gitignore" | "txt" | "properties");
121:     let slash_comment = !hash_comment || matches!(ext, "ts" | "tsx" | "java" | "kt" | "swift" | "cpp" | "hpp" | "cc" | "cs" | "php" | "scss");
122:     let b = src.as_bytes();
123:     let mut out: Vec<(usize, usize, Cls)> = vec![];
124:     let push = |out: &mut Vec<(usize, usize, Cls)>, s: usize, e: usize, c: Cls| {
125:         if e > s {
126:             match out.last_mut() {
127:                 Some(l) if l.2 == c && l.1 == s => l.1 = e,
128:                 _ => out.push((s, e, c)),
129:             }
130:         }
131:     };
132:     let mut i = 0;
133:     while i < b.len() {
134:         let c = b[i];
135:         let start = i;
136:         if (hash_comment && c == b'#') || (slash_comment && c == b'/' && b.get(i + 1) == Some(&b'/')) || (ext == "sql" && c == b'-' && b.get(i + 1) == Some(&b'-')) {
137:             while i < b.len() && b[i] != b'\n' {
138:                 i += 1;
139:             }
140:    
```

### TOOL: shell [seq 121] 2026-11-09 13:47:30 status=completed

```json
{
  "command": "cat /home/lufuno/crates/smart-core/src/lib.rs\necho ===MODEL-ENTRY===\ngrep -n \"pub struct Entry\\|pub enum Kind\\|pub struct Location\\|impl Kind\\|from_ext\\|kind_label\\|modified\" /home/lufuno/crates/smart-core/src/model.rs | head -n 60\necho ===EXEC===\ncat /home/lufuno/crates/smart-core/src/exec.rs\necho ===VFS===\ncat /home/lufuno/crates/smart-core/src/vfs.rs"
}
```

```
//! # smart-core
//! The Smart Files engine. Everything a file manager *does* lives here; nothing it *looks like* does.
//!
//! * [`vfs`]      – storage abstraction (`Vfs`), with local disk, memory, ADB phone providers
//! * [`search`]   – query language, cached index, fuzzy ranking, content scan
//! * [`ops`]      – copy/move/trash/delete jobs across providers with progress + cancel
//! * [`devices`]  – real mount / unmount / eject, phones, network
//! * [`apps`]     – default app + “Open With…”
//! * [`backend`]  – the facade frontends use: call methods, `poll()` events
pub mod analyze;
pub mod archive;
pub mod apps;
pub mod audio;
pub mod backend;
pub mod devices;
pub mod exec;
pub mod library;
pub mod memfs;
pub mod model;
pub mod ops;
pub mod registry;
pub mod search;
pub mod thumbs;
pub mod tools;
pub mod vfs;

#[cfg(not(target_arch = "wasm32"))]
pub mod local;

#[cfg(feature = "demo")]
pub mod demo;

pub use backend::{Backend, Event, Place};
pub use devices::{Device, DeviceKind};
pub use library::{Library, Settings, SharedLibrary, TagDef};
pub use model::*;
pub use ops::{JobEvent, JobKind};
pub use search::{Hit, Query};
===MODEL-ENTRY===
7:pub struct Location {
70:pub enum Kind {
86:impl Kind {
87:    pub fn from_ext(ext: &str) -> Kind {
142:pub struct Entry {
148:    pub modified: Option<i64>,
159:    pub fn new(loc: Location, is_dir: bool, size: u64, modified: Option<i64>) -> Entry {
162:        let kind = if is_dir { Kind::Folder } else { Kind::from_ext(&ext) };
163:        Entry { hidden: name.starts_with('.'), name, loc, is_dir, size, modified, kind, ext, symlink: false, readonly: false, paced: false }
165:    pub fn kind_label(&self) -> String {
259:            SortKey::Modified => "Date modified",
316:            SortKey::Modified => b.modified.cmp(&a.modified).then_with(|| natural_cmp(&a.name, &b.name)),
318:            SortKey::Kind => a.kind_label().cmp(&b.kind_label()).then_with(|| natural_cmp(&a.name, &b.name)),
===EXEC===
//! Tiny execution shim: real threads on native, run-inline in the browser (no threads there).
pub const THREADED: bool = cfg!(not(target_arch = "wasm32"));

pub fn spawn<F: FnOnce() + Send + 'static>(f: F) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(f);
    }
    #[cfg(target_arch = "wasm32")]
    {
        f();
    }
}

pub fn now_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}
===VFS===
//! The storage abstraction every provider implements (local disk, memory, phone, cloud…).
use crate::model::{Entry, Location};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum FsError {
    #[error("not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("already exists")]
    Exists,
    #[error("not supported here")]
    Unsupported,
    #[error("cancelled")]
    Cancelled,
    #[error("{0}")]
    Other(String),
}

pub type FsResult<T> = Result<T, FsError>;

impl From<std::io::Error> for FsError {
    fn from(e: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        match e.kind() {
            NotFound => FsError::NotFound,
            PermissionDenied => FsError::PermissionDenied,
            AlreadyExists => FsError::Exists,
            _ => FsError::Other(e.to_string()),
        }
    }
}

pub struct Capacity {
    pub total: u64,
    pub free: u64,
}

/// Synchronous, thread-safe file-system provider. Paths are absolute, `/`-separated, per-provider.
pub trait Vfs: Send + Sync {
    fn id(&self) -> &str;
    fn list(&self, path: &str) -> FsResult<Vec<Entry>>;
    fn stat(&self, path: &str) -> FsResult<Entry>;
    fn open_read(&self, path: &str) -> FsResult<Box<dyn Read + Send>>;
    fn create_write(&self, path: &str) -> FsResult<Box<dyn Write + Send>>;
    fn mkdir(&self, path: &str) -> FsResult<()>;
    fn rename(&self, from: &str, to: &str) -> FsResult<()>;
    /// `to_trash` moves to the platform trash where one exists, otherwise deletes.
    fn remove(&self, path: &str, to_trash: bool) -> FsResult<()>;

    /// Real on-disk path when the provider is backed by one (enables “Open with…”).
    fn real_path(&self, _path: &str) -> Option<PathBuf> {
        None
    }
    fn capacity(&self, _path: &str) -> Option<Capacity> {
        None
    }
    fn supports_trash(&self) -> bool {
        false
    }
    fn is_writable(&self) -> bool {
        true
    }

    /// Providers that support “virtual” sizes (the web demo) can preserve them when copying.
    fn set_size_hint(&self, _path: &str, _hint: Option<u64>) -> FsResult<()> {
        Err(FsError::Unsupported)
    }

    fn read(&self, path: &str, limit: usize) -> FsResult<Vec<u8>> {
        let mut r = self.open_read(path)?;
        let mut buf = Vec::new();
        r.by_ref().take(limit as u64).read_to_end(&mut buf).map_err(FsError::from)?;
        Ok(buf)
    }
    fn write(&self, path: &str, data: &[u8]) -> FsResult<()> {
        let mut w = self.create_write(path)?;
        w.write_all(data)?;
        w.flush()?;
        Ok(())
    }
    fn exists(&self, path: &str) -> bool {
        self.stat(path).is_ok()
    }
    fn loc(&self, path: &str) -> Location {
        Location::new(self.id(), path)
    }
}

```

### TOOL: shell [seq 138] 2026-11-09 13:47:34 status=completed

```json
{
  "command": "grep -n \"^pub struct\\|^pub enum\\|^struct\\|^enum\\|^impl\\|^pub fn\\|^    pub fn\\|^fn \" /home/lufuno/crates/smart-ui-egui/src/storage.rs | head -n 80\necho ===STORAGE-HEAD===\nhead -n 80 /home/lufuno/crates/smart-ui-egui/src/storage.rs\necho ===MODEL-KIND===\nsed -n '1,200p' /home/lufuno/crates/smart-core/src/model.rs"
}
```

```
21:pub fn usage_colors(th: &Theme, used: f32) -> (Color32, Color32) {
32:pub struct GNode {
47:pub struct Graph {
59:pub struct Snap {
76:pub struct StorageState {
94:impl StorageState {
108:fn hue_for(name: &str, th: &Theme) -> Color32 {
114:fn arc_mesh(p: &egui::Painter, ctr: Pos2, r_out: f32, r_in: f32, a0: f32, a1: f32, col: Color32) {
133:impl SmartApp {
135:    pub fn storage_start(&mut self, home_scope: bool) {
189:    pub fn storage_cmd(&mut self, c: &str) {
284:    pub fn storage_appbar(&mut self, ui: &mut Ui, left: f32, right: f32, cy: f32) {
350:    pub fn draw_storage(&mut self, ui: &mut Ui, rect: Rect) {
===STORAGE-HEAD===
//! Storage workspace: opens as a tab inside Files. Overview, Cleanup (context-aware junk), Map (treemap),
//! Graph (an Obsidian-style force-directed view of the folder tree) and Biggest. Scanning runs off the UI thread on
//! desktop and is sliced across frames on the web.
use crate::app::*;
use crate::appkit::*;
use crate::chrome::*;
use crate::content::kind_color;
use crate::icons::Ic;
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Pos2, Rect, Sense, Ui, Vec2};
use smart_core::analyze::*;
use smart_core::vfs::Vfs;
use smart_core::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub const VIEWS: [(&str, &str); 5] = [("pie-chart-2", "Overview"), ("trash-bin-minimalistic", "Cleanup"), ("widget-4", "Map"), ("graph-new", "Graph"), ("sort-vertical", "Biggest")];

/// Drive gauge colours: accent, then amber from 80 %, then red from 90 %.
pub fn usage_colors(th: &Theme, used: f32) -> (Color32, Color32) {
    if used >= 0.90 {
        (mix(th.danger, Color32::WHITE, 0.22), th.danger)
    } else if used >= 0.80 {
        (mix(th.warn, Color32::WHITE, 0.22), th.warn)
    } else {
        th.accent_grad()
    }
}

#[derive(Clone)]
pub struct GNode {
    pub idx: Option<usize>,
    pub label: String,
    pub path: String,
    pub size: u64,
    pub pos: Vec2,
    pub vel: Vec2,
    pub r: f32,
    pub depth: u8,
    pub pinned: bool,
    pub junk: bool,
    pub is_dir: bool,
    pub kind: Kind,
}

pub struct Graph {
    pub focus: usize,
    pub sig: (usize, usize),
    pub nodes: Vec<GNode>,
    pub edges: Vec<(usize, usize)>,
    pub zoom: f32,
    pub pan: Vec2,
    pub hover: Option<usize>,
    pub drag: Option<usize>,
    pub heat: f32,
}

pub struct Snap {
    pub at: f64,
    pub total: u64,
    pub buckets: [u64; 6],
    pub files: u64,
    pub dirs: u64,
    pub errors: u64,
    pub finished: bool,
    pub progress: f32,
    pub cands: Vec<Junk>,
    pub kinds: Vec<(JunkKind, u64, usize)>,
    pub safe: u64,
    pub all: u64,
    pub big_files: Vec<FileInfo>,
    pub big_dirs: Vec<(String, String, u64)>,
}

pub struct StorageState {
    pub scan: Arc<Mutex<Scan>>,
    pub vfs: Option<Arc<dyn Vfs>>,
    pub scope_home: bool,
    pub view: usize,
===MODEL-KIND===
//! Plain data types shared by every frontend. No UI, no I/O.
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};
use serde::{Deserialize, Serialize};

/// A place in the virtual namespace: which file system, and a `/`-separated absolute path in it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Location {
    pub fs: String,
    pub path: String,
}

impl Location {
    pub fn new(fs: impl Into<String>, path: impl Into<String>) -> Self {
        let mut path: String = path.into();
        if path.is_empty() {
            path = "/".into();
        }
        while path.len() > 1 && path.ends_with('/') {
            path.pop();
        }
        Self { fs: fs.into(), path }
    }
    pub fn file(path: impl Into<String>) -> Self {
        Self::new("file", path)
    }
    pub fn mem(path: impl Into<String>) -> Self {
        Self::new("mem", path)
    }
    pub fn is_root(&self) -> bool {
        self.path == "/"
    }
    pub fn parent(&self) -> Option<Location> {
        if self.is_root() {
            return None;
        }
        let idx = self.path.rfind('/')?;
        let p = if idx == 0 { "/" } else { &self.path[..idx] };
        Some(Location::new(self.fs.clone(), p))
    }
    pub fn name(&self) -> String {
        if self.is_root() {
            return self.fs.clone();
        }
        self.path.rsplit('/').next().unwrap_or("").to_string()
    }
    pub fn join(&self, name: &str) -> Location {
        let base = if self.is_root() { String::new() } else { self.path.clone() };
        Location::new(self.fs.clone(), format!("{}/{}", base, name))
    }
    pub fn starts_with(&self, other: &Location) -> bool {
        self.fs == other.fs
            && (self.path == other.path
                || other.is_root()
                || self.path.starts_with(&format!("{}/", other.path)))
    }
    /// Path segments from the root, for breadcrumbs.
    pub fn ancestors(&self) -> Vec<Location> {
        let mut v = vec![];
        let mut cur = Some(self.clone());
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
```

### TOOL: shell [seq 149] 2026-11-09 13:47:38 status=completed

```json
{
  "command": "echo ===DESKTOP-DIFF-HEAD===\ndiff /tmp/opencode/canonical-photos-audit/home-crates/photos.home.rs /tmp/opencode/canonical-photos-audit/zip-desktop/smart-desktop/apps/photos/mod.rs | head -n 250\necho ===DESKTOP-DIFF-STAT===\ndiff /home/lufuno/crates/smart-ui-egui/src/photos.rs /tmp/opencode/canonical-photos-audit/zip-desktop/smart-desktop/apps/photos/mod.rs 2>&1 | wc -l\necho ===CAPTURE-DESKTOP===\nunzip -l /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip | grep -i \"capture\\|settings\\|assistant\\|files/\" | head -n 30"
}
```

```
===DESKTOP-DIFF-HEAD===
466c466
<     fn photos_thumb(&mut self, ctx: &egui::Context, l: &Location, budget: &mut u32) -> Option<(egui::TextureId, [f32; 2])> {
---
>     pub(crate) fn photos_thumb(&mut self, ctx: &egui::Context, l: &Location, budget: &mut u32) -> Option<(egui::TextureId, [f32; 2])> {
739a740,748
>             // Ask Smart (app-level AI, same pattern as Files)
>             if avail > 460.0 {
>                 let askw = self.tw(&p, "Ask Smart", &f_semi(T_BODY)) + 44.0;
>                 let ask = Rect::from_min_size(pos2(rx - askw, cy - 17.0), vec2(askw, 34.0));
>                 if self.label_button(ui, ask, Some("stars-minimalistic"), "Ask Smart", "ph-ask", Btn::Tonal).on_hover_text("Ask your assistant about this library").clicked() {
>                     self.push_act(Act::App("ai.open:photos".into()));
>                 }
>                 rx = ask.left() - 8.0;
>             }
1223c1232
<                 let s = spring_to(&ctx, ("ph-fs", &e.loc), if on { 1.0 } else if hov { 0.4 } else { 0.0 }, 300.0, 22.0);
---
>                 let s = spring_to(&ctx, ("ph-fs", &e.loc), if on { 1.0 } else if hov { 0.4 } else { 0.0 }, 300.0, 40.0).clamp(0.0, 1.0);
===DESKTOP-DIFF-STAT===
18
===CAPTURE-DESKTOP===
        0  11-05-2026 08:48   smart-desktop/apps/assistant/
    25539  11-05-2026 08:48   smart-desktop/apps/assistant/mod.rs
        0  11-05-2026 08:48   smart-desktop/apps/capture/
    19750  11-05-2026 08:48   smart-desktop/apps/capture/mod.rs
        0  11-05-2026 08:48   smart-desktop/apps/files/
    67631  11-05-2026 08:48   smart-desktop/apps/files/app.rs
    69663  11-05-2026 08:48   smart-desktop/apps/files/chrome.rs
    46532  11-05-2026 08:48   smart-desktop/apps/files/content.rs
    22216  11-05-2026 08:48   smart-desktop/apps/files/copyfx.rs
      196  11-05-2026 08:48   smart-desktop/apps/files/mod.rs
    95268  11-05-2026 08:48   smart-desktop/apps/files/modals.rs
    75426  11-05-2026 08:48   smart-desktop/apps/files/overlays.rs
    12482  11-05-2026 08:48   smart-desktop/apps/files/picker.rs
        0  11-05-2026 08:48   smart-desktop/apps/settings/
    18792  11-05-2026 08:48   smart-desktop/apps/settings/mod.rs
    27695  11-05-2026 08:48   smart-desktop/protocol/schema/capture.capnp

```

### TOOL: shell [seq 156] 2026-11-09 13:47:47 status=completed

```json
{
  "command": "mkdir -p /tmp/opencode/canonical-photos-audit/zip-desktop2\nunzip -o -q /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip \"smart-desktop/apps/capture/mod.rs\" \"smart-desktop/apps/settings/mod.rs\" \"smart-desktop/apps/assistant/mod.rs\" \"smart-desktop/apps/files/app.rs\" -d /tmp/opencode/canonical-photos-audit/zip-desktop2\nwc -l -c /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/capture/mod.rs /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/settings/mod.rs /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/assistant/mod.rs\necho ===CAPTURE-FUNCS===\ngrep -n \"^pub struct\\|^pub enum\\|^struct\\|^enum\\|^impl\\|^pub fn\\|^    pub fn\\|^fn \\|^    fn \" /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/capture/mod.rs | head -n 60\necho ===SETTINGS-FUNCS===\ngrep -n \"photos\\|Photos\" /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/settings/mod.rs | head -n 40\necho ===ASSISTANT-AI===\ngrep -n \"ai.open\\|photos\\|Photos\" /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/assistant/mod.rs | head -n 40\necho ===FILES-APP-PHOTOS-REFS===\ngrep -n \"photos\\|Photos\\|WallpaperFrom\\|Picker\" /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/files/app.rs | head -n 60"
}
```

```
  400 19750 /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/capture/mod.rs
  300 18792 /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/settings/mod.rs
  450 25539 /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/assistant/mod.rs
 1150 64081 total
===CAPTURE-FUNCS===
17:pub struct Capture {
31:impl Capture {
32:    pub fn recording_secs(&self, now: f64) -> Option<f32> {
44:    fn download(blob: &web_sys::Blob, name: &str) {
54:    fn attach(stream: &web_sys::MediaStream, name: String, mime: &'static str) -> bool {
86:    pub fn start_video(stamp: &str) -> bool {
93:    pub fn start_audio(stamp: String) {
108:    pub fn stop() {
117:impl SmartApp {
118:    fn stamp(&self) -> String {
284:    pub fn draw_capture(&mut self, ui: &mut Ui, fr: &Frame) {
===SETTINGS-FUNCS===
===ASSISTANT-AI===
189:            Some(AppId::Photos) => vec!["Make this photo warmer", "Snap photos to the left", "Set a new wallpaper"],
192:            Some(AppId::Capture) => vec!["Take a screenshot", "Open Photos", "Tile my windows"],
201:        let s = spring_to(&ctx, "aipal", if self.shell.ai.open { 1.0 } else { 0.0 }, 260.0, 25.0);
306:            self.shell.ai.open = false;
335:            self.shell.ai.open = false;
===FILES-APP-PHOTOS-REFS===
110:    WallpaperFrom(Location),
129:    /// Command for one of the suite apps ("term.new_tab", "photos.fit" …).
137:    OpenInPhotos(Location),
1243:            Act::WallpaperFrom(l) => self.apply_wallpaper_image(l),
1375:            Act::OpenInPhotos(l) => {
1376:                self.open_app(crate::shell::AppId::Photos);
1377:                self.photos_show(l);

```

### TOOL: shell [seq 163] 2026-11-09 13:47:52 status=completed

```json
{
  "command": "echo ===EIGA-LIB===\ncat /home/lufuno/crates/eiga-core/src/lib.rs\necho ===EIGA-PLAYER-HEAD===\nhead -n 120 /home/lufuno/crates/eiga-core/src/player.rs\ngrep -n \"thumb\\|Thumb\\|cache\\|LRU\\|mtime\\|budget\\|memory\\|ColorImage\\|load_texture\" /home/lufuno/crates/eiga-core/src/player.rs | head -n 40\necho ===EIGA-SOURCE-HEAD===\nhead -n 80 /home/lufuno/crates/eiga-core/src/source.rs\ngrep -n \"thumb\\|cache\\|mtime\\|EXIF\\|exif\\|jpeg\\|decode\" /home/lufuno/crates/eiga-core/src/source.rs | head -n 40"
}
```

```
===EIGA-LIB===
//! # eiga-core — the engine behind **Eiga (映画)**, the Smart suite's video player.
//!
//! * [`subs`]   SRT / WebVTT / ASS-SSA parsing, encoding detection, fast "what is on screen now" lookup.
//! * [`player`] pure playback state machine (seek, speed, A-B loop, chapters, frame-step), playlist and key map.
//! * [`source`] ffprobe parsing, a procedural test-card source (works everywhere, incl. WebAssembly) and — on native
//!   targets — an `ffmpeg` pipe source that decodes any format ffmpeg knows.
//!
//! Nothing in this crate draws, opens windows or reads clocks, so every behaviour is unit-tested.
pub mod player;
pub mod source;
pub mod subs;

pub use player::{Cmd, Event, LoopMode, Player, Playlist, Repeat};
pub use source::{Frame, MediaInfo};
pub use subs::{Cue, SubTrack};

/// `83.4` → `1:23`, `3723` → `1:02:03`.
pub fn fmt_time(secs: f64) -> String {
    if !secs.is_finite() || secs < 0.0 {
        return "0:00".into();
    }
    let s = secs as u64;
    let (h, m, s) = (s / 3600, (s / 60) % 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

/// Like [`fmt_time`] with milliseconds, used for sync offsets and the stats overlay.
pub fn fmt_time_ms(secs: f64) -> String {
    if !secs.is_finite() || secs < 0.0 {
        return "0:00.000".into();
    }
    let ms = (secs * 1000.0).round() as u64;
    let (h, m, s, ms) = (ms / 3_600_000, (ms / 60_000) % 60, (ms / 1000) % 60, ms % 1000);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}.{ms:03}")
    } else {
        format!("{m}:{s:02}.{ms:03}")
    }
}

/// Parses `90`, `1:30`, `1:02:03.5`. Used by "jump to time".
pub fn parse_time(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut total = 0.0;
    for part in s.split(':') {
        let v: f64 = part.trim().parse().ok()?;
        if !v.is_finite() || v < 0.0 {
            return None;
        }
        total = total * 60.0 + v;
    }
    if s.split(':').count() > 3 {
        return None;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_formatting() {
        assert_eq!(fmt_time(0.0), "0:00");
        assert_eq!(fmt_time(83.9), "1:23");
        assert_eq!(fmt_time(3723.0), "1:02:03");
        assert_eq!(fmt_time(f64::NAN), "0:00");
        assert_eq!(fmt_time(-4.0), "0:00");
        assert_eq!(fmt_time_ms(3723.456), "1:02:03.456");
        assert_eq!(fmt_time_ms(5.5), "0:05.500");
    }

    #[test]
    fn time_parsing() {
        assert_eq!(parse_time("90"), Some(90.0));
        assert_eq!(parse_time("1:30"), Some(90.0));
        assert_eq!(parse_time("1:02:03.5"), Some(3723.5));
        assert_eq!(parse_time(" 0:05 "), Some(5.0));
        assert_eq!(parse_time(""), None);
        assert_eq!(parse_time("a:b"), None);
        assert_eq!(parse_time("-3"), None);
        assert_eq!(parse_time("1:2:3:4"), None);
        assert_eq!(parse_time("inf"), None);
    }
}
===EIGA-PLAYER-HEAD===
//! Playback state machine, playlist and key map. Pure logic: feed it `tick(dt)`, read back [`Event`]s.
use crate::subs::SubTrack;

pub const SPEEDS: [f64; 10] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
pub const MAX_VOLUME: f32 = 130.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LoopMode {
    Off,
    /// Repeat the current file.
    File,
    /// Repeat between A and B. `b` is `None` while the user has only set A.
    Ab { a: f64, b: Option<f64> },
}

/// What the engine/UI must do in response to a state change.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    /// Restart decoding at this media time.
    Seek(f64),
    Ended,
    PauseChanged(bool),
    SpeedChanged(f64),
    VolumeChanged,
    /// Short message for the on-screen display ("Speed ×1.25").
    Osd,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Chapter {
    pub start: f64,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub duration: f64,
    pub position: f64,
    pub paused: bool,
    pub speed: f64,
    pub volume: f32,
    pub muted: bool,
    pub looping: LoopMode,
    pub eof: bool,
    pub fps: f64,
    pub audio_delay: f64,
    pub chapters: Vec<f64>,
    pub osd: Option<String>,
}

impl Default for Player {
    fn default() -> Self {
        Player { duration: 0.0, position: 0.0, paused: false, speed: 1.0, volume: 100.0, muted: false, looping: LoopMode::Off, eof: false, fps: 25.0, audio_delay: 0.0, chapters: vec![], osd: None }
    }
}

impl Player {
    pub fn new(duration: f64, fps: f64) -> Player {
        Player { duration: if duration.is_finite() { duration.max(0.0) } else { 0.0 }, fps: if fps.is_finite() && fps > 1.0 && fps < 480.0 { fps } else { 25.0 }, ..Player::default() }
    }

    fn say(&mut self, s: impl Into<String>) {
        self.osd = Some(s.into());
    }

    pub fn clamp_pos(&self, t: f64) -> f64 {
        if !t.is_finite() {
            return 0.0;
        }
        if self.duration > 0.0 {
            t.clamp(0.0, (self.duration - 0.001).max(0.0))
        } else {
            t.max(0.0)
        }
    }

    pub fn fraction(&self) -> f64 {
        if self.duration > 0.0 {
            (self.position / self.duration).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Advance the clock. Returns events (A-B wrap, loop, end of file).
    pub fn tick(&mut self, dt: f64) -> Vec<Event> {
        let mut ev = vec![];
        if self.paused || self.eof || !dt.is_finite() || dt <= 0.0 {
            return ev;
        }
        let dt = dt.min(0.25); // a stalled frame must not skip half a minute of video
        self.position += dt * self.speed;
        if let LoopMode::Ab { a, b: Some(b) } = self.looping {
            if self.position >= b {
                self.position = a;
                ev.push(Event::Seek(a));
                return ev;
            }
        }
        if self.duration > 0.0 && self.position >= self.duration {
            if self.looping == LoopMode::File {
                self.position = 0.0;
                ev.push(Event::Seek(0.0));
            } else {
                self.position = self.duration;
                self.eof = true;
                self.paused = true;
                ev.push(Event::Ended);
            }
        }
        ev
    }

    pub fn seek_abs(&mut self, t: f64) -> Vec<Event> {
        self.position = self.clamp_pos(t);
        self.eof = false;
        vec![Event::Seek(self.position)]
    }
    pub fn seek_rel(&mut self, d: f64) -> Vec<Event> {
        let was_eof = self.eof;
        let base = if was_eof { self.duration } else { self.position };
423:/// Defaults follow mpv wherever mpv has an opinion, so muscle memory transfers.
790:    fn mpv_muscle_memory_is_preserved() {
===EIGA-SOURCE-HEAD===
//! Media metadata (ffprobe), frame sources (procedural test card everywhere; ffmpeg pipe on native).
use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VideoStream {
    pub index: usize,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bitrate: Option<u64>,
    pub pix_fmt: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AudioStream {
    pub index: usize,
    pub codec: String,
    pub channels: u32,
    pub sample_rate: u32,
    pub lang: String,
    pub title: String,
    pub default: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubStream {
    pub index: usize,
    pub codec: String,
    pub lang: String,
    pub title: String,
    pub default: bool,
    pub forced: bool,
}

impl SubStream {
    /// Text subtitles can be extracted and rendered; bitmap ones (PGS, VobSub) cannot.
    pub fn is_text(&self) -> bool {
        matches!(self.codec.as_str(), "subrip" | "srt" | "ass" | "ssa" | "webvtt" | "mov_text" | "text")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaInfo {
    pub title: String,
    pub container: String,
    pub duration: f64,
    pub size: u64,
  
```

### TOOL: shell [seq 170] 2026-11-09 13:47:55 status=completed

```json
{
  "command": "echo ===VIDEO.RS-THUMB-CACHE===\ngrep -n \"thumb\\|Thumb\\|cache\\|LRU\\|lru\\|mtime\\|budget\\|texture\\|ColorImage\\|load_texture\\|failed\\|HashMap\" /home/lufuno/crates/smart-ui-egui/src/video.rs | head -n 80\necho ===VIDEO-STRUCT===\ngrep -n \"^pub struct\\|^pub enum\\|^struct\\|^enum\\|^impl\\|^pub fn\\|^    pub fn\\|^fn \\|^    fn \" /home/lufuno/crates/smart-ui-egui/src/video.rs | head -n 80\necho ===PHOTOS-THUMB-COMPARE===\ngrep -n \"thumb\\|Thumb\\|budget\\|thumbnail\\|ColorImage\\|load_texture\\|failed\" /home/lufuno/crates/smart-ui-egui/src/photos.rs | head -n 40"
}
```

```
===VIDEO.RS-THUMB-CACHE===
397:    /// Frees decoder, textures and audio when the window closes so memory returns to the system.
682:                            Err(er) => format!("Screenshot failed: {er}"),
873:                let img = egui::ColorImage::from_rgba_unmultiplied([f.w, f.h], &f.rgba);
876:                    None => e.tex = Some(ctx.load_texture("eiga-frame", img, egui::TextureOptions::LINEAR)),
944:                let mut m = egui::epaint::Mesh::with_texture(tex.id());
954:                textured_rrect(&vp, dest, 16.0, tex.id(), uv, Color32::WHITE);
===VIDEO-STRUCT===
19:pub struct AudioOut {
26:impl AudioOut {
27:    fn new() -> Result<Self, String> {
33:    fn restart(&mut self, path: &str, start: f64, track: usize, speed: f64, gain: f32, paused: bool) {
42:    fn pump(&mut self) {
55:pub struct Eiga {
101:impl Default for Eiga {
102:    fn default() -> Self {
107:fn key_name(k: Key, shift: bool) -> Option<&'static str> {
171:impl SmartApp {
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
===PHOTOS-THUMB-COMPARE===
64:    pub thumbs: HashMap<Location, (TextureHandle, [f32; 2])>,
65:    pub failed: HashSet<Location>,
110:            thumbs: HashMap::new(),
111:            failed: HashSet::new(),
226:        let mut budget = 6000;
231:                if budget == 0 {
234:                budget -= 1;
462:            Err(e) => self.toast(format!("Save failed: {e}"), false),
466:    fn photos_thumb(&mut self, ctx: &egui::Context, l: &Location, budget: &mut u32) -> Option<(egui::TextureId, [f32; 2])> {
467:        if let Some((t, s)) = self.shell.photos.thumbs.get(l) {
470:        if *budget == 0 || self.shell.photos.failed.contains(l) {
473:        *budget -= 1;
476:            self.shell.photos.failed.insert(l.clone());
479:        let t = img.thumbnail(420, 420).to_rgba8();
481:        let tex = ctx.load_texture(format!("ph-t-{}", l.path), egui::ColorImage::from_rgba_unmultiplied(sz, t.as_raw()), TextureOptions::LINEAR);
483:        self.shell.photos.thumbs.insert(l.clone(), (tex, [sz[0] as f32, sz[1] as f32]));
503:        let mk = |data: &[u8], w: usize, h: usize, name: &str| ctx.load_texture(name.to_string(), egui::ColorImage::from_rgba_unmultiplied([w, h], data), TextureOptions::LINEAR);
515:        let small = image::imageops::thumbnail(&rgba, 120, 120);
529:            c.tex = if adj == Adjust::default() { c.orig.clone() } else { ctx.load_texture(format!("ph-e-{}", c.loc.path), egui::ColorImage::from_rgba_unmultiplied([c.w, c.h], &process(&c.rgba, c.w, c.h, &adj)), TextureOptions::LINEAR) };
575:            mi("widget", "Thumbnail size", Act::Toast(String::new())).sub(sizes),
577:            mi("crop-minimalistic", "Fill thumbnails", a("fill")).on(ph.fill),
886:        let mut budget = 2u32;
906:                match self.photos_thumb(ui.ctx(), &e.loc, &mut budget) {
1213:            let mut budget = 2u32;
1225:                match self.photos_thumb(&ctx, &e.loc, &mut budget) {
1413:                    SRow::Choice { icon: "crop-minimalistic", label: "Thumbnails", sub: "Fill crops to a tile, Fit shows the whole photo", opts: &["Fill", "Fit"], sel: if ph.fill { 0 } else { 1 } },
1414:                    SRow::Toggle { icon: "text", label: "Always show captions", sub: "File name under each thumbnail", on: ph.captions },
1415:                    SRow::Slider { icon: "gallery", label: "Thumbnail size", sub: "Larger tiles, fewer columns", v: ph.tile, lo: 120.0, hi: 340.0, unit: " px" },
1416:                    SRow::Slider { icon: "widget", label: "Grid spacing", sub: "Gap between thumbnails", v: ph.gap, lo: 4.0, hi: 24.0, unit: " px" },
1427:                    SRow::Button { icon: "trash-bin-minimalistic", label: "Reset thumbnails", sub: "Drop the thumbnail cache and rebuild", cta: "Reset" },
1452:                    ph.thumbs.clear();
1453:                    ph.failed.clear();
1454:                    self.toast("Thumbnails will rebuild", true);

```

### TOOL: shell [seq 177] 2026-11-09 13:47:58 status=completed

```json
{
  "command": "echo ===PROTOCOL-MEDIA===\ncat /home/lufuno/smart/smart-protocol-rs/schema/media.capnp\necho ===PROTOCOL-STORAGE===\ncat /home/lufuno/smart/smart-protocol-rs/schema/storage.capnp\necho ===PROTOCOL-FILES-HEAD===\nhead -n 120 /home/lufuno/smart/smart-protocol-rs/schema/files.capnp"
}
```

```
;
    cancelled @4;
    custom @5;
}
===PROTOCOL-STORAGE===
@0xa1b2c3d4e5f60103;

using T = import "types.capnp";
using M = import "mount.capnp";

# ═══════════════════════════════════════════════════════════════════════
# Storage — block device management, LVM/RAID, storage pools.
#
# Zero backend assumptions: no LVM, no mdadm, no ZFS, no Btrfs,
# no Storage Spaces, no VxVM. Backend provides storage management;
# protocol defines the contract.
#
# hal.capnp covers hardware; this covers OS-level storage pools.
#
# Revamp fixes (this wave):
#   - The header's promise is now true. It opens with "no LVM, no mdadm,
#     no ZFS, no Btrfs, no Storage Spaces, no VxVM" and then declared
#     `PoolType` — a REQUIRED field — whose values are exactly those
#     products. A consumer reading the header would pass values the
#     header promised cannot exist. The pool/bus/redundancy/encryption
#     vocabularies are open tokens now: reported, never prescribed.
#   - The promoted typed IDs are adopted (T.StoragePoolId / T.VolumeId /
#     T.SnapshotId); the local shadow structs are deleted.
#   - Day-long operations (scrub, rebuild, migration, replication) are
#     subscribable. They were reachable only by polling a getter, so a
#     consumer monitored a 40 TB scrub with an RPC loop for hours.
#   - `FilesystemInfo` and `FilesystemCheckResult` come from
#     mount.capnp, their canonical home — this file declared private
#     copies with different shapes.
#   - Block I/O can name its buffer (T.BufferId beside the slot), so the
#     zero-copy block path is expressible instead of out-of-band.
#   - The block-device driver (BlockProvider) is reachable: acquire it
#     by presenting a capability, per the ai.capnp precedent.
#
# Used by: mount.capnp (mount sources), device.capnp (storage devices)
# ═══════════════════════════════════════════════════════════════════════

interface StorageService {
    # List all block devices.
    listDevices @0 (filter :BlockDeviceFilter, pagination :T.Pagination) -> (devices :List(BlockDevice), totalCount :UInt32);

    # Get detailed info for a block device.
    getDevice @1 (deviceId :T.DeviceId) -> (device :BlockDevice);

    # List all storage pools.
    listPools @2 (pagination :T.Pagination) -> (pools :List(StoragePool), totalCount :UInt32);

    # Get detailed info for a storage pool.
    getPool @3 (poolId :T.StoragePoolId) -> (pool :StoragePool);

    # Create a new storage pool.
    createPool @4 (request :CreatePoolRequest) -> (pool :StoragePool);

    # Resize a storage pool (add/remove devices).
    resizePool @5 (poolId :T.StoragePoolId, request :ResizePoolRequest) -> T.VoidResult;

    # Delete a storage pool.
    deletePool @6 (poolId :T.StoragePoolId, force :Bool) -> T.VoidResult;

    # Create a volume in a pool.
    createVolume @7 (request :CreateVolumeRequest) -> (volume :Volume);

    # Resize a volume.
    resizeVolume @8 (volumeId :T.VolumeId, newSize :UInt64) -> T.VoidResult;

    # Delete a volume.
    deleteVolume @9 (volumeId :T.VolumeId, force :Bool) -> T.VoidResult;

    # List volumes in a pool.
    listVolumes @10 (poolId :T.StoragePoolId, pagination :T.Pagination) -> (volumes :List(Volume), totalCount :UInt32);

    # Get volume info.
    getVolume @11 (volumeId :T.VolumeId) -> (volume :Volume);

    # Create a snapshot of a volume.
    createSnapshot @12 (volumeId :T.VolumeId, name :Text) -> (snapshot :Snapshot);

    # List snapshots of a volume.
    listSnapshots @13 (volumeId :T.VolumeId, pagination :T.Pagination) -> (snapshots :List(Snapshot), totalCount :UInt32);

    # Rollback a volume to a snapshot.
    rollbackSnapshot @14 (volumeId :T.VolumeId, snapshotId :T.SnapshotId) -> T.VoidResult;

    # Delete a snapshot.
    deleteSnapshot @15 (snapshotId :T.SnapshotId) -> T.VoidResult;

    # Check and repair filesystem on a volume.
    checkFilesystem @16 (volumeId :T.VolumeId, repair :Bool) -> (result :M.FilesystemCheckResult);

    # Get SMART data for a block device.
    getSmartData @17 (deviceId :T.DeviceId) -> (smart :SmartData);

    # Run SMART self-test.
    runSmartTest @18 (deviceId :T.DeviceId, type :SmartTestType) -> (testId :Text);

    # Get SMART test results.
    getSmartTestResults @19 (deviceId :T.DeviceId, pagination :T.Pagination) -> (results :List(SmartTestResult), totalCount :UInt64);

    # Subscribe to storage events (device added/removed, pool state changes, etc.).
    subscribeStorage @20 () -> (subscription :StorageSubscription);

    # Get I/O statistics for a device or pool.
    getIoStats @21 (target :IoStatsTarget, duration :T.Duration) -> (stats :IoStats);

    # Initiate a scrub on a storage pool (ZFS, btrfs).
    scrubPool @22 (poolId :T.StoragePoolId) -> (scrubId :Text);

    # Get scrub progress for a pool.
    getScrubProgress @23 (poolId :T.StoragePoolId) -> (progress :ScrubProgress);

    # Manage cache tier on a pool (add/remove cache device).
    manageCacheTier @24 (poolId :T.StoragePoolId, request :CacheTierRequest) -> T.VoidResult;

    # Partition a disk (create/modify/delete partition table).
    partitionDisk @25 (request :PartitionDiskRequest) -> (result :PartitionDiskResult);

    # Encrypt a volume (LUKS/dm-crypt).
    encryptVolume @26 (request :EncryptVolumeRequest) -> T.VoidResult;

    # ─── RAID Rebuild ──────────────────────────────────────────

    # Initiate RAID rebuild on a degraded pool.
    rebuildPool @27 (poolId :T.StoragePoolId, failedDevice :T.DeviceId, replacementDevice :T.DeviceId) -> (rebuildId :Text);

    # Get RAID rebuild progress.
    getRebuildProgress @28 (poolId :T.StoragePoolId) -> (progress :RebuildProgress);

    # ─── Thin Pool Management ──────────────────────────────────

    # Create a thin pool.
    createThinPool @29 (request :CreateThinPoolRequest) -> (pool :StoragePool);

    # Resize a thin volume.
    resizeThinVolume @30 (volumeId :T.VolumeId, newSize :UInt64) -> T.VoidResult;

    # Get thin pool statistics.
    getThinPoolStats @31 (poolId :T.StoragePoolId) -> (stats :ThinPoolStats);

    # ─── Device Replacement ────────────────────────────────────

    # Replace a failed device in a pool (hot-swap).
    replaceDevice @32 (poolId :T.StoragePoolId, failedDevice :T.DeviceId, newDevice :T.DeviceId) -> T.VoidResult;

    # ─── Pool Export / Import ──────────────────────────────────

    # Export a pool (make it portable).
    exportPool @33 (poolId :T.StoragePoolId) -> (exportData :T.DataHandle);

    # Import a pool from exported data.
    importPool @34 (request :ImportPoolRequest) -> (pool :StoragePool);

    # ─── Volume Clone ──────────────────────────────────────────

    # Create an instant copy (clone) of a volume.
    cloneVolume @35 (request :CloneVolumeRequest) -> (volume :Volume);

    # ─── Volume Migration ──────────────────────────────────────

    # Live-migrate a volume between pools/devices.
    migrateVolume @36 (request :MigrateVolumeRequest) -> (migrationId :Text);

    # Get migration progress.
    getMigrationProgress @37 (migrationId :Text) -> (progress :MigrationProgress);

    # ─── Storage Tiering ───────────────────────────────────────

    # Manage storage tier assignment for a volume.
    manageStorageTier @38 (request :StorageTierRequest) -> T.VoidResult;

    # Get current tier status for a volume.
    getStorageTierStatus @39 (volumeId :T.VolumeId) -> (status :StorageTierStatus);

    # ─── Replication ───────────────────────────────────────────

    # Set up async/sync replication between pools.
    replicatePool @40 (request :ReplicatePoolRequest) -> (replicationId :Text);

    # Get replication status.
    getReplicationStatus @41 (replicationId :Text) -> (status :PoolReplicationStatus);

    # Stop replication.
    stopReplication @42 (replicationId :Text) -> T.VoidResult;

    # ─── I/O Stats Streaming ───────────────────────────────────

    # Subscribe to real-time I/O statistics.
    subscribeIoStats @43 (target :IoStatsTarget, interval :T.Duration) -> (subsc
```

### TOOL: shell [seq 184] 2026-11-09 13:48:01 status=completed

```json
{
  "command": "ls -lh /home/lufuno/smart/smart-protocol-rs/schema/media.capnp\nwc -l /home/lufuno/smart/smart-protocol-rs/schema/media.capnp /home/lufuno/smart/smart-protocol-rs/schema/files.capnp /home/lufuno/smart/smart-protocol-rs/schema/camera.capnp /home/lufuno/smart/smart-protocol-rs/schema/backup.capnp /home/lufuno/smart/smart-protocol-rs/schema/location.capnp /home/lufuno/smart/smart-protocol-rs/schema/display.capnp 2>&1\necho ===MEDIA-HEAD===\nhead -n 200 /home/lufuno/smart/smart-protocol-rs/schema/media.capnp"
}
```

```
-rwxr-xr-x 1 lufuno lufuno 17K Oct 29 13:14 /home/lufuno/smart/smart-protocol-rs/schema/media.capnp
   433 /home/lufuno/smart/smart-protocol-rs/schema/media.capnp
   832 /home/lufuno/smart/smart-protocol-rs/schema/files.capnp
   622 /home/lufuno/smart/smart-protocol-rs/schema/camera.capnp
   473 /home/lufuno/smart/smart-protocol-rs/schema/backup.capnp
   318 /home/lufuno/smart/smart-protocol-rs/schema/location.capnp
   958 /home/lufuno/smart/smart-protocol-rs/schema/display.capnp
  3636 total
===MEDIA-HEAD===
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

# -- Audit-9: Download subscription --
interface DownloadSubscription {
    pull @0 () -> (status :DownloadStatus);
    cancel @1 () -> T.VoidResult;
}

# -- Audit-9: Live interaction subscription --
interface LiveInteractionSubscription {
    pull @0 () -> (interactions :List(LiveInteraction));
    cancel @1 () -> T.VoidResult;
}

# -- Audit-9: Transcode subscription --
interface TranscodeSubscription {
    pull @0 () -> (progress :TranscodeProgress);
    cancel @1 () -> T.VoidResult;
}

struct PlayerEvent {
    union {
        stateChanged @0 :PlaybackStatus;
        metadataChanged @1 :MediaMetadata;
        positionChanged @2 :T.Duration;
        volumeChanged @3 :Float32;
        # Forward compatibility: a backend reporting a playback event the
        # contract never named (a DRM licence change, a transport stall
        # class, a new interpolation mode) must still be dispatchable
        # rather than arriving as an unrecognisable union member with no
        # data (TIMELESSNESS.md:98, DESIGN_PRINCIPLES "Forward
        # compatibility"). Batch B found and deferred this gap; closed
        # here so the file honours the convention every other event union
        # in the tree already follows.
        unknown @4 :T.UnknownEvent;
    }
}

# ── Player ──────────────────────────────────────────────────

struct MediaPlayer {
    id @0 :T.MediaPlayerId;
    name @1 :Text;
    identity @2 :Text;
    status @3 :PlaybackStatus;
    shuffle @4 :Bool;
    repeat @5 :RepeatMode;
    # Playback rate: 1.0 = normal, 2.0 = double speed, 0.5 = half speed.
    rate @6 :Float32;
    metadata @7 :MediaMetadata;
    # Audit-8 additions:
    syncGroup @8 :Text;
    supportedFormats @9 :List(Text);
}

enum PlaybackStatus {
    stopped @0;
    playing @1;
    paused @2;
    forward @3;
    rewind @4;
    custom @5;
}

enum RepeatMode {
    none @0;
    one @1;
    all @2;
    custom @3;
}

# ── Metadata ────────────────────────────────────────────────

struct Media
```

### TOOL: shell [seq 191] 2026-11-09 13:48:04 status=completed

```json
{
  "command": "sed -n '200,433p' /home/lufuno/smart/smart-protocol-rs/schema/media.capnp\necho ===CAMERA-HEAD===\nhead -n 100 /home/lufuno/smart/smart-protocol-rs/schema/camera.capnp\necho ===BACKUP-HEAD===\nhead -n 80 /home/lufuno/smart/smart-protocol-rs/schema/backup.capnp\necho ===LOCATION-HEAD===\nhead -n 80 /home/lufuno/smart/smart-protocol-rs/schema/location.capnp"
}
```

```
    year @6 :UInt32;
    trackNumber @7 :UInt32;
    discNumber @8 :UInt32;
    duration @9 :T.Duration;
    artUri @10 :Text;
    # Artwork data — use buffer handle for images >1KB.
    artData @11 :T.DataHandle;
    # Audit-8 additions:
    contentHash @12 :Data;
    encrypted @13 :Bool;
    aiTags @14 :T.StringMap;
    chapters @15 :List(Chapter);
    lyrics @16 :LyricsData;
}

# ── Item ────────────────────────────────────────────────────

struct MediaItem {
    id @0 :T.MediaItemId;
    uri @1 :T.Uri;
    metadata @2 :MediaMetadata;
    # Audit-8 additions:
    contentHash @3 :Data;
    encrypted @4 :Bool;
}

# ════════════════════════════════════════════════════════════════════════
# Audit-8 additions: live-stream, content-addressed media, chapters,
# lyrics, sync-aware playback, playlist management, format negotiation,
# transcoding, recommendations, analytics, sharing, download, crossfade.
# ════════════════════════════════════════════════════════════════════════

# ── Chapter ────────────────────────────────────────────────────────────
# Chapter markers for long-form content (audiobooks, podcasts, video series).
# Enables chapter-based navigation and display.
struct Chapter {
    title @0 :Text;
    startTime @1 :T.Duration;
    endTime @2 :T.Duration;
    description @3 :Text;
}

# ── Lyrics Data ────────────────────────────────────────────────────────
# Synchronized lyrics with timestamps. Supports line-by-line,
# word-by-word, and karaoke display modes.
struct LyricsData {
    lines @0 :List(LyricLine);
    syncType @1 :SyncType;
    language @2 :Text;
}

struct LyricLine {
    text @0 :Text;
    startTime @1 :T.Duration;
    endTime @2 :T.Duration;
}

enum SyncType {
    unsynchronized @0;
    lineSync @1;
    wordSync @2;
    custom @3;
}

# ── Live Stream ────────────────────────────────────────────────────────
# Real-time broadcast content. Low-latency delivery for radio,
# live events, and streaming with viewer analytics.
struct LiveStream {
    streamUrl @0 :T.Uri;
    latency @1 :T.Duration;
    bitrate @2 :UInt64;
    viewerCount @3 :UInt64;
    streamHealth @4 :StreamHealth;
}

enum StreamHealth {
    excellent @0;
    good @1;
    fair @2;
    poor @3;
    offline @4;
    custom @5;
}

# ── Media Analytics ────────────────────────────────────────────────────
# Playback analytics for listening habits, skip rates, and completion.
# Enables content recommendation and user behavior analysis.
struct MediaAnalytics {
    mediaId @0 :T.MediaItemId;
    playCount @1 :UInt64;
    totalListenTime @2 :T.Duration;
    skipRate @3 :Float32;
    completionRate @4 :Float32;
    lastPlayed @5 :T.Timestamp;
    averageRating @6 :Float32;
}

# ════════════════════════════════════════════════════════════════════════
# Audit-9 additions: recommendation scoring, sharing preview,
# download status, crossfade configuration, live stream interaction,
# and transcoding progress.
# ════════════════════════════════════════════════════════════════════════

# ── Recommendation Score ───────────────────────────────────────────────
# Detailed recommendation with scoring and reasoning. Answers
# "why was this item recommended?" for transparency.
struct RecommendationScore {
    mediaId @0 :T.MediaItemId;
    # Recommendation score (0.0–1.0, higher = more relevant).
    score @1 :Float32;
    # Reason for recommendation (e.g., "similar to X", "trending").
    reason @2 :Text;
    # Source algorithm (e.g., "collaborative", "content-based", "popular").
    algorithm @3 :Text;
    # Confidence in this recommendation (0.0–1.0).
    confidence @4 :Float32;
}

# ── Share Preview ──────────────────────────────────────────────────────
# Preview metadata for a shared media link. Provides thumbnail,
# title, and description for link preview rendering.
struct SharePreview {
    # Share URL.
    url @0 :T.Uri;
    # Preview title.
    title @1 :Text;
    # Preview description.
    description @2 :Text;
    # Thumbnail image data.
    thumbnail @3 :T.DataHandle;
    # MIME type of the thumbnail.
    thumbnailMime @4 :Text;
    # Duration of the media (for audio/video).
    duration @5 :T.Duration;
}

# ── Download Status ────────────────────────────────────────────────────
# Status tracking for media downloads. Enables monitoring download
# progress, pausing, and resuming.
struct DownloadStatus {
    downloadId @0 :UInt64;
    mediaId @1 :T.MediaItemId;
    # Download progress (0.0–1.0).
    progress @2 :Float32;
    # Bytes downloaded so far.
    bytesDownloaded @3 :UInt64;
    # Total bytes to download.
    totalBytes @4 :UInt64;
    # Current state of the download.
    state @5 :DownloadState;
    # Download speed in bytes per second.
    speed @6 :UInt64;
    # Estimated time remaining.
    estimatedRemaining @7 :T.Duration;
}

enum DownloadState {
    pending @0;
    downloading @1;
    paused @2;
    completed @3;
    failed @4;
    cancelled @5;
    custom @6;
}

# ── Crossfade Configuration ────────────────────────────────────────────
# Detailed crossfade control. Extends the basic crossfade duration
# with curve/easing and gapless options.
struct CrossfadeConfig {
    # Crossfade duration.
    duration @0 :T.Duration;
    # Crossfade curve (linear, exponential, equal-power).
    curve @1 :CrossfadeCurve;
    # Whether to enable gapless playback when crossfade is off.
    gapless @2 :Bool;
    # Volume adjustment during crossfade (0.0–1.0).
    volumeAdjust @3 :Float32;
}

enum CrossfadeCurve {
    linear @0;
    equalPower @1;
    exponential @2;
    sCurve @3;
    custom @4;
}

# ── Live Stream Interaction ────────────────────────────────────────────
# Interactive features for live streams. Supports chat messages,
# polls, and reactions for viewer engagement.
struct LiveInteraction {
    streamUrl @0 :T.Uri;
    type @1 :LiveInteractionType;
    # Message/poll content.
    content @2 :Text;
    # Sender identity.
    senderId @3 :T.AppId;
    timestamp @4 :T.Timestamp;
}

enum LiveInteractionType {
    chatMessage @0;
    poll @1;
    pollResponse @2;
    reaction @3;
    tip @4;
    custom @5;
}

# ── Transcode Progress ─────────────────────────────────────────────────
# Progress tracking for media transcoding operations. Enables
# monitoring transcoding progress and receiving completion events.
struct TranscodeProgress {
    transcodeId @0 :UInt64;
    mediaId @1 :T.MediaItemId;
    # Transcoding progress (0.0–1.0).
    progress @2 :Float32;
    # Current state.
    state @3 :TranscodeState;
    # Output format being transcoded to.
    outputFormat @4 :Text;
    # Estimated time remaining.
    estimatedRemaining @5 :T.Duration;
}

enum TranscodeState {
    queued @0;
    processing @1;
    completed @2;
    failed @3;
    cancelled @4;
    custom @5;
}
===CAMERA-HEAD===
@0xa1b2c3d4e5f60026;

using T = import "types.capnp";
# The frame-delivery envelope (Frame, FramePullResult, FrameStream) has
# its canonical home in capture.capnp — one contract serves a camera
# stream and a screen-capture stream, so there is one spelling of what
# a frame IS rather than two that drift (R6).
using C = import "capture.capnp";

# ═══════════════════════════════════════════════════════════════════════
# Camera - photo/video capture, preview, zoom, flash, focus, HDR,
#         depth sensing, face/scene detection, privacy, calibration.
#
# Improvements beyond the original schema:
#   - CameraId, RecordingId, CaptureId, EventId typed IDs.
#   - T.Pagination on listCameras.
#   - Subscription cancel for clean teardown.
#   - T.Duration for recording duration.
#   - T.DataHandle for captured data.
#   - Custom variants on all enums.
#
# Audit-8 enhancements:
#   - CameraFeature: faceTracking, eyeTracking, depth, thermal,
#     sceneDetection, privacyMask, multiCamera.
#   - DepthFormat: depth16, depth32f, disparity16.
#   - PrivacyMask: blur/redact regions before capture.
#   - WatermarkRequest: programmatic watermarking.
#   - HdrOptions: bracket capture with merge strategy.
#   - Ca
```

### TOOL: shell [seq 198] 2026-11-09 13:48:08 status=completed

```json
{
  "command": "echo ===FILES-PICKER-FULL===\ncat /tmp/opencode/canonical-photos-audit/zip-desktop/smart-desktop/apps/files/picker.rs\necho ===SHELL-APPID===\ngrep -rn \"Photos\\|photos\" /home/lufuno/crates/smart-ui-egui/src/shell.rs | head -n 40\necho ===APP-ACT-REFS===\ngrep -n \"photos\\|WallpaperFrom\\|OpenInPhotos\\|library\\|Library\" /home/lufuno/crates/smart-ui-egui/src/app.rs | head -n 80"
}
```

```
===FILES-PICKER-FULL===
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
        let rows =
```

### TOOL: shell [seq 205] 2026-11-09 13:48:11 status=completed

```json
{
  "command": "echo ===SHELL-PHOTOS-WINDOW===\nsed -n '1,220p' /home/lufuno/crates/smart-ui-egui/src/shell.rs\necho ===DEVICES-MTP===\ngrep -n \"MTP\\|mtp\\|adb\\|ADB\\|phone\\|Phone\\|mount\\|Mount\\|gdisk\\|Storage\\|SD\\|usb\" /home/lufuno/crates/smart-core/src/devices.rs | head -n 80"
}
```

```
===SHELL-PHOTOS-WINDOW===
//! Desktop shell for the web showcase: system status bar, dock with spring magnification and launch bounce,
//! and a tiny window manager (z-order, focus-on-press, drag, resize, maximise) shared by the app suite
//! (Files, Photos, Terminal). Everything here is drawn with the same depth primitives as the file manager.
use crate::app::*;
use crate::chrome::*;
use crate::icons::Ic;
use crate::overlays::{mi, MI};
use crate::paint::*;
use crate::theme::*;
use egui::{pos2, vec2, Align2, Color32, Id, Pos2, Rect, Sense, Ui};
use smart_core::*;
use std::collections::HashMap;

pub const TOPBAR_H: f32 = 30.0;
pub const DOCK_H: f32 = 98.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppId {
    Files,
    Photos,
    Terminal,
    Video,
}

impl AppId {
    pub fn name(self) -> &'static str {
        match self {
            AppId::Files => "Files",
            AppId::Photos => "Photos",
            AppId::Terminal => "Terminal",
            AppId::Video => "Eiga",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            AppId::Files => "folder",
            AppId::Photos => "gallery",
            AppId::Terminal => "terminal",
            AppId::Video => "clapperboard",
        }
    }
}

#[derive(Clone)]
pub struct AppWin {
    pub open: bool,
    pub min: bool,
    pub rect: Rect,
    pub restore: Option<Rect>,
    pub born: f64,
    pub placed: bool,
    pub side: bool,
}

impl AppWin {
    fn new(w: f32, h: f32, open: bool) -> Self {
        Self { open, min: false, rect: Rect::from_min_size(pos2(0.0, 0.0), vec2(w, h)), restore: None, born: 0.0, placed: false, side: true }
    }
}

pub struct Shell {
    pub z: Vec<AppId>,
    pub photos_w: AppWin,
    pub term_w: AppWin,
    pub video_w: AppWin,
    pub eiga: crate::video::Eiga,
    pub term: crate::terminal::Terminal,
    pub photos: crate::photos::Photos,
    pub menu: Option<usize>,
    pub settings: Option<(AppId, usize)>,
    pub appmenu: Option<crate::appkit::AppMenu>,
    pub qs: bool,
    pub wifi: bool,
    pub bt: bool,
    pub focus: bool,
    pub brightness: f32,
    pub volume: f32,
    pub launched: HashMap<AppId, f64>,
}

impl Shell {
    pub fn new(home: Location) -> Self {
        Self {
            z: vec![AppId::Video, AppId::Terminal, AppId::Photos, AppId::Files],
            photos_w: AppWin::new(980.0, 640.0, false),
            term_w: AppWin::new(880.0, 540.0, false),
            video_w: { let mut w = AppWin::new(1040.0, 640.0, false); w.side = false; w },
            eiga: crate::video::Eiga::default(),
            term: crate::terminal::Terminal::new(home),
            photos: crate::photos::Photos::default(),
            menu: None,
            settings: None,
            appmenu: None,
            qs: false,
            wifi: true,
            bt: true,
            focus: false,
            brightness: 1.0,
            volume: 0.62,
            launched: HashMap::new(),
        }
    }
    pub fn active(&self) -> AppId {
        *self.z.last().unwrap_or(&AppId::Files)
    }
}

impl SmartApp {
    pub fn open_app(&mut self, id: AppId) {
        let t = self.time;
        match id {
            AppId::Files => {
                self.win_closed = false;
                self.win_min = false;
            }
            AppId::Photos => {
                if !self.shell.photos_w.open {
                    self.shell.photos_w.born = t;
                }
                self.shell.photos_w.open = true;
                self.shell.photos_w.min = false;
            }
            AppId::Terminal => {
                if !self.shell.term_w.open {
                    self.shell.term_w.born = t;
                }
                self.shell.term_w.open = true;
                self.shell.term_w.min = false;
            }
            AppId::Video => {
                if !self.shell.video_w.open {
                    self.shell.video_w.born = t;
                }
                self.shell.video_w.open = true;
                self.shell.video_w.min = false;
            }
        }
        self.shell.launched.insert(id, t);
        self.raise(id);
    }

    /// Topmost window that is actually on screen.
    pub fn focused_app(&self) -> AppId {
        for id in self.shell.z.iter().rev() {
            let vis = match id {
                AppId::Files => !(self.win_closed || self.win_min),
                other => self.app_win(*other).map(|w| w.open && !w.min).unwrap_or(false),
            };
            if vis {
                return *id;
            }
        }
        AppId::Files
    }

    /// Window the palette should centre itself in.
    pub fn palette_win(&self) -> Rect {
        match self.palette.app {
            Some(a) if a != AppId::Files => self.app_win(a).map(|w| w.rect).unwrap_or_else(crate::overlays::win_rect),
            _ => crate::overlays::win_rect(),
        }
    }

    pub fn raise(&mut self, id: AppId) {
        self.shell.z.retain(|x| *x != id);
        self.shell.z.push(id);
    }

    pub fn app_cmd(&mut self, cmd: &str) {
        if cmd == "settings" {
            let a = self.focused_app();
            if a != AppId::Files {
                self.shell.settings = Some((a, 0));
            }
            return;
        }
        if let Some(c) = cmd.strip_prefix("storage.") {
            self.storage_cmd(c);
        } else if let Some(c) = cmd.strip_prefix("eiga.") {
            self.eiga_cmd(c);
        } else if let Some(c) = cmd.strip_prefix("term.") {
            self.term_cmd(c);
        } else if let Some(c) = cmd.strip_prefix("photos.") {
            self.photos_cmd(c);
        }
    }

    pub fn app_win(&self, id: AppId) -> Option<&AppWin> {
        match id {
            AppId::Files => None,
            AppId::Photos => Some(&self.shell.photos_w),
            AppId::Terminal => Some(&self.shell.term_w),
            AppId::Video => Some(&self.shell.video_w),
        }
    }
    fn app_win_mut(&mut self, id: AppId) -> &mut AppWin {
        match id {
            AppId::Terminal => &mut self.shell.term_w,
            AppId::Video => &mut self.shell.video_w,
            _ => &mut self.shell.photos_w,
        }
    }

    // ───────────── entry point ─────────────
    pub fn draw_window(&mut self, ui: &mut Ui) {
        if !IS_WEB {
            clear_drop_targets();
            let a = self.focused_app();
            if a == AppId::Files {
                return self.draw_files_window(ui);
            }
            // desktop: the suite apps fill the real OS window (there is no fake desktop around them)
            let screen = ui.max_rect();
            let win = self.window_rect(screen);
            {
                let w = self.app_win_mut(a);
                w.rect = win;
                w.placed = true;
                w.restore = None;
            }
===DEVICES-MTP===
1://! Devices & locations: internal disks, USB sticks, phones, network and cloud. Mount/unmount are real.
12:    Phone,
22:            DeviceKind::Phone => "smartphone",
32:            DeviceKind::Phone => "Continuity",
44:    pub mounted: bool,
49:    pub can_unmount: bool,
69:pub struct Mounted {
71:    /// Provider to add to the registry (phones, cloud); local mounts reuse the `file` provider.
77:    fn mount(&self, id: &str) -> Result<Mounted, String>;
78:    fn unmount(&self, id: &str) -> Result<(), String>;
80:        self.unmount(id)
131:                let mp = d.mount_point().to_string_lossy().to_string();
161:                    mounted: true,
162:                    loc: Some(path_to_loc(d.mount_point())),
166:                    can_unmount: !is_root,
171:            // block devices that are present but not mounted (USB sticks, SD cards…)
174:                let mounted = !s("mountpoint").is_empty();
178:                if mounted || !removable || fs.is_empty() || s("type") == "disk" && !s("fstype").is_empty() && false {
192:                    mounted: false,
197:                    can_unmount: false,

```

### TOOL: shell [seq 212] 2026-11-09 13:48:14 status=completed

```json
{
  "command": "sed -n '380,540p' /home/lufuno/crates/smart-core/src/devices.rs\necho ===LOCAL-VFS===\ngrep -n \"pub struct\\|pub fn\\|is_writable\\|supports_trash\\|real_path\\|fn list\\|fn read\" /home/lufuno/crates/smart-core/src/local.rs | head -n 30\necho ===BACKEND-KEY===\ngrep -n \"pub struct Backend\\|pub fn \\|pub places\\|read_bytes\\|create_file\\|stat\\|trash\\|Trash\\|registry\\|library\" /home/lufuno/crates/smart-core/src/backend.rs | head -n 100"
}
```

```
                let serial = id.trim_start_matches("adb:");
                Ok(Mounted { loc: Location::new(id, "/sdcard"), vfs: Some(Arc::new(AdbVfs::new(serial))) })
            }
            fn unmount(&self, _id: &str) -> Result<(), String> {
                Err("Disconnect the phone to unmount".into())
            }
            fn owns(&self, id: &str) -> bool {
                id.starts_with("adb:")
            }
        }
    }
}
===LOCAL-VFS===
7:pub struct LocalVfs;
23:pub fn path_to_loc(p: &Path) -> Location {
41:    fn list(&self, path: &str) -> FsResult<Vec<Entry>> {
83:    fn real_path(&self, path: &str) -> Option<PathBuf> {
86:    fn supports_trash(&self) -> bool {
===BACKEND-KEY===
7:use crate::library::{Library, SharedLibrary};
10:use crate::registry::Registry;
23:    pub icon: &'static str,
36:    DeviceOp { id: String, op: &'static str, result: Result<Option<Location>, String> },
49:pub struct Backend {
50:    pub registry: Arc<Registry>,
51:    pub library: SharedLibrary,
72:    fn assemble(registry: Arc<Registry>, library: Library, apps: Arc<dyn AppProvider>, providers: Vec<Arc<dyn DeviceProvider>>, demo: bool) -> Self {
73:        let library = library.shared();
78:            search: SearchEngine::new(registry.clone(), library.clone()),
79:            thumbs: Thumbs::new(registry.clone()),
80:            registry,
81:            library,
102:    pub fn new_native() -> Self {
117:                            b.registry.insert(v);
128:    pub fn new_demo(library: Library) -> Self {
130:        Self::assemble(reg, library, Arc::new(crate::apps::DemoApps), vec![dev], true)
138:    pub fn home(&self) -> Location {
150:    pub fn places(&self) -> Vec<Place> {
151:        let mk = |id: &str, title: &str, icon: &'static str, loc: Location| Place { id: id.into(), title: title.into(), icon, loc };
156:            let add = |v: &mut Vec<Place>, id: &str, t: &str, i: &'static str, p: Option<std::path::PathBuf>| {
176:    pub fn list(&self, loc: &Location) -> u64 {
177:        let (id, reg, tx, loc) = (self.id(), self.registry.clone(), self.tx.clone(), loc.clone());
188:    pub fn summarize(&self, loc: &Location) {
189:        let (reg, tx, loc) = (self.registry.clone(), self.tx.clone(), loc.clone());
198:    pub fn stat(&self, loc: &Location) -> Result<Entry, String> {
199:        self.registry.get(&loc.fs).ok_or("unavailable")?.stat(&loc.path).map_err(|e| e.to_string())
202:    pub fn capacity(&self, loc: &Location) -> Option<(u64, u64)> {
203:        self.registry.get(&loc.fs)?.capacity(&loc.path).map(|c| (c.total, c.free))
207:    pub fn watch(&mut self, loc: &Location) {
210:        let Some(path) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
223:    pub fn watch(&mut self, _loc: &Location) {}
226:    pub fn search(&mut self, raw: &str, scope: &Location, contents: bool) -> u64 {
236:    pub fn cancel_search(&mut self) {
241:    pub fn indexed_count(&self) -> usize {
247:        self.registry.get(&loc.fs).ok_or_else(|| "This location is no longer available".to_string())
250:    pub fn create_folder(&self, dir: &Location, name: &str) -> Result<Location, String> {
258:    pub fn create_file(&self, dir: &Location, name: &str) -> Result<Location, String> {
266:    pub fn rename(&self, loc: &Location, new_name: &str) -> Result<Location, String> {
280:        self.library.write().unwrap().rekey(loc, &to);
284:    pub fn read_bytes(&self, loc: &Location, limit: usize) -> Result<Vec<u8>, String> {
287:    pub fn read_text(&self, loc: &Location, limit: usize) -> Result<String, String> {
288:        let b = self.read_bytes(loc, limit)?;
294:    pub fn write_text(&self, loc: &Location, text: &str) -> Result<(), String> {
299:    pub fn submit(&self, kind: JobKind) -> u64 {
303:        let (reg, tx) = (self.registry.clone(), self.tx.clone());
312:    pub fn has_paced_sources(&self, kind: &JobKind) -> bool {
313:        kind.srcs().iter().take(8).any(|l| self.registry.get(&l.fs).and_then(|v| v.stat(&l.path).ok()).is_some_and(|e| e.paced))
317:    pub fn submit_paced(&self, kind: JobKind, secs: f32) -> u64 {
325:    pub fn archive_entries(&self, loc: &Location, password: Option<&str>) -> Result<Vec<crate::archive::ZipEntry>, String> {
326:        crate::archive::preview_entries(&self.registry, loc, password).map_err(|e| e.to_string())
330:    pub fn archive_extract(&self, src: &Location, dest: &Location, password: Option<String>) {
331:        let (reg, tx, src, dest, id) = (self.registry.clone(), self.tx.clone(), src.clone(), dest.clone(), self.id());
352:    pub fn archive_compress(&self, srcs: Vec<Location>, dest_zip: Location, password: Option<String>, level: u8) {
353:        let (reg, tx, id) = (self.registry.clone(), self.tx.clone(), self.id());
377:    pub fn trash_dir() -> Option<std::path::PathBuf> {
381:        let d = base.join("Trash");
385:    /// Files currently sitting in the user's Trash. On the web this is empty.
386:    pub fn trash_entries(&self) -> Vec<Entry> {
389:            let Some(dir) = Self::trash_dir() else { return vec![] };
409:    /// Permanently delete everything in the Trash. Returns how many items were removed.
410:    pub fn trash_empty(&self) -> Result<usize, String> {
413:            let Some(dir) = Self::trash_dir() else { return Ok(0) };
435:    /// Restore a trashed item to the path recorded in its .trashinfo file.
436:    pub fn trash_restore(&self, loc: &Location) -> Result<Location, String> {
439:            let dir = Self::trash_dir().ok_or("no Trash on this system")?;
441:            let info = dir.join("info").join(format!("{name}.trashinfo"));
479:    pub fn open_terminal(&self, loc: &Location) {
480:        let Some(p) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
500:    pub fn cancel_job(&self, id: u64) {
507:    pub fn request_thumb(&self, loc: &Location, edge: u32) {
510:    pub fn forget_thumb(&self, loc: &Location) {
515:    pub fn devices(&self) -> Vec<Device> {
518:    pub fn refresh_devices(&self) {
526:    fn device_op(&self, id: &str, op: &'static str) {
527:        let (providers, tx, reg, id) = (self.providers.clone(), self.tx.clone(), self.registry.clone(), id.to_string());
550:    pub fn mount(&self, id: &str) {
553:    pub fn unmount(&self, id: &str) {
556:    pub fn eject(&self, id: &str) {
561:    pub fn connect_server(&self, url: &str) {
568:                    Ok(o) if o.status.success() => (format!("Connected to {}", url), true),
580:    pub fn apps_for(&self, loc: &Location) -> Vec<AppInfo> {
583:    pub fn open(&self, loc: &Location) {
586:    pub fn open_with(&self, loc: &Location, app: &AppInfo) {
589:    pub fn set_default_app(&self, loc: &Location, app: &AppInfo) -> Result<(), String> {
593:        self.library.write().unwrap().touch_recent(loc);
594:        let (reg, tx, apps, loc) = (self.registry.clone(), self.tx.clone(), self.apps.clone(), loc.clone());
622:    pub fn reveal_in_system(&self, loc: &Location) {
623:        let Some(p) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
635:    pub fn poll(&mut self) -> Vec<Event> {
658:            match ops::run_sync(&self.registry, &kind) {
704:    pub fn busy(&self) -> bool {
708:    pub fn save_library(&self) {
711:            self.library.read().unwrap().save();

```

### TOOL: shell [seq 219] 2026-11-09 13:48:18 status=completed

```json
{
  "command": "echo ===CAPTURE-FULL===\ncat /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/capture/mod.rs\necho ===SETTINGS-HEAD===\nhead -n 120 /tmp/opencode/canonical-photos-audit/zip-desktop2/smart-desktop/apps/settings/mod.rs"
}
```

```
===CAPTURE-FULL===
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
            let sel = se
```

### REASONING [seq 226] 2026-11-09 13:48:21

Auditing photos module for drag-drop, caching, search, and protocol dependency gaps.

### REASONING [seq 226] 2026-11-09 13:48:21

Verifying EXIF rotation handling and non-destructive edit constraints in the image pipeline.

### TOOL: shell [seq 226] 2026-11-09 13:48:21 status=completed

```json
{
  "command": "echo ===EXIF-CROP-UNDO-DELETE===\ngrep -n -i \"exif\\|jpeg.*orient\\|crop\\|undo\\|redo\\|history\\|delete\\|trash\\|remove\\|heart\\|fav\" /home/lufuno/crates/smart-ui-egui/src/photos.rs | head -n 60\necho ===IMAGE-CRATE-FEATURES===\ncat /home/lufuno/crates/smart-ui-egui/Cargo.toml /home/lufuno/crates/smart-core/Cargo.toml /home/lufuno/Cargo.toml 2>&1 | head -n 120\necho ===DROPPED-FILES===\ngrep -rn \"DroppedFile\\|drop_files\\|drag.*drop\\|clear_drop\" /home/lufuno/crates/smart-ui-egui/src/*.rs | head -n 40\necho ===HEIC-RAW===\ngrep -rn \"heic\\|avif\\|raw\\|dng\\|cr2\\|nef\\|arw\" /home/lufuno/crates/smart-core/src/*.rs /home/lufuno/crates/smart-ui-egui/src/*.rs | head -n 30\necho ===A11Y===\ngrep -n \"accessib\\|screen_reader\\|aria\\|focus\\|keyboard\\|shortcut\\|Key::\" /home/lufuno/crates/smart-ui-egui/src/photos.rs | head -n 40"
}
```

```
===EXIF-CROP-UNDO-DELETE===
44:    Favorites,
66:    pub favs: HashSet<Location>,
112:            favs: HashSet::new(),
282:    fn photos_toggle_fav(&mut self, l: Location) {
283:        if !self.shell.photos.favs.remove(&l) {
284:            self.shell.photos.favs.insert(l);
340:                    "fav" => Album::Favorites,
344:            "fav" => {
346:                    self.photos_toggle_fav(l);
410:                Album::Favorites => ph.favs.contains(&e.loc),
555:        let fav = self.shell.photos.favs.contains(loc);
559:            mi("heart", if fav { "Remove from favorites" } else { "Add to favorites" }, a("fav")).k("F").q(),
571:        let sorts: Vec<MI> = SORTS.iter().enumerate().map(|(i, n)| mi(["clock-circle", "history", "text", "database"][i], *n, a(&format!("sort:{i}"))).on(ph.sort == i)).collect();
577:            mi("crop-minimalistic", "Fill thumbnails", a("fill")).on(ph.fill),
588:        let fav = ph.open.as_ref().is_some_and(|l| ph.favs.contains(l));
595:            mi("heart", if fav { "Remove from favorites" } else { "Add to favorites" }, a("fav")).k("F").q(),
673:        let frac = (self.shell.photos.favs.len() as f32 / self.shell.photos.all.len().max(1) as f32).clamp(0.0, 1.0).max(0.04);
675:        p.text(card.left_top() + vec2(14.0, 50.0), Align2::LEFT_CENTER, format!("{} favorites \u{b7} {} albums", self.shell.photos.favs.len(), folders.len()), f_medium(T_LABEL), th.ink3);
690:        let mut rows: Vec<(String, &'static str, usize, Album, String)> = vec![("All photos".into(), "gallery", self.shell.photos.all.len(), Album::All, "all".into()), ("Favorites".into(), "heart", self.shell.photos.favs.len(), Album::Favorites, "fav".into())];
804:                Album::Favorites => "Favorites".to_string(),
821:            let fav = ph.open.as_ref().is_some_and(|l| ph.favs.contains(l));
826:            let btns: [(&'static str, &str, bool, &str, &str); 4] = [("menu-dots", "ph-v-more", false, "More", "more"), ("tuning-2", "ph-v-edit", edit, "Edit  E", "edit"), ("info-circle", "ph-v-info", info, "Info  I", "info"), ("heart", "ph-v-fav", fav, "Favorite  F", "fav")];
829:                if compact && *cmd != "more" && *cmd != "fav" {
833:                let kind = if *cmd == "fav" || !*on { Btn::Raised } else { Btn::Active };
835:                if *cmd == "fav" && *on {
836:                    self.ico(&p, "heart", Ic::Filled, r.center(), 17.0, hex("#ff6b8a"));
862:            let fav = self.shell.photos.album == Album::Favorites;
863:            self.ico(&p, if fav { "heart" } else { "gallery" }, Ic::Duo, c0 - vec2(0.0, 26.0), 44.0, alpha(th.ink4, 0.7));
864:            p.text(c0 + vec2(0.0, 16.0), Align2::CENTER_CENTER, if fav { "No favorites yet" } else { "No photos here" }, f_semi(T_TITLE), th.ink2);
865:            p.text(c0 + vec2(0.0, 38.0), Align2::CENTER_CENTER, if fav { "Tap the heart on any photo to keep it close." } else { "Images from your folders appear here." }, f_medium(T_BODY), th.ink3);
888:        let mut fav_toggle: Option<Location> = None;
933:                let fav = self.shell.photos.favs.contains(&e.loc);
934:                if a > 0.05 || fav {
936:                    let hr = self.hit(ui, hb, ("ph-heart", &e.loc), Sense::click());
937:                    let pop = spring_to(ui.ctx(), ("ph-fav", &e.loc), if fav { 1.0 } else { 0.0 }, 380.0, 15.0);
939:                    self.ico(&p, "heart", if fav { Ic::Filled } else { Ic::Outline }, hb.center(), 16.0 * (1.0 + 0.35 * (pop - 1.0).max(0.0) + if hr.hovered() { 0.1 } else { 0.0 }), if fav { hex("#ff6b8a") } else { Color32::WHITE });
941:                        fav_toggle = Some(e.loc.clone());
949:                if resp.double_clicked() || (resp.clicked() && !matches!(fav_toggle, Some(ref l) if *l == e.loc)) {
954:        if let Some(l) = fav_toggle {
955:            self.photos_toggle_fav(l);
1018:                        "f" => "fav",
1413:                    SRow::Choice { icon: "crop-minimalistic", label: "Thumbnails", sub: "Fill crops to a tile, Fit shows the whole photo", opts: &["Fill", "Fit"], sel: if ph.fill { 0 } else { 1 } },
1425:                    SRow::Button { icon: "refresh", label: "Rescan library", sub: "Look for new and removed pictures", cta: "Rescan" },
1426:                    SRow::Button { icon: "heart", label: "Clear favorites", sub: "Remove the heart from every photo", cta: "Clear" },
1427:                    SRow::Button { icon: "trash-bin-minimalistic", label: "Reset thumbnails", sub: "Drop the thumbnail cache and rebuild", cta: "Reset" },
1448:                    ph.favs.clear();
1449:                    self.toast("Favorites cleared", true);
===IMAGE-CRATE-FEATURES===
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
[package]
name = "smart-core"
description = "Smart Files backend: virtual file systems, search, jobs, devices, apps — no UI code."
version.workspace = true
edition.workspace = true
license.workspace = true

[features]
default = ["demo"]
demo = []          # seeded in-memory file system used by the web build
audio = ["dep:rodio"]
cloud = []         # placeholder for opendal-backed providers (see src/cloud.rs)

[dependencies]
chrono = { version = "0.4", default-features = false, features = ["clock", "std", "wasmbind"] }
image = { version = "0.25", default-features = false, features = ["jpeg", "png", "gif", "webp", "bmp"] }
memchr = "2"
mime_guess = "2"
miniz_oxide = "0.9"
nucleo-matcher = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
dirs = "7"
notify = "8"
rayon = "1"
sysinfo = { version = "0.40", default-features = false, features = ["disk"] }
trash = "5"
rodio = { version = "0.23", default-features = false, features = ["playback", "symphonia-all"], optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
js-sys = "0.3"
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Blob", "BlobPropertyBag", "Url", "HtmlAudioElement", "HtmlMediaElement", "Window", "Storage"] }
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
===DROPPED-FILES===
/home/lufuno/crates/smart-ui-egui/src/chrome.rs:41:pub fn clear_drop_targets() {
/home/lufuno/crates/smart-ui-egui/src/chrome.rs:117
```

### TOOL: shell [seq 239] 2026-11-09 13:48:36 status=completed

```json
{
  "command": "echo ===SEARCH-ENGINE===\nhead -n 120 /home/lufuno/crates/smart-core/src/search.rs\necho ===OPS-TRASH===\ngrep -n \"trash\\|JobKind\\|remove\\|delete\" /home/lufuno/crates/smart-core/src/ops.rs | head -n 60\necho ===LOCAL-TRASH===\ncat /home/lufuno/crates/smart-core/src/local.rs\necho ===FILES-CONTENT-DELETE===\ngrep -n \"trash\\|Trash\\|confirm_delete\\|destructive\\|delete\" /home/lufuno/crates/smart-ui-egui/src/content.rs /home/lufuno/crates/smart-ui-egui/src/chrome.rs /home/lufuno/crates/smart-ui-egui/src/modals.rs | head -n 60"
}
```

```
===SEARCH-ENGINE===
//! Search: query language + cached name index + fuzzy ranking (nucleo) + optional content scan.
//!
//! * `report kind:pdf size:>1mb modified:week tag:design in:name`  — terms are fuzzy, filters are exact.
//! * First search under a root walks the tree while streaming matches, then caches the index;
//!   later searches rank the cached index in parallel (rayon) and answer in milliseconds.
use crate::exec::{now_millis, spawn};
use crate::library::SharedLibrary;
use crate::model::*;
use crate::ops::Registry;
use crate::vfs::Vfs;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filters {
    pub kinds: Vec<Kind>,
    pub exts: Vec<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub modified_after: Option<i64>,
    pub modified_before: Option<i64>,
    pub tags: Vec<String>,
    pub dirs: Option<bool>,
    pub content: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query {
    pub raw: String,
    pub terms: Vec<String>,
    pub filters: Filters,
}

fn parse_size(s: &str) -> Option<u64> {
    let s = s.to_lowercase();
    let idx = s.find(|c: char| !(c.is_ascii_digit() || c == '.')).unwrap_or(s.len());
    let n: f64 = s[..idx].parse().ok()?;
    let mul = match s[idx..].trim() {
        "" | "b" => 1.0,
        "k" | "kb" => 1e3,
        "m" | "mb" => 1e6,
        "g" | "gb" => 1e9,
        "t" | "tb" => 1e12,
        _ => return None,
    };
    Some((n * mul) as u64)
}

fn midnight(days_ago: i64) -> i64 {
    use chrono::{Duration, Local, TimeZone};
    let d = Local::now().date_naive() - Duration::days(days_ago);
    Local.from_local_datetime(&d.and_hms_opt(0, 0, 0).unwrap()).single().map(|t| t.timestamp()).unwrap_or(0)
}

impl Query {
    pub fn parse(raw: &str) -> Query {
        let mut q = Query { raw: raw.to_string(), ..Default::default() };
        let mut chars = raw.trim().chars().peekable();
        let mut toks: Vec<String> = vec![];
        let mut cur = String::new();
        let mut quoted = false;
        while let Some(c) = chars.next() {
            match c {
                '"' => quoted = !quoted,
                c if c.is_whitespace() && !quoted => {
                    if !cur.is_empty() {
                        toks.push(std::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            }
        }
        if !cur.is_empty() {
            toks.push(cur);
        }
        for t in toks {
            let Some((k, v)) = t.split_once(':').filter(|(k, v)| !k.is_empty() && !v.is_empty() && k.chars().all(|c| c.is_ascii_alphabetic())) else {
                q.terms.push(t);
                continue;
            };
            let f = &mut q.filters;
            match k.to_lowercase().as_str() {
                "kind" | "type" | "is" => match v.to_lowercase().as_str() {
                    "image" | "images" | "photo" | "picture" => f.kinds.extend([Kind::Image, Kind::Vector]),
                    "audio" | "music" | "sound" => f.kinds.push(Kind::Audio),
                    "video" | "movie" | "movies" => f.kinds.push(Kind::Video),
                    "pdf" => f.kinds.push(Kind::Pdf),
                    "doc" | "docs" | "document" => f.kinds.extend([Kind::Document, Kind::Pdf, Kind::Text]),
                    "text" => f.kinds.push(Kind::Text),
                    "code" | "source" => f.kinds.push(Kind::Code),
                    "archive" | "zip" => f.kinds.push(Kind::Archive),
                    "app" => f.kinds.push(Kind::App),
                    "folder" | "dir" | "directory" => f.dirs = Some(true),
                    "file" => f.dirs = Some(false),
                    _ => q.terms.push(t.clone()),
                },
                "ext" => f.exts.push(v.trim_start_matches('.').to_lowercase()),
                "tag" => f.tags.push(v.to_lowercase()),
                "content" | "text-in" | "contains" => f.content = Some(v.to_string()),
                "size" => {
                    if let Some(r) = v.strip_prefix('>') {
                        f.min_size = parse_size(r);
                    } else if let Some(r) = v.strip_prefix('<') {
                        f.max_size = parse_size(r);
                    } else {
                        f.min_size = parse_size(v);
                    }
                }
                "modified" | "date" | "changed" => match v.to_lowercase().as_str() {
                    "today" => f.modified_after = Some(midnight(0)),
                    "yesterday" => {
                        f.modified_after = Some(midnight(1));
                        f.modified_before = Some(midnight(0));
                    }
                    "week" => f.modified_after = Some(midnight(7)),
                    "month" => f.modified_after = Some(midnight(30)),
                    "year" => f.modified_after = Some(midnight(365)),
===OPS-TRASH===
1://! Copy / move / delete jobs that work across any pair of providers, with progress + cancel.
10:pub enum JobKind {
19:impl JobKind {
22:            JobKind::Copy { srcs, .. } | JobKind::Move { srcs, .. } | JobKind::Trash { srcs } | JobKind::Delete { srcs } | JobKind::Duplicate { srcs } => srcs.clone(),
28:            JobKind::Copy { srcs, dest } => format!("Copying {} to {}", n(srcs), dest.name()),
29:            JobKind::Move { srcs, dest } => format!("Moving {} to {}", n(srcs), dest.name()),
30:            JobKind::Trash { srcs } => format!("Moving {} to Trash", n(srcs)),
31:            JobKind::Delete { srcs } => format!("Deleting {}", n(srcs)),
32:            JobKind::Duplicate { srcs } => format!("Duplicating {}", n(srcs)),
131:            let _ = dv.remove(&dst.path, false);
154:pub fn run_sync(r: &Registry, job: &JobKind) -> FsResult<(Vec<Location>, Vec<Location>)> {
160:pub fn run(r: &Registry, job: JobKind, ctx: &JobCtx) {
169:fn run_inner(r: &Registry, job: &JobKind, ctx: &JobCtx) -> FsResult<(Vec<Location>, Vec<Location>)> {
173:        JobKind::Copy { srcs, dest } | JobKind::Move { srcs, dest } => {
174:            let moving = matches!(job, JobKind::Move { .. });
197:                        Err(_) => {} // cross-device: fall through to copy + delete
203:                    vfs(r, s)?.remove(&s.path, false)?;
208:        JobKind::Duplicate { srcs } => {
221:        JobKind::Trash { srcs } | JobKind::Delete { srcs } => {
222:            let trash = matches!(job, JobKind::Trash { .. });
228:                v.remove(&s.path, trash && v.supports_trash())?;
===LOCAL-TRASH===
//! The real disk. Native only.
use crate::model::{Entry, Location};
use crate::vfs::*;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct LocalVfs;

fn unix(t: std::io::Result<std::time::SystemTime>) -> Option<i64> {
    t.ok()?.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs() as i64)
}

fn to_pathbuf(p: &str) -> PathBuf {
    #[cfg(windows)]
    {
        let t = p.trim_start_matches('/');
        return PathBuf::from(if t.len() == 2 { format!("{}/", t) } else { t.to_string() });
    }
    #[cfg(not(windows))]
    PathBuf::from(p)
}

pub fn path_to_loc(p: &Path) -> Location {
    let s = p.to_string_lossy().replace('\\', "/");
    #[cfg(windows)]
    let s = format!("/{}", s);
    Location::file(s)
}

fn entry_from(path: &Path, md: &std::fs::Metadata, symlink: bool) -> Entry {
    let mut e = Entry::new(path_to_loc(path), md.is_dir(), if md.is_dir() { 0 } else { md.len() }, unix(md.modified()));
    e.symlink = symlink;
    e.readonly = md.permissions().readonly();
    e
}

impl Vfs for LocalVfs {
    fn id(&self) -> &str {
        "file"
    }
    fn list(&self, path: &str) -> FsResult<Vec<Entry>> {
        let rd = std::fs::read_dir(to_pathbuf(path))?;
        let mut out = Vec::new();
        for ent in rd.fla
```

### TOOL: shell [seq 246] 2026-11-09 13:48:40 status=completed

```json
{
  "command": "ls -R /home/lufuno/smart/naledzi_ui/examples/smart-photos/ 2>&1 | head -n 40\nls -R /home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/ 2>&1 | head -n 40\necho ===NALEDZI-PHOTOS-EXAMPLE===\nfind /home/lufuno/smart/naledzi_ui/examples/smart-photos -type f | head\necho ===KILO-PLAN===\nls -lh /home/lufuno/.local/share/kilo/plans/1790953699107-gpui-photos-app-plan.md 2>&1; wc -l /home/lufuno/.local/share/kilo/plans/1790953699107-gpui-photos-app-plan.md 2>&1"
}
```

```
/home/lufuno/smart/naledzi_ui/examples/smart-photos/:
Cargo.toml
SPEC.md
src
target
tests

/home/lufuno/smart/naledzi_ui/examples/smart-photos/src:
bar.rs
frame.rs
index.rs
lib.rs
main.rs
media.rs
model
model.rs
msg.rs
places.rs
route.rs
scan.rs
search.rs
settings.rs
state.rs
thumbs.rs
undo.rs
view
view.rs
xmp.rs

/home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model:
cells.rs
grid.rs
viewer.rs

/home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view:
dialog.rs
grid_cell.rs
info.rs
settings.rs
sheet.rs
/home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/:
Cargo.toml
examples
src

/home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/examples:
probe.rs

/home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/src:
lib.rs
library.rs
main.rs
model.rs
probe.rs
view.rs
===NALEDZI-PHOTOS-EXAMPLE===
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/selection.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/shell.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/observe.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/perf.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/xmp.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/geometry.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/tests/index.rs
/home/lufuno/smart/naledzi_ui/examples/smart-photos/target/shots/light-untagged-360@1.png
/home/lufuno/smart/naledzi_ui/examples/smart-photos/target/shots/light-timeline-selected-1200@2.png
/home/lufuno/smart/naledzi_ui/examples/smart-photos/target/shots/light-timeline-360@1.png
===KILO-PLAN===
-rw-r--r-- 1 lufuno lufuno 11K Oct  2 17:15 /home/lufuno/.local/share/kilo/plans/1790953699107-gpui-photos-app-plan.md
212 /home/lufuno/.local/share/kilo/plans/1790953699107-gpui-photos-app-plan.md

```

### TOOL: shell [seq 255] 2026-11-09 13:48:45 status=completed

```json
{
  "command": "wc -l -c /home/lufuno/smart/naledzi_ui/examples/smart-photos/SPEC.md /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/*.rs /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/*.rs /home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/src/*.rs 2>&1 | head -n 40\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/SPEC.md"
}
```

```
  └── cells.rs               cell rects, hit testing, marquee bands     §9.3
    ├── state.rs                   State, the update reducer, undo
    ├── scan.rs                    enumeration, classification               §11.2
    ├── index.rs                   the index record, load / validate / save  §11.6
    ├── thumbs.rs                  cache keys, the priority queue, workers   §12
    ├── media.rs                   decode dispatch, EXIF, dimension probe    §12.2
    ├── xmp.rs                     the sidecar reader and writer             §13.2
    ├── search.rs                  the query surface, ranking                §9.6
    ├── places.rs                  GPS clustering, the offline gazetteer     §5.2.5
    ├── undo.rs                    the operation log                         §16.3
    ├── settings.rs                config, persistence                       §3.7
    └── views/
        ├── timeline.rs            the landing view                          §6
        ├── grid_cell.rs           one cell + its overlays                  §9.4
        ├── viewer.rs              §7
        ├── sidebar.rs             §5
        ├── info.rs                §7.10
        ├── sheet.rs               the shared sheet primitive               §13.4
        ├── dialog.rs              §16.4
        └── settings.rs            §3.7
tests/
    ├── geometry.rs                pure model, no display, no filesystem    §18.2
    ├── index.rs                   scan -> index -> reload identity         §18.3
    ├── xmp.rs                     sidecar round-trip fidelity              §18.4
    ├── selection.rs               every gesture, without a window          §18.5
    ├── keyboard.rs                the table, and what each key does        §18.5
    └── perf.rs                    the budgets in §12.8                     §18.6
```

**The dependency arrows point one way:** `model` depends on nothing but iced's types. `state`
depends on `model`. `views` depend on `state` and `model`. `bar` depends on `model` and
`smart_ui_core`. Nothing in `model` imports `views`, and nothing computes a decision that
`model` already computes. A test asserts the module graph, because a cycle here is a spec
violation that happens to compile.


**The dependency arrows point one way:** `model` depends on nothing. `state` depends on
`model`. `views` depend on `state` and `model`. `bar` depends on `model` and `smart_ui_core`.
Nothing in `model` imports `iced` beyond its types, and nothing in `model` imports anything
from `views`. A test asserts the module graph, because a cycle here is a spec violation that
happens to compile.

### 17.3 The `Msg` enum

```rust
pub enum Msg {
    // the app bar, adapted (§1.1)
    ChromeHover(Option<&'static str>), Command(Command),
    Overflow(OverflowTarget), SearchToggle, CollectionFilter(usize), Ignored(String),
    // navigation
    Route(Route), Back, Forward, Up, TrailPush, TrailPop,
    ToggleSidebar(Option<bool>), SetSidebarWidth(f32),
    // the grid
    CellPress(PhotoId, Modifiers), CellOpen(PhotoId), CellContext(PhotoId, Pixel),
    Marquee(Pixel), MarqueeEnd, MarqueeCancel(Merge),
    FocusMove(Direction, Modifiers), TypeAhead(char), TypeAheadFlush,
    Scroll(f32), ScrollIdle,
    // selection
    SelectAll(Scope), Invert, ClearSelection,
    // the viewer
    OpenViewer(PhotoId), CloseViewer, ZoomTo(ZoomState), ZoomBy(f32, Option<Pixel>),
    Pan(Pixel), PanEnd, Rotate(i8, Commit), ToggleInfo, ToggleFilmstrip(Option<bool>),
    Slideshow(SlideshowCmd), Compare(PhotoId, PhotoId),
    // data
    Scan(ScanEvent), IndexReady(Result<IndexSnapshot, String>),
    Thumb(ThumbEvent), TextureUploaded(PhotoId, u32), Watch(WatchEvent),
    MissingReassociate(PhotoId, PathBuf),
    // organisation
    Tag(Edit), Rate(Vec<PhotoId>, u8), Favourite(Vec<PhotoId>, bool), AlbumCmd(AlbumCmd),
    MoveTo(PathBuf, MoveMode), TrashCmd(TrashCmd), Restore(Vec<PhotoId>),
    // edit & export
    Edit(EditCmd), Export(ExportSpec), CaptureFrame(PhotoId),
    // chrome
    SetSheet(Sheet), CloseSheet, Toast(Toast), Undo, Redo,
    SettingChanged(Setting), Shortcut(Shortcut), WindowPhase(WindowPhase),
    Clipboard(ClipboardPaste),
}
```

`Command` is a `#[non_exhaustive]` enum of the Appendix A ids, and it is the **only** place a
bar command and a context-menu command are named. The bar's `Press(id)` and the menu's row
both construct a `Command`, so there is exactly one implementation of *Rotate Right* in the
app, and `A7` is a test rather than a review.


### 17.4 Why the index is not capnp, and why that could change

The workspace rule is *use capnp zero cost*. The index is not capnp because it is a **private
cache in `$XDG_CACHE_HOME`** that the user is entitled to delete at any moment, is read whole
at startup, and is never exchanged with another process. Capnp's zero-copy advantage is
real when two processes share a mapped message; it is irrelevant when a single process
`read`s 40 MB once and pays nothing. Schema'd, versioned, and a **magic number plus a length**
on the file.

If the ecosystem ever needs a second process to read the index — a `smart-photos-indexd`
daemon, a flatpak-spawned helper, a shared index across two windows — then it becomes a capnp
schema, because at that point two processes *do* map the same message and the zero-copy
argument is real. Recorded here so the decision is a known ratchet rather than an oversight.

### 17.5 Crate dependencies

```toml
[package]
name = "smart-photos"
version = "1.0.0"
edition = "2024"
rust-version = "1.85"
description = "Smart Photos — a daily-driver photo manager for the folders you already keep."
license = "MIT"

[lib]
name = "smart_photos"
path = "src/lib.rs"

[[bin]]
name = "smart-photos"
path = "src/main.rs"

[dependencies]
smart-ui-core = { path = "../../smart-ui-core" }
iced = { version = "=0.14.0", features = ["tiny-skia", "advanced", "smol"] }

# decode
image = { version = "0.25", default-features = false, features = ["jpeg", "png", "tiff", "bmp", "ico"] }
zune-jpeg = { version = "0.5", features = ["simd"] }
zune-png = "0.2"
zune-core = "0.5"
zune-inflate = "0.2"
image-webp = "0.2"
qoi = "0.4"
gif = "0.14"
jxl-oxide = "0.12"
resvg = "0.47"
fast_image_resize = { version = "6.1", features = ["rayon", "image"] }

# metadata, identity
kamadak-exif = "0.6"
blake3 = "1.8"

# concurrency — exactly one pool, §12.5
rayon = "1.12"
crossbeam-channel = "0.5"

# filesystem
notify = "8.2"
notify-debouncer-full = "0.7"
walkdir = "2.5"
trash = "5.2"

# time, locations, persistence
chrono = { version = "0.4", features = ["serde"] }
time = { version = "0.3", features = ["formatting", "parsing", "serde"] }
xdg = "3.0"
xdg-user = "0.2"
dirs = "7.0"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

`unsafe_code = "forbid"`, and the same four `deny` lints as every other crate in the
workspace. `image` is `default-features = false` with an explicit feature list because its
defaults pull AVIF and OpenEXR, which cost build time and buy nothing here.

**No network client. No `reqwest`, no `ureq`, no `hyper`.** §0.2 is a dependency-level rule so
it cannot be violated by accident, and `tests/perf.rs::P6` asserts the crate is absent from
the resolved graph.

**And the list above is the whole list: `smart-ui-core` and `iced`.** No `crextio-ui`, no
`appbar-demo`, no second UI crate, and no façade of this app's own. **A build that still links
`crextio-ui` is not a port, it is a wrapper — and a wrapper is how this problem started**
(`smart-ui-core/SPEC.md` Appendix A, step 2).



### 17.6 What v2 needs to prove before it is admitted

| Candidate | Admission test |
|---|---|
| **RAW decoding** (`libraw` or `rawloader`) | decode a 100-photo RAW corpus in ≤ 3 s wall on 4 cores, with peak RSS ≤ 400 MB, and produce a thumbnail that is visibly equal to the embedded JPEG's. If `rawloader` cannot do it, `libraw` is a C++ build and the answer is a helper process. |
| **Lossy WebP** | `image-webp` is lossless-only today. Eithe
```

### TOOL: shell [seq 262] 2026-11-09 13:48:48 status=completed

```json
{
  "command": "cp /home/lufuno/smart/naledzi_ui/examples/smart-photos/SPEC.md /tmp/opencode/canonical-photos-audit/naledzi-SPEC.md\nwc -l -c /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/*.rs /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/*.rs /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/*.rs\necho ===SPEC-TOC===\ngrep -n \"^#\\{1,3\\} \" /home/lufuno/smart/naledzi_ui/examples/smart-photos/SPEC.md | head -n 120"
}
```

```
  1012  38175 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/bar.rs
   344  13395 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/frame.rs
   431  15650 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/index.rs
    35   1181 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/lib.rs
    75   3063 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/main.rs
   499  18944 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/media.rs
   413  14634 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model.rs
  1068  45570 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/msg.rs
   177   6252 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/places.rs
   109   3481 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/route.rs
   267   9614 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/scan.rs
   205   6480 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/search.rs
   323  12067 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/settings.rs
  2603 101985 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/state.rs
   268   9409 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/thumbs.rs
   208   7186 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/undo.rs
   484  18508 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view.rs
   613  24014 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/xmp.rs
   194   6632 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/cells.rs
   668  24221 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/grid.rs
   306  10352 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/viewer.rs
   102   3970 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/dialog.rs
   230   8856 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/grid_cell.rs
   264   8439 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/info.rs
    99   3505 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/settings.rs
   117   4681 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/sheet.rs
   261  10888 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/sidebar.rs
   172   6781 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/timeline.rs
   184   6920 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/view/viewer.rs
 11731 444853 total
===SPEC-TOC===
1:# SMART-PHOTOS — Specification
11:## 0. MANDATE
13:### 0.1 What this is
30:### 0.2 What this is not
46:### 0.3 Non-negotiable product laws
65:### 0.4 The one-sentence test
73:## 1. THE UI CORE, AND WHAT THIS APP OWNS
83:### 1.0 What this app takes, and what it keeps
107:### 1.1 The seam, and it is six messages wide
171:### 1.1a What the app does *not* inherit
192:### 1.2 What is replaced, and what is not
225:### 1.3 The law
241:## 2. ARCHITECTURE DECISIONS AND REJECTED ALTERNATIVES
246:### 2.1 No SQLite. A binary index cache instead.
269:### 2.2 Hand-written XMP. No `xmp_toolkit`.
288:### 2.3 RAW files are indexed, not decoded.
299:### 2.4 Thumbnail cache: two caches, BLAKE3 keys, mtime in the key.
312:### 2.5 Fast SIMD downscale, not `image::resize`.
323:### 2.6 Canonical sort key, one definition, three consumers.
337:### 2.7 Threading: one bounded pool, four priority tiers, no per-item threads.
344:### 2.8 Rendering: the grid is widgets, not a shader.
354:### 2.9 No per-app fork of the design system.
363:## 3. DESIGN TOKENS
368:### 3.1 Colour — the two planes
399:### 3.2 The accent law
417:### 3.3 Radius scale
441:### 3.4 Spacing — a 4-point ladder, named
456:### 3.5 Typography — nine roles, two scales, no overlap
497:### 3.6 Motion
512:### 3.7 Density and scale
526:## 4. THE HEADER — TWO LEVELS, AND WHERE EVERY COMMAND ACTUALLY LIVES
528:### 4.0 The rule that reorganised this section
557:### 4.1 Two levels
610:### 4.2 L1: the bar, and what is in it
647:### 4.3 TRAILING: a function of **view *and* state**, and at most three
815:### 4.4 Where every command lives — nothing is deleted
849:### 4.5 The palette, and why this section is survivable
860:### 4.6 Height, gutter, and the fold — unchanged, and still exactly right
884:### 4.7 Everything is a pill, and the bar's content is vertically centred
950:### 4.8 L1 and L2 float, on one gradient
969:### 4.9 Geometry, verified
995:### 4.10 Focus order, derived from what is drawn
1021:### 4.11 The bar's laws, as tests
1047:## 5. THE SIDEBAR
1049:### 5.1 Pane geometry comes from the framework, never from local numbers
1071:### 5.2 Six sections, in this order, always
1192:### 5.3 Row anatomy
1225:### 5.4 Collapsed rail
1239:### 5.5 The screenshot heuristic
1261:### 5.6 Duplicates
1275:### 5.7 Sidebar state, and what is remembered where
1294:## 6. THE TIMELINE
1299:### 6.1 Granularity ramp
1318:### 6.2 Section headers, and the sticky one
1347:### 6.3 Grid layouts — four, with exact arithmetic
1452:### 6.4 Moments
1474:### 6.5 Virtualisation, and the hard rule
1511:### 6.6 Keyboard navigation in the grid
1537:### 6.7 Type-ahead
1544:### 6.8 The scroll frame, itemised
1578:## 7. THE VIEWER
1584:### 7.1 Zoom: five named states and a continuous value
1603:### 7.2 Zoom-to-cursor, specified exactly
1620:### 7.3 Pan and its bounds
1630:### 7.4 Rotation, view-only by default
1638:### 7.5 Navigation, and the order law
1657:### 7.6 The viewer chrome
1677:### 7.7 Filmstrip
1701:### 7.8 Slideshow
1719:### 7.9 Compare (light table)
1743:### 7.10 The info panel
1788:### 7.11 Transparent images
1795:### 7.12 Video
1814:### 7.13 The viewer's own context menu
1823:## 8. THE CONTEXT MENU
1829:### 8.1 The rule
1838:### 8.2 The full command set
1859:### 8.3 Gestures, and what each one is
1881:### 8.4 Submenus
1895:### 8.5 Menu construction rules
1913:### 8.6 Availability, derived not declared
1935:## 9. SELECTION AND KEYBOARD
1937:### 9.1 The model
1954:### 9.2 Every way a selection changes
1979:### 9.3 The selection rectangle, precisely
2001:### 9.4 The cell, and its three cues
2031:### 9.5 The complete shortcut table
2140:### 9.6 The command palette
2156:### 9.7 Cut, copy, and paste of files
2178:## 10. RESPONSIVE — SIZE-AGNOSTIC
2183:### 10.1 The six widths
2196:### 10.2 The height axis, which is the one everyone forgets
2209:### 10.3 The pane stack
2221:### 10.4 What the app bar does at each width
2256:### 10.5 The drawer, and when it exists
2275:### 10.6 Ultrawide
2291:### 10.7 The "no ugly control" law, per size
2309:### 10.8 Resize behaviour
2328:## 11. THE LIBRARY: SOURCES, SCANNING, AND THE INDEX
2330:### 11.1 Roots
2358:### 11.2 The scan
2388:### 11.3 What counts as a photo
2403:### 11.4 `taken_at`, resolved once
2429:### 11.5 Orientation
2436:### 11.6 The index
2471:### 11.7 Scan budgets
2491:### 11.8 Live watching
2510:### 11.9 Removable devices and import
2529:### 11.10 Missing, and Trash
2544:## 12. THUMBNAILS AND PERFORMANCE
2546:### 12.1 The two caches
2582:### 12.2 The decode pipeline
2613:### 12.3 Texture residency
2636:### 12.4 The priority queue
2653:### 12.5 Threading, enumerated
2667:### 12.6 Level of detail
2685:### 12.7 Placeholders and the broken tile
2700:### 12.8 What is measured, and when it is a failure
2721:## 13. EDITING — NON-DESTRUCTIVE, SIDECAR-BACKED
2723:### 13.1 The law

```

### TOOL: shell [seq 271] 2026-11-09 13:48:56 status=completed

```json
{
  "command": "grep -n \"^#\\{1,3\\} \" /home/lufuno/smart/naledzi_ui/examples/smart-photos/SPEC.md | sed -n '120,220p'\necho ===THUMBS-RS===\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/thumbs.rs"
}
```

```
2723:### 13.1 The law
2732:### 13.2 The sidecar
2780:### 13.3 What the editor can do
2801:### 13.4 The editor surface
2821:### 13.5 Export — where pixels are allowed
2844:### 13.6 Caption and title
2853:## 14. ALBUMS, TAGS, RATINGS, TRASH
2855:### 14.1 Ratings
2874:### 14.2 Tags
2891:### 14.3 Albums
2906:### 14.4 Trash
2914:## 15. WINDOW DECORATIONS — SERVER-SIDE RENDERED
2923:### 15.1 The window
2953:### 15.2 The title bar is not ours
2971:### 15.3 Rounded corners: the number does not exist
2978:### 15.4 The app bar is not a frame
2991:### 15.5 The window is one window
3005:## 16. STATES, TOASTS, UNDO
3007:### 16.1 Every state a surface can be in
3024:### 16.2 Empty states
3043:### 16.3 Toasts
3065:### 16.4 The confirmation that earns its place
3081:### 16.5 The error surface
3096:## 17. MODULE ARCHITECTURE
3098:### 17.1 The law, inherited
3106:### 17.2 The modules
3164:### 17.3 The `Msg` enum
3207:### 17.4 Why the index is not capnp, and why that could change
3221:### 17.5 Crate dependencies
3244:# decode
3257:# metadata, identity
3261:# concurrency — exactly one pool, §12.5
3265:# filesystem
3271:# time, locations, persistence
3296:### 17.6 What v2 needs to prove before it is admitted
3307:### 17.7 The `.desktop` file and the file handler
3326:## 18. TEST STRATEGY
3328:### 18.1 The rule
3337:### 18.2 `tests/geometry.rs` — pure model, no display, no filesystem
3400:### 18.3 `tests/index.rs` — a real filesystem, real files
3423:### 18.4 `tests/xmp.rs` — the sidecar, byte-exact
3442:### 18.5 `tests/selection.rs` and `tests/keyboard.rs` — state, no display
3469:### 18.6 `tests/perf.rs` — the budgets
3485:### 18.7 What is deliberately not tested
3494:### 18.8 The performance of the tests
3510:### 18.9 `tests/geometry.rs` — the rects, not the decisions
3627:## 19. ACCEPTANCE
3629:### 19.1 The twenty things that make it a daily driver
3654:### 19.2 Definition of done
3668:### 19.3 What this app refuses to become
3678:## APPENDIX A — THE COMMAND INDEX
3776:## APPENDIX B — THE TOKEN INDEX
4097:## APPENDIX C — RESEARCH SOURCES
===THUMBS-RS===
//! Thumbnails: the cache key, the priority queue, and the one pool.
//!
//! §12.5's enumeration: the runtime, `min(8, max(2, cores / 2))` workers created
//! once, one index worker, one watcher, and **zero per-item threads** — because
//! per-item thread pools are how an app reaches 31 threads, and 31 threads for
//! one user was measured at 990 MB.

use std::collections::VecDeque;
use std::path::PathBuf;

use crate::model::PhotoId;

/// The ladder, on the long edge. Multiple sizes are the design, not an
/// optimisation: reducing from 128 is far cheaper than reducing from 360, and the
/// grid needs 64 while the viewer needs 2048.
pub const LADDER: [u32; 7] = [64, 128, 256, 512, 1024, 2048, 4096];
/// The cache's cap, and it is a cache: the user may delete it at any moment.
pub const CAP_BYTES: u64 = 2 << 30;
/// The fanout, 256 ways. A 100,000-entry directory is a `readdir` disaster on
/// ext4 and a `stat` disaster on btrfs.
pub const FANOUT: u32 = 256;
pub const QUALITY: u8 = 82;
/// The four priority tiers, and the discipline that matters is the last row of
/// §12.4: **the queue is rebuilt per scroll direction rather than accumulated.**
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tier {
    /// The photograph being opened, the focused cell, the selected set.
    P0,
    /// Cells inside the viewport.
    P1,
    /// The viewport plus one screen, in the direction of travel.
    P2,
    /// The next 500 in the direction of travel; the whole library on idle.
    P3,
}

impl Tier {
    pub const ALL: [Tier; 4] = [Tier::P0, Tier::P1, Tier::P2, Tier::P3];
}

/// One request, and it carries everything the key needs so a second pass does not
/// re-stat anything.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Request {
    pub id: PhotoId,
    pub path: PathBuf,
    pub rung: u32,
    pub mtime_ns: i64,
    pub orientation: u8,
    /// A non-destructive edit changes the *render*, not the file, so the file's
    /// mtime is unchanged and cannot be the signal.
    pub edit_generation: u32,
    pub tier: Tier,
}

/// The cache key, and every field in it is there because a stale thumbnail is a
/// permanent visual bug.
///
/// `BLAKE3(abspath ++ 0x00 ++ bucket_px ++ 0x00 ++ mtime_ns ++ 0x00 ++
///         orientation ++ 0x00 ++ edit_generation)`
#[must_use]
pub fn key(request: &Request) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(request.path.as_os_str().as_encoded_bytes());
    hasher.update(&[0u8]);
    hasher.update(&request.rung.to_le_bytes());
    hasher.update(&[0u8]);
    hasher.update(&request.mtime_ns.to_le_bytes());
    hasher.update(&[0u8]);
    hasher.update(&[request.orientation]);
    hasher.update(&[0u8]);
    hasher.update(&request.edit_generation.to_le_bytes());
    *hasher.finalize().as_bytes()
}

/// The cache path, with two and two hex of fanout and a 128-bit hex name.
#[must_use]
pub fn cache_path(request: &Request) -> PathBuf {
    let digest = key(request);
    let hex: String = digest[..16].iter().map(|byte| format!("{byte:02x}")).collect();
    let root = crate::model::cache_dir().join("thumbs").join("v1");
    root.join(&hex[0..2]).join(&hex[2..4]).join(format!("{hex}.webp"))
}

/// The rung to ask for at a cell size.
///
/// Never larger than `1.5 x needed` — requesting the 2048 rung for a 900 px cell
/// is 16.8 MiB of texture for a 900 px square — and **re-request only above a 25 %
/// change**, because slight softness during a drag is invisible and a re-decode
/// storm is not.
#[must_use]
pub fn wanted_rung(cell_px: f32, dpr: f32) -> u32 {
    let needed = (cell_px * dpr).ceil() as u32 + 1;
    let at_least = LADDER.iter().copied().find(|rung| *rung >= needed).unwrap_or(4096);
    // The 1.5x ceiling **snaps down to the ladder**. Taking `min` of two numbers
    // returns a number that is on neither, and a 97 px rung is not a rung at all:
    // it is a cache path no ladder rung hashes to and no budget accounts for.
    let ceiling = (needed * 3 / 2).max(64);
    let mut snapped = LADDER[0];
    for rung in LADDER {
        if rung > ceiling {
            break;
        }
        snapped = rung;
    }
    snapped.min(at_least)
}

/// Whether a resident rung is good enough, or the change is above the 25 % that
/// makes a re-request worth it.
///
/// `want * 4 > have * 5` is 25 % in integers, and the direction matters: `have * 4
/// < want` is a **300 %** change, which is why a 256 px cell resized to 320 px sat
/// on its old thumbnail for a second and a half and then popped. `G33` reads the
/// boundary from both sides.
#[must_use]
pub const fn needs_request(have: u32, want: u32) -> bool {
    have == 0 || have * 5 < want * 4
}

/// The texture budget, and it is a fraction of RAM with a floor and a ceiling.
#[must_use]
pub fn gpu_budget(bytes: u64) -> u64 {
    (bytes / 10).clamp(128 << 20, 512 << 20)
}

/// The CPU bitmap cache, for re-rasterising after a resize.
#[must_use]
pub fn cpu_budget(bytes: u64) -> u64 {
    (bytes / 20).clamp(64 << 20, 256 << 20)
}

/// Uploads are **batched**: a queue drained once per frame, capped at eight, and
/// skipped entirely on frames where the scroll delta exceeds 2 px — above 1024 px
/// a texture is 4 MiB and eight of those is 32 MiB of PCIe traffic inside 16 ms.
pub const MAX_UPLOADS_PER_FRAME: usize = 8;
pub const SCROLL_SKIP_PX: f32 = 2.0;

/// The queue, and it is **bounded**: a queue that grows without bound during a
/// fast flick is "I scrolled into a day with 10,000 photos and it goes black".
#[derive(Debug, Default)]
pub struct Queue {
    tiers: [VecDeque<Request>; 4],
    pub dropped: u64,
}

impl Queue {
    #[must_use]
    pub fn new() -> Self {
        Self { tiers: std::array::from_fn(|_| VecDeque::new()), dropped: 0 }
    }

    /// The selected set is capped at 200, because a selection of 4,000 cells is
    /// 4,000 requests and the user is looking at four of them.
    pub
```

### TOOL: shell [seq 278] 2026-11-09 13:48:59 status=completed

```json
{
  "command": "echo ===MEDIA===\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/media.rs | head -n 200\necho ===SCAN===\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/scan.rs | head -n 160"
}
```

```
===MEDIA===
//! Decode dispatch, EXIF, and the dimension probe.
//!
//! One decoder per format, dispatched by extension, and the reduced-scale path
//! for the formats that have one: a 6,000 x 4,000 photograph needs a 256 px
//! thumbnail, and an IDCT-scaled decode produces that in roughly the time a
//! 64 px thumbnail takes from a 640 x 480 source.

use std::path::{Path, PathBuf};

use crate::model::Photo;

/// What a probe learns from a header. It reads 64 KiB and no more, because a
/// stat-and-probe per file is the expensive part of a scan.
#[derive(Clone, Default, Debug)]
pub struct Header {
    pub dims: Option<(u32, u32)>,
    pub kind: u8,
}

/// What an EXIF walk learns, bounded to the first 128 KiB.
#[derive(Clone, Default, Debug)]
pub struct Exif {
    pub orientation: Option<u8>,
    pub taken_at_utc: Option<i64>,
    pub camera: Option<(String, String)>,
    pub gps: Option<(f64, f64)>,
    pub software: Option<String>,
    pub container_time: Option<i64>,
}

/// Apply the EXIF orientation to a dimension pair, so the cell's aspect, the
/// thumbnail bake and the viewer's fit all agree.
#[must_use]
pub fn oriented(dims: Option<(u32, u32)>, orientation: u8) -> Option<(u32, u32)> {
    let (w, h) = dims?;
    match orientation {
        5..=8 => Some((h, w)),
        _ => Some((w, h)),
    }
}

/// Read the dimensions from the header alone.
///
/// `image`'s reader is asked for the size and not for the pixels, so a 16 MP
/// photograph costs a seek and a read rather than a decode.
#[must_use]
pub fn probe_header(path: &Path) -> Header {
    let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    if name.ends_with(".svg") {
        return Header { dims: svg_size(path), kind: 1 };
    }
    if name.ends_with(".qoi") {
        return Header { dims: qoi_size(path), kind: 2 };
    }
    if name.ends_with(".jxl") {
        return Header { dims: jxl_size(path), kind: 3 };
    }
    if name.ends_with(".gif") {
        return Header { dims: gif_size(path), kind: 4 };
    }
    let Ok(reader) = image::ImageReader::open(path) else {
        return Header::default();
    };
    let reader = match reader.with_guessed_format() {
        Ok(reader) => reader,
        Err(_) => return Header::default(),
    };
    match reader.into_dimensions() {
        Ok(dims) => Header { dims: Some((dims.0, dims.1)), kind: 0 },
        Err(_) => Header::default(),
    }
}

fn read_head(path: &Path, bytes: usize) -> Option<Vec<u8>> {
    use std::io::Read as _;
    let mut file = std::fs::File::open(path).ok()?;
    let mut buffer = vec![0u8; bytes.min(crate::index::HEADER_BYTES as usize)];
    let read = file.read(&mut buffer).ok()?;
    buffer.truncate(read);
    Some(buffer)
}

fn svg_size(path: &Path) -> Option<(u32, u32)> {
    let text = std::fs::read_to_string(path).ok()?;
    let width = attribute(&text, "width")?;
    let height = attribute(&text, "height")?;
    Some((width, height))
}

fn attribute(text: &str, name: &str) -> Option<u32> {
    let at = text.find(name)?;
    let rest = &text[at + name.len()..];
    let rest = rest.trim_start().strip_prefix('=')?.trim_start();
    let rest = rest.strip_prefix('"').or_else(|| rest.strip_prefix('\''))?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

fn qoi_size(path: &Path) -> Option<(u32, u32)> {
    let head = read_head(path, 14)?;
    if head.len() < 14 || &head[..4] != b"qoif" {
        return None;
    }
    Some((
        u32::from_be_bytes(head[4..8].try_into().ok()?),
        u32::from_be_bytes(head[8..12].try_into().ok()?),
    ))
}

/// A JPEG XL's size, from the crate that reads JPEG XL.
///
/// There were two hand-rolled header parsers here — one for the codestream's
/// `div8` arithmetic and one that searched the first 10 KiB of the container for
/// the text `width=`. Both were wrong: the codestream one read the 16-bit size
/// eight times too wide, and the container one is a text search over binary data
/// that finds `width=` in a thumbnail's EXIF and calls it the image. A decoder is
/// already a dependency and it already knows the format; §0.3's own rule — *a
/// library, never a C FFI* — is satisfied by using it rather than by
/// reimplementing its header.
fn jxl_size(path: &Path) -> Option<(u32, u32)> {
    let file = std::fs::File::open(path).ok()?;
    let image = jxl_oxide::JxlImage::builder().read(std::io::BufReader::new(file)).ok()?;
    let size = &image.image_header().size;
    Some((size.width, size.height))
}

fn gif_size(path: &Path) -> Option<(u32, u32)> {
    let head = read_head(path, 10)?;
    if head.len() < 10 || &head[..3] != b"GIF" {
        return None;
    }
    Some((
        u16::from_le_bytes(head[6..8].try_into().ok()?) as u32,
        u16::from_le_bytes(head[8..10].try_into().ok()?) as u32,
    ))
}

/// The EXIF walk. `kamadak-exif` on the already-open handle, bounded to the
/// first 128 KiB, so it is free on the same read the header probe used.
#[must_use]
pub fn probe_exif(path: &Path) -> Option<Exif> {
    use exif::{In, Tag, Value};
    let file = std::fs::File::open(path).ok()?;
    let mut bufreader = std::io::BufReader::new(file);
    let exif = exif::Reader::new()
        .read_from_container(&mut bufreader)
        .ok()?;
    let field = |tag: Tag| -> Option<String> {
        let found = exif.get_field(tag, In::PRIMARY)?;
        let Value::Ascii(ref value) = found.value else { return None };
        // An EXIF ASCII field is bytes, and a NUL-terminated one must lose the
        // NUL before `parse` sees it.
        let joined = value.concat();
        let text = String::from_utf8_lossy(&joined);
        Some(text.trim_end_matches('\0').to_owned())
    };
    let taken = field(Tag::DateTimeOriginal)
        .or_else(|| field(Tag::DateTimeDigitized))
        .or_else(|| field(Tag::DateTime))
        .and_then(|text| parse_exif_datetime(&text));
    let camera = match (field(Tag::Make), field(Tag::Model)) {
        (Some(make), Some(model)) => Some((make.trim().to_owned(), model.trim().to_owned())),
        (None, Some(model)) => Some((String::new(), model.trim().to_owned())),
        _ => None,
    };
    let gps = gps_pair(&exif);
    Some(Exif {
        orientation: exif
            .get_field(Tag::Orientation, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0))
            .and_then(|v| u8::try_from(v).ok()),
        taken_at_utc: taken,
        camera,
        gps,
        software: field(Tag::Software),
        container_time: None,
    })
}

#[must_use]
pub fn parse_exif_datetime(text: &str) -> Option<i64> {
    // `YYYY:MM:DD HH:MM:SS`, and nothing else. A general date parser is how a
    // library fills up with photographs from 2001.
    let (date, clock) = text.trim().split_once(' ')?;
    let mut parts = date.split(':');
    let number = |part: &str| -> Option<i64> { part.trim().parse().ok() };
    let year = number(parts.next()?)?;
    let month = number(parts.next()?)?;
    let day = number(parts.next()?)?;
    let clock = clock.trim();
    let (h, m, s) = match clock.split_once(':') {
        Some((h, rest)) => match rest.split_once(':') {
            Some((m, s)) => (h, m, s),
            None => (h, rest, "00"),
        },
        None => (clock, "00", "00"),
    };
    crate::model::utc(
        year.to_string().as_str(),
        month.to_string().as_str(),
===SCAN===
//! The scan: a cheap listing in bulk, the expensive thing second.
//!
//! §11.2's three deliberate restraints: a directory listing is cheap and a
//! stat-and-probe per file is not; symlinks are not followed, because a symlink
//! loop in a photo library is a real and recurring problem; and hidden files are
//! skipped, because screenshot tools write to `~/.cache` and
//! `Pictures/.thumbnails` is not the user's photographs.

use std::path::{Path, PathBuf};

use crate::index::{IndexSnapshot, PER_ROOT_CEILING, SCAN_DEPTH};
use crate::media;
use crate::model::{Kind,
```

### TOOL: shell [seq 289] 2026-11-09 13:49:04 status=completed

```json
{
  "command": "echo ===INDEX===\nhead -n 220 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/index.rs\necho ===XMP-HEAD===\nhead -n 140 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/xmp.rs\necho ===UNDO===\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/undo.rs"
}
```

```
===INDEX===
//! The index: a validated, self-healing binary cache in `$XDG_CACHE_HOME`.
//!
//! §2.1's whole argument: **the application does not own the directory it
//! indexes.** A database is a *claim* about a folder the user edits in a file
//! manager, and every way that claim becomes false is a bug class. So the index
//! is validated rather than trusted, a mismatch is a miss rather than an error,
//! and **a corrupt index is a missing index** — a truncated file is unlinked and
//! rebuilt, and the user may `rm -rf ~/.cache/smart-photos` at any moment.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::model::{Photo, PhotoId, Kind, TakenSource, photo_id};

/// The magic, the version, and a length. A file without all three is a file from
/// a different app, and it is deleted rather than misread.
pub const MAGIC: &[u8; 8] = b"SMPHOTOS";
pub const VERSION: u32 = 1;
/// The scan publishes every 500 items, and the index is written once at the end
/// of a scan rather than on every record.
pub const PUBLISH_EVERY: u32 = 500;
/// The scan's per-file probe is bounded: 128 KiB for the header and the EXIF
/// walk, 64 KiB for a header-only probe.
pub const HEADER_BYTES: u64 = 64 * 1024;
pub const EXIF_BYTES: u64 = 128 * 1024;
pub const SCAN_DEPTH: usize = 64;
pub const PER_ROOT_CEILING: usize = 4_000_000;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IndexSnapshot {
    pub photos: Vec<Photo>,
    /// Files that were seen and are not photographs. They are counted and
    /// reported, because "12,441 photos · 2,109 skipped" is how a user finds the
    /// `.PSD` they forgot about.
    pub skipped: u32,
    pub missing: u32,
}

impl IndexSnapshot {
    #[must_use]
    pub fn len(&self) -> usize {
        self.photos.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.photos.is_empty()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScanPhase {
    Enumerate,
    Probe,
    Done,
    Failed(String),
}

impl ScanPhase {
    #[must_use]
    pub const fn sentence(&self) -> &'static str {
        match self {
            ScanPhase::Enumerate => "Looking for photographs",
            ScanPhase::Probe => "Reading photographs",
            ScanPhase::Done => "Ready",
            ScanPhase::Failed(_) => "Stopped",
        }
    }
}

pub fn path() -> PathBuf {
    crate::model::cache_dir().join("index.v1")
}

/// Write the index: one file, one write, no compression.
///
/// 100,000 records at about 400 bytes is 40 MB, and reading 40 MB is about 40 ms,
/// which is the entire point of §2.1.
///
/// Delegates to [`store_at`] at the real cache root. The indirection exists so a
/// test can exercise the REAL serialiser — the same encode, the same `sync_all`,
/// the same atomic rename — against a directory it owns, instead of writing 8 MB
/// into the user's `~/.cache` and fsyncing it there.
pub fn store(snapshot: &IndexSnapshot) -> Result<PathBuf, String> {
    store_at(&crate::model::cache_dir(), snapshot)
}

/// [`store`] at an explicit root. `root` must exist or be creatable.
pub fn store_at(root: &std::path::Path, snapshot: &IndexSnapshot) -> Result<PathBuf, String> {
    let target = root.join("index.v1");
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Could not create {}: {e}", parent.display()))?;
    }
    // **Per thread, and not per process.** It was `v1.tmp.<pid>`, which is one name for every
    // thread in the process: two concurrent `store()` calls wrote the same temp file and renamed
    // over each other, and the loser's rename either failed or published the other's half-written
    // index. `p7` writes this file while the rest of `tests/perf.rs` is building 50,000-record
    // states, so the race was reachable from the test suite itself. `AtomicU64` is a `static` of a
    // primitive, which `K4`'s equivalent in this crate's tests does not forbid, and it allocates
    // nothing.
    static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let temp = target.with_extension(format!(
        "v1.tmp.{}.{}",
        std::process::id(),
        SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let mut bytes = Vec::with_capacity(snapshot.photos.len() * 400);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&(snapshot.photos.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&snapshot.skipped.to_le_bytes());
    bytes.extend_from_slice(&snapshot.missing.to_le_bytes());
    for photo in &snapshot.photos {
        bytes.extend_from_slice(&encode(photo));
    }
    {
        let mut file = std::fs::File::create(&temp)
            .map_err(|e| format!("Could not write {}: {e}", temp.display()))?;
        file.write_all(&bytes)
            .map_err(|e| format!("Could not write {}: {e}", temp.display()))?;
        file.sync_all().map_err(|e| format!("Could not flush {}: {e}", temp.display()))?;
    }
    std::fs::rename(&temp, &target)
        .map_err(|e| format!("Could not replace {}: {e}", target.display()))?;
    Ok(target)
}

/// Read the index. **A corrupt index is a missing index**: the file is unlinked
/// and the caller rescans, and the user sees no error at all.
#[must_use]
pub fn load() -> Option<IndexSnapshot> {
    load_from(&crate::model::cache_dir())
}

/// [`load`] at an explicit root — the read half of [`store_at`].
#[must_use]
pub fn load_from(root: &std::path::Path) -> Option<IndexSnapshot> {
    let file_path = root.join("index.v1");
    let mut file = std::fs::File::open(&file_path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;
    match decode(&bytes) {
        Some(snapshot) => Some(snapshot),
        None => {
            // Self-healing: a truncated or unmagic'd cache is deleted, because it
            // lives in `~/.cache` and the user may delete it at any moment.
            let _ = std::fs::remove_file(&file_path);
            None
        }
    }
}

fn encode(photo: &Photo) -> Vec<u8> {
    let mut out = Vec::with_capacity(256);
    out.extend_from_slice(&photo.id);
    put_str(&mut out, photo.path.to_string_lossy().as_ref());
    out.extend_from_slice(&photo.source.to_le_bytes());
    put_str(&mut out, &photo.name);
    out.push(match photo.kind {
        Kind::Image => 0,
        Kind::Video => 1,
        Kind::Raw => 2,
    });
    out.extend_from_slice(&photo.size.to_le_bytes());
    out.extend_from_slice(&photo.mtime_ns.to_le_bytes());
    out.extend_from_slice(&photo.dims.0.to_le_bytes());
    out.extend_from_slice(&photo.dims.1.to_le_bytes());
    out.extend_from_slice(&photo.taken_at_utc.to_le_bytes());
    out.push(match photo.taken_source {
        TakenSource::Exif => 0,
        TakenSource::FileName => 1,
        TakenSource::Container => 2,
        TakenSource::Modified => 3,
        TakenSource::Unknown => 4,
    });
    out.push(photo.rating);
    out.push(u8::from(photo.favourite));
    put_str(&mut out, &photo.tags.join("\u{1}"));
    put_str(&mut out, &photo.caption);
    put_str(&mut out, &photo.title);
    match photo.gps {
        Some((lat, lon, alt)) => {
            out.push(1);
            out.extend_from_slice(&lat.to_le_bytes());
            out.extend_from_slice(&lon.to_le_bytes());
            out.extend_from_slice(&alt.unwrap_or(0.0).to_le_bytes());
        }
        None => out.push(0),
    }
    match &photo.camera {
        Some((make, model)) => {
            out.push(1);
            put_str(&mut out, make);
            put_str(&mut out, model);
        }
        None => out.push(0),
    }
    out.extend_from_slice(&photo.dominant);
    out.extend_from_slice(&photo.hash256k);
    out.push(u8::from(photo.available));
    out.extend_from_slice(&photo.missing_since.unwrap_or(0).to_le_bytes());
    out
}

fn put_str(out: &mut Vec<u8>, text: &str) {
    let bytes = text.as_bytes();
    out.extend_from_slice(&(by
```

### TOOL: shell [seq 296] 2026-11-09 13:49:08 status=completed

```json
{
  "command": "echo ===MODEL===\nhead -n 200 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model.rs\necho ===SEARCH===\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/search.rs | head -n 120\necho ===PLACES===\ncat /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/places.rs | head -n 100\necho ===SETTINGS===\nhead -n 120 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/settings.rs"
}
```

```
===MODEL===
//! The model. Pure geometry, no widgets, no filesystem, no display.
//!
//! The law: **the model decides, the painter paints, tests read the model.**
//! Nothing here renders a pixel, and nothing in `view` re-decides anything a
//! function in here already decided.

pub mod cells;
pub mod grid;
pub mod viewer;

use std::cmp::Reverse;
use std::path::{Path, PathBuf};

use smart_ui_core::layout::Sp;

/// 16 bytes, derived from the **device and the inode**.
pub type PhotoId = [u8; 16];

/// A file's identity, hashed. `blake3` of the device and the inode — and
/// **nothing else**.
///
/// The path was in here too, and the doc comment above it said a rename keeps its
/// identity because the inode is stable, which is exactly what including the path
/// prevents: `blake3(path ++ dev ++ inode)` changes when the path changes, so
/// every rename in a watcher produced a "new" photograph. The old record stayed
/// marked missing, the album membership stayed with a record nothing resolves to,
/// and the timeline grew a duplicate. `I9` and the identity test are what found
/// it, and both are one rename away from a real library.
///
/// Zeros still hash: a file that is missing at scan time keeps the same identity
/// when it comes back, which is what a rename-in looks like.
pub fn photo_id(path: &Path, dev: u64, inode: u64) -> PhotoId {
    let mut hasher = blake3::Hasher::new();
    let _ = path;
    hasher.update(&dev.to_le_bytes());
    hasher.update(&inode.to_le_bytes());
    *hasher.finalize().as_bytes().first_chunk().expect("16 bytes is a chunk")
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Image,
    Video,
    Raw,
}

impl Kind {
    /// An extension is a kind. No MIME sniff on the hot path: a file with no
    /// extension is indexed by sniffing, and everything else is indexed by this.
    #[must_use]
    pub fn from_extension(extension: &str) -> Option<Kind> {
        let lower = extension.to_ascii_lowercase();
        const IMAGE: &[&str] = &[
            "jpg", "jpeg", "jfif", "png", "webp", "gif", "bmp", "tiff", "tif", "avif", "jxl",
            "qoi", "tga", "ico", "ppm", "pgm", "pnm", "pam", "ff", "heic", "heif", "svg",
        ];
        const VIDEO: &[&str] =
            &["mp4", "mkv", "webm", "mov", "avi", "m4v", "mpg", "mpeg"];
        const RAW: &[&str] = &[
            "cr2", "cr3", "nef", "arw", "dng", "orf", "rw2", "raf", "pef", "srw", "x3f", "3fr",
            "fff", "rwl", "raw",
        ];
        if IMAGE.contains(&lower.as_str()) {
            Some(Kind::Image)
        } else if VIDEO.contains(&lower.as_str()) {
            Some(Kind::Video)
        } else if RAW.contains(&lower.as_str()) {
            Some(Kind::Raw)
        } else {
            None
        }
    }
}

/// Where a capture time came from. Stored with the value, because "why is this
/// photograph in 2019" is a question the user will ask.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TakenSource {
    /// `EXIF DateTimeOriginal`, `DateTimeDigitized` or `DateTime`, in that order.
    Exif,
    /// One of four filename shapes. **Only these four**: a general
    /// date-from-string is how a library fills up with photographs from 2001,
    /// from a filename containing a `2`.
    FileName,
    /// The container's creation time, for a video.
    Container,
    /// The file's mtime.
    Modified,
    /// Unresolvable, and rendered with a badge rather than guessed at.
    Unknown,
}

impl TakenSource {
    /// The sentence the info panel shows beside the date. It is a sentence
    /// because a user who cannot tell where a date came from does not trust the
    /// date.
    #[must_use]
    pub const fn sentence(self) -> &'static str {
        match self {
            TakenSource::Exif => "EXIF",
            TakenSource::FileName => "filename",
            TakenSource::Container => "container",
            TakenSource::Modified => "file modified time",
            TakenSource::Unknown => "unknown",
        }
    }
}

/// The four filename shapes §11.4 admits, and nothing else.
#[must_use]
pub fn taken_from_file_name(name: &str) -> Option<(i64, TakenSource)> {
    let bytes = name.as_bytes();
    // `YYYYMMDD_HHMMSS` and `YYYYMMDD-HHMMSS` anywhere in the name.
    for at in 0..bytes.len().saturating_sub(15) {
        let window = name.get(at..at + 15)?;
        let date = window.get(0..8)?;
        let (Some(y), Some(m), Some(d)) = (date.get(..4), date.get(4..6), date.get(6..8)) else {
            continue;
        };
        if !y.bytes().chain(m.bytes()).chain(d.bytes()).all(|b| b.is_ascii_digit()) {
            continue;
        }
        let clock = window.get(9..15)?;
        if !clock.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let (Some(h), Some(mi), Some(s)) =
            (clock.get(0..2), clock.get(2..4), clock.get(4..6))
        else {
            continue;
        };
        return utc(y, m, d, h, mi, s).map(|t| (t, TakenSource::FileName));
    }
    // `YYYY-MM-DD HH.MM.SS` and `YYYY-MM-DD`.
    for at in 0..bytes.len().saturating_sub(9) {
        let Some(window) = name.get(at..at + 10) else { continue };
        if window.as_bytes()[4] != b'-' || window.as_bytes()[7] != b'-' {
            continue;
        }
        if !window.bytes().enumerate().all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit()) {
            continue;
        }
        let (Some(y), Some(m), Some(d)) =
            (window.get(..4), window.get(5..7), window.get(8..10))
        else {
            continue;
        };
        if let Some(time) = utc(y, m, d, "00", "00", "00") {
            return Some((time, TakenSource::FileName));
        }
    }
    None
}

/// Seconds UTC for six two-digit fields, or `None` for a date that does not
/// exist. A `31st of February` is not a photograph from the 31st of February.
#[must_use]
pub fn utc(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str) -> Option<i64> {
    let number = |text: &str| -> Option<i64> { text.trim().parse().ok() };
    let y = number(year)?;
    let m = number(month)?;
    let d = number(day)?;
    let h = number(hour)?;
    let mi = number(minute)?;
    let se = number(second)?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || h > 23 || mi > 59 || se > 60 {
        return None;
    }
    if d > days_in_month(y, m) {
        return None;
    }
    Some(days_from_civil(y, m, d) * 86_400 + h * 3_600 + mi * 60 + se)
}

#[must_use]
pub const fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Days since 1970-01-01 for a proleptic Gregorian date. Howard Hinnant's
/// algorithm: four lines, no lookup table, and no `chrono` in the hot path.
#[must_use]
pub const fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
===SEARCH===
//! The query surface and the palette's ranking.
//!
//! Search is free text over the filename, the caption, the tags and the camera
//! make and model, and it is **capped**: a query that would read every field of
//! every record is a query that stops being a search and becomes a scan.

use crate::model::{Photo, PhotoId};

/// The cap, and it is a fact the bar's support line reports rather than a
/// spinner over content that is already there.
pub const MAX_SCAN: usize = 50_000;
/// Type-ahead's window, and it is a function and not a widget flag.
pub const TYPE_AHEAD_MS: u64 = 800;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Query {
    pub t
```

### TOOL: shell [seq 303] 2026-11-09 13:49:11 status=completed

```json
{
  "command": "echo ===STATE-FUNCS===\ngrep -n \"^pub fn\\|^fn \\|^    pub fn\\|^    fn \\|EditCmd\\|Crop\\|Rotate\\|Filter\\|Adjust\\|History\\|Trash\\|Album\\|import\\|Import\" /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/state.rs | head -n 120\necho ===STATE-REDUCER-HEAD===\nsed -n '1,140p' /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/state.rs\necho ===GRID===\nhead -n 120 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/grid.rs\necho ===VIEWER===\nhead -n 120 /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/model/viewer.rs"
}
```

```
===STATE-FUNCS===
22:    Filter,
48:    pub fn new() -> Self {
53:    pub fn len(&self) -> usize {
58:    pub fn is_empty(&self) -> bool {
63:    pub fn contains(&self, id: &PhotoId) -> bool {
70:    pub fn uniform_kind(&self, kinds: impl Fn(&PhotoId) -> Kind) -> bool {
83:    pub fn all_kind(&self, want: Kind, kinds: impl Fn(&PhotoId) -> Kind) -> bool {
88:    pub fn click(&mut self, id: PhotoId) {
100:    pub fn toggle(&mut self, id: PhotoId) {
115:    pub fn extend(&mut self, ids: &[PhotoId], target: PhotoId, replace: bool) {
131:    pub fn marquee(&mut self, ids: &[PhotoId], anchor: PhotoId, merge: Merge) {
153:    pub fn clear(&mut self) {
161:    pub fn select_all(&mut self, ids: &[PhotoId]) {
164:        self.source = SelectionSource::Filter;
168:    pub fn invert(&mut self, ids: &[PhotoId]) {
179:        self.source = SelectionSource::Filter;
183:    pub fn deselect(&mut self, ids: &[PhotoId]) {
192:    pub fn move_focus(&mut self, to: PhotoId) {
198:    pub fn focus_extend(&mut self, to: PhotoId) {
211:    pub fn canonical_order(&self, order: &[PhotoId]) -> Vec<PhotoId> {
219:    pub fn holding_line(&self, kinds: impl Fn(&PhotoId) -> Kind) -> String {
245:    pub fn held_bytes(&self, sizes: impl Fn(&PhotoId) -> u64) -> u64 {
298:    AlbumList,
329:    pub fn push(&mut self, toast: smart_ui_core::feedback::Toast) {
337:    pub fn visible(&self) -> Vec<&smart_ui_core::feedback::Toast> {
341:    pub fn expire(&mut self, id: u32) {
383:    SlideshowCmd, TrashCmd, WatchEvent,
524:    Trash { ids: Vec<PhotoId>, permanent: bool },
526:    EmptyTrash,
530:    fn default() -> Self {
624:    pub fn subscription(state: &Self) -> iced::Subscription<crate::Msg> {
632:    pub fn photos(&self) -> &[Photo] {
641:    pub fn rebuild(&mut self) {
690:    fn announce_hidden(&mut self) {
715:    fn rebuild_dialog(&mut self) {
731:    fn rebuild_settings(&mut self) {
743:    pub fn photo_by_index(&self, index: u32) -> Option<&Photo> {
750:    pub fn visible_ids(&self) -> Vec<PhotoId> {
756:    pub fn photo(&self, id: PhotoId) -> Option<&Photo> {
763:    pub fn visible_photos(&self) -> impl Iterator<Item = &Photo> + '_ {
769:    pub fn visible_at(&self, index: usize) -> Option<&Photo> {
774:    pub fn visible_len(&self) -> usize {
782:    pub fn visible_position(&self, id: PhotoId) -> Option<usize> {
792:    fn rebuild_info(&mut self) {
819:    fn route_holds(&self, photo: &Photo) -> bool {
846:    fn facet_holds(&self, photo: &Photo) -> bool {
860:    pub fn kind_of(&self, id: &PhotoId) -> Kind {
865:    pub fn current_photo(&self) -> Option<&Photo> {
873:    pub fn route_title(&self) -> String {
886:            "trash" => "Trash".to_owned(),
895:    pub fn support_line(&self) -> String {
903:    pub fn day_label(&self, taken_at_utc: i64) -> String {
908:    pub fn has_device(&self) -> bool {
930:    pub fn grid(&self) -> GridSpec {
935:    pub fn columns(&self) -> usize {
940:    pub fn cell_pitch(&self) -> Sp {
946:    pub fn built_range(&self) -> std::ops::Range<usize> {
954:    pub fn cell_budget(&self) -> usize {
961:    pub fn move_focus(&mut self, direction: Direction, modifiers: Modifiers) {
989:    pub fn scroll_to_focus(&mut self) {
1004:    pub fn page(&mut self, up: bool) {
1020:    pub fn press(&mut self, id: PhotoId, modifiers: Modifiers) {
1032:    fn sync_focus(&mut self, id: PhotoId) {
1041:    pub fn rectangle(&self, from: Option<PhotoId>, to: PhotoId) -> Vec<PhotoId> {
1050:    pub fn marquee(&mut self, to: Cell, merge: MergeMode, modifiers: Modifiers) {
1086:    pub fn marquee_end(&mut self) {
1094:    pub fn open(&mut self, id: PhotoId) {
1102:    pub fn close_viewer(&mut self) {
1113:    pub fn step(&mut self, delta: isize) {
1126:    pub fn zoom_by(&mut self, factor: f32, up: bool, cursor: Option<(Sp, Sp)>) {
1139:    pub fn viewport(&self) -> (Sp, Sp) {
1145:    pub fn clamp_pan(&mut self, dims: (u32, u32), scale: f32) {
1151:    pub fn rotate(&mut self, step: i8) {
1165:    pub fn write_sidecar(&mut self, id: PhotoId, change: impl FnOnce(&mut crate::xmp::Sidecar)) -> Result<(), String> {
1193:    pub fn trash(&mut self, ids: &[PhotoId], permanent: bool) -> Result<String, String> {
1208:                    .map_err(|e| format!("{} could not be moved to the Trash: {e}", photo.name))?;
1218:                format!("Moved {} to Trash", crate::model::compact_count(photos.len() as u64))
1221:            body: Body::Trashed { ids: ids.to_vec(), original },
1223:        Ok(if permanent { "Deleted".to_owned() } else { "Moved to Trash".to_owned() })
1230:    pub fn toast(&mut self, text: impl Into<String>, kind: ToastKind, verb: Option<&str>) -> u32 {
1248:    pub fn take_pending(&mut self) -> Option<Task<Core>> {
1272:pub fn subscription(_state: &State) -> iced::Subscription<Msg> {
1350:pub fn state_mods(modifiers: iced::keyboard::Modifiers) -> Modifiers {
1360:pub fn update(state: &mut State, message: Msg) -> Option<Core> {
1364:fn reduce(state: &mut State, message: Msg) -> Option<Core> {
1387:fn reduce_inner(state: &mut State, message: Msg) -> Option<Core> {
1407:        Msg::CollectionFilter(index) => {
1434:        Msg::EmptyTrash => {
1437:            state.pending = Some(Pending::EmptyTrash);
1438:            state.dialog = Some(crate::view::dialog::Request::EmptyTrash { count });
1624:        Msg::Rotate(step) => state.rotate(step),
1712:        Msg::Album(command) => {
1718:        Msg::TrashCmd(ids, command) => {
1719:            let result = state.trash(&ids, matches!(command, TrashCmd::Permanent));
1748:                            format!("The Trash could not be read: {reason}"),
1821:fn start_drag(state: &mut State, index: usize) {
1830:/// multi-selection keeps the whole selection, so *Move to Trash* and *Add to
1832:fn context_menu(state: &mut State, id: PhotoId) {
1841:fn rotation_orientation(rotation: viewer::Rotation) -> u8 {
1851:pub fn cell_of(state: &State, id: PhotoId) -> Option<Cell> {
1858:pub fn cell_of_index(state: &State, index: usize) -> Cell {
1867:fn perform(state: &mut State, pending: Pending) {
1869:        Pending::Trash { ids, permanent } => {
1870:            let _ = update(state, Msg::TrashCmd(ids, if permanent { TrashCmd::Permanent } else { TrashCmd::Trash }));
1891:        Pending::EmptyTrash => {
1893:            let _ = update(state, Msg::TrashCmd(ids, TrashCmd::Permanent));
1899:fn reindex(state: &mut State) {
1917:fn relink(state: &mut State, id: PhotoId, path: PathBuf) {
1930:fn apply_edit(state: &mut State, edit: Edit) -> Result<(), String> {
1945:fn move_photos(state: &mut State, destination: PathBuf, mode: MoveMode) {
1993:    pub fn item_count(&self) -> String {
2005:    pub fn reframe(&mut self) {
2014:    pub fn refresh_shell(&mut self) {
2024:pub fn held_bytes(state: &State) -> u64 {
2028:pub fn unique(path: PathBuf) -> PathBuf {
2044:fn export(state: &mut State, spec: &crate::msg::ExportSpec) -> Result<(), String> {
2073:fn run(state: &mut State, command: Command) {
2136:        Command::Trash | Command::DeletePermanent => {
2146:                state.pending = Some(Pending::Trash { ids, permanent: true });
2149:                let _ = reduce(state, Msg::TrashCmd(ids, TrashCmd::Trash));
===STATE-REDUCER-HEAD===
//! The selection, the state, and the reducer.
//!
//! §17.1's law: **the model decides, the painter paints, tests read the model.**
//! Nothing here renders a pixel, and every test in this crate drives this reducer
//! without a window.

//! The selection model, and the nine ways it changes.
//!
//! **`anchor` is an id, never an index.** Delete item 3 of 50,000 and an index
//! anchor makes the next `Shift+Click` select a range offset by one, silently,
//! forever. `order` is separate from `held` because a `HashSet` has no order and
//! "export in the order I clicked them" is a real user need.

use crate::model::cells::Cell;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum SelectionSource {
    #[default]
    User,
    Marquee,
    Find,
    Filter,
}

/// How a marquee mer
```

### TOOL: shell [seq 312] 2026-11-09 13:49:15 status=completed

```json
{
  "command": "echo ===STATE-EDIT===\ngrep -n \"Edit\\|Crop\\|crop\\|Rotate\\|rotate\\|Adjust\\|adjust\\|history\\|Undo\\|undo\\|Rating\\|rating\" /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/msg.rs | head -n 80\necho ===STATE-EDIT-FNS===\nsed -n '1930,2010p' /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/state.rs\necho ===MSG-EDIT===\ngrep -n \"Edit\\|Crop\\|Rotate\\|Adjust\" /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/state.rs | head -n 40\necho ===GPUI-LIBRARY===\nhead -n 120 /home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/src/library.rs\nwc -l -c /home/lufuno/smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/src/library.rs"
}
```

```
===STATE-EDIT===
6://! implementation of *Rotate Right* in the app.
160:    RotateCw,
161:    RotateCcw,
165:    Crop,
166:    Adjust,
167:    Edit,
168:    ResetEdits,
187:    ClearRating,
203:    Undo,
217:    Edit,
227:        Group::Edit,
238:            Group::Edit => "EDIT",
249:/// selection, because a rating applied to a mixed set has no meaning.
344:    e(Command::Rate, "rate", "Set rating", "star", Some("hold"), Some(Group::Organise), Uniform, &[Shortcut::ctrl("0", "Ctrl+0"), Shortcut::ctrl("5", "Ctrl+5")], Holding),
352:    e(Command::RotateCw, "rotate-cw", "Rotate right 90", "rotate-cw", Some("hold"), Some(Group::Edit), Image, &[Shortcut::key("r", "r")], Holding),
353:    e(Command::RotateCcw, "rotate-ccw", "Rotate left 90", "rotate-ccw", None, Some(Group::Edit), Image, &[Shortcut::shift("r", "Shift+r")], NoBar),
354:    e(Command::FlipH, "flip-h", "Flip horizontal", "flip-horizontal", None, Some(Group::Edit), Image, &[Shortcut::shift("h", "Shift+h")], NoBar),
355:    e(Command::FlipV, "flip-v", "Flip vertical", "flip-vertical", None, Some(Group::Edit), Image, &[Shortcut::shift("v", "Shift+v")], NoBar),
356:    e(Command::ApplyRotation, "apply-rotation", "Apply rotation to file", "file-check", None, Some(Group::Edit), Touches, &[], NoBar),
357:    e(Command::Crop, "crop", "Crop", "crop", None, Some(Group::Edit), Image, &[Shortcut::ctrl_shift("x", "Ctrl+Shift+x")], NoBar),
358:    e(Command::Adjust, "adjust", "Adjust", "sliders-horizontal", None, Some(Group::Edit), Image, &[Shortcut::ctrl_alt("a", "Ctrl+Alt+A")], NoBar),
359:    e(Command::Edit, "edit", "Edit in photo editor", "wand-sparkles", Some("hold"), Some(Group::Edit), Image, &[Shortcut::ctrl("e", "Ctrl+e")], Holding),
360:    e(Command::ResetEdits, "reset-edits", "Reset all edits", "rotate-ccw", None, Some(Group::Edit), Image, &[], NoBar),
361:    e(Command::ClearMetadata, "clear-metadata", "Clear metadata", "file-x", None, Some(Group::Edit), Touches, &[], NoBar),
382:    e(Command::ClearRating, "clear-rating", "Clear rating", "star-off", None, Some(Group::Delete), Nothing, &[], NoBar),
390:    e(Command::ZoomReset, "zoom-reset", "Reset zoom", "rotate-ccw", None, None, Viewer, &[Shortcut::key("0", "0")], NoBar),
401:    e(Command::Undo, "undo", "Undo", "undo", None, None, Nothing, &[Shortcut::ctrl("z", "Ctrl+z")], NoBar),
405:    e(Command::Rescan, "rescan", "Rescan source", "rotate-cw", None, None, Nothing, &[Shortcut::ctrl("r", "Ctrl+r")], NoBar),
410:/// through here, so there is one implementation of *Rotate Right*.
482:    /// To the freedesktop trash. Undoable, so it never asks.
489:pub enum Edit {
606:    Rotate(i8),
622:    Apply(Edit),
635:    UndoRequested,
788:            Core::ToastAction(_) => Msg::UndoRequested,
938:        Command::Edit => 80,
939:        Command::RotateCw => 80,
1051:/// A rating is never a filter's only condition except in the collections where
1052:/// it is the point, and a crop cannot be applied to a multi-selection because
1053:/// one crop rect across three photographs is never what anyone wanted.
===STATE-EDIT-FNS===
fn apply_edit(state: &mut State, edit: Edit) -> Result<(), String> {
    match edit {
        Edit::Tag { id, tag } => state.write_sidecar(id, |sidecar| {
            if !sidecar.tags.contains(&tag) {
                sidecar.tags.push(tag.clone());
            }
        }),
        Edit::Caption { id, text } => {
            state.write_sidecar(id, |sidecar| sidecar.description = Some(text.clone()))
        }
        Edit::Title { id, text } => state.write_sidecar(id, |sidecar| sidecar.title = Some(text.clone())),
        Edit::People { id, names } => state.write_sidecar(id, |sidecar| sidecar.people = names.clone()),
    }
}

fn move_photos(state: &mut State, destination: PathBuf, mode: MoveMode) {
    let ids = state.selection.order.clone();
    let photos: Vec<Photo> = state
        .photos()
        .iter()
        .filter(|photo| ids.contains(&photo.id))
        .cloned()
        .collect();
    let mut moved = 0;
    for photo in photos {
        let Some(name) = photo.path.file_name() else { continue };
        let target = destination.join(name);
        if target == photo.path {
            continue;
        }
        // A collision appends ` (2)` and the toast names the count, because a
        // photo app that quietly overwrites is a photo app that loses.
        let target = unique(target);
        let result = match mode {
            MoveMode::Move => std::fs::rename(&photo.path, &target),
            MoveMode::Copy => std::fs::copy(&photo.path, &target).map(|_| ()),
        };
        if result.is_ok() {
            moved += 1;
            if let Some(record) = state.library.value_mut() {
                if let Some(found) = record.photos.iter_mut().find(|found| found.id == photo.id) {
                    found.path = target;
                }
            }
        }
    }
    if moved > 0 {
        state.toast(
            format!(
                "{moved} {} {destination:?}",
                if matches!(mode, MoveMode::Move) { "moved to" } else { "copied to" }
            ),
            ToastKind::Undo,
            Some("Undo"),
        );
    }
}

impl State {
    /// How many items the list is showing, as a person reads it, and it is the
    /// right-hand end of the content row. A row that says `4120` is a row nobody
    /// scans.
    #[must_use]
    pub fn item_count(&self) -> String {
        crate::model::compact_count(self.visible.len() as u64)
    }

    /// **Recompute everything that is a function of the frame**, and nothing that
    /// is a function of the library. Called once per message, after `rebuild`.
    ///
    /// The old `reframe()` was a no-op: it read `state.frame.width` and passed it
    /// back into `Frame::at` alongside the same height, the same sidebar flag and
    /// the same density — a function that could not change anything and was called
    /// from two places as if it could. This one moves the pane's boundary and
    /// re-derives the content measure, which is a real change at a real width.
    pub fn reframe(&mut self) {
        self.frame.refresh();
    }

    /// The bar's spec, its lines and its folded height.
    ///
===MSG-EDIT===
295:    Editor,
297:    TagEditor,
382:    Command, Direction, Edit, Modifiers, MoveMode, Scope, Shortcut, Msg,
1624:        Msg::Rotate(step) => state.rotate(step),
1930:fn apply_edit(state: &mut State, edit: Edit) -> Result<(), String> {
1932:        Edit::Tag { id, tag } => state.write_sidecar(id, |sidecar| {
1937:        Edit::Caption { id, text } => {
1940:        Edit::Title { id, text } => state.write_sidecar(id, |sidecar| sidecar.title = Some(text.clone())),
1941:        Edit::People { id, names } => state.write_sidecar(id, |sidecar| sidecar.people = names.clone()),
===GPUI-LIBRARY===
//! Finding photos on disk. Filesystem access lives here and nowhere else, so the view can
//! stay declarative and this can be tested against real directories.

use crate::model::{Library, Photo, is_image};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Reads the header to get a width and height. Only the header is parsed, not the pixels —
/// GPUI does the decoding when it paints. A file that cannot be read still gets listed; it
/// just cannot be fitted to the window.
fn dimensions(path: &Path) -> Option<(u32, u32)> {
    image::image_dimensions(path).ok()
}

/// Every image directly inside `dir`. Non-recursive, so a folder of 50,000 files does not
/// stall the UI. Unreadable files are skipped rather than aborting the whole listing.
pub fn scan(dir: &Path) -> Library {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Library::default();
    };

    let mut photos: Vec<Photo> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // `is_dir` first: a directory called `backup.png` is not a photo.
            let metadata = std::fs::metadata(&path
```

### REASONING [seq 319] 2026-11-09 13:49:19

Comparing egui Photos and naledzi iced smart-photos implementations, noting feature gaps and caching design tradeoffs.

### REASONING [seq 319] 2026-11-09 13:49:19

Identifying unimplemented pixel editing and face detection versus implemented metadata handling and thumbnail caching.

### TOOL: shell [seq 319] 2026-11-09 13:49:19 status=completed

```json
{
  "command": "grep -n \"confirm\\|destructive\\|typed\\|Danger\\|Ask before\" /home/lufuno/crates/smart-ui-egui/src/modals.rs | head -n 30\necho ===PALETTE===\ngrep -n \"Palette\\|palette\\|Ctrl+K\\|command\" /home/lufuno/crates/smart-ui-egui/src/shell.rs | head -n 25\necho ===EIGA-IMPORT===\ngrep -n \"import\\|Import\\|removable\\|device\" /home/lufuno/smart/naledzi_ui/examples/smart-photos/src/state.rs | head -n 25\necho ===PROTO-BACKUP===\ngrep -n \"interface BackupService\\|createBackup\\|restoreBackup\\|syncToRemote\" /home/lufuno/smart/smart-protocol-rs/schema/backup.capnp | head -n 15\necho ===PROTO-CAMERA-MEDIA===\ngrep -n \"detectFaces\\|detectScene\\|capturePhoto\\|getRichMetadata\\|HdrOptions\\|aiTags\\|contentHash\" /home/lufuno/smart/smart-protocol-rs/schema/camera.capnp /home/lufuno/smart/smart-protocol-rs/schema/media.capnp | head -n 25\necho ===OWN-PHOTOS-SETTINGS-PAGE===\ngrep -n \"photos_settings\\|Photos\" /home/lufuno/crates/smart-ui-egui/src/lib.rs | head -n 20"
}
```

```
213:                Modal::Confirm(c) => self.m_confirm(ui, card, c, &mut close, &mut acts),
757:    fn m_confirm(&mut self, ui: &mut Ui, card: Rect, cf: &Confirm, close: &mut bool, acts: &mut Vec<Act>) {
1330:                    self.setting_row(ui, body, y, "trash-bin-minimalistic", "Ask before deleting", "Show a confirmation for permanent deletes", Act::ToggleConfirmDelete, Some(st.confirm_delete), "confirm");
===PALETTE===
156:    /// Window the palette should centre itself in.
157:    pub fn palette_win(&self) -> Rect {
158:        match self.palette.app {
602:                        mi("command", "Command palette", Act::Seq(vec![Act::OpenApp(AppId::Files), Act::Palette])).k("Ctrl+K").gap(),
731:            Palette,
737:        let items = [Di::App(AppId::Files), Di::App(AppId::Photos), Di::App(AppId::Terminal), Di::App(AppId::Video), Di::Sep, Di::Settings, Di::Palette];
785:                _ => ("Command palette", "command", hex("#8f7cf5"), hex("#5b4fd6"), false, None),
838:            Some(Di::Palette) => {
840:                self.acts.push(Act::Palette);
===EIGA-IMPORT===
258:    pub removable: bool,
265:        Self { read_only: true, removable: false, watched: false }
269:    pub const fn removable() -> Self {
270:        Self { read_only: true, removable: true, watched: true }
276:        if self.removable {
277:            "This device is read-only."
610:            removable: false,
908:    pub fn has_device(&self) -> bool {
909:        self.sources.iter().any(|source| source.removable)
2336:/// The one non-removable source at `/photos`, for a state that has a library.
2343:        removable: false,
2395:/// Whether the frame is a phone frame, which is a width and not a device.
2485:pub fn add_source(state: &mut State, path: PathBuf, id: u16, removable: bool) {
2487:    state.sources.push(Source { id, path, watch: false, removable, available, last_count: None });
===PROTO-BACKUP===
28:#     `SyncState` (now returned by `syncToRemote`).
47:interface BackupService {
55:    createBackup @2 (options :BackupOptions) -> (result :CreateBackupResult);
58:    restoreBackup @3 (backupId :T.BackupId, options :RestoreOptions) -> (result :RestoreBackupResult);
80:    syncToRemote @10 (backupId :T.BackupId, remoteTarget :RemoteTarget) -> (result :BackupSyncStateResult);
===PROTO-CAMERA-MEDIA===
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:28:#   - HdrOptions: bracket capture with merge strategy.
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:45:    capturePhoto @2 (cameraId :T.CameraId, options :PhotoOptions) -> (result :PhotoCaptureResult);
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:71:    detectFaces @16 (cameraId :T.CameraId) -> (faces :List(FaceDetection));
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:72:    detectScene @17 (cameraId :T.CameraId) -> (scene :SceneDetection);
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:95:    getRichMetadata @28 (captureId :T.CaptureId) -> (metadata :RichPhotoMetadata);
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:204:    hdr @8 :HdrOptions;
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:263:    hdr @7 :HdrOptions;
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:360:struct HdrOptions {
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:559:    aiTags @5 :List(Text);
/home/lufuno/smart/smart-protocol-rs/schema/media.capnp:23:#   - Content-addressed media via contentHash on MediaItem.
/home/lufuno/smart/smart-protocol-rs/schema/media.capnp:208:    contentHash @12 :Data;
/home/lufuno/smart/smart-protocol-rs/schema/media.capnp:210:    aiTags @14 :T.StringMap;
/home/lufuno/smart/smart-protocol-rs/schema/media.capnp:222:    contentHash @3 :Data;
===OWN-PHOTOS-SETTINGS-PAGE===
185:        // the focused suite app (Terminal / Photos) owns the keyboard
400:        "photos" => a.open_app(AppId::Photos),
408:                "photos" => a.open_app(AppId::Photos),
415:            let app = if arg == "terminal" { AppId::Terminal } else { AppId::Photos };
420:            a.open_app(AppId::Photos);
440:            a.open_app(AppId::Photos);

```

### REASONING [seq 330] 2026-11-09 13:49:37

Structuring a non-destructive PHOTOS-SPECS spec with reality matrix, lie audit, merge plan, perf budget, and protocol-first architecture.

### REASONING [seq 330] 2026-11-09 13:49:37

Expanding the spec with cache budget reconciliation, data model, UX, and acceptance criteria.

### REASONING [seq 330] 2026-11-09 13:49:37

Detailing cache key design, per-file component selection, MTP support gaps, and accessibility and persistence requirements.

### TOOL: shell [seq 330] 2026-11-09 13:49:37 status=completed

```json
{
  "command": "grep -n \"photos\\|Photos\" /home/lufuno/crates/smart-ui-egui/src/lib.rs | head -n 12\ngrep -n \"pub mod\\|mod \" /home/lufuno/crates/smart-ui-egui/src/lib.rs | head -n 30\necho ===BE-HOME===\nsed -n '138,180p' /home/lufuno/crates/smart-core/src/backend.rs\necho ===EIGA-DROP===\nsed -n '563,605p' /home/lufuno/crates/smart-ui-egui/src/video.rs\necho ===VERIFY-NOEXIF===\ngrep -c -i \"exif\\|orientation\" /home/lufuno/crates/smart-ui-egui/src/photos.rs; grep -n -i \"exif\\|orientation\" /home/lufuno/crates/smart-ui-egui/src/photos.rs | head\necho ===VERIFY-FAVS-PERSIST===\ngrep -rn \"favs\" /home/lufuno/crates/smart-core/src/library.rs | head; echo \"library has favs? (empty=not persisted) OK\""
}
```

```
13:pub mod photos;
185:        // the focused suite app (Terminal / Photos) owns the keyboard
384:/// `#terminal=ls;neofetch`, `#photos`, `#photo` / `#photo=edit` / `#photo=info`, `#menu` — open the suite in a given state
400:        "photos" => a.open_app(AppId::Photos),
408:                "photos" => a.open_app(AppId::Photos),
415:            let app = if arg == "terminal" { AppId::Terminal } else { AppId::Photos };
420:            a.open_app(AppId::Photos);
422:                if let Some(l) = a.photos_first() {
423:                    a.photos_show(l);
425:                let items = a.photos_viewer_menu();
428:                let items = a.photos_library_menu();
440:            a.open_app(AppId::Photos);
2:pub mod app;
3:pub mod appkit;
4:pub mod chrome;
5:pub mod content;
6:pub mod copyfx;
7:pub mod icons;
8:pub mod highlight;
9:pub mod icons_gen;
10:pub mod modals;
11:pub mod overlays;
12:pub mod paint;
13:pub mod photos;
14:pub mod pty;
15:pub mod storage;
16:pub mod video;
17:pub mod shell;
18:pub mod terminal;
20:pub mod termlive;
21:pub mod theme;
497:mod web {
===BE-HOME===
    pub fn home(&self) -> Location {
        #[cfg(feature = "demo")]
        if self.demo {
            return crate::demo::home();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(h) = dirs::home_dir() {
            return crate::local::path_to_loc(&h);
        }
        Location::new("mem", "/")
    }

    pub fn places(&self) -> Vec<Place> {
        let mk = |id: &str, title: &str, icon: &'static str, loc: Location| Place { id: id.into(), title: title.into(), icon, loc };
        let home = self.home();
        let mut v = vec![mk("home", "Home", "home", home.clone())];
        #[cfg(not(target_arch = "wasm32"))]
        if !self.demo {
            let add = |v: &mut Vec<Place>, id: &str, t: &str, i: &'static str, p: Option<std::path::PathBuf>| {
                if let Some(p) = p.filter(|p| p.is_dir()) {
                    v.push(mk(id, t, i, crate::local::path_to_loc(&p)));
                }
            };
            add(&mut v, "desktop", "Desktop", "monitor", dirs::desktop_dir());
            add(&mut v, "documents", "Documents", "document-text", dirs::document_dir());
            add(&mut v, "downloads", "Downloads", "download-minimalistic", dirs::download_dir());
            add(&mut v, "pictures", "Pictures", "gallery", dirs::picture_dir());
            add(&mut v, "music", "Music", "music-note", dirs::audio_dir());
            add(&mut v, "movies", "Movies", "clapperboard", dirs::video_dir());
            return v;
        }
        for (id, t, i) in [("desktop", "Desktop", "monitor"), ("documents", "Documents", "document-text"), ("downloads", "Downloads", "download-minimalistic"), ("pictures", "Pictures", "gallery"), ("music", "Music", "music-note"), ("movies", "Movies", "clapperboard")] {
            v.push(mk(id, t, i, home.join(t)));
        }
        v
    }

    // ───────── browsing ─────────
    pub fn list(&self, loc: &Location) -> u64 {
        let (id, reg, tx, loc) = (self.id(), self.registry.clone(), self.tx.clone(), loc.clone());
        spawn(move || {
            let result = match reg.get(&loc.fs) {
                Some(v) => v.list(&loc.path).map_err(|e| e.to_string()),
===EIGA-DROP===
    fn eiga_drop(&mut self, files: Vec<egui::DroppedFileHandle>) {
        let t = self.time;
        let mut videos: Vec<Location> = vec![];
        let mut subs_added = 0usize;
        for f in &files {
            let ext = f.path().extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            let is_video = ["mp4", "mkv", "mov", "webm", "avi", "m4v", "ts"].contains(&ext.as_str());
            let is_sub = matches!(ext.as_str(), "srt" | "vtt" | "ass" | "ssa");
            #[cfg(not(target_arch = "wasm32"))]
            if is_video {
                videos.push(Location::new("file", f.path().to_string_lossy().to_string()));
            }
            #[cfg(not(target_arch = "wasm32"))]
            if is_sub {
                if let Ok(b) = f.bytes() {
                    let tr = SubTrack::from_bytes(&f.path().file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), &b);
                    if !tr.is_empty() {
                        self.shell.eiga.sub_i = Some(self.shell.eiga.subs.len());
                        self.shell.eiga.subs.push(tr);
                        subs_added += 1;
                    }
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            if !files.is_empty() {
                self.shell.eiga.osd = Some(("File drops open real videos in the desktop app".into(), t));
                return;
            }
        }
        if !videos.is_empty() {
            let l = videos[0].clone();
            let nm = l.name();
            self.eiga_open(l);
            self.shell.eiga.osd = Some((format!("Opened {nm}"), t));
        } else if subs_added > 0 {
            self.shell.eiga.osd = Some((format!("{} subtitle track(s) added", subs_added), t));
        }
    }

    fn eiga_pick(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
===VERIFY-NOEXIF===
0
===VERIFY-FAVS-PERSIST===
library has favs? (empty=not persisted) OK

```

### TOOL: write [seq 343] 2026-11-09 13:49:55 status=completed

```json
{
  "content": "# PHOTOS-SPECS — Canonical Specification for Smart Photos\n\n> **Status:** Canonical. The single source of truth for what Smart Photos is, what it must do,\n> how it must behave, and how it must be proven. Every claim in this document is testable, and\n> every claim below is traceable to a real file, a real line, or a real measurement.\n>\n> **Scope:** Library, Viewer, Filmstrip, Editor, Capture, Picker, Search, Organisation,\n> Places, Import, Trash, Sync, Backup, Settings, Accessibility — the whole app.\n>\n> **Read order:** §0 Mandate → §1 What is true today → §2 Lie hunt → §3 Per-file best →\n> §4 Performance & memory → §5 Protocol-first → §6 New ideas → §7 Editor → §8 Data →\n> §9 UX (a11y, drag-drop, picker, trash, dialogs, settings) → §10 Integration →\n> §11 Acceptance → Appendices (command index, token index, research).\n\n---\n\n## Table of contents\n\n- [0. Mandate](#0-mandate)\n- [1. What is true today (reality matrix)](#1-what-is-true-today-reality-matrix)\n- [2. Lie hunt](#2-lie-hunt)\n- [3. Per-file best (merge plan)](#3-per-file-best-merge-plan)\n- [4. Performance & memory budget](#4-performance--memory-budget)\n- [5. Protocol-first architecture](#5-protocol-first-architecture)\n- [6. New ideas — the unknown unknowns](#6-new-ideas--the-unknown-unknowns)\n- [7. Editor (non-destructive, sidecar-backed, reversible)](#7-editor-non-destructive-sidecar-backed-reversible)\n- [8. Data model, index, XMP, tombstones](#8-data-model-index-xmp-tombstones)\n- [9. UX — accessibility, drag-drop, picker, trash, dialogs, settings](#9-ux--accessibility-drag-drop-picker-trash-dialogs-settings)\n- [10. Integration](#10-integration)\n- [11. Acceptance](#11-acceptance)\n- [Appendix A — Command index](#appendix-a--command-index)\n- [Appendix B — Token index](#appendix-b--token-index)\n- [Appendix C — Research sources](#appendix-c--research-sources)\n\n---\n\n## 0. Mandate\n\n### 0.1 What this is\n\nSmart Photos is a daily-driver photo manager for the folders the user already keeps. It is in\nthe Smart suite as `AppId::Photos` (`crates/smart-ui-egui/src/shell.rs`), shares the suite's\ndepth primitives, icons, theme, and appkit settings chrome with Files, and hands back\n`Location`s — never private paths — to whatever summoned it.\n\n### 0.2 What this is not\n\n- **Not** a copy-library app. Photos never copies a library the user gave it a path to.\n- **Not** an account app. No network connection is ever opened to look at a folder.\n- **Not** a native-control app. No stock egui `button`/`slider`/`text_edit` rendering where the\n  suite has a drawn primitive; every control is drawn with the suite's depth primitives.\n- **Not** a destructive editor. The image's bytes are never overwritten by an adjustment.\n\n### 0.3 Non-negotiable product laws\n\n| # | Law | In one sentence |\n|---|-----|-----------------|\n| L1 | **Source is truth** | The filesystem the user edits in a file manager is the library. Everything else is a cache that can be deleted. |\n| L2 | **No file is ever lost** | Delete is Trash. Permanent delete is typed-confirmed. Both are undoable. |\n| L3 | **The image is never touched** | Edits write a sidecar. Pixels are baked only on Export. |\n| L4 | **No silent black** | An undecodable file is a declared state with a retry, never a black tile. |\n| L5 | **No fake metadata** | Every field in Info is either read from the file/sidecar or labelled with its source. |\n| L6 | **One implementation per command** | The bar, the menu, the key, and the palette all dispatch the same `Command`. |\n| L7 | **Protocol-first** | The app depends on the protocol, never on a backend. Backends are disposable. |\n| L8 | **Reversible AI** | Every AI grouping, suggestion, and auto-tag can be undone to exactly the pre-AI state. |\n| L9 | **Honest degrade** | A missing capability (decoder, ffmpeg, MTP) is a stated reason, not a silent gap. |\n| L10 | **Bounded everything** | Queues, caches, indexes, and memory are bounded. A 50,000-photo flick is a normal frame. |\n\n### 0.4 The one-sentence test\n\n> *If the user can lose a photo, lose metadata, or be lied to about what they are looking at,\n> this spec has failed.*\n\n---\n\n## 1. What is true today (reality matrix)\n\n### 1.1 Sources audited (exact, non-destructive)\n\nWork was done non-destructively under `/tmp/opencode/canonical-photos-audit/`. The canonical\ncheckouts are at `/home/lufuno/crates/` (byte-identical to\n`/home/lufuno/Documents/rust-wasm-media-manager-latest/` for the files below).\n\n| # | File | Size | Lines | MD5 (canonical) | Variant notes |\n|---|------|------|-------|-----------------|---------------|\n| F1 | `crates/smart-ui-egui/src/photos.rs` | 74978 | 1467 | `8d7738deb67839512a5b18093b9d48db` | Identical in all 3 file-manager zips (`improve-app-consistency-and-completeness.zip`, `build-production-rust-file-manager-2-very-good.zip`, `develop-rust-wasm-file-manager.zip`) and in `/home/lufuno/crates`. |\n| F2 | `smart-desktop/apps/photos/mod.rs` (bad2) | 75586 | 1476 | `19415a53d0002d65f3ed98ad4fadfc0c` | F1 + `Ask Smart` button + spring clamp + `pub(crate)` visibility (see §3). |\n| F3 | `crates/smart-core/src/library.rs` | 6223 | 192 | `18e59884bb1458df96c17dbc24c94b2c` | Identical in a1 / wasm / desktop. Very-good zip is 7176 (adds 6 settings fields — see §3). |\n| F4 | `crates/smart-core/src/thumbs.rs` | 3184 | 91 | `cfd4f201cbbc603bd0c44d0338f11acd` | Base in a1/wasm/desktop/egui. Very-good zip is 5183 (adds ffmpeg `video_thumb` + disk cache — see §3). |\n| F5 | `crates/smart-core/src/analyze.rs` | 31515 | 746 | `6336b4af39123b640f0ddb7968641974` | Storage scanner, tested. Used by `storage.rs`, not by Photos. |\n| F6 | `crates/smart-ui-egui/src/storage.rs` | 54375 | 1039 | `963f9d660ebf7cef107c1b80442b44ba` | Files > Storage workspace (Overview/Cleanup/Map/Graph/Biggest). |\n| F7 | `crates/smart-ui-egui/src/highlight.rs` | 13912 | 300 | `d0f77f332751fc6993d052fe3379de70` | Tree-sitter code highlighting + fallback lexer. Text-preview feature. |\n| F8 | `crates/eiga-core/src/{player,source,subs,lib}.rs` | 30125/30036/14951/2991 | 820/731/401/92 | — | Video engine (Eiga). No still-image thumb cache. |\n| F9 | `smart/naledzi_ui/examples/smart-photos/` (iced) | 444853 total src | 11731 total | — | Reference implementation + 4192-line SPEC.md. The design authority for §4, §7, §8. |\n| F10 | `smart/naledzi_ui/crates/smart-ui-gpui/apps/image-viewer/src/library.rs` | 10403 | 283 | — | Minimal single-folder viewer (GPUI). Baseline for fast viewer. |\n| F11 | `smart-protocol-rs/schema/{files,media,storage,camera,capture,backup,location}.capnp` | — | 832/433/—/622/—/473/318 | — | Canonical protocol. Photos currently depends on none of it (see §5). |\n\n> **Per-file best is decided in §3.** This section states what is true in each file today,\n> without picking a winner.\n\n### 1.2 Reality matrix — Smart Photos (egui build, F1)\n\nVerdict key: **REAL** (works, honest) · **PARTIAL** (works with a material gap) ·\n**LIE** (claims more than it does) · **MISSING** (absent).\n\n| Area | Feature | Verdict | Evidence |\n|------|---------|---------|----------|\n| Library | Scan home tree for `Kind::Image` | **PARTIAL** | Recursive from `be.home()`, budget 6000 entries (`photos.rs:223-248`). Home-only: no Pictures-specific root, no removable/phone sources. No skip of `.cache/target/node_modules` — DFS descends into them. |\n| Library | Sort (Newest/Oldest/Name/Size) | **REAL** | `photos_list()` + `SORTS` (4). Reverse-for-oldest is correct. |\n| Library | Search | **PARTIAL** | Filename `contains()` only (`photos_list`: `e.name.to_lowercase().contains(&q)`). No EXIF/caption/tag/camera search. Ignores the suite's `search.rs` engine (query language + nucleo + content scan) entirely. |\n| Library | Albums | **PARTIAL** | `Album::Folders(f)` = parent directory. No metadata-only albums, no covers, no membership editing. Sidebar lists folders by count. |\n| Library | Favorites | **LIE** | `favs: HashSet<Location>` is session-only in-memory (`photos.rs:66`). Never persisted to `Library` (F3 has no favs field — verified by grep). Restart = all favorites gone. |\n| Library | Tags | **MISSING** (Photos-side) | Info panel *reads* tags from `Library` (`draw_photo_info: tags_of`) but Photos provides no way to add/remove a tag. Tags are Files-only. |\n| Library | Ratings / People | **MISSING** | No star ratings, no People, no captions. |\n| Viewer | Zoom / pan | **PARTIAL** | Spring zoom 0.4–10, pan gated on `zoom>1`, wheel + pinch (`draw_photo_viewer:1072-1102`). Real texture scaling. **But** the base image is capped at 1600 px (`photos_load_cur:500`), so `Actual size` cannot show real pixels (see LIE-1). |\n| Viewer | Zoom-to-cursor | **MISSING** | Wheel zoom changes `zoom` but never anchors to cursor (`pan` is only decayed ×0.8 near fit). The iced model has the exact formula `pan_new = c − (c−pan)·(new/old)` (F9 `model/viewer.rs`). |\n| Viewer | Rotate / flip | **PARTIAL** | `rot/flip_h/flip_v` are view-only UV transforms (`draw_tex`/`uvs`) + baked on save. Real, but view-only (no commit command), and EXIF orientation is never applied on load (grep count 0 for exif/orientation in F1). |\n| Viewer | Filmstrip | **REAL** | Wheel + drag, spring follow, edge fades, context menu, selection outline (`draw_photo_viewer:1163-1277`). |\n| Viewer | Slideshow | **REAL** | Interval 2–10 s (setting), crossfade 0.55 s, progress bar, arrows. Wraps via `photos_nav` `rem_euclid`. |\n| Viewer | Compare (Hold: original) | **REAL** | `compare` toggle shows `cur.orig` (\"ORIGINAL\" badge). Correct, hold-to-compare. |\n| Viewer | Info panel | **LIE** | Histogram (48 bins, luminance) + Name/Dimensions/Size/Kind/Modified/Where/Tags. **No EXIF at all**: no camera, lens, exposure, GPS, orientation, software. Dimensions labelled `(preview)` — honest, but the panel promises \"Info\" and delivers filesystem facts only (see LIE-2). |\n| Editor | Exposure/Contrast/Saturation/Warmth/Vignette (5 sliders) | **REAL** | `process()` CPU per-pixel (exposure `2^e`, contrast, luma-based sat+warmth, smoothstep vignette). Slider double-click resets one field. Real math. |\n| Editor | 8 filter presets with previews | **REAL** | `PRESETS` + 120 px live previews (`photos_load_cur:515-517`). Preset = `Adjust` values. |\n| Editor | Crop | **MISSING** | No crop tool at all. The only \"crop\" in the UI is the thumbnail Fill/Fit *display* mode. |\n| Editor | History / undo / redo | **MISSING** | One-shot `adj/rot/flip` state. `photos_nav`/open resets to default with no record. No operation log. |\n| Editor | Save | **PARTIAL** | `photos_save()` re-runs `process` + rotate90/flip on full-res, encodes JPEG q92 **always**, writes `\"<stem> edited.jpg\"` (collision ` edited 2.jpg`…). Alpha dropped, EXIF stripped, original never overwritten (good). Everything edited to JPEG (see LIE-4). |\n| Capture | Screenshot → Pictures | **REAL** (desktop variant) | `capture/mod.rs` (bad2): Screen/Window/Region + countdown, PNG to `places[pictures]` + clipboard, immediate Photos visibility. Web = canvas stream + download. Native video/audio record is a stub (`ok=true` with no stream — see LIE-5). |\n| Capture | Camera (CameraService) | **MISSING** | No `camera.capnp` `capturePhoto` call. Photos has no capture affordance of its own. |\n| Picker | Wallpaper picker (desktop variant) | **PARTIAL** | `picker.rs` (233 lines): places rail + breadcrumb + current-folder search + reuses `photos_thumb`. No **New Folder**, no recursive search, no multi-select, folders+images only (`!hidden && (is_dir \\|\\| Image)`). |\n| Picker | Generic file picker (open/save) | **MISSING** | `PickPurpose::Wallpaper` is the *only* purpose. No Open/Save, no filter choice. |\n| Import | Phone / removable import | **MISSING + BUG** | See §6.9 (MTP never mounts). `photos_scan` starts at `be.home()`; `adb:/sdcard` and USB volumes are never scanned. |\n| Trash | Delete / restore | **MISSING** | Zero delete affordance in Photos menus 
```

```
Created file successfully: /tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md
```

### TOOL: shell [seq 352] 2026-11-09 13:52:39 status=completed

```json
{
  "command": "wc -l -c /tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md\npython3 - << 'PY'\nimport re, pathlib\np = pathlib.Path(\"/tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md\").read_text()\nprint(\"chars:\", len(p))\nprint(\"tables:\", p.count(\"\\n|\"))\nprint(\"lie refs:\", len(re.findall(r\"LIE-\\d+\", p)))\n# check headers\nfor h in re.findall(r\"^#{1,2} .+$\", p, flags=re.M)[:30]:\n    print(h)\nPY"
}
```

```
 1135 90800 /tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md
chars: 89500
tables: 365
lie refs: 23
# PHOTOS-SPECS — Canonical Specification for Smart Photos
## Table of contents
## 0. Mandate
## 1. What is true today (reality matrix)
## 2. Lie hunt
## 3. Per-file best (merge plan)
## 4. Performance & memory budget
## 5. Protocol-first architecture
## 6. New ideas — the unknown unknowns
## 7. Editor (non-destructive, sidecar-backed, reversible)
## 8. Data model, index, XMP, tombstones
## 9. UX — accessibility, drag-drop, picker, trash, dialogs, settings
## 10. Integration
## 11. Acceptance
## Appendix A — Command index
## Appendix B — Token index
## Appendix C — Research sources

```

### TOOL: shell [seq 359] 2026-11-09 13:52:44 status=completed

```json
{
  "command": "mkdir -p /home/lufuno/smart/smart-suite-official/specs\ncp /tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md\nls -lh /home/lufuno/smart/smart-suite-official/specs/\nmd5sum /tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md\nls -R /tmp/opencode/canonical-photos-audit | head -n 50"
}
```

```
total 104K
-rw-r--r-- 1 lufuno lufuno 12K Nov  9 13:45 CANONICAL-SPECS.md
-rw-r--r-- 1 lufuno lufuno 89K Nov  9 13:52 PHOTOS-SPECS.md
e7f0a405b12e07f927f1d7067bb494b8  /tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md
e7f0a405b12e07f927f1d7067bb494b8  /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md
/tmp/opencode/canonical-photos-audit:
PHOTOS-SPECS.md
home-crates
naledzi-SPEC.md
proc
zip-a1
zip-desktop
zip-desktop2
zip-verygood
zip-wasm

/tmp/opencode/canonical-photos-audit/home-crates:
analyze.home.rs
highlight.home.rs
lib.rs
library.home.rs
photos.home.rs
player.rs
source.rs
storage.home.rs
subs.rs
thumbs.home.rs

/tmp/opencode/canonical-photos-audit/proc:

/tmp/opencode/canonical-photos-audit/zip-a1:
smart-files

/tmp/opencode/canonical-photos-audit/zip-a1/smart-files:
crates

/tmp/opencode/canonical-photos-audit/zip-a1/smart-files/crates:
smart-core
smart-ui-egui

/tmp/opencode/canonical-photos-audit/zip-a1/smart-files/crates/smart-core:
src

/tmp/opencode/canonical-photos-audit/zip-a1/smart-files/crates/smart-core/src:
library.rs
thumbs.rs

/tmp/opencode/canonical-photos-audit/zip-a1/smart-files/crates/smart-ui-egui:
src

/tmp/opencode/canonical-photos-audit/zip-a1/smart-files/crates/smart-ui-egui/src:
photos.rs

/tmp/opencode/canonical-photos-audit/zip-desktop:
smart-desktop

```

### ASSISTANT [seq 368] 2026-11-09 13:52:48

Delivering PHOTOS canonical audit + spec.

**Output (verified copy, identical MD5 `e7f0a405…`):**
- **`/home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md`** (89 KB / 1135 lines) — the deliverable the orchestrator will copy/ship.
- **`/tmp/opencode/canonical-photos-audit/PHOTOS-SPECS.md`** — non-destructive work copy.
- Audit extras in `/tmp/opencode/canonical-photos-audit/`: `zip-a1/`, `zip-verygood/`, `zip-wasm/`, `zip-desktop/`, `zip-desktop2/`, `home-crates/`, `naledzi-SPEC.md`. No sources modified.

**What was verified (all from real reads, not assumed):**
- **Per-file hashes:** `photos.rs` 74978/1467 MD5 `8d7738…` byte-identical across all 3 file-manager zips + `/home/lufuno/crates` (canonical). Desktop bad2 `mod.rs` 75586/1476 differs by exactly 3 deltas: `pub(crate) photos_thumb`, `Ask Smart` button, spring `.clamp(0,1)` — spec takes F1 base + F2 delta as per-file best.
- **Library/thumbs divergence:** base `library.rs` 6223 vs very-good 7176 (+6 serde-back-compat Settings fields — winner: very-good). Base `thumbs.rs` 3184 vs very-good 5183 (+ffmpeg `video_thumb`, path-hash-only key — LIE-7 stale-cache bug; winner: neither, construct with §4.2 key).
- **Lies found (10, with file:line):** fake Actual-size (1600px cap vs menu claim), EXIF-free Info panel, session-only Favorites, JPEG-q92-only save (alpha+EXIF stripped), Capture native record fake-success (`let ok = { true }`), HEIC/AVIF/TIFF advertised but undecodable (`image` crate features = png/jpeg only → silent black tile), ffmpeg cache never expires, 6000-entry truncated scan reported as complete, filename-only search, folders-as-albums.
- **Perf budget specified:** disk LRU 500 MB, big_thumb pool 8, key = `BLAKE3(path+rung+mtime_ns+orientation+edit_generation)`, GPU `clamp(RAM·0.10,128,512MB)`, uploads ≤8/frame, workers `min(8,max(2,cores/2))`, P0–P3 queue, RSS ≤600MB@50k (F9 P1–P7 adopted).
- **Eiga reuse + format honesty**, **protocol mapping** (files/media/storage/camera/backup/location), **new ideas** (duplicate finder with hash256k, offline sync, backup, face/object gated, reversible smart albums/AI grouping, offline Places, Recents tombstones 30d/90d, MTP-never-mounts P0 bug, RAW+HDR, palette vs command-palette rule, print/share, light table), **editor** (sidecar dng/lr, 8 owned XMP elements, 100-deep undo, Export), **UX** (a11y targets/focus, drag-drop from `eiga_drop` pattern, picker + New Folder/search/multi-select, trash + typed `delete`, `SRow/SVal` settings persisted), **acceptance** (A1–A20 + L/N/E tables).

**Unknowns / things not verified:**
- Runtime behaviour on a live profile (no GUI run, no perf measurement here — budgets are targets with tests specified, not measured numbers).
- Whether `/home/lufuno/crates` is the deployment checkout or `Documents/rust-wasm-media-manager-latest` is (both byte-identical for photos.rs; noted in spec §1.1).
- MTP provider availability on this machine (bug verified in code only — no device attached for live test).

Nothing left running. Spec is ready for the orchestrator to point implementation at.

### IDLE [seq 375] 2026-11-09 13:52:55

```json
{
  "time": {
    "created": 1794225175074
  },
  "outcome": "succeeded"
}
```

