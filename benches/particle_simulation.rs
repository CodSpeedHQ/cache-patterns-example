use cache_patterns::{aos, soa, Vec3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const MEDIUM: usize = 1_000_000;

// ============================================================================
// Array of Structures (AoS) - Cache Unfriendly
// ============================================================================

fn aos_update_positions(c: &mut Criterion) {
    c.bench_function("aos_update_positions", |b| {
        let mut system = aos::ParticleSystem::new(MEDIUM);
        b.iter(|| {
            system.update_positions(black_box(0.016));
        });
    });
}

fn aos_kinetic_energy(c: &mut Criterion) {
    c.bench_function("aos_kinetic_energy", |b| {
        let system = aos::ParticleSystem::new(MEDIUM);
        b.iter(|| {
            black_box(system.compute_kinetic_energy());
        });
    });
}

fn aos_apply_gravity(c: &mut Criterion) {
    c.bench_function("aos_apply_gravity", |b| {
        let mut system = aos::ParticleSystem::new(MEDIUM);
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        b.iter(|| {
            system.apply_gravity(black_box(gravity), black_box(0.016));
        });
    });
}

fn aos_full_update(c: &mut Criterion) {
    c.bench_function("aos_full_update", |b| {
        let mut system = aos::ParticleSystem::new(MEDIUM);
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        let dt = 0.016;
        b.iter(|| {
            black_box(system.update(black_box(gravity), black_box(dt)));
        });
    });
}

// ============================================================================
// Structure of Arrays - Cache Friendly
// ============================================================================

fn soa_update_positions(c: &mut Criterion) {
    c.bench_function("soa_update_positions", |b| {
        let mut system = soa::ParticleSystem::new(MEDIUM);
        b.iter(|| {
            system.update_positions(black_box(0.016));
        });
    });
}

fn soa_kinetic_energy(c: &mut Criterion) {
    c.bench_function("soa_kinetic_energy", |b| {
        let system = soa::ParticleSystem::new(MEDIUM);
        b.iter(|| {
            black_box(system.compute_kinetic_energy());
        });
    });
}

fn soa_apply_gravity(c: &mut Criterion) {
    c.bench_function("soa_apply_gravity", |b| {
        let mut system = soa::ParticleSystem::new(MEDIUM);
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        b.iter(|| {
            system.apply_gravity(black_box(gravity), black_box(0.016));
        });
    });
}

fn soa_full_update(c: &mut Criterion) {
    c.bench_function("soa_full_update", |b| {
        let mut system = soa::ParticleSystem::new(MEDIUM);
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        let dt = 0.016;
        b.iter(|| {
            black_box(system.update(black_box(gravity), black_box(dt)));
        });
    });
}

criterion_group!(
    benches,
    aos_update_positions,
    aos_kinetic_energy,
    aos_apply_gravity,
    aos_full_update,
    soa_update_positions,
    soa_kinetic_energy,
    soa_apply_gravity,
    soa_full_update
);
criterion_main!(benches);
