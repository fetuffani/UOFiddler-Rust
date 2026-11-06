//! Auto-updater based on the GitHub releases of `mbnunes/UOFiddler-Rust`. UI-free: the egui side
//! lives in `update_ui.rs`.
//!
//! Releases are tagged `build-N` and carry the assets `uofiddler-0.1.0.N-windows-x86_64.zip`,
//! `uofiddler-0.1.0.N-linux-x86_64.tar.gz`, `UOFiddler-0.1.0.N-x86_64.AppImage`,
//! `uofiddler_0.1.0.N_amd64.deb` and `SHA256SUMS.txt`. The running build number is embedded at
//! compile time (`UOF_BUILD`, see `build.rs`; 0 = local "dev" build).
//!
//! Safety rules: nothing is downloaded or applied here without the caller having asked for it, the
//! SHA-256 from `SHA256SUMS.txt` is checked before anything touches the installation, the old
//! executable is kept as `<exe>.old` and put back when the new one does not start.

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

pub const REPO: &str = "mbnunes/UOFiddler-Rust";
const LATEST_URL: &str = "https://api.github.com/repos/mbnunes/UOFiddler-Rust/releases/latest";
const CHECKSUMS_ASSET: &str = "SHA256SUMS.txt";
/// Env var the old process sets for the new one: path of the file to create once it is up.
pub const ACK_ENV: &str = "UOF_UPDATE_ACK";
/// How long the old process waits for the new one to say it started.
const ACK_TIMEOUT: Duration = Duration::from_secs(30);

/// Build number of the running binary (0 = dev build, not made by the CI).
pub fn current_build() -> u32 {
    env!("UOF_BUILD").parse().unwrap_or(0)
}

/// `0.1.0.N`, or `0.1.0` for a dev build.
pub fn current_version() -> String {
    match current_build() {
        0 => env!("CARGO_PKG_VERSION").to_string(),
        n => format!("{}.{n}", env!("CARGO_PKG_VERSION")),
    }
}

// ---------------------------------------------------------------- errors

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateError {
    /// No network / GitHub unavailable / bad HTTP status.
    Network(String),
    /// The API answer is not a release we understand.
    BadRelease,
    /// The release has no file for this platform.
    NoAsset,
    /// `SHA256SUMS.txt` is missing or has no line for the file: never applied.
    NoChecksum,
    /// The downloaded file does not match `SHA256SUMS.txt`: never applied.
    HashMismatch,
    /// The installation folder cannot be written.
    Permission(String),
    Io(String),
    /// The new version did not start; the old one was put back.
    RolledBack,
    Cancelled,
}

impl UpdateError {
    /// Localization key of the message (the detail, when any, goes in `{detail}`).
    pub fn key(&self) -> &'static str {
        match self {
            UpdateError::Network(_) => "update.err.network",
            UpdateError::BadRelease => "update.err.badrelease",
            UpdateError::NoAsset => "update.err.noasset",
            UpdateError::NoChecksum => "update.err.nochecksum",
            UpdateError::HashMismatch => "update.err.hash",
            UpdateError::Permission(_) => "update.err.permission",
            UpdateError::Io(_) => "update.err.io",
            UpdateError::RolledBack => "update.err.rolledback",
            UpdateError::Cancelled => "update.err.cancelled",
        }
    }

    pub fn detail(&self) -> String {
        match self {
            UpdateError::Network(s) | UpdateError::Permission(s) | UpdateError::Io(s) => s.clone(),
            _ => String::new(),
        }
    }
}

fn io_err(e: std::io::Error) -> UpdateError {
    if e.kind() == std::io::ErrorKind::PermissionDenied {
        UpdateError::Permission(e.to_string())
    } else {
        UpdateError::Io(e.to_string())
    }
}

// ---------------------------------------------------------------- versions

/// `build-12` -> 12.
pub fn parse_build_tag(tag: &str) -> Option<u32> {
    tag.trim().strip_prefix("build-")?.parse().ok()
}

/// `0.1.0.12` -> 12 (the last component is the build number; a plain `0.1.0` has none).
pub fn parse_version_build(version: &str) -> Option<u32> {
    let parts: Vec<&str> = version.trim().split('.').collect();
    if parts.len() < 4 {
        return None;
    }
    parts.last()?.parse().ok()
}

