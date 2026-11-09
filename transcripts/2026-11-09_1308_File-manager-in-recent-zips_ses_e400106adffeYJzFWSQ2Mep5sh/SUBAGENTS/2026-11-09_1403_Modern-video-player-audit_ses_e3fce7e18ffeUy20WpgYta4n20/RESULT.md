# Subagent result — Modern video player audit

Session `ses_e3fce7e18ffeUy20WpgYta4n20` | 2026-11-09 14:03:52 → 2026-11-09 14:08:29

## Task

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


## Final result (last assistant message, seq 214)

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

---

## Full assistant texts

### [seq 214] 2026-11-09 14:08:23

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

