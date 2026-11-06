//! Case-insensitive file discovery and path manager for Ultima Online files on Linux and Windows.
//! Supports both Classic Client (2D) and Enhanced Client (3D) directories.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Client languages: (file suffix, name). Localized files are named `<resource>.<suffix>`
/// (`Cliloc.enu`, `Tooltips.deu`, `Intloc00.fra`, ...).
pub const LANGUAGES: [(&str, &str); 8] = [
    ("enu", "Inglês"),
    ("deu", "Alemão"),
    ("esp", "Espanhol"),
    ("fra", "Francês"),
    ("jpn", "Japonês"),
    ("kor", "Coreano"),
    ("cht", "Chinês tradicional"),
    ("chs", "Chinês simplificado"),
];

/// "Inglês (enu)" for a known suffix, the suffix itself otherwise.
pub fn language_name(code: &str) -> String {
    let c = code.to_lowercase();
    match LANGUAGES.iter().find(|(k, _)| *k == c) {
        Some((k, n)) => format!("{} ({})", n, k),
        None => c,
    }
}

/// Orders language suffixes like `LANGUAGES` (unknown ones last, alphabetical) and removes duplicates.
pub fn sort_languages(mut codes: Vec<String>) -> Vec<String> {
    for c in &mut codes {
        *c = c.to_lowercase();
    }
    codes.sort_by_key(|c| (LANGUAGES.iter().position(|(k, _)| k == c).unwrap_or(usize::MAX), c.clone()));
    codes.dedup();
    codes
}

pub struct FileManager {
    /// language used for localized files (suffix, e.g. "enu")
    pub language: String,
    /// interface language (None = detect from the system on first run)
    pub ui_language: Option<String>,
    pub uo_dir: Option<PathBuf>,
    pub uo_ec_dir: Option<PathBuf>,
    pub high_seas: bool,
    file_map: HashMap<String, PathBuf>,
    ec_file_map: HashMap<String, PathBuf>,
}

#[derive(Clone, Debug, Default)]
pub struct FileDetectionStatus {
    pub classic_art: Option<PathBuf>,
    pub tiledata: Option<PathBuf>,
    pub gumps: Option<PathBuf>,
    pub animations: Option<PathBuf>,
    pub multis: Option<PathBuf>,
    pub hues: Option<PathBuf>,
    pub ec_gamedata: Option<PathBuf>,
    pub ec_tileart: Option<PathBuf>,
}

impl FileManager {
    pub fn new() -> Self {
        let mut fm = Self {
            language: "enu".to_string(),
            ui_language: None,
            uo_dir: None,
            uo_ec_dir: None,
            high_seas: true,
            file_map: HashMap::new(),
            ec_file_map: HashMap::new(),
        };

        fm.load_settings();
        // UOFIDDLER_UO_DIR=<folder> points at a client folder for this run only (tests, captures).
        if let Some(dir) = std::env::var_os("UOFIDDLER_UO_DIR") {
            fm.uo_dir = Some(PathBuf::from(dir));
        }

        if fm.uo_dir.is_none() {
            fm.uo_dir = Self::auto_detect_uo_dir();
        }
        if fm.uo_ec_dir.is_none() {
            fm.uo_ec_dir = Self::auto_detect_ec_dir();
        }

        fm.rebuild_cache();
        fm
    }

    pub fn set_uo_dir<P: AsRef<Path>>(&mut self, dir: P) {
        self.uo_dir = Some(dir.as_ref().to_path_buf());
        self.rebuild_cache();
        self.save_settings();
    }

    pub fn set_uo_ec_dir<P: AsRef<Path>>(&mut self, dir: P) {
        self.uo_ec_dir = Some(dir.as_ref().to_path_buf());
        self.rebuild_cache();
        self.save_settings();
    }

