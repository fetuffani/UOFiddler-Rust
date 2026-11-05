use uo_core::*;

#[test]
fn test_debug_all() {
    let fm = FileManager::new();
    println!("UO DIR: {:?}", fm.uo_dir);

    // 1. Gumps
    if let Some(uop) = fm.get_file_path("gumpartlegacymul.uop") {
        println!("Found gumpartlegacymul.uop: {:?}", uop);
        let fi = FileIndex::from_uop(&uop, ".tga", 0xFFFF).expect("uop open failed");
        let valid_count = (0..0xFFFF).filter(|&i| fi.is_valid(i)).count();
        println!("Valid gumps in UOP: {}", valid_count);
        for id in [0, 1, 10, 100, 1000] {
            if fi.is_valid(id) {
                let data = fi.get_data(id).unwrap();
                let entry = fi.entries[id];
                println!("Gump {} len: {}, entry: {:?}", id, data.len(), entry);
                let (w, h, payload) = if fi.is_uop() || entry.compression == 1 {
                    if data.len() >= 8 {
                        let w = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                        let h = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                        (w, h, &data[8..])
                    } else {
                        (0, 0, &data[..])
                    }
                } else {
                    let w = ((entry.extra >> 16) & 0xFFFF) as u32;
                    let h = (entry.extra & 0xFFFF) as u32;
                    (w, h, &data[..])
                };
                println!(
                    "Gump {} w={}, h={}, payload_len={}",
                    id,
                    w,
                    h,
                    payload.len()
                );
                let bmp = decode_gump(payload, w, h);
                println!(
                    "Decoded gump {}: {:?}",
                    id,
                    bmp.as_ref().map(|b| (b.width, b.height))
                );
            }
        }
    } else {
        println!("gumpartlegacymul.uop NOT found!");
    }

    // 2. Textures
    if let (Some(idx), Some(mul)) = (
        fm.get_file_path("texidx.mul"),
        fm.get_file_path("texmaps.mul"),
    ) {
        println!("Found texidx.mul and texmaps.mul");
        let fi = FileIndex::from_mul_idx(&idx, &mul, 0x4000).expect("tex open failed");
        let valid_count = (0..0x4000).filter(|&i| fi.is_valid(i)).count();
        println!("Valid textures: {}", valid_count);
        for id in 0..10 {
            if fi.is_valid(id) {
                let data = fi.get_data(id).unwrap();
                let entry = fi.entries[id];
                let bmp = decode_texture(&data, entry.extra);
                println!("Decoded tex {}: {:?}", id, bmp.as_ref().map(|b| b.size));
            }
        }
    }

    // 3. RadarCol
    if let Some(rc_path) = fm.get_file_path("radarcol.mul") {
        println!("Found radarcol.mul: {:?}", rc_path);
        let rc = RadarColManager::from_file(&rc_path).expect("radarcol open failed");
        println!("RadarCol total colors: {}", rc.colors.len());
    } else {
        println!("radarcol.mul NOT found!");
    }

    // 5. Sounds
    if let (Some(idx), Some(mul)) = (
        fm.get_file_path("soundidx.mul"),
        fm.get_file_path("sound.mul"),
    ) {
        println!("Found soundidx.mul and sound.mul");
        let fi = FileIndex::from_mul_idx(&idx, &mul, 0xFFF).expect("sound open failed");
        let mut count = 0;
        for i in 0..0xFFF {
            if fi.is_valid(i) {
                if let Some(data) = fi.get_data(i) {
                    if let Some(snd) = SoundEntry::parse(i as u32, &data) {
                        if count < 5 {
                            println!("Sound 0x{:03X}: {:?}", i, snd.name);
                        }
                        count += 1;
                    }
                }
            }
        }
        println!("Total parsed sounds: {}", count);
    }

    // 6. Fonts
    if let Some(fp) = fm.get_file_path("fonts.mul") {
        println!("Found fonts.mul: {:?}", fp);
        let f_mgr = FontsManager::from_file(&fp).expect("fonts open failed");
        let valid_f = f_mgr.ascii_fonts.iter().filter(|f| f.is_some()).count();
        println!("Valid ASCII fonts: {}", valid_f);
        if let Some(f0) = &f_mgr.ascii_fonts[0] {
            let rendered = f0.render_text("Test UOFiddler");
            println!(
                "Font 0 render 'Test UOFiddler': {:?}",
                rendered.as_ref().map(|(w, h, _)| (*w, *h))
            );
        }
    }

    // 7. Speech
    if let Some(sp) = fm.get_file_path("speech.mul") {
        println!("Found speech.mul: {:?}", sp);
        let sm = SpeechManager::from_file(&sp).expect("speech open failed");
        println!("Speech entries: {}", sm.entries.len());
        for s in sm.entries.iter().take(3) {
            println!("Speech #{}: {:?}", s.entry_id, s.keyword);
        }
    }

    // 8. Animdata
    if let Some(ap) = fm.get_file_path("animdata.mul") {
        println!("Found animdata.mul: {:?}", ap);
        let adm = AnimDataManager::from_file(&ap).expect("animdata open failed");
        println!("AnimData entries: {}", adm.entries.len());
    }

    // 9. Maps
    let mut mm = MapManager::new();
    mm.load_map(
        0,
        fm.get_file_path("map0.mul"),
        fm.get_file_path("staidx0.mul"),
        fm.get_file_path("statics0.mul"),
    );
    if let Some(rc_path) = fm.get_file_path("radarcol.mul") {
        let rc = RadarColManager::from_file(&rc_path).unwrap();
        let mini = mm.render_minimap(0, 175, 210, 32, 32, &rc, true);
        println!(
            "Map 0 Britain minimap (32x32 blocks = 256x256 px): {:?}",
            mini.as_ref().map(|(w, h, _)| (*w, *h))
        );
    }

    // 10. Multis (Classic & Enhanced UOP)
    if let (Some(idx), Some(mul)) = (fm.get_file_path("multi.idx"), fm.get_file_path("multi.mul")) {
        let fi = FileIndex::from_mul_idx(&idx, &mul, 0x4000).expect("multi open failed");
        let valid_classic = (0..0x4000).filter(|&i| fi.is_valid(i)).count();
        println!("Valid classic multis: {}", valid_classic);
        assert!(valid_classic > 0, "Classic multis should have entries");
        if let Some(data) = fi.get_data(0) {
            let m = MultiStructure::parse(0, &data).expect("parse multi 0 failed");
            println!("Multi 0x0000 components: {}", m.components.len());
            assert!(!m.components.is_empty());
        }
    }

    if let Some(uop) = fm.get_file_path("MultiCollection.uop") {
        let uop_multis = load_uop_multis(&uop);
        println!("UOP Enhanced multis parsed: {}", uop_multis.len());
        assert!(!uop_multis.is_empty(), "UOP multis should not be empty");
        if let Some(m) = uop_multis.get(&0) {
            println!("Enhanced multi 0x0000 components: {}", m.components.len());
        }
    }

    let multilist = load_multilist_entries(&fm);
    println!("Loaded multilist entries from XML: {}", multilist.len());
    assert!(
        !multilist.is_empty(),
        "Multilist.xml entries should not be empty"
    );

    // 11. UOP Animations Indexing & Decoding
    let mut uop_anim = UopAnimationIndex::new();
    if let Some(dir) = &fm.uo_dir {
        uop_anim.load_directory(dir);
    }
    if let Some(ec_dir) = &fm.uo_ec_dir {
        uop_anim.load_directory(ec_dir);
    }
    println!("UOP Animation entries loaded: {}", uop_anim.entries.len());
    assert!(
        !uop_anim.entries.is_empty(),
        "UOP animations should have entries"
    );

    // Test body 130 (Shadow Fiend / UOP creature) act 0 (Walk)
    let frames = uop_anim.get_animation_frames(130, 0, 0);
    println!(
        "UOP body 130 act 0 frames decoded: {:?}",
        frames.as_ref().map(|f| f.len())
    );
    assert!(frames.is_some() && !frames.unwrap().is_empty());

    // 12. EC Art (LegacyTexture.uop / tileart.uop)
    let mut ec_art = EcArtIndex::new();
    if let Some(ec_dir) = &fm.uo_ec_dir {
        ec_art.load_directory(ec_dir);
    }
    println!("EC Art entries loaded: {}", ec_art.entries.len());
    if !ec_art.entries.is_empty() {
        let bmp = ec_art.get_static(2);
        println!(
            "EC static 2 decoded: {:?}",
            bmp.as_ref().map(|b| (b.width, b.height))
        );
    }

    // 13. GameData.uop (Enhanced Client 3D tables)
    let mut gamedata = GameDataArchive::new();
    if let Some(ec_dir) = &fm.uo_ec_dir {
        gamedata.load_directory(ec_dir);
    }
    println!("GameData entries loaded: {}", gamedata.entries.len());
    println!("GameData containers parsed: {}", gamedata.containers.len());
    println!(
        "GameData legacy-to-new parsed: {}",
        gamedata.legacy_to_new.len()
    );
    assert!(
        !gamedata.entries.is_empty(),
        "GameData entries should not be empty"
    );
}
