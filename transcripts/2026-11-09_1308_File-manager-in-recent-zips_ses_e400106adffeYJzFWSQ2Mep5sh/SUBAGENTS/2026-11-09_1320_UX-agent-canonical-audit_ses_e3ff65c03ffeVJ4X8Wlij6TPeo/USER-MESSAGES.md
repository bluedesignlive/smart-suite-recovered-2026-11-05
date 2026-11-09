# Task prompt(s) — UX agent canonical audit (ses_e3ff65c03ffeVJ4X8Wlij6TPeo)

## 1. [seq 4] 2026-11-09 13:20:19

You are a subagent spawned by another session.
You are the UX agent for a canonical Rust file manager project to be sold for $299. Source zips are in /home/lufuno (last 7 days, ignore hidden). There are 21 file-manager zips across 7 lineages, plus logdrum/kingdrum distractions. Do thorough research, find things the user has NOT thought of yet (unknown unknowns), do non-destructive work only (extract to /tmp/opencode/canonical-ux-audit/, never modify originals).

Base understanding: canonical is `smart-files` lineage (smart-core + smart-ui-egui + xtask), NOT smart-desktop shell, NOT full eiga media player. Key zips:
- /home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip
- /home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip
- /home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip
- /home/lufuno/work/BIGBOY/rust-wasm-ui-refinements.zip
- /home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-good1.zip
- /home/lufuno/work/BIGBOY/recreate-file-manager-in-rust-2.zip (alt smart-egui branch with store/engine/ai/sys/treemap/dialogs)
- /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip (smart-desktop, for reference only)
- /home/lufuno/Documents/rust-wasm-media-manager-latest.zip (eiga-core)

Your focus:
1. UX COMPLETENESS AUDIT - Compare chrome.rs/content.rs/app.rs/modals.rs/overlays.rs/photos.rs/shell.rs/terminal.rs/copyfx.rs/paint.rs/theme/icons/appkit across those zips by size and feature. List which version has best version of each file and what each lacks. E.g. terminal.rs 120k vs 86k, chrome 64k vs 71k vs 72k, etc. Do not trust zip titles.
2. VIDEO THUMBNAILS BLAZING - User says file manager DOES need some eiga stuff for blazing video thumbnails. Audit eiga-core (player/source/subs) + smart-core analyze.rs + video.rs/highlight.rs/pty/storage/termlive + thumbs.rs/gen_thumbs.py. Find the fastest thumbnail pipeline and how to integrate into file manager without shipping full media player. Propose UX for gallery grid, filmstrip, lazy loading, caching.
3. DRAG FOLDER TO TAB BUG - It opens now but doesn't paste. Trace drag-drop in chrome/content/ops/backend. Find root cause and spec fix, including paste preview, conflict, progress, cancel.
4. SETTINGS CLEANUP - Settings view has wallpapers/desktop stuff it shouldn't. List every current setting key found in smart-files and smart-desktop, mark file-manager relevant vs wallpaper/assistant/capture/desktop to REMOVE. Propose final file-manager-only settings.
5. CATCH ALL LIES - Find every lie/stub the app does: unimplemented buttons, TODOs, panic!/unimplemented!/todo!, demo FS presented as real, fake progress bars, silent failures, hardcoded fake devices, fake search results, disabled trash/archive actions. Grep for todo/unimplemented/fake/demo. List file:line and required honest behavior. No stubs allowed for $299.
6. PHONE/USB DISCOVERY UX - User connected phone, never appeared as mounted. Audit devices.rs/vfs.rs/mounts UX. What does user see when phone plugged? What diagnostics are missing? Propose mount indicator, fallback, error surfacing.
7. PERFORMANCE + UNKNOWN UNKNOWNS - Large-folder rendering, search responsiveness, thumbnail cache, memory, error handling, accessibility, keyboard shortcuts, etc. Deliberately look for edge cases user didn't list.
8. OUTPUT: Write /tmp/opencode/canonical-ux-audit/UX-SPECS.md with: feature matrix, per-file best version table, drag-paste fix spec, settings in/out list, lie inventory, thumbnail pipeline spec, phone-mount UX spec, performance budget, unknown-unknowns list, and final UX acceptance criteria. Also print summary to your final answer. Do not merge code yet, do not modify originals. Focus on making it completely usable.

---

## 2. [seq 295] 2026-11-09 13:22:23

CRITICAL ADDITION from user - prompt1.txt in /home/lufuno/smart/smart-suite/prompt1.txt (you must incorporate all 11 into your audit and UX-SPECS.md, this overrides previous settings scope):

1. When open AppImages/executables app LIES: says opening but doesn't really open. Can't make executable like other managers via Get Info. Investigate exec.rs/backend.rs. Executable/ chmod +x integration can be done differently than hidden if good UX.
2. Window edges have NO resize cursor changes - user must guess N/S/E/W/NE/NW etc drag spots. Needs OS cursor messages on hover. Right-side resize handler fights scrollbar focus. Audit chrome/shell window frame hitboxes.
3. NO TRASH + NO destructive actions dialog. Files are very case-sensitive, needs good UX. Need Trash + confirm dialog spec, reversible, no destructive without dialog.
4. REMOVE wallpaper + irrelevant settings stuff. Remove Demo 5GB file stuff. Settings app must be real file-manager settings (toggles/options). Research what new file managers that replace traditional have/need.
5. Checkbox in List view overlaps file icon, 3 dots overlaps time. Hover must be proper/clean/responsive, no overlapping. Fix list row layout.
6. Too many versions with good/bad stuff. Combine ALL features. MUST USE smart-ui-egui so other apps in same ecosystem can reuse same components. DECOUPLE apps from libs. Need complete Linux backend, deeper desktop integrations for notifications/drag-drop/resizing.
7. Newer versions lost Smart AI composer input styling. All AI stuff must be REVERSABLE, no destructive actions, everything reversible.
8. Memory: no dumping -> 600MB leak. But dealloc rescan is dumb: traversing trees expensive, rescans large dirs inefficiently. Needs professional code, save time/resources, multi-threading + robust Rust crates, smart cache/invalidate.
9. Reveal/Open in File Manager lies: opens Nautilus, not itself. Must open inside itself.
10. Search bug: small window + type ".appimage" selects last letter/only 1-char search; large window highlights all. 1-char search bug. Fix focus/selection/layout coupling.
11. Use REAL mimetypes (Solar-style), not hardcoded mimetypes for icons. Implicit correct, visuals over extension reading.

Add these as P0 acceptance criteria to your UX-SPECS.md. Include per-item root cause in code, fix spec, and lie inventory. You already have eiga thumbnails, drag-paste, phone-mount, settings cleanup, lie hunt, perf - incorporate all of prompt1.txt as additional mandatory.

---

