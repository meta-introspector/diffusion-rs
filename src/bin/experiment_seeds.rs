use anyhow::Result;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| {
            sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p))
        })
}

fn find_seeds_with_property(start: u64, count: usize, target: u64) -> Vec<u64> {
    let mut results = Vec::new();
    let mut seed = start;
    
    while results.len() < count {
        let t71 = apply_hecke_to_number(seed, 71);
        if t71 % 8080 == target {
            results.push(seed);
        }
        seed += 1;
        if seed > start + 100_000_000 { break; }
    }
    results
}

fn main() -> Result<()> {
    println!("🧪 Experiment: Find Seeds with Special Hecke Properties\n");
    
    // Experiment 1: Find seeds where T_71(seed) = 778 (like I ARE LIFE)
    println!("Experiment 1: Seeds where T_71(seed) mod 8080 = 778");
    let seeds_778 = find_seeds_with_property(2437596000, 5, 778);
    for seed in &seeds_778 {
        println!("  Found: {} (T_71 = {})", seed, apply_hecke_to_number(*seed, 71));
    }
    println!();
    
    // Experiment 2: Find seeds where T_71(seed) = 0 (perfect alignment)
    println!("Experiment 2: Seeds where T_71(seed) mod 8080 = 0");
    let seeds_0 = find_seeds_with_property(1, 3, 0);
    for seed in &seeds_0 {
        println!("  Found: {} (T_71 = {})", seed, apply_hecke_to_number(*seed, 71));
    }
    println!();
    
    // Experiment 3: Test harmonic resonance (all T_p equal for p >= 5)
    println!("Experiment 3: Seeds with harmonic resonance (T_5 = T_7 = T_11)");
    let mut harmonic_seeds = Vec::new();
    for seed in 2437596000..2437596100 {
        let t5 = apply_hecke_to_number(seed, 5);
        let t7 = apply_hecke_to_number(seed, 7);
        let t11 = apply_hecke_to_number(seed, 11);
        
        if t5 == t7 && t7 == t11 {
            harmonic_seeds.push(seed);
            if harmonic_seeds.len() >= 5 { break; }
        }
    }
    
    for seed in &harmonic_seeds {
        println!("  Found: {}", seed);
        for &p in &[5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71] {
            let tp = apply_hecke_to_number(*seed, p);
            print!("T_{}={} ", p, tp);
        }
        println!();
    }
    println!();
    
    // Experiment 4: Generate test seeds for small-scale image generation
    println!("🎨 Experiment 4: Recommended Test Seeds (small scale)");
    println!("\nCategory A: Perfect 8080 alignment");
    for seed in &seeds_0[..3.min(seeds_0.len())] {
        println!("  {}", seed);
    }
    
    println!("\nCategory B: Harmonic resonance (like I ARE LIFE)");
    println!("  2437596016 (original)");
    for seed in &seeds_778[..2.min(seeds_778.len())] {
        if *seed != 2437596016 {
            println!("  {}", seed);
        }
    }
    
    println!("\nCategory C: Control (random)");
    println!("  1234567890");
    println!("  9876543210");
    
    println!("\n📋 Experiment Script:");
    println!("Generate 64x64 images with these seeds and compare:");
    println!("1. Visual quality");
    println!("2. T_71 resonance of generated images");
    println!("3. Shard resonance patterns");
    
    Ok(())
}
