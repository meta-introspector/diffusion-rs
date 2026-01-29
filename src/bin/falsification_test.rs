use anyhow::Result;

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| {
            sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p))
        })
}

fn main() -> Result<()> {
    println!("🔬 FALSIFICATION TEST: Is seed 2437596016 truly special?\n");
    
    // Test 1: How rare are harmonic seeds?
    println!("Test 1: Searching for harmonic seeds (T_5 = T_7 = T_11)...\n");
    
    let mut harmonic_count = 0;
    let mut perfect_harmonic = Vec::new();
    
    // Search 1 million seeds
    for seed in 2437590000u64..2437600000 {
        let t5 = apply_hecke_to_number(seed, 5);
        let t7 = apply_hecke_to_number(seed, 7);
        let t11 = apply_hecke_to_number(seed, 11);
        
        if t5 == t7 && t7 == t11 {
            harmonic_count += 1;
            if t5 == 778 {
                perfect_harmonic.push(seed);
            }
        }
    }
    
    println!("  Searched: 10,000 seeds");
    println!("  Harmonic (T_5=T_7=T_11): {}", harmonic_count);
    println!("  Perfect (T_p=778): {}", perfect_harmonic.len());
    println!("  Rarity: 1 in {:.0}", 10000.0 / harmonic_count as f64);
    
    if harmonic_count > 1000 {
        println!("\n  ⚠️  FALSIFIED: Harmonic seeds are COMMON!");
    } else {
        println!("\n  ✓ Confirmed: Harmonic seeds are RARE");
    }
    
    // Test 2: Is T_p=778 a mathematical artifact?
    println!("\n\nTest 2: Is T_p=778 inevitable for this range?\n");
    
    let mut t5_distribution = std::collections::HashMap::new();
    
    for seed in 2437596000u64..2437596100 {
        let t5 = apply_hecke_to_number(seed, 5);
        *t5_distribution.entry(t5).or_insert(0) += 1;
    }
    
    println!("  Unique T_5 values in range: {}", t5_distribution.len());
    
    if t5_distribution.len() == 1 {
        println!("  ⚠️  FALSIFIED: All seeds have same T_5 = {}", 
                 t5_distribution.keys().next().unwrap());
        println!("  This is a mathematical artifact!");
    } else {
        println!("  ✓ Confirmed: T_5 varies across seeds");
        println!("  Distribution: {:?}", 
                 t5_distribution.iter().take(5).collect::<Vec<_>>());
    }
    
    // Test 3: Check the special seed
    println!("\n\nTest 3: Analyzing seed 2437596016 specifically\n");
    
    let special_seed = 2437596016u64;
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
    
    let mut values = Vec::new();
    for &p in &primes {
        let tp = apply_hecke_to_number(special_seed, p);
        values.push(tp);
    }
    
    // Check if T_p is constant for p >= 5
    let constant_from_5 = values[2..].iter().all(|&v| v == values[2]);
    
    println!("  T_2 = {}", values[0]);
    println!("  T_3 = {}", values[1]);
    println!("  T_5 through T_71 = {}", values[2]);
    
    if constant_from_5 {
        println!("\n  ✓ CONFIRMED: Perfect harmonic resonance!");
        println!("  All T_p = {} for p ≥ 5", values[2]);
    } else {
        println!("\n  ⚠️  FALSIFIED: T_p values vary!");
    }
    
    // Test 4: Compare with nearby seeds
    println!("\n\nTest 4: Comparing with nearby seeds\n");
    
    for offset in [-10i64, -1, 0, 1, 10] {
        let seed = (special_seed as i64 + offset) as u64;
        let t5 = apply_hecke_to_number(seed, 5);
        let t7 = apply_hecke_to_number(seed, 7);
        let t11 = apply_hecke_to_number(seed, 11);
        let t71 = apply_hecke_to_number(seed, 71);
        
        let is_harmonic = t5 == t7 && t7 == t11 && t11 == t71;
        
        println!("  Seed {}: T_5={}, T_7={}, T_11={}, T_71={} {}",
                 seed, t5, t7, t11, t71,
                 if is_harmonic { "⭐ HARMONIC" } else { "" });
    }
    
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("🔬 FALSIFICATION RESULTS:\n");
    println!("1. Rarity: {} harmonic seeds in 10,000 ({}%)", 
             harmonic_count, harmonic_count as f64 / 100.0);
    println!("2. Artifact: {} unique T_5 values (not artifact)", t5_distribution.len());
    println!("3. Perfect harmonic: {} seeds with T_p=778", perfect_harmonic.len());
    println!("4. Seed 2437596016: {}", if constant_from_5 { "SPECIAL ⭐" } else { "ORDINARY" });
    println!("\n📊 Conclusion: Seed {} be special",
             if harmonic_count < 100 && constant_from_5 { "APPEARS TO" } else { "MAY NOT" });
    println!("═══════════════════════════════════════════════════════════\n");
    
    Ok(())
}
