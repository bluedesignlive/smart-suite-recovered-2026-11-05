# Task prompt(s) — Smart Photos app audit (ses_e3fde12afffeLACN95001OTzgH)

## 1. [seq 4] 2026-11-09 13:46:51

You are a subagent spawned by another session.
You are the SMART PHOTOS APP agent. Source is really complete if honest — make it real too. Source in /home/lufuno last 7 days, ignore hidden: /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip, /home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip, /home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip for smart-ui-egui/src/photos.rs 74978 (1467 lines), plus /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip smart-desktop/apps/photos/mod.rs 75586, plus smart-core library.rs 6223, thumbs.rs 5183, analyze.rs 31515, storage.rs, highlight.rs. Do non-destructive work to /tmp/opencode/canonical-photos-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md.

Focus:
- Full photos app reality: library, viewer, filmstrip, editor, capture, picker integration. Per-file best, lie hunt (fake zoom, fake editor, fake metadata), performance (thumb cache LRU 500MB, big_thumbs 8, mtime key), memory.
- Eiga thumb pipeline reuse for blazing previews, but for photos: RAW/HEIC/AVIF, EXIF, face detection, object search, smart albums, AI grouping (reversible), editor (crop/rotate/filter/history, all reversible).
- Missing features the user hasn't thought of: find them (unknown unknowns: duplicate finder, offline sync, backup, RAW+ HDR stack, palette, locations/on-map, Recents tombstones, MTP/phone import bug like file manager phone never mounts).
- Protocol-first: depend on smart-protocol-rs domain-storage/domain-media/domain-files, backends disposable (Linux/Android/iOS).
- Accessibility, drag-drop, picker (with New Folder/search), trash integration, destructive dialogs, settings file-manager-only style.
- Output PHOTOS-SPECS.md with matrix, lies, new ideas, perf budget, UX, integration, acceptance. Non-destructive, thorough.

---

