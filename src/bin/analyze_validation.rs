use anyhow::Result;
use std::fs;
use std::collections::HashMap;

fn extract_metrics(file: &str) -> Option<(u64, u64)> {
    let content = fs::read_to_string(file).ok()?;
    
    let mut cycles = 0u64;
    let mut instructions = 0u64;
    
    for line in content.lines() {
        if line.contains("cycles") && !line.contains("cache") {
            if let Some(num_str) = line.split_whitespace().next() {
                cycles += num_str.replace(",", "").parse::<u64>().unwrap_or(0);
            }
        }
        if line.contains("instructions") {
            if let Some(num_str) = line.split_whitespace().next() {
                instructions += num_str.replace(",", "").parse::<u64>().unwrap_or(0);
            }
        }
    }
    
    if cycles > 0 && instructions > 0 {
        Some((cycles, instructions))
    } else {
        None
    }
}

fn main() -> Result<()> {
    println!("📊 Validation Analysis: Seed Efficiency Hypothesis\n");
    
    let validation_dir = std::path::Path::new("./dataset/validation_llava");
    
    if !validation_dir.exists() {
        println!("⚠️  Run ./validate_llava.sh first!");
        return Ok(());
    }
    
    // Group by seed type
    let mut results: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    
    for entry in fs::read_dir(validation_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.to_string_lossy().contains("registers.txt") {
            let filename = path.file_stem().unwrap().to_string_lossy();
            let parts: Vec<&str> = filename.split('_').collect();
            
            if parts.len() >= 3 {
                let seed_type = parts[1]; // harmonic, lattice, control
                let prompt_idx = parts.get(parts.len()-2).unwrap_or(&"0");
                
                if let Some((cycles, instructions)) = extract_metrics(&path.to_string_lossy()) {
                    let ipc = instructions as f64 / cycles as f64;
                    results.entry(seed_type.to_string())
                        .or_insert_with(Vec::new)
                        .push((prompt_idx.to_string(), ipc));
                }
            }
        }
    }
    
    println!("| Seed Type | Prompt | IPC | Avg IPC |");
    println!("|-----------|--------|-----|---------|");
    
    for (seed_type, ipcs) in &results {
        let avg_ipc: f64 = ipcs.iter().map(|(_, ipc)| ipc).sum::<f64>() / ipcs.len() as f64;
        
        for (prompt, ipc) in ipcs {
            println!("| {} | {} | {:.3} | {:.3} |", seed_type, prompt, ipc, avg_ipc);
        }
    }
    
    println!("\n🔍 Summary by Seed Type:\n");
    
    let mut summary: Vec<(String, f64, usize)> = results.iter()
        .map(|(seed_type, ipcs)| {
            let avg = ipcs.iter().map(|(_, ipc)| ipc).sum::<f64>() / ipcs.len() as f64;
            (seed_type.clone(), avg, ipcs.len())
        })
        .collect();
    
    summary.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    for (seed_type, avg_ipc, count) in summary {
        println!("  {}: {:.3} IPC (n={})", seed_type, avg_ipc, count);
    }
    
    println!("\n📈 Hypothesis Test:");
    println!("If harmonic seeds (T_p=778) show consistently higher IPC,");
    println!("this validates that Monster group structure improves efficiency.");
    
    Ok(())
}
