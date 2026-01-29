#!/bin/bash
# Validate LLaVA efficiency hypothesis across different prompts

echo "🔬 Validation: LLaVA Efficiency Analysis"
echo "========================================"
echo ""

OUTPUT_DIR="./dataset/validation_llava"
mkdir -p "$OUTPUT_DIR"

# Find all validation images
for img in validate_*.png; do
    if [ ! -f "$img" ]; then
        continue
    fi
    
    basename=$(basename "$img" .png)
    output="$OUTPUT_DIR/${basename}_llava.txt"
    registers="$OUTPUT_DIR/${basename}_registers.txt"
    
    echo "Analyzing: $basename"
    
    # Run LLaVA with perf stat
    perf stat -e cycles,instructions \
        ollama run llava "Describe this image briefly" "$img" \
        > "$output" 2> "$registers"
    
    # Extract metrics
    if [ -f "$registers" ]; then
        cycles=$(grep -E "^\s+[0-9,]+" "$registers" | grep "cycles" | head -1 | awk '{print $1}' | tr -d ',')
        instr=$(grep -E "^\s+[0-9,]+" "$registers" | grep "instructions" | head -1 | awk '{print $1}' | tr -d ',')
        
        if [ -n "$cycles" ] && [ -n "$instr" ]; then
            ipc=$(echo "scale=3; $instr / $cycles" | bc)
            echo "  📊 Cycles: $cycles, Instructions: $instr, IPC: $ipc"
        fi
    fi
    echo ""
done

echo "✅ Validation complete!"
echo ""
echo "Run: cargo run --bin analyze_validation"
