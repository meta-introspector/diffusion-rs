use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};

const MONSTER_PRIMES: [u32; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Monster Group Image Generation");
    println!("==================================\n");
    
    let prompts = vec![
        "abstract visualization of the Monster group, mathematical beauty, sacred geometry",
        "71 prime numbers glowing in harmonic pattern, Monster group structure",
        "Hecke operator T_71, modular forms, mathematical symmetry, golden ratio",
        "hypercube with 357,911 glowing points, each representing prime factorization",
        "automorphic orbit, self-similar fractal, Monster group resonance",
    ];
    
    for (i, prompt) in prompts.iter().enumerate() {
        let prime = MONSTER_PRIMES[i % MONSTER_PRIMES.len()];
        
        println!("[{}/5] Generating T_{}...", i+1, prime);
        println!("Prompt: {}", prompt);
        
        let (config, mut model_config) = PresetBuilder::default()
            .preset(Preset::SDXLTurbo1_0)
            .prompt(prompt.to_string())
            .build()?;
        
        gen_img(&config, &mut model_config)?;
        
        // Rename output to monster_T_{prime}.png
        std::fs::rename("output.png", format!("monster_T_{}.png", prime))?;
        println!("   ✓ Saved: monster_T_{}.png\n", prime);
    }
    
    println!("✅ Generated 5 Monster images!");
    
    Ok(())
}
