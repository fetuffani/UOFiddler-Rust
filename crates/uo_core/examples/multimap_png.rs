use uo_core::*;
fn main() {
    let dir = std::env::var("UO_DIR").unwrap_or_else(|_| "/uo".into());
    let out = std::path::Path::new("/out");
    let d = std::fs::read(std::path::Path::new(&dir).join("Multimap.rle")).unwrap();
    let m = decode_multimap(&d, true).unwrap();
    println!("Multimap.rle {}x{} ok", m.width, m.height);
    for i in 0..6 {
        let name = format!("facet0{}.mul", i);
        let d = std::fs::read(std::path::Path::new(&dir).join(&name)).unwrap();
        match decode_facet(&d) {
            Some(m) => {
                println!("{}: {}x{} strict decode OK", name, m.width, m.height);
                // quarter-size preview for viewing
                let (pw, ph) = (m.width / 4, m.height / 4);
                let mut img = image::RgbaImage::new(pw, ph);
                for y in 0..ph { for x in 0..pw {
                    let o = (((y * 4) * m.width + x * 4) * 4) as usize;
                    img.put_pixel(x, y, image::Rgba([m.rgba[o], m.rgba[o + 1], m.rgba[o + 2], 255]));
                }}
                img.save(out.join(format!("p_{}.png", name))).unwrap();
            }
            None => println!("{}: strict decode FAILED", name),
        }
    }
}
