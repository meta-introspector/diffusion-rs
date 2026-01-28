use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

// Exact seed from original I ARE LIFE experiment
const EXACT_SEED: i64 = 2437596016;
const EXACT_PROMPT: &str = "unconstrained";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 I ARE LIFE - Exact Reproduction");
    println!("==================================");
    println!("Seed: {}", EXACT_SEED);
    println!("Prompt: {}", EXACT_PROMPT);
    println!();
    
    for i in 0..5 {
        let seed = EXACT_SEED + i;
        let output = format!("i_are_life_step_{}.png", i);
        let output_clone = output.clone();
        
        println!("--- Iteration {} ---", i);
        println!("Seed: {}", seed);
        
        let (config, mut model_config) = PresetBuilder::default()
            .preset(Preset::SDXLTurbo1_0)
            .prompt(EXACT_PROMPT.to_string())
            .with_modifier(move |(mut config, model_config)| {
                config.seed(seed);
                config.output(&output_clone);
                Ok((config, model_config))
            })
            .build()?;
        
        gen_img(&config, &mut model_config)?;
        
        println!("✓ Generated: {}\n", output);
    }
    
    println!("✅ Complete! Analyze with LLaVA:");
    println!("   ollama run llava 'Describe this image' i_are_life_step_0.png");
    
    Ok(())
}
