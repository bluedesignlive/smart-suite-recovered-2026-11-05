# Subagent result — Core integration audit

Session `ses_e3ff65befffe9LYbjXJKunLcBZ` | 2026-11-09 13:20:19 → 2026-11-09 13:26:13

## Task

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

## Final result (last assistant message, seq 370)

Core audit complete — `/tmp/opencode/canonical-core-audit/CORE-SPECS.md` (764 lines, non-destructive, originals untouched).

**21 zips / 7 lineages:** canonical = `rust-wasm-media-manager-latest` (57M, media/eiga superset). Prod line adds CLI archive + `icon_path`/settings/`kind_label` (take). Develop adds notify-coalesce + `restore_trashed` + panic guard + largest `app.rs` 70991 (merge). Recreate line (store 75k/engine 40k/ai/sys/treemap) is an incompatible `&mut self` VFS arch — ideas only. Wasm-refine (33 rs) ships **without `appkit.rs`** and stripped terminal/photos — never ship. Cross-orig has **no `archive.rs`**.

**Best per-file:** `backend` media+develop merge; `archive` prod (CLI fallback for tar/7z/rar + data-descriptor fix, still ZIP-only on wasm, ZIP64/AES broken); `devices` 17378 identical ×14 — frozen buggy; `thumbs` prod (only video-thumb path, stale path-only cache); `apps/library/model` prod; `ops` develop; `search` canonical; `analyze` media-only keep; `xtask` prod (fix wallpaper path — develop correct).

**Trash:** `trash 5.2.9` wired via `LocalVfs`+`ops`, but `Mem/Adb` ignore `to_trash` (permanent delete labelled Trash), manual `.trashinfo` parse no `%decoding`, dir sizes ~4k, `trash_empty` hides failures. Fix: adopt develop `os_limited::restore_all`, ID-based restore, Undo toast, AppImage gio probe. **P0-3 adds:** exact-name case-sensitive dialogs, reversible-by-default, volume Trash.

**Phone never mounts:** no polling/hotplug (`refresh_devices` only on modal/job), ADB gated **once at startup** (`adb_available`), MTP/iPhone have no provider (modal claims MTP automatic — false), `unauthorized` filtered silently, registry never evicted, WASM modal lies. Fix: 3s poller re-checking ADB, state surfacing, GVfs MTP probe, timeouts, honest wasm copy.

**Archive:** canonical ZIP-only fails tar/xz/7z/rar with `NotZip`; prod CLI fixes native only. Whole-file RAM read + O(n²) `data_range`; AES/ZIP64 truncated silently. Spec: prod base + pure-Rust tar + streamed extract.

**Eiga → thumbs:** keep `parse_ffprobe/MediaInfo/fit_size/ffmpeg` single-frame pattern; cut full player/TestCard/AudioPipe/76k `video.rs`. New `thumb_service`: ffprobe→frame≤384px→mtime-keyed LRU (512MiB), dedicated pool, cancellable, 800ms cold/60ms cached budget.

**Lies L-01…L-50:** zero `todo!`, zero project `panic!`; behavioural lies dominate — fake `connect_server ok:true`, silent audio stub, 5GB paced demo copy (32B animated 5s), WASM inline `spawn` blocking, search skips hidden/JUNK/PDFs + no stale filtering, `Open` lies for AppImage/ELF (L-34/35: no exec-bit API anywhere, `xdg-open` after `Ok`, broken `split_exec`), Nautilus reveal (L-44), resize cursor flicker + scrollbar fight (L-37), list checkbox/3-dot overlap (L-40), 600MB texture/scan leak + rescan (L-43), 1-char search dual-TextEdit + stale hits (L-45), hardcoded extension→icon map (L-46), wallpaper/dead settings (L-39), no OS notifications, React vite distraction.

**P0-1…11 spec'd:** exec+chmod+trust dialog; unified resize-cursor owner + 14px scrollbar gutter; Trash probe+Undo; Settings rewrite (delete wallpaper/demo-5GB, modern toggle table); list columns; `smart-ui-egui` component split + notify-rust/drag-drop; AI composer unify + reversible `Plan`/Undo journal; LRU caches + `walkdir/jwalk/ignore/rayon/memmap2` + ≤350MB RSS gate; internal Reveal default; single search input + req filter; `tree_magic_mini` magic→Solar icon binding.