/// Version text inside an asset name (`uofiddler-0.1.0.12-linux-x86_64.tar.gz` -> `0.1.0.12`).
pub fn version_from_asset_name(name: &str) -> Option<String> {
    name.split(|c: char| c == '-' || c == '_')
        .find(|p| p.chars().next().map_or(false, |c| c.is_ascii_digit()) && parse_version_build(p).is_some())
        .map(|p| p.trim_end_matches(".AppImage").to_string())
}

/// True when `latest` is a newer build than `current`.
pub fn is_newer(current: u32, latest: u32) -> bool {
    latest > current
}

// ---------------------------------------------------------------- release

#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    pub name: String,
    pub url: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Release {
    pub build: u32,
    pub version: String,
    pub notes: String,
    pub page_url: String,
    pub assets: Vec<Asset>,
}

/// Reads the answer of `GET /repos/.../releases/latest`.
pub fn parse_release(json: &str) -> Result<Release, UpdateError> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|_| UpdateError::BadRelease)?;
    let assets: Vec<Asset> = v["assets"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| {
                    Some(Asset {
                        name: x["name"].as_str()?.to_string(),
                        url: x["browser_download_url"].as_str()?.to_string(),
                        size: x["size"].as_u64().unwrap_or(0),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    let tag = v["tag_name"].as_str().unwrap_or("");
    let asset_version = assets.iter().find_map(|a| version_from_asset_name(&a.name));
    let build = parse_build_tag(tag)
        .or_else(|| asset_version.as_deref().and_then(parse_version_build))
        .ok_or(UpdateError::BadRelease)?;
    let version = asset_version.unwrap_or_else(|| format!("{}.{build}", env!("CARGO_PKG_VERSION")));
    let page_url = v["html_url"]
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| format!("https://github.com/{REPO}/releases/latest"));
    Ok(Release { build, version, notes: v["body"].as_str().unwrap_or("").to_string(), page_url, assets })
}

// ---------------------------------------------------------------- platform

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageKind {
    /// Windows: `.exe` inside the `.zip`.
    WindowsZip,
    /// Linux AppImage: the downloaded file replaces the running one.
    AppImage,
    /// Linux plain archive: the `uofiddler` binary inside the `.tar.gz`.
    TarGz,
    /// System package (.deb), other systems or CPUs: only the release page is opened.
    PageOnly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Install {
    pub kind: PackageKind,
    /// The file that gets replaced (the .exe, the AppImage or the plain binary).
    pub target: PathBuf,
}

/// Works out how this copy of the app is installed. `appimage_env` is `$APPIMAGE` (only when it
/// points at an existing file).
pub fn detect_install(os: &str, arch: &str, exe: &Path, appimage_env: Option<PathBuf>) -> Install {
    let page_only = Install { kind: PackageKind::PageOnly, target: exe.to_path_buf() };
    if arch != "x86_64" {
        return page_only;
    }
    match os {
        "windows" => Install { kind: PackageKind::WindowsZip, target: exe.to_path_buf() },
        "linux" => {
            if let Some(p) = appimage_env {
                Install { kind: PackageKind::AppImage, target: p }
            } else if exe.starts_with("/usr/") {
                page_only // installed by the .deb (or another package manager)
            } else {
                Install { kind: PackageKind::TarGz, target: exe.to_path_buf() }
            }
        }
        _ => page_only,
    }
}

pub fn detect_current_install() -> Install {
    let exe = std::env::current_exe().unwrap_or_default();
    let appimage = std::env::var_os("APPIMAGE").map(PathBuf::from).filter(|p| p.is_file());
    // UOF_UPDATE_TEST_ARCH exists only so the script packaging/test-updater.sh can be rehearsed on
    // machines that are not x86_64 (the releases only have x86_64 files); the GUI never reads it.
    let arch = std::env::var("UOF_UPDATE_TEST_ARCH").ok().filter(|_| HEADLESS.load(Ordering::Relaxed));
    detect_install(std::env::consts::OS, arch.as_deref().unwrap_or(std::env::consts::ARCH), &exe, appimage)
}

/// Set by the headless `--self-update` entry point.
static HEADLESS: AtomicBool = AtomicBool::new(false);

/// The release file for this kind of install.
pub fn select_asset(assets: &[Asset], kind: PackageKind) -> Option<&Asset> {
    let ok = |n: &str| match kind {
        PackageKind::WindowsZip => n.ends_with("-windows-x86_64.zip"),
        PackageKind::AppImage => n.ends_with("-x86_64.AppImage"),
        PackageKind::TarGz => n.ends_with("-linux-x86_64.tar.gz"),
        PackageKind::PageOnly => false,
    };
    assets.iter().find(|a| ok(&a.name))
}

// ---------------------------------------------------------------- checksums

/// Parses `sha256sum` output (`<hash>  <name>` or `<hash> *<name>`) into name -> lowercase hash.
pub fn parse_sha256sums(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|l| {
            let (hash, name) = l.trim().split_once(char::is_whitespace)?;
            let name = name.trim().trim_start_matches('*');
            let valid = hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit());
            (valid && !name.is_empty()).then(|| (name.to_string(), hash.to_ascii_lowercase()))
        })
        .collect()
}

pub fn sha256_hex_of_file(path: &Path) -> std::io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// Ok only when the file's SHA-256 equals `expected`.
pub fn verify_sha256(path: &Path, expected: &str) -> Result<(), UpdateError> {
    let actual = sha256_hex_of_file(path).map_err(io_err)?;
    if actual.eq_ignore_ascii_case(expected.trim()) {
        Ok(())
    } else {
        Err(UpdateError::HashMismatch)
    }
}

// ---------------------------------------------------------------- applying

fn with_suffix(p: &Path, suffix: &str) -> PathBuf {
    let mut s = p.as_os_str().to_os_string();
    s.push(suffix);
    PathBuf::from(s)
}

/// Where the old executable is kept.
pub fn backup_path(target: &Path) -> PathBuf {
    with_suffix(target, ".old")
}

fn staging_path(target: &Path) -> PathBuf {
    with_suffix(target, ".new")
}

/// Copies the entry whose file name is `wanted` out of the archive into `out`.
fn extract_entry(kind: PackageKind, archive: &Path, wanted: &str, out: &Path) -> Result<(), UpdateError> {
    let file = fs::File::open(archive).map_err(io_err)?;
    let mut dest = fs::File::create(out).map_err(io_err)?;
    let mut found = false;
    match kind {
        PackageKind::WindowsZip => {
            let mut z = zip::ZipArchive::new(file).map_err(|e| UpdateError::Io(e.to_string()))?;
            for i in 0..z.len() {
                let mut e = z.by_index(i).map_err(|e| UpdateError::Io(e.to_string()))?;
                let name = e.name().rsplit('/').next().unwrap_or("").to_string();
                if e.is_file() && name == wanted {
                    std::io::copy(&mut e, &mut dest).map_err(io_err)?;
                    found = true;
                    break;
                }
            }
        }
        PackageKind::TarGz => {
            let mut t = tar::Archive::new(flate2::read::GzDecoder::new(file));
            for e in t.entries().map_err(io_err)? {
                let mut e = e.map_err(io_err)?;
                let is_it = e.header().entry_type().is_file()
                    && e.path().map_err(io_err)?.file_name().map_or(false, |n| n == wanted);
                if is_it {
                    std::io::copy(&mut e, &mut dest).map_err(io_err)?;
                    found = true;
                    break;
                }
            }
        }
        _ => {}
    }
    dest.flush().map_err(io_err)?;
    if !found || fs::metadata(out).map(|m| m.len()).unwrap_or(0) == 0 {
        let _ = fs::remove_file(out);
        return Err(UpdateError::BadRelease);
    }
    Ok(())
}

#[cfg(unix)]
fn make_executable(p: &Path) -> Result<(), UpdateError> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(p, fs::Permissions::from_mode(0o755)).map_err(io_err)
}
#[cfg(not(unix))]
fn make_executable(_: &Path) -> Result<(), UpdateError> {
    Ok(())
}

