/// Structure of Arrays - Cache Friendly
/// Data is organized so that accessing positions only touches position data,
/// leading to excellent cache utilization

use crate::Vec3;

pub struct ParticleSystem {
    pub positions: Vec<Vec3>,
    pub velocities: Vec<Vec3>,
    pub masses: Vec<f32>,
}

impl ParticleSystem {
    pub fn new(count: usize) -> Self {
        let mut positions = Vec::with_capacity(count);
        let mut velocities = Vec::with_capacity(count);
        let mut masses = Vec::with_capacity(count);

        for i in 0..count {
            let fi = i as f32;
            positions.push(Vec3::new(fi, fi * 2.0, fi * 3.0));
            velocities.push(Vec3::new(fi * 0.1, fi * 0.2, fi * 0.3));
            masses.push(1.0 + fi * 0.01);
        }

        Self {
            positions,
            velocities,
            masses,
        }
    }

    /// Update particle positions based on velocity
    /// Excellent cache behavior: positions and velocities are contiguous,
    /// all data in cache lines is useful
    pub fn update_positions(&mut self, dt: f32) {
        for i in 0..self.positions.len() {
            self.positions[i] = self.positions[i].add(&self.velocities[i].scale(dt));
        }
    }

    /// Compute total kinetic energy
    /// Good cache behavior: sequential access to velocities and masses
    pub fn compute_kinetic_energy(&self) -> f32 {
        let mut total = 0.0;
        for i in 0..self.velocities.len() {
            let v = &self.velocities[i];
            let v2 = v.x * v.x + v.y * v.y + v.z * v.z;
            total += 0.5 * self.masses[i] * v2;
        }
        total
    }

    /// Apply gravity to all particles
    /// Excellent cache behavior: only touching velocity array
    pub fn apply_gravity(&mut self, gravity: Vec3, dt: f32) {
        for velocity in &mut self.velocities {
            *velocity = velocity.add(&gravity.scale(dt));
        }
    }
}
