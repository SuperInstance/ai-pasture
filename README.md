# ai-pasture

Farming and agriculture simulation layer for the Lau game

## Overview

Part of the [Flux→PTX](https://github.com/SuperInstance/cuda-oxide/blob/main/FLUX_TO_PTX.md) experimental suite — building a distributed GPU runtime on five layers:

## Architecture

This crate sits within the **five-layer Oxide Stack**:

| Layer | Crate | Role |
|-------|-------|------|
| 1 | open-parallel | Async runtime (tokio fork) |
| 2 | pincher | "Vector DB as runtime, LLM as compiler" |
| 3 | flux-core | Bytecode VM + A2A agent protocol |
| 4 | cuda-oxide | Flux→MIR→Pliron→NVVM→PTX compiler |
| 5 | cudaclaw | Persistent GPU kernels, warp consensus, SmartCRDT |

The key insight: **ternary values {-1, 0, +1} map directly to GPU compute**. They pack 16× denser than FP32, enable XNOR+popcount matmul, and conservation laws become compile-time checks.

## Stats

| Metric | Value |
|--------|-------|
| Tests | 45 |
| Lines of Code | 955 |
| Public API Surface | 35 items |
| License | MIT |

## Installation

```toml
[dependencies]
ai-pasture = "0.1.0"
```

## Usage

```rust
use ai_pasture::*;
// See src/lib.rs tests for complete working examples
```

### Key Types

```
- pub struct Soil {
    pub fn new() -> Self {
    pub fn fertility(&self) -> f64 {
    pub fn deplete(&mut self, amounts: &Soil) {
    pub fn replenish(&mut self, amounts: &Soil) {
    pub fn tick(&mut self) {
- pub enum CropType {
    pub fn nutrient_need(&self) -> Soil {
    pub fn growth_time(&self) -> u64 {
    pub fn yield_amount(&self) -> u32 {
```

## Design Philosophy

This crate uses **ternary algebra** (Z₃) where every value is {-1, 0, +1}:

- **+1** → positive signal (healthy, allocated, converged, ready)
- **0** → neutral (pending, balanced, monitoring, degraded)
- **-1** → negative signal (failed, free, diverged, overloaded)

This isn't arbitrary — ternary is the natural encoding for:
1. **BitNet b1.58** (Microsoft) — ternary neural networks at 60% less power
2. **GPU warp voting** — hardware ballot instructions return ternary consensus
3. **Conservation laws** — {-1, 0, +1} preserves quantity (what goes in must come out)

## Testing

```bash
git clone https://github.com/SuperInstance/ai-pasture.git
cd ai-pasture
cargo test
```

## License

MIT
