/// Array of Structures (AoS) - Cache Unfriendly
/// When we iterate to update positions, we skip over velocity and mass data,
/// leading to poor cache utilization

use crate::Vec3;

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            let fi = i as f32;
            particles.push(Particle::new(
                Vec3::new(fi, fi * 2.0, fi * 3.0),
                Vec3::new(fi * 0.1, fi * 0.2, fi * 0.3),
                1.0 + fi * 0.01,
            ));
        }
        Self { particles }
    }

    /// Update particle positions based on velocity
    /// Poor cache behavior: we load entire Particle struct (40 bytes) but only need
    /// position (12 bytes) and velocity (12 bytes)
    #[inline(never)]
    pub fn update_positions(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.position = particle.position.add(&particle.velocity.scale(dt));
        }
    }

    /// Compute total kinetic energy
    /// Poor cache behavior: we access velocity and mass, skipping position data
    #[inline(never)]
    pub fn compute_kinetic_energy(&self) -> f32 {
        let mut total = 0.0;
        for particle in &self.particles {
            let v2 = particle.velocity.x * particle.velocity.x
                + particle.velocity.y * particle.velocity.y
                + particle.velocity.z * particle.velocity.z;
            total += 0.5 * particle.mass * v2;
        }
        total
    }

    /// Apply gravity to all particles
    /// Poor cache behavior: we only need to modify velocity, but load entire struct
    #[inline(never)]
    pub fn apply_gravity(&mut self, gravity: Vec3, dt: f32) {
        for particle in &mut self.particles {
            particle.velocity = particle.velocity.add(&gravity.scale(dt));
        }
    }
}
