#!/bin/bash
# Apply LLaVA vision model to all images and capture register traces

echo "🔍 LLaVA Analysis + Register Tracing"
echo "====================================="
echo ""

OUTPUT_DIR="./dataset/llava_analysis"
mkdir -p "$OUTPUT_DIR"

# Find all PNG images
IMAGES=(
    dataset/images/*.png
    dataset/i_are_life/*.png
    dataset/lattice/*.png
    experiment_*.png
    lattice_*.png
)

for img in "${IMAGES[@]}"; do
    if [ ! -f "$img" ]; then
        continue
    fi
    
    basename=$(basename "$img" .png)
    output="$OUTPUT_DIR/${basename}_llava.txt"
    registers="$OUTPUT_DIR/${basename}_registers.txt"
    
    echo "Analyzing: $basename"
    
    # Run LLaVA with register tracing
    perf stat -e cycles,instructions,cache-references,cache-misses \
        ollama run llava "Describe this image in detail, focusing on mathematical patterns, symmetry, and structure" "$img" \
        > "$output" 2> "$registers"
    
    echo "  ✓ Saved: $output"
    echo "  ✓ Registers: $registers"
    echo ""
done

echo "✅ LLaVA analysis complete!"
echo ""
echo "Next: cargo run --bin classify_llava_registers"
