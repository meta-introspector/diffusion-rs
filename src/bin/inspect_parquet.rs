use anyhow::Result;
use polars::prelude::*;

fn main() -> Result<()> {
    let df = LazyFrame::scan_parquet("./dataset/invokeai_images.parquet", Default::default())?
        .collect()?;
    
    println!("{}", df);
    
    Ok(())
}
