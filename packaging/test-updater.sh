#!/usr/bin/env bash
# End-to-end test of the self-updater (used by .github/workflows/updater-test.yml, runnable locally).
#   packaging/test-updater.sh <linux|windows> [work dir]
# Builds the app as build 1 ("installed") and as build 2 ("release", packaged like the release job),
# runs the installed one with `--self-update <release dir>` (same download/verify/apply code as the
# GUI, but a local directory instead of GitHub, no window) and checks the result. Also checks that a
# tampered SHA256SUMS.txt is refused and leaves the installation untouched.
# Linux additionally runs the AppImage case (needs appimagetool, downloaded by package.sh).
set -euo pipefail

os="$1"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work="${2:-$(mktemp -d)}"
cd "$root"
mkdir -p "$work"
work="$(cd "$work" && pwd)"
echo "work dir: $work"

if [ "$os" = windows ]; then exe=uo_gui.exe; else exe=uo_gui; fi
version_base="$(sed -n 's/^version = "\(.*\)"/\1/p' crates/uo_gui/Cargo.toml | head -1)"
v1="$version_base.1"; v2="$version_base.2"

fail() { echo "FAIL: $*" >&2; exit 1; }
ver() { "$1" --version | tr -d '\r\n'; }
expect_ver() { [ "$(ver "$1")" = "$2" ] || fail "$1 prints '$(ver "$1")', expected $2"; }

echo "== build 1 (installed) and build 2 (release)"
UOF_BUILD=1 cargo build --release --locked
mkdir -p "$work/build1" && cp "target/release/$exe" "$work/build1/$exe"
UOF_BUILD=2 cargo build --release --locked
mkdir -p "$work/build2" && cp "target/release/$exe" "$work/build2/$exe"

release="$work/release"; mkdir -p "$release"
if [ "$os" = windows ]; then
  # Same layout as the release job: the .exe at the root of the zip.
  # PowerShell needs Windows paths, not the /d/a/... ones of Git Bash.
  src_win="$(cygpath -w "$work/build2/$exe")"; zip_win="$(cygpath -w "$release")\\uofiddler-$v2-windows-x86_64.zip"
  powershell -NoProfile -Command "Compress-Archive -Path '$src_win' -DestinationPath '$zip_win'"
else
  # Local rehearsal on a non-x86_64 machine (releases only have x86_64 files): skip the AppImage
  # (appimagetool is x86_64-only) and tell the headless updater to assume x86_64.
  if [ "$(uname -m)" != x86_64 ]; then export UOF_UPDATE_TEST_ARCH=x86_64 SKIP_APPIMAGE=1; fi
  packaging/linux/package.sh "$work/build2/$exe" "$v2" "$release"
fi
(cd "$release" && sha256sum * > SHA256SUMS.txt)
ls -l "$release"

# fresh_install <dir>: a copy of build 1 as an "installed" program.
fresh_install() {
  rm -rf "$1"; mkdir -p "$1"
  if [ "$os" = windows ]; then cp "$work/build1/$exe" "$1/$exe"; else cp "$work/build1/$exe" "$1/uofiddler"; fi
}
installed_path() { if [ "$os" = windows ]; then echo "$1/$exe"; else echo "$1/uofiddler"; fi; }

echo "== positive: update the plain executable ($os)"
d="$work/inst-ok"; fresh_install "$d"; bin="$(installed_path "$d")"
expect_ver "$bin" 1
"$bin" --self-update "$release"
expect_ver "$bin" 2
[ -f "$bin.old" ] || fail "backup $bin.old missing"
expect_ver "$bin.old" 1   # the old executable is kept as the backup
echo "   ok: now build 2, backup kept and still build 1"

echo "== negative: tampered SHA256SUMS.txt is refused"
bad="$work/release-bad"; rm -rf "$bad"; cp -r "$release" "$bad"
# change the first digit of every hash, so each is certainly wrong
awk '{ h=$1; f=(substr(h,1,1)=="0") ? "1" : "0"; print f substr(h,2) "  " $2 }' "$release/SHA256SUMS.txt" > "$bad/SHA256SUMS.txt"
d="$work/inst-bad"; fresh_install "$d"; bin="$(installed_path "$d")"
if "$bin" --self-update "$bad"; then fail "tampered checksums were accepted"; fi
expect_ver "$bin" 1
[ ! -e "$bin.old" ] || fail "backup created although nothing was applied"
[ ! -e "$bin.new" ] || fail "staging file left behind"
echo "   ok: refused, installation still build 1"

if [ "$os" = linux ] && [ "$(uname -m)" != x86_64 ]; then
  echo "== AppImage case SKIPPED: appimagetool is x86_64-only and this machine is $(uname -m)"
elif [ "$os" = linux ]; then
  echo "== AppImage: update the AppImage file itself"
  export APPIMAGE_EXTRACT_AND_RUN=1   # no FUSE needed to run AppImages on the runner
  mkdir -p "$work/ai1"
  packaging/linux/package.sh "$work/build1/$exe" "$v1" "$work/ai1"
  d="$work/inst-appimage"; rm -rf "$d"; mkdir -p "$d"
  cp "$work/ai1/UOFiddler-$v1-x86_64.AppImage" "$d/UOFiddler.AppImage"
  ai="$d/UOFiddler.AppImage"
  expect_ver "$ai" 1
  APPIMAGE="$ai" "$ai" --self-update "$release"
  expect_ver "$ai" 2
  [ -f "$ai.old" ] || fail "AppImage backup missing"
  expect_ver "$ai.old" 1
  echo "   ok: AppImage replaced, backup kept"
fi
echo "ALL UPDATER TESTS PASSED"