/// Replaces `target` by `new`, keeping the old one as `<target>.old`; puts it back if the second
/// step fails. Renaming a running executable is allowed on Linux and on Windows (deleting is not),
/// so the same code serves both. Returns the backup path.
pub fn swap_in(target: &Path, new: &Path) -> Result<PathBuf, UpdateError> {
    let backup = backup_path(target);
    let _ = fs::remove_file(&backup);
    fs::rename(target, &backup).map_err(io_err)?;
    if let Err(e) = fs::rename(new, target) {
        let _ = fs::rename(&backup, target);
        return Err(io_err(e));
    }
    Ok(backup)
}

/// Undoes `swap_in`.
pub fn rollback(target: &Path, backup: &Path) -> Result<(), UpdateError> {
    let _ = fs::remove_file(target);
    fs::rename(backup, target).map_err(io_err)
}

/// Verifies `downloaded` against `expected_hash` FIRST, then prepares the new executable next to
/// `target` and swaps it in. With a wrong hash nothing in the installation is touched.
pub fn verify_and_install(kind: PackageKind, downloaded: &Path, expected_hash: &str, target: &Path) -> Result<PathBuf, UpdateError> {
    verify_sha256(downloaded, expected_hash)?;
    let new = staging_path(target);
    let _ = fs::remove_file(&new);
    let staged = match kind {
        PackageKind::AppImage => fs::copy(downloaded, &new).map(|_| ()).map_err(io_err),
        PackageKind::WindowsZip => extract_entry(kind, downloaded, "uo_gui.exe", &new),
        PackageKind::TarGz => extract_entry(kind, downloaded, "uofiddler", &new),
        PackageKind::PageOnly => Err(UpdateError::NoAsset),
    };
    if let Err(e) = staged.and_then(|_| make_executable(&new)) {
        let _ = fs::remove_file(&new);
        return Err(e);
    }
    swap_in(target, &new).map_err(|e| {
        let _ = fs::remove_file(&new);
        e
    })
}

