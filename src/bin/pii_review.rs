use anyhow::Result;
use polars::prelude::*;
use regex::Regex;

fn main() -> Result<()> {
    let df = LazyFrame::scan_parquet("./dataset/invokeai_images.parquet", Default::default())?
        .collect()?;
    
    println!("🔍 PII Review Report\n");
    println!("Total images: {}\n", df.height());
    
    // PII patterns
    let email_re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")?;
    let phone_re = Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b")?;
    let ssn_re = Regex::new(r"\b\d{3}-\d{2}-\d{4}\b")?;
    
    let prompts = df.column("prompt")?.str()?;
    
    let mut pii_found = false;
    
    for (idx, prompt_opt) in prompts.into_iter().enumerate() {
        if let Some(prompt) = prompt_opt {
            let mut issues = Vec::new();
            
            if email_re.is_match(prompt) {
                issues.push("EMAIL");
            }
            if phone_re.is_match(prompt) {
                issues.push("PHONE");
            }
            if ssn_re.is_match(prompt) {
                issues.push("SSN");
            }
            
            if !issues.is_empty() {
                pii_found = true;
                println!("⚠️  Image {}: {}", idx, issues.join(", "));
                println!("   Prompt: {}\n", &prompt[..prompt.len().min(100)]);
            }
        }
    }
    
    if !pii_found {
        println!("✅ No PII detected in prompts");
        println!("\n📋 Sample prompts:");
        for i in 0..3.min(df.height()) {
            if let Some(prompt) = prompts.get(i) {
                let preview = prompt.chars().take(80).collect::<String>();
                println!("  {}. {}", i+1, preview);
            }
        }
    }
    
    println!("\n⚠️  Manual review needed for:");
    println!("  - Faces or identifiable people");
    println!("  - Personal addresses or locations");
    println!("  - License plates or ID numbers");
    println!("  - Handwritten personal information");
    
    Ok(())
}
