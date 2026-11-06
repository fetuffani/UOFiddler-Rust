//! Interface localization: embedded `key = value` tables, English fallback, `{name}` parameters.
//!
//! Every user-visible text goes through `t!("key")` / `t!("key", n = 3)`. Add the key to **every**
//! file in `locales/` (a test fails when a language has missing or extra keys). Languages are added
//! by dropping a new `locales/<code>.txt` and listing it in `LANGUAGES`.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// (code, table). `en` is the default and the fallback.
pub const LANGUAGES: [(&str, &str); 2] = [
    ("en", include_str!("../locales/en.txt")),
    ("pt-BR", include_str!("../locales/pt-BR.txt")),
];

pub const DEFAULT_LANGUAGE: &str = "en";

static CURRENT: RwLock<String> = RwLock::new(String::new());

/// Parses a table; returns the entries in file order (duplicates kept, so tests can detect them).
pub fn parse_table(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                return None;
            }
            let (k, v) = l.split_once('=')?;
            Some((k.trim().to_string(), v.trim().replace("\\n", "\n")))
        })
        .collect()
}

fn tables() -> &'static HashMap<&'static str, HashMap<String, String>> {
    static T: OnceLock<HashMap<&'static str, HashMap<String, String>>> = OnceLock::new();
    T.get_or_init(|| LANGUAGES.iter().map(|(code, text)| (*code, parse_table(text).into_iter().collect())).collect())
}

/// Looks `key` up in `lang`, then in English; `None` when neither has it.
pub fn lookup(lang: &str, key: &str) -> Option<String> {
    let t = tables();
    t.get(lang).and_then(|m| m.get(key)).or_else(|| t.get(DEFAULT_LANGUAGE).and_then(|m| m.get(key))).cloned()
}

pub fn is_known(code: &str) -> bool {
    LANGUAGES.iter().any(|(c, _)| *c == code)
}

/// Sets the interface language; returns false (and changes nothing) for an unknown code.
pub fn set_language(code: &str) -> bool {
    if !is_known(code) {
        return false;
    }
    *CURRENT.write().unwrap() = code.to_string();
    true
}

pub fn language() -> String {
    let c = CURRENT.read().unwrap();
    if c.is_empty() { DEFAULT_LANGUAGE.to_string() } else { c.clone() }
}

/// (code, native name) of every available language.
pub fn available() -> Vec<(&'static str, String)> {
    LANGUAGES.iter().map(|(c, _)| (*c, lookup(c, "language.name").unwrap_or_else(|| c.to_string()))).collect()
}

/// Text for `key` in the current language (English when missing there, the key itself as a last resort).
pub fn tr(key: &str) -> String {
    lookup(&language(), key).unwrap_or_else(|| key.to_string())
}

/// Like `tr`, replacing `{name}` with the given values.
pub fn tr_args(key: &str, args: &[(&str, String)]) -> String {
    args.iter().fold(tr(key), |s, (n, v)| s.replace(&format!("{{{}}}", n), v))
}

/// Picks `<key>.one` for 1 and `<key>.other` otherwise, with `{n}` replaced.
pub fn tr_plural(key: &str, n: usize) -> String {
    let k = format!("{}.{}", key, if n == 1 { "one" } else { "other" });
    tr_args(&k, &[("n", n.to_string())])
}

/// Language for the first run: the system language when supported, English otherwise.
/// Reads LC_ALL / LC_MESSAGES / LANG (Unix); other systems fall back to English.
pub fn detect_system_language() -> &'static str {
    for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(v) = std::env::var(var) {
            if let Some(code) = match_system_locale(&v) {
                return code;
            }
            if !v.is_empty() {
                break;
            }
        }
    }
    DEFAULT_LANGUAGE
}

/// Maps a locale such as `pt_BR.UTF-8` to a supported language code.
pub fn match_system_locale(locale: &str) -> Option<&'static str> {
    let l = locale.to_lowercase().replace('_', "-");
    let l = l.split('.').next().unwrap_or("");
    LANGUAGES
        .iter()
        .map(|(c, _)| *c)
        .find(|c| c.to_lowercase() == l)
        .or_else(|| LANGUAGES.iter().map(|(c, _)| *c).find(|c| c.to_lowercase().split('-').next() == l.split('-').next() && !l.is_empty()))
}

/// Initializes from the saved choice (or the system language).
pub fn init(saved: Option<&str>) {
    let code = match saved {
        Some(c) if is_known(c) => c.to_string(),
        _ => detect_system_language().to_string(),
    };
    set_language(&code);
}

