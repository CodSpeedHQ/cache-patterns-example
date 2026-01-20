# Cache Patterns Benchmark

This crate demonstrates the performance impact of different data layouts on CPU cache utilization through a particle physics simulation.

## Initial Assumption

**Hypothesis**: Data layout significantly impacts CPU cache behavior. Specifically, organizing data as a Structure of Arrays (SoA) should show measurably better cache performance than Array of Structures (AoS) when operations only access a subset of fields.

This benchmark is designed to validate this hypothesis using CodSpeed's walltime instrument, which provides hardware performance counters including cache hit/miss rates, memory bandwidth, and IPC (instructions per cycle).

## The Problem: Array of Structures (AoS) vs Structure of Arrays (SoA)

### Array of Structures (AoS) - Cache Unfriendly
```rust
struct Particle {
    position: Vec3,  // 12 bytes
    velocity: Vec3,  // 12 bytes
    mass: f32,       // 4 bytes
}                    // = 28 bytes per particle (40 with padding)

particles: Vec<Particle>
```

**Memory layout**: `[pos0, vel0, mass0, pos1, vel1, mass1, pos2, vel2, mass2, ...]`

When we only need to update positions, we load entire cache lines containing velocity and mass data that we don't use, wasting bandwidth and cache space.

### Structure of Arrays (SoA) - Cache Friendly
```rust
struct ParticleSystem {
    positions: Vec<Vec3>,
    velocities: Vec<Vec3>,
    masses: Vec<f32>,
}
```

**Memory layout**:
- `positions: [pos0, pos1, pos2, ...]`
- `velocities: [vel0, vel1, vel2, ...]`
- `masses: [mass0, mass1, mass2, ...]`

When we update positions, every byte in the cache line is useful data, maximizing cache efficiency.

## Expected Performance Characteristics

### AoS (Cache Unfriendly)
- Higher L1/L2/L3 cache miss rates
- Lower memory bandwidth utilization
- More stalls waiting for memory

### SoA (Cache Friendly)
- Lower cache miss rates (better spatial locality)
- Higher effective memory bandwidth
- Better prefetcher efficiency

## Running the Benchmarks

```bash
# Run with standard benchmarking
cargo bench

# Run with CodSpeed profiling to see cache counters
# (requires CodSpeed setup with walltime instrument)
codspeed run cargo bench
```

## What to Look For in CodSpeed Profiling

When comparing AoS vs SoA versions with CodSpeed's walltime instrument, you should see:

1. **Cache Misses**: SoA should show significantly fewer L1/L2/L3 cache misses
2. **Memory Operations**: Better cache line utilization in SoA version
3. **Instructions Per Cycle (IPC)**: Higher IPC in SoA due to less memory stalls
4. **Wall Time**: SoA should be faster, especially with larger datasets

## Benchmark Operations

Each version implements three operations:

1. **update_positions**: `position = position + velocity * dt`
   - Tests spatial locality when accessing two arrays

2. **compute_kinetic_energy**: `sum(0.5 * mass * velocity²)`
   - Tests cache behavior when skipping position data

3. **apply_gravity**: `velocity = velocity + gravity * dt`
   - Tests cache behavior when accessing only one field

## Dataset Sizes

- **Small**: 1,000 particles (~40 KB for AoS, ~32 KB for SoA)
- **Medium**: 10,000 particles (~400 KB for AoS, ~320 KB for SoA)
- **Large**: 100,000 particles (~4 MB for AoS, ~3.2 MB for SoA)

Different sizes stress different cache levels (L1/L2/L3).
