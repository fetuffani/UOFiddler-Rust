#!/usr/bin/env bash
# Packages a Linux build of uo_gui as a .tar.gz, a .deb and an AppImage.
#   packaging/linux/package.sh <uo_gui binary> <version, e.g. 0.1.0.6> <out dir>
# The binary only links glibc (X11, Wayland and OpenGL are loaded at run time), so it runs on any
# distro with a glibc at least as new as the one it was built against.
set -euo pipefail

bin="$1"
version="$2"
out="$3"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$out"

# Plain archive: the program and the editable lists next to it.
tarname="uofiddler-${version}-linux-x86_64"
mkdir -p "$work/$tarname"
install -m 755 "$bin" "$work/$tarname/uofiddler"
cp "$root/README.md" "$root/README_pt-br.md" "$root/Animationlist.xml" "$root/Multilist.xml" "$work/$tarname/"
tar -C "$work" -czf "$out/$tarname.tar.gz" "$tarname"

# Debian / Ubuntu package: /usr/bin, the menu entry and the icon.
deb="$work/deb"
install -Dm 755 "$bin" "$deb/usr/bin/uofiddler"
install -Dm 644 "$here/uofiddler.desktop" "$deb/usr/share/applications/uofiddler.desktop"
install -Dm 644 "$here/uofiddler.png" "$deb/usr/share/icons/hicolor/512x512/apps/uofiddler.png"
mkdir -p "$deb/DEBIAN"
cat > "$deb/DEBIAN/control" <<CONTROL
Package: uofiddler
Version: ${version}
Section: graphics
Priority: optional
Architecture: amd64
Depends: libc6 (>= 2.30), libgl1
Recommends: libx11-6, libxkbcommon0, libwayland-client0
Maintainer: UOFiddler-Rust <https://github.com/mbnunes/UOFiddler-Rust>
Homepage: https://github.com/mbnunes/UOFiddler-Rust
Description: Ultima Online client file viewer and editor
 Native viewer and editor for Ultima Online client files (art, gumps, animations,
 maps, multis, sounds and more), classic MUL and UOP, with an isometric map editor
 that saves into the client files.
CONTROL
dpkg-deb --build --root-owner-group "$deb" "$out/uofiddler_${version}_amd64.deb" >/dev/null

# SKIP_APPIMAGE=1 stops here (local rehearsals on machines where appimagetool, x86_64-only, cannot run).
[ -z "${SKIP_APPIMAGE:-}" ] || exit 0

# AppImage: one file to download and run.
app="$work/UOFiddler.AppDir"
install -Dm 755 "$bin" "$app/usr/bin/uofiddler"
cp "$here/uofiddler.desktop" "$here/uofiddler.png" "$app/"
ln -s usr/bin/uofiddler "$app/AppRun"
tool="${APPIMAGETOOL:-$work/appimagetool}"
if [ ! -x "$tool" ]; then
  curl -fsSL -o "$tool" \
    https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
  chmod +x "$tool"
fi
ARCH=x86_64 APPIMAGE_EXTRACT_AND_RUN=1 VERSION="$version" "$tool" -n "$app" \
  "$out/UOFiddler-${version}-x86_64.AppImage" >/dev/null 2>&1

ls -la "$out"
