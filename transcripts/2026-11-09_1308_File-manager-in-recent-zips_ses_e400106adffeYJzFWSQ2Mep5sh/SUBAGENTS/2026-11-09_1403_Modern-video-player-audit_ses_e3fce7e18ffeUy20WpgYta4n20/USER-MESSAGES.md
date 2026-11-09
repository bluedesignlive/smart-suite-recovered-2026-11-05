# Task prompt(s) — Modern video player audit (ses_e3fce7e18ffeUy20WpgYta4n20)

## 1. [seq 4] 2026-11-09 14:03:52

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


---

## 2. [seq 130] 2026-11-09 14:04:49

CLARIFICATION from user: hypnotic animated caret is whole framework, not terminal-only. All smart-suite apps need it (search, rename, pickers, AI composer, dialogs). Re-read: in VIDEO-SPECS.md, make caret a smart-ui-egui shared component (inherits framework style), not terminal-owned. Preserve breathing/jump hypnotic feel across all inputs. Update spec.

---

