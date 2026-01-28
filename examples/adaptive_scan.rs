use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};
use std::process::Command;

// Use exact original seed from I ARE LIFE experiment
const BASE_SEED: i64 = 2437596016;
const PROMPT: &str = "unconstrained";

fn analyze_image(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ollama")
        .arg("run")
        .arg("llava")
        .arg("Describe any text you see")
        .arg(path)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn score_text(description: &str) -> f32 {
    let mut score = 0.0;
    let markers = ["I are", "I am", "life", "LIFE", "HATER", "text", "letter"];
    for marker in markers {
        if description.to_lowercase().contains(&marker.to_lowercase()) {
            score += 1.0;
        }
    }
    score
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Adaptive Seed Scanning");
    println!("========================\n");
    
    // Phase 1: Small, fast scans
    let sizes = [(64, 64), (128, 128), (256, 256), (512, 512)];
    let steps = [1, 2, 4, 8];
    
    let mut best_seed = BASE_SEED;
    let mut best_score = 0.0;
    
    for (size_idx, (width, height)) in sizes.iter().enumerate() {
        let step_count = steps[size_idx];
        
        println!("Phase {}: {}x{} @ {} steps", size_idx + 1, width, height, step_count);
        
        // Scan 5 seeds around current best
        for offset in -2..=2 {
            let seed = best_seed + offset;
            let output = format!("scan_{}x{}_{}.png", width, height, seed);
            let output_clone = output.clone();
            let w = *width as i32;
            let h = *height as i32;
            
            print!("  Seed {}: ", seed);
            
            let (config, mut model_config) = PresetBuilder::default()
                .preset(Preset::SDXLTurbo1_0)
                .prompt(PROMPT.to_string())
                .with_modifier(move |(mut config, model_config)| {
                    config.seed(seed);
                    config.width(w);
                    config.height(h);
                    config.steps(step_count);
                    config.output(&output_clone);
                    Ok((config, model_config))
                })
                .build()?;
            
            gen_img(&config, &mut model_config)?;
            
            // Analyze
            let description = analyze_image(&output)?;
            let score = score_text(&description);
            
            println!("score={:.1}", score);
            
            if score > best_score {
                best_score = score;
                best_seed = seed;
                println!("    ⭐ New best!");
            }
        }
        
        println!("  Best so far: seed={}, score={:.1}\n", best_seed, best_score);
    }
    
    // Phase 2: Full resolution at best seed
    println!("🎯 Final Generation");
    println!("Seed: {}", best_seed);
    println!("Size: 1024x1024");
    println!("Steps: 50\n");
    
    let final_output = format!("final_{}.png", best_seed);
    let final_clone = final_output.clone();
    let (config, mut model_config) = PresetBuilder::default()
        .preset(Preset::SDXLTurbo1_0)
        .prompt(PROMPT.to_string())
        .with_modifier(move |(mut config, model_config)| {
            config.seed(best_seed);
            config.width(1024);
            config.height(1024);
            config.steps(50);
            config.output(&final_clone);
            Ok((config, model_config))
        })
        .build()?;
    
    gen_img(&config, &mut model_config)?;
    
    let description = analyze_image(&final_output)?;
    println!("Final description:\n{}", description);
    
    Ok(())
}