/// Starts the new executable and waits for it to confirm (see `confirm_started`). When it exits or
/// stays silent, the old executable is put back and `RolledBack` is returned.
pub fn restart_and_confirm(target: &Path, backup: &Path) -> Result<(), UpdateError> {
    let ack = std::env::temp_dir().join(format!("uofiddler-update-ack-{}", std::process::id()));
    let _ = fs::remove_file(&ack);
    let mut cmd = Command::new(target);
    cmd.args(std::env::args_os().skip(1)).env(ACK_ENV, &ack);
    // The new AppImage must mount itself, not reuse the old one's mount.
    for v in ["APPDIR", "APPIMAGE", "ARGV0", "OWD"] {
        cmd.env_remove(v);
    }
    let spawned: Result<Child, _> = cmd.spawn();
    let mut child = match spawned {
        Ok(c) => c,
        Err(e) => {
            let _ = rollback(target, backup);
            return Err(io_err(e));
        }
    };
    let start = Instant::now();
    loop {
        if ack.exists() {
            let _ = fs::remove_file(&ack);
            return Ok(());
        }
        let exited = matches!(child.try_wait(), Ok(Some(_)));
        if exited || start.elapsed() > ACK_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            rollback(target, backup)?;
            return Err(UpdateError::RolledBack);
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// Headless check of the freshly installed executable: `--version` must print the expected build.
pub fn smoke_test(target: &Path, backup: &Path, expected_build: u32) -> Result<(), UpdateError> {
    let out = Command::new(target).arg("--version").output();
    let ok = matches!(&out, Ok(o) if o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == expected_build.to_string());
    if ok {
        Ok(())
    } else {
        rollback(target, backup)?;
        Err(UpdateError::RolledBack)
    }
}

/// Command line entry points that run without a window. Returns the exit code, or `None` when the
/// arguments ask for the normal GUI.
///   `--version`                 prints the embedded build number
///   `--self-update <dir>`       runs the real update path against a local release directory
///                               (also `UOF_UPDATE_SOURCE=<dir>`), then exits 0 on success
pub fn cli(args: &[String]) -> Option<i32> {
    if args.iter().any(|a| a == "--version") {
        println!("{}", current_build());
        return Some(0);
    }
    let pos = args.iter().position(|a| a == "--self-update")?;
    HEADLESS.store(true, Ordering::Relaxed);
    let source = args.get(pos + 1).cloned().or_else(|| std::env::var("UOF_UPDATE_SOURCE").ok());
    let Some(source) = source else {
        eprintln!("--self-update needs a release directory");
        return Some(2);
    };
    let result = (|| {
        let release = local_release(Path::new(&source))?;
        if !is_newer(current_build(), release.build) {
            println!("up to date ({})", current_build());
            return Ok(());
        }
        let install = detect_current_install();
        if install.kind == PackageKind::PageOnly {
            return Err(UpdateError::NoAsset);
        }
        run_update(&release, &install, &Progress::default(), true)?;
        println!("updated to build {}", release.build);
        Ok(())
    })();
    match result {
        Ok(()) => Some(0),
        Err(e) => {
            eprintln!("update failed: {e:?}");
            Some(1)
        }
    }
}

/// Called by the new version once its window is up: tells the old process it can quit.
pub fn confirm_started() {
    if let Some(p) = std::env::var_os(ACK_ENV) {
        std::env::remove_var(ACK_ENV);
        let _ = fs::write(p, b"ok");
    }
}

/// At startup (not right after an update): drops the backup of a previous update.
pub fn cleanup_backup() {
    if std::env::var_os(ACK_ENV).is_some() {
        return; // the old process is still running from the backup
    }
    let install = detect_current_install();
    if install.kind != PackageKind::PageOnly {
        let _ = fs::remove_file(backup_path(&install.target));
    }
}

// ---------------------------------------------------------------- network

/// Local file behind a `file://` URL. Only produced by `local_release` (headless `--self-update`
/// tests); releases that come from GitHub must be `https://`, which `https_only` enforces.
fn local_path(url: &str) -> Option<PathBuf> {
    url.strip_prefix("file://").map(PathBuf::from)
}

/// A release laid out in a local directory (the release assets plus `SHA256SUMS.txt`), for the
/// headless `--self-update <dir>` used by the updater CI test. The build comes from the file names.
pub fn local_release(dir: &Path) -> Result<Release, UpdateError> {
    let mut assets = Vec::new();
    for e in fs::read_dir(dir).map_err(io_err)?.flatten() {
        let path = e.path();
        if path.is_file() {
            let name = e.file_name().to_string_lossy().to_string();
            let size = e.metadata().map(|m| m.len()).unwrap_or(0);
            let abs = fs::canonicalize(&path).unwrap_or(path);
            assets.push(Asset { name, url: format!("file://{}", abs.display()), size });
        }
    }
    let version = assets.iter().find_map(|a| version_from_asset_name(&a.name)).ok_or(UpdateError::BadRelease)?;
    let build = parse_version_build(&version).ok_or(UpdateError::BadRelease)?;
    Ok(Release { build, version, notes: String::new(), page_url: String::new(), assets })
}

fn agent(connect: u64, read: u64) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(connect))
        .timeout_read(Duration::from_secs(read))
        .timeout_write(Duration::from_secs(read))
        .user_agent(&format!("UOFiddler-updater/{}", current_version()))
        .build()
}

