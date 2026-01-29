use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

// Monster Walk Hecke frequencies from analysis
const MONSTER_WALK_STEPS: &[(&str, i64, u64, &[u64])] = &[
    ("Full", 2437596016, 778, &[110, 2568, 4331, 2315, 7677, 6534, 3405, 3754, 3988, 5457, 2032, 4420, 1861, 6595, 6134]),
    ("÷71", 34332338, 250, &[4577, 3671, 3789, 5368, 1752, 7686, 3249, 7357, 3994, 3665, 7317, 6717, 6762, 518, 1843]),
    ("÷71×59", 581904, 241, &[1832, 393, 2126, 6373, 2352, 5341, 5925, 1295, 3608, 7028, 7380, 7807, 4355, 1150, 1589]),
    ("÷71×59×47", 12380, 48, &[7583, 4626, 7446, 2005, 984, 4106, 6171, 614, 2903, 7103, 1882, 1468, 4442, 2833, 3357]),
    ("÷4primes", 301, 1, &[5732, 6491, 2338, 62, 3963, 1588, 7473, 993, 7320, 645, 4571, 4935, 5075, 6567, 7639]),
];

const SAMPLE_RATE: u32 = 44100;
const MONSTER_PRIMES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn hecke_to_hz(hecke_val: u64, min_hz: f32, max_hz: f32) -> f32 {
    let normalized = hecke_val as f32 / 8080.0;
    min_hz + (max_hz - min_hz) * normalized
}

fn generate_tone(frequency: f32, duration: f32, amplitude: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            amplitude * (2.0 * PI * frequency * t).sin()
        })
        .collect()
}

fn generate_chord(frequencies: &[f32], duration: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut wave = vec![0.0; num_samples];
    
    for &freq in frequencies {
        let tone = generate_tone(freq, duration, 0.2);
        for (i, &sample) in tone.iter().enumerate() {
            wave[i] += sample;
        }
    }
    
    // Normalize
    let max_val = wave.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
    if max_val > 0.0 {
        for sample in &mut wave {
            *sample = (*sample / max_val) * 0.8;
        }
    }
    
    wave
}

fn write_wav(filename: &str, samples: &[f32]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    let num_samples = samples.len() as u32;
    let byte_rate = SAMPLE_RATE * 2; // 16-bit mono
    let data_size = num_samples * 2;
    
    // WAV header
    file.write_all(b"RIFF")?;
    file.write_all(&(36 + data_size).to_le_bytes())?;
    file.write_all(b"WAVE")?;
    
    // fmt chunk
    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?; // chunk size
    file.write_all(&1u16.to_le_bytes())?;  // PCM
    file.write_all(&1u16.to_le_bytes())?;  // mono
    file.write_all(&SAMPLE_RATE.to_le_bytes())?;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&2u16.to_le_bytes())?;  // block align
    file.write_all(&16u16.to_le_bytes())?; // bits per sample
    
    // data chunk
    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;
    
    // Write samples as 16-bit PCM
    for &sample in samples {
        let sample_i16 = (sample * 32767.0) as i16;
        file.write_all(&sample_i16.to_le_bytes())?;
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("🎵 Monster Walk Music Generator\n");
    
    let mut full_audio = Vec::new();
    
    for (step_name, seed, t71, frequencies) in MONSTER_WALK_STEPS {
        println!("Step: {}", step_name);
        println!("  Seed: {}, T_71: {}", seed, t71);
        
        // Convert Hecke frequencies to Hz (A1 to A5 range)
        let hz_freqs: Vec<f32> = frequencies.iter()
            .map(|&f| hecke_to_hz(f, 55.0, 880.0))
            .collect();
        
        println!("  Frequencies: {:.1}Hz, {:.1}Hz, {:.1}Hz...", 
                 hz_freqs[0], hz_freqs[1], hz_freqs[2]);
        
        // Generate chord (2 seconds)
        let chord = generate_chord(&hz_freqs, 2.0);
        full_audio.extend_from_slice(&chord);
        
        // Add silence (0.5 seconds)
        let silence = vec![0.0; (SAMPLE_RATE as f32 * 0.5) as usize];
        full_audio.extend_from_slice(&silence);
        
        println!();
    }
    
    // Write full composition
    let output_file = "monster_walk_music.wav";
    write_wav(output_file, &full_audio)?;
    
    let duration = full_audio.len() as f32 / SAMPLE_RATE as f32;
    println!("✅ Generated: {}", output_file);
    println!("   Duration: {:.1} seconds", duration);
    println!("   Sample rate: {} Hz", SAMPLE_RATE);
    
    // Generate individual notes for each prime
    println!("\n🎼 Generating individual Monster prime notes\n");
    
    for (step_name, _seed, _t71, frequencies) in MONSTER_WALK_STEPS {
        for (i, (&prime, &hecke_val)) in MONSTER_PRIMES.iter().zip(frequencies.iter()).enumerate() {
            let hz = hecke_to_hz(hecke_val, 55.0, 880.0);
            let tone = generate_tone(hz, 1.0, 0.3);
            
            let filename = format!("note_{}_{}_T{}_{}hz.wav", 
                                   step_name.replace("÷", "div").replace("×", "x"),
                                   step_name.chars().filter(|c| c.is_numeric()).collect::<String>(),
                                   prime, hz as u32);
            write_wav(&filename, &tone)?;
            
            if i == 0 {
                println!("  {}: T_{} = {} → {:.1} Hz", step_name, prime, hecke_val, hz);
            }
        }
    }
    
    println!("\n🎹 To play:");
    println!("   ffplay {}", output_file);
    println!("\n✅ Monster Walk music generation complete!");
    
    Ok(())
}
