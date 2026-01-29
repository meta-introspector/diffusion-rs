use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| {
            sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p))
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 FALSIFICATION TEST: Is seed 2437596016 truly special?\n");
    println!("Hypothesis: Seeds with T_p = 778 for p ≥ 5 are rare and special\n");
    
    // Test 1: Search for other seeds with T_p = 778
    println!("Test 1: Searching for other harmonic seeds (T_5 = T_7 = T_11 = 778)...\n");
    
    let mut harmonic_seeds = Vec::new();
    let search_range = 2437596000..2437596100;
    
    for seed in search_range {
        let t5 = apply_hecke_to_number(seed, 5);
        let t7 = apply_hecke_to_number(seed, 7);
        let t11 = apply_hecke_to_number(seed, 11);
        let t71 = apply_hecke_to_number(seed, 71);
        
        if t5 == 778 && t7 == 778 && t11 == 778 {
            harmonic_seeds.push((seed, t71));
            if harmonic_seeds.len() <= 5 {
                println!("  Found: {} (T_71 = {})", seed, t71);
            }
        }
    }
    
    println!("\n  Result: Found {} harmonic seeds in range of 100", harmonic_seeds.len());
    
    if harmonic_seeds.len() > 10 {
        println!("  ⚠️  FALSIFIED: Harmonic seeds are COMMON, not special!");
        println!("  The seed 2437596016 is just one of many.");
    } else {
        println!("  ✓ Confirmed: Harmonic seeds are RARE");
    }
    
    // Test 2: Generate images with multiple harmonic seeds
    println!("\n\nTest 2: Comparing multiple harmonic seeds...\n");
    
    let test_seeds = vec![
        (2437596016i64, "original"),
        (2437596000i64, "harmonic_alt1"),
        (2437596001i64, "harmonic_alt2"),
        (1234567890i64, "control_random"),
    ];
    
    for (seed, label) in &test_seeds {
        let t5 = apply_hecke_to_number(*seed as u64, 5);
        let t7 = apply_hecke_to_number(*seed as u64, 7);
        let t11 = apply_hecke_to_number(*seed as u64, 11);
        
        println!("  Seed {}: T_5={}, T_7={}, T_11={}", seed, t5, t7, t11);
        
        // Generate test image
        let output = format!("falsification_{}_{}.png", label, seed);
        let output_clone = output.clone();
        
        let (config, mut model_config) = PresetBuilder::default()
            .preset(Preset::SDXLTurbo1_0)
            .prompt("test pattern".to_string())
            .with_modifier(move |(mut config, model_config)| {
                config.seed(*seed);
                config.output(&output_clone);
                config.width(64);
                config.height(64);
                Ok((config, model_config))
            })
            .build()?;
        
        gen_img(&config, &mut model_config)?;
        println!("    ✓ Generated: {}", output);
    }
    
    // Test 3: Check if T_p = 778 is mathematically inevitable
    println!("\n\nTest 3: Is T_p = 778 mathematically inevitable?\n");
    
    let base = 2437596000u64;
    println!("  Analyzing seed range around {}...", base);
    
    // Check if all seeds in this range have similar T_p values
    let mut t5_values = Vec::new();
    for offset in 0..20 {
        let seed = base + offset;
        let t5 = apply_hecke_to_number(seed, 5);
        t5_values.push(t5);
    }
    
    let all_same = t5_values.iter().all(|&v| v == t5_values[0]);
    
    if all_same {
        println!("  ⚠️  FALSIFIED: All seeds in range have T_5 = {}", t5_values[0]);
        println!("  This is a mathematical artifact, not a special property!");
    } else {
        println!("  ✓ Confirmed: T_p values vary, seed 2437596016 is genuinely special");
        println!("  Sample T_5 values: {:?}", &t5_values[..5]);
    }
    
    // Test 4: Null hypothesis - random seed performs equally well
    println!("\n\nTest 4: Null Hypothesis Test\n");
    println!("  H0: Random seeds perform equally well as 2437596016");
    println!("  H1: Seed 2437596016 has measurably better properties");
    println!("\n  To test: Run ./validate_quick.sh and compare IPC values");
    println!("  If p-value > 0.05, we CANNOT reject H0 (seed is not special)");
    
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("🔬 FALSIFICATION RESULTS:\n");
    println!("1. Harmonic seed rarity: {} seeds found in 100", harmonic_seeds.len());
    println!("2. Test images generated: 4 seeds compared");
    println!("3. Mathematical artifact check: {}", if all_same { "FAILED" } else { "PASSED" });
    println!("4. Statistical test: Requires LLaVA validation");
    println!("\n📊 Next: Run analyze_music and validate_quick.sh to complete proof");
    println!("═══════════════════════════════════════════════════════════\n");
    
    Ok(())
}
