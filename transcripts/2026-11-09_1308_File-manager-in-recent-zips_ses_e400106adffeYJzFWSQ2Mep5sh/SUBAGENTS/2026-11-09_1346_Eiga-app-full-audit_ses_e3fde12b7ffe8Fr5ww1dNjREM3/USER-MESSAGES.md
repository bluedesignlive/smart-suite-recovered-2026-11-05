# Task prompt(s) — Eiga app full audit (ses_e3fde12b7ffe8Fr5ww1dNjREM3)

## 1. [seq 4] 2026-11-09 13:46:51

You are a subagent spawned by another session.
You are the EIGA APP agent. The user says eiga app needs to be made real too — all its source is done in /home/lufuno (last 7 days, ignore hidden). Source: /home/lufuno/Documents/rust-wasm-media-manager-latest.zip (and -2, .zip) with crates/eiga-core/{lib.rs,player.rs 30125,source.rs 30036,subs.rs 14951}, crates/smart-core/src/analyze.rs 31515, crates/smart-ui-egui/src/{video.rs,highlight.rs,app.rs,chrome.rs,modals.rs}, plus smart-core backend/thumbs/storage. Also check refine-futuristic zips for any eiga-adjacent. Do non-destructive work to /tmp/opencode/canonical-eiga-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md (user said tmp disappears, official is canonical). The spec must be protocol-first (smart-protocol-rs domain-media, domain-video) with backends disposable.

Focus: 
- Full player reality check: does it really play? Check player/source/subs completeness, ffmpeg/ffprobe integration, streaming, playlist, subtitles, OSD, audio pipe, TestCard honesty. Catch all lies (stub audio, fake connect_server, demo progress).
- Like file-manager audits: per-file best version, lie inventory L-01..., performance (memory, rescan, thumb pipeline vs full player speed), robustness, missing features the user hasn't thought of (unknown unknowns: HDR, spatial audio, casting, PiP, chapters, shader effects).
- Video thumbnails already spec'd for file manager, but for eiga itself: frame-accurate seek, 4K60, HDR10, codecs, network streams.
- Integration: protocol media service, clipboard, notifications, MTP/phone library, AppImage packaging, smart-ui-egui reuse.
- AppImage + WASM packaging, no destructive actions without dialog, reversible.
- Output EIGA-SPECS.md with: architecture, per-file matrix, lie list, missing features + new ideas, performance budget, player UX, integration + protocol mapping, acceptance criteria. Non-destructive, thorough.

---

