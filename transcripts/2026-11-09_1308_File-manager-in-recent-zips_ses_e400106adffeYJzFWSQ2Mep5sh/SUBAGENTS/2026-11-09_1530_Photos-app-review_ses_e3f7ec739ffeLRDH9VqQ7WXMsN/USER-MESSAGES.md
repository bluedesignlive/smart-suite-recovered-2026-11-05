# Task prompt(s) — Photos app review (ses_e3f7ec739ffeLRDH9VqQ7WXMsN)

## 1. [seq 4] 2026-11-09 15:30:56

You are a subagent spawned by another session.
You are the reviewer for Smart Photos app in smart-suite-official (like manager app got). Workspace at /home/lufuno/smart/smart-suite-official/. Apps are thin, use smart-ui-egui + smart-core, protocol-first (smart-protocol-rs). You own ONLY apps/smart-photos/ + PHOTOS-STATUS.md (do NOT touch crates/smart-core, crates/smart-ui-egui, apps/smart-files). 
Tasks:
1. Review apps/smart-photos/ for completeness: library/viewer/editor, thumbs LRU 500MB, BLAKE3 key, honest RAW/HEIC, MTP honest, trash typed delete, XMP preserve, EXIF oriented, dup finder, picker new folder/search, settings persisted, a11y.
2. Ensure thumbnail cache logic exists and is not naive: images use in-memory LRU + on-demand 2-thread LIFO + seen dedupe, video uses ffmpeg -ss 0.7 single frame -> disk cache ~/.cache/smart-files/thumbs LRU, mtime in key, cancellable, visible-only (never decode off-screen). If missing, ADD IT (mechanism like thumbs.rs video_thumb pattern, but for photos use BLAKE3 key already). Cache is required — no-cache decode is too slow for old machines / large folders / video. Add if missing.
3. Lie hunt like file-manager: grep todo!/panic!/unwrap, fake zoom, fake editor, silent black tiles, etc. Fix lies or document honestly.
4. cargo check -p smart-photos green, cargo test -p smart-photos 43+ passed, workspace cargo check --workspace (ignore unrelated crates mid-edit but verify photos builds solo). Update PHOTOS-STATUS.md (append Review section, don't overwrite P0 section).
5. After green, git add apps/smart-photos PHOTOS-STATUS.md + specs and commit locally "photos: review + cache fix" (no push).
Non-destructive, originals untouched. File manager hardening is done, don't touch it.


---

