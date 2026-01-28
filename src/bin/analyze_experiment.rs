use anyhow::Result;
use image::io::Reader as ImageReader;
use std::fs;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke(data: &[u8], p: u64) -> u64 {
    data.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p)))
}

fn main() -> Result<()> {
    println!("📊 Experiment Results Analysis\n");
    
    let experiments = vec![
        ("experiment_perfect_0_1.png", 1i64, "Perfect A"),
        ("experiment_perfect_0_2.png", 2i64, "Perfect A"),
        ("experiment_perfect_0_3.png", 3i64, "Perfect A"),
        ("experiment_harmonic_778_2437596000.png", 2437596000i64, "Harmonic B"),
        ("experiment_i_are_life_2437596016.png", 2437596016i64, "Harmonic B"),
        ("experiment_control_random_1234567890.png", 1234567890i64, "Control C"),
    ];
    
    println!("| Seed | Group | T_71(PNG) | T_71(RAW) | PNG mod 8080 | RAW mod 8080 |");
    println!("|------|-------|-----------|-----------|--------------|--------------|");
    
    for (file, seed, group) in &experiments {
        if !std::path::Path::new(file).exists() {
            continue;
        }
        
        let png = fs::read(file)?;
        let img = ImageReader::open(file)?.decode()?;
        let raw = img.to_rgb8().into_raw();
        
        let t71_png = apply_hecke(&png, 71);
        let t71_raw = apply_hecke(&raw, 71);
        
        println!("| {} | {} | {} | {} | {} | {} |",
                 seed, group, t71_png, t71_raw, t71_png % 8080, t71_raw % 8080);
    }
    
    println!("\n🔍 Detailed Analysis:\n");
    
    for (file, seed, group) in &experiments {
        if !std::path::Path::new(file).exists() {
            continue;
        }
        
        println!("{}:", file);
        println!("  Seed: {} ({})", seed, group);
        
        let png = fs::read(file)?;
        let raw = ImageReader::open(file)?.decode()?.to_rgb8().into_raw();
        
        println!("  All Hecke operators (RAW):");
        for &p in &MONSTER_PRIMES {
            let tp = apply_hecke(&raw, p);
            print!("    T_{}={} ", p, tp % 8080);
        }
        println!("\n");
    }
    
    Ok(())
}
