use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Causation Test: T_71 Value vs LLaVA IPC\n");
    
    let test_seeds = vec![
        (1000016i64, 96, "T71_96"),
        (5000016i64, 227, "T71_227"),
        (10000016i64, 454, "T71_454"),
        (100000016i64, 730, "T71_730"),
        (2437596016i64, 778, "T71_778"),
        (3673070016i64, 1283, "T71_1283"),
    ];
    
    let prompt = "abstract geometric pattern";
    
    for (seed, t71, label) in test_seeds {
        println!("Generating: {} (seed={}, T_71={})", label, seed, t71);
        
        for i in 0..3 {
            let output = format!("causation_{}_{}.png", label, i);
            let output_clone = output.clone();
            
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
            println!("  ✓ {}", output);
        }
        println!();
    }
    
    println!("✅ Generated 18 images (6 seeds × 3 replicates)");
    println!("\nNext: ./causation_test_llava.sh");
    
    Ok(())
}
