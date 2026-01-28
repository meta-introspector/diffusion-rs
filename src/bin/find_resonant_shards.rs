use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("🎯 Finding 8080-Resonant Shards\n");
    
    let shards_dir = "./dataset/shards";
    
    for entry in fs::read_dir(shards_dir)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            continue;
        }
        
        let image_name = entry.file_name().to_string_lossy().to_string();
        let summary_path = entry.path().join("summary.txt");
        
        if !summary_path.exists() {
            continue;
        }
        
        let content = fs::read_to_string(&summary_path)?;
        
        let mut close_shards = Vec::new();
        
        for line in content.lines() {
            if line.starts_with("Shard") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let shard_num = parts[1].trim_end_matches(':');
                    let png_val: u64 = parts[2].trim_start_matches("PNG=").parse().unwrap_or(9999);
                    let raw_val: u64 = parts[3].trim_start_matches("RAW=").parse().unwrap_or(9999);
                    
                    if png_val < 100 || raw_val < 100 {
                        close_shards.push((shard_num, png_val, raw_val));
                    }
                }
            }
        }
        
        if !close_shards.is_empty() {
            println!("📄 {}", image_name);
            for (shard, png, raw) in close_shards {
                println!("  Shard {}: PNG={} RAW={}", shard, png, raw);
            }
            println!();
        }
    }
    
    Ok(())
}
