# Task prompt(s) — Photos app build (ses_e3f8d5678ffehSlBIoBwypYCK4)

## 1. [seq 4] 2026-11-09 15:15:02

You are a subagent spawned by another session.
You are the Smart Photos builder in smart-suite-official, working in parallel with a file-manager hardening agent that owns crates/smart-core, crates/smart-ui-egui, apps/smart-files. DO NOT edit those shared crates or Cargo.toml workspace members except adding your own crate to workspace if needed but not editing others' files. Work only in /home/lufuno/smart/smart-suite-official/apps/smart-photos/ (create it) + spec PHOTOS-SPECS.md at /home/lufuno/smart/smart-suite-official/specs/PHOTOS-SPECS.md (read). Non-destructive, originals untouched. Protocol-first (smart-protocol-rs domain-files/media/storage/camera), backends disposable, 1 sidebar/1 behaviour via smart-ui-egui (read-only import, no hardcoded duplicates). Build: library/viewer/filmstrip/editor (reversible 100-deep undo), capture/picker integration, thumbs LRU 500MB + big_thumb pool, BLake3 key (path+rung+mtime+orientation+edit_generation), RAW/HEIC honest (image crate png/jpeg only -> honest Unable), EXIF, face/AI albums reversible, duplicate finder, MTP import (honest), settings file-manager-only style persisted, drag-drop/paste anywhere (x-special), trash typed delete, accessibility. Research crates, cargo test for photos only. After green cargo check -p smart-photos, init git in smart-suite-official if not git and commit locally with message "photos: scaffold P0 green" (local commit only, no push). Do not edit core/ui crates. Update BUILD-STATUS.md only with a new section for photos (append, don't overwrite file-manager section atomically via edit not write if possible - create PHOTOS-STATUS.md instead to avoid clash).


---

