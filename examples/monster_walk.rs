use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

// Monster group order: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
// Monster Walk: Systematically remove prime factors to create 10 groups

const BASE_SEED: i64 = 2437596016; // I ARE LIFE seed with T_p = 778

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔱 Monster Walk on Seed {}\n", BASE_SEED);
    println!("Dividing by 10 groups of Monster primes\n");
    
    // Monster Walk groups (removing primes in order)
    let monster_walk = vec![
        (BASE_SEED, "step_0_full", "🌀 Full seed"),
        (BASE_SEED / 71, "step_1_div71", "🎵 Remove 71"),
        (BASE_SEED / (71 * 59), "step_2_div71x59", "🎶 Remove 71×59"),
        (BASE_SEED / (71 * 59 * 47), "step_3_div71x59x47", "🎸 Remove 71×59×47"),
        (BASE_SEED / (71 * 59 * 47 * 41), "step_4_div71x59x47x41", "🎹 Remove 71×59×47×41"),
        (BASE_SEED / (71 * 59 * 47 * 41 * 31), "step_5_div71x59x47x41x31", "🎺 Remove 71×59×47×41×31"),
        (BASE_SEED / (71 * 59 * 47 * 41 * 31 * 29), "step_6_div5primes", "🎻 Remove 5 largest primes"),
        (BASE_SEED / (71 * 59 * 47 * 41 * 31 * 29 * 23), "step_7_div6primes", "🥁 Remove 6 largest"),
        (BASE_SEED / (71 * 59 * 47 * 41 * 31 * 29 * 23 * 19), "step_8_div7primes", "🎼 Remove 7 largest"),
        (BASE_SEED / (71 * 59 * 47 * 41 * 31 * 29 * 23 * 19 * 17), "step_9_div8primes", "🎤 Remove 8 largest"),
        (BASE_SEED / (71 * 59 * 47 * 41 * 31 * 29 * 23 * 19 * 17 * 13), "step_10_div9primes", "🔊 Remove 9 largest"),
    ];
    
    let prompts = vec![
        "vibrational frequency pattern",
        "harmonic resonance structure", 
        "prime ladder ascending",
        "emoji wave interference",
    ];
    
    for (seed, label, emoji_desc) in &monster_walk {
        let seed_val = *seed;
        let label_str = label.to_string();
        let emoji_str = emoji_desc.to_string();
        
        println!("=== {} ===", emoji_desc);
        println!("Seed: {} ({})", seed, label);
        
        for (i, prompt) in prompts.iter().enumerate() {
            let output = format!("monster_walk_{}_{}.png", label, i);
            let output_clone = output.clone();
            let prompt_with_emoji = format!("{} {}", emoji_str, prompt);
            
            println!("  Generating: '{}'", prompt);
            
            let (config, mut model_config) = PresetBuilder::default()
                .preset(Preset::SDXLTurbo1_0)
                .prompt(prompt_with_emoji)
                .with_modifier(move |(mut config, model_config)| {
                    config.seed(seed_val);
                    config.output(&output_clone);
                    config.width(64);
                    config.height(64);
                    Ok((config, model_config))
                })
                .build()?;
            
            gen_img(&config, &mut model_config)?;
            println!("    ✓ {}", output);
        }
        println!();
    }
    
    println!("✅ Monster Walk complete!");
    println!("\nGenerated 11 steps × 4 prompts = 44 images");
    println!("Next: cargo run --bin analyze_monster_walk");
    
    Ok(())
}
