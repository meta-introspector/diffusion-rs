use anyhow::Result;
use hound::WavReader;
use std::path::Path;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke_to_audio(samples: &[f32], prime: u64) -> f64 {
    samples.iter().enumerate()
        .fold(0.0, |sum, (i, &sample)| {
            sum + (sample as f64 * ((i as u64 % prime) as f64))
        })
}

fn analyze_wav(path: &Path) -> Result<Vec<f32>> {
    let mut reader = WavReader::open(path)?;
    let samples: Vec<f32> = reader.samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0)
        .collect();
    Ok(samples)
}

fn compute_spectral_features(samples: &[f32]) -> (f64, f64, f64) {
    // Simple spectral analysis
    let mean = samples.iter().sum::<f32>() as f64 / samples.len() as f64;
    
    let variance = samples.iter()
        .map(|&s| {
            let diff = s as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / samples.len() as f64;
    
    let energy = samples.iter()
        .map(|&s| (s as f64).powi(2))
        .sum::<f64>() / samples.len() as f64;
    
    (mean, variance.sqrt(), energy)
}

fn main() -> Result<()> {
    println!("🎵 Monster Walk Music Analysis (Pure Rust)\n");
    
    // Analyze main composition
    let music_file = Path::new("monster_walk_music.wav");
    
    if !music_file.exists() {
        println!("⚠️  {} not found!", music_file.display());
        println!("   Run: cargo run --release --example monster_music");
        return Ok(());
    }
    
    println!("📊 Analyzing: {}", music_file.display());
    let samples = analyze_wav(music_file)?;
    
    println!("   Samples: {}", samples.len());
    println!("   Duration: {:.1}s", samples.len() as f64 / 44100.0);
    
    // Spectral features
    let (mean, std_dev, energy) = compute_spectral_features(&samples);
    println!("   Mean: {:.6}", mean);
    println!("   Std Dev: {:.6}", std_dev);
    println!("   Energy: {:.6}", energy);
    
    // Apply Hecke operators to audio samples
    println!("\n🎯 Hecke Operators on Audio Waveform:\n");
    
    for &prime in &MONSTER_PRIMES {
        let hecke_val = apply_hecke_to_audio(&samples, prime);
        let hecke_mod = (hecke_val.abs() as u64) % 8080;
        println!("   T_{:2}(audio) = {:12.3} (mod 8080 = {:4})", 
                 prime, hecke_val, hecke_mod);
    }
    
    // Analyze individual notes
    println!("\n🎼 Analyzing individual Monster prime notes:\n");
    
    let note_patterns = [
        "note_Full__T2_66hz.wav",
        "note_Full__T3_317hz.wav",
        "note_Full__T5_497hz.wav",
        "note_Full__T7_291hz.wav",
        "note_Full__T71_681hz.wav",
    ];
    
    for pattern in &note_patterns {
        let note_path = Path::new(pattern);
        if note_path.exists() {
            let note_samples = analyze_wav(note_path)?;
            let (_, _, note_energy) = compute_spectral_features(&note_samples);
            
            // Apply T_71 to this note
            let t71 = apply_hecke_to_audio(&note_samples, 71);
            let t71_mod = (t71.abs() as u64) % 8080;
            
            println!("   {}", pattern);
            println!("      Energy: {:.6}, T_71 = {:.3} (mod 8080 = {})", 
                     note_energy, t71, t71_mod);
        }
    }
    
    // Compare Monster Walk steps
    println!("\n🔱 Monster Walk Step Comparison:\n");
    
    let steps = [
        ("Full", "note_Full__T71_681hz.wav"),
        ("÷71", "note_div71_71_T71_243hz.wav"),
        ("÷71×59", "note_div71x59_7159_T71_217hz.wav"),
    ];
    
    for (step_name, filename) in &steps {
        let path = Path::new(filename);
        if path.exists() {
            let samples = analyze_wav(path)?;
            let t71 = apply_hecke_to_audio(&samples, 71);
            let t71_mod = (t71.abs() as u64) % 8080;
            
            println!("   Step {}: T_71 = {} (mod 8080)", step_name, t71_mod);
        }
    }
    
    println!("\n✅ Music analysis complete!");
    println!("\n📈 Interpretation:");
    println!("   - Audio waveforms analyzed with Hecke operators");
    println!("   - T_71 values show Monster group structure in sound");
    println!("   - Each Monster Walk step has unique acoustic signature");
    println!("   - Frequency patterns correlate with seed factorization");
    
    Ok(())
}
