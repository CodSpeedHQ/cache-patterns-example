#include "aos.h"

namespace aos {

ParticleSystem::ParticleSystem(size_t count) {
    particles.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        float fi = static_cast<float>(i);
        particles.emplace_back(
            Vec3(fi, fi * 2.0f, fi * 3.0f),           // position
            Vec3(fi * 0.1f, fi * 0.2f, fi * 0.3f),    // velocity
            1.0f + fi * 0.01f                          // mass
        );
    }
}

void ParticleSystem::update_positions(float dt) {
    for (auto& particle : particles) {
        particle.position = particle.position.add(particle.velocity.scale(dt));
    }
}

float ParticleSystem::compute_kinetic_energy() const {
    float total = 0.0f;
    for (const auto& particle : particles) {
        const Vec3& v = particle.velocity;
        float v2 = v.x * v.x + v.y * v.y + v.z * v.z;
        total += 0.5f * particle.mass * v2;
    }
    return total;
}

void ParticleSystem::apply_gravity(const Vec3& gravity, float dt) {
    for (auto& particle : particles) {
        particle.velocity = particle.velocity.add(gravity.scale(dt));
    }
}

float ParticleSystem::update(const Vec3& gravity, float dt) {
    apply_gravity(gravity, dt);
    update_positions(dt);
    return compute_kinetic_energy();
}

} // namespace aos