Next build: mmust start from media-latest + small prod/develop patches, never recreate/wasm-refine; `panic=abort` must become `unwind` or `catch_unwind` is dead.


---

## Full assistant texts

### [seq 370] 2026-11-09 13:26:01

Core audit complete — `/tmp/opencode/canonical-core-audit/CORE-SPECS.md` (764 lines, non-destructive, originals untouched).

**21 zips / 7 lineages:** canonical = `rust-wasm-media-manager-latest` (57M, media/eiga superset). Prod line adds CLI archive + `icon_path`/settings/`kind_label` (take). Develop adds notify-coalesce + `restore_trashed` + panic guard + largest `app.rs` 70991 (merge). Recreate line (store 75k/engine 40k/ai/sys/treemap) is an incompatible `&mut self` VFS arch — ideas only. Wasm-refine (33 rs) ships **without `appkit.rs`** and stripped terminal/photos — never ship. Cross-orig has **no `archive.rs`**.

**Best per-file:** `backend` media+develop merge; `archive` prod (CLI fallback for tar/7z/rar + data-descriptor fix, still ZIP-only on wasm, ZIP64/AES broken); `devices` 17378 identical ×14 — frozen buggy; `thumbs` prod (only video-thumb path, stale path-only cache); `apps/library/model` prod; `ops` develop; `search` canonical; `analyze` media-only keep; `xtask` prod (fix wallpaper path — develop correct).

**Trash:** `trash 5.2.9` wired via `LocalVfs`+`ops`, but `Mem/Adb` ignore `to_trash` (permanent delete labelled Trash), manual `.trashinfo` parse no `%decoding`, dir sizes ~4k, `trash_empty` hides failures. Fix: adopt develop `os_limited::restore_all`, ID-based restore, Undo toast, AppImage gio probe. **P0-3 adds:** exact-name case-sensitive dialogs, reversible-by-default, volume Trash.

**Phone never mounts:** no polling/hotplug (`refresh_devices` only on modal/job), ADB gated **once at startup** (`adb_available`), MTP/iPhone have no provider (modal claims MTP automatic — false), `unauthorized` filtered silently, registry never evicted, WASM modal lies. Fix: 3s poller re-checking ADB, state surfacing, GVfs MTP probe, timeouts, honest wasm copy.

**Archive:** canonical ZIP-only fails tar/xz/7z/rar with `NotZip`; prod CLI fixes native only. Whole-file RAM read + O(n²) `data_range`; AES/ZIP64 truncated silently. Spec: prod base + pure-Rust tar + streamed extract.

**Eiga → thumbs:** keep `parse_ffprobe/MediaInfo/fit_size/ffmpeg` single-frame pattern; cut full player/TestCard/AudioPipe/76k `video.rs`. New `thumb_service`: ffprobe→frame≤384px→mtime-keyed LRU (512MiB), dedicated pool, cancellable, 800ms cold/60ms cached budget.

**Lies L-01…L-50:** zero `todo!`, zero project `panic!`; behavioural lies dominate — fake `connect_server ok:true`, silent audio stub, 5GB paced demo copy (32B animated 5s), WASM inline `spawn` blocking, search skips hidden/JUNK/PDFs + no stale filtering, `Open` lies for AppImage/ELF (L-34/35: no exec-bit API anywhere, `xdg-open` after `Ok`, broken `split_exec`), Nautilus reveal (L-44), resize cursor flicker + scrollbar fight (L-37), list checkbox/3-dot overlap (L-40), 600MB texture/scan leak + rescan (L-43), 1-char search dual-TextEdit + stale hits (L-45), hardcoded extension→icon map (L-46), wallpaper/dead settings (L-39), no OS notifications, React vite distraction.

**P0-1…11 spec'd:** exec+chmod+trust dialog; unified resize-cursor owner + 14px scrollbar gutter; Trash probe+Undo; Settings rewrite (delete wallpaper/demo-5GB, modern toggle table); list columns; `smart-ui-egui` component split + notify-rust/drag-drop; AI composer unify + reversible `Plan`/Undo journal; LRU caches + `walkdir/jwalk/ignore/rayon/memmap2` + ≤350MB RSS gate; internal Reveal default; single search input + req filter; `tree_magic_mini` magic→Solar icon binding.

Next build: mmust start from media-latest + small prod/develop patches, never recreate/wasm-refine; `panic=abort` must become `unwind` or `catch_unwind` is dead.


