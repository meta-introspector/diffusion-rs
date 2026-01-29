#!/bin/bash
# Quick validation: Test 2 images per seed type

echo "🔬 Quick Validation (6 images)"
echo "=============================="
echo ""

OUTPUT_DIR="./dataset/validation_llava"
mkdir -p "$OUTPUT_DIR"

# Sample: 1 image per seed, 2 different prompts
IMAGES=(
    "validate_harmonic_2437596016_0.png"
    "validate_harmonic_2437596016_2.png"
    "validate_lattice_2p5_32_0.png"
    "validate_lattice_2p5_32_2.png"
    "validate_control_1234567890_0.png"
    "validate_control_1234567890_2.png"
)

for img in "${IMAGES[@]}"; do
    if [ ! -f "$img" ]; then
        echo "⚠️  $img not found"
        continue
    fi
    
    basename=$(basename "$img" .png)
    output="$OUTPUT_DIR/${basename}_llava.txt"
    registers="$OUTPUT_DIR/${basename}_registers.txt"
    
    echo "Analyzing: $basename"
    
    perf stat -e cycles,instructions \
        ollama run llava "Describe this image" "$img" \
        > "$output" 2> "$registers"
    
    # Extract and display
    cycles=$(grep -oP '^\s+\K[0-9,]+(?=\s+.*cycles)' "$registers" | tr -d ',' | paste -sd+ | bc)
    instr=$(grep -oP '^\s+\K[0-9,]+(?=\s+.*instructions)' "$registers" | tr -d ',' | paste -sd+ | bc)
    
    if [ -n "$cycles" ] && [ -n "$instr" ] && [ "$cycles" -gt 0 ]; then
        ipc=$(echo "scale=3; $instr / $cycles" | bc)
        echo "  📊 Cycles: $cycles, IPC: $ipc"
    fi
    echo ""
done

echo "✅ Quick validation complete!"
echo ""
echo "Run: cargo run --bin analyze_validation"
