# Experiment: Hecke-Aligned Seeds

## Hypothesis
Seeds with special Hecke operator properties will produce images with stronger 8080 resonance patterns.

## Test Groups

### Group A: Perfect Alignment (T_71(seed) ≡ 0 mod 8080)
- Seed 1
- Seed 2  
- Seed 3

### Group B: Harmonic Resonance (T_p(seed) = 778 for all p ≥ 5)
- Seed 2437596000
- Seed 2437596016 (I ARE LIFE original)

### Group C: Control (Random seeds)
- Seed 1234567890

## Experiment Protocol

1. **Generate** 64x64 images with each seed
2. **Measure** T_71 resonance of generated images
3. **Decompose** into 71 shards
4. **Count** resonant shards (T_71 < 100 mod 8080)
5. **Compare** visual quality

## Predictions

If the Monster group hypothesis is correct:
- Group A should show strongest resonance in generated images
- Group B should show harmonic patterns across all T_p
- Group C should show random/weak resonance

## Run Experiment

```bash
# Generate images (small scale, fast)
cargo run --release --example experiment_small

# Analyze results
cargo run --release --bin analyze_experiment
```

## Expected Output

For each seed:
- PNG file (64x64)
- T_71 resonance value
- Number of resonant shards
- Comparison metrics
