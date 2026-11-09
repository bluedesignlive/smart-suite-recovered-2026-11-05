# Subagent result — Terminal Kuro port map

Session `ses_e3f3051f1ffe8PXh0EEnBOskrD` | 2026-11-09 16:56:38 → 2026-11-09 17:05:03

## Task

You are a subagent spawned by another session.
You are a RESEARCH-ONLY port-map agent for smart-suite-official. Do NOT edit code, do NOT write specs, do NOT modify any crate/app. Read-only on all source zips + current workspace. Write exactly one report: /home/lufuno/smart/smart-suite-official/audits/TERMINAL-KURO-PORT-MAP.md (create audits/ if needed; no other file writes). Non-destructive, no rm, no git changes.

Context: current canonical workspace /home/lufuno/smart/smart-suite-official/ has crates/smart-core, crates/smart-ui-egui, apps/smart-files, apps/smart-photos. Framework closure is in progress by another agent — do not touch its files. Terminal and Kuro are NOT built yet. User has 21 diverged zips; do not prioritize one zip. Union the best working foundations.

Inputs to scan equally:
- /tmp/opencode/canonical-ux-audit/zips/* (all extracted dirs)
- /home/lufuno/work/BIGBOY/*.zip (extract read-only to /tmp/opencode/port-map-extract if needed; never modify originals)
- /home/lufuno/smart/smart-suite/apps/*.zip
- existing specs: specs/TERMINAL-SPECS.md, specs/VIDEO-SPECS.md, specs/EIGA-SPECS.md, audits/CANONICAL-AUDIT.md
- current workspace files for integration points only.

Deliverable: exhaustive port map with file:line evidence, not prose opinions:
1. TERMINAL: every real implementation found (PTY, VT parsing, blocks, GUI overlay, palette, tabs/splits, search, scrollback, protocol console/AI, tree-sitter/tokenizer, keybindings, config, themes). For each: zip path + file:line + what works + deps + integration risk. Identify the best foundation per subsystem (union matrix) and anything that is fake/stub.
2. KURO (video player, formerly Eiga): every real implementation found (mpv/libmpv/ffmpeg decode, seek, 720p/HDR, subtitles movable, resume, playlist, audio tracks, speed, frame-step, screenshot, OSD, cache, hardware decode, thumbnail engine shared with file manager). For each: zip + file:line + what works + deps + risk. Best foundation per subsystem + fake/stub inventory.
3. LAUNCH REALITY: for the best Terminal and best Kuro foundation, list exact build/run blockers (missing crates, assets, system libs, eframe/egui version mismatch, no main/window, silent exit) with evidence. Do not claim a launch works without a command/log.
4. INTEGRATION CONTRACT: what each app must import from smart-core/smart-ui-egui vs own domain code; flag duplicate-core risks. No code.
5. UNION MATRIX table: subsystem | best zip/file:line | alternative | current canonical status (missing/partial) | port effort S/M/L.
6. TOP 10 RISKS and exact verification gates needed (tests, smokes, screenshots) before calling either app done.

Rules: no hardcoded zip as "the" base; no claims without file:line; no spec writing; no code changes. If a zip is corrupt/missing, note it. Finish with a one-page executive summary at top. This report is the build input for later Terminal/Kuro agents.

## Final result (last assistant message, seq 5)

I'll start by surveying all input locations in parallel.



---

## Full assistant texts

### [seq 5] 2026-11-09 16:56:38

I'll start by surveying all input locations in parallel.



