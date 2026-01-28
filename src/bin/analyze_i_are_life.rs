use anyhow::Result;
use image::io::Reader as ImageReader;
use std::fs;
use std::path::Path;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
const EXACT_SEED: i64 = 2437596016;

fn apply_hecke(data: &[u8], p: u64) -> u64 {
    data.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p)))
}

fn shard_data(data: &[u8], n: usize) -> Vec<Vec<u8>> {
    let size = (data.len() + n - 1) / n;
    (0..n).map(|i| {
        let start = i * size;
        let end = (start + size).min(data.len());
        data[start..end].to_vec()
    }).collect()
}

fn main() -> Result<()> {
    println!("🌱 I ARE LIFE Analysis - Seed {}\n", EXACT_SEED);
    
    let dir = Path::new("./dataset/i_are_life");
    let out = Path::new("./dataset/i_are_life_shards");
    fs::create_dir_all(out)?;
    
    for i in 0..5 {
        let seed = EXACT_SEED + i;
        let path = dir.join(format!("i_are_life_step_{}.png", i));
        
        if !path.exists() { continue; }
        
        println!("📄 Step {}: seed = {}", i, seed);
        
        let png = fs::read(&path)?;
        let img = ImageReader::open(&path)?.decode()?;
        let raw = img.to_rgb8().into_raw();
        
        println!("  PNG: {} bytes, RAW: {} bytes", png.len(), raw.len());
        
        // Full image Hecke
        let t71_png = apply_hecke(&png, 71);
        let t71_raw = apply_hecke(&raw, 71);
        println!("  T_71: PNG={} (mod 8080={}), RAW={} (mod 8080={})",
                 t71_png, t71_png % 8080, t71_raw, t71_raw % 8080);
        
        // 71 shards
        let png_shards = shard_data(&png, 71);
        let raw_shards = shard_data(&raw, 71);
        
        let shard_dir = out.join(format!("step_{}", i));
        fs::create_dir_all(&shard_dir)?;
        
        let mut resonant = Vec::new();
        
        for (j, (ps, rs)) in png_shards.iter().zip(raw_shards.iter()).enumerate() {
            let tp = apply_hecke(ps, 71);
            let tr = apply_hecke(rs, 71);
            
            if tp % 8080 < 100 || tr % 8080 < 100 {
                resonant.push((j, tp % 8080, tr % 8080));
            }
        }
        
        println!("  ✨ Resonant shards: {}", resonant.len());
        for (j, p, r) in &resonant {
            println!("    Shard {}: PNG={} RAW={}", j, p, r);
        }
        
        // Save summary
        let mut summary = format!("Seed: {}\n\n", seed);
        summary.push_str(&format!("T_71 (full): PNG={} RAW={}\n\n", t71_png % 8080, t71_raw % 8080));
        summary.push_str("Resonant shards:\n");
        for (j, p, r) in resonant {
            summary.push_str(&format!("Shard {}: PNG={} RAW={}\n", j, p, r));
        }
        fs::write(shard_dir.join("summary.txt"), summary)?;
        
        println!();
    }
    
    Ok(())
}
