# Task prompt(s) — Canonical audit missing features (ses_e3f70f36fffeCI2bhJ1m5hQCO5)

## 1. [seq 4] 2026-11-09 15:46:02

You are a subagent spawned by another session.
You are the CANONICAL AUDIT agent (research ONLY, no code edits, no destructive stuff). Non-destructive, read-only + write report to /home/lufuno/smart/smart-suite-official/audits/CANONICAL-AUDIT.md (and copy to /tmp). Do NOT fix code. Do NOT hardcode any zip. Audit all 21 file-manager zips equally (list them from earlier: prod2_vgood, prod2_huge, develop, eiga_latest, improve, refinements, xplat, recreate, desktop etc. from /tmp/opencode/canonical-ux-audit/zips/ and /home/lufuno/work/BIGBOY/*.zip and /home/lufuno/smart/smart-suite/apps/*.zip) AND current canonical version at /home/lufuno/smart/smart-suite-official/Cargo.toml workspace (crates/smart-core, crates/smart-ui-egui, apps/smart-files) vs prompt1.txt 11 P0 + CANONICAL-SPECS.md. Produce exhaustive tables: what current has, what it never implemented (never landed) by comparing each zip's best per-file (per UX-SPECS per-file matrix), list 3-dots close to icon, left padding huge, text not premium, settings no new relevant toggles, collapsed icon buttons missing, resize icons don't appear, bottom sidebar not red at 95% (usage_color threshold), storage beautiful maps/nodes (eiga storage 54375) never landed, trash/destructive dialog, exec AppImage, search 1-char, mimetypes hardcoded, phone MTP, memory 600MB, internal reveal, AI composer, drag-paste, list overlap, plus all other wrong/half-baked. Include file:line evidence for each missing. User has many diverged versions good at specific stuff bad at others — produce union matrix (x for each lineage best) + gap list. No coding, audit only. Report must be precise, no claims without evidence.

---

