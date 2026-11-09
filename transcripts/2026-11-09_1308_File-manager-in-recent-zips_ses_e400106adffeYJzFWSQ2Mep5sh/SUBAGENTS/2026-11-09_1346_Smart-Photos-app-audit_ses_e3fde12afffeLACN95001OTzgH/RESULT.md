# Subagent result — Smart Photos app audit

Session `ses_e3fde12afffeLACN95001OTzgH` | 2026-11-09 13:46:51 → 2026-11-09 13:52:55

## Task

You are a subagent spawned by another session.
You are the SMART PHOTOS APP agent. Source is really complete if honest — make it real too. Source in /home/lufuno last 7 days, ignore hidden: /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip, /home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip, /home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip for smart-ui-egui/src/photos.rs 74978 (1467 lines), plus /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip smart-desktop/apps/photos/mod.rs 75586, plus smart-core library.rs 6223, thumbs.rs 5183, analyze.rs 31515, storage.rs, highlight.rs. Do non-destructive work to /tmp/opencode/canonical-photos-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md.

Focus:
- Full photos app reality: library, viewer, filmstrip, editor, capture, picker integration. Per-file best, lie hunt (fake zoom, fake editor, fake metadata), performance (thumb cache LRU 500MB, big_thumbs 8, mtime key), memory.
- Eiga thumb pipeline reuse for blazing previews, but for photos: RAW/HEIC/AVIF, EXIF, face detection, object search, smart albums, AI grouping (reversible), editor (crop/rotate/filter/history, all reversible).
- Missing features the user hasn't thought of: find them (unknown unknowns: duplicate finder, offline sync, backup, RAW+ HDR stack, palette, locations/on-map, Recents tombstones, MTP/phone import bug like file manager phone never mounts).
- Protocol-first: depend on smart-protocol-rs domain-storage/domain-media/domain-files, backends disposable (Linux/Android/iOS).
- Accessibility, drag-drop, picker (with New Folder/search), trash integration, destructive dialogs, settings file-manager-only style.
- Output PHOTOS-SPECS.md with matrix, lies, new ideas, perf budget, UX, integration, acceptance. Non-destructive, thorough.

## Final result (last assistant message, seq 368)

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

---

## Full assistant texts

### [seq 368] 2026-11-09 13:52:48

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

