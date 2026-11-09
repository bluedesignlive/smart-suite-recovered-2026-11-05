# Subagent result — Photos app review

Session `ses_e3f7ec739ffeLRDH9VqQ7WXMsN` | 2026-11-09 15:30:56 → 2026-11-09 15:35:01

## Task

You are a subagent spawned by another session.
You are the reviewer for Smart Photos app in smart-suite-official (like manager app got). Workspace at /home/lufuno/smart/smart-suite-official/. Apps are thin, use smart-ui-egui + smart-core, protocol-first (smart-protocol-rs). You own ONLY apps/smart-photos/ + PHOTOS-STATUS.md (do NOT touch crates/smart-core, crates/smart-ui-egui, apps/smart-files). 
Tasks:
1. Review apps/smart-photos/ for completeness: library/viewer/editor, thumbs LRU 500MB, BLAKE3 key, honest RAW/HEIC, MTP honest, trash typed delete, XMP preserve, EXIF oriented, dup finder, picker new folder/search, settings persisted, a11y.
2. Ensure thumbnail cache logic exists and is not naive: images use in-memory LRU + on-demand 2-thread LIFO + seen dedupe, video uses ffmpeg -ss 0.7 single frame -> disk cache ~/.cache/smart-files/thumbs LRU, mtime in key, cancellable, visible-only (never decode off-screen). If missing, ADD IT (mechanism like thumbs.rs video_thumb pattern, but for photos use BLAKE3 key already). Cache is required — no-cache decode is too slow for old machines / large folders / video. Add if missing.
3. Lie hunt like file-manager: grep todo!/panic!/unwrap, fake zoom, fake editor, silent black tiles, etc. Fix lies or document honestly.
4. cargo check -p smart-photos green, cargo test -p smart-photos 43+ passed, workspace cargo check --workspace (ignore unrelated crates mid-edit but verify photos builds solo). Update PHOTOS-STATUS.md (append Review section, don't overwrite P0 section).
5. After green, git add apps/smart-photos PHOTOS-STATUS.md + specs and commit locally "photos: review + cache fix" (no push).
Non-destructive, originals untouched. File manager hardening is done, don't touch it.


## Final result (last assistant message, seq 545)

**Smart Photos review — DONE, committed locally `2aa5745 "photos: review + cache fix"` (no push).**

## What was wrong / what I fixed

| Finding | Verdict | Fix |
|---|---|---|
| Thumbnail *policy* existed but no *mechanism* (no workers, no 2-thread LIFO, no ffmpeg, private `smart-photos/thumbs` dir, unbounded seen leak) | **S1 cache miss** — task required ADD, not document | **ADDED** in `thumb_cache.rs`: 2-thread LIFO + bounded seen(1024) + ffmpeg `-ss 0.7` single-frame → `~/.cache/smart-files/thumbs` LRU (Files `thumbs.rs` pattern) keyed by full BLAKE3 (mtime always in key), `rebuild_visible` replaces (visible-only), `cancel()` + per-job generation, disk hit-before-decode, no upscale past source, wasm inline `pump_inline`. 7 new tests. |
| 5/10 `Adjust` sliders decorative (Blacks/Whites/Clarity/Texture/Dehaze no-ops) | **S1 fake editor** | All 10 applied + `every_slider_is_live` test. |
| `protocol::export` claimed pixel-bake, was byte-copy; plus `usize::MAX` 0-buf OOM bomb; `process()` panic on short buffers | **S1 lie + S3 crash** | Honest bounded chunked copy ("Copy (bytes)"), 48 MB caps, buffer guards + tests. |
| Picker `query`/`search_everywhere`/`format`/`sort` were state-only, no filter | **S1 fake search** | Real `filter_visible()` + scope/format/hidden/sort tests. |
| `thumbnail()` upscaled tiny sources to rung (blur) | **S2** | Capped at source long-edge + test. |
| Dead `Zoom::step` code, clippy warnings | **S3** | Removed; clippy now 0 warnings. |

Completeness re-checked: library/viewer/editor, 500 MB LRU, BLAKE3 key, RAW/HEIC honest, MTP honest, trash typed `delete`, XMP foreign-preserve, EXIF oriented, dup `hash256k→full`, picker new folder, settings persisted, a11y contract — all DONE. Honest gaps stated (png/jpeg decode only, export byte-copy until bake worker, Face/AI PLANNED, drag-drop PLANNED).

**Proof (real output):**
- `cargo check -p smart-photos` ✅ green, `cargo clippy -p smart-photos` ✅ 0 warnings
- `cargo test -p smart-photos` ✅ **53 passed, 0 failed** (was 43; +10: thumb 7, editor 2, picker 1)
- `cargo check --workspace` ✅ green (warnings only, sibling-owned)
- `grep todo!|panic!` → no matches
- Ran the failing tests iteratively (rebuild accumulated 3≠2, upscale 256≠8) — both fixed in mechanism, not assertions.
- Scope: only `apps/smart-photos/` + `PHOTOS-STATUS.md` (+ `specs/PHOTOS-SPECS.md` added to git) touched; `crates/smart-core`, `crates/smart-ui-egui`, `apps/smart-files` untouched.

---

## Full assistant texts

### [seq 545] 2026-11-09 15:34:57

**Smart Photos review — DONE, committed locally `2aa5745 "photos: review + cache fix"` (no push).**

## What was wrong / what I fixed

| Finding | Verdict | Fix |
|---|---|---|
| Thumbnail *policy* existed but no *mechanism* (no workers, no 2-thread LIFO, no ffmpeg, private `smart-photos/thumbs` dir, unbounded seen leak) | **S1 cache miss** — task required ADD, not document | **ADDED** in `thumb_cache.rs`: 2-thread LIFO + bounded seen(1024) + ffmpeg `-ss 0.7` single-frame → `~/.cache/smart-files/thumbs` LRU (Files `thumbs.rs` pattern) keyed by full BLAKE3 (mtime always in key), `rebuild_visible` replaces (visible-only), `cancel()` + per-job generation, disk hit-before-decode, no upscale past source, wasm inline `pump_inline`. 7 new tests. |
| 5/10 `Adjust` sliders decorative (Blacks/Whites/Clarity/Texture/Dehaze no-ops) | **S1 fake editor** | All 10 applied + `every_slider_is_live` test. |
| `protocol::export` claimed pixel-bake, was byte-copy; plus `usize::MAX` 0-buf OOM bomb; `process()` panic on short buffers | **S1 lie + S3 crash** | Honest bounded chunked copy ("Copy (bytes)"), 48 MB caps, buffer guards + tests. |
| Picker `query`/`search_everywhere`/`format`/`sort` were state-only, no filter | **S1 fake search** | Real `filter_visible()` + scope/format/hidden/sort tests. |
| `thumbnail()` upscaled tiny sources to rung (blur) | **S2** | Capped at source long-edge + test. |
| Dead `Zoom::step` code, clippy warnings | **S3** | Removed; clippy now 0 warnings. |

Completeness re-checked: library/viewer/editor, 500 MB LRU, BLAKE3 key, RAW/HEIC honest, MTP honest, trash typed `delete`, XMP foreign-preserve, EXIF oriented, dup `hash256k→full`, picker new folder, settings persisted, a11y contract — all DONE. Honest gaps stated (png/jpeg decode only, export byte-copy until bake worker, Face/AI PLANNED, drag-drop PLANNED).

**Proof (real output):**
- `cargo check -p smart-photos` ✅ green, `cargo clippy -p smart-photos` ✅ 0 warnings
- `cargo test -p smart-photos` ✅ **53 passed, 0 failed** (was 43; +10: thumb 7, editor 2, picker 1)
- `cargo check --workspace` ✅ green (warnings only, sibling-owned)
- `grep todo!|panic!` → no matches
- Ran the failing tests iteratively (rebuild accumulated 3≠2, upscale 256≠8) — both fixed in mechanism, not assertions.
- Scope: only `apps/smart-photos/` + `PHOTOS-STATUS.md` (+ `specs/PHOTOS-SPECS.md` added to git) touched; `crates/smart-core`, `crates/smart-ui-egui`, `apps/smart-files` untouched.

