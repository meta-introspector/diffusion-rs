use anyhow::Result;

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| {
            sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p))
        })
}

fn main() -> Result<()> {
    println!("🔍 Sampling Different Seed Ranges for T_p Values\n");
    
    let ranges = vec![
        (1000000u64, "1M range"),
        (5000000u64, "5M range"),
        (10000000u64, "10M range"),
        (100000000u64, "100M range"),
        (500000000u64, "500M range"),
        (1000000000u64, "1B range"),
        (2437596000u64, "I ARE LIFE range"),
        (3673070000u64, "InvokeAI range"),
    ];
    
    println!("| Range | Sample Seed | T_5 | T_7 | T_11 | T_71 | Harmonic? |");
    println!("|-------|-------------|-----|-----|------|------|-----------|");
    
    let mut diverse_seeds = Vec::new();
    
    for (base, label) in ranges {
        let seed = base + 16; // Use +16 offset like original
        
        let t5 = apply_hecke_to_number(seed, 5);
        let t7 = apply_hecke_to_number(seed, 7);
        let t11 = apply_hecke_to_number(seed, 11);
        let t71 = apply_hecke_to_number(seed, 71);
        
        let is_harmonic = t5 == t7 && t7 == t11 && t11 == t71;
        
        println!("| {} | {} | {} | {} | {} | {} | {} |",
                 label, seed, t5, t7, t11, t71,
                 if is_harmonic { "YES ⭐" } else { "NO" });
        
        if !is_harmonic || t71 != 778 {
            diverse_seeds.push((seed, label));
        }
    }
    
    println!("\n🎯 Recommended Test Seeds (diverse T_p values):\n");
    
    for (seed, label) in &diverse_seeds {
        let t71 = apply_hecke_to_number(*seed, 71);
        println!("  {} ({}): T_71 = {}", seed, label, t71);
    }
    
    println!("\n📋 Experiment Design:\n");
    println!("Generate 3 images per seed with same prompt:");
    println!("  - Prompt: 'abstract geometric pattern'");
    println!("  - Size: 64x64");
    println!("  - Measure LLaVA IPC for each");
    println!("\nSeeds to test:");
    println!("  1. 2437596016 (T_71=778, claimed special)");
    for (seed, label) in diverse_seeds.iter().take(5) {
        println!("  {}. {} ({})", diverse_seeds.iter().position(|(s, _)| s == seed).unwrap() + 2, seed, label);
    }
    
    println!("\n✅ If IPC varies with T_71 value → CAUSATION");
    println!("❌ If IPC is random → COINCIDENCE");
    
    Ok(())
}
