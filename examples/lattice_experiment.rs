use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

// Monster group order factorization: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔱 Monster Lattice Experiment: Seeds as Prompts\n");
    
    // Generate seeds from Monster lattice structure
    // Using 2^n forms where n corresponds to Monster prime positions
    let lattice_seeds: Vec<(u64, String)> = vec![
        // Powers of 2 (2^1 through 2^15)
        (2u64.pow(1), "2^1".to_string()),
        (2u64.pow(2), "2^2".to_string()),
        (2u64.pow(3), "2^3".to_string()),
        (2u64.pow(4), "2^4".to_string()),
        (2u64.pow(5), "2^5".to_string()),
        (2u64.pow(6), "2^6".to_string()),
        (2u64.pow(7), "2^7".to_string()),
        (2u64.pow(8), "2^8".to_string()),
        
        // Monster prime products
        (2 * 3, "2×3".to_string()),
        (2 * 5, "2×5".to_string()),
        (2 * 7, "2×7".to_string()),
        (2 * 11, "2×11".to_string()),
        (2 * 71, "2×71".to_string()),
        
        // Triple products
        (2 * 3 * 5, "2×3×5".to_string()),
        (2 * 3 * 7, "2×3×7".to_string()),
        (2 * 5 * 7, "2×5×7".to_string()),
    ];
    
    for (seed, label) in lattice_seeds {
        let seed_i64 = seed as i64;
        let prompt = format!("{} lattice structure", label);
        let output = format!("lattice_{}_{}.png", label.replace("×", "x").replace("^", "p"), seed);
        let output_clone = output.clone();
        
        println!("Generating: {} (seed={}, prompt='{}')", label, seed, prompt);
        
        let (config, mut model_config) = PresetBuilder::default()
            .preset(Preset::SDXLTurbo1_0)
            .prompt(prompt)
            .with_modifier(move |(mut config, model_config)| {
                config.seed(seed_i64);
                config.output(&output_clone);
                config.width(64);
                config.height(64);
                Ok((config, model_config))
            })
            .build()?;
        
        gen_img(&config, &mut model_config)?;
        println!("  ✓ {}\n", output);
    }
    
    println!("✅ Monster lattice experiment complete!");
    println!("\nNext: cargo run --bin analyze_lattice");
    
    Ok(())
}
