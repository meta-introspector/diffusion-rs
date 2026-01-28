#!/bin/bash
# Quick test: Analyze 3 key images with LLaVA

echo "🔍 LLaVA Quick Test (3 images)"
echo "=============================="
echo ""

OUTPUT_DIR="./dataset/llava_analysis"
mkdir -p "$OUTPUT_DIR"

# Test on 3 key images
IMAGES=(
    "lattice_2p5_32.png"
    "lattice_2x3x7_42.png"
    "experiment_i_are_life_2437596016.png"
)

for img in "${IMAGES[@]}"; do
    if [ ! -f "$img" ]; then
        echo "⚠️  $img not found, skipping"
        continue
    fi
    
    basename=$(basename "$img" .png)
    output="$OUTPUT_DIR/${basename}_llava.txt"
    registers="$OUTPUT_DIR/${basename}_registers.txt"
    
    echo "Analyzing: $basename"
    
    # Run LLaVA with perf stat
    perf stat -e cycles,instructions,cache-references,cache-misses \
        ollama run llava "Describe this image" "$img" \
        > "$output" 2> "$registers"
    
    echo "  ✓ Description saved"
    
    # Extract key metrics
    if [ -f "$registers" ]; then
        cycles=$(grep "cycles" "$registers" | awk '{print $1}' | tr -d ',')
        instr=$(grep "instructions" "$registers" | awk '{print $1}' | tr -d ',')
        echo "  📊 Cycles: $cycles, Instructions: $instr"
    fi
    echo ""
done

echo "✅ Quick test complete!"
echo ""
echo "Run: cargo run --bin classify_llava_registers"
