use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let mut fm = FileManager::new();
    fm.set_uo_dir(&dir);
    let f = FontsManager::from_file(fm.get_file_path("fonts.mul").unwrap()).unwrap();
    for (i, font) in f.ascii_fonts.iter().enumerate() {
        let Some(font) = font else { println!("font {i}: none"); continue };
        let g = |c: char| font.characters[(c as usize) - 0x20].as_ref().map(|g| (g.width, g.height));
        println!("font {} header {} height {} | space {:?} 'A' {:?} 'i' {:?} 'g' {:?} '.' {:?}", i, font.header, font.height, g(' '), g('A'), g('i'), g('g'), g('.'));
    }
    for fi in [1usize, 2] {
        for top in [false, true] {
            let (w, h, rgba) = sample(f.ascii_fonts[fi].as_ref().unwrap(), "Agjpq Hello World, gypsy", top);
            image::RgbaImage::from_raw(w as u32, h as u32, rgba).unwrap().save(format!("/out/font{}_{}.png", fi, if top { "top" } else { "bottom" })).unwrap();
        }
    }
    for n in [0usize, 1, 3] {
        let name = if n == 0 { "unifont.mul".to_string() } else { format!("unifont{}.mul", n) };
        let u = UnicodeFont::from_file(fm.get_file_path(&name).unwrap()).unwrap();
        let (w, h, rgba) = u.render("Example gump, Hello World!\nSecond line: gypsy 0123", [235, 235, 235, 255], None).unwrap();
        let mut img = image::RgbaImage::new(w as u32, h as u32);
        for (i, px) in img.pixels_mut().enumerate() { let o = i * 4; *px = if rgba[o + 3] > 0 { image::Rgba([rgba[o], rgba[o+1], rgba[o+2], 255]) } else { image::Rgba([40, 40, 60, 255]) }; }
        img.save(format!("/out/uni{}.png", n)).unwrap();
        println!("unifont{} line_height {} size {}x{}", n, u.line_height, w, h);
    }
    for n in 0..13 { if let Some(p) = fm.get_file_path(&format!("unifont{}.mul", if n == 0 { String::new() } else { n.to_string() })) { println!("unifont{} {} bytes", n, std::fs::metadata(p).unwrap().len()); } }
}

#[allow(dead_code)]
pub fn sample(font: &AsciiFont, text: &str, top_aligned: bool) -> (usize, usize, Vec<u8>) {
    let glyphs: Vec<&Glyph> = text.chars().filter_map(|c| font.characters.get((c as usize).wrapping_sub(0x20)).and_then(|g| g.as_ref())).collect();
    let w: usize = glyphs.iter().map(|g| g.width).sum();
    let h = glyphs.iter().map(|g| g.height).max().unwrap_or(0);
    let mut out = vec![0u8; w * h * 4];
    for px in out.chunks_exact_mut(4) { px.copy_from_slice(&[40, 40, 60, 255]); }
    let mut x0 = 0;
    for g in glyphs {
        let yo = if top_aligned { 0 } else { h - g.height };
        for y in 0..g.height { for x in 0..g.width {
            let s = (y * g.width + x) * 4;
            if g.rgba[s + 3] > 0 { let d = ((yo + y) * w + x0 + x) * 4; out[d..d + 4].copy_from_slice(&g.rgba[s..s + 4]); }
        }}
        x0 += g.width;
    }
    (w, h, out)
}
