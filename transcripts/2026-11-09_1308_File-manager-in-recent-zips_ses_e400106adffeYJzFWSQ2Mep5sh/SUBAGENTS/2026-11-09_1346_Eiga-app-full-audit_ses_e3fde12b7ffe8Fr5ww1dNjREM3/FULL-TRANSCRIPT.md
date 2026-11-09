# Subagent transcript — Eiga app full audit

- Session ID: `ses_e3fde12b7ffe8Fr5ww1dNjREM3`
- Parent: `ses_e400106adffeYJzFWSQ2Mep5sh`
- Created: 2026-11-09 13:46:51
- Updated: 2026-11-09 13:53:41
- Messages: 31
- User/task msgs: 1

---

### TASK / USER [seq 4] 2026-11-09 13:46:51

You are a subagent spawned by another session.
You are the EIGA APP agent. The user says eiga app needs to be made real too — all its source is done in /home/lufuno (last 7 days, ignore hidden). Source: /home/lufuno/Documents/rust-wasm-media-manager-latest.zip (and -2, .zip) with crates/eiga-core/{lib.rs,player.rs 30125,source.rs 30036,subs.rs 14951}, crates/smart-core/src/analyze.rs 31515, crates/smart-ui-egui/src/{video.rs,highlight.rs,app.rs,chrome.rs,modals.rs}, plus smart-core backend/thumbs/storage. Also check refine-futuristic zips for any eiga-adjacent. Do non-destructive work to /tmp/opencode/canonical-eiga-audit/ then copy verified spec to /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md (user said tmp disappears, official is canonical). The spec must be protocol-first (smart-protocol-rs domain-media, domain-video) with backends disposable.

Focus: 
- Full player reality check: does it really play? Check player/source/subs completeness, ffmpeg/ffprobe integration, streaming, playlist, subtitles, OSD, audio pipe, TestCard honesty. Catch all lies (stub audio, fake connect_server, demo progress).
- Like file-manager audits: per-file best version, lie inventory L-01..., performance (memory, rescan, thumb pipeline vs full player speed), robustness, missing features the user hasn't thought of (unknown unknowns: HDR, spatial audio, casting, PiP, chapters, shader effects).
- Video thumbnails already spec'd for file manager, but for eiga itself: frame-accurate seek, 4K60, HDR10, codecs, network streams.
- Integration: protocol media service, clipboard, notifications, MTP/phone library, AppImage packaging, smart-ui-egui reuse.
- AppImage + WASM packaging, no destructive actions without dialog, reversible.
- Output EIGA-SPECS.md with: architecture, per-file matrix, lie list, missing features + new ideas, performance budget, player UX, integration + protocol mapping, acceptance criteria. Non-destructive, thorough.

### TOOL: read [seq 5] 2026-11-09 13:46:51 status=completed

```json
{
  "path": "/home/lufuno"
}
```

```
Read directory /home/lufuno, entries 1-105
.aider/
.bun/
.cache/
.cargo/
.cline/
.config/
.cua-driver/
.dbus/
.dsh/
.gnupg/
.herdr/
.hermes/
.hiddel/
.jcode/
.kilo/
.local/
.mozilla/
.npm/
.opencode/
.pki/
.rustup/
.var/
.vscode-oss-shared/
.vscode-oss/
.wine.bak/
.wine/
.zcode/
assets/
crates/
Desktop/
Documents/
donghua/
Downloads/
Lemon-Studio/
mirui/
models/
Music/
Notes/
Pictures/
Projects/
Public/
smart-interface/
smart/
target/
Templates/
tests/
tools/
untitled folder/
Videos/
web/
wine-staging/
work/
.aider.chat.history.md
.aider.input.history
.bash_history
.bash_logout
.bash_profile
.bashrc
.dmrc
.git-credentials
.gitconfig
.ICEauthority
.inputrc
.netrc
.profile
.wget-hsts
.Xauthority
.xsession-errors
.xsession-errors.old
.zshenv
.zshrc
62 - 《牧神记》第58话：虫军【玄幻 ｜ 修仙 ｜ 小说改 ｜ Made By Bilibili】.f251.webm.part
bonsai-ternary.html
Cargo.lock
Cargo.toml
direct_usb
direct_usb.c
downloaded.txt
enable_keyboard.bat
file-de-cluade.txt
fl-studio.sh
index.html
kilo_audit_005_chat_context.txt
kilo_chats_export.txt
last-de-cluade-2.txt
last-de-cluade-prompt.txt
last-de-cluade.txt
lint-stub.ts
package-lock.json
package.json
probe_usb
probe_usb.c
qwen-last.txt
qwen.txt
README.md
run_palera1n.sh
tsconfig.json
Untitled.flp
usb_fix
usb_fix.c
usb_inject.c
usb_inject2.c
usb_pre.c
veryl-x86_64-linux.zip
vite.config.ts
```

### TOOL: shell [seq 5] 2026-11-09 13:46:51 status=completed

```json
{
  "command": "ls -R /home/lufuno/Documents 2>&1 | head -n 500; echo ---; ls -lh /home/lufuno/Documents/*.zip 2>&1; echo ---; ls -lh /home/lufuno/smart/smart-suite-official/specs/ 2>&1; echo ---; ls -R /tmp/opencode 2>&1 | head -n 300"
}
```

```
/home/lufuno/Documents:
Cline
Fightland.S01.720P.MoviesMod.zone.zip
Image-Line
MX-25.3_KDE_x64.iso
Pasted image.png
SMART_APPLICATION_SPEC.md
Saved
Screenshot 2026-10-12 at 17-27-44 Design Editor.png
Screenshot 2026-11-05 at 01-53-19 Smart Desktop.png
Screenshot 2026-11-05 at 02-00-19 Smart Desktop.png
Screenshot 2026-11-05 at 11-13-07 Smart Desktop.png
Smart-Logo
SmartShell-x86_64.AppImage
[Windows X-Lite] Optimum 10 Pro v6 (FBConan).7z
artix-plasma-dinit-20260813-x86_64.iso
botse
crextio+protocol
crextio-complete-repo-with-source-2026-09-21
crextio-complete-repo-with-source-2026-09-21.zip
crextio-protocol-first-2026-09-21--part-2.zip
eiga.png
files
files.zip
full-capability-based-kernel-requirements.zip
full-capability-based-kernel-requirementskimi.zip
kilo bak
liquid-glass-lab
liquid-glass-lab.zip
mimoclaw_workspace--file-manager.tar.gz
mimoclaw_workspace.tar.gz
modern-premium-os-design-system--(kimi-2.7).zip
modern-premium-os-design-system--(minimax 3).zip
modern-premium-os-design-system.zip
modern-premium-os-design-system2.zip
modern-premium-os-design-systemkimi.zip
modern-premium-os-design-systemminim.zip
rust-wasm-media-manager-2.zip
rust-wasm-media-manager-latest
rust-wasm-media-manager-latest.zip
rust-wasm-media-manager.zip
smart-files.png
smart-photos-demo
smart-photos-demo.zip
ventoy-1.1.10-linux
ventoy-1.1.10-linux.tar.gz
ventoy-1.1.17-linux
ventoy-1.1.17-linux.tar.gz

/home/lufuno/Documents/Cline:
Hooks
Workflows

/home/lufuno/Documents/Cline/Hooks:

/home/lufuno/Documents/Cline/Workflows:

/home/lufuno/Documents/Image-Line:
Downloads
FL Studio
FLEX
Maximus

/home/lufuno/Documents/Image-Line/Downloads:
FL Studio Mobile Factory Data
Languages
Loop starter scripts
Loop starter scripts.ini

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data:
3xOsc Shapes
Chord Sets
CirclePixelShader.cso
DLL
DirectWave Samples
DirectWave Samples EXP
Drum Samples
Drum Templates
Effect Presets
FeatherPixelShader.cso
FilterTable_Ctf_Res.data
FontPixelShader.cso
Fonts
GMSynthWaveshapes
Keyboard Shortcuts
Loop Samples
New Song Templates
Rack Presets
SamplePixelShader.cso
SampleVertexShader.cso
Songs
SoundFonts
Synth Presets2
TexturePixelShader.cso
gamepad.png
gamepad_fill.png
oscdata.dat
samplesList.data
title.png
wrap.png

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/3xOsc Shapes:
HQ
Misc

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/3xOsc Shapes/HQ:
HQ_Ramp.wav
HQ_RoundSaw.wav
HQ_Saw.wav
HQ_Sine_Dist.wav
HQ_SmoothInvSaw.wav
HQ_SmoothInvSaw_2.wav
HQ_SmoothInvSaw_Dist.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/3xOsc Shapes/Misc:
SHP_Additive.wav
SHP_Additive_2.wav
SHP_BungList.wav
SHP_BungList_2.wav
SHP_BungList_3.wav
SHP_BungList_4.wav
SHP_BungList_5.wav
SHP_BungList_6.wav
SHP_BungList_7.wav
SHP_BungList_8.wav
SHP_BungList_9.wav
SHP_Detuned Saw.wav
SHP_Harmonics.wav
SHP_Moog.wav
SHP_Noise Bass.wav
SHP_Noise.wav
SHP_Overtone.wav
SHP_Overtone_2.wav
SHP_Ramp Saw.wav
SHP_Ramp.wav
SHP_Rand.wav
SHP_Rand_2.wav
SHP_Rough Square.wav
SHP_Rough Step.wav
SHP_Rough Step_2.wav
SHP_Saw+Sine.wav
SHP_Sawing Synth.wav
SHP_Synth.wav
SHP_Thin Pulse.wav
SHP_Thin.wav
SHP_Vowel A.wav
SHP_Vowel U.wav
SHP_Wire Scan.wav
SHP_Wooble Synth.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Chord Sets:
Minor 1.chords
Minor 2.chords
Minor 3.chords
Minor 4.chords
Minor 5.chords
Minor 6.chords
Minor 7.chords
Minor 8.chords

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DLL:
FLMManagedBridge_x64.dll
FLMManagedBridge_x64.dll.metagen

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples:
Guitar
Keyboard
Orchestral
Synth Key

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Guitar:
Nylon Guitar
Nylon Guitar.dwp
Picked Bass
Picked Bass.dwp

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Guitar/Nylon Guitar:
Nylon Guitar a3.wav
Nylon Guitar b6.wav
Nylon Guitar d4.wav
Nylon Guitar d5.wav
Nylon Guitar d6.wav
Nylon Guitar g4.wav
Nylon Guitar g5.wav
Nylon Guitar g6.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Guitar/Picked Bass:
5String BopBass Pick_C1.wav
5String BopBass Pick_C2.wav
5String BopBass Pick_C3.wav
5String BopBass Pick_E1.wav
5String BopBass Pick_E2.wav
5String BopBass Pick_E3.wav
5String BopBass Pick_G1.wav
5String BopBass Pick_G2.wav
5String BopBass Pick_G3.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Keyboard:
Electric
Electric.dwp
Sytrus Rhodes Piano
Sytrus Rhodes Piano.dwp

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Keyboard/Electric:
Piano Electric (1).wav
Piano Electric (2).wav
Piano Electric (3).wav
Piano Electric (4).wav
Piano Electric (5).wav
Piano Electric (6).wav
Piano Electric (7).wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Keyboard/Sytrus Rhodes Piano:
Rhodes Piano (1).wav
Rhodes Piano (10).wav
Rhodes Piano (11).wav
Rhodes Piano (12).wav
Rhodes Piano (2).wav
Rhodes Piano (3).wav
Rhodes Piano (4).wav
Rhodes Piano (5).wav
Rhodes Piano (6).wav
Rhodes Piano (7).wav
Rhodes Piano (8).wav
Rhodes Piano (9).wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Orchestral:
Strings Section
Strings Section.dwp

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Orchestral/Strings Section:
OSTR C1.wav
OSTR C2.wav
OSTR C3.wav
OSTR C4.wav
OSTR C5.wav
OSTR C6.wav
OSTR F1.wav
OSTR F2.wav
OSTR F3.wav
OSTR F4.wav
OSTR F5.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Synth Key:
Club Pluck
Club Pluck.dwp

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples/Synth Key/Club Pluck:
SY - Club Pluck C2.wav
SY - Club Pluck C3.wav
SY - Club Pluck C4.wav
SY - Club Pluck G2.wav
SY - Club Pluck G3.wav
SY - Club Pluck G4.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/DirectWave Samples EXP:

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Drum Samples:
Cymbal
Hat
Kick
Percussion
SFX
Snare
Tom

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Drum Samples/Cymbal:
FL 808 Crash.wav
FL 909 Crash.wav
FL 909 Ride.wav
FL Grv Crash 02.wav
FL Grv Ride 03.wav
FL Jung Ride.wav
HouseGen Crash 02.wav
HouseGen Crash 05.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Drum Samples/Hat:
Attack Hat 06.wav
Attack OHat 02.wav
Downstream CHat 03.wav
Downstream CHat 08.wav
FL 808 CH.wav
FL 808 OH.wav
FL 909 CH 1.wav
FL 909 CH 2.wav
FL 909 OH.wav
FL AMX CH 2.wav
FL Basic Hat.wav
FL Bracke CH 2.wav
FL Club OH 2.wav
FL Grv CH 01.wav
FL Grv CH 03.wav
FL Grv CH 06.wav
FL Grv CH 08.wav
FL Grv OH 03.wav
HouseGen CHat 01.wav
HouseGen CHat 02.wav
HouseGen CHat 03.wav
HouseGen OHat 12.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Drum Samples/Kick:
Attack Kick 15.wav
Attack Kick 46.wav
Downstream Kick 04.wav
FL 808 Kick.wav
FL 909 Kick Alt.wav
FL 909 Kick.wav
FL Basic Kick.wav
FL Grv Kick 06.wav
FL Grv Kick 28.wav
FL Grv Kick 29.wav
FL Grv Kick 30.wav
Power ShortKick 13.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Drum Samples/Percussion:
Attack Blip 03.wav
Attack Shaker 03.wav
Attack Shaker 04.wav
Attack Shaker 09.wav
Attack Shaker 10.wav
FL 808 Clav.wav
FL 808 Conga.wav
FL 808 Cowbell.wav
FL 909 Clap.wav
FL 909 Clav.wav
FL DNC Clap 1.wav
FL GM Cabasa.wav
FL RD Tamb 2.wav
MA Sideways Perc 01.wav
MA Sideways Perc 02.wav

/home/lufuno/Documents/Image-Line/Downloads/FL Studio Mobile Factory Data/Drum Samples/SFX:
FL Alma Radio 3 SFX.wav
FL Cymbal Drop.wav
FL Cymbal Rise.wav
FL FM Drop.
```

### TOOL: shell [seq 5] 2026-11-09 13:46:51 status=completed

```json
{
  "command": "find /home/lufuno -maxdepth 4 -type f -mtime -7 -not -path \"*/.*\" 2>/dev/null | head -n 400; echo ===COUNT===; find /home/lufuno -maxdepth 4 -type f -mtime -7 -not -path \"*/.*\" 2>/dev/null | wc -l"
}
```

```
/home/lufuno/last-de-cluade-prompt.txt
/home/lufuno/Cargo.lock
/home/lufuno/qwen-last.txt
/home/lufuno/work/BIGBOY/run-egui-wasm-showcase-rush2.zip
/home/lufuno/work/BIGBOY/i-logdrum.jpeg
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-last.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-rush3-plugin.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin2.zip
/home/lufuno/work/BIGBOY/initial-user-greeting.2.zip
/home/lufuno/work/BIGBOY/1793699139844-73986115-906c-4458-8744-be1b105fee51-01a1a0cc-3fef-79ef-94a4-02c273d9e26b.png
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin-rush5-dx10 needs fix.zip
/home/lufuno/work/BIGBOY/premium-skeuomorphic-daw-prototype-bad.zip
/home/lufuno/work/BIGBOY/recreate-file-manager-in-rust-bad.zip
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/index.html
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/vite.config.ts
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/package-lock.json
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/tsconfig.json
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2/package.json
/home/lufuno/work/BIGBOY/seedream-5.0-pro_a_create_a_premium_ui_.jpeg
/home/lufuno/work/BIGBOY/run-egui-wasm-showcase-.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush4.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush6.zip
/home/lufuno/work/BIGBOY/gpt-image-2.5-sunburst_b_remove_the_sounds_fr.png
/home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin.zip
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin-rush.zip
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin-rush4.zip
/home/lufuno/work/BIGBOY/run-egui-wasm-showcase-rush3.zip
/home/lufuno/work/BIGBOY/initial-user-greeting.zip
/home/lufuno/work/BIGBOY/recreate-file-manager-in-rust-good.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush7.zip
/home/lufuno/work/BIGBOY/premium-skeuomorphic-daw-prototype.zip
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin-4.zip
/home/lufuno/work/BIGBOY/1793698688326-c77aaa75-17e1-4f22-9b36-f7f1f3f14e5b-01a1a0c5-55b8-7274-8641-0f73b6b59452.png
/home/lufuno/work/BIGBOY/premium-skeuomorphic-daw-prototype-2-different.zip
/home/lufuno/work/BIGBOY/recreate-file-manager-in-rust-2.zip
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness.zip
/home/lufuno/work/BIGBOY/run-egui-wasm-showcase-rush.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-3.zip
/home/lufuno/work/BIGBOY/Smart Recording 02-02-24.webm
/home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip
/home/lufuno/work/BIGBOY/gpt-image-2.5-sunburst_a_add_a_piano_and_a_pr.png
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good/index.html
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good/vite.config.ts
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good/package-lock.json
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good/package.json
/home/lufuno/work/BIGBOY/run-egui-wasm-showcase-e-log.zip
/home/lufuno/work/BIGBOY/1793698466656-29156505-7dc0-4fe0-8ebe-70a264c527d9-01a1a0c1-3e34-7c0d-8edb-74cc950f032f.jpeg
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin.zip
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin-05.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush.zip
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-good.zip
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements.zip
/home/lufuno/work/BIGBOY/run-egui-wasm-showcase.zip
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-good1.zip
/home/lufuno/work/BIGBOY/rust-wasm-ui-refinements2.zip
/home/lufuno/work/BIGBOY/recreate-file-manager-in-rust.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush5.zip
/home/lufuno/work/BIGBOY/high-fidelity-skeuomorphic-daw-prototype.zip
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/index.html
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/vite.config.ts
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/package-lock.json
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/tsconfig.json
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager-last/package.json
/home/lufuno/work/BIGBOY/gpt-image-2.5-sunburst_b_I_am_planning_to_bui.png
/home/lufuno/work/BIGBOY/Smart Voice 01-14-37.ogg
/home/lufuno/work/BIGBOY/build-cross-platform-rust-file-manager.zip
/home/lufuno/work/BIGBOY/initial-user-greeting.2.1.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush2.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush5 broken.zip
/home/lufuno/work/BIGBOY/improve-app-consistency-and-completeness-good.zip
/home/lufuno/work/BIGBOY/build-skeuomorphic-amapiano-logdrum-plugin-rush5 broken2.zip
/home/lufuno/work/BIGBOY/develop-amapiano-logdrum-plugin-rush5.zip
/home/lufuno/work/BIGBOY/premium-skeuomorphic-daw-prototype-bad2.zip
/home/lufuno/file-de-cluade.txt
/home/lufuno/README.md
/home/lufuno/last-de-cluade.txt
/home/lufuno/Documents/Screenshot 2026-11-05 at 02-00-19 Smart Desktop.png
/home/lufuno/Documents/rust-wasm-media-manager-2.zip
/home/lufuno/Documents/Fightland.S01.720P.MoviesMod.zone.zip
/home/lufuno/Documents/eiga.png
/home/lufuno/Documents/Screenshot 2026-11-05 at 01-53-19 Smart Desktop.png
/home/lufuno/Documents/rust-wasm-media-manager-latest.zip
/home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.lock
/home/lufuno/Documents/rust-wasm-media-manager-latest/README.md
/home/lufuno/Documents/rust-wasm-media-manager-latest/lint-stub.ts
/home/lufuno/Documents/rust-wasm-media-manager-latest/index.html
/home/lufuno/Documents/rust-wasm-media-manager-latest/vite.config.ts
/home/lufuno/Documents/rust-wasm-media-manager-latest/target/CACHEDIR.TAG
/home/lufuno/Documents/rust-wasm-media-manager-latest/package-lock.json
/home/lufuno/Documents/rust-wasm-media-manager-latest/tools/gen_icons.py
/home/lufuno/Documents/rust-wasm-media-manager-latest/tools/build_web.sh
/home/lufuno/Documents/rust-wasm-media-manager-latest/assets/tokens.toml
/home/lufuno/Documents/rust-wasm-media-manager-latest/assets/icon_names.txt
/home/lufuno/Documents/rust-wasm-media-manager-latest/tsconfig.json
/home/lufuno/Documents/rust-wasm-media-manager-latest/package.json
/home/lufuno/Documents/rust-wasm-media-manager-latest/web/dev.html
/home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.toml
/home/lufuno/Documents/rust-wasm-media-manager.zip
/home/lufuno/Documents/SmartShell-x86_64.AppImage
/home/lufuno/Documents/Screenshot 2026-11-05 at 11-13-07 Smart Desktop.png
/home/lufuno/Documents/ventoy-1.1.10-linux/ventoy-1.1.10/log.txt
/home/lufuno/Documents/smart-files.png
/home/lufuno/lint-stub.ts
/home/lufuno/index.html
/home/lufuno/smart/smart-protocol-official/drafts/daw0.html
/home/lufuno/smart/smart-protocol-official/drafts/daw01.html
/home/lufuno/smart/smart-protocol-official/drafts/daw1.html
/home/lufuno/smart/bantu-labs/Cargo.lock
/home/lufuno/smart/bantu-labs/target/CACHEDIR.TAG
/home/lufuno/smart/bantu-labs/Cargo.toml
/home/lufuno/smart/cluade.md
/home/lufuno/smart/prototypes/Xplore/Cargo.lock
/home/lufuno/smart/prototypes/Xplore/Cargo.toml
/home/lufuno/smart/smart-suite-official/specs/CANONICAL-SPECS.md
/home/lufuno/smart/smart-suite-official/audits/CORE-SPECS.md
/home/lufuno/smart/smart-suite-official/audits/UX-SPECS.md
/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager.zip
/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip
/home/lufuno/smart/smart-suite/apps/develop-rust-wasm-file-manager.zip
/home/lufuno/smart/smart-suite/apps/build-production-rust-file-manager-2.zip
/home/lufuno/smart/smart-suite/prom
```

### TOOL: shell [seq 24] 2026-11-09 13:46:54 status=completed

```json
{
  "command": "ls -lh /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/ 2>&1; echo ---; ls -lh /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/ 2>&1; echo ---; ls -lh /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/ 2>&1; echo ---; ls -lh /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/ 2>&1\nmkdir -p /tmp/opencode/canonical-eiga-audit\nls -lh /home/lufuno/smart/smart-suite-official/ 2>&1\nls -lh /home/lufuno/smart/smart-protocol-official/ 2>&1 | head -n 100"
}
```

```
total 84K
-rw-rw-r-- 1 lufuno lufuno 3.0K Nov  6 17:02 lib.rs
-rw-rw-r-- 1 lufuno lufuno  30K Nov  6 17:02 player.rs
-rw-rw-r-- 1 lufuno lufuno  30K Nov  6 17:02 source.rs
-rw-rw-r-- 1 lufuno lufuno  15K Nov  6 17:02 subs.rs
---
total 232K
-rw-rw-r-- 1 lufuno lufuno  31K Nov  6 17:02 analyze.rs
-rw-rw-r-- 1 lufuno lufuno 8.8K Nov  6 17:02 apps.rs
-rw-rw-r-- 1 lufuno lufuno  18K Nov  6 17:02 archive.rs
-rw-rw-r-- 1 lufuno lufuno 6.5K Nov  6 17:02 audio.rs
-rw-rw-r-- 1 lufuno lufuno  30K Nov  6 17:02 backend.rs
-rw-rw-r-- 1 lufuno lufuno  13K Nov  6 17:02 demo.rs
-rw-rw-r-- 1 lufuno lufuno  17K Nov  6 17:02 devices.rs
-rw-rw-r-- 1 lufuno lufuno  428 Nov  6 17:02 exec.rs
-rw-rw-r-- 1 lufuno lufuno 1.2K Nov  6 17:02 lib.rs
-rw-rw-r-- 1 lufuno lufuno 6.1K Nov  6 17:02 library.rs
-rw-rw-r-- 1 lufuno lufuno 3.5K Nov  6 17:02 local.rs
-rw-rw-r-- 1 lufuno lufuno 6.5K Nov  6 17:02 memfs.rs
-rw-rw-r-- 1 lufuno lufuno  12K Nov  6 17:02 model.rs
-rw-rw-r-- 1 lufuno lufuno 8.7K Nov  6 17:02 ops.rs
-rw-rw-r-- 1 lufuno lufuno  783 Nov  6 17:02 registry.rs
-rw-rw-r-- 1 lufuno lufuno  22K Nov  6 17:02 search.rs
-rw-rw-r-- 1 lufuno lufuno 3.2K Nov  6 17:02 thumbs.rs
-rw-rw-r-- 1 lufuno lufuno 2.3K Nov  6 17:02 tools.rs
-rw-rw-r-- 1 lufuno lufuno 2.8K Nov  6 17:02 vfs.rs
---
total 928K
-rw-rw-r-- 1 lufuno lufuno  67K Nov  6 17:02 app.rs
-rw-rw-r-- 1 lufuno lufuno  35K Nov  6 17:02 appkit.rs
-rw-rw-r-- 1 lufuno lufuno  71K Nov  6 17:02 chrome.rs
-rw-rw-r-- 1 lufuno lufuno  46K Nov  6 17:02 content.rs
-rw-rw-r-- 1 lufuno lufuno  16K Nov  6 17:02 copyfx.rs
-rw-rw-r-- 1 lufuno lufuno  14K Nov  6 17:02 highlight.rs
-rw-rw-r-- 1 lufuno lufuno 2.0K Nov  6 17:02 icons.rs
-rw-rw-r-- 1 lufuno lufuno  40K Nov  6 17:02 icons_gen.rs
-rw-rw-r-- 1 lufuno lufuno  21K Nov  6 17:02 lib.rs
-rw-rw-r-- 1 lufuno lufuno   64 Nov  6 17:02 main.rs
-rw-rw-r-- 1 lufuno lufuno 100K Nov  6 17:02 modals.rs
-rw-rw-r-- 1 lufuno lufuno  74K Nov  6 17:02 overlays.rs
-rw-rw-r-- 1 lufuno lufuno  11K Nov  6 17:02 paint.rs
-rw-rw-r-- 1 lufuno lufuno  74K Nov  6 17:02 photos.rs
-rw-rw-r-- 1 lufuno lufuno  16K Nov  6 17:02 pty.rs
-rw-rw-r-- 1 lufuno lufuno  42K Nov  6 17:02 shell.rs
-rw-rw-r-- 1 lufuno lufuno  54K Nov  6 17:02 storage.rs
-rw-rw-r-- 1 lufuno lufuno 120K Nov  6 17:02 terminal.rs
-rw-rw-r-- 1 lufuno lufuno  16K Nov  6 17:02 termlive.rs
-rw-rw-r-- 1 lufuno lufuno 8.0K Nov  6 17:02 theme.rs
-rw-rw-r-- 1 lufuno lufuno  75K Nov  6 17:02 video.rs
---
total 16K
drwxrwxr-x 3 lufuno lufuno 4.0K Nov  6 17:02 eiga-core
drwxrwxr-x 3 lufuno lufuno 4.0K Nov  6 17:02 smart-core
drwxrwxr-x 3 lufuno lufuno 4.0K Nov  6 17:02 smart-ui-egui
drwxrwxr-x 3 lufuno lufuno 4.0K Nov  6 17:02 xtask
total 8.0K
drwxr-xr-x 2 lufuno lufuno 4.0K Nov  9 13:45 audits
drwxr-xr-x 2 lufuno lufuno 4.0K Nov  9 13:45 specs
total 100K
-rwxr-xr-x 1 lufuno lufuno 9.9K Oct 23 16:54 BINDING_CONVENTIONS.md
-rwxr-xr-x 1 lufuno lufuno  47K Oct 23 16:55 DESIGN_PRINCIPLES.md
-rw-r--r-- 1 lufuno lufuno  14K Oct 23 16:55 README.md
drwxr-xr-x 9 lufuno lufuno 4.0K Oct 28 18:52 audit-revamp
drwxr-xr-x 2 lufuno lufuno 4.0K Oct 23 16:40 docs
drwxr-xr-x 3 lufuno lufuno 4.0K Nov  2 17:51 drafts
drwxr-xr-x 3 lufuno lufuno 4.0K Oct 11 22:51 gen
drwxr-xr-x 2 lufuno lufuno 4.0K Oct 10 17:38 schema
drwxr-xr-x 3 lufuno lufuno 4.0K Oct 10 16:35 scripts

```

### TOOL: shell [seq 33] 2026-11-09 13:46:58 status=completed

```json
{
  "command": "ls -lh /home/lufuno/smart/smart-suite-official/specs/ /home/lufuno/smart/smart-suite-official/audits/ 2>&1\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.toml 2>&1\necho ===; cat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/Cargo.toml 2>&1; echo ===; cat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/Cargo.toml 2>&1; echo ===; cat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml 2>&1"
}
```

```
/home/lufuno/smart/smart-suite-official/audits/:
total 220K
-rw-r--r-- 1 lufuno lufuno 111K Nov  9 13:45 CORE-SPECS.md
-rw-r--r-- 1 lufuno lufuno 107K Nov  9 13:45 UX-SPECS.md

/home/lufuno/smart/smart-suite-official/specs/:
total 12K
-rw-r--r-- 1 lufuno lufuno 12K Nov  9 13:45 CANONICAL-SPECS.md
[workspace]
resolver = "2"
members = ["crates/smart-core", "crates/eiga-core", "crates/smart-ui-egui", "crates/xtask"]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

[profile.wasm]
inherits = "release"
opt-level = "s"
lto = true

[profile.dev]
opt-level = 1
===
[package]
name = "eiga-core"
description = "Eiga (映画) engine: playback state machine, playlist, key map, subtitle parsers, ffprobe parsing and frame sources. No UI code."
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde_json = "1"
===
[package]
name = "smart-core"
description = "Smart Files backend: virtual file systems, search, jobs, devices, apps — no UI code."
version.workspace = true
edition.workspace = true
license.workspace = true

[features]
default = ["demo"]
demo = []          # seeded in-memory file system used by the web build
audio = ["dep:rodio"]
cloud = []         # placeholder for opendal-backed providers (see src/cloud.rs)

[dependencies]
chrono = { version = "0.4", default-features = false, features = ["clock", "std", "wasmbind"] }
image = { version = "0.25", default-features = false, features = ["jpeg", "png", "gif", "webp", "bmp"] }
memchr = "2"
mime_guess = "2"
miniz_oxide = "0.9"
nucleo-matcher = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
dirs = "7"
notify = "8"
rayon = "1"
sysinfo = { version = "0.40", default-features = false, features = ["disk"] }
trash = "5"
rodio = { version = "0.23", default-features = false, features = ["playback", "symphonia-all"], optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
js-sys = "0.3"
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Blob", "BlobPropertyBag", "Url", "HtmlAudioElement", "HtmlMediaElement", "Window", "Storage"] }
===
[package]
name = "smart-ui-egui"
description = "egui frontend for Smart Files (desktop + WebAssembly)"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "smart-files"
path = "src/main.rs"

[features]
default = ["audio"]
audio = ["smart-core/audio"]

[dependencies]
smart-core = { path = "../smart-core", default-features = true }
eframe = { version = "0.37", default-features = false, features = ["glow", "default_fonts", "x11", "wayland"] }
egui = "0.37"
vt100 = "0.15"
image = { version = "0.25", default-features = false, features = ["png", "jpeg"] }
eiga-core = { version = "0.1.0", path = "../eiga-core" }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Window", "Document", "HtmlCanvasElement", "Element", "console", "Location"] }
console_error_panic_hook = "0.1"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
portable-pty = "0.9.0"
tree-sitter = "0.27.1"
tree-sitter-highlight = "0.27.1"
tree-sitter-rust = "0.24.2"
tree-sitter-python = "0.25.0"
tree-sitter-javascript = "0.25.0"
tree-sitter-json = "0.24.8"
tree-sitter-toml-ng = "0.7.0"
tree-sitter-bash = "0.25.1"
tree-sitter-css = "0.25.0"
tree-sitter-html = "0.23.2"
tree-sitter-c = "0.24.2"
tree-sitter-go = "0.25.0"
tree-sitter-md = "0.5.4"
rodio = { version = "0.23", default-features = false, features = ["playback"] }
rfd = "0.15"

```