/// `t!("key")` or `t!("key", n = 3, name = "x")`.
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::tr($key)
    };
    ($key:expr, $($name:ident = $val:expr),+ $(,)?) => {
        $crate::i18n::tr_args($key, &[$((stringify!($name), $val.to_string())),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn keys(text: &str) -> BTreeSet<String> {
        parse_table(text).into_iter().map(|(k, _)| k).collect()
    }
    fn placeholders(v: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut rest = v;
        while let Some(a) = rest.find('{') {
            if let Some(b) = rest[a..].find('}') {
                out.insert(rest[a + 1..a + b].to_string());
                rest = &rest[a + b + 1..];
            } else {
                break;
            }
        }
        out
    }

    #[test]
    fn every_language_has_exactly_the_english_keys() {
        let en = keys(LANGUAGES[0].1);
        assert!(!en.is_empty());
        for (code, text) in LANGUAGES.iter() {
            let k = keys(text);
            let missing: Vec<_> = en.difference(&k).collect();
            let extra: Vec<_> = k.difference(&en).collect();
            assert!(missing.is_empty() && extra.is_empty(), "{code}: missing {missing:?}, extra {extra:?}");
        }
    }

    #[test]
    fn no_duplicate_keys_and_same_placeholders() {
        let en: HashMap<String, String> = parse_table(LANGUAGES[0].1).into_iter().collect();
        for (code, text) in LANGUAGES.iter() {
            let entries = parse_table(text);
            let unique: BTreeSet<&String> = entries.iter().map(|(k, _)| k).collect();
            assert_eq!(unique.len(), entries.len(), "{code} has duplicate keys");
            for (k, v) in &entries {
                assert_eq!(placeholders(v), placeholders(&en[k]), "{code}: placeholders of {k} differ from English");
            }
        }
    }

    #[test]
    fn lookup_falls_back_to_english_and_then_to_the_key() {
        assert_eq!(lookup("pt-BR", "top.about").unwrap(), "ℹ Sobre");
        assert_eq!(lookup("xx", "top.about").unwrap(), "ℹ About");
        assert!(lookup("pt-BR", "does.not.exist").is_none());
    }

    #[test]
    fn parameters_and_plural() {
        let s = tr_args("pill.art", &[("n", "7".to_string())]);
        assert!(s.contains('7') && !s.contains('{'));
    }

    #[test]
    fn system_locale_mapping() {
        assert_eq!(match_system_locale("pt_BR.UTF-8"), Some("pt-BR"));
        assert_eq!(match_system_locale("pt_PT.UTF-8"), Some("pt-BR"));
        assert_eq!(match_system_locale("en_US.UTF-8"), Some("en"));
        assert_eq!(match_system_locale("de_DE.UTF-8"), None);
        assert_eq!(match_system_locale(""), None);
    }

    /// Hard-coded user-visible widget text is only allowed to shrink: new text must use `t!`.
    /// Run with `UPDATE_I18N_BASELINE=1` after migrating strings to lower the allowed counts.
    #[test]
    fn hardcoded_ui_text_never_grows() {
        const PATTERNS: [&str; 6] = ["label(\"", "button(\"", "heading(\"", "on_hover_text(\"", "RichText::new(\"", "status_message = \""];
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().map_or(false, |x| x == "rs") {
                    out.push(p);
                }
            }
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        walk(&root.join("src"), &mut files);
        files.sort();
        let mut actual: Vec<(String, usize)> = Vec::new();
        for f in &files {
            let rel = f.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            if rel == "src/i18n.rs" {
                continue;
            }
            let text = std::fs::read_to_string(f).unwrap();
            let n: usize = PATTERNS.iter().map(|p| text.matches(p).count()).sum();
            if n > 0 {
                actual.push((rel, n));
            }
        }
        let baseline_path = root.join("i18n_baseline.txt");
        let allowed: HashMap<String, usize> = std::fs::read_to_string(&baseline_path)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.rsplit_once(' ').and_then(|(f, n)| Some((f.to_string(), n.parse().ok()?))))
            .collect();
        let grew: Vec<String> = actual
            .iter()
            .filter(|(f, n)| *n > allowed.get(f).copied().unwrap_or(0))
            .map(|(f, n)| format!("{f}: {n} hard-coded texts, {} allowed", allowed.get(f).copied().unwrap_or(0)))
            .collect();
        if std::env::var("UPDATE_I18N_BASELINE").is_ok() && (grew.is_empty() || !baseline_path.exists()) {
            let body: String = actual.iter().map(|(f, n)| format!("{f} {n}\n")).collect();
            std::fs::write(&baseline_path, body).unwrap();
            return;
        }
        assert!(grew.is_empty(), "New hard-coded user-visible text. Use t!(\"key\") and add the key to every file in locales/ (see CONTRIBUTING.md):\n{}", grew.join("\n"));
    }

    #[test]
    fn unknown_language_is_rejected() {
        assert!(!is_known("xx"));
        assert!(is_known("en") && is_known("pt-BR"));
    }
}
