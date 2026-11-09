#!/usr/bin/env bash
# Build dist/Smart_Files-<version>-x86_64.AppImage from the native release binary.
#
#   tools/build_appimage.sh            # builds with the `release` profile (fat LTO) unless PROFILE=dist
#   PROFILE=dist tools/build_appimage.sh   # thinner LTO, for machines with little RAM
#
# Needs: cargo, curl. FUSE is NOT needed to *build* (appimagetool is run with --appimage-extract-and-run).
set -euo pipefail
cd "$(dirname "$0")/.."
ROOT="$PWD"
PROFILE="${PROFILE:-release}"
ARCH="${ARCH:-x86_64}"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
APPDIR="$ROOT/target/AppDir"
OUT="$ROOT/dist-appimage"

echo "==> building smart-files ($PROFILE)"
cargo build --profile "$PROFILE" -p smart-ui-egui --bin smart-files
BIN="$ROOT/target/$([ "$PROFILE" = release ] && echo release || echo "$PROFILE")/smart-files"
[ -x "$BIN" ] || { echo "binary not found at $BIN" >&2; exit 1; }

echo "==> assembling AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/lib" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/256x256/apps" "$APPDIR/usr/share/metainfo"
install -m755 "$BIN" "$APPDIR/usr/bin/smart-files"
install -m755 packaging/AppRun "$APPDIR/AppRun"
install -m644 packaging/smart-files.desktop "$APPDIR/smart-files.desktop"
install -m644 packaging/smart-files.desktop "$APPDIR/usr/share/applications/smart-files.desktop"
install -m644 assets/icon/smart-files-256.png "$APPDIR/smart-files.png"
install -m644 assets/icon/smart-files-256.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/smart-files.png"
install -m644 packaging/smart-files.metainfo.xml "$APPDIR/usr/share/metainfo/smart-files.appdata.xml"
ln -sf smart-files.png "$APPDIR/.DirIcon"

# Bundle only what a typical desktop might lack. glibc, GL/EGL, X11/Wayland/xkb are loaded from the host
# (the GUI stack dlopen()s them at run time so the host's GPU driver is always the one used).
echo "==> bundling shared libraries not guaranteed on the host"
ldd "$BIN" | awk '/=> \// {print $3}' | while read -r lib; do
  case "$(basename "$lib")" in
    libasound.so*) cp -L "$lib" "$APPDIR/usr/lib/" && echo "    + $(basename "$lib")" ;;
  esac
done

echo "==> fetching appimagetool"
TOOL="$ROOT/target/appimagetool-$ARCH.AppImage"
if [ ! -x "$TOOL" ]; then
  curl -fsSL -o "$TOOL" "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-$ARCH.AppImage"
  chmod +x "$TOOL"
fi

mkdir -p "$OUT"
FILE="$OUT/Smart_Files-$VERSION-$ARCH.AppImage"
echo "==> packing $FILE"
ARCH="$ARCH" "$TOOL" --appimage-extract-and-run --no-appstream "$APPDIR" "$FILE"
chmod +x "$FILE"
ls -lh "$FILE"
( cd "$OUT" && sha256sum "$(basename "$FILE")" | tee "$(basename "$FILE").sha256" )
