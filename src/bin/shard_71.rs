use anyhow::Result;
use image::io::Reader as ImageReader;
use std::fs;
use std::path::Path;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke_operator(data: &[u8], prime: u64) -> u64 {
    data.iter().enumerate()
        .fold(0u64, |sum, (i, &byte)| {
            sum.wrapping_add((byte as u64).wrapping_mul(i as u64 % prime))
        })
}

fn shard_data(data: &[u8], num_shards: usize) -> Vec<Vec<u8>> {
    let shard_size = (data.len() + num_shards - 1) / num_shards;
    (0..num_shards)
        .map(|i| {
            let start = i * shard_size;
            let end = (start + shard_size).min(data.len());
            data[start..end].to_vec()
        })
        .collect()
}

fn main() -> Result<()> {
    println!("🔱 Hecke Operators + 71-Shard Analysis\n");
    
    let image_dir = Path::new("./dataset/images");
    let output_dir = Path::new("./dataset/shards");
    fs::create_dir_all(output_dir)?;
    
    let mut images: Vec<_> = fs::read_dir(image_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .collect();
    
    images.sort_by_key(|e| e.path());
    
    for (img_idx, entry) in images.iter().enumerate() {
        let path = entry.path();
        let filename = path.file_stem().unwrap().to_string_lossy();
        
        println!("📄 Image {}: {}", img_idx + 1, filename);
        
        let png_bytes = fs::read(&path)?;
        let img = ImageReader::open(&path)?.decode()?;
        let raw_pixels = img.to_rgb8().into_raw();
        
        println!("  PNG: {} bytes, RAW: {} bytes", png_bytes.len(), raw_pixels.len());
        
        // Split into 71 shards
        let png_shards = shard_data(&png_bytes, 71);
        let raw_shards = shard_data(&raw_pixels, 71);
        
        println!("  Created 71 shards (PNG: ~{} bytes/shard, RAW: ~{} bytes/shard)",
                 png_bytes.len() / 71, raw_pixels.len() / 71);
        
        // Apply Hecke operators to each shard
        let shard_dir = output_dir.join(&*filename);
        fs::create_dir_all(&shard_dir)?;
        
        let mut shard_analysis = Vec::new();
        
        for (shard_idx, (png_shard, raw_shard)) in png_shards.iter().zip(raw_shards.iter()).enumerate() {
            let mut shard_data = format!("Shard {}\n", shard_idx);
            shard_data.push_str(&format!("PNG bytes: {}, RAW bytes: {}\n", png_shard.len(), raw_shard.len()));
            shard_data.push_str("\nHecke Operators:\n");
            
            for &p in &MONSTER_PRIMES {
                let t_p_png = apply_hecke_operator(png_shard, p);
                let t_p_raw = apply_hecke_operator(raw_shard, p);
                shard_data.push_str(&format!("T_{:2}: PNG={:12} RAW={:12}\n", p, t_p_png, t_p_raw));
            }
            
            // T_71 resonance
            let t71_png = apply_hecke_operator(png_shard, 71);
            let t71_raw = apply_hecke_operator(raw_shard, 71);
            shard_data.push_str(&format!("\nT_71 mod 8080: PNG={} RAW={}\n", 
                                         t71_png % 8080, t71_raw % 8080));
            
            shard_analysis.push((shard_idx, t71_png % 8080, t71_raw % 8080));
            
            // Save shard data
            fs::write(shard_dir.join(format!("shard_{:02}.txt", shard_idx)), shard_data)?;
        }
        
        // Summary
        let png_8080_count = shard_analysis.iter().filter(|(_, p, _)| *p == 0).count();
        let raw_8080_count = shard_analysis.iter().filter(|(_, _, r)| *r == 0).count();
        
        println!("  ✨ Shards with T_71 ≡ 0 (mod 8080): PNG={}, RAW={}", 
                 png_8080_count, raw_8080_count);
        
        // Save summary
        let mut summary = format!("Image: {}\n\n", filename);
        summary.push_str("Shard Analysis (T_71 mod 8080):\n");
        for (idx, png_res, raw_res) in shard_analysis {
            summary.push_str(&format!("Shard {:2}: PNG={:4} RAW={:4}\n", idx, png_res, raw_res));
        }
        fs::write(shard_dir.join("summary.txt"), summary)?;
        
        println!();
    }
    
    println!("✅ Shard analysis complete. Results in ./dataset/shards/");
    
    Ok(())
}