fn net_err(e: ureq::Error) -> UpdateError {
    UpdateError::Network(e.to_string())
}

fn https_only(url: &str) -> Result<(), UpdateError> {
    if url.starts_with("https://") { Ok(()) } else { Err(UpdateError::BadRelease) }
}

fn get_text(url: &str, limit: u64) -> Result<String, UpdateError> {
    if let Some(path) = local_path(url) {
        return fs::read_to_string(path).map_err(io_err);
    }
    https_only(url)?;
    let resp = agent(5, 10).get(url).set("Accept", "application/vnd.github+json").call().map_err(net_err)?;
    let mut s = String::new();
    resp.into_reader().take(limit).read_to_string(&mut s).map_err(|e| UpdateError::Network(e.to_string()))?;
    Ok(s)
}

/// Asks GitHub for the latest release (blocking; call it from a background thread).
pub fn fetch_latest() -> Result<Release, UpdateError> {
    parse_release(&get_text(LATEST_URL, 2_000_000)?)
}

/// Progress shared with the UI.
#[derive(Default)]
pub struct Progress {
    pub done: AtomicU64,
    pub total: AtomicU64,
    /// 0 downloading, 1 verifying, 2 installing, 3 restarting
    pub phase: AtomicU64,
    pub cancel: AtomicBool,
}