### TOOL: read [seq 40] 2026-11-09 13:47:00 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs, lines 1-92
1: //! # eiga-core — the engine behind **Eiga (映画)**, the Smart suite's video player.
2: //!
3: //! * [`subs`]   SRT / WebVTT / ASS-SSA parsing, encoding detection, fast "what is on screen now" lookup.
4: //! * [`player`] pure playback state machine (seek, speed, A-B loop, chapters, frame-step), playlist and key map.
5: //! * [`source`] ffprobe parsing, a procedural test-card source (works everywhere, incl. WebAssembly) and — on native
6: //!   targets — an `ffmpeg` pipe source that decodes any format ffmpeg knows.
7: //!
8: //! Nothing in this crate draws, opens windows or reads clocks, so every behaviour is unit-tested.
9: pub mod player;
10: pub mod source;
11: pub mod subs;
12: 
13: pub use player::{Cmd, Event, LoopMode, Player, Playlist, Repeat};
14: pub use source::{Frame, MediaInfo};
15: pub use subs::{Cue, SubTrack};
16: 
17: /// `83.4` → `1:23`, `3723` → `1:02:03`.
18: pub fn fmt_time(secs: f64) -> String {
19:     if !secs.is_finite() || secs < 0.0 {
20:         return "0:00".into();
21:     }
22:     let s = secs as u64;
23:     let (h, m, s) = (s / 3600, (s / 60) % 60, s % 60);
24:     if h > 0 {
25:         format!("{h}:{m:02}:{s:02}")
26:     } else {
27:         format!("{m}:{s:02}")
28:     }
29: }
30: 
31: /// Like [`fmt_time`] with milliseconds, used for sync offsets and the stats overlay.
32: pub fn fmt_time_ms(secs: f64) -> String {
33:     if !secs.is_finite() || secs < 0.0 {
34:         return "0:00.000".into();
35:     }
36:     let ms = (secs * 1000.0).round() as u64;
37:     let (h, m, s, ms) = (ms / 3_600_000, (ms / 60_000) % 60, (ms / 1000) % 60, ms % 1000);
38:     if h > 0 {
39:         format!("{h}:{m:02}:{s:02}.{ms:03}")
40:     } else {
41:         format!("{m}:{s:02}.{ms:03}")
42:     }
43: }
44: 
45: /// Parses `90`, `1:30`, `1:02:03.5`. Used by "jump to time".
46: pub fn parse_time(s: &str) -> Option<f64> {
47:     let s = s.trim();
48:     if s.is_empty() {
49:         return None;
50:     }
51:     let mut total = 0.0;
52:     for part in s.split(':') {
53:         let v: f64 = part.trim().parse().ok()?;
54:         if !v.is_finite() || v < 0.0 {
55:             return None;
56:         }
57:         total = total * 60.0 + v;
58:     }
59:     if s.split(':').count() > 3 {
60:         return None;
61:     }
62:     Some(total)
63: }
64: 
65: #[cfg(test)]
66: mod tests {
67:     use super::*;
68: 
69:     #[test]
70:     fn time_formatting() {
71:         assert_eq!(fmt_time(0.0), "0:00");
72:         assert_eq!(fmt_time(83.9), "1:23");
73:         assert_eq!(fmt_time(3723.0), "1:02:03");
74:         assert_eq!(fmt_time(f64::NAN), "0:00");
75:         assert_eq!(fmt_time(-4.0), "0:00");
76:         assert_eq!(fmt_time_ms(3723.456), "1:02:03.456");
77:         assert_eq!(fmt_time_ms(5.5), "0:05.500");
78:     }
79: 
80:     #[test]
81:     fn time_parsing() {
82:         assert_eq!(parse_time("90"), Some(90.0));
83:         assert_eq!(parse_time("1:30"), Some(90.0));
84:         assert_eq!(parse_time("1:02:03.5"), Some(3723.5));
85:         assert_eq!(parse_time(" 0:05 "), Some(5.0));
86:         assert_eq!(parse_time(""), None);
87:         assert_eq!(parse_time("a:b"), None);
88:         assert_eq!(parse_time("-3"), None);
89:         assert_eq!(parse_time("1:2:3:4"), None);
90:         assert_eq!(parse_time("inf"), None);
91:     }
92: }
```

### TOOL: read [seq 40] 2026-11-09 13:47:00 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs, lines 1-820
1: //! Playback state machine, playlist and key map. Pure logic: feed it `tick(dt)`, read back [`Event`]s.
2: use crate::subs::SubTrack;
3: 
4: pub const SPEEDS: [f64; 10] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];
5: pub const MAX_VOLUME: f32 = 130.0;
6: 
7: #[derive(Clone, Copy, PartialEq, Debug)]
8: pub enum LoopMode {
9:     Off,
10:     /// Repeat the current file.
11:     File,
12:     /// Repeat between A and B. `b` is `None` while the user has only set A.
13:     Ab { a: f64, b: Option<f64> },
14: }
15: 
16: /// What the engine/UI must do in response to a state change.
17: #[derive(Clone, Copy, PartialEq, Debug)]
18: pub enum Event {
19:     /// Restart decoding at this media time.
20:     Seek(f64),
21:     Ended,
22:     PauseChanged(bool),
23:     SpeedChanged(f64),
24:     VolumeChanged,
25:     /// Short message for the on-screen display ("Speed ×1.25").
26:     Osd,
27: }
28: 
29: #[derive(Clone, Copy, PartialEq, Debug)]
30: pub struct Chapter {
31:     pub start: f64,
32: }
33: 
34: #[derive(Clone, Debug)]
35: pub struct Player {
36:     pub duration: f64,
37:     pub position: f64,
38:     pub paused: bool,
39:     pub speed: f64,
40:     pub volume: f32,
41:     pub muted: bool,
42:     pub looping: LoopMode,
43:     pub eof: bool,
44:     pub fps: f64,
45:     pub audio_delay: f64,
46:     pub chapters: Vec<f64>,
47:     pub osd: Option<String>,
48: }
49: 
50: impl Default for Player {
51:     fn default() -> Self {
52:         Player { duration: 0.0, position: 0.0, paused: false, speed: 1.0, volume: 100.0, muted: false, looping: LoopMode::Off, eof: false, fps: 25.0, audio_delay: 0.0, chapters: vec![], osd: None }
53:     }
54: }
55: 
56: impl Player {
57:     pub fn new(duration: f64, fps: f64) -> Player {
58:         Player { duration: if duration.is_finite() { duration.max(0.0) } else { 0.0 }, fps: if fps.is_finite() && fps > 1.0 && fps < 480.0 { fps } else { 25.0 }, ..Player::default() }
59:     }
60: 
61:     fn say(&mut self, s: impl Into<String>) {
62:         self.osd = Some(s.into());
63:     }
64: 
65:     pub fn clamp_pos(&self, t: f64) -> f64 {
66:         if !t.is_finite() {
67:             return 0.0;
68:         }
69:         if self.duration > 0.0 {
70:             t.clamp(0.0, (self.duration - 0.001).max(0.0))
71:         } else {
72:             t.max(0.0)
73:         }
74:     }
75: 
76:     pub fn fraction(&self) -> f64 {
77:         if self.duration > 0.0 {
78:             (self.position / self.duration).clamp(0.0, 1.0)
79:         } else {
80:             0.0
81:         }
82:     }
83: 
84:     /// Advance the clock. Returns events (A-B wrap, loop, end of file).
85:     pub fn tick(&mut self, dt: f64) -> Vec<Event> {
86:         let mut ev = vec![];
87:         if self.paused || self.eof || !dt.is_finite() || dt <= 0.0 {
88:             return ev;
89:         }
90:         let dt = dt.min(0.25); // a stalled frame must not skip half a minute of video
91:         self.position += dt * self.speed;
92:         if let LoopMode::Ab { a, b: Some(b) } = self.looping {
93:             if self.position >= b {
94:                 self.position = a;
95:                 ev.push(Event::Seek(a));
96:                 return ev;
97:             }
98:         }
99:         if self.duration > 0.0 && self.position >= self.duration {
100:             if self.looping == LoopMode::File {
101:                 self.position = 0.0;
102:                 ev.push(Event::Seek(0.0));
103:             } else {
104:                 self.position = self.duration;
105:                 self.eof = true;
106:                 self.paused = true;
107:                 ev.push(Event::Ended);
108:             }
109:         }
110:         ev
111:     }
112: 
113:     pub fn seek_abs(&mut self, t: f64) -> Vec<Event> {
114:         self.position = self.clamp_pos(t);
115:         self.eof = false;
116:         vec![Event::Seek(self.position)]
117:     }
118:     pub fn seek_rel(&mut self, d: f64) -> Vec<Event> {
119:         let was_eof = self.eof;
120:         let base = if was_eof { self.duration } else { self.position };
121:         let ev = self.seek_abs(base + d);
122:         self.say(format!("{}{}s", if d >= 0.0 { "+" } else { "\u{2212}" }, d.abs()));
123:         ev
124:     }
125:     pub fn seek_frac(&mut self, f: f64) -> Vec<Event> {
126:         self.seek_abs(f.clamp(0.0, 1.0) * self.duration)
127:     }
128: 
129:     pub fn set_paused(&mut self, p: bool) -> Vec<Event> {
130:         if self.eof && !p {
131:             // pressing play at the end restarts, like every player
132:             self.position = 0.0;
133:             self.eof = false;
134:             self.paused = false;
135:             return vec![Event::Seek(0.0), Event::PauseChanged(false)];
136:         }
137:         if self.paused == p {
138:             return vec![];
139:         }
140:         self.paused = p;
141:         vec![Event::PauseChanged(p)]
142:     }
143:     pub fn toggle_pause(&mut self) -> Vec<Event> {
144:         let p = !self.paused;
145:         self.set_paused(p)
146:     }
147: 
148:     /// Step one frame (mpv `.` and `,`). Pauses first.
149:     pub fn frame_step(&mut self, dir: i32) -> Vec<Event> {
150:         let mut ev = vec![];
151:         if !self.paused {
152:             self.paused = true;
153:             ev.push(Event::PauseChanged(true));
154:         }
155:         let t = self.position + dir as f64 / self.fps;
156:         ev.extend(self.seek_abs(t));
157:         self.say(if dir >= 0 { "Next frame" } else { "Previous frame" });
158:         ev
159:     }
160: 
161:     pub fn set_speed(&mut self, s: f64) -> Vec<Event> {
162:         self.speed = if s.is_finite() { s.clamp(0.1, 8.0) } else { 1.0 };
163:         let msg = format!("Speed \u{d7}{:.2}", self.speed);
164:         self.say(msg);
165:         vec![Event::SpeedChanged(self.speed)]
166:     }
167:     /// mpv: `]` ×1.1, `[` ×(1/1.1)
168:     pub fn speed_scale(&mut self, f: f64) -> Vec<Event> {
169:         let s = (self.speed * f * 100.0).round() / 100.0;
170:         self.set_speed(s)
171:     }
172:     /// Move through the preset ladder ({/} in mpv halve/double; here ↑/↓ nudges along presets).
173:     pub fn speed_step(&mut self, up: bool) -> Vec<Event> {
174:         let cur = self.speed;
175:         let next = if up { SPEEDS.iter().cloned().find(|s| *s > cur + 1e-6) } else { SPEEDS.iter().rev().cloned().find(|s| *s < cur - 1e-6) };
176:         match next {
177:             Some(s) => self.set_speed(s),
178:             None => vec![],
179:         }
180:     }
181: 
182:     pub fn volume_step(&mut self, d: f32) -> Vec<Event> {
183:         self.volume = (self.volume + d).clamp(0.0, MAX_VOLUME);
184:         if d > 0.0 {
185:             self.muted = false;
186:         }
187:         let v = self.volume.round();
188:         self.say(format!("Volume {v}%"));
189:         vec![Event::VolumeChanged]
190:     }
191:     pub fn toggle_mute(&mut self) -> Vec<Event> {
192:         self.muted = !self.muted;
193:         self.say(if self.muted { "Muted".to_string() } else { "Unmuted".to_string() });
194:         vec![Event::VolumeChanged]
195:     }
196:     /// 0..1 gain for the output device (volume above 100 amplifies).
197:     pub fn gain(&self) -> f32 {
198:         if self.muted {
199:             0.0
200:         } else {
201:             self.volume / 100.0
202:         }
203:     }
204: 
205:     /// mpv `l`: first press sets A, second sets B (and starts looping), third clears.
206:     pub fn cycle_ab(&mut self) {
207:         self.looping = match self.looping {
208:             LoopMode::Ab { a, b: None } if self.position > a + 0.2 => LoopMode::Ab { a, b: Some(self.position) },
209:             LoopMode::Ab { .. } => LoopMode::Off,
210:             _ => LoopMode::Ab { a: self.position, b: None },
211:         };
212:         let m = mat
```

### TOOL: read [seq 40] 2026-11-09 13:47:00 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs, lines 1-731
1: //! Media metadata (ffprobe), frame sources (procedural test card everywhere; ffmpeg pipe on native).
2: use serde_json::Value;
3: 
4: #[derive(Clone, Debug, Default, PartialEq)]
5: pub struct VideoStream {
6:     pub index: usize,
7:     pub codec: String,
8:     pub width: u32,
9:     pub height: u32,
10:     pub fps: f64,
11:     pub bitrate: Option<u64>,
12:     pub pix_fmt: String,
13: }
14: 
15: #[derive(Clone, Debug, Default, PartialEq)]
16: pub struct AudioStream {
17:     pub index: usize,
18:     pub codec: String,
19:     pub channels: u32,
20:     pub sample_rate: u32,
21:     pub lang: String,
22:     pub title: String,
23:     pub default: bool,
24: }
25: 
26: #[derive(Clone, Debug, Default, PartialEq)]
27: pub struct SubStream {
28:     pub index: usize,
29:     pub codec: String,
30:     pub lang: String,
31:     pub title: String,
32:     pub default: bool,
33:     pub forced: bool,
34: }
35: 
36: impl SubStream {
37:     /// Text subtitles can be extracted and rendered; bitmap ones (PGS, VobSub) cannot.
38:     pub fn is_text(&self) -> bool {
39:         matches!(self.codec.as_str(), "subrip" | "srt" | "ass" | "ssa" | "webvtt" | "mov_text" | "text")
40:     }
41: }
42: 
43: #[derive(Clone, Debug, Default, PartialEq)]
44: pub struct MediaInfo {
45:     pub title: String,
46:     pub container: String,
47:     pub duration: f64,
48:     pub size: u64,
49:     pub bitrate: u64,
50:     pub video: Vec<VideoStream>,
51:     pub audio: Vec<AudioStream>,
52:     pub subs: Vec<SubStream>,
53:     pub chapters: Vec<(f64, String)>,
54: }
55: 
56: impl MediaInfo {
57:     pub fn primary_video(&self) -> Option<&VideoStream> {
58:         self.video.first()
59:     }
60:     pub fn size_wh(&self) -> (u32, u32) {
61:         self.primary_video().map(|v| (v.width, v.height)).unwrap_or((0, 0))
62:     }
63:     pub fn fps(&self) -> f64 {
64:         self.primary_video().map(|v| v.fps).filter(|f| *f > 1.0).unwrap_or(25.0)
65:     }
66: }
67: 
68: fn parse_rate(s: &str) -> f64 {
69:     match s.split_once('/') {
70:         Some((n, d)) => {
71:             let (n, d): (f64, f64) = (n.parse().unwrap_or(0.0), d.parse().unwrap_or(0.0));
72:             if d > 0.0 {
73:                 n / d
74:             } else {
75:                 0.0
76:             }
77:         }
78:         None => s.parse().unwrap_or(0.0),
79:     }
80: }
81: 
82: fn s(v: &Value, k: &str) -> String {
83:     v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string()
84: }
85: fn num(v: &Value, k: &str) -> f64 {
86:     match v.get(k) {
87:         Some(Value::String(t)) => t.parse().unwrap_or(0.0),
88:         Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
89:         _ => 0.0,
90:     }
91: }
92: 
93: /// Parses `ffprobe -print_format json -show_format -show_streams -show_chapters` output.
94: pub fn parse_ffprobe(json: &str) -> Result<MediaInfo, String> {
95:     let v: Value = serde_json::from_str(json).map_err(|e| format!("unreadable ffprobe output: {e}"))?;
96:     let mut m = MediaInfo::default();
97:     let f = v.get("format").cloned().unwrap_or(Value::Null);
98:     m.container = s(&f, "format_name").split(',').next().unwrap_or("").to_string();
99:     m.duration = num(&f, "duration");
100:     m.size = num(&f, "size") as u64;
101:     m.bitrate = num(&f, "bit_rate") as u64;
102:     m.title = f.get("tags").map(|t| s(t, "title")).unwrap_or_default();
103:     for st in v.get("streams").and_then(|x| x.as_array()).cloned().unwrap_or_default() {
104:         let index = num(&st, "index") as usize;
105:         let tags = st.get("tags").cloned().unwrap_or(Value::Null);
106:         let disp = st.get("disposition").cloned().unwrap_or(Value::Null);
107:         let flag = |k: &str| num(&disp, k) as i64 == 1;
108:         match s(&st, "codec_type").as_str() {
109:             "video" if !flag("attached_pic") => {
110:                 let mut fps = parse_rate(&s(&st, "avg_frame_rate"));
111:                 if fps <= 0.0 {
112:                     fps = parse_rate(&s(&st, "r_frame_rate"));
113:                 }
114:                 if m.duration <= 0.0 {
115:                     m.duration = num(&st, "duration");
116:                 }
117:                 m.video.push(VideoStream { index, codec: s(&st, "codec_name"), width: num(&st, "width") as u32, height: num(&st, "height") as u32, fps, bitrate: Some(num(&st, "bit_rate") as u64).filter(|b| *b > 0), pix_fmt: s(&st, "pix_fmt") });
118:             }
119:             "audio" => m.audio.push(AudioStream { index, codec: s(&st, "codec_name"), channels: num(&st, "channels") as u32, sample_rate: num(&st, "sample_rate") as u32, lang: s(&tags, "language"), title: s(&tags, "title"), default: flag("default") }),
120:             "subtitle" => m.subs.push(SubStream { index, codec: s(&st, "codec_name"), lang: s(&tags, "language"), title: s(&tags, "title"), default: flag("default"), forced: flag("forced") }),
121:             _ => {}
122:         }
123:     }
124:     for c in v.get("chapters").and_then(|x| x.as_array()).cloned().unwrap_or_default() {
125:         let t = c.get("tags").map(|t| s(t, "title")).unwrap_or_default();
126:         m.chapters.push((num(&c, "start_time"), t));
127:     }
128:     if !m.duration.is_finite() || m.duration < 0.0 {
129:         m.duration = 0.0;
130:     }
131:     if m.video.is_empty() && m.audio.is_empty() {
132:         return Err("no playable streams".into());
133:     }
134:     Ok(m)
135: }
136: 
137: // ─────────────────────────────────────────── frames ───────────────────────────────────────────
138: 
139: #[derive(Clone, Debug)]
140: pub struct Frame {
141:     pub w: usize,
142:     pub h: usize,
143:     pub rgba: Vec<u8>,
144:     pub pts: f64,
145: }
146: 
147: pub trait FrameSource: Send {
148:     fn info(&self) -> &MediaInfo;
149:     fn seek(&mut self, t: f64);
150:     /// The newest frame with `pts <= clock` that has not been returned yet.
151:     fn poll(&mut self, clock: f64) -> Option<Frame>;
152:     fn name(&self) -> &'static str;
153:     /// No more frames will ever arrive (end of file or decoder died).
154:     fn exhausted(&self) -> bool {
155:         false
156:     }
157: }
158: 
159: /// Even output dimensions that fit inside `max`, preserving aspect and never upscaling.
160: pub fn fit_size(sw: u32, sh: u32, max_w: u32, max_h: u32) -> (u32, u32) {
161:     if sw == 0 || sh == 0 {
162:         return (640, 360);
163:     }
164:     let k = (max_w as f64 / sw as f64).min(max_h as f64 / sh as f64).min(1.0);
165:     let ev = |x: f64| ((x.round() as u32).max(2)) & !1;
166:     (ev(sw as f64 * k), ev(sh as f64 * k))
167: }
168: 
169: // 5×7 digit font for the burnt-in timecode
170: const GLYPHS: [[u8; 7]; 12] = [
171:     [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
172:     [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
173:     [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
174:     [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
175:     [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
176:     [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
177:     [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
178:     [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
179:     [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
180:     [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
181:     [0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00],
182:     [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
183: ];
184: 
185: fn put(buf: &mut [u8], w: usize, x: usize, y: usize, c: [u8; 3]) {
186:     let i = (y * w + x) * 4;
187:     if i + 3 < buf.len() {
188:         buf[i] = c[0];
189:         buf[i + 1] = c[1];
190:         buf[i + 2] = c[2];
191:         buf[i + 3] = 255;
192:     }
193: }
194: 
195: fn draw_text(buf: &mut [u8], w: usize, h: usize, x0: usize, y0: usize, scale: usize, text: &str, c: [u8; 3]) {
196:     let mut x = x0;
197:     for ch in text.chars() {
198:         let g = match ch {
199:             '0'
```

### TOOL: read [seq 40] 2026-11-09 13:47:00 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/subs.rs, lines 1-401
1: //! Subtitle parsing. Accepts SRT, WebVTT and ASS/SSA, tolerates BOMs, UTF-16, CRLF, missing indices and garbage.
2: 
3: #[derive(Clone, Debug, PartialEq)]
4: pub struct Cue {
5:     pub start: f64,
6:     pub end: f64,
7:     /// Plain text, lines separated by `\n`. Markup is stripped.
8:     pub text: String,
9:     pub italic: bool,
10:     /// ASS `\an7..9` (top row) positioning.
11:     pub top: bool,
12: }
13: 
14: #[derive(Clone, Debug, Default)]
15: pub struct SubTrack {
16:     pub name: String,
17:     pub lang: String,
18:     cues: Vec<Cue>,
19:     max_dur: f64,
20:     /// Seconds added to every cue (positive = later).
21:     pub delay: f64,
22: }
23: 
24: /// Decodes subtitle bytes: UTF-8 (BOM aware), UTF-16 LE/BE with BOM, otherwise Latin-1/Windows-1252-ish.
25: pub fn decode_bytes(b: &[u8]) -> String {
26:     if b.starts_with(&[0xFF, 0xFE]) || b.starts_with(&[0xFE, 0xFF]) {
27:         let le = b[0] == 0xFF;
28:         let units: Vec<u16> = b[2..].chunks_exact(2).map(|c| if le { u16::from_le_bytes([c[0], c[1]]) } else { u16::from_be_bytes([c[0], c[1]]) }).collect();
29:         return String::from_utf16_lossy(&units);
30:     }
31:     let b = b.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(b);
32:     match std::str::from_utf8(b) {
33:         Ok(s) => s.to_string(),
34:         Err(_) => b.iter().map(|&x| x as char).collect(),
35:     }
36: }
37: 
38: /// `00:01:02,500`, `1:02.5`, `0:00:01.00`, `62.5` → seconds.
39: pub fn parse_ts(s: &str) -> Option<f64> {
40:     let s = s.trim().replace(',', ".");
41:     if s.is_empty() || s.len() > 16 {
42:         return None;
43:     }
44:     let mut total = 0.0f64;
45:     let parts: Vec<&str> = s.split(':').collect();
46:     if parts.len() > 3 {
47:         return None;
48:     }
49:     for p in &parts {
50:         let v: f64 = p.trim().parse().ok()?;
51:         if !v.is_finite() || v < 0.0 {
52:             return None;
53:         }
54:         total = total * 60.0 + v;
55:     }
56:     Some(total)
57: }
58: 
59: fn decode_entities(s: &str) -> String {
60:     s.replace("&nbsp;", " ").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&")
61: }
62: 
63: /// Strips `<i>…</i>`, `<font …>`, `{\an8}` etc. Returns (text, italic, top).
64: fn clean(raw: &str) -> (String, bool, bool) {
65:     let mut italic = false;
66:     let mut top = false;
67:     let mut out = String::with_capacity(raw.len());
68:     let mut chars = raw.chars().peekable();
69:     while let Some(c) = chars.next() {
70:         match c {
71:             '<' => {
72:                 let mut tag = String::new();
73:                 let mut closed = false;
74:                 for t in chars.by_ref() {
75:                     if t == '>' {
76:                         closed = true;
77:                         break;
78:                     }
79:                     tag.push(t);
80:                     if tag.len() > 80 {
81:                         break;
82:                     }
83:                 }
84:                 if !closed {
85:                     out.push('<');
86:                     out.push_str(&tag);
87:                 } else if tag.eq_ignore_ascii_case("i") {
88:                     italic = true;
89:                 }
90:             }
91:             '{' => {
92:                 let mut tag = String::new();
93:                 let mut closed = false;
94:                 for t in chars.by_ref() {
95:                     if t == '}' {
96:                         closed = true;
97:                         break;
98:                     }
99:                     tag.push(t);
100:                     if tag.len() > 200 {
101:                         break;
102:                     }
103:                 }
104:                 if !closed {
105:                     out.push('{');
106:                     out.push_str(&tag);
107:                 } else {
108:                     if tag.contains("\\i1") {
109:                         italic = true;
110:                     }
111:                     for n in ["\\an7", "\\an8", "\\an9"] {
112:                         if tag.contains(n) {
113:                             top = true;
114:                         }
115:                     }
116:                 }
117:             }
118:             '\\' if matches!(chars.peek(), Some('N') | Some('n')) => {
119:                 chars.next();
120:                 out.push('\n');
121:             }
122:             _ => out.push(c),
123:         }
124:     }
125:     let text = decode_entities(&out).lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n");
126:     (text, italic, top)
127: }
128: 
129: fn parse_arrow_block(lines: &[&str]) -> Option<Cue> {
130:     let ai = lines.iter().position(|l| l.contains("-->"))?;
131:     let (a, b) = lines[ai].split_once("-->")?;
132:     let start = parse_ts(a)?;
133:     // VTT cue settings follow the end time ("align:start position:10%")
134:     let b = b.trim().split_whitespace().next()?;
135:     let end = parse_ts(b)?;
136:     let (text, italic, top) = clean(&lines[ai + 1..].join("\n"));
137:     if text.is_empty() || end <= start {
138:         return None;
139:     }
140:     Some(Cue { start, end, text, italic, top })
141: }
142: 
143: fn parse_srt_vtt(src: &str) -> Vec<Cue> {
144:     let norm = src.replace("\r\n", "\n").replace('\r', "\n");
145:     let mut cues = vec![];
146:     for block in norm.split("\n\n") {
147:         let lines: Vec<&str> = block.lines().map(|l| l.trim_end()).filter(|l| !l.trim().is_empty()).collect();
148:         if lines.is_empty() || lines[0].starts_with("WEBVTT") || lines[0].starts_with("NOTE") || lines[0].starts_with("STYLE") || lines[0].starts_with("REGION") {
149:             continue;
150:         }
151:         if let Some(c) = parse_arrow_block(&lines) {
152:             cues.push(c);
153:         }
154:     }
155:     cues
156: }
157: 
158: fn parse_ass(src: &str) -> Vec<Cue> {
159:     let mut cues = vec![];
160:     let (mut in_events, mut si, mut ei, mut ti, mut nfields) = (false, 1usize, 2usize, 9usize, 10usize);
161:     for raw in src.replace("\r\n", "\n").lines() {
162:         let line = raw.trim();
163:         if line.starts_with('[') {
164:             in_events = line.eq_ignore_ascii_case("[events]");
165:             continue;
166:         }
167:         if !in_events {
168:             continue;
169:         }
170:         if let Some(f) = line.strip_prefix("Format:") {
171:             let cols: Vec<String> = f.split(',').map(|c| c.trim().to_ascii_lowercase()).collect();
172:             nfields = cols.len();
173:             si = cols.iter().position(|c| c == "start").unwrap_or(1);
174:             ei = cols.iter().position(|c| c == "end").unwrap_or(2);
175:             ti = cols.iter().position(|c| c == "text").unwrap_or(nfields.saturating_sub(1));
176:         } else if let Some(d) = line.strip_prefix("Dialogue:") {
177:             let f: Vec<&str> = d.trim_start().splitn(nfields.max(1), ',').collect();
178:             if f.len() <= ti.max(si).max(ei) {
179:                 continue;
180:             }
181:             let (Some(start), Some(end)) = (parse_ts(f[si]), parse_ts(f[ei])) else { continue };
182:             let (text, italic, top) = clean(f[ti]);
183:             if !text.is_empty() && end > start {
184:                 cues.push(Cue { start, end, text, italic, top });
185:             }
186:         }
187:     }
188:     cues
189: }
190: 
191: /// Parses any supported format by sniffing the content. Never panics; unparseable input yields no cues.
192: pub fn parse(src: &str) -> Vec<Cue> {
193:     let head = src.trim_start_matches('\u{feff}').trim_start();
194:     let mut cues = if head.starts_with("[Script Info]") || head.contains("\n[Events]") || head.starts_with("[V4") {
195:         parse_ass(src)
196:     } else {
197
```

### TOOL: shell [seq 59] 2026-11-09 13:47:01 status=completed

```json
{
  "command": "ls -lh /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs\ngrep -rn \"connect_server\\|detect://\\|demo\\|TestCard\\|testcard\\|Fake\\|stub\\|todo\\|unimplemented\\|panic!\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/ /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs 2>&1 | head -n 300\necho ===; grep -rn \"ffmpeg\\|ffprobe\\|AudioPipe\\|FfmpegSource\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/ 2>&1 | head -n 200"
}
```

```
-rw-rw-r-- 1 lufuno lufuno  67K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs
-rw-rw-r-- 1 lufuno lufuno  71K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs
-rw-rw-r-- 1 lufuno lufuno  14K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs
-rw-rw-r-- 1 lufuno lufuno 100K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs
-rw-rw-r-- 1 lufuno lufuno  75K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:222:pub fn render_testcard(w: usize, h: usize, t: f64, fps: f64) -> Frame {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:268:pub struct TestCard {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:276:impl TestCard {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:277:    pub fn new(title: &str, duration: f64) -> TestCard {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:279:        let info = MediaInfo { title: title.into(), container: "testcard".into(), duration, video: vec![VideoStream { index: 0, codec: "procedural".into(), width: w as u32, height: h as u32, fps, bitrate: None, pix_fmt: "rgba".into() }], chapters: vec![(0.0, "Intro".into()), (duration * 0.35, "Middle".into()), (duration * 0.7, "Finale".into())], ..Default::default() };
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:280:        TestCard { info, w, h, last: None, fps }
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:284:impl FrameSource for TestCard {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:297:        Some(render_testcard(self.w, self.h, idx as f64 / self.fps, self.fps))
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:577:    fn testcard_is_deterministic_and_animated() {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:578:        let a = render_testcard(160, 90, 1.0, 30.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:579:        let b = render_testcard(160, 90, 1.0, 30.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:583:        let c = render_testcard(160, 90, 4.0, 30.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:585:        let tiny = render_testcard(1, 1, 0.0, 0.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:590:    fn testcard_source_returns_each_frame_once() {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:591:        let mut s = TestCard::new("x", 10.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:11:use eiga_core::source::{FrameSource, TestCard};
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:327:                let s = TestCard::new(&name, dur);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:841:            // nothing loaded yet: open the first demo/real video we can find so the window is never empty
===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:1://! **Eiga (映画)** — the suite's video player. Engine: `eiga-core` (state machine, subtitles, key map, ffmpeg pipes).
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:22:    pipe: Option<eiga_core::source::ffmpeg::AudioPipe>,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:40:        self.pipe = eiga_core::source::ffmpeg::AudioPipe::open(path, start, track, speed).ok();
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:304:            if eiga_core::source::ffmpeg::available() {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:305:                match eiga_core::source::ffmpeg::probe(rp) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:306:                    Ok(i) => match eiga_core::source::ffmpeg::FfmpegSource::open(rp, i.clone(), 1280, 720) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:316:                error = Some("ffmpeg was not found. Install it (apt install ffmpeg) to play real files. Showing the test card.".to_string());
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:325:                    error = Some("Web preview: decoding runs natively (ffmpeg). This is the built-in test card with real subtitles.".into());
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:348:                if let Ok(txt) = eiga_core::source::ffmpeg::extract_sub(rp, n) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:669:        let apps = [("folder", "Files", "File manager with tabs, search, tags, archives, devices"), ("pie-chart-2", "Storage", "Opens as a Files tab: cleanup, treemap, folder graph"), ("terminal", "Terminal", "Real shell on a pty (desktop) or a virtual shell (web)"), ("gallery", "Photos", "Library, viewer and light editing"), ("clapperboard", "Eiga", "Video player on ffmpeg, subtitles, mpv-style keys")];
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs:684:            ("Video", if web { "Built-in test card. Real decoding needs the desktop app + ffmpeg".into() } else { "ffmpeg pipes; test card as fallback".into() }),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:1://! Media metadata (ffprobe), frame sources (procedural test card everywhere; ffmpeg pipe on native).
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:93:/// Parses `ffprobe -print_format json -show_format -show_streams -show_chapters` output.
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:94:pub fn parse_ffprobe(json: &str) -> Result<MediaInfo, String> {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:95:    let v: Value = serde_json::from_str(json).map_err(|e| format!("unreadable ffprobe output: {e}"))?;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:304:// ─────────────────────────────────────────── ffmpeg (native) ───────────────────────────────────────────
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:307:pub mod ffmpeg {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:316:        Command::new("ffmpeg").arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false)
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:320:        let out = Command::new("ffprobe")
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:324:            .map_err(|e| format!("ffprobe is not installed ({e})"))?;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:329:        let mut info = parse_ffprobe(&String::from_utf8_lossy(&out.stdout))?;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:375:        let out = Command::new("ffmpeg").args(sub_args(path, sub_n)).stdin(Stdio::null()).output().map_err(|e| e.to_string())?;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs:382:    pub str
```

### TOOL: read [seq 66] 2026-11-09 13:47:04 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs, lines 1-1073
1: //! **Eiga (映画)** — the suite's video player. Engine: `eiga-core` (state machine, subtitles, key map, ffmpeg pipes).
2: //! This file is only the surface: it draws frames, the controls, subtitles, menus and routes keys.
3: use crate::app::*;
4: use crate::appkit::Frame;
5: use crate::chrome::*;
6: use crate::icons::Ic;
7: use crate::overlays::{mi, MI};
8: use crate::paint::*;
9: use crate::theme::*;
10: use eiga_core::player::{default_bindings, lookup, Binding, Cmd, Event as PEvent, LoopMode, Repeat};
11: use eiga_core::source::{FrameSource, TestCard};
12: use eiga_core::{fmt_time, parse_time, MediaInfo, Player, Playlist, SubTrack};
13: use egui::{pos2, vec2, Align2, Color32, Key, Rect, Sense, Ui};
14: use smart_core::*;
15: 
16: const VIDEO_EXTS: [&str; 8] = ["mp4", "mkv", "mov", "webm", "avi", "m4v", "mpg", "ts"];
17: 
18: #[cfg(not(target_arch = "wasm32"))]
19: pub struct AudioOut {
20:     dev: rodio::MixerDeviceSink,
21:     player: rodio::Player,
22:     pipe: Option<eiga_core::source::ffmpeg::AudioPipe>,
23: }
24: 
25: #[cfg(not(target_arch = "wasm32"))]
26: impl AudioOut {
27:     fn new() -> Result<Self, String> {
28:         let mut dev = rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| format!("No audio output: {e}"))?;
29:         dev.log_on_drop(false);
30:         let player = rodio::Player::connect_new(dev.mixer());
31:         Ok(AudioOut { dev, player, pipe: None })
32:     }
33:     fn restart(&mut self, path: &str, start: f64, track: usize, speed: f64, gain: f32, paused: bool) {
34:         self.player.stop();
35:         self.player = rodio::Player::connect_new(self.dev.mixer());
36:         self.player.set_volume(gain);
37:         if paused {
38:             self.player.pause();
39:         }
40:         self.pipe = eiga_core::source::ffmpeg::AudioPipe::open(path, start, track, speed).ok();
41:     }
42:     fn pump(&mut self) {
43:         let (Some(ch), Some(sr)) = (std::num::NonZeroU16::new(2), std::num::NonZeroU32::new(48_000)) else { return };
44:         if let Some(p) = &self.pipe {
45:             while self.player.len() < 3 {
46:                 match p.try_chunk() {
47:                     Some(c) => self.player.append(rodio::buffer::SamplesBuffer::new(ch, sr, c)),
48:                     None => break,
49:                 }
50:             }
51:         }
52:     }
53: }
54: 
55: pub struct Eiga {
56:     pub player: Player,
57:     pub playlist: Playlist,
58:     pub src: Option<Box<dyn FrameSource>>,
59:     pub info: Option<MediaInfo>,
60:     pub tex: Option<egui::TextureHandle>,
61:     pub frame_wh: (usize, usize),
62:     pub subs: Vec<SubTrack>,
63:     pub sub_i: Option<usize>,
64:     pub sub_scale: f32,
65:     pub audio_i: usize,
66:     pub title: String,
67:     pub path: Option<String>,
68:     pub real: Option<String>,
69:     pub bindings: Vec<Binding>,
70:     pub last_move: f64,
71:     pub last_tick: f64,
72:     pub theater: bool,
73:     pub show_list: bool,
74:     pub show_stats: bool,
75:     pub show_help: bool,
76:     pub aspect: u8,
77:     pub zoom: f32,
78:     pub rotate: u8,
79:     pub osd: Option<(String, f64)>,
80:     pub error: Option<String>,
81:     pub jump: Option<String>,
82:     pub scrub: Option<f64>,
83:     pub frames_shown: u64,
84:     pub fps_meter: (f64, u64, f32),
85:     pub locs: Vec<Location>,
86:     pub cfg_hide_secs: f32,
87:     pub cfg_sub_size: f32,
88:     pub cfg_def_speed: f64,
89:     pub cfg_double_click: bool,
90:     pub cfg_loop_default: bool,
91:     #[cfg(not(target_arch = "wasm32"))]
92:     pub picker_rx: Option<std::sync::mpsc::Receiver<Vec<std::path::PathBuf>>>,
93:     pub pending_evs: Vec<PEvent>,
94:     pub pending_cmd: Option<Cmd>,
95:     pub last_rgba: Option<(usize, usize, Vec<u8>)>,
96:     pub fs_dirty: bool,
97:     #[cfg(not(target_arch = "wasm32"))]
98:     pub audio: Option<AudioOut>,
99: }
100: 
101: impl Default for Eiga {
102:     fn default() -> Self {
103:         Eiga { player: Player::default(), playlist: Playlist::default(), src: None, info: None, tex: None, frame_wh: (0, 0), subs: vec![], sub_i: None, sub_scale: 1.0, audio_i: 0, title: String::new(), path: None, real: None, bindings: default_bindings(), last_move: 0.0, last_tick: 0.0, theater: false, show_list: true, show_stats: false, show_help: false, aspect: 0, zoom: 1.0, rotate: 0, osd: None, error: None, jump: None, scrub: None, frames_shown: 0, fps_meter: (0.0, 0, 0.0), locs: vec![], cfg_hide_secs: 2.6, cfg_sub_size: 1.0, cfg_def_speed: 1.0, cfg_double_click: true, cfg_loop_default: false, #[cfg(not(target_arch = "wasm32"))] picker_rx: None, pending_evs: vec![], pending_cmd: None, last_rgba: None, fs_dirty: false, #[cfg(not(target_arch = "wasm32"))] audio: None }
104:     }
105: }
106: 
107: fn key_name(k: Key, shift: bool) -> Option<&'static str> {
108:     Some(match k {
109:         Key::Space => "space",
110:         Key::ArrowLeft => "left",
111:         Key::ArrowRight => "right",
112:         Key::ArrowUp => "up",
113:         Key::ArrowDown => "down",
114:         Key::Home => "home",
115:         Key::PageUp => "pageup",
116:         Key::PageDown => "pagedown",
117:         Key::Enter => "enter",
118:         Key::Escape => "escape",
119:         Key::Backspace => "backspace",
120:         Key::Tab => "tab",
121:         Key::Comma => if shift { "comma" } else { "," },
122:         Key::Period => if shift { "period" } else { "." },
123:         Key::OpenBracket => "[",
124:         Key::CloseBracket => "]",
125:         Key::OpenCurlyBracket => "{",
126:         Key::CloseCurlyBracket => "}",
127:         Key::Minus => "-",
128:         Key::Equals | Key::Plus => "=",
129:         Key::Slash => "/",
130:         Key::Questionmark => "?",
131:         Key::Num0 => "0",
132:         Key::Num1 => "1",
133:         Key::Num2 => "2",
134:         Key::Num3 => "3",
135:         Key::Num4 => "4",
136:         Key::Num5 => "5",
137:         Key::Num6 => "6",
138:         Key::Num7 => "7",
139:         Key::Num8 => "8",
140:         Key::Num9 => "9",
141:         Key::A => "a",
142:         Key::B => "b",
143:         Key::C => "c",
144:         Key::D => "d",
145:         Key::E => "e",
146:         Key::F => "f",
147:         Key::G => "g",
148:         Key::H => "h",
149:         Key::I => "i",
150:         Key::J => "j",
151:         Key::K => "k",
152:         Key::L => "l",
153:         Key::M => "m",
154:         Key::N => "n",
155:         Key::O => "o",
156:         Key::P => "p",
157:         Key::Q => "q",
158:         Key::R => "r",
159:         Key::S => "s",
160:         Key::T => "t",
161:         Key::U => "u",
162:         Key::V => "v",
163:         Key::W => "w",
164:         Key::X => "x",
165:         Key::Y => "y",
166:         Key::Z => "z",
167:         _ => return None,
168:     })
169: }
170: 
171: impl SmartApp {
172:     // ───────────── opening ─────────────
173:     pub fn eiga_cmd(&mut self, c: &str) {
174:         let (name, arg) = c.split_once(':').unwrap_or((c, ""));
175:         match name {
176:             "open" => {
177:                 if let Some((fs, path)) = arg.split_once('|') {
178:                     self.eiga_open(Location::new(fs, path));
179:                 }
180:             }
181:             "play_idx" => {
182:                 if let Ok(i) = arg.parse::<usize>() {
183:                     self.eiga_play_index(i);
184:                 }
185:             }
186:             "sub" => {
187:                 let e = &mut self.shell.eiga;
188:                 e.sub_i = if arg == "off" { None } else { arg.parse().ok().filter(|i| *i < e.subs.len()) };
189:                 let msg = match e.sub_i.and_then(|i| e.subs.get(i)) {
190:                     Some(t) => format!("Subtitles: {}", t.name),
191:                     None => "Subtitles off".to_string(),
192:                 };
193:              
```

### TOOL: shell [seq 73] 2026-11-09 13:47:06 status=completed

```json
{
  "command": "wc -l /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs 2>&1\ngrep -rn \"eiga\\|Eiga\\|video\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/*.rs 2>&1 | head -n 100\ngrep -rn \"thumbs\\|thumb\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/*.rs 2>&1 | head -n 100"
}
```

```
  1523 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
  1794 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs
  1370 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs
  1591 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs
   300 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs
   746 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs
  7324 total
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs:177:            Bucket::Media => "Photos, audio & video",
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:166:            add(&mut v, "movies", "Movies", "clapperboard", dirs::video_dir());
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs:173:            Kind::Video if !e.is_empty() => format!("{} video", e),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/search.rs:89:                    "video" | "movie" | "movies" => f.kinds.push(Kind::Video),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/search.rs:128:    /// “big videos from last week” → `kind:video size:>100mb modified:week`. Everything stays on-device.
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/search.rs:138:            (&["video", "videos", "movie", "movies", "clips"][..], "kind:video", "videos"),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:12:use crate::thumbs::{ThumbResult, Thumbs};
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:57:    thumbs: Thumbs,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:79:            thumbs: Thumbs::new(registry.clone()),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:506:    // ───────── thumbnails ─────────
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:507:    pub fn request_thumb(&self, loc: &Location, edge: u32) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:508:        self.thumbs.request(loc, edge);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:510:    pub fn forget_thumb(&self, loc: &Location) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:511:        self.thumbs.forget(loc);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:700:        for t in self.thumbs.poll() {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:705:        self.thumbs.pending() || !self.jobs.lock().unwrap().is_empty()
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs:10:        include_bytes!(concat!("../../../assets/thumbs/", $n, ".jpg")).to_vec()
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs:101:        (".cache/thumbnails/large/cache.bin", 620 * MB, 24.0 * 3.0),
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/lib.rs:23:pub mod thumbs;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:34:    pub thumb_scale: f32,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:55:            thumb_scale: 1.0,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs:1://! Background thumbnail decoding (image crate). Frontends upload the RGBA to a texture.
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs:34:    let t = img.thumbnail(edge, edge).to_rgba8();
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs:75:    /// Drain finished thumbnails. In the browser this also does the decoding, a few per call.

```

### TOOL: read [seq 80] 2026-11-09 13:47:09 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs",
  "offset": 1074
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs, lines 1074-1523
1074:                     self.shell.eiga = e;
1075:                     self.eiga_events(ev);
1076:                     e = std::mem::take(&mut self.shell.eiga);
1077:                 }
1078:             } else if !esc {
1079:                 e.jump = Some(text);
1080:             }
1081:         }
1082:         if e.show_help {
1083:             self.eiga_help(ui, area, &mut e);
1084:         }
1085:         // restore state, then drain actions that need &mut self
1086:         if e.player.eof && e.playlist.items.len() > 1 && e.playlist.repeat != Repeat::Off {
1087:             // handled by Ended event
1088:         }
1089:         self.shell.eiga = e;
1090:     }
1091: 
1092:     fn eiga_default_video(&self) -> Option<Location> {
1093:         let mut dirs = vec![];
1094:         for id in ["movies", "videos"] {
1095:             if let Some(p) = self.places.iter().find(|p| p.id == id) {
1096:                 dirs.push(p.loc.clone());
1097:             }
1098:         }
1099:         dirs.push(self.be.home());
1100:         for d in dirs {
1101:             if let Some(v) = self.be.registry.get(&d.fs) {
1102:                 if let Some(e) = v.list(&d.path).unwrap_or_default().into_iter().find(|e| !e.is_dir && e.kind == Kind::Video) {
1103:                     return Some(e.loc);
1104:                 }
1105:             }
1106:         }
1107:         None
1108:     }
1109: 
1110:     fn eiga_load_first(&mut self, l: Location) {
1111:         let locs = self.eiga_siblings(&l);
1112:         let start = locs.iter().position(|x| *x == l).unwrap_or(0);
1113:         self.shell.eiga.playlist = Playlist::new(locs.iter().map(|x| x.name()).collect(), start);
1114:         self.shell.eiga.locs = locs;
1115:         self.eiga_load(l);
1116:         self.shell.eiga.player.paused = true;
1117:     }
1118: 
1119:     fn eiga_toolbar(&mut self, ui: &Ui, fr: &Frame, e: &Eiga) {
1120:         let th = self.th.clone();
1121:         let p = ui.painter().clone();
1122:         let cy = fr.bar.center().y;
1123:         let mut rx = fr.right_x;
1124:         let more = Rect::from_min_size(pos2(rx - 34.0, cy - 17.0), vec2(34.0, 34.0));
1125:         if self.icon_button(ui, more, "menu-dots", "ei-more", Btn::Raised, true).on_hover_text("More").clicked() {
1126:             let items = self.eiga_more_menu();
1127:             self.open_app_menu(pos2(more.right() - 250.0, more.bottom() + 8.0), items);
1128:         }
1129:         rx = more.left() - 8.0;
1130:         let avail = rx - fr.lead_x;
1131:         // dropdown chips: speed · subtitles · audio
1132:         let speed_label = format!("\u{d7}{}", (e.player.speed * 100.0).round() / 100.0);
1133:         let sub_label = match e.sub_i.and_then(|i| e.subs.get(i)) {
1134:             Some(s) => if s.lang.is_empty() { "Subtitles".to_string() } else { s.lang.to_uppercase() },
1135:             None => "Subs off".to_string(),
1136:         };
1137:         let chips: [(&'static str, String, &str, u8); 3] = [("bolt", speed_label, "ei-speed", 0), ("subtitles", sub_label, "ei-subs", 1), ("volume-loud", format!("Audio {}", e.audio_i + 1), "ei-audio", 2)];
1138:         for (icon, label, id, which) in chips {
1139:             if avail < 380.0 && which == 2 {
1140:                 continue;
1141:             }
1142:             let w = self.tw(&p, &label, &f_semi(T_META)) + 52.0;
1143:             let r = Rect::from_min_size(pos2(rx - w, cy - 16.0), vec2(w, 32.0));
1144:             let resp = self.hit(ui, r, id, Sense::click());
1145:             raised(&p, &th, r, 16.0, resp.hovered(), resp.is_pointer_button_down_on());
1146:             self.ico(&p, icon, Ic::Duo, pos2(r.left() + 18.0, r.center().y), 15.0, th.accent_ink);
1147:             p.text(pos2(r.left() + 32.0, r.center().y), Align2::LEFT_CENTER, &label, f_semi(T_META), th.ink1);
1148:             self.ico(&p, "alt-arrow-down", Ic::Outline, pos2(r.right() - 12.0, r.center().y), 11.0, th.ink3);
1149:             if resp.clicked() {
1150:                 let items = match which {
1151:                     0 => self.eiga_speed_menu(),
1152:                     1 => self.eiga_sub_menu(),
1153:                     _ => self.eiga_audio_menu(),
1154:                 };
1155:                 self.open_app_menu(pos2(r.left(), r.bottom() + 8.0), items);
1156:             }
1157:             rx = r.left() - 8.0;
1158:         }
1159:         // title
1160:         let tx = fr.lead_x;
1161:         let w = (rx - tx - 12.0).max(40.0);
1162:         self.ico(&p, "clapperboard", Ic::Duo, pos2(tx + 10.0, cy), 18.0, th.accent_ink);
1163:         p.text(pos2(tx + 28.0, cy - 7.0), Align2::LEFT_CENTER, self.fit(&p, &e.title, &f_semi(T_BODY), w - 30.0), f_semi(T_BODY), th.ink1);
1164:         let sub = match &e.info {
1165:             Some(i) => {
1166:                 let (w, h) = i.size_wh();
1167:                 let mut parts = vec![];
1168:                 if w > 0 {
1169:                     parts.push(format!("{w}\u{d7}{h}"));
1170:                 }
1171:                 if let Some(v) = i.primary_video() {
1172:                     parts.push(format!("{} {:.0} fps", v.codec, i.fps()));
1173:                 }
1174:                 parts.push(fmt_time(i.duration));
1175:                 parts.join(" \u{b7} ")
1176:             }
1177:             None => String::new(),
1178:         };
1179:         p.text(pos2(tx + 28.0, cy + 9.0), Align2::LEFT_CENTER, self.fit(&p, &sub, &f_medium(10.5), w - 30.0), f_medium(10.5), th.ink3);
1180:     }
1181: 
1182:     /// Fullscreen top bar: no background, drawn straight over the video, fading in when the pointer nears the top.
1183:     fn eiga_overlay_bar(&mut self, ui: &mut Ui, area: Rect, e: &mut Eiga, t: f32) {
1184:         let th = self.th.clone();
1185:         let p = ui.painter().with_clip_rect(area);
1186:         let cy = area.top() + 34.0;
1187:         let white = alpha(Color32::WHITE, t);
1188:         let mut rx = area.right() - 18.0;
1189:         let more = Rect::from_center_size(pos2(rx - 17.0, cy), vec2(34.0, 34.0));
1190:         let mr = self.hit(ui, more, "eiga-ov-more", Sense::click());
1191:         if mr.hovered() {
1192:             p.circle_filled(more.center(), 17.0, Color32::from_white_alpha((0.14 * t) as u8));
1193:         }
1194:         self.ico(&p, "menu-dots", Ic::Duo, more.center(), 18.0, white);
1195:         if mr.clicked() {
1196:             let items = self.eiga_more_menu();
1197:             self.open_app_menu(pos2(more.right() - 250.0, more.bottom() + 8.0), items);
1198:         }
1199:         rx = more.left() - 6.0;
1200:         let chips: [(f32, &'static str, String, &str, u8); 3] = [
1201:             (46.0, "folder-open", "Open".into(), "eiga-ov-open", 0),
1202:             (64.0, "bolt", format!("\u{d7}{}", (e.player.speed * 100.0).round() / 100.0), "eiga-ov-speed", 1),
1203:             (70.0, "subtitles", match e.sub_i.and_then(|i| e.subs.get(i)) { Some(s) => if s.lang.is_empty() { "Subs".into() } else { s.lang.to_uppercase() }, None => "Subs".into() }, "eiga-ov-subs", 2),
1204:         ];
1205:         for (w, icon, label, id, which) in chips {
1206:             let r = Rect::from_min_size(pos2(rx - w, cy - 16.0), vec2(w, 32.0));
1207:             let resp = self.hit(ui, r, id, Sense::click());
1208:             if resp.hovered() {
1209:                 p.rect_filled(r, cr(16.0), Color32::from_white_alpha((0.14 * t) as u8));
1210:             }
1211:             shadow(&p, Rect::from_center_size(pos2(r.left() + 17.0, r.center().y), vec2(15.0, 15.0)), 8.0, 1.0, 4.0, 1.0, c(0, 0, 0, 0.4 * t));
1212:             self.ico(&p, icon, Ic::Duo, pos2(r.left() + 17.0, r.center().y), 15.0, white);
1213:             p.text(pos2(r.left() + 30.0, r.center().y), Align2::LEFT_CENTER, &label, f_semi(T_META), white);
1214:             if resp.clicked() {
1215:                 match which {
```

### TOOL: read [seq 80] 2026-11-09 13:47:09 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs, lines 1-91
1: //! Background thumbnail decoding (image crate). Frontends upload the RGBA to a texture.
2: use crate::exec::THREADED;
3: use crate::model::Location;
4: use crate::ops::Registry;
5: use std::collections::HashSet;
6: use std::sync::{Arc, Condvar, Mutex};
7: 
8: pub struct ThumbResult {
9:     pub loc: Location,
10:     pub size: [usize; 2],
11:     pub rgba: Vec<u8>,
12:     pub original: Option<(u32, u32)>,
13:     pub edge: u32,
14: }
15: 
16: #[derive(Default)]
17: struct Shared {
18:     queue: Mutex<Vec<(Location, u32)>>,
19:     cv: Condvar,
20:     out: Mutex<Vec<ThumbResult>>,
21: }
22: 
23: pub struct Thumbs {
24:     shared: Arc<Shared>,
25:     registry: Arc<Registry>,
26:     seen: Mutex<HashSet<(Location, u32)>>,
27: }
28: 
29: fn decode(reg: &Registry, loc: &Location, edge: u32) -> Option<ThumbResult> {
30:     let v = reg.get(&loc.fs)?;
31:     let bytes = v.read(&loc.path, 48 << 20).ok()?;
32:     let img = image::load_from_memory(&bytes).ok()?;
33:     let original = Some((img.width(), img.height()));
34:     let t = img.thumbnail(edge, edge).to_rgba8();
35:     Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
36: }
37: 
38: impl Thumbs {
39:     pub fn new(registry: Arc<Registry>) -> Self {
40:         let shared = Arc::new(Shared::default());
41:         if THREADED {
42:             let n = 2;
43:             for _ in 0..n {
44:                 let (s, r) = (shared.clone(), registry.clone());
45:                 crate::exec::spawn(move || loop {
46:                     let job = {
47:                         let mut q = s.queue.lock().unwrap();
48:                         while q.is_empty() {
49:                             q = s.cv.wait(q).unwrap();
50:                         }
51:                         q.pop() // newest first = what is on screen now
52:                     };
53:                     if let Some((loc, edge)) = job {
54:                         let res = decode(&r, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
55:                         s.out.lock().unwrap().push(res);
56:                     }
57:                 });
58:             }
59:         }
60:         Self { shared, registry, seen: Mutex::new(HashSet::new()) }
61:     }
62: 
63:     pub fn request(&self, loc: &Location, edge: u32) {
64:         if !self.seen.lock().unwrap().insert((loc.clone(), edge)) {
65:             return;
66:         }
67:         self.shared.queue.lock().unwrap().push((loc.clone(), edge));
68:         self.shared.cv.notify_one();
69:     }
70: 
71:     pub fn forget(&self, loc: &Location) {
72:         self.seen.lock().unwrap().retain(|(l, _)| l != loc);
73:     }
74: 
75:     /// Drain finished thumbnails. In the browser this also does the decoding, a few per call.
76:     pub fn poll(&self) -> Vec<ThumbResult> {
77:         if !THREADED {
78:             for _ in 0..2 {
79:                 let job = self.shared.queue.lock().unwrap().pop();
80:                 let Some((loc, edge)) = job else { break };
81:                 let r = decode(&self.registry, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
82:                 self.shared.out.lock().unwrap().push(r);
83:             }
84:         }
85:         std::mem::take(&mut *self.shared.out.lock().unwrap())
86:     }
87: 
88:     pub fn pending(&self) -> bool {
89:         !self.shared.queue.lock().unwrap().is_empty()
90:     }
91: }
```

### TOOL: read [seq 80] 2026-11-09 13:47:09 status=completed

```json
{
  "path": "/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs"
}
```

```
Read file /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs, lines 1-746
1: //! Storage analysis: an incremental, bounded-memory directory scanner plus a *context-aware* junk detector.
2: //!
3: //! Design notes
4: //! * Only **directories** become nodes; per directory we keep the ten biggest files and aggregate the rest, so a
5: //!   home folder with millions of files stays in the tens of megabytes.
6: //! * The scanner is a state machine (`Scan::step`). Desktop runs it on a worker thread, the web build slices it
7: //!   across frames. Nothing here touches threads or clocks, so it behaves identically (and is unit-tested) everywhere.
8: //! * Junk is not "big files". A `target/` next to a `Cargo.toml`, a `node_modules/` next to a `package.json`, a
9: //!   `.venv`, package-manager caches, the Trash, stale installers and "Report (1).pdf" copies are all recognised from
10: //!   their *surroundings*, graded by how safe they are to remove, and annotated with how to get them back.
11: use crate::model::{Entry, Kind, Location};
12: use crate::vfs::Vfs;
13: use std::collections::HashSet;
14: 
15: /// How risky it is to remove something.
16: #[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
17: pub enum Safety {
18:     /// Regenerated automatically (build output, caches). Removing loses nothing.
19:     Safe,
20:     /// Probably unwanted, but it is *your* call (old installers, copies, logs).
21:     Review,
22:     /// Looks like personal data that is merely large or old. Never pre-selected.
23:     Careful,
24: }
25: 
26: impl Safety {
27:     pub fn label(self) -> &'static str {
28:         match self {
29:             Safety::Safe => "Safe",
30:             Safety::Review => "Review",
31:             Safety::Careful => "Careful",
32:         }
33:     }
34: }
35: 
36: #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
37: pub enum JunkKind {
38:     BuildArtifacts,
39:     Dependencies,
40:     Environments,
41:     Caches,
42:     Temp,
43:     Logs,
44:     Trash,
45:     Installers,
46:     Duplicates,
47:     LargeStale,
48: }
49: 
50: impl JunkKind {
51:     pub const ALL: [JunkKind; 10] = [
52:         JunkKind::BuildArtifacts,
53:         JunkKind::Dependencies,
54:         JunkKind::Environments,
55:         JunkKind::Caches,
56:         JunkKind::Temp,
57:         JunkKind::Logs,
58:         JunkKind::Trash,
59:         JunkKind::Installers,
60:         JunkKind::Duplicates,
61:         JunkKind::LargeStale,
62:     ];
63:     pub fn label(self) -> &'static str {
64:         match self {
65:             JunkKind::BuildArtifacts => "Build artifacts",
66:             JunkKind::Dependencies => "Dependencies",
67:             JunkKind::Environments => "Virtual environments",
68:             JunkKind::Caches => "Caches",
69:             JunkKind::Temp => "Temporary files",
70:             JunkKind::Logs => "Large logs",
71:             JunkKind::Trash => "Trash",
72:             JunkKind::Installers => "Old installers",
73:             JunkKind::Duplicates => "Likely copies",
74:             JunkKind::LargeStale => "Large & untouched",
75:         }
76:     }
77:     pub fn icon(self) -> &'static str {
78:         match self {
79:             JunkKind::BuildArtifacts => "code-square",
80:             JunkKind::Dependencies => "box-minimalistic",
81:             JunkKind::Environments => "box-minimalistic",
82:             JunkKind::Caches => "refresh",
83:             JunkKind::Temp => "clock-circle",
84:             JunkKind::Logs => "text",
85:             JunkKind::Trash => "trash-bin-minimalistic",
86:             JunkKind::Installers => "download-minimalistic",
87:             JunkKind::Duplicates => "copy",
88:             JunkKind::LargeStale => "history",
89:         }
90:     }
91:     pub fn blurb(self) -> &'static str {
92:         match self {
93:             JunkKind::BuildArtifacts => "Compiler output that your next build recreates",
94:             JunkKind::Dependencies => "Downloaded packages that the package manager restores",
95:             JunkKind::Environments => "Language environments you can rebuild from a lockfile",
96:             JunkKind::Caches => "Data apps keep to be faster. They rebuild it on demand",
97:             JunkKind::Temp => "Leftovers: partial downloads, backups, editor swap files",
98:             JunkKind::Logs => "Log files that grew far beyond usefulness",
99:             JunkKind::Trash => "Already deleted. Emptying frees the space for real",
100:             JunkKind::Installers => "Setup files you already ran",
101:             JunkKind::Duplicates => "Files named like a copy of a sibling",
102:             JunkKind::LargeStale => "Big files nobody has touched for a long time",
103:         }
104:     }
105: }
106: 
107: #[derive(Clone, Debug)]
108: pub struct FileInfo {
109:     pub name: String,
110:     pub path: String,
111:     pub size: u64,
112:     pub modified: Option<i64>,
113:     pub kind: Kind,
114: }
115: 
116: #[derive(Clone, Debug)]
117: pub struct DirNode {
118:     pub name: String,
119:     pub path: String,
120:     pub parent: Option<usize>,
121:     /// Recursive size in bytes (grows while scanning).
122:     pub size: u64,
123:     /// Bytes of files directly inside this folder.
124:     pub own: u64,
125:     /// Files directly inside this folder.
126:     pub files: u64,
127:     pub children: Vec<usize>,
128:     /// The biggest files directly inside (at most `TOP_FILES`).
129:     pub top_files: Vec<FileInfo>,
130:     pub newest: i64,
131:     pub junk: Option<usize>,
132:     pub in_junk: bool,
133:     pub done: bool,
134: }
135: 
136: #[derive(Clone, Debug)]
137: pub struct Junk {
138:     pub path: String,
139:     pub name: String,
140:     pub is_dir: bool,
141:     pub kind: JunkKind,
142:     pub safety: Safety,
143:     pub reason: String,
144:     /// How the data comes back, when it does.
145:     pub restore: Option<&'static str>,
146:     pub modified: Option<i64>,
147:     size: u64,
148:     node: Option<usize>,
149: }
150: 
151: impl Junk {
152:     pub fn size(&self) -> u64 {
153:         self.size
154:     }
155: }
156: 
157: pub const TOP_FILES: usize = 10;
158: pub const MIN_JUNK_BYTES: u64 = 512 * 1024;
159: const MAX_FILE_JUNK: usize = 1500;
160: const DAY: i64 = 86_400;
161: 
162: /// Coarse content buckets for the overview chart.
163: #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
164: pub enum Bucket {
165:     Media,
166:     Documents,
167:     Code,
168:     Archives,
169:     Apps,
170:     Other,
171: }
172: 
173: impl Bucket {
174:     pub const ALL: [Bucket; 6] = [Bucket::Media, Bucket::Documents, Bucket::Code, Bucket::Archives, Bucket::Apps, Bucket::Other];
175:     pub fn label(self) -> &'static str {
176:         match self {
177:             Bucket::Media => "Photos, audio & video",
178:             Bucket::Documents => "Documents",
179:             Bucket::Code => "Code & text",
180:             Bucket::Archives => "Archives",
181:             Bucket::Apps => "Apps & installers",
182:             Bucket::Other => "Other",
183:         }
184:     }
185:     pub fn of(k: Kind) -> Bucket {
186:         match k {
187:             Kind::Image | Kind::Vector | Kind::Audio | Kind::Video | Kind::Project => Bucket::Media,
188:             Kind::Pdf | Kind::Document => Bucket::Documents,
189:             Kind::Text | Kind::Code => Bucket::Code,
190:             Kind::Archive => Bucket::Archives,
191:             Kind::App => Bucket::Apps,
192:             _ => Bucket::Other,
193:         }
194:     }
195:     fn idx(self) -> usize {
196:         Bucket::ALL.iter().position(|b| *b == self).unwrap_or(5)
197:     }
198: }
199: 
200: pub struct Scan {
201:     pub fs: String,
202:     pub root: String,
203:     pub nodes: Vec<DirNode>,
204:     pub junk: Vec<Junk>,
205:     pub buckets: [u64; 6],
206:     pub files_seen: u64,
207:     pub dirs_seen: u64,
208:     pub er
```

### TOOL: shell [seq 95] 2026-11-09 13:47:12 status=completed

```json
{
  "command": "ls -R /tmp/opencode/canonical-eiga-audit 2>&1\nls -lh /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/*.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/*.rs 2>&1\necho \"=== highlight head ===\"\nhead -n 120 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs\necho \"=== app grep eiga/video ===\"\ngrep -n \"Eiga\\|eiga\\|Video\\|video_w\\|AppId\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs | head -n 100\necho \"=== backend head ===\"\nhead -n 100 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs"
}
```

```
/tmp/opencode/canonical-eiga-audit:
-rw-rw-r-- 1 lufuno lufuno  31K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/analyze.rs
-rw-rw-r-- 1 lufuno lufuno 8.8K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/apps.rs
-rw-rw-r-- 1 lufuno lufuno  18K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/archive.rs
-rw-rw-r-- 1 lufuno lufuno 6.5K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/audio.rs
-rw-rw-r-- 1 lufuno lufuno  30K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs
-rw-rw-r-- 1 lufuno lufuno  13K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs
-rw-rw-r-- 1 lufuno lufuno  17K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/devices.rs
-rw-rw-r-- 1 lufuno lufuno  428 Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/exec.rs
-rw-rw-r-- 1 lufuno lufuno 1.2K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/lib.rs
-rw-rw-r-- 1 lufuno lufuno 6.1K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs
-rw-rw-r-- 1 lufuno lufuno 3.5K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/local.rs
-rw-rw-r-- 1 lufuno lufuno 6.5K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/memfs.rs
-rw-rw-r-- 1 lufuno lufuno  12K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs
-rw-rw-r-- 1 lufuno lufuno 8.7K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/ops.rs
-rw-rw-r-- 1 lufuno lufuno  783 Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/registry.rs
-rw-rw-r-- 1 lufuno lufuno  22K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/search.rs
-rw-rw-r-- 1 lufuno lufuno 3.2K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/thumbs.rs
-rw-rw-r-- 1 lufuno lufuno 2.3K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/tools.rs
-rw-rw-r-- 1 lufuno lufuno 2.8K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/vfs.rs
-rw-rw-r-- 1 lufuno lufuno  67K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs
-rw-rw-r-- 1 lufuno lufuno  35K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/appkit.rs
-rw-rw-r-- 1 lufuno lufuno  71K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/chrome.rs
-rw-rw-r-- 1 lufuno lufuno  46K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs
-rw-rw-r-- 1 lufuno lufuno  16K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/copyfx.rs
-rw-rw-r-- 1 lufuno lufuno  14K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs
-rw-rw-r-- 1 lufuno lufuno 2.0K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/icons.rs
-rw-rw-r-- 1 lufuno lufuno  40K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/icons_gen.rs
-rw-rw-r-- 1 lufuno lufuno  21K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs
-rw-rw-r-- 1 lufuno lufuno   64 Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/main.rs
-rw-rw-r-- 1 lufuno lufuno 100K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/modals.rs
-rw-rw-r-- 1 lufuno lufuno  74K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/overlays.rs
-rw-rw-r-- 1 lufuno lufuno  11K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/paint.rs
-rw-rw-r-- 1 lufuno lufuno  74K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/photos.rs
-rw-rw-r-- 1 lufuno lufuno  16K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/pty.rs
-rw-rw-r-- 1 lufuno lufuno  42K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs
-rw-rw-r-- 1 lufuno lufuno  54K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/storage.rs
-rw-rw-r-- 1 lufuno lufuno 120K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/terminal.rs
-rw-rw-r-- 1 lufuno lufuno  16K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/termlive.rs
-rw-rw-r-- 1 lufuno lufuno 8.0K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/theme.rs
-rw-rw-r-- 1 lufuno lufuno  75K Nov  6 17:02 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
=== highlight head ===
//! Syntax highlighting with tree-sitter (Rust, Python, JavaScript, JSON, TOML, Bash, CSS, HTML, C, Go, Markdown) and a
//! small lexer for everything else, so *every* text file gets colour and nothing ever fails to preview.
use egui::{text::LayoutJob, Color32, FontId, TextFormat};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cls {
    Plain,
    Keyword,
    Function,
    Type,
    Str,
    Number,
    Comment,
    Punct,
    Property,
    Tag,
    Title,
    Link,
    Escape,
}

const NAMES: [&str; 37] = [
    "attribute", "comment", "constant", "constant.builtin", "constructor", "embedded", "escape", "function", "function.builtin", "function.method", "keyword", "number", "operator", "property", "punctuation", "punctuation.bracket", "punctuation.delimiter", "punctuation.special", "string", "string.special", "string.escape", "tag", "type", "type.builtin", "variable", "variable.builtin", "variable.parameter", "text.title", "text.literal", "text.uri", "text.reference", "text.emphasis", "text.strong", "label", "module", "boolean", "character",
];

fn cls_of(i: usize) -> Cls {
    match NAMES[i] {
        "keyword" | "boolean" | "variable.builtin" | "label" => Cls::Keyword,
        "function" | "function.builtin" | "function.method" | "constructor" => Cls::Function,
        "type" | "type.builtin" | "module" | "attribute" => Cls::Type,
        "string" | "string.special" | "text.literal" | "character" | "embedded" => Cls::Str,
        "number" | "constant" | "constant.builtin" => Cls::Number,
        "comment" => Cls::Comment,
        "operator" | "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => Cls::Punct,
        "property" | "variable.parameter" => Cls::Property,
        "tag" => Cls::Tag,
        "text.title" | "text.strong" => Cls::Title,
        "text.uri" | "text.reference" | "text.emphasis" => Cls::Link,
        "escape" | "string.escape" => Cls::Escape,
        _ => Cls::Plain,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn config(ext: &str) -> Option<tree_sitter_highlight::HighlightConfiguration> {
    let (lang, name, q): (tree_sitter::Language, &str, &str) = match ext {
        "rs" => (tree_sitter_rust::LANGUAGE.into(), "rust", tree_sitter_rust::HIGHLIGHTS_QUERY),
        "py" | "pyw" => (tree_sitter_python::LANGUAGE.into(), "python", tree_sitter_python::HIGHLIGHTS_QUERY),
        "js" | "mjs" | "cjs" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), "javascript", tree_sitter_javascript::HIGHLIGHT_QUERY),
        "json" | "jsonc" => (tree_sitter_json::LANGUAGE.into(), "json", tree_sitter_json::HIGHLIGHTS_QUERY),
        "toml" => (tree_sitte
```

### TOOL: shell [seq 106] 2026-11-09 13:47:17 status=completed

```json
{
  "command": "cat /home/lufuno/smart/smart-suite-official/specs/CANONICAL-SPECS.md 2>&1 | head -n 200\necho ===CORE TOC===\nhead -n 150 /tmp/opencode/canonical-core-audit/CORE-SPECS.md 2>&1\necho ===UX===\nhead -n 150 /tmp/opencode/canonical-ux-audit/UX-SPECS.md 2>&1 || head -n 150 /home/lufuno/smart/smart-suite-official/audits/UX-SPECS.md 2>&1 | head -n 150\nls /tmp/opencode/canonical-ux-audit/ 2>&1; ls /home/lufuno/smart/smart-protocol-official/schema/ /home/lufuno/smart/smart-protocol-official/gen/ 2>&1 | head -n 60"
}
```

```
uess, arboard, notify-rust, zbus, memmap2`.

AppImage: `scripts/build-appimage.sh` (prod), `vite.config.ts`, `xtask`. Verify native+WASM (`wasm32-unknown-unknown` via `wasm-bindgen` + `egui`).

---

## 8. Unknown Unknowns (new)

Broken symlinks, permission/run-as-root hint, NFC/NFD/RTL/grapheme, 255/NUL, NFS timeout+Retry, inotify limits, disk-full preflight + partial cleanup, archive bombs, thumb/icon LRU, search Rebuild, `x-special` clipboard + external drop, multi-window honesty, DPI hitzones, focus trap/screen-reader, Recents tombstones, stable sort, local-time label, Trash `info` hidden, `is_writable`, atomic cancel, `smart.toml` migration.

---

## 9. Acceptance

* [ ] Extracted non-destructive, originals untouched
* [ ] `smart-ui-egui` rlib + `apps/smart-files` decoupled, protocol traits only
* [ ] Eiga player cut, thumb service in
* [ ] Picker new folder/search/places, portal wired
* [ ] Drag-paste freeze/spec, Trash UI, Settings 6-tab, resize cursors, list overlap, internal Reveal, 1-char search, real mimetypes, exec+chmod, AI reversible, memory LRU+scan, phone MTP+ADB poller, archive streamed, AppImage green
* [ ] Zero `todo!` lies, `guarded_frame`, budgets met

*Next build starts from `media-latest + small prod/develop patches` — never recreate/wasm-refine.*
===CORE TOC===
# Canonical Core Audit — smart-core / eiga-core / packaging / robustness

Non-destructive audit of the 21 file-manager zips (last 7 days, hidden excluded) + the live canonical base at `/home/lufuno/crates/` (`smart-core` + `eiga-core` + `smart-ui-egui` + `xtask`). Originals were never modified; all comparisons ran from `/tmp/opencode/canonical-core-audit/extracted/`.

Calendar date: 2026-11-09. Prices / scope context: canonical file manager sold as a $299 non-destructive build. Every `lie / stub / fake` below is therefore treated as a ship-blocker.

---

## 0. Zip inventory (21 zips, 7 lineages)

Hashes are `sha256(size:hash12)` over the extracted `smart-core/src/<file>` bytes. "Canonical" = `/home/lufuno/crates/smart-core/src/…` measured the same way.

| # | Zip (short) | Location | MB | Lineage |
|---|---|---|---|---|
| 1 | rust-wasm-media-manager-latest | `Documents/` | 57 (8011 entries, 68 rs) | **media / eiga** |
| 2 | rust-wasm-media-manager-2 | `Documents/` | 57 (identical to #1 entries) | media / eiga |
| 3 | rust-wasm-media-manager | `Documents/` | 22 | media / eiga (older) |
| 4 | build-production-rust-file-manager-2-very-good | `smart/smart-suite/apps/` | 25 (3031 entries, 51 rs) | **prod** |
| 5 | build-production-rust-file-manager-2 | `smart/smart-suite/apps/` | 276 (25434 entries — bundles `.cargo`/`.rustup`) | prod (bloated) |
| 6 | build-production-rust-file-manager | `smart/smart-suite/apps/` | 25 | prod (older) |
| 7 | develop-rust-wasm-file-manager | `smart/smart-suite/apps/` | 21 | **develop** |
| 8 | build-cross-platform-rust-file-manager-good1 | `work/BIGBOY/` | 7 | cross |
| 9 | build-cross-platform-rust-file-manager-last | `work/BIGBOY/` | 7 | cross (=#8 content) |
| 10 | build-cross-platform-rust-file-manager | `work/BIGBOY/` | 6 | cross (oldest, **no `archive.rs`**) |
| 11 | improve-app-consistency-and-completeness | `work/BIGBOY/` | 19 | improve (=#12/#13) |
| 12 | improve-app-consistency-and-completeness-good | `work/BIGBOY/` | 19 | improve |
| 13 | improve-app-consistency-and-completeness-last | `work/BIGBOY/` | 19 | improve |
| 14 | recreate-file-manager-in-rust-2 | `work/BIGBOY/` | 11 | **recreate** (incompatible arch) |
| 15 | recreate-file-manager-in-rust-bad | `work/BIGBOY/` | 11 | recreate |
| 16 | recreate-file-manager-in-rust-good | `work/BIGBOY/` | 9 | recreate |
| 17 | recreate-file-manager-in-rust | `work/BIGBOY/` | 1 (no Rust at all — TS shell) | recreate (dead) |
| 18 | refine-futuristic-desktop-environment-bad | `work/BIGBOY/` | 19 | desk (`smart-desktop/`) |
| 19 | refine-futuristic-desktop-environment-bad2 | `work/BIGBOY/` | 22 | desk |
| 20 | rust-wasm-ui-refinements | `work/BIGBOY/` | 19 (33 rs — **no `appkit.rs`**) | wasm-refine |
| 21 | rust-wasm-ui-refinements2 | `work/BIGBOY/` | 19 (identical) | wasm-refine |

Structural facts:

- **prod line (#4–#6)**: `smart-files/crates/{smart-core,smart-ui-egui,xtask}` + `scripts/build-appimage.sh` (2888 B, hash `97c12e89`) + React `vite.config.ts`. No `eiga-core`, no `analyze.rs`. UI has **16 files** — missing `highlight.rs`, `pty.rs`, `storage.rs`, `termlive.rs`, `video.rs`. So prod-very-good *cannot* show the Storage workspace or the Eiga player.
- **develop (#7)**: flat `crates/` (no `smart-files/` nesting) + `packaging/`, `docs/`, `dist-appimage/`, `tools/`. Same 51 rs as prod. Has the best `backend.rs` (notify fix + `restore_impl`) and best `app.rs` (70991 B).
- **media line (#1–#3)**: flat `crates/` + `eiga-core` (4 files) + `analyze.rs` + full 22-file UI (adds `highlight`, `pty`, `storage` 54375 B, `termlive`, `video`). This is the canonical superset — **`/home/lufuno/crates` is byte-identical to media-latest** for every `smart-core` file except the small per-file improvements listed in §1. `media-2` and `media-latest` are bit-identical (same 8011 entries); `media` is older (`video.rs` 55274 B vs 76313 B).
- **recreate (#14–#16)**: **a different application**, not a fork. `smart-core` = `{ai.rs 19909, archive.rs 8160, demo.rs ~14k, engine.rs 40615, lib.rs 478, model.rs 9708, search.rs 12417, store.rs 75523, sys.rs 13291, treemap.rs 1987, vfs.rs 12799}`. `Vfs` trait is `&mut self` + `read_at/write_chunk` + `Chunk::Zeros/Data` — mutually incompatible with canonical `Vfs: Send+Sync` (`open_read/create_write`). `Store`/`Action`/`Route` replaces `Backend`/`Event`/`Place`. Only ideas are portable (§1.9).
- **cross (#8–#10)**: oldest file-manager line. #10 has **no `archive.rs` at all** (only zip-era compression lives in UI). #8/#9 add it back (18331 B). Also the only line where `memfs.rs` lacks `paced`/`set_size_hint` (no fake 5 GB animation).
- **improve (#11–#13)**: = prod-older (`app.rs` 65641 B = prod #6, xtask 13629 B). No new core.
- **desk (#18–#19)**: `smart-desktop/` with `smart-desktop/crates/smart-desktop`. Fake `JobKind::Extract` (writes 4 hardcoded text files — see Lie #L-31).
- **wasm-refine (#20–#21)**: 33 rs = 18 core + 15 UI. **Missing `appkit.rs`** and the media-only UI files. `terminal.rs` 86888 B (vs 120099 B full), `photos.rs` 52497 B (vs 74978 B), `overlays.rs` 70157 B. This is a *stripped* build, not a refinement — do not ship as base.
- **React/`vite.config.ts` in every zip** is byte-identical React + Tailwind + `vite-plugin-singlefile` (`@` → `src/`). It is unrelated to the Rust/egui ship and must be excluded from the $299 artefact (§7).

Cargo workspace (canonical): `members = [smart-core, eiga-core, smart-ui-egui, xtask]`, `panic="abort"`, `lto="fat"`. `trash = "5"` resolves to **trash 5.2.9** in `Cargo.lock`.

---

## 1. CORE COMPLETENESS — per-file best version

Verdict key: **KEEP** = canonical as-is, **TAKE** = overwrite canonical with the named variant, **MERGE** = canonical + patch from variant, **DISCARD** = do not merge.

| File | Distinct (size,hash) | Best | Action | Why |
|---|---|---|---|---|
| `backend.rs` | 6: media 30667 `b10aaf`, develop 29461, prod 27404, desk 26281, cross-majority 26241, cross-orig 19322 | **media (canonical) + develop patches** | **MERGE** | Media is the feature superset (archive jobs, `ArchiveDone`, library). Develop contributes two fixes canonical lacks: (a) notify coalescing — canonical `Backend::watch` fires `Event::Changed` on *any* `notify::Result::Ok`, including `Access`/`Modify(Metadata)` from its own `list()` → list→event→list CPU spin; develop filters to `Create/Remove/Modify(Data)` and debounces 150 ms; (b) `restore_trashed` + `restore_impl` via `trash::os_limited::{list,restore_all}` (correct percent-decoding, dedup) vs canonical manual `.trashinfo` parse (§2). Prod/desk/cross are older feature subsets. |
| `archive.rs` | 4: prod 23199/23165, canonical 18331 `704170`, recreate 8160 (different API) | **prod
```

### TOOL: shell [seq 113] 2026-11-09 13:47:20 status=completed

```json
{
  "command": "ls /home/lufuno/smart/smart-protocol-official/schema/ | tr '\\n' ' '; echo\ncat /home/lufuno/smart/smart-protocol-official/schema/media.capnp 2>&1 | head -n 400\necho \"=== VIDEO? ===\"\nls /home/lufuno/smart/smart-protocol-official/schema/ | grep -i -E \"video|media|audio\" \ngrep -rn \"domain-media\\|domain-video\\|smart-protocol-rs\" /home/lufuno/smart/ 2>&1 | head -n 100"
}
```

```
abi.capnp access.capnp accessibility.capnp ai.capnp app.capnp appindex.capnp archive.capnp audio.capnp auth.capnp backup.capnp biometric.capnp bluetooth.capnp boot.capnp buffer.capnp calendar.capnp camera.capnp capability.capnp capture.capnp cli.capnp clipboard.capnp clock.capnp compositor.capnp console.capnp contacts.capnp content.capnp crypto.capnp database.capnp device.capnp diagnostics.capnp discovery.capnp display.capnp dns.capnp endpoint.capnp extension.capnp files.capnp font.capnp framebuffer.capnp gpu.capnp hal.capnp ime.capnp init.capnp input.capnp intent.capnp kernel.capnp key_management.capnp klog.capnp location.capnp logging.capnp manifest.capnp media.capnp module.capnp mount.capnp network.capnp nfc.capnp notification.capnp package.capnp power.capnp print.capnp process.capnp queue.capnp runtime_search.capnp sandbox.capnp screenlock.capnp secret.capnp sensor.capnp service.capnp session.capnp settings.capnp shell.capnp signal.capnp signature.capnp socket.capnp speech.capnp storage.capnp system.capnp system_search.capnp task_scheduler.capnp telephony.capnp theme.capnp time.capnp timezone.capnp translation.capnp tunnel.capnp types.capnp update.capnp usb.capnp vcard.capnp web.capnp windowmanager.capnp 
@0xa1b2c3d4e5f60011;

using T = import "types.capnp";

# ═══════════════════════════════════════════════════════════════════════
# Media - playback control, metadata, volume, transport, playlists,
#         live-stream, chapters, lyrics, analytics, sync-aware.
#
# Improvements beyond the original schema:
#   - MediaPlayerId typed ID replaces raw UInt64.
#   - MediaItemId typed ID replaces bare Text.
#   - All positions/durations use T.Duration (nanosecond precision).
#   - DataHandle for artwork that may exceed 1KB.
#   - Subscription cancel method added.
#   - Pagination on listPlayers.
#   - Custom/other variants on enums for forward compatibility.
#   - Volume and rate documented with units and valid ranges.
#
# Audit-8 enhancements:
#   - Chapter markers for long-form content navigation.
#   - LyricsData with synchronized line/word timing.
#   - LiveStream for real-time broadcast with viewer analytics.
#   - Content-addressed media via contentHash on MediaItem.
#   - E2E encryption flag on MediaItem.
#   - AI-generated tags on MediaMetadata.
#   - Sync-aware playback via syncGroup on MediaPlayer.
#   - Playlist management (create, add, remove, reorder).
#   - Format negotiation and transcoding.
#   - Recommendations, analytics, sharing, download, crossfade.
# ═══════════════════════════════════════════════════════════════════════

interface Media {
    # ── Player discovery ──
    listPlayers @0 (pagination :T.Pagination) -> (players :List(MediaPlayer));
    getPlayer @1 (playerId :T.MediaPlayerId) -> (player :MediaPlayer);

    # ── Transport controls ──
    play @2 (playerId :T.MediaPlayerId) -> T.VoidResult;
    pause @3 (playerId :T.MediaPlayerId) -> T.VoidResult;
    stop @4 (playerId :T.MediaPlayerId) -> T.VoidResult;
    next @5 (playerId :T.MediaPlayerId) -> T.VoidResult;
    previous @6 (playerId :T.MediaPlayerId) -> T.VoidResult;
    seek @7 (playerId :T.MediaPlayerId, position :T.Duration) -> T.VoidResult;

    # ── Metadata / position ──
    getMetadata @8 (playerId :T.MediaPlayerId) -> (metadata :MediaMetadata);
    getPosition @9 (playerId :T.MediaPlayerId) -> (position :T.Duration, duration :T.Duration);

    # ── Volume / rate ──
    # volume: 0.0 (mute) to 1.0 (full). Values outside range are clamped.
    setVolume @10 (playerId :T.MediaPlayerId, volume :Float32) -> T.VoidResult;
    setPosition @11 (playerId :T.MediaPlayerId, position :T.Duration) -> T.VoidResult;

    # ── URI playback ──
    openUri @12 (playerId :T.MediaPlayerId, uri :T.Uri) -> T.VoidResult;

    # ── Subscription ──
    subscribePlayer @13 (playerId :T.MediaPlayerId) -> (subscription :PlayerSubscription);

    # ── Audit-8: Format negotiation ──
    getSupportedFormats @14 (playerId :T.MediaPlayerId) -> (formats :List(Text));

    # ── Audit-8: Transcoding ──
    transcode @15 (mediaId :T.MediaItemId, targetFormat :Text) -> (transcoded :T.DataHandle);

    # ── Audit-8: Recommendations ──
    getRecommendations @16 (playerId :T.MediaPlayerId) -> (items :List(MediaItem));

    # ── Audit-8: Analytics ──
    getAnalytics @17 (mediaId :T.MediaItemId) -> (analytics :MediaAnalytics);

    # ── Audit-8: Sharing ──
    shareMedia @18 (mediaId :T.MediaItemId, targetDeviceId :T.DeviceId) -> (shareLink :T.Uri);

    # ── Audit-8: Download ──
    downloadMedia @19 (mediaId :T.MediaItemId) -> (downloadId :UInt64);

    # ── Audit-8: Crossfade ──
    setCrossfade @20 (playerId :T.MediaPlayerId, duration :T.Duration) -> T.VoidResult;

    # ── Audit-8: Sync-aware playback ──
    setSyncGroup @21 (playerId :T.MediaPlayerId, syncGroup :Text) -> T.VoidResult;

    # ── Audit-8: Playlist management ──
    createPlaylist @22 (name :Text, items :List(MediaItem)) -> (playlistId :UInt64);
    addToPlaylist @23 (playlistId :UInt64, item :MediaItem) -> T.VoidResult;
    removeFromPlaylist @24 (playlistId :UInt64, index :UInt32) -> T.VoidResult;
    reorderPlaylist @25 (playlistId :UInt64, fromIndex :UInt32, toIndex :UInt32) -> T.VoidResult;

    # -- Audit-9: Recommendation scoring --
    getRecommendationsWithScores @26 (playerId :T.MediaPlayerId, page :T.Pagination)
        -> (recommendations :List(RecommendationScore), total :UInt32);

    # -- Audit-9: Sharing preview --
    getSharePreview @27 (mediaId :T.MediaItemId) -> (preview :SharePreview);

    # -- Audit-9: Download status --
    getDownloadStatus @28 (downloadId :UInt64) -> (status :DownloadStatus);
    listDownloads @29 (pagination :T.Pagination) -> (statuses :List(DownloadStatus), total :UInt32);
    pauseDownload @30 (downloadId :UInt64) -> T.VoidResult;
    resumeDownload @31 (downloadId :UInt64) -> T.VoidResult;
    cancelDownload @32 (downloadId :UInt64) -> T.VoidResult;
    subscribeDownload @33 (downloadId :UInt64) -> (subscription :DownloadSubscription);

    # -- Audit-9: Crossfade details --
    setCrossfadeConfig @34 (playerId :T.MediaPlayerId, config :CrossfadeConfig) -> T.VoidResult;
    getCrossfadeConfig @35 (playerId :T.MediaPlayerId) -> (config :CrossfadeConfig);

    # -- Audit-9: Live stream interaction --
    sendLiveInteraction @36 (streamUrl :T.Uri, interaction :LiveInteraction) -> T.VoidResult;
    subscribeLiveInteractions @37 (streamUrl :T.Uri) -> (subscription :LiveInteractionSubscription);

    # -- Audit-9: Transcoding progress --
    subscribeTranscode @38 (transcodeId :UInt64) -> (subscription :TranscodeSubscription);
}

interface PlayerSubscription {
    pull @0 () -> (events :List(PlayerEvent));
    cancel @1 () -> T.VoidResult;
}

# -- Audit-9: Download subscription --
interface DownloadSubscription {
    pull @0 () -> (status :DownloadStatus);
    cancel @1 () -> T.VoidResult;
}

# -- Audit-9: Live interaction subscription --
interface LiveInteractionSubscription {
    pull @0 () -> (interactions :List(LiveInteraction));
    cancel @1 () -> T.VoidResult;
}

# -- Audit-9: Transcode subscription --
interface TranscodeSubscription {
    pull @0 () -> (progress :TranscodeProgress);
    cancel @1 () -> T.VoidResult;
}

struct PlayerEvent {
    union {
        stateChanged @0 :PlaybackStatus;
        metadataChanged @1 :MediaMetadata;
        positionChanged @2 :T.Duration;
        volumeChanged @3 :Float32;
        # Forward compatibility: a backend reporting a playback event the
        # contract never named (a DRM licence change, a transport stall
        # class, a new interpolation mode) must still be dispatchable
        # rather than arriving as an unrecognisable union member with no
        # data (TIMELESSNESS.md:98, DESIGN_PRINCIPLES "Forward
        # compatibility"). Batch B found and deferred this gap; closed
        # here so the file honours the convention every other event union
        # in the tree already follows.
        unknown @4 :T.UnknownEvent;
    }
}

# ──
```

### TOOL: shell [seq 120] 2026-11-09 13:48:07 status=completed

```json
{
  "command": "cat /home/lufuno/smart/smart-protocol-official/schema/audio.capnp 2>&1 | head -n 250\necho \"=== files.capnp ===\"\nhead -n 300 /home/lufuno/smart/smart-protocol-official/schema/files.capnp 2>&1\necho \"=== notification ===\"\nhead -n 150 /home/lufuno/smart/smart-protocol-official/schema/notification.capnp 2>&1"
}
```

```
@0xa1b2c3d4e5f60010;

using T = import "types.capnp";
using C = import "capability.capnp";
using import "capability.capnp".Capability;

# ═══════════════════════════════════════════════════════════════════════
# Audio - volume, devices, streams, focus, spatial, codecs, routing,
#         effects, presets, and accessibility.
#
# Improvements beyond the original schema (audit_analysis.md):
#   - All UInt64 stream/focus IDs -> T.AudioStreamId, T.AudioFocusId
#   - All UInt64 timeouts -> T.Duration
#   - T.Pagination on listDevices/listStreams/listCodecs
#   - cancel() on every subscription interface
#   - T.Error in failure paths
#
# 2026-10-10 integration pass:
#   - FocusEvent -> AudioFocusEvent, HealthStatus -> AudioHealth: three
#     files each declared those names for different concepts.
#     compositor.capnp's surface focus event is SurfaceFocusEvent and the
#     canonical health struct is diagnostics.capnp's HealthStatus.
#   - Every subscription's `pull` is bounded by `maxEvents`, and the
#     convenience `pull*` methods on the root forward the bound. A device
#     arrival burst used to be able to flood one message.
#   - codec :Text -> T.CodecId typed
#   - custom/other variants on all enums
#   - H2: the recording and microphone consent shapes are deleted; both
#     surfaces ask for the tree's canonical approval (capability.capnp)
#     and receive the canonical `AuditEntry` receipt
#   - SpatialProfile has typed ChannelMode
#   - DuckPolicy gained duckKind union
#
# Audit-7 enhancements (zero-cost, info-density):
#   - AudioEffect: parametric EQ, reverb, compressor, limiter, etc.
#   - AudioPreset: user-facing audio profiles (flat, bass boost, etc.)
#   - GlobalDuckPolicy: system-wide duck for incoming calls
#   - AudioStream enhancements (bufferSize, underrunCount)
#   - LoudnessEnhancer for accessibility
#   - AudioMetering for real-time level monitoring
# ═══════════════════════════════════════════════════════════════════════

interface Audio {
    # -- Core --
    getState @0 () -> (state :AudioState);

    listDevices @1 (pagination :T.Pagination) -> (devices :List(AudioDevice), total :UInt32);
    listStreams @2 (pagination :T.Pagination) -> (streams :List(AudioStream), total :UInt32);

    setVolume @3 (device :T.AudioDeviceId, volume :Float32) -> T.VoidResult;
    setMute @4 (device :T.AudioDeviceId, mute :Bool) -> T.VoidResult;
    setStreamVolume @5 (streamId :T.AudioStreamId, volume :Float32) -> T.VoidResult;
    setStreamMute @6 (streamId :T.AudioStreamId, mute :Bool) -> T.VoidResult;

    # -- Device lifecycle --
    enableDevice @7 (device :T.AudioDeviceId) -> T.VoidResult;
    disableDevice @8 (device :T.AudioDeviceId) -> T.VoidResult;

    # -- Subscriptions --
    subscribeState @9 () -> (subscription :AudioStateSubscription);
    pullState @10 (subscription :AudioStateSubscription, maxEvents :UInt32)
                 -> (state :AudioState);
    cancelState @11 (subscription :AudioStateSubscription) -> T.VoidResult;

    subscribeDevices @12 () -> (subscription :DevicesSubscription);
    pullDevices @13 (subscription :DevicesSubscription, maxEvents :UInt32)
                   -> (devices :List(AudioDevice));
    cancelDevices @14 (subscription :DevicesSubscription) -> T.VoidResult;

    subscribeStreams @15 () -> (subscription :StreamsSubscription);
    pullStreams @16 (subscription :StreamsSubscription, maxEvents :UInt32)
                    -> (streams :List(AudioStream));
    cancelStreams @17 (subscription :StreamsSubscription) -> T.VoidResult;

    # -- Focus / priority --
    requestFocus @18 (request :FocusRequest) -> (focusId :T.AudioFocusId);
    releaseFocus @19 (focusId :T.AudioFocusId) -> T.VoidResult;
    subscribeFocus @20 () -> (subscription :FocusSubscription);
    pullFocus @21 (subscription :FocusSubscription, maxEvents :UInt32)
                  -> (events :List(AudioFocusEvent));
    cancelFocus @22 (subscription :FocusSubscription) -> T.VoidResult;

    # -- Spatial --
    setStreamPosition @23 (streamId :T.AudioStreamId, position :SpatialPosition) -> T.VoidResult;
    setListenerPosition @24 (listener :SpatialPosition) -> T.VoidResult;
    setStreamSpatialProfile @25 (streamId :T.AudioStreamId, profile :SpatialProfile) -> T.VoidResult;

    # -- Recording --
    # H2: the canonical approval primitive, not a second consent flow.
    # The request is the tree's canonical `ApprovalRequest`; the result
    # is the canonical `Approval` reference — a reference, a bounded
    # pull and a cancel. The decision arrives on it, the grant is an
    # ordinary `Capability`, and the receipt is the canonical
    # `AuditEntry`.
    #
    # The recording-specific context travels in
    # `ApprovalRequest.metadata`:
    #   "appId"       the requesting application (T.AppId, as text)
    #   "purpose"     the user-visible purpose string
    #   "dataTypes"   comma-separated recordable data types (see the
    #                 recordable-data-type conformance tokens below)
    #   "perSession"  "true" when the grant must die with the session
    # The request's `target` names what the grant covers (for example
    # "audio/recording"); the provider resolves it.
    recordConsent @26 (request :C.ApprovalRequest) -> (approval :C.Approval);
    declareRecordingPolicy @27 (policy :RecordingPolicy) -> T.VoidResult;

    # -- Codecs --
    listCodecs @28 (pagination :T.Pagination) -> (codecs :List(AudioCodec), totalCount :UInt32);
    negotiateCodec @29 (streamId :T.AudioStreamId, codec :T.CodecId) -> (result :NegotiationResult);
    subscribeCodecs @30 () -> (subscription :CodecSubscription);
    pullCodecs @31 (subscription :CodecSubscription, maxEvents :UInt32)
                   -> (codecs :List(AudioCodec));
    cancelCodecs @32 (subscription :CodecSubscription) -> T.VoidResult;

    # -- Routing --
    setRoute @33 (streamId :T.AudioStreamId, route :AudioRoute) -> T.VoidResult;
    duckOnCall @34 (streamId :T.AudioStreamId, policy :DuckPolicy) -> T.VoidResult;

    # -- Audit-7: Effects --
    listEffects @35 (pagination :T.Pagination) -> (effects :List(AudioEffect), totalCount :UInt32);
    addEffect @36 (streamId :T.AudioStreamId, effect :AudioEffect) -> (effectId :UInt32);
    removeEffect @37 (effectId :UInt32) -> T.VoidResult;
    setEffectEnabled @38 (effectId :UInt32, enabled :Bool) -> T.VoidResult;
    setEffectParameter @39 (effectId :UInt32, param :EffectParameter) -> T.VoidResult;

    # -- Audit-7: Presets --
    listPresets @40 (pagination :T.Pagination) -> (presets :List(AudioPreset), totalCount :UInt32);
    applyPreset @41 (presetId :Text) -> T.VoidResult;
    saveCustomPreset @42 (preset :AudioPreset) -> (presetId :Text);
    deleteCustomPreset @43 (presetId :Text) -> T.VoidResult;

    # -- Audit-7: Global Duck --
    setGlobalDuckPolicy @44 (policy :GlobalDuckPolicy) -> T.VoidResult;
    getGlobalDuckPolicy @45 () -> (policy :GlobalDuckPolicy);

    # -- Audit-7: Metering --
    getStreamMetering @46 (streamId :T.AudioStreamId) -> (meter :AudioMetering);
    getDeviceMetering @47 (device :T.AudioDeviceId) -> (meter :AudioMetering);

    # -- Audit-8: Audio session coordination --
    createSession @48 (request :AudioSessionRequest) -> (sessionId :Text);
    joinSession @49 (sessionId :Text, appId :T.AppId) -> T.VoidResult;
    leaveSession @50 (sessionId :Text, appId :T.AppId) -> T.VoidResult;
    getSession @51 (sessionId :Text) -> (session :AudioSession);
    listSessions @52 (pagination :T.Pagination) -> (sessions :List(AudioSession), total :UInt32);
    setSessionRouting @53 (sessionId :Text, routing :CrossDeviceRouting) -> T.VoidResult;

    # -- Audit-9: Typed transport query --
    getAudioTransport @54 (device :T.AudioDeviceId) -> (transport :Text);

    # -- Audit-9: Typed channel layouts --
    listChannelLayouts @55 (device :T.AudioDeviceId) -> (layouts :List(ChannelLayout));

    # -- Audit-9: Per-role duck levels --
    setPerRoleDuckLevels @56 (levels :List(PerRoleDuckLevel)) -> T.
```

### TOOL: shell [seq 127] 2026-11-09 13:48:10 status=completed

```json
{
  "command": "ls /home/lufuno/smart/smart-protocol-rs/src/ 2>&1\ngrep -rn \"domain-media\\|domain-video\\|Media\\|Video\" /home/lufuno/smart/smart-protocol-rs/src/lib.rs 2>&1 | head -n 100\ncat /home/lufuno/smart/smart-protocol-rs/src/lib.rs 2>&1 | head -n 200\necho ===; ls /home/lufuno/Documents/rust-wasm-media-manager-latest/tools/ /home/lufuno/Documents/rust-wasm-media-manager-latest/web/ 2>&1\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/tools/build_web.sh 2>&1\necho ===XTASK===; cat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/xtask/src/main.rs 2>&1 | head -n 200"
}
```

```
lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

include!(concat!(env!("OUT_DIR"), "/schema_modules.rs"));
include!(concat!(env!("OUT_DIR"), "/contract.rs"));

pub use schema::*;
pub use contract::*;
===
/home/lufuno/Documents/rust-wasm-media-manager-latest/tools/:
build_web.sh
gen_icons.py

/home/lufuno/Documents/rust-wasm-media-manager-latest/web/:
dev.html
#!/usr/bin/env bash
# Build the egui frontend for the browser and drop the wasm-bindgen output next to the Vite entry.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build -p smart-ui-egui --lib --target wasm32-unknown-unknown --profile wasm
wasm-bindgen --target web --no-typescript --out-dir ../src/smart target/wasm32-unknown-unknown/wasm/smart_ui_egui.wasm
if command -v wasm-opt >/dev/null; then wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext --enable-mutable-globals -o ../src/smart/smart_ui_egui_bg.wasm ../src/smart/smart_ui_egui_bg.wasm || true; fi
ls -la ../src/smart
sed -i "s#module_or_path = new URL('smart_ui_egui_bg.wasm', import.meta.url);#throw new Error('wasm url required');#" ../src/smart/smart_ui_egui.js
===XTASK===
//! Build tooling for Smart Files, written in Rust so the whole pipeline stays JS-free.
//!
//!   cargo run -p xtask --release -- assets   rasterise icons, synthesise demo audio, make wallpapers, copy fonts/thumbs
//!   cargo run -p xtask --release -- bundle   wasm-bindgen output -> ONE self-contained ../index.html (wasm embedded)
//!
//! The HTML shell below is *generated* here (it is the standard wasm-bindgen bootstrap: decode wasm, call `start`).
use std::{fs, path::{Path, PathBuf}, process::Command};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap()
}

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_default();
    match cmd.as_str() {
        "assets" => assets(),
        "bundle" => bundle(),
        _ => eprintln!("usage: xtask assets|bundle"),
    }
}

// ───────────────────────────────────────────── assets ─────────────────────────────────────────────

fn assets() {
    let r = root();
    let a = r.join("assets");
    fs::create_dir_all(a.join("icons")).unwrap();
    fs::create_dir_all(a.join("fonts")).unwrap();
    fs::create_dir_all(a.join("audio")).unwrap();
    fonts(&a);
    icons(&r, &a);
    audio(&a);
    wallpapers(&a);
}

fn fonts(a: &Path) {
    for (src, dst) in [("Regular", "regular"), ("Medium", "medium"), ("SemiBold", "semibold"), ("Bold", "bold")] {
        let from = PathBuf::from(format!("/usr/share/fonts/opentype/inter/Inter-{src}.otf"));
        if from.exists() {
            fs::copy(&from, a.join(format!("fonts/Inter-{dst}.otf"))).unwrap();
        } else {
            eprintln!("missing font {from:?} (apt-get install fonts-inter)");
        }
    }
}

fn icons(r: &Path, a: &Path) {
    let json = std::env::var("SOLAR_JSON").unwrap_or("/tmp/ic/node_modules/@iconify-json/solar/icons.json".into());
    let data: serde_json::Value = serde_json::from_str(&fs::read_to_string(&json).expect("SOLAR_JSON (npm i @iconify-json/solar)")).unwrap();
    let find = |n: &str| -> Option<String> {
        if let Some(i) = data["icons"].get(n) {
            return i["body"].as_str().map(String::from);
        }
        let p = data["aliases"].get(n)?["parent"].as_str()?;
        data["icons"][p]["body"].as_str().map(String::from)
    };
    let list = fs::read_to_string(a.join("icon_names.txt")).unwrap();
    let mut gen = String::from("// @generated by `cargo run -p xtask -- assets` from assets/icon_names.txt — do not edit\npub fn bytes(name: &str, v: u8) -> Option<&'static [u8]> {\n    Some(match (name, v) {\n");
    let mut count = 0;
    for line in list.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (name, src) = line.split_once('=').unwrap_or((line, line));
        for (code, suffix) in [('o', "linear"), ('d', "bold-duotone"), ('f', "bold")] {
            let Some(body) = find(&format!("{src}-{suffix}")) else {
                eprintln!("icon missing: {src}-{suffix}");
                continue;
            };
            let svg = format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="128" height="128">{}</svg>"#,
                body.replace("currentColor", "#ffffff")
            );
            let tree = resvg::usvg::Tree::from_str(&svg, &resvg::usvg::Options::default()).expect("svg");
            let mut pm = resvg::tiny_skia::Pixmap::new(128, 128).unwrap();
            resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pm.as_mut());
            pm.save_png(a.join(format!("icons/{name}_{code}.png"))).unwrap();
            gen.push_str(&format!("        (\"{name}\", b'{code}') => include_bytes!(\"../../../assets/icons/{name}_{code}.png\"),\n"));
            count += 1;
        }
    }
    gen.push_str("        _ => return None,\n    })\n}\n");
    fs::write(r.join("crates/smart-ui-egui/src/icons_gen.rs"), gen).unwrap();
    println!("icons: {count} rasters");
}

struct Rng(u32);
impl Rng {
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

// Tiny additive synth so the demo library has real, playable audio.
fn audio(a: &Path) {
    const SR: u32 = 22_050;
    let tau = std::f32::consts::TAU;
    let put = |name: &str, secs: f32, f: &mut dyn FnMut(f32) -> f32| {
        let n = (SR as f32 * secs) as usize;
        let mut pcm = Vec::with_capacity(n * 2);
        for i in 0..n {
            let t = i as f32 / SR as f32;
            let fade = (1.0 - t / secs).clamp(0.0, 1.0).min(t * 200.0).min(1.0);
            let v = (f(t) * fade).clamp(-1.0, 1.0);
            pcm.extend_from_slice(&((v * 26000.0) as i16).to_le_bytes());
        }
        let mut w = Vec::new();
        w.extend_from_slice(b"RIFF");
        w.extend_from_slice(&(36 + pcm.len() as u32).to_le_bytes());
        w.extend_from_slice(b"WAVEfmt ");
        w.extend_from_slice(&16u32.to_le_bytes());
        w.extend_from_slice(&1u16.to_le_bytes());
        w.extend_from_slice(&1u16.to_le_bytes());
        w.extend_from_slice(&SR.to_le_bytes());
        w.extend_from_slice(&(SR * 2).to_le_bytes());
        w.extend_from_slice(&2u16.to_le_bytes());
        w.extend_from_slice(&16u16.to_le_bytes());
        w.extend_from_slice(b"data");
        w.extend_from_slice(&(pcm.len() as u32).to_le_bytes());
        w.extend_from_slice(&pcm);
        fs::write(a.join(format!("audio/{name}.wav")), w).unwrap();
    };
    let beat = 60.0 / 126.0;
    put("kick_logdrum", 3.8, &mut |t| {
        let tt = t % beat;
        let f = 58.0 + 90.0 * (-tt * 28.0).exp();
        ((tt * f * tau).sin() * (-tt * 5.0).exp()) * 0.95 + (tau * 2.0 * f * tt).sin() * 0.15 * (-tt * 14.0).exp()
    });
    put("logdrum_conga", 3.2, &mut |t| {
        let step = beat / 2.0;
        let tt = t % step;
        let base = [196.0f32, 247.0, 220.0, 294.0][(t / step) as usize % 4];
        (tt * base * tau).sin() * (-tt * 11.0).exp() * 0.8
    });
    let mut rng = Rng(0x9e3779b9);
    put("shaker", 3.0, &mut |t| {
        let step = beat / 4.0;
        let tt = t % step;
        rng.next() * (-tt * 38.0).exp() * if ((t / step) as i32) % 2 == 0 { 0.7 } else { 0.4 }
    });
    put("pad_warm", 4.0, &mut |t| {
        let mut s = 0.0;
        for (k, f) in [220.0f32, 277.2, 329.6, 440.0].iter().enumerate() {
            s += (t * f * tau + (t * 0.5 + k as f32).sin() * 0.4).sin() * 0.18;
            s += (t * f * 1.004 * tau).sin() * 0.1;
        }
        s * (0.6 + 0.4 * (t * 0.7).sin())
    });
    put("piano_lead", 3.6, &mut |t| {
        let notes = [523.25f32, 659.25, 587.33, 783.99, 659.25, 523.25, 587.33, 440.0];
        let d = 0.45;
        let tt = t % d;
        let f = notes[((t / d) as usize) % notes.len()];
        ((tt * f * tau).sin() + 0.4 * (tt * f * 2.0 * 
```

### TOOL: shell [seq 134] 2026-11-09 13:48:14 status=completed

```json
{
  "command": "grep -rn \"appimage\\|AppImage\\|build.*appimage\" /home/lufuno/Documents/rust-wasm-media-manager-latest/ 2>&1 | head -n 60\nls /home/lufuno/Documents/rust-wasm-media-manager-latest/scripts/ /home/lufuno/Documents/rust-wasm-media-manager-latest/tools/ 2>&1\nls /home/lufuno/smart/smart-suite-official/ 2>&1\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/README.md 2>&1 | head -n 150\necho ===MEMFS-DEMO-AUDIO-DEVICES===\nhead -n 80 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/audio.rs 2>&1\nhead -n 80 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/devices.rs 2>&1\nhead -n 60 /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs 2>&1"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs:59:    fs.put(&p(&format!("{dw}/smart-daw-0.4.2.AppImage")), vec![0u8; 64], Some(22_400 * KB), ago(30.0));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs:61:    fs.put(&p(&format!("{dw}/beta-notes.md")), b"# SMART-DAW beta\n\n- Piano roll snaps to the 8 px grid\n- Log drum preset needs more punch\n- **Ship** the AppImage on Friday\n".to_vec(), Some(12 * KB), ago(36.0));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/demo.rs:80:    fs.put(&p("Downloads/smart-daw-0.4.2.AppImage"), vec![0u8; 32], Some(22_400 * KB), ago(41.0));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs:98:            "appimage" | "exe" | "msi" | "app" | "deb" | "rpm" | "apk" | "dmg" | "flatpak" => Kind::App,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/highlight.rs:76:/// preview uses the lexer below; the desktop app (and AppImage) gets full parse-tree highlighting.
ls: cannot access '/home/lufuno/Documents/rust-wasm-media-manager-latest/scripts/': No such file or directory
/home/lufuno/Documents/rust-wasm-media-manager-latest/tools/:
build_web.sh
gen_icons.py
audits
specs
# Smart Shell

An experimental desktop shell written in Rust: a file manager and a few apps that share one design system
(`assets/tokens.toml`) and one codebase. It runs natively (Linux first) and, as a preview, in the browser through
WebAssembly.

**What this is not:** a finished product, or an OS. Windows, macOS and FreeBSD builds are untested. The web preview
uses an in-memory demo file system, so nothing in it touches your disk.

## Is this "a smart suite"?

Yes, in the sense that GNOME Core, KDE Gear and the apps bundled with macOS are suites: one design language, shared
components, apps that hand work to each other (Files opens videos in Eiga, folders in Terminal, images in Photos).
The prototype's job is to prove that one Rust toolkit and one token file can drive all of them.

## Apps

| App | What it does | Desktop | Web preview |
|---|---|---|---|
| **Files** | Tabs, search, tags, archives, devices, undoable copy/move, Open With | real files | demo files |
| **Storage** (a tab inside Files) | Overview, Cleanup, Treemap, Folder graph, Biggest | scans your disk on a worker thread | scans the demo drive |
| **Terminal** | A real shell on a pty (vt100 screen, scrollback, selection, resize), or Smart command blocks | `$SHELL` | virtual shell on the demo file system |
| **Photos** | Library, viewer, light editing | yes | yes |
| **Eiga (映画)** | Video player: mpv-style keys, subtitles, chapters, A-B loop, playlists | decodes with `ffmpeg` | built-in test card |

## Storage workspace

Click the drive card at the bottom of the sidebar (it turns amber at 80 % and red at 90 %) to open Storage in a new tab.

* **Overview**: usage gauge, content breakdown, the biggest wins, a warning banner when the drive is nearly full.
* **Cleanup**: *context-aware* junk detection, not just "big files": `target/` next to a `Cargo.toml`, `node_modules`,
  `.venv`, `__pycache__`, framework caches, `~/.cache`, `~/.cargo/registry`, `~/.npm/_cacache`, the Trash, partial
  downloads, huge logs, old installers in Downloads, "Report (1).pdf" copies and large files nobody touched for months.
  Every item has a safety grade (Safe, Review, Careful) and, where it applies, how it comes back (`cargo build`,
  `npm install`). Nothing is deleted without a confirmation, and everything goes to the Trash.
* **Map**: squarified treemap with drill-down. **Graph**: force-directed, Obsidian-style view of the folder tree
  (drag nodes, zoom, pan, click to focus, double-click to open). **Biggest**: largest files and folders.

The scanner (`smart-core/src/analyze.rs`) keeps only folders plus each folder's ten biggest files, so memory stays small
even on huge disks, and it is a pure state machine with unit tests.

## Eiga (映画)

Engine: `crates/eiga-core` (no UI code, 39 tests). Surface: `crates/smart-ui-egui/src/video.rs`.

* Keys follow mpv: `Space` pause, `←/→` ±5 s, `Shift+←/→` ±1 s, `↑/↓` ±1 min, `,` `.` frame step, `[` `]` speed ×1.1,
  `{` `}` half/double, `Backspace` reset speed, `9/0` volume, `m` mute, `v` subtitles, `z/x` audio delay,
  `Shift+z/x` subtitle delay, `f` fullscreen, `i` statistics, `s` screenshot, `Shift+L` A-B loop, `Ctrl+G` jump to time,
  `1-8` seek to 10-80 %. Press `H` in the player for the full sheet.
* Subtitles: SRT, WebVTT, ASS/SSA; sidecar files (`Movie.srt`, `Movie.en.srt`) and embedded text tracks; UTF-8,
  UTF-16 and Latin-1; per-track delay and size.
* Decoding: `ffmpeg`/`ffprobe` as child processes (video as raw RGBA over a pipe, audio as f32 PCM into rodio). This keeps
  the Rust side small and supports every format ffmpeg does. Audio and video share a wall clock, not a hardware
  clock, so expect a few tens of milliseconds of drift; use `z`/`x` to trim. There is no hardware-accelerated
  presentation yet (decoding can use `-hwaccel auto`).
* The web preview cannot spawn processes, so it plays a procedural test card with the real subtitle and control stack.

## This pass: UX refinements

* **Dialogs size to their content** (no dead space below the buttons), close from the corner, and spring in / fade out.
  The Devices dialog is a two-column grid with usage bars that turn amber at 80 % and red at 90 %.
* **About** says exactly what this is: a prototype, what each app does, and what the web preview can and cannot do.
* **Quick Look** for text and code: no fake window chrome, scrollable, selectable, syntax-coloured (tree-sitter on the
  desktop, a built-in lexer everywhere), with line numbers for code.
* **Storage** opens from the drive card in the sidebar (the card itself warns when the drive is nearly full). The app
  bar truncates its title on small bars and the clean action collapses to an icon button — nothing ever overlaps.
* **Trash**: a sidebar entry and a real view over your freedesktop Trash, with Restore and Empty Trash.
* **Tabs**: drag a folder onto a tab and it previews after a short hover delay; dropping a file on a tab targets that
  folder. Every segmented control and settings rail glides on the same spring.
* **Eiga refinements**: the sidebar starts hidden (toggle it back); fullscreen releases the window rounding and shows a
  background-free top bar when the pointer nears the top; the transport glyphs are light and lifted by a shadow so they
  read over bright footage; the rounded video corners no longer bleed; right-click gives a full context menu; the
  playlist sidebar has per-item menus; you can drag videos and subtitle files straight onto the player; `Ctrl+O` opens
  the system file dialog (rfd on desktop); and Eiga has its own settings sheet. Closing the player frees the decoder,
  textures and audio device so memory returns to the system.

## Terminal

On the desktop the Terminal runs your `$SHELL` on a pseudo-terminal (`portable-pty`) and renders it through a vt100
screen, so vim, htop, ssh, less, tmux and your prompt work. `Ctrl+Shift+C/V` copy/paste, wheel and `Shift+PgUp` scroll
the 5000-line scrollback, drag to select, double-click selects a word. Right-click to switch to the Smart command-block
view. On the web there are no processes, so the virtual shell remains.

## Build

```sh
cargo run -p xtask --release -- assets      # icons, audio, wallpapers (needs the Solar icon JSON, fonts-inter)
cargo run --release -p smart-ui-egui        # desktop
cargo test -p smart-core -p eiga-core -p smart-ui-egui
# web preview
cargo build -p smart-ui-egui --lib --target wasm32-unknown-unknown --profile wasm
cargo run -p xtask --release -- bundle      # writes ./index.html with the wasm embedded
```

Desktop needs `ffmpeg` for video and the ALSA development headers to build audio. tree-sitter highlighting (Rust,
Python, JavaScript, JSON, TOML, Bash, CSS, HTML, C, Go, Markdown) is compiled
```

### TOOL: shell [seq 141] 2026-11-09 13:48:19 status=completed

```json
{
  "command": "echo ===refine check===\nls /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad.zip /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip 2>&1 | head -n 20\nunzip -l /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip 2>&1 | grep -i \"eiga\\|video\\|media\" | head -n 60\nunzip -l /home/lufuno/Documents/rust-wasm-media-manager-latest.zip 2>&1 | grep -E \"eiga-core|video\\.rs|analyze\" | head -n 60\nunzip -l /home/lufuno/Documents/rust-wasm-media-manager.zip 2>&1 | grep -E \"eiga-core|video\\.rs|analyze\" | head -n 60\necho ===zip diff newest===\nls -lh /home/lufuno/Documents/rust-wasm-media-manager*.zip\ndiff <(unzip -l /home/lufuno/Documents/rust-wasm-media-manager-latest.zip | awk '{print $4}' | sort) <(unzip -l /home/lufuno/Documents/rust-wasm-media-manager-2.zip | awk '{print $4}' | sort) | head -n 40\necho \"=== appimage script hunt ===\"\nfind /home/lufuno/Documents/rust-wasm-media-manager-latest -name \"*.sh\" -o -name \"*.yaml\" -o -name \"*.yml\" 2>/dev/null | head -n 20\nfind /tmp/opencode/canonical-core-audit/extracted -name \"build-appimage.sh\" 2>/dev/null | head -n 10"
}
```

```
===refine check===
/home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad.zip
/home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip
        0  11-06-2026 17:02   crates/eiga-core/
      283  11-06-2026 17:02   crates/eiga-core/Cargo.toml
        0  11-06-2026 17:02   crates/eiga-core/src/
     2991  11-06-2026 17:02   crates/eiga-core/src/lib.rs
    30125  11-06-2026 17:02   crates/eiga-core/src/player.rs
    30036  11-06-2026 17:02   crates/eiga-core/src/source.rs
    14951  11-06-2026 17:02   crates/eiga-core/src/subs.rs
    31515  11-06-2026 17:02   crates/smart-core/src/analyze.rs
    76313  11-06-2026 17:02   crates/smart-ui-egui/src/video.rs
        0  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-1f0b8094b69612b7/
       48  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-1f0b8094b69612b7/invoked.timestamp
       16  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-1f0b8094b69612b7/test-lib-eiga_core
      420  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-1f0b8094b69612b7/test-lib-eiga_core.json
        0  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-38f46411961c6224/
       48  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-38f46411961c6224/invoked.timestamp
       16  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-38f46411961c6224/lib-eiga_core
      413  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-38f46411961c6224/lib-eiga_core.json
        0  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/
       48  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/invoked.timestamp
       16  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/lib-eiga_core
      414  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/lib-eiga_core.json
        0  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-d234157ebbcfb7a2/
       48  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-d234157ebbcfb7a2/invoked.timestamp
       16  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-d234157ebbcfb7a2/lib-eiga_core
      414  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-d234157ebbcfb7a2/lib-eiga_core.json
        0  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/
       48  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/invoked.timestamp
       16  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/test-lib-eiga_core
      419  11-06-2026 17:02   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/test-lib-eiga_core.json
        0  11-06-2026 17:02   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/
       48  11-06-2026 17:02   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/invoked.timestamp
       16  11-06-2026 17:02   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/lib-eiga_core
      454  11-06-2026 17:02   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/lib-eiga_core.json
        0  11-06-2026 09:21   crates/eiga-core/
      283  11-06-2026 09:21   crates/eiga-core/Cargo.toml
        0  11-06-2026 09:21   crates/eiga-core/src/
     2991  11-06-2026 09:21   crates/eiga-core/src/lib.rs
    30125  11-06-2026 09:21   crates/eiga-core/src/player.rs
    30036  11-06-2026 09:21   crates/eiga-core/src/source.rs
    14951  11-06-2026 09:21   crates/eiga-core/src/subs.rs
    31515  11-06-2026 09:21   crates/smart-core/src/analyze.rs
    55274  11-06-2026 09:21   crates/smart-ui-egui/src/video.rs
        0  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-38f46411961c6224/
       48  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-38f46411961c6224/invoked.timestamp
       16  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-38f46411961c6224/lib-eiga_core
      413  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-38f46411961c6224/lib-eiga_core.json
        0  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/
       48  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/invoked.timestamp
       16  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/lib-eiga_core
      414  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-81bcb15c6f7147cf/lib-eiga_core.json
        0  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/
       48  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/invoked.timestamp
       16  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/test-lib-eiga_core
      419  11-06-2026 09:21   target/debug/.fingerprint/eiga-core-f17f1b9caf41051d/test-lib-eiga_core.json
        0  11-06-2026 09:21   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/
       48  11-06-2026 09:21   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/invoked.timestamp
       16  11-06-2026 09:21   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/lib-eiga_core
      454  11-06-2026 09:21   target/wasm32-unknown-unknown/wasm/.fingerprint/eiga-core-3ad602632e077c51/lib-eiga_core.json
===zip diff newest===
-rw-r--r-- 1 lufuno lufuno 58M Nov  6 13:19 /home/lufuno/Documents/rust-wasm-media-manager-2.zip
-rw-r--r-- 1 lufuno lufuno 58M Nov  6 19:02 /home/lufuno/Documents/rust-wasm-media-manager-latest.zip
-rw-r--r-- 1 lufuno lufuno 23M Nov  6 11:21 /home/lufuno/Documents/rust-wasm-media-manager.zip
=== appimage script hunt ===
/home/lufuno/Documents/rust-wasm-media-manager-latest/tools/build_web.sh
/tmp/opencode/canonical-core-audit/extracted/prod-very-good/scripts/build-appimage.sh

```

### TOOL: shell [seq 148] 2026-11-09 13:48:23 status=completed

```json
{
  "command": "cat /tmp/opencode/canonical-core-audit/extracted/prod-very-good/scripts/build-appimage.sh 2>&1\necho ===; cat /tmp/opencode/canonical-core-audit/extracted/develop/packaging/* 2>&1 | head -n 100; ls -R /tmp/opencode/canonical-core-audit/extracted/develop/packaging/ 2>&1\necho ===recent eiga check===\nfind /home/lufuno -type f -name \"*.rs\" -not -path \"*/.*\" -not -path \"*/target/*\" 2>/dev/null | xargs grep -l \"eiga-core\\|FfmpegSource\\|AudioPipe\" 2>/dev/null | head -n 30\necho ===CARGO-ROOT-LIVE===\nls /home/lufuno/crates/ 2>&1 | head -n 30; diff -rq /home/lufuno/crates/eiga-core /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core 2>&1 | head -n 20"
}
```

```
#!/usr/bin/env bash
# Build Smart Files as a portable AppImage (x86_64 glibc Linux).
# Usage:  scripts/build-appimage.sh          (uses the fast profile)
#         scripts/build-appimage.sh release  (max-optimised, slower)
set -euo pipefail
cd "$(dirname "$0")/.."

PROFILE="${1:-nativefast}"
HERE="$(pwd)"
SF="$HERE/smart-files"
TOOL="$SF/tools/appimagetool-x86_64"
APPDIR="$SF/target/AppDir"

. "$HOME/.cargo/env" 2>/dev/null || true

echo "==> [1/4] native build (profile: $PROFILE)"
(cd "$SF" && cargo build -p smart-ui-egui --bin smart-files --profile "$PROFILE")
BIN="$SF/target/$PROFILE/smart-files"

echo "==> [2/4] assemble AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/256x256/apps"
cp "$BIN" "$APPDIR/usr/bin/smart-files"

cat > "$APPDIR/usr/share/applications/smart-files.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=Smart Files
Comment=Rust file manager & app shell
Exec=smart-files
Icon=smart-files
Categories=System;FileManager;
Terminal=false
EOF
cp "$APPDIR/usr/share/applications/smart-files.desktop" "$APPDIR/"

# Icon: reuse a generated wallpaper-derived png if present, else a tiny placeholder.
ICON_SRC="$SF/assets/icon-256.png"
if [ ! -f "$ICON_SRC" ] && command -v python3 >/dev/null; then
python3 - "$ICON_SRC" <<'PY'
import struct, zlib, sys
w = h = 256
def px(x, y):
    # deep-navy rounded tile with a cobalt folder glyph
    if x < 14 or y < 14 or x > w-15 or y > h-15:
        c = (0, 0, 0, 0)
    elif 60 <= y <= 96 and 56 <= x <= 120:
        c = (61, 123, 247, 255)
    elif 88 <= y <= 196 and 40 <= x <= 216:
        c = (111, 155, 255, 255)
    else:
        c = (16, 22, 34, 255)
    return c
raw = b""
for y in range(h):
    raw += b"\x00"
    for x in range(w):
        raw += bytes(px(x, y))
def chunk(t, d):
    return struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d))
png = b"\x89PNG\r\n\x1a\n"
png += chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
png += chunk(b"IDAT", zlib.compress(raw, 9))
png += chunk(b"IEND", b"")
open(sys.argv[1], "wb").write(png)
PY
fi
if [ -f "$ICON_SRC" ]; then
  cp "$ICON_SRC" "$APPDIR/usr/share/icons/hicolor/256x256/apps/smart-files.png"
  cp "$ICON_SRC" "$APPDIR/smart-files.png"
fi

cat > "$APPDIR/AppRun" <<'EOF'
#!/bin/sh
SELF="$(readlink -f "$0")"
exec "$(dirname "$SELF")/usr/bin/smart-files" "$@"
EOF
chmod +x "$APPDIR/AppRun"

echo "==> [3/4] fetch appimagetool if needed"
if [ ! -x "$TOOL" ]; then
  curl -sL -o "$TOOL" https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
  chmod +x "$TOOL"
fi

echo "==> [4/4] pack AppImage"
OUT="$SF/Smart-Files-x86_64.AppImage"
ARCH=x86_64 APPIMAGE_EXTRACT_AND_RUN=1 "$TOOL" "$APPDIR" "$OUT"
chmod +x "$OUT"
echo
echo "Done: $OUT"
echo "Run it:  $OUT      (works on Wayfire, Gnome X11, any glibc distro)"
===
cat: '/tmp/opencode/canonical-core-audit/extracted/develop/packaging/*': No such file or directory
ls: cannot access '/tmp/opencode/canonical-core-audit/extracted/develop/packaging/': No such file or directory
===recent eiga check===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/lib.rs
/home/lufuno/smart/smartui-workspace/examples/media-cinema/src/main.rs
/home/lufuno/smart/smartui-workspace/smartui-media/src/engine/session.rs
/home/lufuno/smart/smartui-workspace/smartui-media/src/engine/ffmpeg_cli.rs
/home/lufuno/crates/smart-ui-egui/src/video.rs
/home/lufuno/crates/eiga-core/src/source.rs
/home/lufuno/crates/eiga-core/src/lib.rs
===CARGO-ROOT-LIVE===
eiga-core
smart-core
smart-ui-egui
xtask

```

### TOOL: shell [seq 155] 2026-11-09 13:48:28 status=completed

```json
{
  "command": "ls /home/lufuno/smart/smartui-workspace/ 2>&1 | head -n 40\nls /home/lufuno/smart/smartui-workspace/smartui-media/ /home/lufuno/smart/smartui-workspace/examples/media-cinema/ 2>&1\necho ===; cat /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/session.rs 2>&1 | head -n 150\necho ===ffmpeg_cli===; cat /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/ffmpeg_cli.rs 2>&1 | head -n 150\ndiff -rq /home/lufuno/crates/eiga-core /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core 2>&1 | head -n 20\necho \"===live vs zip smart-core diff count===\"\ndiff -rq /home/lufuno/crates/smart-core /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core 2>&1 | head -n 30"
}
```

```
1
AGENTS.md
ARCHITECTURE.md
CHECKLIST.md
Cargo.lock
Cargo.toml
DESIGN.md
Design_System.html
INPUT_UX.md
ISSUES.md
PLAN-layout-test.md
PROGRESS.md
RUNTIME_ISSUES.md
TESTING.md
ai-chat
audit-002
bounce-app
design_system
docs
dont_touch_or_delete_by_user
examples
fonts
fox-app
gallery-app
image-test
layout-test
plans
smart-de
smart-protocol
smartui-anim
smartui-app
smartui-core
smartui-dsl
smartui-dsl-format
smartui-icons
smartui-image
smartui-input
smartui-layout
smartui-magick
smartui-media
/home/lufuno/smart/smartui-workspace/examples/media-cinema/:
Cargo.toml
src

/home/lufuno/smart/smartui-workspace/smartui-media/:
Cargo.toml
src
tests
===
//! `PlaybackSession` — a complete, timeline-authoritative A/V player.
//!
//! ## Architecture contract (why this exists)
//!
//! Apps previously re-implemented play/pause/seek state machines and got
//! them wrong (seek loops, stale subtitles, clock fights between decoders
//! and the UI). This module owns ALL of it, once, behind a small API:
//!
//! ```text
//! session.play() / .pause() / .seek_ms(t)
//! session.position_ms()      // THE timeline — monotonic, seek-exact
//! session.subtitle_text()    // pure function of the timeline
//! session.tick()             // call once per frame; advances everything
//! ```
//!
//! ## Invariants (enforced here, tested below)
//! 1. **The timeline is king.** Decoders NEVER move it. Only explicit
//!    user seeks set it backward; playback only moves it forward.
//! 2. **Seek is exact**: after `seek_ms(t)` returns, `position_ms() == t`
//!    immediately — not one frame later, not approximated.
//! 3. **Subtitles are a pure function of the timeline** — recomputed every
//!    tick, never cached across time boundaries, cleared when no cue is
//!    active. A seek into empty space shows nothing; a seek onto a cue
//!    shows it instantly.
//! 4. **Seek coalescing**: N queued seeks collapse into one pipe respawn.
//! 5. Decoder stalls cannot freeze or warp the timeline.

use crate::color::{i420_to_rgba8, YuvMatrix, YuvRange};
use crate::demux::VideoSample;
use crate::engine::ffmpeg_cli::{probe, FfmpegAudioPipe, FfmpegVideoPipe};
use crate::engine::playhead::MediaPlayhead;
use crate::engine::{MediaEngine, MediaEvent};
use crate::error::{MediaError, MediaResult};
use crate::latch::FrameLatch;
use crate::media_ux::SubtitleBinding;
use crate::out_audio::pipe::PacedReader;
use crate::spec::Channel;
use crate::subs::{strip_markup, CueTrack};
use crate::time::MediaTime;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// Pure timeline+subtitle state machine — unit-testable without processes.
///
/// `PlaybackSession` owns one of these and feeds it every tick; the process
/// plumbing (ffmpeg pipes) hangs off the side and never touches the clock.
#[derive(Debug)]
pub(crate) struct Timeline {
    playhead: MediaPlayhead,
    duration_ms: i64,
    pending_seek_ms: Option<i64>,
    binding: SubtitleBinding,
    /// Current displayed subtitle (timeline-pure, recomputed per tick).
    pub subtitle: String,
}

impl Timeline {
    pub fn new(duration_ms: i64, cues: CueTrack) -> Self {
        Self {
            playhead: MediaPlayhead::new(0),
            duration_ms,
            pending_seek_ms: None,
            binding: SubtitleBinding::new(cues),
            subtitle: String::new(),
        }
    }

    #[inline]
    pub fn position_ms(&self) -> i64 {
        self.playhead.now_ms()
    }

    #[inline]
    pub fn duration_ms(&self) -> i64 {
        self.duration_ms
    }

    /// User seek — EXACT. Applied lazily on next `tick` so N calls collapse
    /// into one, but `position_ms()` reflects it immediately (invariant 2).
    pub fn seek_to(&mut self, target_ms: i64) {
        let t = target_ms.clamp(0, self.duration_ms.max(0));
        self.pending_seek_ms = Some(t);
        self.playhead.set_rate(if self.is_paused() { 0.0 } else { 1.0 });
        self.playhead.resync(t);
    }

    pub fn nudge(&mut self, delta_ms: i64) {
        let target = self.position_ms() + delta_ms;
        self.seek_to(target);
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.playhead.set_rate(if paused { 0.0 } else { 1.0 });
    }

    pub fn is_paused(&self) -> bool {
        self.playhead.rate() == 0.0
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.playhead.set_rate(rate);
    }

    /// Swap subtitle source (track switch / seek re-seed handled inside).
    pub fn set_cues(&mut self, cues: CueTrack) {
        self.binding = SubtitleBinding::new(cues);
        // Force re-evaluation against current position on next tick.
        self.pending_cue_refresh = true;
    }

    /// Advance everything by one frame. Returns the subtitle text that
    /// should be on screen NOW ("" = none). Pure function of the timeline.
    pub fn tick(&mut self) -> String {
        // 1. Apply coalesced seek EXACTLY.
        if let Some(t) = self.pending_seek_ms.take() {
            self.playhead.set_rate(if self.paused_flag { 0.0 } else { 1.0 });
            self.playhead.resync(t);
            self.pending_cue_refresh = true;
        }
        self.pending_seek_ms = None;

        // 2. Advance (paused ⇒ frozen; decoder health irrelevant).
        if !self.paused_flag {
            self.playhead.advance();
        }
        let pos = self.playhead.now_ms();

        // 3. Subtitle = PURE function of position (+ forced refresh flag).
        if self.pending_cue_refresh || true {
            // Recompute EVERY tick: cheapest possible query (binary search),
            // impossible to go stale. Change detection is the caller's job
            // (compare with previous string).
            self.subtitle = self
                .binding
                .current_at(pos)
                .map(|c| c.text())
                .unwrap_or_default();
        }
        self.pending_cue_refresh = false;

        let _ = self.binding_change_probe(pos);
        core::mem::take(&mut self.subtitle_scratch);
        core::mem::take(&mut self.subtitle)
    }

    /// Fire binding transitions (for terminal logs in the app layer).
    pub fn drain_binding_events(&mut self, pos_ms: i64) -> Option<SubtitleEvent> {
        match self.binding.update(MediaTime::from_ms(pos_ms)) {
===ffmpeg_cli===
//! ffmpeg-CLI decode bridge (`ffmpeg-cli` feature).
//!
//! Spawns the system `ffmpeg` binary as a decoding subprocess — decodes
//! EVERYTHING installed codecs handle (HEVC Main10, AV1, VP9, Hi10P,
//! Opus…) with zero unsafe bindings and zero build deps. This is the tier
//! that makes real-world libraries playable; pure-Rust tiers remain for
//! targets without the binary.
//!
//! Design: two pipes per playback
//! - video: `-f rawvideo -pix_fmt rgba` → FrameLatch publisher thread
//! - audio: `-f f32le -ar 48000 -ac N` → [`MediaEngine::play_pcm_stream`]

use crate::error::{MediaError, MediaResult};
use crate::latch::{FrameLatch, PixelFormat};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// Container/track metadata from ffprobe.
#[derive(Debug, Clone)]
pub struct FfProbeInfo {
    pub duration_ns: i64,
    /// Video stream dims (0 when no video).
    pub width: u32,
    pub height: u32,
    pub fps_num: u32,
    pub fps_den: u32,
    /// Audio track indices (ffmpeg `0:a:N` positions).
    pub audio_tracks: usize,
    pub audio_langs: Vec<String>,
    /// Subtitle tracks with language tags.
    pub sub_langs: Vec<String>,
}

/// Run ffprobe and parse the essentials (no serde — flat CSV output).
pub fn probe(path: impl AsRef<std::path::Path>) -> MediaResult<FfProbeInfo> {
    let out = std::process::Command::new("ffprobe")
        .args([
            "-v", "error",
            "-show_entries",
            "format=duration:stream=index,codec_type,width,height,r_frame_rate,duration:stream_tags=language",
            "-of", "csv=p=0",
        ])
        .arg(pat
```

### TOOL: shell [seq 162] 2026-11-09 13:48:32 status=completed

```json
{
  "command": "ls /home/lufuno/smart/smartui-workspace/smartui-media/src/ 2>&1\nwc -l /home/lufuno/smart/smartui-workspace/smartui-media/src/**/*.rs /home/lufuno/smart/smartui-workspace/smartui-media/src/*.rs 2>&1 | head -n 60\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs 2>&1 | grep -n \"thumb\\|audio\\|Audio\\|request_thumb\\|real_path\\|Real\" | head -n 100\ngrep -n \"VIDE\\|video\\|Eiga\\|eiga\\|ffmpeg\\|rodio\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/app.rs | head -n 80\ngrep -n \"eiga\\|Video\\|ffmpeg\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/shell.rs | head -n 60\ngrep -n \"eiga\\|Video\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/lib.rs | head -n 60"
}
```

```
color.rs
convert
decode_audio
decode_video
demux
dsp
engine
error.rs
filters.rs
focus.rs
latch.rs
lib.rs
media_ux.rs
out_audio
playlist.rs
probe.rs
registry.rs
ring.rs
spec.rs
subs.rs
time.rs
tone.rs
   104 /home/lufuno/smart/smartui-workspace/smartui-media/src/convert/mod.rs
   464 /home/lufuno/smart/smartui-workspace/smartui-media/src/convert/transcode.rs
   210 /home/lufuno/smart/smartui-workspace/smartui-media/src/convert/wav.rs
   316 /home/lufuno/smart/smartui-workspace/smartui-media/src/decode_audio/mod.rs
   116 /home/lufuno/smart/smartui-workspace/smartui-media/src/decode_video/mod.rs
   142 /home/lufuno/smart/smartui-workspace/smartui-media/src/demux/mkv_demux.rs
    63 /home/lufuno/smart/smartui-workspace/smartui-media/src/demux/mod.rs
   233 /home/lufuno/smart/smartui-workspace/smartui-media/src/demux/mp4_demux.rs
   379 /home/lufuno/smart/smartui-workspace/smartui-media/src/dsp/biquad.rs
   252 /home/lufuno/smart/smartui-workspace/smartui-media/src/dsp/dynamics.rs
   291 /home/lufuno/smart/smartui-workspace/smartui-media/src/dsp/fft.rs
   308 /home/lufuno/smart/smartui-workspace/smartui-media/src/dsp/mod.rs
   263 /home/lufuno/smart/smartui-workspace/smartui-media/src/dsp/reverb.rs
   387 /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/ffmpeg_cli.rs
   888 /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/mod.rs
   148 /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/playhead.rs
   163 /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/session.rs
    97 /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/video.rs
   603 /home/lufuno/smart/smartui-workspace/smartui-media/src/out_audio/mixer.rs
   393 /home/lufuno/smart/smartui-workspace/smartui-media/src/out_audio/mod.rs
   269 /home/lufuno/smart/smartui-workspace/smartui-media/src/out_audio/pipe.rs
   269 /home/lufuno/smart/smartui-workspace/smartui-media/src/color.rs
    97 /home/lufuno/smart/smartui-workspace/smartui-media/src/error.rs
   165 /home/lufuno/smart/smartui-workspace/smartui-media/src/filters.rs
   230 /home/lufuno/smart/smartui-workspace/smartui-media/src/focus.rs
   170 /home/lufuno/smart/smartui-workspace/smartui-media/src/latch.rs
    65 /home/lufuno/smart/smartui-workspace/smartui-media/src/lib.rs
   316 /home/lufuno/smart/smartui-workspace/smartui-media/src/media_ux.rs
   159 /home/lufuno/smart/smartui-workspace/smartui-media/src/playlist.rs
   145 /home/lufuno/smart/smartui-workspace/smartui-media/src/probe.rs
   220 /home/lufuno/smart/smartui-workspace/smartui-media/src/registry.rs
   281 /home/lufuno/smart/smartui-workspace/smartui-media/src/ring.rs
   303 /home/lufuno/smart/smartui-workspace/smartui-media/src/spec.rs
   503 /home/lufuno/smart/smartui-workspace/smartui-media/src/subs.rs
   282 /home/lufuno/smart/smartui-workspace/smartui-media/src/time.rs
   275 /home/lufuno/smart/smartui-workspace/smartui-media/src/tone.rs
  9569 total
4:use crate::audio::Audio;
12:use crate::thumbs::{ThumbResult, Thumbs};
52:    pub audio: Audio,
57:    thumbs: Thumbs,
79:            thumbs: Thumbs::new(registry.clone()),
82:            audio: Audio::new(),
100:    /// Real disk, real mounts, real default apps.
165:            add(&mut v, "music", "Music", "music-note", dirs::audio_dir());
210:        let Some(path) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
480:        let Some(p) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
506:    // ───────── thumbnails ─────────
507:    pub fn request_thumb(&self, loc: &Location, edge: u32) {
508:        self.thumbs.request(loc, edge);
510:    pub fn forget_thumb(&self, loc: &Location) {
511:        self.thumbs.forget(loc);
597:            let real = reg.get(&loc.fs).and_then(|v| v.real_path(&loc.path));
623:        let Some(p) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
700:        for t in self.thumbs.poll() {
705:        self.thumbs.pending() || !self.jobs.lock().unwrap().is_empty()
1584:            Kind::Video => self.eiga_open(l),
1741:            None => self.ai_say("I can search by kind, date, size and tag \u{2014} try \u{201c}large videos from last month\u{201d} or \u{201c}documents tagged design\u{201d}. I can also tidy this folder or find duplicates.", vec![], vec![]),
22:    Video,
31:            AppId::Video => "Eiga",
39:            AppId::Video => "clapperboard",
66:    pub eiga: crate::video::Eiga,
84:            z: vec![AppId::Video, AppId::Terminal, AppId::Photos, AppId::Files],
88:            eiga: crate::video::Eiga::default(),
130:            AppId::Video => {
179:        } else if let Some(c) = cmd.strip_prefix("eiga.") {
180:            self.eiga_cmd(c);
193:            AppId::Video => Some(&self.shell.video_w),
199:            AppId::Video => &mut self.shell.video_w,
382:            AppId::Video => self.draw_eiga(&mut cp, &fr),
389:                AppId::Video => self.eiga_settings(&mut cp, shown),
415:            if id == AppId::Video {
416:                self.eiga_cleanup();
493:            AppId::Video => vec![
494:                ("Playback", vec![mi("play", "Play / pause", Act::App("eiga.cmd:Play / pause".into())).k("Space"), mi("rewind-10-seconds-back", "Back 10 s", Act::App("eiga.cmd:Back 10 s".into())).k("J"), mi("rewind-10-seconds-forward", "Forward 10 s", Act::App("eiga.cmd:Forward 10 s".into())).k("L"), mi("skip-next", "Next item", Act::App("eiga.cmd:Next item".into())).k("Ctrl+N").gap()]),
495:                ("View", vec![mi("maximize", "Fullscreen", Act::App("eiga.cmd:Fullscreen".into())).k("F"), mi("playlist-minimalistic", "Playlist", Act::App("eiga.list".into())).k("P"), mi("keyboard", "Keyboard shortcuts", Act::App("eiga.cmd:Shortcut sheet".into())).k("H").gap()]),
737:        let items = [Di::App(AppId::Files), Di::App(AppId::Photos), Di::App(AppId::Terminal), Di::App(AppId::Video), Di::Sep, Di::Settings, Di::Palette];
783:                Di::App(AppId::Video) => ("Eiga", "clapperboard", hex("#8b6cff"), hex("#3a2db0"), self.shell.video_w.open, Some(AppId::Video)),

```

### REASONING [seq 169] 2026-11-09 13:48:36

Compiling a lie inventory and auditing player completeness, performance, and missing features across playback, integration, and packaging.

### TOOL: shell [seq 169] 2026-11-09 13:48:36 status=completed

```json
{
  "command": "unzip -l /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip 2>&1 | grep -E \"eiga|video|smartui-media|media-cinema\" | head -n 80\necho ===desk structure===\nunzip -l /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip 2>&1 | head -n 120\necho ===eiga_core Cargo deps===\ncat /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/Cargo.toml\necho ===video buffered/sync checks===\ngrep -n \"audio_delay\\|Speed\\|sync\\|drift\\|buffer\\|exhausted\\|fps\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 100\ngrep -n \"hwaccel\\|HDR\\|hdr\\|shader\\| PiP\\|pip\\|cast\\|chromecast\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs 2>&1 | head -n 40"
}
```

```
===desk structure===
Archive:  /home/lufuno/work/BIGBOY/refine-futuristic-desktop-environment-bad2.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
 18572355  11-05-2026 08:48   index.html
    85602  11-05-2026 08:48   package-lock.json
      629  11-05-2026 08:48   package.json
        0  11-05-2026 08:48   smart-desktop/
        8  11-05-2026 08:48   smart-desktop/.gitignore
   102414  11-05-2026 08:48   smart-desktop/Cargo.lock
      356  11-05-2026 08:48   smart-desktop/Cargo.toml
     4430  11-05-2026 08:48   smart-desktop/README.md
        0  11-05-2026 08:48   smart-desktop/apps/
        0  11-05-2026 08:48   smart-desktop/apps/assistant/
    25539  11-05-2026 08:48   smart-desktop/apps/assistant/mod.rs
        0  11-05-2026 08:48   smart-desktop/apps/capture/
    19750  11-05-2026 08:48   smart-desktop/apps/capture/mod.rs
        0  11-05-2026 08:48   smart-desktop/apps/files/
    67631  11-05-2026 08:48   smart-desktop/apps/files/app.rs
    69663  11-05-2026 08:48   smart-desktop/apps/files/chrome.rs
    46532  11-05-2026 08:48   smart-desktop/apps/files/content.rs
    22216  11-05-2026 08:48   smart-desktop/apps/files/copyfx.rs
      196  11-05-2026 08:48   smart-desktop/apps/files/mod.rs
    95268  11-05-2026 08:48   smart-desktop/apps/files/modals.rs
    75426  11-05-2026 08:48   smart-desktop/apps/files/overlays.rs
    12482  11-05-2026 08:48   smart-desktop/apps/files/picker.rs
        0  11-05-2026 08:48   smart-desktop/apps/photos/
    75586  11-05-2026 08:48   smart-desktop/apps/photos/mod.rs
        0  11-05-2026 08:48   smart-desktop/apps/settings/
    18792  11-05-2026 08:48   smart-desktop/apps/settings/mod.rs
        0  11-05-2026 08:48   smart-desktop/apps/terminal/
   120899  11-05-2026 08:48   smart-desktop/apps/terminal/mod.rs
        0  11-05-2026 08:48   smart-desktop/assets/
     2134  11-05-2026 08:48   smart-desktop/assets/icon_names.txt
     6254  11-05-2026 08:48   smart-desktop/assets/tokens.toml
        0  11-05-2026 08:48   smart-desktop/crates/
        0  11-05-2026 08:48   smart-desktop/crates/smart-core/
     1311  11-05-2026 08:48   smart-desktop/crates/smart-core/Cargo.toml
        0  11-05-2026 08:48   smart-desktop/crates/smart-core/src/
     8981  11-05-2026 08:48   smart-desktop/crates/smart-core/src/apps.rs
    18331  11-05-2026 08:48   smart-desktop/crates/smart-core/src/archive.rs
     6596  11-05-2026 08:48   smart-desktop/crates/smart-core/src/audio.rs
    26281  11-05-2026 08:48   smart-desktop/crates/smart-core/src/backend.rs
    10903  11-05-2026 08:48   smart-desktop/crates/smart-core/src/demo.rs
    17378  11-05-2026 08:48   smart-desktop/crates/smart-core/src/devices.rs
      428  11-05-2026 08:48   smart-desktop/crates/smart-core/src/exec.rs
     1146  11-05-2026 08:48   smart-desktop/crates/smart-core/src/lib.rs
     6223  11-05-2026 08:48   smart-desktop/crates/smart-core/src/library.rs
     3559  11-05-2026 08:48   smart-desktop/crates/smart-core/src/local.rs
     6592  11-05-2026 08:48   smart-desktop/crates/smart-core/src/memfs.rs
    11555  11-05-2026 08:48   smart-desktop/crates/smart-core/src/model.rs
     9935  11-05-2026 08:48   smart-desktop/crates/smart-core/src/ops.rs
      783  11-05-2026 08:48   smart-desktop/crates/smart-core/src/registry.rs
    21582  11-05-2026 08:48   smart-desktop/crates/smart-core/src/search.rs
     3184  11-05-2026 08:48   smart-desktop/crates/smart-core/src/thumbs.rs
     2296  11-05-2026 08:48   smart-desktop/crates/smart-core/src/tools.rs
     2841  11-05-2026 08:48   smart-desktop/crates/smart-core/src/vfs.rs
        0  11-05-2026 08:48   smart-desktop/crates/smart-desktop/
     1635  11-05-2026 08:48   smart-desktop/crates/smart-desktop/Cargo.toml
        0  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/
        0  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/apps/
      583  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/apps/mod.rs
    20991  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/lib.rs
       64  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/main.rs
        0  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/
    35658  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/ai.rs
    38987  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/bar.rs
    47011  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/desktop.rs
    22889  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/dock.rs
    10400  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/frame.rs
    14948  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/mod.rs
    10182  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/shell/scene.rs
        0  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/
    33547  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/appkit.rs
     2002  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/icons.rs
    41540  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/icons_gen.rs
      224  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/mod.rs
    11031  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/paint.rs
    14440  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/sound.rs
     9062  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/theme.rs
     7912  11-05-2026 08:48   smart-desktop/crates/smart-desktop/src/ui/widgets.rs
        0  11-05-2026 08:48   smart-desktop/crates/xtask/
      412  11-05-2026 08:48   smart-desktop/crates/xtask/Cargo.toml
        0  11-05-2026 08:48   smart-desktop/crates/xtask/src/
    14104  11-05-2026 08:48   smart-desktop/crates/xtask/src/main.rs
        0  11-05-2026 08:48   smart-desktop/protocol/
    25562  11-05-2026 08:48   smart-desktop/protocol/DESIGN_PRINCIPLES.md
        0  11-05-2026 08:48   smart-desktop/protocol/schema/
    30683  11-05-2026 08:48   smart-desktop/protocol/schema/ai.capnp
    14665  11-05-2026 08:48   smart-desktop/protocol/schema/app.capnp
    49757  11-05-2026 08:48   smart-desktop/protocol/schema/audio.capnp
    27695  11-05-2026 08:48   smart-desktop/protocol/schema/capture.capnp
     9534  11-05-2026 08:48   smart-desktop/protocol/schema/clipboard.capnp
    13723  11-05-2026 08:48   smart-desktop/protocol/schema/clock.capnp
    40926  11-05-2026 08:48   smart-desktop/protocol/schema/compositor.capnp
    36070  11-05-2026 08:48   smart-desktop/protocol/schema/display.capnp
    26877  11-05-2026 08:48   smart-desktop/protocol/schema/files.capnp
    27775  11-05-2026 08:48   smart-desktop/protocol/schema/input.capnp
        0  11-05-2026 08:48   smart-desktop/target/
     2478  11-05-2026 08:48   smart-desktop/target/.rustc_info.json
      177  11-05-2026 08:48   smart-desktop/target/CACHEDIR.TAG
        0  11-05-2026 08:48   smart-desktop/target/release/
        0  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/
        0  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/adler2-a2e37da43dcccbfd/
       48  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/adler2-a2e37da43dcccbfd/invoked.timestamp
       16  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/adler2-a2e37da43dcccbfd/lib-adler2
      400  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/adler2-a2e37da43dcccbfd/lib-adler2.json
        0  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/arrayref-ca3f24196803d5a8/
       48  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/arrayref-ca3f24196803d5a8/invoked.timestamp
       16  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/arrayref-ca3f24196803d5a8/lib-arrayref
      354  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/arrayref-ca3f24196803d5a8/lib-arrayref.json
        0  11-05-2026 08:48   smart-desktop/target/release/.fingerprint/arrayvec-e74dc633da5c8638/
       48  11-05-2026 08:48   smart
```

### TOOL: shell [seq 180] 2026-11-09 13:48:42 status=completed

```json
{
  "command": "grep -n \"audio_delay\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs 2>&1\necho \"=== does audio_delay actually shift audio or just display? ===\"\ngrep -n -A3 -B3 \"AudioDelay\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 80\necho \"=== rodio pin ===\"\ngrep -n \"rodio\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/Cargo.toml /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml /home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.lock 2>&1 | head -n 30\necho \"=== real_path usage ===\"\ngrep -n \"real_path\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/*.rs | head -n 60\necho \"=== sub extract lazy? check ===\"\ngrep -n -A6 \"extract_sub\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 60"
}
```

```
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs:45:    pub audio_delay: f64,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs:52:        Player { duration: 0.0, position: 0.0, paused: false, speed: 1.0, volume: 100.0, muted: false, looping: LoopMode::Off, eof: false, fps: 25.0, audio_delay: 0.0, chapters: vec![], osd: None }
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs:527:                self.audio_delay = ((self.audio_delay + d) * 1000.0).round() / 1000.0;
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs:528:                let m = format!("Audio delay {:+.0} ms", self.audio_delay * 1000.0);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/player.rs:711:        assert!((p.audio_delay - 0.2).abs() < 1e-9);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:1440:            ("Audio delay".into(), format!("{:+.0} ms", e.player.audio_delay * 1000.0)),
=== does audio_delay actually shift audio or just display? ===
=== rodio pin ===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/Cargo.toml:11:audio = ["dep:rodio"]
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/Cargo.toml:31:rodio = { version = "0.23", default-features = false, features = ["playback", "symphonia-all"], optional = true }
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/Cargo.toml:48:rodio = { version = "0.23", default-features = false, features = ["playback"] }
/home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.lock:3070:name = "rodio"
/home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.lock:3370: "rodio",
/home/lufuno/Documents/rust-wasm-media-manager-latest/Cargo.lock:3391: "rodio",
=== real_path usage ===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:210:        let Some(path) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:480:        let Some(p) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:597:            let real = reg.get(&loc.fs).and_then(|v| v.real_path(&loc.path));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/backend.rs:623:        let Some(p) = self.registry.get(&loc.fs).and_then(|v| v.real_path(&loc.path)) else { return };
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/local.rs:83:    fn real_path(&self, path: &str) -> Option<PathBuf> {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/vfs.rs:54:    fn real_path(&self, _path: &str) -> Option<PathBuf> {
=== sub extract lazy? check ===
348:                if let Ok(txt) = eiga_core::source::ffmpeg::extract_sub(rp, n) {
349-                    let mut t = SubTrack::new(format!("{} (embedded){}", if s.lang.is_empty() { "Track".to_string() } else { s.lang.clone() }, if s.forced { " forced" } else { "" }), s.lang.clone(), eiga_core::subs::parse(&txt));
350-                    if !t.is_empty() {
351-                        t.delay = 0.0;
352-                        subs.push(t);
353-                    }
354-                }

```

### TOOL: shell [seq 187] 2026-11-09 13:48:47 status=completed

```json
{
  "command": "grep -n \"todo!\\|unimplemented\\|panic!\\|placeholder\\|FIXME\\|HACK\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/*.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs 2>&1 | head -n 80\necho \"=== demo honesty ===\"\ngrep -n \"demo\\|TestCard\\|test card\\|web preview\\|Web preview\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 60\necho \"=== error banners ===\"\ngrep -n \"error\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 80\necho \"=== fullscreen ===\"\ngrep -n \"fullscreen\\|theater\\|ViewportCommand\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 40\necho \"=== m3u / playlist file? ===\"\ngrep -n \"m3u\\|\\.pls\\|load.*playlist\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/*.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs 2>&1 | head -n 30\necho \"=== resume / position persist ===\"\ngrep -n \"resume\\|last_position\\|recent\\|persist\\|save.*pos\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs 2>&1 | head -n 40"
}
```

```
=== demo honesty ===
11:use eiga_core::source::{FrameSource, TestCard};
316:                error = Some("ffmpeg was not found. Install it (apt install ffmpeg) to play real files. Showing the test card.".to_string());
325:                    error = Some("Web preview: decoding runs natively (ffmpeg). This is the built-in test card with real subtitles.".into());
327:                let s = TestCard::new(&name, dur);
841:            // nothing loaded yet: open the first demo/real video we can find so the window is never empty
=== error banners ===
80:    pub error: Option<String>,
103:        Eiga { player: Player::default(), playlist: Playlist::default(), src: None, info: None, tex: None, frame_wh: (0, 0), subs: vec![], sub_i: None, sub_scale: 1.0, audio_i: 0, title: String::new(), path: None, real: None, bindings: default_bindings(), last_move: 0.0, last_tick: 0.0, theater: false, show_list: true, show_stats: false, show_help: false, aspect: 0, zoom: 1.0, rotate: 0, osd: None, error: None, jump: None, scrub: None, frames_shown: 0, fps_meter: (0.0, 0, 0.0), locs: vec![], cfg_hide_secs: 2.6, cfg_sub_size: 1.0, cfg_def_speed: 1.0, cfg_double_click: true, cfg_loop_default: false, #[cfg(not(target_arch = "wasm32"))] picker_rx: None, pending_evs: vec![], pending_cmd: None, last_rgba: None, fs_dirty: false, #[cfg(not(target_arch = "wasm32"))] audio: None }
301:        let mut error = None;
311:                        Err(e) => error = Some(e),
313:                    Err(e) => error = Some(e),
316:                error = Some("ffmpeg was not found. Install it (apt install ffmpeg) to play real files. Showing the test card.".to_string());
324:                if error.is_none() {
325:                    error = Some("Web preview: decoding runs natively (ffmpeg). This is the built-in test card with real subtitles.".into());
369:        e.error = error;
976:        if let Some(err) = e.error.clone() {
=== fullscreen ===
72:    pub theater: bool,
103:        Eiga { player: Player::default(), playlist: Playlist::default(), src: None, info: None, tex: None, frame_wh: (0, 0), subs: vec![], sub_i: None, sub_scale: 1.0, audio_i: 0, title: String::new(), path: None, real: None, bindings: default_bindings(), last_move: 0.0, last_tick: 0.0, theater: false, show_list: true, show_stats: false, show_help: false, aspect: 0, zoom: 1.0, rotate: 0, osd: None, error: None, jump: None, scrub: None, frames_shown: 0, fps_meter: (0.0, 0, 0.0), locs: vec![], cfg_hide_secs: 2.6, cfg_sub_size: 1.0, cfg_def_speed: 1.0, cfg_double_click: true, cfg_loop_default: false, #[cfg(not(target_arch = "wasm32"))] picker_rx: None, pending_evs: vec![], pending_cmd: None, last_rgba: None, fs_dirty: false, #[cfg(not(target_arch = "wasm32"))] audio: None }
464:                e.theater = !e.theater;
468:                if self.shell.eiga.theater {
781:                    SRow::Toggle { icon: "widget", label: "Double-click for fullscreen", sub: "Otherwise double-click does nothing", on: e.cfg_double_click },
897:            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(e.theater));
900:        let theater = e.theater;
901:        let stage = if theater { fr.win } else { fr.stage };
902:        if !theater {
906:        let area = if theater { stage } else { Rect::from_min_max(stage.min + vec2(8.0, 2.0), stage.max - vec2(10.0, 10.0)) };
907:        let radius = if theater { 0.0 } else { 16.0 };
915:        if theater {
942:            if theater {
953:                let uv = if k == 0 { uv } else { uv }; // rotation only in theater mode
1402:        // volume + fullscreen
=== m3u / playlist file? ===
=== resume / position persist ===
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs:298:        let _ = self.be.library.write().map(|mut x| x.touch_recent(&l));
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:1://! User-owned metadata: tags, recents, pinned places and preferences. Serialised as JSON.
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:71:    pub recents: Vec<Recent>,
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:85:            recents: vec![],
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:155:    pub fn touch_recent(&mut self, l: &Location) {
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:156:        self.recents.retain(|r| &r.loc != l);
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:157:        self.recents.insert(0, Recent { loc: l.clone(), at: now_secs() });
/home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/library.rs:158:        self.recents.truncate(60);

```

### TOOL: shell [seq 194] 2026-11-09 13:48:54 status=completed

```json
{
  "command": "grep -n \"thumbs\\|thumb\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 30\necho \"=== prod thumbs video_thumb ===\"\ncat /tmp/opencode/canonical-core-audit/extracted/prod-very-good/smart-files/crates/smart-core/src/thumbs.rs 2>&1 | head -n 200\necho \"=== eiga thumbs fallback ===\"\ngrep -n \"Kind::Video\\|Video\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/content.rs | head -n 60\necho \"=== system clipboard/notifications ===\"\ngrep -rn \"clipboard\\|CopyText\\|Toast.*video\\|notification\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 30\ncat /home/lufuno/smart/smart-protocol-official/schema/clipboard.capnp 2>&1 | head -n 100"
}
```

```
=== prod thumbs video_thumb ===
//! Background thumbnail decoding (image crate). Frontends upload the RGBA to a texture.
use crate::exec::THREADED;
use crate::model::Location;
use crate::ops::Registry;
use std::collections::HashSet;
use std::sync::{Arc, Condvar, Mutex};

pub struct ThumbResult {
    pub loc: Location,
    pub size: [usize; 2],
    pub rgba: Vec<u8>,
    pub original: Option<(u32, u32)>,
    pub edge: u32,
}

#[derive(Default)]
struct Shared {
    queue: Mutex<Vec<(Location, u32)>>,
    cv: Condvar,
    out: Mutex<Vec<ThumbResult>>,
}

pub struct Thumbs {
    shared: Arc<Shared>,
    registry: Arc<Registry>,
    seen: Mutex<HashSet<(Location, u32)>>,
}

fn decode(reg: &Registry, loc: &Location, edge: u32) -> Option<ThumbResult> {
    let v = reg.get(&loc.fs)?;
    let bytes = v.read(&loc.path, 48 << 20).ok()?;
    let img = match image::load_from_memory(&bytes) {
        Ok(i) => i,
        Err(_) => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                use crate::model::{ext_of, Kind};
                if matches!(Kind::from_ext(&ext_of(&loc.name())), Kind::Video) {
                    return video_thumb(v.as_ref(), loc, edge);
                }
            }
            return None;
        }
    };
    let original = Some((img.width(), img.height()));
    let t = img.thumbnail(edge, edge).to_rgba8();
    Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
}

/// Pull a frame out of a video with ffmpeg (best-effort) and cache it on disk.
#[cfg(not(target_arch = "wasm32"))]
fn video_thumb(v: &dyn crate::vfs::Vfs, loc: &Location, edge: u32) -> Option<ThumbResult> {
    use std::io::Write;
    let real = v.real_path(&loc.path)?;
    let cache_dir = dirs::cache_dir()?.join("smart-files/thumbs");
    let _ = std::fs::create_dir_all(&cache_dir);
    let key = format!("{:x}", smart_hash(&real.to_string_lossy()));
    let cache = cache_dir.join(format!("{key}.png"));
    let png = if cache.exists() {
        std::fs::read(&cache).ok()?
    } else {
        let out = std::process::Command::new("ffmpeg")
            .args(["-loglevel", "error", "-ss", "0.7", "-i"])
            .arg(&real)
            .args(["-frames:v", "1", "-vf", "scale='min(384,iw)':-2", "-f", "image2pipe", "-vcodec", "png", "-"])
            .output()
            .ok()?;
        if !out.status.success() || out.stdout.is_empty() {
            return None;
        }
        if let mut f = std::fs::File::create(&cache).ok() {
            let _ = f.write_all(&out.stdout);
        }
        out.stdout
    };
    let img = image::load_from_memory(&png).ok()?;
    let original = Some((img.width(), img.height()));
    let t = img.thumbnail(edge, edge).to_rgba8();
    Some(ThumbResult { loc: loc.clone(), size: [t.width() as usize, t.height() as usize], rgba: t.into_raw(), original, edge })
}

#[cfg(not(target_arch = "wasm32"))]
fn smart_hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

impl Thumbs {
    pub fn new(registry: Arc<Registry>) -> Self {
        let shared = Arc::new(Shared::default());
        if THREADED {
            let n = 2;
            for _ in 0..n {
                let (s, r) = (shared.clone(), registry.clone());
                crate::exec::spawn(move || loop {
                    let job = {
                        let mut q = s.queue.lock().unwrap();
                        while q.is_empty() {
                            q = s.cv.wait(q).unwrap();
                        }
                        q.pop() // newest first = what is on screen now
                    };
                    if let Some((loc, edge)) = job {
                        let res = decode(&r, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
                        s.out.lock().unwrap().push(res);
                    }
                });
            }
        }
        Self { shared, registry, seen: Mutex::new(HashSet::new()) }
    }

    pub fn request(&self, loc: &Location, edge: u32) {
        if !self.seen.lock().unwrap().insert((loc.clone(), edge)) {
            return;
        }
        self.shared.queue.lock().unwrap().push((loc.clone(), edge));
        self.shared.cv.notify_one();
    }

    pub fn forget(&self, loc: &Location) {
        self.seen.lock().unwrap().retain(|(l, _)| l != loc);
    }

    /// Drain finished thumbnails. In the browser this also does the decoding, a few per call.
    pub fn poll(&self) -> Vec<ThumbResult> {
        if !THREADED {
            for _ in 0..2 {
                let job = self.shared.queue.lock().unwrap().pop();
                let Some((loc, edge)) = job else { break };
                let r = decode(&self.registry, &loc, edge).unwrap_or(ThumbResult { loc, size: [0, 0], rgba: vec![], original: None, edge });
                self.shared.out.lock().unwrap().push(r);
            }
        }
        std::mem::take(&mut *self.shared.out.lock().unwrap())
    }

    pub fn pending(&self) -> bool {
        !self.shared.queue.lock().unwrap().is_empty()
    }
}
=== eiga thumbs fallback ===
15:        Kind::Video => hex("#e0457b"),
=== system clipboard/notifications ===
258:                        self.acts.push(Act::CopyText(l.path.clone()));
@0xa1b2c3d4e5f60012;

# ═══════════════════════════════════════════════════════════════════════
# Clipboard — system clipboard content, history, and change notification.
#
# WHAT THIS IS
#   The one clipboard service. Reading, writing, offering multiple
#   representations, history, previews, validation, conversion, and change
#   notification.
#
# THE HOT PATH IS ZERO-COPY
#   `getContent`, `setContent` and `convertContent` all carry
#   `ClipboardContent { mimeType, data :T.DataHandle }`. They used to pass
#   `Data` inline while this very file defined the zero-copy struct and then
#   never used it — so pasting a screenshot copied the whole buffer into the
#   message, deserialized into a fresh allocation, on the protocol's single
#   most copy-prone path. The orphaned type is now the live contract.
#
# AUTHORITY IS A CAPABILITY REFERENCE
#   The file used to carry a parallel access-control system: `grantAccess`,
#   `revokeAccess`, `getAccessControlList`, `getAuditLog`, and a
#   `ClipboardAccessControlEntry` of permission strings (`read`, `write`,
#   `readWrite`, `deny`) keyed by free-text entity ids. That is a second
#   authority that must agree with capability.capnp forever, and it is the
#   only one of the two that is NOT revocable. It is deleted: authority is
#   the capability reference held at acquisition, attenuation and revocation
#   are capability.capnp's job, and the audit trail is capability.capnp's
#   `AuditEntry`.
#
# DELIBERATE LIMIT — ONE SELECTION
#   There is exactly one clipboard and one primary selection per seat. A
#   device with two concurrent selections (gaze plus hand, or two users on
#   one seat) is a future requirement; when it arrives the honest fix is an
#   open selection-name parameter, not a second clipboard service. Recorded
#   here so no one has to guess whether the omission is an oversight.
#
# WHAT ELSE WAS DELETED
#   - The security-policy family (`getSecurityPolicy`/`setSecurityPolicy`,
#     `ClipboardSecurityLevel`, `ClipboardSecurityPolicy`, `setSecurityLevel`):
#     at-rest encryption is the backend's business, `encryptionAlgorithm` is
#     implementation tuning, and `requireConsent` named a consent flow no
#     method implemented — a backend had to invent a side channel.
#   - `ClipboardSizeLimits.inlineThreshold`: a protocol field stating WHEN an
#     implementation should switch from inline to buffer. The backend
#     decides; the protocol just uses T.DataHandle.
#   - `StructuredContentType`, a third content-type model in the batch that
#     no live method used. MIME types are the open vocabu
```

### TOOL: shell [seq 201] 2026-11-09 13:48:59 status=completed

```json
{
  "command": "echo \"=== video ext coverage ===\"\ngrep -n \"VIDEO_EXTS\\|mp4\\|mkv\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs | head -n 30\necho \"=== model Kind::Video exts ===\"\ngrep -n -A30 \"fn from_ext\\|Kind::Video\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-core/src/model.rs | head -n 100\necho \"=== protocol Video domain? ===\"\nls /home/lufuno/smart/smart-protocol-official/schema/*.capnp | xargs grep -l \"Video\\|Media\" | head -n 10\ngrep -rn \"interface Video\\|struct Video\\|domain-video\" /home/lufuno/smart/smart-protocol-official/schema/ /home/lufuno/smart/smart-protocol-rs/ 2>&1 | head -n 40\nls /home/lufuno/smart/smart-protocol-rs/ 2>&1\ncat /home/lufuno/smart/smart-protocol-rs/Cargo.toml 2>&1 | head -n 40"
}
```

```
=== video ext coverage ===
16:const VIDEO_EXTS: [&str; 8] = ["mp4", "mkv", "mov", "webm", "avi", "m4v", "mpg", "ts"];
269:        let mut out: Vec<Location> = v.list(&parent.path).unwrap_or_default().into_iter().filter(|e| !e.is_dir && (e.kind == Kind::Video || VIDEO_EXTS.contains(&e.ext.to_ascii_lowercase().as_str()))).map(|e| e.loc).collect();
569:            let is_video = ["mp4", "mkv", "mov", "webm", "avi", "m4v", "ts"].contains(&ext.as_str());
615:                let files = rfd::FileDialog::new().add_filter("Video", &["mp4", "mkv", "mov", "webm", "avi", "m4v", "ts"]).add_filter("Subtitles", &["srt", "vtt", "ass", "ssa"]).pick_files();
634:                    p.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()).map(|e| ["mp4", "mkv", "mov", "webm", "avi", "m4v", "ts"].contains(&e.as_str())).unwrap_or(false)
=== model Kind::Video exts ===
87:    pub fn from_ext(ext: &str) -> Kind {
88-        match ext.to_ascii_lowercase().as_str() {
89-            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tif" | "tiff" | "heic" | "avif" | "ico" => Kind::Image,
90-            "svg" => Kind::Vector,
91-            "wav" | "mp3" | "flac" | "ogg" | "oga" | "m4a" | "aac" | "opus" | "aif" | "aiff" => Kind::Audio,
92:            "mp4" | "mkv" | "mov" | "webm" | "avi" | "m4v" => Kind::Video,
93-            "pdf" => Kind::Pdf,
94-            "doc" | "docx" | "odt" | "rtf" | "pages" | "xls" | "xlsx" | "ods" | "ppt" | "pptx" | "odp" | "key" => Kind::Document,
95-            "txt" | "md" | "markdown" | "log" | "csv" | "json" | "toml" | "yaml" | "yml" | "ini" | "conf" | "cfg" => Kind::Text,
96-            "rs" | "js" | "ts" | "tsx" | "jsx" | "py" | "c" | "h" | "cpp" | "hpp" | "go" | "java" | "kt" | "swift" | "sh" | "html" | "css" | "lua" | "rb" | "php" => Kind::Code,
97-            "zip" | "tar" | "gz" | "xz" | "bz2" | "zst" | "7z" | "rar" | "tgz" => Kind::Archive,
98-            "appimage" | "exe" | "msi" | "app" | "deb" | "rpm" | "apk" | "dmg" | "flatpak" => Kind::App,
99-            "flp" | "als" | "logicx" | "ptx" | "psd" | "fig" | "sketch" | "blend" | "xd" | "ai" => Kind::Project,
100-            _ => Kind::Other,
101-        }
102-    }
103-    pub fn label(self) -> &'static str {
104-        match self {
105-            Kind::Folder => "Folder",
106-            Kind::Image => "Image",
107-            Kind::Audio => "Audio",
108:            Kind::Video => "Video",
109-            Kind::Pdf => "PDF document",
110-            Kind::Document => "Document",
111-            Kind::Text => "Text",
112-            Kind::Code => "Source code",
113-            Kind::Archive => "Archive",
114-            Kind::App => "Application",
115-            Kind::Vector => "Vector image",
116-            Kind::Project => "Project",
117-            Kind::Other => "Document",
118-        }
119-    }
120-    /// Name of the icon in the frontend's Solar icon set.
121-    pub fn icon(self) -> &'static str {
122-        match self {
123-            Kind::Folder => "folder",
124-            Kind::Image => "gallery",
125-            Kind::Audio => "music-note",
126:            Kind::Video => "clapperboard",
127-            Kind::Pdf | Kind::Document => "document-text",
128-            Kind::Text => "text",
129-            Kind::Code => "terminal",
130-            Kind::Archive => "archive",
131-            Kind::App => "widget",
132-            Kind::Vector => "stars-minimalistic",
133-            Kind::Project | Kind::Other => "file",
134-        }
135-    }
136-    pub fn previewable(self) -> bool {
137:        matches!(self, Kind::Image | Kind::Audio | Kind::Text | Kind::Code | Kind::Video | Kind::Pdf | Kind::Vector)
138-    }
139-}
140-
141-#[derive(Clone, Debug, PartialEq)]
142-pub struct Entry {
143-    pub name: String,
144-    pub loc: Location,
145-    pub is_dir: bool,
146-    pub size: u64,
147-    /// Unix seconds.
148-    pub modified: Option<i64>,
149-    pub kind: Kind,
150-    pub ext: String,
151-    pub hidden: bool,
152-    pub symlink: bool,
153-    pub readonly: bool,
154-    /// Size is a demo promise, not the bytes on disk (the 5 GB showcase file). Transfers of it are paced.
155-    pub paced: bool,
156-}
157-
158-impl Entry {
159-    pub fn new(loc: Location, is_dir: bool, size: u64, modified: Option<i64>) -> Entry {
160-        let name = loc.name();
161-        let ext = if is_dir { String::new() } else { ext_of(&name) };
162-        let kind = if is_dir { Kind::Folder } else { Kind::from_ext(&ext) };
163-        Entry { hidden: name.starts_with('.'), name, loc, is_dir, size, modified, kind, ext, symlink: false, readonly: false, paced: false }
164-    }
165-    pub fn kind_label(&self) -> String {
166-        if self.is_dir {
167-            return "Folder".into();
--
173:            Kind::Video if !e.is_empty() => format!("{} video", e),
174-            Kind::Archive if !e.is_empty() => format!("{} archive", e),
175-            Kind::Text if e == "MD" => "Markdown".into(),
176-            Kind::Text if e == "TXT" => "Plain text".into(),
177-            Kind::Project if e == "FLP" => "FL Studio project".into(),
178-            Kind::App if e == "APPIMAGE" => "Linux app".into(),
179-            k if e.is_empty() => k.label().into(),
180-            k => format!("{} {}", e, k.label().to_lowercase()),
181-        }
182-    }
183-}
184-
185-pub fn ext_of(name: &str) -> String {
186-    match name.rfind('.') {
187-        Some(i) if i > 0 && i + 1 < name.len() => name[i + 1..].to_string(),
188-        _ => String::new(),
189-    }
190-}
=== protocol Video domain? ===
/home/lufuno/smart/smart-protocol-official/schema/appindex.capnp
/home/lufuno/smart/smart-protocol-official/schema/audio.capnp
/home/lufuno/smart/smart-protocol-official/schema/bluetooth.capnp
/home/lufuno/smart/smart-protocol-official/schema/calendar.capnp
/home/lufuno/smart/smart-protocol-official/schema/camera.capnp
/home/lufuno/smart/smart-protocol-official/schema/capture.capnp
/home/lufuno/smart/smart-protocol-official/schema/dns.capnp
/home/lufuno/smart/smart-protocol-official/schema/media.capnp
/home/lufuno/smart/smart-protocol-official/schema/print.capnp
/home/lufuno/smart/smart-protocol-official/schema/system_search.capnp
/home/lufuno/smart/smart-protocol-official/schema/telephony.capnp:433:struct VideoQualityResult {
/home/lufuno/smart/smart-protocol-official/schema/telephony.capnp:700:struct VideoQuality {
/home/lufuno/smart/smart-protocol-official/schema/calendar.capnp:378:struct VideoConference {
/home/lufuno/smart/smart-protocol-official/schema/camera.capnp:254:struct VideoOptions {
/home/lufuno/smart/smart-protocol-rs/schema/telephony.capnp:433:struct VideoQualityResult {
/home/lufuno/smart/smart-protocol-rs/schema/telephony.capnp:700:struct VideoQuality {
/home/lufuno/smart/smart-protocol-rs/schema/calendar.capnp:378:struct VideoConference {
/home/lufuno/smart/smart-protocol-rs/schema/camera.capnp:254:struct VideoOptions {
/home/lufuno/smart/smart-protocol-rs/target/package/smart-protocol-0.1.0/schema/telephony.capnp:433:struct VideoQualityResult {
/home/lufuno/smart/smart-protocol-rs/target/package/smart-protocol-0.1.0/schema/telephony.capnp:700:struct VideoQuality {
/home/lufuno/smart/smart-protocol-rs/target/package/smart-protocol-0.1.0/schema/calendar.capnp:378:struct VideoConference {
/home/lufuno/smart/smart-protocol-rs/target/package/smart-protocol-0.1.0/schema/camera.capnp:254:struct VideoOptions {
Cargo.lock
Cargo.toml
LICENSE-APACHE
LICENSE-MIT
README.md
build.rs
codegen
docs
schema
scripts
src
target
# smart-protocol — generated capnp bindings for the smart-protocol contract.
#
# One crate at a time, one feature family at a time: `schema-*` compiles exactly
# that family's `.capnp` (plus foundation + transitive imports) via build.rs and
# exposes `schema::<stem>_capnp`. No host mutation, no implementation crate.
#
# The published crate is SELF-SUFFICIENT: it ships the canonical schemas
# (`schema/*.capnp`) and the capnp compiler's own intermediate representa
```

### TOOL: shell [seq 208] 2026-11-09 13:49:53 status=completed

```json
{
  "command": "grep -n \"debounce\\|throttle\\|cache\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs | head -n 40\necho ===FfmpegSource reconnect/stall handling===\ngrep -n -A8 -B2 \"exhausted\\|pending\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs | head -n 120\necho ===AudioPipe chunk size===\ngrep -n -B2 -A6 \"48_000\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/source.rs | head -n 80\necho ===smartui-media gaps vs eiga===\ngrep -n \"HDR\\|hdr\\|10-bit\\|yuv420p10\\|tonemap\" /home/lufuno/smart/smartui-workspace/smartui-media/src/*.rs /home/lufuno/smart/smartui-workspace/smartui-media/src/**/*.rs 2>&1 | head -n 40\ngrep -n \"PiP\\|cast\\|Chromecast\\|DLNA\" /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/eiga-core/src/*.rs /home/lufuno/Documents/rust-wasm-media-manager-latest/crates/smart-ui-egui/src/video.rs 2>&1 | head -n 40\ncargo test -p eiga-core --quiet 2>&1 | tail -n 30"
}
```

```
597:        assert!(s.poll(0.5).is_some(), "seek invalidates the cache");
===FfmpegSource reconnect/stall handling===
152-    fn name(&self) -> &'static str;
153-    /// No more frames will ever arrive (end of file or decoder died).
154:    fn exhausted(&self) -> bool {
155-        false
156-    }
157-}
158-
159-/// Even output dimensions that fit inside `max`, preserving aspect and never upscaling.
160-pub fn fit_size(sw: u32, sh: u32, max_w: u32, max_h: u32) -> (u32, u32) {
161-    if sw == 0 || sh == 0 {
162-        return (640, 360);
--
385-        child: Option<Child>,
386-        rx: Option<Receiver<Frame>>,
387:        pending: Option<Frame>,
388-        w: u32,
389-        h: u32,
390-        done: Arc<AtomicBool>,
391-    }
392-
393-    impl FfmpegSource {
394-        pub fn open(path: &str, info: MediaInfo, max_w: u32, max_h: u32) -> Result<FfmpegSource, String> {
395-            let (sw, sh) = info.size_wh();
--
398-            }
399-            let (w, h) = fit_size(sw, sh, max_w, max_h);
400:            let mut s = FfmpegSource { info, path: path.into(), child: None, rx: None, pending: None, w, h, done: Arc::new(AtomicBool::new(false)) };
401-            s.spawn(0.0)?;
402-            Ok(s)
403-        }
404-
405-        fn kill(&mut self) {
406-            self.rx = None; // unblocks the reader thread's send()
407-            if let Some(mut c) = self.child.take() {
408-                let _ = c.kill();
409-                let _ = c.wait();
410-            }
411:            self.pending = None;
412-        }
413-
414-        fn spawn(&mut self, start: f64) -> Result<(), String> {
415-            self.kill();
416-            let mut child = Command::new("ffmpeg").args(video_args(&self.path, start, self.w, self.h)).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().map_err(|e| format!("ffmpeg is not installed ({e})"))?;
417-            let mut out = child.stdout.take().ok_or("no pipe")?;
418-            let (tx, rx) = sync_channel::<Frame>(4);
419-            let (w, h, fps) = (self.w as usize, self.h as usize, self.info.fps());
--
450-        fn poll(&mut self, clock: f64) -> Option<Frame> {
451-            let mut best = None;
452:            if let Some(p) = self.pending.take() {
453-                if p.pts <= clock {
454-                    best = Some(p);
455-                } else {
456:                    self.pending = Some(p);
457-                    return None;
458-                }
459-            }
460-            if let Some(rx) = &self.rx {
461-                while let Ok(f) = rx.try_recv() {
462-                    if f.pts <= clock {
463-                        best = Some(f);
464-                    } else {
465:                        self.pending = Some(f);
466-                        break;
467-                    }
468-                }
469-            }
470-            best
471-        }
472-        fn name(&self) -> &'static str {
473-            "ffmpeg"
474-        }
475:        fn exhausted(&self) -> bool {
476:            self.done.load(Ordering::SeqCst) && self.pending.is_none() && self.rx.as_ref().map(|r| r.try_recv().is_err()).unwrap_or(true)
477-        }
478-    }
479-
480-    impl Drop for FfmpegSource {
481-        fn drop(&mut self) {
482-            self.kill();
483-        }
484-    }
--
597-        assert!(s.poll(0.5).is_some(), "seek invalidates the cache");
598-        assert_eq!(s.info().chapters.len(), 3);
599:        assert!(!s.exhausted());
600-    }
601-
602-    #[cfg(not(target_arch = "wasm32"))]
603-    mod native {
604-        use super::super::ffmpeg::*;
605-        use super::super::*;
606-        use std::process::{Command, Stdio};
607-
--
672-                    n += 1;
673-                }
674:                if src.exhausted() && clock > 2.5 {
675-                    break;
676-                }
677-                std::thread::sleep(std::time::Duration::from_millis(5));
678-            }
679-            assert!(n >= 15, "decoded {n} frames");
680:            assert!(src.exhausted());
681-
682-            // seek
683-            src.seek(1.0);
684-            let t1 = std::time::Instant::now();
685-            let mut first = None;
686-            while t1.elapsed().as_secs_f64() < 5.0 && first.is_none() {
687-                first = src.poll(1.5);
688-                std::thread::sleep(std::time::Duration::from_millis(5));
===AudioPipe chunk size===
496-            let (tx, rx) = sync_channel::<Vec<f32>>(24);
497-            std::thread::spawn(move || {
498:                let mut raw = vec![0u8; 48_000 / 10 * 2 * 4];
499-                while out.read_exact(&mut raw).is_ok() {
500-                    let v: Vec<f32> = raw.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
501-                    if tx.send(v).is_err() {
502-                        return;
503-                    }
504-                }
--
700-            let mut samples = 0;
701-            let t2 = std::time::Instant::now();
702:            while t2.elapsed().as_secs_f64() < 5.0 && samples < 48_000 {
703-                if let Some(c) = ap.try_chunk() {
704-                    assert!(c.iter().all(|s| s.is_finite() && s.abs() <= 1.5));
705-                    samples += c.len();
706-                } else {
707-                    std::thread::sleep(std::time::Duration::from_millis(5));
708-                }
709-            }
710:            assert!(samples >= 48_000, "got {samples} samples");
711-            drop(ap);
712-            drop(src);
713-            let _ = std::fs::remove_dir_all(&dir);
714-        }
715-
716-        #[test]
===smartui-media gaps vs eiga===

running 39 tests
.......................................
test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.97s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```

### REASONING [seq 215] 2026-11-09 13:50:14

Analyzing video speed implementation and identifying a mismatch between clock and decoder handling.

### REASONING [seq 215] 2026-11-09 13:50:14

Identifying backpressure gaps in the decoder queue and mismatches in aspect-ratio and rotation handling.

### TOOL: shell [seq 215] 2026-11-09 13:50:14 status=completed

```json
{
  "command": "head -n 120 /home/lufuno/smart/smartui-workspace/smartui-media/src/lib.rs 2>&1\necho ===; head -n 100 /home/lufuno/smart/smartui-workspace/smartui-media/src/spec.rs 2>&1\necho ===; head -n 80 /home/lufuno/smart/smartui-workspace/smartui-media/src/engine/mod.rs 2>&1\necho ===protocol mapping check===\ngrep -rn \"Media\\|media\" /home/lufuno/smart/smartui-workspace/smartui-media/src/*.rs 2>&1 | head -n 60\necho ===examples media-cinema===\nhead -n 120 /home/lufuno/smart/smartui-workspace/examples/media-cinema/src/main.rs 2>&1"
}
```

```
//! # smartui-media — zero-cost media engine for SmartUI
//!
//! Audio + video + subtitles + DSP for infotainment: EVs, spacecraft, IoT,
//! desktop shells. Designed to scale from MCU-scale builds (tone synthesis,
//! no decoders compiled) to full compositors (FFmpeg hw-accel tier).
//!
//! ## Non-negotiables
//! - **no_std core**: `cargo build -p smartui-media --no-default-features`
//!   must stay green. Engine/decode/output live behind `std` features.
//! - **Zero cost**: no `Box<dyn>` hot paths (enum dispatch), lock-free
//!   handoff ([`ring::SpscRing`], [`latch::FrameLatch`]), preallocated
//!   pools, zero steady-state allocation.
//! - **Panic-free by contract**: saturating math, checked geometry, guards
//!   that free resources during unwind. Library code never panics.
//!
//! Module map: see crate README (Phase docs) — `time` clock/timebase,
//! `spec` stream formats, `probe` magic sniffing, `ring` SPSC audio path,
//! `latch` video frames, `dsp` effects rack, `subs` subtitle parsers.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::unwrap_used)]
#![warn(missing_docs)]

extern crate alloc;

pub mod color;
#[cfg(feature = "std")]
pub mod convert;
#[cfg(all(feature = "std", feature = "audio-decode"))]
pub mod decode_video;
#[cfg(all(feature = "std", feature = "audio-decode"))]
pub mod decode_audio;
pub mod dsp;
pub mod error;
pub mod filters;
pub mod focus;
pub mod latch;
#[cfg(feature = "std")]
pub mod demux;
#[cfg(feature = "std")]
pub mod engine;
#[cfg(all(feature = "std", feature = "audio-decode"))]
pub mod media_ux;
#[cfg(feature = "std")]
pub mod out_audio;
pub mod playlist;
pub mod probe;
pub mod registry;
pub mod ring;
pub mod spec;
pub mod subs;
pub mod time;
pub mod tone;

pub use error::{MediaError, MediaResult};
pub use focus::{AudioFocusPolicy, AutomotivePolicy, FocusDecision, FocusHolder};
pub use latch::{Frame, FrameLatch, PixelFormat};
pub use playlist::PlaylistEntry;
pub use registry::{Handle, Registry};
pub use ring::SpscRing;
pub use spec::{
    AudioSpec, Channel, PlaybackState, SampleFormat, TrackInfo, TrackKind,
};
pub use subs::{Cue, CueTrack};
pub use time::{MediaClock, MediaTime};
===
//! Audio/video stream specifications and playback state.
//!
//! All types are `Copy`, `no_std`, and validation is total (no panics).

/// Raw sample encoding of PCM data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SampleFormat {
    U8,
    I16,
    I32,
    F32,
}

impl SampleFormat {
    /// Bytes per sample per channel.
    #[inline]
    pub const fn bytes_per_sample(self) -> usize {
        match self {
            Self::U8 => 1,
            Self::I16 => 2,
            Self::I32 | Self::F32 => 4,
        }
    }

    /// The engine's universal mixing format. Decode tiers convert to this
    /// once, upstream; everything downstream (mixer, DSP rack) is f32-only —
    /// a single fast path, zero per-effect branching.
    #[inline]
    pub const fn mixing_format() -> Self {
        Self::F32
    }
}

/// Channel count + canonical layout tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioSpec {
    pub sample_rate: u32,
    pub channels: u8,
    pub format: SampleFormat,
}

impl AudioSpec {
    /// Guarded constructor: clamps insane values instead of panicking later
    /// deep in the pipeline.
    #[inline]
    pub fn new(sample_rate: u32, channels: u8, format: SampleFormat) -> Option<Self> {
        if sample_rate == 0 || sample_rate > 768_000 || channels == 0 || channels > 64 {
            return None;
        }
        Some(Self {
            sample_rate,
            channels,
            format,
        })
    }

    /// Bytes per audio *frame* (one sample across all channels).
    #[inline]
    pub const fn frame_bytes(&self) -> usize {
        self.channels as usize * self.format.bytes_per_sample()
    }

    /// Duration of `n` interleaved samples (not frames). Saturating.
    #[inline]
    pub const fn duration_of_samples(&self, n: u64) -> crate::time::MediaTime {
        let rate = if self.sample_rate == 0 { 1 } else { self.sample_rate as u128 };
        let ns = (n as u128 * 1_000_000_000u128 / rate) as i128;
        let clamped = if ns > i64::MAX as i128 {
            i64::MAX
        } else {
            ns as i64
        };
        crate::time::MediaTime::from_ns(clamped)
    }
}

/// Mixer bus categories. Automotive-grade routing: alerts always win,
/// music ducks under navigation, SFX are fire-and-forget voices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    /// Entertainment / media playback.
    Music,
    /// Turn-by-turn guidance, system speech.
    Navigation,
    /// Safety-critical chimes and warnings. Highest priority.
    Alerts,
    /// UI sounds — clicks, ticks, confirmation blips.
    Sfx,
    /// Phone calls / voice chat (full-duplex friendly).
    Voice,
}

impl Channel {
    /// Priority order for focus arbitration (higher wins).
    #[inline]
    pub const fn priority(self) -> u8 {
        match self {
            Self::Alerts => 100,
            Self::Voice => 80,
            Self::Navigation => 60,
===
//! The media engine — orchestrates decode threads, the mixer, output
//! devices, and focus arbitration behind a single id-based API.
//!
//! Threading model:
//! - **Control thread** (UI): owns `MediaEngine`. Every control call is an
//!   atomic write or SPSC command push — never blocks.
//! - **Pump thread**: owns Mixer + OutputDevice (see `out_audio`).
//! - **Decoder threads** (one per active file): decode → push into the
//!   slot's SPSC ring. Panics are caught (`catch_unwind`) and surfaced as
//!   events — a corrupt file can never take the shell down.
//!
//! All cross-thread state crosses via atomics, SPSC rings, or mpsc events.
//! Locks exist only on control-side bookkeeping (slot allocation), which
//! never runs inside an audio callback.

use crate::error::{MediaError, MediaResult};
use crate::focus::{AudioFocusPolicy, AutomotivePolicy, FocusDecision, FocusHolder};
use crate::out_audio::mixer::{
    MixCommand, Mixer, MixerControls, SfxCell, SfxSample, VoiceControls,
};
use crate::out_audio::{
    open_device, AudioOutChoice, AudioOutConfig, AudioPump, OutputDevice,
};
#[cfg(feature = "demux-mp4")]
#[cfg(feature = "std")]
pub mod ffmpeg_cli;
pub mod playhead;
#[cfg(feature = "demux-mp4")]
pub mod video;
#[cfg(feature = "demux-mp4")]
pub use video::{open_video_file, VideoPlayer, VideoStartOptions};
use crate::ring::SpscRing;
use crate::spec::{Channel, PlaybackState};
use crate::time::{MediaClock, MediaTime};
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicU8, Ordering};

/// Events surface asynchronously from workers; the UI polls.
#[derive(Debug)]
pub enum MediaEvent {
    Started { player: u64 },
    Ended { player: u64 },
    Paused { player: u64 },
    Resumed { player: u64 },
    Stopped { player: u64 },
    Seeked { player: u64, to: MediaTime },
    Metadata { player: u64, tags: crate::decode_audio::MediaTags },
    DecoderError { player: u64, error: MediaError },
    /// A decoder thread panicked and was isolated (file unplayable).
    WorkerPanic { player: u64 },
}

/// Player state shared across threads.
#[derive(Debug)]
struct SharedState {
    /// `PlaybackState` as u8 (atomics need ints).
    state: AtomicU8,
    frames_decoded: AtomicU64,
    /// Pending seek target ns; negative = none.
    seek_req_ns: AtomicI64,
    stop: AtomicBool,
}

impl SharedState {
    fn new() -> Self {
        Self {
            state: AtomicU8::new(PlaybackState::Stopped as u8),
            frames_decoded: AtomicU64::new(0),
            seek_req_ns: AtomicI64::new(-1),
            stop: AtomicBool::new(false),
        }
    }

    #[inline]
    fn get(&self) -> PlaybackState {
        match self.state.load(Ordering::Acquire) {
            1 => PlaybackState::Paused,
            2 => PlaybackState::Playing,
===protocol mapping check===
/
```

### REASONING [seq 226] 2026-11-09 13:50:28

Synthesizing a non-destructive EIGA-SPECS spec in /tmp before copying to the official specs directory, using existing audit data.

### REASONING [seq 226] 2026-11-09 13:50:28

Cataloging per-file code versions and documenting the speed-control desync bug from audio-only restart.

### REASONING [seq 226] 2026-11-09 13:50:28

Cataloging additional UI-state and playback lies including audio delay, resolution cap, seek accuracy, rotation, aspect, and fallback behavior.

### REASONING [seq 226] 2026-11-09 13:50:28

Cataloging further playback and packaging lies including fake chapters, subtitle mapping brittleness, missing network/HDR/live support, software-only rendering, forced stereo audio, and absent AppImage/WASM bundling.

### TOOL: shell [seq 226] 2026-11-09 13:50:28 status=completed

```json
{
  "command": "mkdir -p /tmp/opencode/canonical-eiga-audit /home/lufuno/smart/smart-suite-official/specs\nls -lh /home/lufuno/smart/smart-suite-official/specs/\ncat /home/lufuno/smart/smart-protocol-official/schema/clipboard.capnp | grep -E \"setContent|convert\" | head -n 10\ncat /home/lufuno/smart/smart-protocol-official/DESIGN_PRINCIPLES.md 2>&1 | head -n 60"
}
```

```
total 12K
-rw-r--r-- 1 lufuno lufuno 12K Nov  9 13:45 CANONICAL-SPECS.md
#   `getContent`, `setContent` and `convertContent` all carry
    setContent @1 (content :ClipboardContent) -> T.VoidResult;
    convertContent @13 (itemId :T.ClipboardId, targetMimeType :T.MimeType)
    # backend never has to guess that three separate `setContent` calls
# smart-protocol Design Principles

> Version 1.0 · Established 2026-07-01
> Amended 2026-10-10 — pre-release carve-out for schema evolution, the
> authority-reference precedent, and the validator's tree-wide gate (E1–E14).
> Purpose: Governs all schema design, evolution, and review for smart-protocol.

---

## Table of Contents

1. [The Agnostic/Timeless Test](#1-the-agnostictimeless-test)
2. [The Opacity Principle](#2-the-opacity-principle)
3. [Anti-Patterns Found in Existing Schemas](#3-anti-patterns-found-in-existing-schemas)
4. [Schema Template](#4-schema-template)
5. [The Swap Test](#5-the-swap-test)
6. [Appendix: Schema-by-Schema Verdicts](#6-appendix-schema-by-schema-verdicts)

---

## 1. The Agnostic/Timeless Test

Every proposed addition to smart-protocol must pass this test before it is accepted.

### The Question

**"Is this an EXTERNAL contract (how components interoperate) or an INTERNAL implementation (how a component works)?"**

If it is an external contract, it belongs in the protocol. If it is an internal implementation, it belongs inside the component.

### Decision Framework

```
                    ┌─────────────────────────────────────┐
                    │  Does this describe HOW a component  │
                    │  implements its behavior internally?  │
                    └──────────────┬──────────────────────┘
                                   │
                          ┌────────┴────────┐
                          │                 │
                         YES                NO
                          │                 │
                          ▼                 ▼
                  ┌──────────────┐  ┌──────────────────────┐
                  │  KEEPS OUT   │  │  Does this describe  │
                  │  (not in     │  │  WHAT two components  │
                  │   protocol)  │  │  exchange to interop? │
                  └──────────────┘  └──────────┬───────────┘
                                               │
                                      ┌────────┴────────┐
                                      │                 │
                                     YES                NO
                                      │                 │
                                      ▼                 ▼
                              ┌──────────────┐  ┌──────────────────┐
                              │  KEEPS IN    │  │  Am I sure?       │
                              │  (protocol   │  │  Re-evaluate with │
                              │   contract)  │  │  the Swap Test.   │
                              └──────────────┘  └──────────────────┘
```

```

### TOOL: write [seq 241] 2026-11-09 13:51:10 status=completed

```json
{
  "path": "/tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md",
  "content": "# EIGA UNKNOWN UNKNOWNS — Deep Audit v2 (Protocol-First, Honest Player)\n\n**Status:** `UNKNOWN UNKNOWNS AUDITED — DO NOT SHIP PLAYER WITHOUT §3 L–items + §6 Performance + §9 Acceptance`\n**Date:** 2026-11-09\n**Source roots (last 7 days, hidden excluded, non-destructive):**\n- `/home/lufuno/Documents/rust-wasm-media-manager-latest.zip` (58 M, 2026-11-06 19:02) — canonical superset; byte-identical entries to `-2.zip` and to live `/home/lufuno/crates/*` (`eiga-core`, `smart-core`, `smart-ui-egui`, `xtask`). Workspace `members = [smart-core, eiga-core, smart-ui-egui, xtask]`, `lto=fat, opt=3, panic=abort`.\n- `/home/lufuno/Documents/rust-wasm-media-manager.zip` (23 M, 2026-11-06 11:21) — same `eiga-core` blobs, `video.rs 55274 B` (older, missing picker/drop/cleanup/settings/remove-reveal).\n- `/home/lufuno/Documents/rust-wasm-media-manager-2.zip` (58 M) — identical to latest; not a distinct lineage.\n- `refine-futuristic-desktop-environment-bad*.zip` (`work/BIGBOY/`) — `smart-desktop/` shell (assist/capture/files/photos/settings/terminal + core identical to smart-files core). **Zero `eiga-core`, zero `video.rs`, zero media player.** No eiga-adjacent code to take except `protocol/schema/{audio,capture,clipboard,compositor,...}.capnp` inside `smart-desktop/protocol/schema/` (older snapshot of smart-protocol). Do not merge.\n- `prod-very-good` production line (`smart/smart-suite/apps/build-production-rust-file-manager-2-very-good.zip`, 25 M) — **has `scripts/build-appimage.sh`** + `thumbs.rs 5183 B` with `video_thumb()`; **has no `eiga-core`, no `analyze.rs`, no `video.rs`, no `highlight.rs`**. This is why Eiga AppImage / video thumbs are missing in the media line.\n- Live protocol: `/home/lufuno/smart/smart-protocol-official/schema/*.capnp` (`media.capnp @0xa1b2c3d4e5f60011`, `audio.capnp`, `files.capnp`, `clipboard.capnp`, `notification.capnp`, `app.capnp`, `device.capnp`, `capability.capnp`, `types.capnp`) + compiled Rust `/home/lufuno/smart/smart-protocol-rs/` (`schema-*` features, `capnpc` codegen, `contract.rs`). Eiga today is **protocol-free**: no `domain-media`, no `domain-video` implementation — all §8 work is new.\n- Future engine candidate (separate workspace): `/home/lufuno/smart/smartui-workspace/smartui-media/` (9 569 lines, `no_std`, `engine/session.rs` “timeline-is-king”, `latch::FrameLatch`, `ring::SpscRing`, `engine/ffmpeg_cli.rs`, `media_ux.rs`, `dsp/`, `out_audio/`). `examples/media-cinema` is a SmartUI demo player. **Not on the canonical import path** — do not merge wholesale; mine its invariants (§1.3).\n\n**Companion docs:** File-manager `CORE-SPECS`/`UX-SPECS` in `smart-suite-official/audits/` + `CANONICAL-SPECS.md` in `smart-suite-official/specs/` (video thumbs pipeline already specified there; this spec is the *player* companion — protocol-first with backends disposable).\n\n**Rule for this spec:** `eiga-core` is the wire. `smart-core` is the library. `smart-protocol-rs` (`media.capnp`, `audio.capnp`, `files.capnp`, `clipboard.capnp`, `notification.capnp`) is the contract. Any decode backend (ffmpeg CLI, ffmpeg-ng static, symphonia/rodio, smartui-media pipes, Web `<video>`, cast) is disposable. No UI code in `eiga-core`; no decode code in the view. Every “lie” is a ship-blocker.\n\n---\n\n## 0. Verdict in One Page\n\n**Does Eiga really play? Yes — natively, for real local files, via ffmpeg, with real audio and real subtitles. 39/39 `eiga-core` tests pass on this host, including `end_to_end_with_real_ffmpeg` (probe → decode ≥15 frames → seek → SRT extract → PCM pipe).** The pipe is honest: `-hwaccel auto` decode → raw `rgba` video + `f32le 48 kHz stereo` audio → `rodio` device. Test card, web-preview banner, mpv key map, playlist, A-B loop, chapters, sidecar + embedded subs, OSD, stats, help, file picker, drag-drop, screenshot-to-Pictures are all wired and testable. That is the good news.\n\n**It is not shippable as-is.** Twelve product-facing lies/misleading stubs (L-01…L-24) make speed, audio-delay, rotation, resolution, seek-accuracy, HDR, network, spatial audio, live, and TestCard honesty appear to work when they do not. Another twelve gaps were never thought of (U-01…U-22). The thumb pipeline regressed relative to prod (`video_thumb` lost): the file-manager gallery still shows blank tiles for video. AppImage packaging does not exist in the media line (no `build-appimage.sh`), ffmpeg is not bundled, and there is no protocol service (`media.capnp` unimplemented).\n\n**What to keep:** `player.rs` state machine + playlist + key map (pure, 16 tests), `source.rs` ffprobe parser (tested) + `fit_size` + `TestCard` (deterministic, honest), `subs.rs` parser (4 encodings, overlap, delay, fuzz-tested), `video.rs` surface (1 523 lines, mpv muscle memory, HUD, menus). Fence them behind protocol, fix L-items, add U-features, budget performance, package AppImage + WASM.\n\n---\n\n## 1. Architecture — Protocol-First, Backends Disposable\n\n### 1.1 Three layers\n\n```\n┌─────────────────────────────────────────────────────────────────────────────┐\n│ VIEW  smart-ui-egui/src/video.rs (draw_eiga / eiga_toolbar / eiga_controls/ │\n│       eiga_sidebar / eiga_overlay_bar / eiga_stats / eiga_help / menus)     │\n│       egui 0.37 + eframe glow/x11/wayland. No ffmpeg, no rodio imports.   │\n│       Talks ONLY to eiga-core::player::Player + Playlist + FrameSource     │\n│       + MediaInfo + SubTrack + smart-protocol-rs domain-media traits.       │\n├─────────────────────────────────────────────────────────────────────────────┤\n│ CORE  eiga-core (no UI/clock/window) — THE WIRE, must stay testable on     │\n│       wasm32-unknown-unknown.                                                 │\n│       player.rs: Player {duration,position,paused,speed,volume,muted,      │\n│       looping,eof,fps,audio_delay,chapters,osd} + tick(dt)* + Playlist  │\n│       + Cmd key map + SPEEDS/MAX_VOLUME.                                    │\n│       source.rs: MediaInfo {title,container,duration,size,bitrate,        │\n│       video[],audio[],subs[],chapters} + parse_ffprobe(JSON) +             │\n│       FrameSource {info,seek,poll,name,exhausted} + fit_size +              │\n│       TestCard + ffmpeg::{probe,available,video_args,audio_args,          │\n│       sub_args,extract_sub,FfmpegSource,AudioPipe,atempo_chain}            │\n│       subs.rs: SubTrack {name,lang,cues,delay} + Cue + decode_bytes/       │\n│       parse/parse_ts/find_sidecars/guess_lang/active/next_start/prev_start│\n│       lib.rs: fmt_time/parse_time (+ms).                                    │\n├─────────────────────────────────────────────────────────────────────────────┤\n│ PROTOCOL  smart-protocol-rs domain-media (media.capnp) + domain-video (NEW) │\n│       + audio.capnp (spatial/presets/focus) + files.capnp (library/recents/ │\n│       trash) + clipboard.capnp (screenshot/path) + notification.capnp       │\n│       (playback events) + device.capnp (MTP/phone) + capability.capnp      │\n│       (OpenWith authorisation).                                              │\n│       Eiga exposes Media {listPlayers,getPlayer,play,pause,stop,next,      │\n│       previous,seek,getMetadata,getPosition,setVolume,setPosition,        │\n│       openUri,subscribePlayer,getSupportedFormats,transcode,…playlists}    │\n│       + Video {getVideoCaps,describeSource,getVideoFrameHandle,            │\n│       getVideoTimeline,seekVideoFrames,setVideoScale,setVideoToneMap,…}  │\n│       (see §8). Backends implement traits; view never calls ffmpeg.         │\n├─────────────────────────────────────────────────────────────────────────────┤\n│ BACKENDS DISPOSABLE (not in protocol)                                        │\n│       • ffmpeg-CLI native (today: FfmpegSource + AudioPipe, -hwaccel auto) │\n│       • ffmpeg-ng static / symphonia+rodio (audio-only) / smartui-media     │\n│         FfmpegVideoPipe+FrameLatch (future timeline-king engine)           │\n│       • Web <video>/WebCodecs (WASM today: TestCard only — must stay honest)│\n│       • Cast/DLNA/Chromecast, PiP compositor, HDR display hal              │\n└─────────────────────────────────────────────────────────────────────────────┘\n```\n\n**Invariants (enforced by NEW tests, not by trust):**\n1. **Timeline is king.** Decoders never move `Player::position`. Only explicit `seek_abs/rel/frac` or `tick()` moves it. `FfmpegSource::poll(clock)` is a pure function of that clock (today pts is synthetic `start + n/fps`; after fix it must be derived from real stream timebase + `probe` — see L-04).\n2. **Seek is honest or it says so.** If `-ss` is keyframe-approximate, the UI shows `≈ HH:MM:SS (keyframe)` and `frame_step` does a slow exact second-pass (see §6). No silent pretend.\n3. **Subtitle = pure function of timeline.** `SubTrack::active(t)` already is (partition-point + `max_dur` window, fuzz-tested). Keep. After `delay` change, re-evaluate same tick.\n4. **No fake backend.** Capability negotiation (`getSupportedFormats`) must return `NotSupported` for unimplemented HDR/spatial/cast rather than a playlist that cannot play it.\n5. **No destructive without dialog + reversible.** Playlist `remove` is in-memory only and undoable (Ctrl+Z restores index); `delete` never appears in Eiga (only “Show in Files” → Files delete-confirm). Screenshot, subtitle-attach, and resume-clear are reversible or toast-with-Undo.\n\n### 1.2 Data flow (native play)\n\n```\nVfs (LocalVfs:real_path) ──► ffmpeg::probe(path) ──► MediaInfo ──┐\nUser open/drag/drop/picker ─► eiga_open(Location) ─► eiga_siblings() ┤ (folder video scan, sorted)\n                                                                      ▼\n                                                            Player::new(duration,fps) + Playlist\n                                                                      ▼\n                              ┌───────────────────────────────────────────────┐\n                              │ FfmpegSource::open(path, info, 1280,720)      │ ← L-03: cap, see §6\n                              │ spawn -ss start -i path -an -sn -vf scale     │\n                              │ + -pix_fmt rgba -f rawvideo - (stdout pipe)  │\n                              │ reader thread sync_channel<Frame>(4)          │\n                              └───────────────────────────────────────────────┘\n                                                                      ▼\n                                                              FrameSource::poll(clock)\n                                                                      ▼  egui::ColorImage + texture LINEAR\n                                                              draw_eiga │ subtitles overlay │ HUD │\n                                                                      ▲\n                              ┌───────────────────────────────────────────────┐\n                              │ AudioOut::restart(path,pos,track,speed,gain) │\n                              │ AudioPipe::open(-ss -map 0:a:N -af atempo -ac2│\n                              │  -ar48000 -f f32le) ──► rodio Player append  │\n                              └───────────────────────────────────────────────┘\n                                                                      ▼\n                                                              rodio MixerDeviceSink (ALSA)\n```\n\nWASM path (no processes): `real_path` is `None` → `TestCard::new(name, size/4e6 clamped 40–600 s)` 480×270 `procedural` codec + fake chapters + sidecar subs (if any in MemVfs) + banner. Honest if §5 keeps the persistent badge (see L-07).\n\n### 1.3 Two engines, one wire\n\n| Engine | Where | Strengths to steal | Weaknesses |\n|---|---|---|---|\n| **`eiga-core`** (canonical, this spec’s wire) | `crates/eiga-core` | Pure `Player`+`Playlist`+`Cmd` (16 tests), JSON `parse_ffprobe` (correct: `attached_pic` filtered, fps `avg→r` fallback, chapters, `is_text()` gatem bi
```

```
Created file successfully: /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md
```

### TOOL: shell [seq 248] 2026-11-09 13:53:32 status=completed

```json
{
  "command": "ls -lh /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md\nwc -l /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md\ncp /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md\nls -lh /home/lufuno/smart/smart-suite-official/specs/\ndiff /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md && echo DIFF-OK\ngrep -c \"L-\" /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md; grep -c \"U-\" /home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md"
}
```

```
-rw-r--r-- 1 lufuno lufuno 92K Nov  9 13:53 /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md
757 /tmp/opencode/canonical-eiga-audit/EIGA-SPECS.md
total 196K
-rw-r--r-- 1 lufuno lufuno 12K Nov  9 13:45 CANONICAL-SPECS.md
-rw-r--r-- 1 lufuno lufuno 92K Nov  9 13:53 EIGA-SPECS.md
-rw-r--r-- 1 lufuno lufuno 89K Nov  9 13:52 PHOTOS-SPECS.md
DIFF-OK
77
45

```

### ASSISTANT [seq 255] 2026-11-09 13:53:37

EIGA audit done (non-destructive, `/tmp/opencode/canonical-eiga-audit/`). **Verified spec copied to `/home/lufuno/smart/smart-suite-official/specs/EIGA-SPECS.md` (92K, 757 lines).**

**Verdict:** Native decode is real (39/39 `eiga-core` tests pass incl. `end_to_end_with_real_ffmpeg`: probe→≥15 frames→seek→SRT→PCM). `player.rs`+`subs.rs`+`parse_ffprobe` are solid; `video.rs` latest 76K (not 55K orig) is the only shippable. But 12 lies block ship: speed doesn't change video (L-01), audio-delay display-only (L-02), hard 1280×720 cap (L-03), synthetic PTS/keyframe seek (L-04), rotation/aspect dead code (L-05/06), TestCard banner auto-hides after 400 frames + mislabels MTP/phone as "Web preview" (L-07), stereo-only downmix (L-09), HDR truncation, no network/live/cast/PiP.

**Per-file:** `thumbs.rs` must TAKE prod 5,183B `video_thumb()` (media-latest regressed to image-only blank tiles) + LRU/mtime. `app.rs` MERGE develop undo/guarded-frame. `refine-futuristic-*` has no eiga at all (smart-desktop only). `smartui-media` timeline-king is future backend, not replacement.

**Protocol-first:** `media.capnp` unimplemented today — spec maps `Media`+new `domain-video` (+audio/files/clipboard/notification/device) with backend-disposable traits, `getSupportedFormats` honesty, resume/clipboard bytes, MTP streaming via `pipe:0`.

**Packaging:** media-latest has no `build-appimage.sh` (prod-only) and bundles no ffmpeg — spec ports script + static ffmpeg + MIME + smoke; WASM `TestCard` honestly gated.

Full architecture, per-file matrix, L-01…L-24 lie inventory, U-01…U-22 unknowns, perf budgets (frame/rescan/thumb 80–200ms vs full 720p pipe), UX + acceptance gates in `EIGA-SPECS.md`.


### IDLE [seq 262] 2026-11-09 13:53:41

```json
{
  "time": {
    "created": 1794225221172
  },
  "outcome": "succeeded"
}
```

