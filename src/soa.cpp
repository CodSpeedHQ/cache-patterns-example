#include "soa.h"

namespace soa {

ParticleSystem::ParticleSystem(size_t count) {
    positions.reserve(count);
    velocities.reserve(count);
    masses.reserve(count);

    for (size_t i = 0; i < count; ++i) {
        float fi = static_cast<float>(i);
        positions.emplace_back(fi, fi * 2.0f, fi * 3.0f);
        velocities.emplace_back(fi * 0.1f, fi * 0.2f, fi * 0.3f);
        masses.push_back(1.0f + fi * 0.01f);
    }
}

void ParticleSystem::update_positions(float dt) {
    for (size_t i = 0; i < positions.size(); ++i) {
        positions[i] = positions[i].add(velocities[i].scale(dt));
    }
}

float ParticleSystem::compute_kinetic_energy() const {
    float total = 0.0f;
    for (size_t i = 0; i < velocities.size(); ++i) {
        const Vec3& v = velocities[i];
        float v2 = v.x * v.x + v.y * v.y + v.z * v.z;
        total += 0.5f * masses[i] * v2;
    }
    return total;
}

void ParticleSystem::apply_gravity(const Vec3& gravity, float dt) {
    for (auto& velocity : velocities) {
        velocity = velocity.add(gravity.scale(dt));
    }
}

float ParticleSystem::update(const Vec3& gravity, float dt) {
    apply_gravity(gravity, dt);
    update_positions(dt);
    return compute_kinetic_energy();
}

} // namespace soa
