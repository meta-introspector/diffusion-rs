use anyhow::Result;
use polars::prelude::*;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct ImageMetadata {
    seed: Option<u64>,
    positive_prompt: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    steps: Option<u32>,
    guidance: Option<f32>,
    generation_mode: Option<String>,
}

fn main() -> Result<()> {
    let db_path = "/mnt/data1/invokeai/databases/invokeai.db";
    let images_dir = "/mnt/data1/invokeai/outputs/images";
    let output_path = "./dataset/invokeai_images.parquet";

    println!("📊 Extracting InvokeAI data...");
    
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT image_name, metadata, width, height FROM images WHERE metadata IS NOT NULL")?;
    
    let mut image_names = Vec::new();
    let mut seeds = Vec::new();
    let mut prompts = Vec::new();
    let mut widths = Vec::new();
    let mut heights = Vec::new();
    let mut steps_vec = Vec::new();
    let mut guidance_vec = Vec::new();
    let mut modes = Vec::new();
    let mut image_paths = Vec::new();
    
    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let metadata_str: String = row.get(1)?;
        let width: i32 = row.get(2)?;
        let height: i32 = row.get(3)?;
        Ok((name, metadata_str, width, height))
    })?;
    
    for row in rows {
        let (name, metadata_str, width, height) = row?;
        let image_path = format!("{}/{}", images_dir, name);
        
        if !Path::new(&image_path).exists() {
            continue;
        }
        
        let metadata: ImageMetadata = serde_json::from_str(&metadata_str).unwrap_or(ImageMetadata {
            seed: None,
            positive_prompt: None,
            width: Some(width as u32),
            height: Some(height as u32),
            steps: None,
            guidance: None,
            generation_mode: None,
        });
        
        image_names.push(name);
        seeds.push(metadata.seed.map(|s| s as i64));
        prompts.push(metadata.positive_prompt);
        widths.push(metadata.width.map(|w| w as i32).unwrap_or(width));
        heights.push(metadata.height.map(|h| h as i32).unwrap_or(height));
        steps_vec.push(metadata.steps.map(|s| s as i32));
        guidance_vec.push(metadata.guidance.map(|g| g as f64));
        modes.push(metadata.generation_mode);
        image_paths.push(image_path);
    }
    
    println!("✅ Found {} images with metadata", image_names.len());
    
    let df = DataFrame::new(vec![
        Column::Series(Series::new("image_name".into(), image_names)),
        Column::Series(Series::new("image_path".into(), image_paths)),
        Column::Series(Series::new("seed".into(), seeds)),
        Column::Series(Series::new("prompt".into(), prompts)),
        Column::Series(Series::new("width".into(), widths)),
        Column::Series(Series::new("height".into(), heights)),
        Column::Series(Series::new("steps".into(), steps_vec)),
        Column::Series(Series::new("guidance".into(), guidance_vec)),
        Column::Series(Series::new("mode".into(), modes)),
    ])?;
    
    println!("💾 Writing to parquet: {}", output_path);
    
    let mut file = std::fs::File::create(output_path)?;
    ParquetWriter::new(&mut file).finish(&mut df.clone())?;
    
    println!("✅ Dataset created successfully!");
    println!("📈 Shape: {:?}", df.shape());
    
    Ok(())
}
