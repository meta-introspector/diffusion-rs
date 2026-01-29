use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Validation Experiment: Seed Efficiency Hypothesis\n");
    println!("Testing if harmonic seeds show better LLaVA efficiency\n");
    
    // Test seeds with different properties
    let test_cases = vec![
        // Harmonic seeds (T_p = 778 for p >= 5)
        (2437596016i64, "harmonic", vec!["a cat", "a dog", "abstract art", "geometric pattern"]),
        (2437596000i64, "harmonic", vec!["a cat", "a dog", "abstract art", "geometric pattern"]),
        
        // Best lattice seeds
        (32i64, "lattice_2p5", vec!["a cat", "a dog", "abstract art", "geometric pattern"]),
        (42i64, "lattice_2x3x7", vec!["a cat", "a dog", "abstract art", "geometric pattern"]),
        
        // Control (random)
        (1234567890i64, "control", vec!["a cat", "a dog", "abstract art", "geometric pattern"]),
        (9999999999i64, "control2", vec!["a cat", "a dog", "abstract art", "geometric pattern"]),
    ];
    
    for (seed, label, prompts) in test_cases {
        for (i, prompt) in prompts.iter().enumerate() {
            let output = format!("validate_{}_{}_{}.png", label, seed, i);
            let output_clone = output.clone();
            
            println!("Generating: {} with '{}' (seed={})", label, prompt, seed);
            
            let (config, mut model_config) = PresetBuilder::default()
                .preset(Preset::SDXLTurbo1_0)
                .prompt(prompt.to_string())
                .with_modifier(move |(mut config, model_config)| {
                    config.seed(seed);
                    config.output(&output_clone);
                    config.width(64);
                    config.height(64);
                    Ok((config, model_config))
                })
                .build()?;
            
            gen_img(&config, &mut model_config)?;
            println!("  ✓ {}\n", output);
        }
    }
    
    println!("✅ Validation images generated!");
    println!("\nNext steps:");
    println!("1. Run: ./validate_llava.sh");
    println!("2. Run: cargo run --bin analyze_validation");
    
    Ok(())
}
