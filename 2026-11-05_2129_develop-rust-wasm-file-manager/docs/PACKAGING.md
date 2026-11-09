# Packaging

## AppImage (Linux, x86_64)
```sh
tools/build_appimage.sh                 # fat-LTO release build
PROFILE=dist tools/build_appimage.sh    # thin LTO — for machines with ≤ 2 GB RAM
```
The script builds the binary, assembles an `AppDir` (`AppRun`, `.desktop`, icon, metainfo), bundles `libasound`
(the only library a desktop may lack) and packs it with `appimagetool`. GL/EGL/X11/Wayland/xkb are **not**
bundled: they are `dlopen`ed from the host so *your* GPU driver is used. Built on Debian 12 (glibc 2.36), so it
runs on anything newer.

The host needs **libfuse2** (Ubuntu 22.04+: `sudo apt install libfuse2`) *or* run
`./Smart_Files.AppImage --appimage-extract-and-run`.

### Launch fallback chain (`packaging/AppRun`)
| # | Attempt | Why |
|---|---|---|
| 1 | auto (Wayland if `WAYLAND_DISPLAY`, else X11) | Wayfire, GNOME-Wayland, GNOME-X11 |
| 2 | `WINIT_UNIX_BACKEND=x11` | broken Wayland GL, XWayland |
| 3 | X11 + `SMART_FILES_OPAQUE=1` | X11 session without a compositor (transparent window would be black) |
| 4 | + `LIBGL_ALWAYS_SOFTWARE=1` | no usable GPU driver |
| 5 | Wayland + software | last resort |

Only runs that die within 4 s are retried. Log: `~/.local/state/smart-files/launch.log`.

## Other platforms (same source)
`cargo build --release -p smart-ui-egui` on Windows / macOS / FreeBSD. The file-system layer already branches on
`cfg(windows)` / `cfg(target_os="macos")` for opening files; Trash restore and drive mounting are Linux-first
and degrade to a clear message elsewhere. Installers (MSI, `.app`/dmg, pkg) are a follow-up.

## Web preview
`xtask bundle` → `index.html` (wasm base64-embedded) → `npm run build` → `dist/index.html`.

## Verification
Measured in the build sandbox (2 vCPU / 2 GB, Debian 12, Xvfb 1600×1000, Mesa llvmpipe *software* GL — the worst case):

| Check | Result |
|---|---|
| Idle CPU, window open on a real folder, 10 s | **0 ticks (0 %)** |
| Resident memory, 12 real JPEGs with thumbnails | ≈ 150 MB |
| Real file system, real thumbnails, real disk gauge (22.6 GB) | ✔ |
| Del → real freedesktop Trash → toast **Undo** button → restored | ✔ |
| Del → **Ctrl Z** → restored | ✔ |
| AppImage `--version`, FUSE mount, launch through `AppRun` | ✔ |
| No display → fallback chain logs, exits 1 with a readable message | ✔ |
| Wasm bundle renders in headless Chromium | ✔ |

**A bug found by measuring:** the first build burned ~185 % CPU while idle. Root cause was not egui: the folder
watcher reported *access/metadata* events caused by our own directory listing, which triggered a re-list, which
triggered another event. The watcher now reacts only to create/remove/modify events and coalesces bursts
(150 ms); idle dropped to 0 %. Reactive egui is not wasteful — a feedback loop was.

**Not verified here** (no such session in the sandbox): a live Wayfire or GNOME-X11 session, real removable
drives for mount/unmount (the code path uses `udisksctl`/`gio`), Windows and macOS builds.
