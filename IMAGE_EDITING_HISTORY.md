# Image Editing & Processing History (Reverse Chronological Order)

## Recent Commits (2026)

### 🎯 Exact seed: 2437596016 (9d1eda4)
- Latest commit focusing on exact seed reproduction

### 🔍 Adaptive scanning implementation (a1dc511)
- Implemented adaptive scanning for seed search

### 🔧 Fix: Use i64 for exact seed 2437596016 (b4bede6)
- Fixed data type for seed handling

### 🌱 I ARE LIFE: Pure Rust Implementation (45700cc)
- Pure Rust implementation (likely related to generation logic)

### Add Monster generator (af18d64)
- Added Monster generator functionality

---

## 2025 Commits

### feat: twinflow z-image turbo exp preset (dead372)
- Added experimental twinflow z-image turbo preset

### feat: embed gen parameters into image metadata (c8c7665)
- Embedded generation parameters into image EXIF metadata
- Allows tracking of generation settings

### feat(cli): batching image gen support (41f7d0e, 297d3d7)
- Added batch image generation support to CLI
- Enables generating multiple images in one run

### feat: added scm mask presets (f0a1865) ⭐ IMAGE EDITING
**Key commit for image editing capabilities**
- Added Steps Computation Mask (SCM) presets for caching
- Presets: Slow, Medium, Fast, Ultra, Custom
- Mask controls which diffusion steps are computed vs cached
- Format: "1,1,1,1,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,1" (1=compute, 0=cache)
- Enables performance optimization for image generation
- Modified: src/api.rs (+228/-106 lines)

### Feat ovis image (4f39042, 0564834)
- Added Ovis image-to-text model preset
- Vision model for image understanding

### feat: qwenImage (6880da4, 31722e7)
- Added QwenImage vision model support
- Image-to-text capabilities
- Modified preset.rs and preset_builder.rs

### feat: flux2 and zimageturbo presets (0e57451)
- Added Flux2 and Z-Image Turbo presets
- Z-Image Turbo: fast image generation (steps changed from 20 to 9)

### feat: z image turbo preset (7f8b379)
- Initial Z-Image Turbo preset implementation

### feat: 2024 edition (404821b, 85a1db6)
- Updated to Rust 2024 edition

### feat: removed stb_image dependency (5210887)
- Removed external image library dependency
- Likely moved to native Rust image handling

---

## Earlier History

### feat: txt2img implemented (5b52131)
- Initial text-to-image implementation
- Foundation for all image generation

### feat: first functional draft (915e72e)
- First working version of the library

### feat: diffusion.cpp bindings (1325035)
- Initial bindings to stable-diffusion.cpp

### feat: binding crate (51c4859)
- Created initial binding crate structure

### Initial commit (c59617b)
- Project initialization

---

## Image Editing Capabilities Summary

### Direct Image Editing Features:
1. **SCM Mask Presets** (f0a1865) - Controls computation vs caching in diffusion steps
2. **Vision Models** - QwenImage, Ovis (image understanding/captioning)
3. **Batch Generation** - Multiple image generation
4. **Metadata Embedding** - EXIF data with generation parameters

### Missing Traditional Image Editing:
- ❌ No img2img (image-to-image) implementation found
- ❌ No inpainting (masked editing) found
- ❌ No outpainting found
- ❌ No ControlNet integration found
- ❌ No direct image manipulation APIs

### Current Focus:
- Text-to-image generation (txt2img)
- Model presets (SDXL, Flux2, SD3.5, etc.)
- Performance optimization (caching, batching)
- Vision models (image understanding)
- Seed control and reproducibility
