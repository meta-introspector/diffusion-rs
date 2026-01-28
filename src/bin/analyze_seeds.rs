use anyhow::Result;
use polars::prelude::*;

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn main() -> Result<()> {
    let df = LazyFrame::scan_parquet("./dataset/invokeai_images.parquet", Default::default())?
        .collect()?;
    
    println!("🔢 Seed Analysis\n");
    
    let seeds = df.column("seed")?.i64()?;
    
    // Monster primes
    let monster_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
    
    for (idx, seed_opt) in seeds.into_iter().enumerate() {
        if let Some(seed) = seed_opt {
            let seed_u64 = seed as u64;
            let factors = prime_factors(seed_u64);
            
            println!("Image {}: seed = {}", idx + 1, seed);
            println!("  Prime factors: {:?}", factors);
            
            let monster_factors: Vec<u64> = factors.iter()
                .filter(|f| monster_primes.contains(&(**f as i32)))
                .copied()
                .collect();
            
            if !monster_factors.is_empty() {
                println!("  🎯 Monster primes: {:?}", monster_factors);
            }
            
            // Check for patterns
            let seed_str = seed.to_string();
            println!("  Digits: {}", seed_str);
            println!("  First 4: {}", &seed_str[..4.min(seed_str.len())]);
            println!("  Last 4: {}", &seed_str[seed_str.len().saturating_sub(4)..]);
            println!();
        }
    }
    
    // Analyze sequence
    let seed_vec: Vec<i64> = seeds.into_iter().filter_map(|s| s).collect();
    if seed_vec.len() > 1 {
        println!("📊 Sequence Analysis:");
        println!("  Range: {} - {}", seed_vec[0], seed_vec[seed_vec.len()-1]);
        println!("  Count: {}", seed_vec.len());
        
        let diffs: Vec<i64> = seed_vec.windows(2).map(|w| w[1] - w[0]).collect();
        println!("  Differences: {:?}", diffs);
        
        if diffs.iter().all(|&d| d == 1) {
            println!("  ✅ Sequential seeds");
        }
    }
    
    Ok(())
}
