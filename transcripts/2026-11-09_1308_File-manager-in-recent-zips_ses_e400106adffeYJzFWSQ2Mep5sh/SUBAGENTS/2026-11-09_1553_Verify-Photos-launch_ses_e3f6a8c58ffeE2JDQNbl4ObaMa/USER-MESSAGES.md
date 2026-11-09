# Task prompt(s) — Verify Photos launch (ses_e3f6a8c58ffeE2JDQNbl4ObaMa)

## 1. [seq 4] 2026-11-09 15:53:02

You are a subagent spawned by another session.
You are Photos verifier + fixer for smart-suite-official. Manager owns orchestration, you own apps/smart-photos/. Problem: apps/smart-photos was flagged done (cargo check + 43 tests, commit c8003c4/2aa5745) by previous reviewer, but USER says it doesn't even launch — that's "half-baked" false done. You must actually build and launch it. Non-destructive, originals untouched. Allowed crates: smart-core/smart-ui-egui are READ-ONLY (do not edit them), only fix apps/smart-photos/ + Cargo.toml member + PHOTOS-STATUS.md. Steps:
1. cargo build -p smart-photos --release (or debug) and run binary with --help and with a sample profile. Capture stdout/stderr, panic backtrace, exit code, missing assets/thumbs/fonts. Fix launch blockers in apps/smart-photos (main.rs, lib.rs, assets path, capture, icons, fonts, winit/eframe init). If it needs emulated display, try `xvfb-run` or `--help` path headless check. Do not claim done until `cargo run -p smart-photos -- --help` returns 0 and an actual window init succeeds or fails honestly with log.
2. Re-check test suite: cargo test -p smart-photos (43 tests claimed, now 53). Add launch smoke test if missing.
3. Document honestly LAUNCH VERDICT (works / needs display / still broken with exact error) in /home/lufuno/smart/smart-suite-official/PHOTOS-STATUS.md (append Verify §, don't overwrite P0). If you fix launch, git add -A && git commit -m "photos: fix launch + verify" (local only). If you cannot fix in scope, state remaining blockers plainly, don't flag done.
4. Report file:line fixes, build/run evidence, and what remains.


---

