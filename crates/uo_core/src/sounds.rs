//! UO Sound effects parser and audio playback (sound.mul / soundidx.mul).

use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;

#[derive(Clone, Debug)]
pub struct SoundEntry {
    pub sound_id: u32,
    pub name: String,
    pub pcm_data: Vec<u8>,
}

impl SoundEntry {
    pub fn parse(sound_id: u32, data: &[u8]) -> Option<Self> {
        if data.len() <= 40 {
            return None;
        }

        let raw_name = &data[..40];
        let null_pos = raw_name.iter().position(|&b| b == 0).unwrap_or(40);
        let name = String::from_utf8_lossy(&raw_name[..null_pos])
            .trim()
            .to_string();
        let pcm_data = data[40..].to_vec();

        Some(Self {
            sound_id,
            name,
            pcm_data,
        })
    }

    /// Creates standard 44-byte RIFF WAVE header for 22050 Hz, 16-bit Mono PCM.
    pub fn to_wav(&self) -> Vec<u8> {
        let pcm_len = self.pcm_data.len() as u32;
        let mut wav = Vec::with_capacity((pcm_len + 44) as usize);

        // RIFF header
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(pcm_len + 36).to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        // fmt chunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // subchunk1 size
        wav.extend_from_slice(&1u16.to_le_bytes()); // PCM format
        wav.extend_from_slice(&1u16.to_le_bytes()); // Mono (1 channel)
        wav.extend_from_slice(&22050u32.to_le_bytes()); // Sample rate
        wav.extend_from_slice(&44100u32.to_le_bytes()); // Byte rate (22050 * 1 * 2)
        wav.extend_from_slice(&2u16.to_le_bytes()); // Block align
        wav.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

        // data chunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&pcm_len.to_le_bytes());
        wav.extend_from_slice(&self.pcm_data);

        wav
    }

    /// Plays this sound using available system audio player (pw-play, paplay, or aplay) in background.
    pub fn play(&self) {
        let wav = self.to_wav();
        thread::spawn(move || {
            let tmp_path =
                std::env::temp_dir().join(format!("uo_sound_{}.wav", std::process::id()));
            if let Ok(mut f) = File::create(&tmp_path) {
                let _ = f.write_all(&wav);
                drop(f);

                for player in ["pw-play", "paplay", "aplay"] {
                    if let Ok(mut child) = Command::new(player).arg(&tmp_path).spawn() {
                        let _ = child.wait();
                        break;
                    }
                }
                let _ = std::fs::remove_file(tmp_path);
            }
        });
    }
}
