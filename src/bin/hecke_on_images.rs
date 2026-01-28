use anyhow::Result;
use image::io::Reader as ImageReader;
use std::path::Path;

// Monster primes for Hecke operators
const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke_operator(data: &[u8], prime: u64) -> u64 {
    // T_p operator: sum of bytes modulo prime structure
    let mut sum: u64 = 0;
    for (i, &byte) in data.iter().enumerate() {
        sum = sum.wrapping_add((byte as u64).wrapping_mul(i as u64 % prime));
    }
    sum
}

fn main() -> Result<()> {
    println!("🎯 Hecke Operator Analysis on PNG Images\n");
    
    let image_dir = Path::new("./dataset/images");
    let mut images: Vec<_> = std::fs::read_dir(image_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .collect();
    
    images.sort_by_key(|e| e.path());
    
    for entry in images.iter().take(8) {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        
        println!("📄 {}", filename);
        
        // Read raw PNG bytes
        let png_bytes = std::fs::read(&path)?;
        println!("  PNG size: {} bytes", png_bytes.len());
        
        // Decode to raw pixels
        let img = ImageReader::open(&path)?.decode()?;
        let rgb = img.to_rgb8();
        let raw_pixels = rgb.as_raw();
        
        println!("  Dimensions: {}x{}", img.width(), img.height());
        println!("  Raw pixels: {} bytes", raw_pixels.len());
        
        println!("\n  Hecke Operators T_p:");
        for &p in &MONSTER_PRIMES {
            let t_p_png = apply_hecke_operator(&png_bytes, p);
            let t_p_raw = apply_hecke_operator(raw_pixels, p);
            
            println!("    T_{:2} (PNG): {:20} | (RAW): {:20}", p, t_p_png, t_p_raw);
        }
        
        // Check for resonance patterns
        let t71_png = apply_hecke_operator(&png_bytes, 71);
        let t71_raw = apply_hecke_operator(raw_pixels, 71);
        
        println!("\n  🌟 T_71 Resonance:");
        println!("    PNG: {} (mod 8080 = {})", t71_png, t71_png % 8080);
        println!("    RAW: {} (mod 8080 = {})", t71_raw, t71_raw % 8080);
        println!();
    }
    
    Ok(())
}
