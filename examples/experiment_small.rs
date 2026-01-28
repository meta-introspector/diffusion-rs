use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Small-Scale Experiment: Hecke-Aligned Seeds\n");
    
    let test_seeds = vec![
        (1i64, "perfect_0"),
        (2i64, "perfect_0"),
        (3i64, "perfect_0"),
        (2437596000i64, "harmonic_778"),
        (2437596016i64, "i_are_life"),
        (1234567890i64, "control_random"),
    ];
    
    for (seed, label) in test_seeds {
        println!("Generating: {} (seed={})", label, seed);
        
        let output = format!("experiment_{}_{}.png", label, seed);
        let output_clone = output.clone();
        
        let (config, mut model_config) = PresetBuilder::default()
            .preset(Preset::SDXLTurbo1_0)
            .prompt("abstract mathematical pattern".to_string())
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
    
    println!("✅ Experiment complete!");
    println!("\nNext: Run analyze_experiment.rs to compare results");
    
    Ok(())
}