fn download(url: &str, to: &Path, p: &Progress) -> Result<(), UpdateError> {
    let mut r: Box<dyn Read> = if let Some(path) = local_path(url) {
        Box::new(fs::File::open(path).map_err(io_err)?)
    } else {
        https_only(url)?;
        let resp = agent(10, 30).get(url).call().map_err(net_err)?;
        if let Some(len) = resp.header("Content-Length").and_then(|v| v.parse::<u64>().ok()) {
            p.total.store(len, Ordering::Relaxed);
        }
        Box::new(resp.into_reader())
    };
    let mut f = fs::File::create(to).map_err(io_err)?;
    let mut buf = [0u8; 64 * 1024];
    loop {
        if p.cancel.load(Ordering::Relaxed) {
            return Err(UpdateError::Cancelled);
        }
        let n = r.read(&mut buf).map_err(|e| UpdateError::Network(e.to_string()))?;
        if n == 0 {
            break;
        }
        f.write_all(&buf[..n]).map_err(io_err)?;
        p.done.fetch_add(n as u64, Ordering::Relaxed);
    }
    f.flush().map_err(io_err)
}

/// The whole update, to run on a background thread after the user accepted: download, verify,
/// install, restart. `Ok` means the new version is running and this process should quit.
///
/// With `headless` there is no window to restart: the new executable is only run with `--version`
/// to prove it starts and reports the release's build; on failure the old one is put back.
pub fn run_update(release: &Release, install: &Install, p: &Progress, headless: bool) -> Result<(), UpdateError> {
    let asset = select_asset(&release.assets, install.kind).ok_or(UpdateError::NoAsset)?;
    let sums_asset = release.assets.iter().find(|a| a.name == CHECKSUMS_ASSET).ok_or(UpdateError::NoChecksum)?;
    let sums = parse_sha256sums(&get_text(&sums_asset.url, 1_000_000)?);
    let expected = sums.get(&asset.name).ok_or(UpdateError::NoChecksum)?.clone();

    // Fail early, with a clear message, when the install folder cannot be written.
    let probe = with_suffix(&install.target, ".probe");
    fs::write(&probe, b"").map_err(io_err)?;
    let _ = fs::remove_file(&probe);

    let dir = std::env::temp_dir().join(format!("uofiddler-update-{}", release.build));
    fs::create_dir_all(&dir).map_err(io_err)?;
    let file = dir.join(&asset.name);
    p.total.store(asset.size, Ordering::Relaxed);
    let result = (|| {
        download(&asset.url, &file, p)?;
        p.phase.store(1, Ordering::Relaxed);
        p.phase.store(2, Ordering::Relaxed);
        let backup = verify_and_install(install.kind, &file, &expected, &install.target)?;
        p.phase.store(3, Ordering::Relaxed);
        if headless {
            smoke_test(&install.target, &backup, release.build)
        } else {
            restart_and_confirm(&install.target, &backup)
        }
    })();
    let _ = fs::remove_dir_all(&dir);
    result
}

