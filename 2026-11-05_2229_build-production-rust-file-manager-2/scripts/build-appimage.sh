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
