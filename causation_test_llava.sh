#!/bin/bash

echo "🔬 Causation Test: Measuring LLaVA IPC across T_71 values"
echo ""

LLAVA_PATH="/home/mdupont/experiments/llava.cpp/llava-cli"
MODEL="/mnt/data1/models/llava/ggml-model-q4_k.gguf"
MMPROJ="/mnt/data1/models/llava/mmproj-model-f16.gguf"
PROMPT="Describe this image briefly."

echo "| Seed | T_71 | Rep | IPC | Instructions | Cycles |"
echo "|------|------|-----|-----|--------------|--------|"

for label in T71_96 T71_227 T71_454 T71_730 T71_778 T71_1283; do
    for i in 0 1 2; do
        img="causation_${label}_${i}.png"
        
        if [ ! -f "$img" ]; then
            echo "Missing: $img"
            continue
        fi
        
        perf stat -e instructions,cycles "$LLAVA_PATH" \
            -m "$MODEL" \
            --mmproj "$MMPROJ" \
            --image "$img" \
            -p "$PROMPT" \
            -n 50 > /tmp/llava_out.txt 2> /tmp/perf_out.txt
        
        instructions=$(grep instructions /tmp/perf_out.txt | awk '{print $1}' | tr -d ',')
        cycles=$(grep cycles /tmp/perf_out.txt | awk '{print $1}' | tr -d ',')
        
        if [ -n "$instructions" ] && [ -n "$cycles" ]; then
            ipc=$(echo "scale=3; $instructions / $cycles" | bc)
            seed=$(echo "$label" | cut -d_ -f2)
            echo "| $label | $seed | $i | $ipc | $instructions | $cycles |"
        fi
    done
done

echo ""
echo "✅ Analysis complete. Check for correlation between T_71 and IPC."
