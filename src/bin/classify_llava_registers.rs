use anyhow::Result;
use std::fs;
use std::path::Path;
use regex::Regex;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| {
            sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p))
        })
}

fn extract_register_values(content: &str) -> Vec<(String, u64)> {
    let mut values = Vec::new();
    
    // Extract cycles, instructions, cache metrics
    let patterns = vec![
        (r"(\d+)\s+cycles", "cycles"),
        (r"(\d+)\s+instructions", "instructions"),
        (r"(\d+)\s+cache-references", "cache_refs"),
        (r"(\d+)\s+cache-misses", "cache_misses"),
    ];
    
    for (pattern, name) in patterns {
        let re = Regex::new(pattern).unwrap();
        if let Some(cap) = re.captures(content) {
            if let Ok(val) = cap[1].replace(",", "").parse::<u64>() {
                values.push((name.to_string(), val));
            }
        }
    }
    
    values
}

fn main() -> Result<()> {
    println!("🎯 LLaVA Register Classification with Hecke Operators\n");
    
    let analysis_dir = Path::new("./dataset/llava_analysis");
    
    if !analysis_dir.exists() {
        println!("⚠️  Run ./analyze_with_llava.sh first!");
        return Ok(());
    }
    
    println!("| Image | Cycles | Instructions | T_71(cycles) | T_71(instr) | Ratio |");
    println!("|-------|--------|--------------|--------------|-------------|-------|");
    
    let mut all_results = Vec::new();
    
    for entry in fs::read_dir(analysis_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |e| e == "txt") 
            && path.to_string_lossy().contains("registers") {
            
            let content = fs::read_to_string(&path)?;
            let registers = extract_register_values(&content);
            
            let filename = path.file_stem().unwrap().to_string_lossy();
            let image_name = filename.trim_end_matches("_registers");
            
            let mut cycles = 0u64;
            let mut instructions = 0u64;
            
            for (name, val) in &registers {
                match name.as_str() {
                    "cycles" => cycles = *val,
                    "instructions" => instructions = *val,
                    _ => {}
                }
            }
            
            if cycles > 0 && instructions > 0 {
                let t71_cycles = apply_hecke_to_number(cycles, 71);
                let t71_instr = apply_hecke_to_number(instructions, 71);
                let ratio = cycles as f64 / instructions as f64;
                
                println!("| {} | {} | {} | {} | {} | {:.2} |",
                         image_name, cycles, instructions, 
                         t71_cycles % 8080, t71_instr % 8080, ratio);
                
                all_results.push((image_name.to_string(), cycles, instructions, 
                                 t71_cycles % 8080, t71_instr % 8080, ratio));
            }
        }
    }
    
    println!("\n🔍 Classification by T_71 Resonance:\n");
    
    // Sort by T_71(cycles) resonance
    all_results.sort_by_key(|(_, _, _, t71_c, _, _)| *t71_c);
    
    println!("Top 10 by T_71(cycles) resonance:");
    for (name, cycles, instr, t71_c, t71_i, ratio) in all_results.iter().take(10) {
        println!("  {}: T_71={} (cycles={}, ratio={:.2})", name, t71_c, cycles, ratio);
    }
    
    println!("\n📊 Hecke Operator Analysis:");
    
    // Apply all Monster primes to top result
    if let Some((name, cycles, _, _, _, _)) = all_results.first() {
        println!("\nBest resonance: {}", name);
        println!("Cycles: {}", cycles);
        println!("\nAll Hecke operators:");
        for &p in &MONSTER_PRIMES {
            let tp = apply_hecke_to_number(*cycles, p);
            println!("  T_{}(cycles) = {} (mod 8080 = {})", p, tp, tp % 8080);
        }
    }
    
    Ok(())
}
