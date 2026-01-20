#include "aos.h"
#include "soa.h"
#include <benchmark/benchmark.h>

constexpr size_t MEDIUM = 1'000'000;

// ============================================================================
// Array of Structures (AoS) - Cache Unfriendly
// ============================================================================

static void aos_update_positions(benchmark::State& state) {
    aos::ParticleSystem system(MEDIUM);
    float dt = 0.016f;

    for (auto _ : state) {
        system.update_positions(dt);
        benchmark::DoNotOptimize(system.particles.data());
        benchmark::ClobberMemory();
    }
}
BENCHMARK(aos_update_positions);

static void aos_kinetic_energy(benchmark::State& state) {
    aos::ParticleSystem system(MEDIUM);

    for (auto _ : state) {
        float energy = system.compute_kinetic_energy();
        benchmark::DoNotOptimize(energy);
    }
}
BENCHMARK(aos_kinetic_energy);

static void aos_apply_gravity(benchmark::State& state) {
    aos::ParticleSystem system(MEDIUM);
    Vec3 gravity(0.0f, -9.81f, 0.0f);
    float dt = 0.016f;

    for (auto _ : state) {
        system.apply_gravity(gravity, dt);
        benchmark::DoNotOptimize(system.particles.data());
        benchmark::ClobberMemory();
    }
}
BENCHMARK(aos_apply_gravity);

static void aos_full_update(benchmark::State& state) {
    aos::ParticleSystem system(MEDIUM);
    Vec3 gravity(0.0f, -9.81f, 0.0f);
    float dt = 0.016f;

    for (auto _ : state) {
        float energy = system.update(gravity, dt);
        benchmark::DoNotOptimize(energy);
        benchmark::DoNotOptimize(system.particles.data());
        benchmark::ClobberMemory();
    }
}
BENCHMARK(aos_full_update);

// ============================================================================
// Structure of Arrays - Cache Friendly
// ============================================================================

static void soa_update_positions(benchmark::State& state) {
    soa::ParticleSystem system(MEDIUM);
    float dt = 0.016f;

    for (auto _ : state) {
        system.update_positions(dt);
        benchmark::DoNotOptimize(system.positions.data());
        benchmark::ClobberMemory();
    }
}
BENCHMARK(soa_update_positions);

static void soa_kinetic_energy(benchmark::State& state) {
    soa::ParticleSystem system(MEDIUM);

    for (auto _ : state) {
        float energy = system.compute_kinetic_energy();
        benchmark::DoNotOptimize(energy);
    }
}
BENCHMARK(soa_kinetic_energy);

static void soa_apply_gravity(benchmark::State& state) {
    soa::ParticleSystem system(MEDIUM);
    Vec3 gravity(0.0f, -9.81f, 0.0f);
    float dt = 0.016f;

    for (auto _ : state) {
        system.apply_gravity(gravity, dt);
        benchmark::DoNotOptimize(system.velocities.data());
        benchmark::ClobberMemory();
    }
}
BENCHMARK(soa_apply_gravity);

static void soa_full_update(benchmark::State& state) {
    soa::ParticleSystem system(MEDIUM);
    Vec3 gravity(0.0f, -9.81f, 0.0f);
    float dt = 0.016f;

    for (auto _ : state) {
        float energy = system.update(gravity, dt);
        benchmark::DoNotOptimize(energy);
        benchmark::DoNotOptimize(system.positions.data());
        benchmark::ClobberMemory();
    }
}
BENCHMARK(soa_full_update);

BENCHMARK_MAIN();
