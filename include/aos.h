#pragma once

#include "vec3.h"
#include <cstddef>
#include <vector>

/// Array of Structures (AoS) - Cache Unfriendly
/// When we iterate to update positions, we skip over velocity and mass data,
/// leading to poor cache utilization

namespace aos {

struct Particle {
    Vec3 position;
    Vec3 velocity;
    float mass;

    Particle(const Vec3& pos, const Vec3& vel, float m)
        : position(pos), velocity(vel), mass(m) {}
};

class ParticleSystem {
public:
    std::vector<Particle> particles;

    explicit ParticleSystem(size_t count);

    /// Update particle positions based on velocity
    /// Poor cache behavior: we load entire Particle struct (40 bytes) but only need
    /// position (12 bytes) and velocity (12 bytes)
    void update_positions(float dt) __attribute__((noinline));

    /// Compute total kinetic energy
    /// Poor cache behavior: we access velocity and mass, skipping position data
    float compute_kinetic_energy() const __attribute__((noinline));

    /// Apply gravity to all particles
    /// Poor cache behavior: we only need to modify velocity, but load entire struct
    void apply_gravity(const Vec3& gravity, float dt) __attribute__((noinline));

    /// Full simulation update: apply forces, update positions, and compute energy
    /// This simulates a typical physics frame with multiple passes over the data
    float update(const Vec3& gravity, float dt) __attribute__((noinline));
};

} // namespace aos
