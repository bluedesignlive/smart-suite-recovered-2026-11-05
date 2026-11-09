# Smart Suite — Recovered (2026-11-05)

Recovered extracts of 20 Smart Suite build archives from about a week ago
(Nov 4–6, 2026) — original `.zip` files were **not touched or deleted**.

Each subfolder is one archive extracted on its own, named as:

```
<YYYY-MM-DD_HHMM>_<original-zip-title>/
```

The date/time in the name is the zip file's modification time, so the
timeline stays true to when each build was made.

## Layout (20 folders)

### File-manager apps — `smart-files` / `smart-core` / `smart-ui-egui`

- `2026-11-04_1200_build-cross-platform-rust-file-manager/`
- `2026-11-04_1635_build-cross-platform-rust-file-manager-good1/`
- `2026-11-04_1730_build-cross-platform-rust-file-manager-last/`
- `2026-11-04_1757_recreate-file-manager-in-rust-good/`
- `2026-11-04_1842_recreate-file-manager-in-rust-bad/`
- `2026-11-04_1935_rust-wasm-ui-refinements/`
- `2026-11-04_1941_rust-wasm-ui-refinements2/`
- `2026-11-04_2244_improve-app-consistency-and-completeness/`
- `2026-11-04_2248_recreate-file-manager-in-rust-2/`
- `2026-11-04_2301_improve-app-consistency-and-completeness-good/`
- `2026-11-04_2332_improve-app-consistency-and-completeness-last/`
- `2026-11-05_1807_build-production-rust-file-manager/`
- `2026-11-05_2106_build-production-rust-file-manager-2-very-good/`
- `2026-11-05_2129_develop-rust-wasm-file-manager/`
- `2026-11-05_2229_build-production-rust-file-manager-2/`

### Desktop Environment (DE) — `smart-desktop/apps/*`

- `2026-11-05_0140_refine-futuristic-desktop-environment-bad/`
  (`smart-desktop/apps/{assistant,capture,files,photos,settings,terminal}`)
- `2026-11-05_1048_refine-futuristic-desktop-environment-bad2/`

### Media-manager apps — `smart-core` / `eiga-core`

- `2026-11-06_1121_rust-wasm-media-manager/`
- `2026-11-06_1319_rust-wasm-media-manager-2/`
- `2026-11-06_1902_rust-wasm-media-manager-latest/`

## What was removed (junk only)

Only 100% build artifacts were skipped during extraction:

- `target/` (Rust build output, fingerprints, AppDir, etc.)
- `.cargo/` and `.rustup/` (crate registries/toolchains copied by mistake
  into `build-production-rust-file-manager-2.zip`)
- `__MACOSX/`, `__pycache__/`, `*.pyc`, `.DS_Store`, `Thumbs.db`

Everything else was kept as-is: `Cargo.toml`/`Cargo.lock`, `crates/`,
`apps/`, `assets/`, `src/`, `index.html`, `package.json`, docs, configs,
etc.

## Notes

- Source archives (in `work/BIGBOY/`, `smart/smart-suite/apps/`,
  `Documents/`) were left untouched.
- Plugin archives (`*amapiano-logdrum-plugin*.zip`) were **not** included.
- Total size after cleaning: ~259 MB, 1088 files, largest file ~19 MB.