    pub fn rebuild_cache(&mut self) {
        self.file_map.clear();
        self.ec_file_map.clear();

        // 1. Classic UO directory
        if let Some(dir) = &self.uo_dir {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            self.file_map.insert(name.to_lowercase(), path);
                        }
                    }
                }
            }
        }

        // 2. Enhanced UO directory
        if let Some(ec_dir) = &self.uo_ec_dir {
            if let Ok(entries) = fs::read_dir(ec_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            self.ec_file_map.insert(name.to_lowercase(), path);
                        }
                    }
                }
            }
        }
    }

    pub fn get_file_path(&self, filename: &str) -> Option<PathBuf> {
        let lower = filename.to_lowercase();
        if let Some(p) = self.file_map.get(&lower) {
            if p.is_file() {
                return Some(p.clone());
            }
        }
        // Fallback: check EC folder too
        if let Some(p) = self.ec_file_map.get(&lower) {
            if p.is_file() {
                return Some(p.clone());
            }
        }
        None
    }

    /// All files found in the classic and enhanced directories: (display name, path, is_enhanced).
    pub fn list_files(&self) -> Vec<(String, PathBuf, bool)> {
        let mut v: Vec<(String, PathBuf, bool)> = Vec::new();
        for (map, ec) in [(&self.file_map, false), (&self.ec_file_map, true)] {
            for p in map.values() {
                if let Some(n) = p.file_name().and_then(|n| n.to_str()) {
                    v.push((n.to_string(), p.clone(), ec));
                }
            }
        }
        v.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        v
    }

    /// Languages that have a `Cliloc.<suffix>` file in the client folders.
    pub fn available_languages(&self) -> Vec<String> {
        let codes: Vec<String> = self
            .list_files()
            .into_iter()
            .filter_map(|(name, _, _)| {
                let lower = name.to_lowercase();
                lower.strip_prefix("cliloc.").map(|e| e.to_string())
            })
            .collect();
        sort_languages(codes)
    }

    /// Path of a localized file in the chosen language (`base` without extension), falling back to
    /// English when that language has no such file.
    pub fn language_file(&self, base: &str) -> Option<PathBuf> {
        self.get_file_path(&format!("{}.{}", base, self.language))
            .or_else(|| self.get_file_path(&format!("{}.enu", base)))
    }

    pub fn get_ec_file_path(&self, filename: &str) -> Option<PathBuf> {
        self.ec_file_map.get(&filename.to_lowercase()).cloned()
    }

    pub fn get_detection_status(&self) -> FileDetectionStatus {
        FileDetectionStatus {
            classic_art: self
                .get_file_path("artlegacymul.uop")
                .or_else(|| self.get_file_path("art.mul")),
            tiledata: self.get_file_path("tiledata.mul"),
            gumps: self
                .get_file_path("gumpartlegacymul.uop")
                .or_else(|| self.get_file_path("gumpart.mul")),
            animations: self
                .get_file_path("animationframe1.uop")
                .or_else(|| self.get_file_path("anim.mul")),
            multis: self
                .get_file_path("multicollection.uop")
                .or_else(|| self.get_file_path("multi.mul")),
            hues: self.get_file_path("hues.mul"),
            ec_gamedata: self.get_ec_file_path("gamedata.uop"),
            ec_tileart: self.get_ec_file_path("tileart.uop"),
        }
    }

    pub fn load_settings(&mut self) {
        let home = std::env::var("HOME").unwrap_or_default();
        let paths = [
            PathBuf::from(&home).join(".config/uofiddler/settings.json"),
            PathBuf::from(&home).join(".config/uofiddler_rs/settings.json"),
            PathBuf::from(&home).join(".config/uofiddler_py/settings.json"),
        ];

        for path in &paths {
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(dir) = val.get("uo_dir").and_then(|v| v.as_str()) {
                            let p = PathBuf::from(dir);
                            if p.is_dir() {
                                self.uo_dir = Some(p);
                            }
                        }
                        if let Some(ec_dir) = val.get("uo_ec_dir").and_then(|v| v.as_str()) {
                            let p = PathBuf::from(ec_dir);
                            if p.is_dir() {
                                self.uo_ec_dir = Some(p);
                            }
                        }
                        if let Some(hs) = val.get("high_seas").and_then(|v| v.as_bool()) {
                            self.high_seas = hs;
                        }
                        if let Some(lang) = val.get("language").and_then(|v| v.as_str()) {
                            self.language = lang.to_lowercase();
                        }
                        if let Some(lang) = val.get("ui_language").and_then(|v| v.as_str()) {
                            self.ui_language = Some(lang.to_string());
                        }
                        return;
                    }
                }
            }
        }
    }

    pub fn save_settings(&self) {
        let home = std::env::var("HOME").unwrap_or_default();
        let cfg_dir = PathBuf::from(&home).join(".config/uofiddler");
        let _ = fs::create_dir_all(&cfg_dir);
        let cfg_file = cfg_dir.join("settings.json");

        let val = serde_json::json!({
            "uo_dir": self.uo_dir.as_ref().map(|p| p.display().to_string()),
            "uo_ec_dir": self.uo_ec_dir.as_ref().map(|p| p.display().to_string()),
            "high_seas": self.high_seas,
            "language": self.language,
            "ui_language": self.ui_language,
        });

        if let Ok(str_val) = serde_json::to_string_pretty(&val) {
            let _ = fs::write(cfg_file, str_val);
        }
    }

    pub fn auto_detect_uo_dir() -> Option<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        let candidates = [
            "/opt/hd_jogos/UOC",
            "/opt/hd_jogos/UOML",
            &format!("{}/Documentos/Mysterious Age", home),
            &format!("{}/Documents/Mysterious Age", home),
            &format!(
                "{}/.wine/drive_c/Program Files (x86)/Electronic Arts/Ultima Online Classic",
                home
            ),
            &format!(
                "{}/.wine/drive_c/Program Files/Electronic Arts/Ultima Online Classic",
                home
            ),
            &format!("{}/.wine/drive_c/UO", home),
            &format!("{}/Ultima Online", home),
            &format!("{}/uo", home),
            "./Ultima Online",
            "./UO",
        ];

        for path_str in candidates {
            let p = Path::new(&path_str);
            if p.is_dir() {
                for test_file in ["artidx.mul", "art.mul", "tiledata.mul", "MainData.uop"] {
                    if let Ok(entries) = fs::read_dir(p) {
                        for e in entries.flatten() {
                            if let Some(name) = e.file_name().to_str() {
                                if name.eq_ignore_ascii_case(test_file) {
                                    return Some(p.to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn auto_detect_ec_dir() -> Option<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_default();
        let candidates = [
            "/opt/hd_jogos/UOEC",
            &format!(
                "{}/.wine/drive_c/Program Files (x86)/Electronic Arts/Ultima Online Enhanced",
                home
            ),
            &format!(
                "{}/.wine/drive_c/Program Files/Electronic Arts/Ultima Online Enhanced",
                home
            ),
            &format!("{}/UOEC", home),
            &format!("{}/Ultima Online Enhanced", home),
        ];

        for path_str in candidates {
            let p = Path::new(&path_str);
            if p.is_dir() {
                for test_file in [
                    "UOSA.exe",
                    "GameData.uop",
                    "Interface.uop",
                    "gumpartLegacyMUL.uop",
                ] {
                    if let Ok(entries) = fs::read_dir(p) {
                        for e in entries.flatten() {
                            if let Some(name) = e.file_name().to_str() {
                                if name.eq_ignore_ascii_case(test_file) {
                                    return Some(p.to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod language_tests {
    use super::*;

    #[test]
    fn languages_are_ordered_and_named() {
        let v = sort_languages(vec!["KOR".into(), "enu".into(), "xyz".into(), "deu".into(), "enu".into()]);
        assert_eq!(v, vec!["enu", "deu", "kor", "xyz"]);
        assert_eq!(language_name("DEU"), "Alemão (deu)");
        assert_eq!(language_name("xx"), "xx");
    }
}
