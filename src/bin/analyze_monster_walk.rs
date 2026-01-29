use anyhow::Result;
use image::io::Reader as ImageReader;
use std::fs;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke(data: &[u8], p: u64) -> u64 {
    data.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p)))
}

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p)))
}

fn main() -> Result<()> {
    println!("🔱 Monster Walk Analysis\n");
    
    let base_seed = 2437596016u64;
    
    // Monster Walk steps
    let steps = vec![
        (base_seed, "step_0_full", "Full"),
        (base_seed / 71, "step_1_div71", "÷71"),
        (base_seed / (71 * 59), "step_2_div71x59", "÷71×59"),
        (base_seed / (71 * 59 * 47), "step_3_div71x59x47", "÷71×59×47"),
        (base_seed / (71 * 59 * 47 * 41), "step_4_div71x59x47x41", "÷4primes"),
        (base_seed / (71 * 59 * 47 * 41 * 31), "step_5_div71x59x47x41x31", "÷5primes"),
        (base_seed / (71 * 59 * 47 * 41 * 31 * 29), "step_6_div5primes", "÷6primes"),
        (base_seed / (71 * 59 * 47 * 41 * 31 * 29 * 23), "step_7_div6primes", "÷7primes"),
        (base_seed / (71 * 59 * 47 * 41 * 31 * 29 * 23 * 19), "step_8_div7primes", "÷8primes"),
        (base_seed / (71 * 59 * 47 * 41 * 31 * 29 * 23 * 19 * 17), "step_9_div8primes", "÷9primes"),
        (base_seed / (71 * 59 * 47 * 41 * 31 * 29 * 23 * 19 * 17 * 13), "step_10_div9primes", "÷10primes"),
    ];
    
    println!("| Step | Seed | T_71(seed) | T_71(img) | Resonance |");
    println!("|------|------|------------|-----------|-----------|");
    
    for (seed, label, desc) in &steps {
        // Analyze seed
        let t71_seed = apply_hecke_to_number(*seed, 71);
        
        // Analyze first image from this step
        let img_path = format!("monster_walk_{}_0.png", label);
        
        if std::path::Path::new(&img_path).exists() {
            let raw = ImageReader::open(&img_path)?.decode()?.to_rgb8().into_raw();
            let t71_img = apply_hecke(&raw, 71);
            
            println!("| {} | {} | {} | {} | {} |",
                     desc, seed, t71_seed % 8080, t71_img % 8080,
                     if t71_img % 8080 < 1000 { "⭐" } else { "" });
        } else {
            println!("| {} | {} | {} | - | - |", desc, seed, t71_seed % 8080);
        }
    }
    
    println!("\n🎵 Frequency Analysis (T_71 progression):\n");
    
    for (seed, label, desc) in &steps {
        let img_path = format!("monster_walk_{}_0.png", label);
        
        if std::path::Path::new(&img_path).exists() {
            let raw = ImageReader::open(&img_path)?.decode()?.to_rgb8().into_raw();
            
            println!("{} (seed={}):", desc, seed);
            print!("  Frequencies: ");
            for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71] {
                let tp = apply_hecke(&raw, p) % 8080;
                print!("T_{}={} ", p, tp);
            }
            println!("\n");
        }
    }
    
    println!("🔍 Looking for 8080 digit preservation pattern...");
    
    Ok(())
}
