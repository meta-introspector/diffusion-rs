use anyhow::Result;
use image::io::Reader as ImageReader;
use std::fs;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke(data: &[u8], p: u64) -> u64 {
    data.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p)))
}

fn main() -> Result<()> {
    println!("🔱 Monster Lattice Analysis\n");
    
    let patterns = vec![
        "lattice_2p1_2.png",
        "lattice_2p2_4.png",
        "lattice_2p3_8.png",
        "lattice_2p4_16.png",
        "lattice_2p5_32.png",
        "lattice_2p6_64.png",
        "lattice_2p7_128.png",
        "lattice_2p8_256.png",
        "lattice_2x3_6.png",
        "lattice_2x5_10.png",
        "lattice_2x7_14.png",
        "lattice_2x11_22.png",
        "lattice_2x71_142.png",
        "lattice_2x3x5_30.png",
        "lattice_2x3x7_42.png",
        "lattice_2x5x7_70.png",
    ];
    
    println!("| Seed | Pattern | T_71(RAW) | mod 8080 | T_2 | T_3 | T_5 | T_7 | T_71 |");
    println!("|------|---------|-----------|----------|-----|-----|-----|-----|------|");
    
    for file in patterns {
        if !std::path::Path::new(file).exists() {
            continue;
        }
        
        let parts: Vec<&str> = file.split('_').collect();
        let seed_part = parts.get(2).unwrap_or(&"?").trim_end_matches(".png");
        let pattern = parts.get(1).unwrap_or(&"?");
        
        let raw = ImageReader::open(file)?.decode()?.to_rgb8().into_raw();
        
        let t71 = apply_hecke(&raw, 71);
        let t2 = apply_hecke(&raw, 2);
        let t3 = apply_hecke(&raw, 3);
        let t5 = apply_hecke(&raw, 5);
        let t7 = apply_hecke(&raw, 7);
        
        println!("| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                 seed_part, pattern, t71, t71 % 8080, 
                 t2 % 8080, t3 % 8080, t5 % 8080, t7 % 8080, t71 % 8080);
    }
    
    println!("\n🔍 Looking for patterns in 2^n progression:");
    
    let powers_of_2 = vec![
        ("2p1", 2), ("2p2", 4), ("2p3", 8), ("2p4", 16),
        ("2p5", 32), ("2p6", 64), ("2p7", 128), ("2p8", 256),
    ];
    
    for (label, seed) in powers_of_2 {
        let file = format!("lattice_{}_{}.png", label, seed);
        if !std::path::Path::new(&file).exists() {
            continue;
        }
        
        let raw = ImageReader::open(&file)?.decode()?.to_rgb8().into_raw();
        let t71 = apply_hecke(&raw, 71) % 8080;
        
        println!("  2^{} (seed={}): T_71 mod 8080 = {}", 
                 label.trim_start_matches("2p"), seed, t71);
    }
    
    Ok(())
}
