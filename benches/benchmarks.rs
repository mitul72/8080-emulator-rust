use criterion::{black_box, criterion_group, criterion_main, Criterion};
use intel_8080_emu_rust::emulator::cpu::emulate_8080_op;
use intel_8080_emu_rust::emulator::data_types::State8080;

fn benchmark_emulate_8080_op(c: &mut Criterion) {
    let mut state = State8080::default();

    c.bench_function("emulate_8080_op", |b| {
        b.iter(|| emulate_8080_op(black_box(&mut state)))
    });
}

criterion_group!(benches, benchmark_emulate_8080_op);
criterion_main!(benches);