// ---------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("uof-updater-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn hash_of(bytes: &[u8]) -> String {
        Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
    }

    fn asset(n: &str) -> Asset {
        Asset { name: n.into(), url: format!("https://x/{n}"), size: 1 }
    }

    fn assets() -> Vec<Asset> {
        ["uofiddler-0.1.0.7-windows-x86_64.zip", "uofiddler-0.1.0.7-linux-x86_64.tar.gz", "UOFiddler-0.1.0.7-x86_64.AppImage", "uofiddler_0.1.0.7_amd64.deb", "SHA256SUMS.txt"]
            .iter()
            .map(|n| asset(n))
            .collect()
    }

    #[test]
    fn versions_and_tags() {
        assert_eq!(parse_build_tag("build-12"), Some(12));
        assert_eq!(parse_build_tag("v1.2"), None);
        assert_eq!(parse_build_tag("build-x"), None);
        assert_eq!(parse_version_build("0.1.0.12"), Some(12));
        assert_eq!(parse_version_build("0.1.0"), None);
        assert_eq!(version_from_asset_name("uofiddler-0.1.0.12-linux-x86_64.tar.gz").as_deref(), Some("0.1.0.12"));
        assert_eq!(version_from_asset_name("UOFiddler-0.1.0.12-x86_64.AppImage").as_deref(), Some("0.1.0.12"));
        assert_eq!(version_from_asset_name("uofiddler_0.1.0.12_amd64.deb").as_deref(), Some("0.1.0.12"));
        assert_eq!(version_from_asset_name("SHA256SUMS.txt"), None);
    }

    #[test]
    fn newer_is_a_numeric_build_comparison() {
        assert!(is_newer(9, 10)); // not a string comparison
        assert!(!is_newer(10, 10));
        assert!(!is_newer(11, 10));
        assert!(is_newer(0, 1)); // dev build
    }

    #[test]
    fn release_json() {
        let json = r#"{"tag_name":"build-7","html_url":"https://github.com/o/r/releases/tag/build-7","body":"notes",
            "assets":[{"name":"uofiddler-0.1.0.7-linux-x86_64.tar.gz","browser_download_url":"https://d/x.tar.gz","size":42}]}"#;
        let r = parse_release(json).unwrap();
        assert_eq!((r.build, r.version.as_str(), r.notes.as_str()), (7, "0.1.0.7", "notes"));
        assert_eq!(r.assets[0].size, 42);
        assert_eq!(parse_release("{}"), Err(UpdateError::BadRelease));
        assert_eq!(parse_release("not json"), Err(UpdateError::BadRelease));
    }

    #[test]
    fn asset_selection_per_platform() {
        let a = assets();
        assert_eq!(select_asset(&a, PackageKind::WindowsZip).unwrap().name, "uofiddler-0.1.0.7-windows-x86_64.zip");
        assert_eq!(select_asset(&a, PackageKind::AppImage).unwrap().name, "UOFiddler-0.1.0.7-x86_64.AppImage");
        assert_eq!(select_asset(&a, PackageKind::TarGz).unwrap().name, "uofiddler-0.1.0.7-linux-x86_64.tar.gz");
        assert!(select_asset(&a, PackageKind::PageOnly).is_none());
        assert!(select_asset(&a[..1], PackageKind::AppImage).is_none());
    }

    #[test]
    fn install_detection() {
        let exe = Path::new("/home/u/app/uofiddler");
        let ai = Some(PathBuf::from("/home/u/UOFiddler.AppImage"));
        assert_eq!(detect_install("linux", "x86_64", exe, ai.clone()), Install { kind: PackageKind::AppImage, target: ai.unwrap() });
        assert_eq!(detect_install("linux", "x86_64", exe, None).kind, PackageKind::TarGz);
        assert_eq!(detect_install("linux", "x86_64", Path::new("/usr/bin/uofiddler"), None).kind, PackageKind::PageOnly);
        assert_eq!(detect_install("windows", "x86_64", Path::new("C:\\a\\uo_gui.exe"), None).kind, PackageKind::WindowsZip);
        assert_eq!(detect_install("linux", "aarch64", exe, None).kind, PackageKind::PageOnly);
        assert_eq!(detect_install("macos", "x86_64", exe, None).kind, PackageKind::PageOnly);
    }

    #[test]
    fn sha256sums_parsing() {
        let h1 = "a".repeat(64);
        let h2 = "B".repeat(64);
        let text = format!("{h1}  one.zip\n{h2} *two.tar.gz\nnot a line\n123  short.txt\n\n");
        let m = parse_sha256sums(&text);
        assert_eq!(m.len(), 2);
        assert_eq!(m["one.zip"], h1);
        assert_eq!(m["two.tar.gz"], "b".repeat(64));
    }

    #[test]
    fn hash_verification() {
        let d = tmpdir("hash");
        let f = d.join("f.bin");
        fs::write(&f, b"hello").unwrap();
        // sha256("hello")
        let good = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert_eq!(hash_of(b"hello"), good);
        assert_eq!(verify_sha256(&f, good), Ok(()));
        assert_eq!(verify_sha256(&f, &good.to_uppercase()), Ok(()));
        assert_eq!(verify_sha256(&f, &"0".repeat(64)), Err(UpdateError::HashMismatch));
    }

    #[test]
    fn hash_mismatch_never_touches_the_installation() {
        let d = tmpdir("mismatch");
        let target = d.join("uofiddler");
        fs::write(&target, b"old").unwrap();
        let dl = d.join("UOFiddler.AppImage");
        fs::write(&dl, b"tampered").unwrap();
        let r = verify_and_install(PackageKind::AppImage, &dl, &hash_of(b"genuine"), &target);
        assert_eq!(r, Err(UpdateError::HashMismatch));
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert!(!backup_path(&target).exists());
        assert!(!staging_path(&target).exists());
    }

    #[test]
    fn appimage_install_keeps_backup_and_rolls_back() {
        let d = tmpdir("appimage");
        let target = d.join("UOFiddler.AppImage");
        fs::write(&target, b"old").unwrap();
        let dl = d.join("dl");
        fs::write(&dl, b"new build").unwrap();
        let backup = verify_and_install(PackageKind::AppImage, &dl, &hash_of(b"new build"), &target).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"new build");
        assert_eq!(fs::read(&backup).unwrap(), b"old");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(fs::metadata(&target).unwrap().permissions().mode() & 0o111, 0o111);
        }
        rollback(&target, &backup).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert!(!backup.exists());
    }

    #[test]
    fn swap_failure_restores_the_old_file() {
        let d = tmpdir("swapfail");
        let target = d.join("app");
        fs::write(&target, b"old").unwrap();
        // The new file does not exist: the second rename fails and the first is undone.
        assert!(swap_in(&target, &d.join("missing")).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"old");
    }

    #[test]
    fn tar_gz_install_extracts_the_binary() {
        let d = tmpdir("targz");
        let archive = d.join("a.tar.gz");
        {
            let enc = flate2::write::GzEncoder::new(fs::File::create(&archive).unwrap(), flate2::Compression::fast());
            let mut b = tar::Builder::new(enc);
            for (name, data) in [("uofiddler-0.1.0.7-linux-x86_64/README.md", &b"readme"[..]), ("uofiddler-0.1.0.7-linux-x86_64/uofiddler", &b"BINARY"[..])] {
                let mut h = tar::Header::new_gnu();
                h.set_size(data.len() as u64);
                h.set_mode(0o644);
                h.set_cksum();
                b.append_data(&mut h, name, data).unwrap();
            }
            b.into_inner().unwrap().finish().unwrap();
        }
        let target = d.join("uofiddler");
        fs::write(&target, b"old").unwrap();
        let hash = sha256_hex_of_file(&archive).unwrap();
        verify_and_install(PackageKind::TarGz, &archive, &hash, &target).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"BINARY");
        assert_eq!(fs::read(backup_path(&target)).unwrap(), b"old");
    }

    #[test]
    fn zip_install_extracts_the_exe() {
        let d = tmpdir("zip");
        let archive = d.join("a.zip");
        {
            let mut z = zip::ZipWriter::new(fs::File::create(&archive).unwrap());
            z.start_file("uo_gui.exe", zip::write::SimpleFileOptions::default()).unwrap();
            z.write_all(b"MZ-new").unwrap();
            z.finish().unwrap();
        }
        let target = d.join("uo_gui.exe");
        fs::write(&target, b"MZ-old").unwrap();
        let hash = sha256_hex_of_file(&archive).unwrap();
        verify_and_install(PackageKind::WindowsZip, &archive, &hash, &target).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"MZ-new");
        assert_eq!(fs::read(backup_path(&target)).unwrap(), b"MZ-old");
    }

    #[test]
    fn archive_without_the_binary_is_rejected_and_target_kept() {
        let d = tmpdir("nobin");
        let archive = d.join("a.tar.gz");
        {
            let enc = flate2::write::GzEncoder::new(fs::File::create(&archive).unwrap(), flate2::Compression::fast());
            let mut b = tar::Builder::new(enc);
            let mut h = tar::Header::new_gnu();
            h.set_size(1);
            h.set_cksum();
            b.append_data(&mut h, "x/other", &b"x"[..]).unwrap();
            b.into_inner().unwrap().finish().unwrap();
        }
        let target = d.join("uofiddler");
        fs::write(&target, b"old").unwrap();
        let hash = sha256_hex_of_file(&archive).unwrap();
        assert_eq!(verify_and_install(PackageKind::TarGz, &archive, &hash, &target), Err(UpdateError::BadRelease));
        assert_eq!(fs::read(&target).unwrap(), b"old");
    }
}
