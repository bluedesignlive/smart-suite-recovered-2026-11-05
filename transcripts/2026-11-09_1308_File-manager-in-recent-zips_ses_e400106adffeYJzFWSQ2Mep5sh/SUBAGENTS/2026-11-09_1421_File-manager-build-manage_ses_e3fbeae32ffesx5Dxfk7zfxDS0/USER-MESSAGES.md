# Task prompt(s) — File-manager build manage (ses_e3fbeae32ffesx5Dxfk7zfxDS0)

## 1. [seq 4] 2026-11-09 14:21:08

You are a subagent spawned by another session.
You are the builder for smart-suite-official canonical file manager. Manager (orchestrator) sets direction, you execute. Non-destructive, work in /home/lufuno/smart/smart-suite-official/canonical-file-manager/ (do NOT modify originals in /tmp or zips maybe already fixed 3 errors). Reference specs: /home/lufuno/smart/smart-suite-official/specs/CANONICAL-SPECS.md and audits, plus ~/smart/smart-suite/prompt1.txt 11 P0. Protocol-first (smart-protocol-rs), backends disposable, 1 sidebar 1 behaviour via smart-ui-egui, no hardcoded duplicates.

Your task NOW (focus managing = file manager):
1. Finish green build: assets already copied to assets/thumbs,audio, thumbs.rs and archive.rs (which->String) already patched by manager — verify cargo check -p smart-core -p smart-ui-egui passes. Fix any remaining 31 errors (missing includes, lifetimes, panic=abort -> unwind if needed for catch_unwind). No todo!, no stubs.
2. Implement P0 managing slice in order (remove wallpaper/demo-5GB/accent lie, settings 6-tab, list overlap grid, resize cursors + scrollbar gutter, drag-paste freeze, trash + destructive dialog case-sensitive, exec+chmod, internal Reveal, 1-char search, real mimetypes tree_magic_mini/mime_guess, phone MTP + ADB poller diagnostics, memory LRU + scan step, obsidian storage graph as tab, text preview tree-sitter, breadcrumb context menu, open-with wrap fix, AppImage script verified but small part).
3. Do NOT hardcode only prompt1 list. Find unknown unknowns, feature parity, robust crates (jwalk/rayon/walkdir/notify/trash/tree_magic_mini...), performance budgets.
4. cargo test green.
5. Write BUILD-STATUS.md in canonical-file-manager/ with what was fixed, remaining P0s, and next steps. Non-destructive, single agent, thorough.

Do not refactor to unrelated apps (photos/kuro/terminal) — focus managing first.

---

## 2. [seq 386] 2026-11-09 14:24:50

User approved final layout from start. Restructure immediately (non-destructive, move not copy) your current work in /home/lufuno/smart/smart-suite-official/canonical-file-manager/ to final workspace:
/home/lufuno/smart/smart-suite-official/
  Cargo.toml (workspace resolver 2, members = crates/smart-core, crates/smart-ui-egui, crates/xtask, apps/smart-files)
  crates/smart-core/ (from canonical-file-manager/crates/smart-core)
  crates/smart-ui-egui/ (from canonical-file-manager/crates/smart-ui-egui)
  crates/xtask/ (from canonical-file-manager/crates/xtask)
  apps/smart-files/ (new bin crate depends on smart-core+smart-ui-egui, re-export existing smart-ui-egui main, Cargo.toml with [[bin]] name=smart-files, main.rs delegating to smart_ui_egui)
  assets/ (thumbs/audio/icons/tokens at workspace root or crates/smart-core/assets as include path expects - keep include_bytes! paths relative to crates/smart-core/src/demo.rs ../../../assets/{thumbs,audio} working: place assets at crates/smart-core/../assets? Actually demo.rs expects ../../../assets/thumbs from crates/smart-core/src -> that is smart-suite-official/assets/thumbs. So put assets at /home/lufuno/smart/smart-suite-official/assets/)
  protocol/ (symlink or copy from ~/smart/smart-protocol-official/schema)
Then cargo check --workspace green, BUILD-STATUS updated. Do final layout from start so no scattered cores remain: 1 smart-core, 1 smart-ui-egui, apps thin. Continue managing P0s after layout green. Non-destructive originals untouched.

---

