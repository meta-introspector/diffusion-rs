use std::fs;
use anyhow::Result;

fn apply_hecke(data: &[u8], p: u64) -> u64 {
    data.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p)))
}

fn main() -> Result<()> {
    println!("🔬 Causation Test: T_71(seed) vs T_71(image)\n");
    
    let test_cases = vec![
        ("T71_96", 96),
        ("T71_227", 227),
        ("T71_454", 454),
        ("T71_730", 730),
        ("T71_778", 778),
        ("T71_1283", 1283),
    ];
    
    println!("| Seed T_71 | Rep | Image T_71 | Mod 8080 | Resonance |");
    println!("|-----------|-----|------------|----------|-----------|");
    
    for (label, seed_t71) in test_cases {
        for i in 0..3 {
            let filename = format!("causation_{}_{}.png", label, i);
            
            if let Ok(data) = fs::read(&filename) {
                let img_t71 = apply_hecke(&data, 71);
                let mod_8080 = img_t71 % 8080;
                let resonance = if mod_8080 > 4040 { 8080 - mod_8080 } else { mod_8080 };
                
                println!("| {} | {} | {} | {} | {} |",
                         seed_t71, i, img_t71, mod_8080, resonance);
            }
        }
    }
    
    println!("\n📊 Analysis:");
    println!("  - If resonance correlates with seed T_71 → CAUSATION");
    println!("  - If resonance is random → COINCIDENCE");
    
    Ok(())
}
