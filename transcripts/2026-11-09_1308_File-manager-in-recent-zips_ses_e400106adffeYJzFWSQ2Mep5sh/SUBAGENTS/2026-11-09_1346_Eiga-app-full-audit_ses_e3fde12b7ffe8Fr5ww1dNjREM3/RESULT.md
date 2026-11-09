# Subagent result — Eiga app full audit

Session `ses_e3fde12b7ffe8Fr5ww1dNjREM3` | 2026-11-09 13:46:51 → 2026-11-09 13:53:41

## Task

You are a subagent spawned by another session.
You are the EIGA APP agent. The user says eiga app needs to be made real too — all its source is done in /home/lufuno (last 7 days, ignore hidden). Source: /home/lufuno/Documents/rust-wasm-media-manager-latest.zip (and -2, .zip) with crates/eiga-core/{lib.rs,player.rs 30125,source.rs 30036,subs.rs 14951}, crates/smart-core/src/analyze.rs 31515, crates/smart-ui-egui/src/{video.rs,highlight.rs,app.rs,chrome.rs,modals.rs}, plus smart-core backend/thumbs/storage. Also check refine-futuristic zips for any eiga-adjacent. Do non-destructive work to /tmp/opencode/canonical-eiga-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md (user said tmp disappears, official is canonical). The spec must be protocol-first (smart-protocol-rs domain-media, domain-video) with backends disposable.

Focus: 
- Full player reality check: does it really play? Check player/source/subs completeness, ffmpeg/ffprobe integration, streaming, playlist, subtitles, OSD, audio pipe, TestCard honesty. Catch all lies (stub audio, fake connect_server, demo progress).
- Like file-manager audits: per-file best version, lie inventory L-01..., performance (memory, rescan, thumb pipeline vs full player speed), robustness, missing features the user hasn't thought of (unknown unknowns: HDR, spatial audio, casting, PiP, chapters, shader effects).
- Video thumbnails already spec'd for file manager, but for eiga itself: frame-accurate seek, 4K60, HDR10, codecs, network streams.
- Integration: protocol media service, clipboard, notifications, MTP/phone library, AppImage packaging, smart-ui-egui reuse.
- AppImage + WASM packaging, no destructive actions without dialog, reversible.
- Output EIGA-SPECS.md with: architecture, per-file matrix, lie list, missing features + new ideas, performance budget, player UX, integration + protocol mapping, acceptance criteria. Non-destructive, thorough.

## Final result (last assistant message, seq 255)

EIGA audit done (non-destructive, `/tmp/opencode/canonical-eiga-audit/`). **Verified spec copied to `/home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md` (92K, 757 lines).**

**Verdict:** Native decode is real (39/39 `eiga-core` tests pass incl. `end_to_end_with_real_ffmpeg`: probe→≥15 frames→seek→SRT→PCM). `player.rs`+`subs.rs`+`parse_ffprobe` are solid; `video.rs` latest 76K (not 55K orig) is the only shippable. But 12 lies block ship: speed doesn't change video (L-01), audio-delay display-only (L-02), hard 1280×720 cap (L-03), synthetic PTS/keyframe seek (L-04), rotation/aspect dead code (L-05/06), TestCard banner auto-hides after 400 frames + mislabels MTP/phone as "Web preview" (L-07), stereo-only downmix (L-09), HDR truncation, no network/live/cast/PiP.

**Per-file:** `thumbs.rs` must TAKE prod 5,183B `video_thumb()` (media-latest regressed to image-only blank tiles) + LRU/mtime. `app.rs` MERGE develop undo/guarded-frame. `refine-futuristic-*` has no eiga at all (smart-desktop only). `smartui-media` timeline-king is future backend, not replacement.

**Protocol-first:** `media.capnp` unimplemented today — spec maps `Media`+new `domain-video` (+audio/files/clipboard/notification/device) with backend-disposable traits, `getSupportedFormats` honesty, resume/clipboard bytes, MTP streaming via `pipe:0`.

**Packaging:** media-latest has no `build-appimage.sh` (prod-only) and bundles no ffmpeg — spec ports script + static ffmpeg + MIME + smoke; WASM `TestCard` honestly gated.

Full architecture, per-file matrix, L-01…L-24 lie inventory, U-01…U-22 unknowns, perf budgets (frame/rescan/thumb 80–200ms vs full 720p pipe), UX + acceptance gates in `EIGA-SPECS.md`.


---

## Full assistant texts

### [seq 255] 2026-11-09 13:53:37

EIGA audit done (non-destructive, `/tmp/opencode/canonical-eiga-audit/`). **Verified spec copied to `/home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md` (92K, 757 lines).**

**Verdict:** Native decode is real (39/39 `eiga-core` tests pass incl. `end_to_end_with_real_ffmpeg`: probe→≥15 frames→seek→SRT→PCM). `player.rs`+`subs.rs`+`parse_ffprobe` are solid; `video.rs` latest 76K (not 55K orig) is the only shippable. But 12 lies block ship: speed doesn't change video (L-01), audio-delay display-only (L-02), hard 1280×720 cap (L-03), synthetic PTS/keyframe seek (L-04), rotation/aspect dead code (L-05/06), TestCard banner auto-hides after 400 frames + mislabels MTP/phone as "Web preview" (L-07), stereo-only downmix (L-09), HDR truncation, no network/live/cast/PiP.

**Per-file:** `thumbs.rs` must TAKE prod 5,183B `video_thumb()` (media-latest regressed to image-only blank tiles) + LRU/mtime. `app.rs` MERGE develop undo/guarded-frame. `refine-futuristic-*` has no eiga at all (smart-desktop only). `smartui-media` timeline-king is future backend, not replacement.

**Protocol-first:** `media.capnp` unimplemented today — spec maps `Media`+new `domain-video` (+audio/files/clipboard/notification/device) with backend-disposable traits, `getSupportedFormats` honesty, resume/clipboard bytes, MTP streaming via `pipe:0`.

**Packaging:** media-latest has no `build-appimage.sh` (prod-only) and bundles no ffmpeg — spec ports script + static ffmpeg + MIME + smoke; WASM `TestCard` honestly gated.

Full architecture, per-file matrix, L-01…L-24 lie inventory, U-01…U-22 unknowns, perf budgets (frame/rescan/thumb 80–200ms vs full 720p pipe), UX + acceptance gates in `EIGA-SPECS.md`.


