# Task prompt(s) — Terminal app real audit (ses_e3fce7e1dffeeR37Pyi3OpRWB6)

## 1. [seq 4] 2026-11-09 14:03:52

You are a subagent spawned by another session.
You are the TERMINAL APP agent for smart-suite-official. Non-destructive: audit to /tmp/opencode/canonical-terminal-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/TERMINAL-SPECS.md (official, tmp disappears). Source in /home/lufuno last 7 days, ignore hidden: smart-ui-egui/src/terminal.rs 120099-121900 (2366-2401 lines), plus smart-desktop/apps/terminal/mod.rs 120567-120899, plus shell.rs, pty.rs, termlive.rs, appkit.rs. Do thorough research and find unknown unknowns. Must feel performant, modern UX, unmatched, reuse smart-ui-egui components, design system. Complete and refined.

Requirements from user (verbatim priority):
- MAKE THE TERMINAL APP REAL AND RUN FULL SHELL TOO, EVOLVE ITS UX, MAKE IT UNMATCHED. Run full shell, robust, predictable standards, real.
- Research Rust crates (pty: portable-pty, vt100, alacritty-terminal, crossterm etc), text rendering, tree-sitter for code preview already in file manager, same for terminal.
- Typing caret in all input text components hypnotic (user did it, keep breathing/jump).
- Can terminal have UI view on top of terminal output? Make output feel like real app/GUI-integrated, not raw scroll. This is novel: integrate GUI panels on top while keeping terminal standards predictable.
- Ctrl/palette, AI button as iconbutton consistency across all apps (filemanager/photos/terminal/eiga share AI icon/view), AI capnp must be made real and operational (check ~/smart/smart-protocol-official/schema/ai.capnp, ~/smart/smart-protocol-rs domain-ai, reversible, no destructive).
- Micro-interactions: spring animations (capture app), hover/press feedback, visual liveness. Searchbar/sortby icon buttons with optional labels in settings, responsive not forced. Trash has contextual appbar, all appbars contextual and consistent.
- Fix storage 92% indicator: drive progress turns reddish, bottom-left card opens cleaning/storage dialog as dialog TAB inside app (no new window), appbar shows relevant stuff, storage view offers graphs + obsidian connected-dots graph view (timeless file-tree visualisation) + other views, full workspace smart clean junk, waste spacing.
- Performance: no 600MB leak, multi-thread, LRU, process lifecycle, cancel atomic.
- Protocol-first: smart-protocol-rs domain-console/domain-process/domain-clipboard, backends disposable (Linux now, macOS/Win later). decouple apps from libs.
- Lie hunt: no fakes, honest demo where needed, no todo!.
- Output TERMINAL-SPECS.md with matrix, lies, missing features + new ideas, perf budget, UX/dropped micro-interactions, protocol mapping, acceptance. Thorough, correct, cargo test suite plan.


---

## 2. [seq 174] 2026-11-09 14:04:48

CLARIFICATION from user: hypnotic caret is NOT terminal-only. It is whole smart-suite framework. All apps (filemanager, photos, eiga/video, terminal, settings, searchboxes, AI composer, pickers, dialogs) need animated caret: jumping, breathing, hypnotic feel, Mac-like. It was already in prior versions, must be preserved. In TERMINAL-SPECS.md, move caret from terminal-only section to framework-wide design system in smart-ui-egui (shared TextEdit style), with per-app usage (filemanager search, photos search, video subs rename, AI composer, etc). Terminal inherits it, not owns it. Update spec accordingly, do not drop.

---

