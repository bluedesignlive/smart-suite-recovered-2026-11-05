# Task prompt(s) — Framework closure build (ses_e3f60c50effeo0NTerFknr3yFQ)

## 1. [seq 4] 2026-11-09 16:03:43

You are a subagent spawned by another session.
You are Framework Closure builder for smart-suite-official. Manager leaving user out — do not wait, fix everything, be robust, no mess, no vanilla. Non-destructive only (do not rm/purge originals). Workspace root /home/lufuno/smart/smart-suite-official/ (crates/smart-core, crates/smart-ui-egui, crates/xtask, apps/smart-files, apps/smart-photos). Audit at audits/CANONICAL-AUDIT.md (457 lines) is facts only — do not treat as spec. Spec truths are in specs/*.md + screenshots provided by user (/home/lufuno/Pictures/Screenshots/Screenshot From 2026-11-09 15-49-06.png storage, ... 15-57-... photos). Protocol-first (~/smart/smart-protocol-official, smart-protocol-rs), backends disposable, one sidebar/one behaviour, one smart-core/one smart-ui-egui, no hardcoded duplicates, AppImage small part, robust crates allowed.

Mission: Close framework so apps stop being 10% half-baked:
1. smart-ui-egui must be single canonical component lib: caret.rs (hypnotic breathing 0.65+0.35·sin(2πt/1.4) + jump spring_to 420/26, all apps), anim.rs (enter/exit/hover/pack springs, segmented 32/36px icon buttons, no per-row menu jitter — menu open measured once no moves on open), paint/theme/tokens (premium text, not vanilla egui), sidebar chrome.rs single sidebar (Favorites/Places/Folders + bottom Computer 95% reddish card using usage_color 92%+ rule, click opens Storage tab not new window), contextual appbar (SearchBar/SortBy/AI iconbutton collapse <700px, theme/grid/commands), picker.rs (New Folder/search/multi-sort + XDG portal contract), dnd.rs (global drag/drop + x-special), dialogs law (wrap-close, grid, close top+20, spring open/close, no hardcoded heights causing gaps), storage component (donut + category bars + Biggest wins Safe/Review + Map/Graph/Obsidan + Scan::step(6)).
2. smart-core protocol-shaped: VFS/trash/archive/search/devices/thumbs (LRU 500MB, video ffprobe→frame≤384, MTP/ADB poller honest, phone mount diagnostics), no platform leakage in UI.
3. Apps must be styled via framework only: apps/smart-files + apps/smart-photos must REMOVE vanilla egui create-new-text layouts, use framework chrome/content/paint/theme. Photos standalone grid+search is vanilla → re-skin via shared-ui container (real fonts/icons), include inspector, status 111 items·298MB, Computer 25.2GB free line. Kuro/Terminal untouched except import shared types read-only.
4. Fix menu jitter, resize cursors 8-way + scrollbar gutter 14px exclusive, list gutter 28px, collapsed rail, horizontal selectors centered, obsidian view after all.
5. Framework internal screenshots: add harness `xtask screenshot` or `cargo run -- screenshot` that renders main windows headless and saves PNGs to /home/lufuno/smart/smart-suite-official/screenshots/framework/ (files, photos, storage dialog, settings). Generate before and after each app, view them, ensure consistency. Include these screenshots in PR description and docs. CI can check PNGs exist and MD5 not identical (visual diff).
6. Screenshots required (must submit):
   - Framework: files gallery (list vs grid), storage tab (donut + categories + biggest wins + map/graph), settings 6 tabs, picker dialog, menu open (no jitter), sidebar collapsed/expanded, bottom red card at 95% if we can simulate.
   - For each app built: file-manager list, photos grid thumbs, (terminal/kuro later).
   Submit via BUILD-STATUS.md images list + agent final answer must list screenshot paths and visual diff notes.
7. Verification: cargo check --workspace green, cargo test -p smart-core -p smart-ui-egui -p smart-photos -p smart-files green, grep todo!/panic! (except panic_hook) clean, launch smoke per app (--help/--smoke). After green, git add -A && git commit "framework: close single lib" locally (no push).
8. Document honestly in BUILD-STATUS.md at official root what remains (e.g., Conflict modal, tree-sitter full, AI reversible if not in scope) — do not flag done if halfbaked.

Do not refactor protocol schemas, do not create scattered cores, do not use dead code. Thorough, robust, premium UX. This is framework only, apps styling pass.

---

## 2. [seq 172] 2026-11-09 16:04:36

Manager add-on (user): all stuff from 21 zips + components must land to framework lib, no skipped stuff. Audit CANONICAL-AUDIT.md is read-only facts — use it as union union matrix. Implement the never-landed union into crates/smart-ui-egui + crates/smart-core (not app-level patches): eiga storage.rs 54375 (+analyze/squarify/Scan), develop Undo/guarded_frame/catch_unwind, recreate store/engine/ai/sys/treemap (adapter as IdeasOnly if incompatible but port patterns), desktop picker/assistant/capture/settings as components (not shell), prod archive CLI, xplat baseline, thumbnails video LRU, etc. One lib, list in BUILD-STATUS.md union table: zip → landed component/file or honest why incompatible. No skipping. Verify screenshots still consistent. Do not claim done until every row landed or honestly documented as incompatible with evidence.


---

