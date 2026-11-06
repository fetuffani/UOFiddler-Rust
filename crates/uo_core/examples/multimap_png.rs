use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let out = std::path::Path::new("/out");
    for (name, h32) in [("Multimap.rle", true), ("facet00.mul", false), ("facet01.mul", false), ("facet02.mul", false), ("facet03.mul", false), ("facet04.mul", false), ("facet05.mul", false)] {
        let d = std::fs::read(std::path::Path::new(&dir).join(name)).unwrap();
        match decode_multimap(&d, h32) {
            Some(m) => {
                println!("{}: {}x{} decoded_pixels={} expected={}", name, m.width, m.height, m.decoded_pixels, m.width as usize * m.height as usize);
                image::RgbaImage::from_raw(m.width, m.height, m.rgba).unwrap().save(out.join(format!("{}.png", name))).unwrap();
            }
            None => println!("{}: failed", name),
        }
    }
}
