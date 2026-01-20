use cache_patterns::{aos, soa, Vec3};

fn main() {
    divan::main();
}

const SMALL: usize = 100_000;
const MEDIUM: usize = 1_000_000;
const LARGE: usize = 10_000_000;

// ============================================================================
// Array of Structures (AoS) - Cache Unfriendly
// ============================================================================

#[divan::bench(args = [SMALL, MEDIUM, LARGE])]
fn aos_update_positions(bencher: divan::Bencher, count: usize) {
    bencher
        .with_inputs(|| aos::ParticleSystem::new(count))
        .bench_values(|mut system| {
            system.update_positions(0.016);
        });
}

#[divan::bench(args = [SMALL, MEDIUM, LARGE])]
fn aos_kinetic_energy(bencher: divan::Bencher, count: usize) {
    bencher
        .with_inputs(|| aos::ParticleSystem::new(count))
        .bench_values(|system| {
            divan::black_box(system.compute_kinetic_energy());
        });
}

#[divan::bench(args = [SMALL, MEDIUM, LARGE])]
fn aos_apply_gravity(bencher: divan::Bencher, count: usize) {
    bencher
        .with_inputs(|| aos::ParticleSystem::new(count))
        .bench_values(|mut system| {
            system.apply_gravity(Vec3::new(0.0, -9.81, 0.0), 0.016);
        });
}

// ============================================================================
// Structure of Arrays - Cache Friendly
// ============================================================================

#[divan::bench(args = [SMALL, MEDIUM, LARGE])]
fn soa_update_positions(bencher: divan::Bencher, count: usize) {
    bencher
        .with_inputs(|| soa::ParticleSystem::new(count))
        .bench_values(|mut system| {
            system.update_positions(0.016);
        });
}

#[divan::bench(args = [SMALL, MEDIUM, LARGE])]
fn soa_kinetic_energy(bencher: divan::Bencher, count: usize) {
    bencher
        .with_inputs(|| soa::ParticleSystem::new(count))
        .bench_values(|system| {
            divan::black_box(system.compute_kinetic_energy());
        });
}

#[divan::bench(args = [SMALL, MEDIUM, LARGE])]
fn soa_apply_gravity(bencher: divan::Bencher, count: usize) {
    bencher
        .with_inputs(|| soa::ParticleSystem::new(count))
        .bench_values(|mut system| {
            system.apply_gravity(Vec3::new(0.0, -9.81, 0.0), 0.016);
        });
}
