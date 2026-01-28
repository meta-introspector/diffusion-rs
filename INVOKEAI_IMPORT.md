# InvokeAI Dataset Import

## Summary

Successfully imported InvokeAI images and metadata into Parquet format and pushed to HuggingFace.

## Dataset

- **Location:** https://huggingface.co/datasets/introspector/introspector-images
- **Format:** Parquet
- **Records:** 8 images
- **Source:** `/mnt/data1/invokeai/databases/invokeai.db`

## Schema

| Column | Type | Description |
|--------|------|-------------|
| image_name | str | UUID filename |
| image_path | str | Full path to PNG file |
| seed | i64 | Generation seed |
| prompt | str | Positive prompt |
| width | i32 | Image width |
| height | i32 | Image height |
| steps | i32 | Diffusion steps |
| guidance | f64 | Guidance scale |
| mode | str | Generation mode (e.g., flux_txt2img) |

## Seeds Captured

- 3673070247 - 3673070254 (sequential)

## Tools

- `cargo run --bin invokeai_import` - Extract from SQLite to Parquet
- `cargo run --bin inspect_parquet` - View dataset contents

## Next Steps

- Expand to all InvokeAI images (currently only 8 samples)
- Add image embeddings
- Connect to Monster group seed analysis
