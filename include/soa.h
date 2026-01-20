#pragma once

#include "vec3.h"
#include <cstddef>
#include <vector>

/// Structure of Arrays - Cache Friendly
/// Data is organized so that accessing positions only touches position data,
/// leading to excellent cache utilization

namespace soa {

class ParticleSystem {
public:
    std::vector<Vec3> positions;
    std::vector<Vec3> velocities;
    std::vector<float> masses;

    explicit ParticleSystem(size_t count);

    /// Update particle positions based on velocity
    /// Excellent cache behavior: positions and velocities are contiguous,
    /// all data in cache lines is useful
    void update_positions(float dt) __attribute__((noinline));

    /// Compute total kinetic energy
    /// Good cache behavior: sequential access to velocities and masses
    float compute_kinetic_energy() const __attribute__((noinline));

    /// Apply gravity to all particles
    /// Excellent cache behavior: only touching velocity array
    void apply_gravity(const Vec3& gravity, float dt) __attribute__((noinline));

    /// Full simulation update: apply forces, update positions, and compute energy
    /// This simulates a typical physics frame with multiple passes over the data
    float update(const Vec3& gravity, float dt) __attribute__((noinline));
};

} // namespace soa
