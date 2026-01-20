# Cache Patterns Benchmark

This project demonstrates the performance impact of different data layouts on CPU cache utilization through a particle physics simulation implemented in C++.

## Initial Assumption

**Hypothesis**: Data layout significantly impacts CPU cache behavior. Specifically, organizing data as a Structure of Arrays (SoA) should show measurably better cache performance than Array of Structures (AoS) when operations only access a subset of fields.

This benchmark is designed to validate this hypothesis using CodSpeed's walltime instrument, which provides hardware performance counters including cache hit/miss rates, memory bandwidth, and IPC (instructions per cycle).

## The Problem: Array of Structures (AoS) vs Structure of Arrays (SoA)

### Array of Structures (AoS) - Cache Unfriendly
```cpp
struct Particle {
    Vec3 position;  // 12 bytes
    Vec3 velocity;  // 12 bytes
    float mass;     // 4 bytes
};                  // = 28 bytes per particle (40 with padding)

std::vector<Particle> particles;
```

**Memory layout**: `[pos0, vel0, mass0, pos1, vel1, mass1, pos2, vel2, mass2, ...]`

When we only need to update positions, we load entire cache lines containing velocity and mass data that we don't use, wasting bandwidth and cache space.

### Structure of Arrays (SoA) - Cache Friendly
```cpp
class ParticleSystem {
public:
    std::vector<Vec3> positions;
    std::vector<Vec3> velocities;
    std::vector<float> masses;
};
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

## Building and Running the Benchmarks

### Prerequisites

- CMake 3.12 or higher
- C++17 compatible compiler
- Build tools (make, gcc/clang)

### Build Instructions

```bash
# Create build directory
mkdir build && cd build

# Configure with CodSpeed simulation mode for profiling
cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCODSPEED_MODE=simulation ..

# Build
make -j$(nproc)

# Run benchmarks
./particle_simulation_bench
```

### Build Modes

CodSpeed supports different modes via the `CODSPEED_MODE` CMake option:

- `off` (default): Disables CodSpeed instrumentation
- `simulation`: Runs benchmarks on simulated CPU with hardware counters
- `walltime`: For walltime reports

Example:
```bash
# Build for walltime profiling
cmake -DCODSPEED_MODE=walltime ..
make
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

## Dataset Size

The benchmarks use **1,000,000 particles**:
- **AoS**: ~40 MB (28 bytes per particle + padding)
- **SoA**: ~32 MB (separate arrays for positions, velocities, masses)

This size is large enough to exceed L3 cache on most systems, making cache efficiency differences clearly visible.

## Project Structure

```
cache-patterns-example/
├── include/
│   ├── vec3.h          # 3D vector utility
│   ├── aos.h           # Array of Structures header
│   └── soa.h           # Structure of Arrays header
├── src/
│   ├── aos.cpp         # AoS implementation
│   └── soa.cpp         # SoA implementation
├── benchmarks/
│   └── particle_simulation.cpp  # Google Benchmark benchmarks
├── CMakeLists.txt      # CMake build configuration
└── .github/workflows/
    └── codspeed.yml    # CI workflow for CodSpeed
```

## Continuous Integration

The project includes a GitHub Actions workflow that automatically runs benchmarks on every push and pull request using CodSpeed. The workflow:

1. Builds the project with CMake in RelWithDebInfo mode
2. Compiles with CODSPEED_MODE=simulation for hardware counter profiling
3. Runs benchmarks and reports performance metrics
4. Detects performance regressions automatically

View the workflow configuration in `.github/workflows/codspeed.yml`.
