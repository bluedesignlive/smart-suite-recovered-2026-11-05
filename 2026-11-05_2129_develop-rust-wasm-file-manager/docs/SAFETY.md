# Safety & fault tolerance

## Destructive actions
| Action | Behaviour |
|---|---|
| **Del** | moves to the system Trash (`trash` crate, freedesktop spec), shows an **Undo** button for 8 s, and **Ctrl Z** works afterwards |
| **Shift Del** | permanent delete, always behind a confirmation dialog (red button, names the item) — can be switched off in Settings |
| Volumes without a Trash (network, phone) | the same confirmation is forced, with "this can't be undone" |
| Copy / duplicate | undo moves the *new* copies to the Trash (so undo itself is undoable) |
| Rename | undo renames back |
| Name clash | never overwrites: a unique name (`file (2).txt`) is chosen |
| Folder into itself | refused |
| Drop onto Trash | goes through the same Trash path |

## Never glitch
* **Worker panics** — every job runs under `catch_unwind`; a bug becomes a normal *failed job* toast, never a job stuck on "running".
* **UI panics** — each frame is guarded; on a panic the transient state (modal, menu, drag) is reset and the next frame draws cleanly.
* **Crash log** — every panic is appended with a backtrace to `~/.local/state/smart-files/crash.log` (`smart-files --crash-log` prints the path).
* **Release profile uses `panic="unwind"`** (not abort) on native so the above can work; wasm32 must abort.
* **Launcher fallback chain** (AppImage): auto → X11 → X11 opaque → software GL → Wayland software; logged to `launch.log`.
* **No unsafe** in application code. File-system errors are values (`FsResult`), shown as toasts, never unwrapped.
* Cancellation is checked between items and between chunks of a copy, so cancelling is immediate.

## Measured
See the numbers recorded in [PACKAGING.md](PACKAGING.md#verification) (idle CPU of the real binary under Xvfb).
