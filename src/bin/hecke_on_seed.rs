use anyhow::Result;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn apply_hecke_to_number(n: u64, p: u64) -> u64 {
    // Convert number to bytes and apply Hecke operator
    let bytes = n.to_le_bytes();
    bytes.iter().enumerate()
        .fold(0u64, |sum, (i, &b)| {
            sum.wrapping_add((b as u64).wrapping_mul(i as u64 % p))
        })
}

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
    if n > 1 { factors.push(n); }
    factors
}

fn main() -> Result<()> {
    println!("🎯 Hecke Operator T_71 Applied to Seeds\n");
    
    // I ARE LIFE seed
    let i_are_life = 2437596016u64;
    println!("🌱 I ARE LIFE Seed: {}", i_are_life);
    println!("   Prime factors: {:?}", prime_factors(i_are_life));
    
    let t71 = apply_hecke_to_number(i_are_life, 71);
    println!("   T_71(seed) = {}", t71);
    println!("   T_71 mod 8080 = {}", t71 % 8080);
    println!("   T_71 mod 71 = {}", t71 % 71);
    println!();
    
    // Apply all Monster primes
    println!("   All Hecke Operators:");
    for &p in &MONSTER_PRIMES {
        let tp = apply_hecke_to_number(i_are_life, p);
        println!("   T_{:2}(seed) = {:12} (mod 8080 = {:4})", p, tp, tp % 8080);
    }
    println!();
    
    // InvokeAI seeds
    println!("📊 InvokeAI Seeds:");
    for seed in 3673070247u64..=3673070254 {
        let factors = prime_factors(seed);
        let monster_factors: Vec<u64> = factors.iter()
            .filter(|f| MONSTER_PRIMES.contains(f))
            .copied()
            .collect();
        
        let t71 = apply_hecke_to_number(seed, 71);
        
        println!("Seed {}: T_71={} (mod 8080={})", seed, t71, t71 % 8080);
        if !monster_factors.is_empty() {
            println!("  Monster factors: {:?}", monster_factors);
        }
    }
    println!();
    
    // Check if seed itself has special properties
    println!("🔍 Special Properties:");
    println!("   {} mod 71 = {}", i_are_life, i_are_life % 71);
    println!("   {} mod 8080 = {}", i_are_life, i_are_life % 8080);
    
    // Check digit patterns
    let seed_str = i_are_life.to_string();
    println!("   Digits: {}", seed_str);
    println!("   First 4: {}", &seed_str[..4]);
    println!("   Last 4: {}", &seed_str[seed_str.len()-4..]);
    
    Ok(())
}
