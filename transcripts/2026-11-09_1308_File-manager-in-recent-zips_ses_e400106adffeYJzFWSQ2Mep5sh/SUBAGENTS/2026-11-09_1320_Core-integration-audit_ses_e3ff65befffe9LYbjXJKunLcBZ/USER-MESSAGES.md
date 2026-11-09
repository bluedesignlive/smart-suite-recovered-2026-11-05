# Task prompt(s) — Core integration audit (ses_e3ff65befffe9LYbjXJKunLcBZ)

## 1. [seq 4] 2026-11-09 13:20:19

You are a subagent spawned by another session.
You are the Core/Integration agent for a canonical Rust file manager ($299, non-destructive source work). Source zips in /home/lufuno, last 7 days, ignore hidden. 21 file-manager zips across 7 lineages. Do thorough research and find unknown unknowns the user hasn't thought of. Never modify originals; work in /tmp/opencode/canonical-core-audit/. Do not underestimate: catch every lie/stub, find every integration gap.

Context: canonical base is `smart-files` (smart-core + smart-ui-egui + xtask) which has scripts/build-appimage.sh. eiga-core is a SEPARATE media-player app that was wrongly merged, BUT user now says file manager DOES need some eiga stuff for blazing video thumbnails + robustness, so evaluate which eiga pieces to keep. Phone mount bug: once connected phone never appeared as mounted.

Key zips to audit (paths above):
- build-production-rust-file-manager-2-very-good.zip (26M, 51 rs, backend 27404)
- develop-rust-wasm-file-manager.zip (22M, backend 29461, app.rs 70991)
- improve-app-consistency-and-completeness.zip (19M, terminal 120k, photos 74978)
- rust-wasm-ui-refinements.zip (33 rs, no appkit)
- build-cross-platform-rust-file-manager-good1.zip
- recreate-file-manager-in-rust-2.zip (store 75k, engine 40k, ai, sys, treemap)
- refine-futuristic-desktop-environment-bad2.zip (smart-desktop reference)
- rust-wasm-media-manager-latest.zip (eiga-core + analyze 31515, backend 30667, video/pty/storage/highlight/termlive)

Your focus:
1. CORE COMPLETENESS - Compare smart-core backend.rs/archive.rs/devices.rs/search.rs/thumbs.rs/vfs.rs/model.rs/ops.rs/library.rs/apps.rs/audio.rs/demo.rs/exec.rs/local.rs/memfs.rs/registry.rs/tools.rs + xtask across all versions. Which hash/size is most complete per file? What features only exist in one branch (e.g. archive.rs missing in early, store/engine/ai only in recreate, analyze/eiga only in media)?
2. TRASH INTEGRATION - Find best trash implementation (trash-5.2.9, ops.rs). Is it wired to UI or fake? List gaps for real Trash, restore, empty, permissions.
3. ARCHIVES + FIXES - archive zip+password. Compare archive.rs 18331 across branches, find fixes/bugs. What archives fail to list/extract/create cross-platform?
4. PHONE/MOUNT NEVER APPEARS - Debug devices.rs/vfs.rs/local.rs/memfs/phone VFS, Android/iOS/USB/MTP/Drive. Why does connected phone not mount? Where is detection, polling, permission, WASM limitation, error swallowed? Audit for silent failures/lies. Propose robust mount pipeline with diagnostics, hotplug events, Android iOS limitations surfaced honestly.
5. EIGA THUMBNAILS THAT ARE BLAZING - Audit eiga-core player/source/subs + analyze.rs + video.rs + thumbs pipeline. Propose minimal eiga thumbnail service for file manager (video preview, audio metadata, image thumbs) that is fast (rayon/memmap?), cached, cancellable, without full player UI. Include perf budget.
6. ALL LIES AUDIT - Grep all 21 zips for todo!/unimplemented!/panic!/unwrap spam, fake demo FS, stub functions (e.g. audio.rs 6596, exec.rs 428, vfs 2841?), hardcoded dates/devices, search that doesn't search, copy without progress, etc. Every lie is a $299 risk. List file:line, what it claims vs does, required honest fix.
7. APPIMAGE + PACKAGING + DEEPER INTEGRATIONS - scripts/build-appimage.sh, vite.config.ts, Cargo workspace, wasm vs native gates (cfg(target_arch="wasm32")), AudioWorklet/clap distractions to exclude, file picker integration, notification, xdg/mime, drag-drop OS level. Find unknown packaging/robustness gaps.
8. PERFORMANCE/ROBUSTNESS + UNKNOWN UNKNOWNS - VFS jobs, search indexing, large folders, memory, cancel, error recovery, corruption, permissions, network drives, robustness for sale.

OUTPUT: Write /tmp/opencode/canonical-core-audit/CORE-SPECS.md with: per-file best version table, trash/archive/device/vfs matrix, phone-mount root cause hypothesis + fix spec, eiga thumbnail integration spec, lie inventory with fixes, packaging spec, performance/robustness budget, unknown-unknowns. Also print summary to final answer. Do not merge code yet, non-destructive only.

---

## 2. [seq 265] 2026-11-09 13:24:02

CRITICAL ADDITION from user - prompt1.txt in /home/lufuno/smart/smart-suite/prompt1.txt (incorporate all 11 into your CORE-SPECS.md, P0):

1. AppImages/executables: lies OPENING but doesn't open. No exec-bit via Get Info. Audit exec.rs/backend.rs for real exec/spawn + chmod +x.
2. Window resize cursors missing. Needs OS cursor via hits. Right resize fights scrollbar. Audit window frame/OS integration.
3. NO TRASH + NO destructive dialog. Must implement real trash crate integration (trash-5.2.9) + dialog, case-sensitive protection, reversible.
4. Remove wallpaper/irrelevant settings + Demo 5GB file stuff. Research modern file manager toggles.
5. List row overlapping: checkbox overlaps icon, 3-dots overlaps time. UI overlap bug.
6. Decouple apps from libs, use smart-ui-egui as shared components for ecosystem. Complete Linux backend, desktop notifications/drag-drop/resizing.
7. Smart AI lost composer input styling. All AI destructive must be reversible.
8. Memory 600MB leak + dumb dealloc -> rescan large trees. Traversal expensive. Needs multi-threading + robust crates (rayon, walkdir, notify, memmap?) + smart cache. Audit memory/caching.
9. Reveal/Open in File Manager opens Nautilus not itself. Must be internal.
10. Search small-window 1-char bug: typing .appimage selects last letter only when window small, works when large. Focus/selection bug.
11. Use real mimetypes (Solar), not hardcoded icon map. Audit tree_magic_mini/mime_guess.

Add to your lie inventory and core specs: exec lie, trash missing, resize cursor missing, demo 5GB lie, Nautilus lie, mimetypes lie, 1-char search lie. Include phone-mount, trash, archive, eiga thumbnails, perf already in scope - combine all.

Do not modify originals, still non-destructive in /tmp/opencode/canonical-core-audit/.

---

