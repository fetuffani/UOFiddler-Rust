//! Fonts: the egui defaults + the icon font, plus a CJK fallback taken from the system on demand.
//!
//! egui's built-in fonts have no Chinese / Japanese / Korean glyphs, and the client files do (the
//! `speech.mul` keywords, the localized Cliloc tables, ...), so those texts showed as boxes. Bundling
//! a CJK font would add many MB to the executable, so the first screen that needs one loads a font that
//! the operating system already has.

use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};

/// Font files that cover CJK, in order of preference (the first ones found are used, up to
/// `MAX_FALLBACKS`). `.ttc` collections are read at index 0.
const CJK_CANDIDATES: &[&str] = &[
    // Windows
    "C:\\Windows\\Fonts\\msyh.ttc",
    "C:\\Windows\\Fonts\\msjh.ttc",
    "C:\\Windows\\Fonts\\YuGothR.ttc",
    "C:\\Windows\\Fonts\\meiryo.ttc",
    "C:\\Windows\\Fonts\\malgun.ttf",
    "C:\\Windows\\Fonts\\simsun.ttc",
    // macOS
    "/System/Library/Fonts/PingFang.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
    "/System/Library/Fonts/AppleSDGothicNeo.ttc",
    "/Library/Fonts/Arial Unicode.ttf",
    // Linux
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/google-noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
    "/usr/share/fonts/wenquanyi/wqy-microhei/wqy-microhei.ttc",
    "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
    "/usr/share/fonts/truetype/arphic/uming.ttc",
];

/// One system font is enough for most installs; a second covers the ones whose first font lacks
/// a script (e.g. Windows' YaHei has no kana).
const MAX_FALLBACKS: usize = 2;

static CJK_TRIED: AtomicBool = AtomicBool::new(false);

/// Adds a system CJK font as the last fallback of the proportional and monospace families, once.
/// Cheap after the first call. Call it from the screens that show text from the client files.
pub fn ensure_cjk(ctx: &egui::Context) {
    if CJK_TRIED.swap(true, Ordering::Relaxed) {
        return;
    }
    let mut fonts = current_definitions(ctx);
    if add_cjk_fallback(&mut fonts, CJK_CANDIDATES) > 0 {
        ctx.set_fonts(fonts);
    }
}

fn current_definitions(ctx: &egui::Context) -> egui::FontDefinitions {
    if ctx.cumulative_pass_nr() > 0 {
        ctx.fonts(|f| f.lock().fonts.definitions().clone())
    } else {
        egui::FontDefinitions::default()
    }
}

/// Reads the first `MAX_FALLBACKS` existing files of `paths` and registers them. Returns how many.
fn add_cjk_fallback(fonts: &mut egui::FontDefinitions, paths: &[&str]) -> usize {
    let mut added = 0;
    for path in paths {
        if added >= MAX_FALLBACKS {
            break;
        }
        let Ok(bytes) = std::fs::read(path) else { continue };
        let name = format!("cjk-fallback-{added}");
        fonts.font_data.insert(name.clone(), egui::FontData::from_owned(bytes));
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push(name.clone());
        }
        added += 1;
    }
    added
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{parse_table, LANGUAGES};

    /// Context with the same fonts the app starts with (egui defaults + icon font).
    fn app_context() -> egui::Context {
        let ctx = egui::Context::default();
        let mut fonts = egui::FontDefinitions::default();
        crate::map::ui::add_fonts(&mut fonts);
        ctx.set_fonts(fonts);
        // fonts are built on the first pass
        let _ = ctx.run(egui::RawInput::default(), |_| {});
        ctx
    }

    #[test]
    fn every_interface_text_can_be_drawn_with_the_app_fonts() {
        let ctx = app_context();
        let mut missing = Vec::new();
        for (code, text) in LANGUAGES.iter() {
            for (key, value) in parse_table(text) {
                for c in value.chars() {
                    let ok = ctx.fonts(|f| f.has_glyph(&egui::FontId::proportional(14.0), c));
                    if !ok && !c.is_whitespace() {
                        missing.push(format!("{code} {key}: U+{:04X} {c}", c as u32));
                    }
                }
            }
        }
        assert!(missing.is_empty(), "characters without a glyph:\n{}", missing.join("\n"));
    }

    #[test]
    fn missing_font_files_are_skipped() {
        let mut fonts = egui::FontDefinitions::default();
        let before = fonts.font_data.len();
        assert_eq!(add_cjk_fallback(&mut fonts, &["/nonexistent/a.ttc", "/nonexistent/b.ttf"]), 0);
        assert_eq!(fonts.font_data.len(), before);
    }
}
