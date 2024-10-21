use criterion::{black_box, criterion_group, criterion_main, Criterion};
use intel_8080_emu_rust::emulator::cpu::emulate_8080_op;
use intel_8080_emu_rust::emulator::data_types::State8080;
use intel_8080_emu_rust::emulator::machine::SpaceInvadersMachine;

fn benchmark_emulate_8080_op(c: &mut Criterion) {
    let mut state = State8080::default();

    c.bench_function("emulate_8080_op", |b| {
        b.iter(|| emulate_8080_op(black_box(&mut state)))
    });
}

fn benchmark_16_666_cycles(c: &mut Criterion) {
    c.bench_function("emulate_8080_op", |b| {
        b.iter(|| {
            let mut state = State8080::default();
            let mut cycles: u32 = 0;
            while cycles < 16_666 {
                cycles += emulate_8080_op(black_box(&mut state)) as u32;
            }
        })
    });
}

fn emulator_benchmark(c: &mut Criterion) {
    let mut machine = SpaceInvadersMachine::new();
    c.bench_function("emulate_8080_op", |b| {
        b.iter(|| {
            machine.do_cpu();
        })
    });
}

criterion_group!(
    benches,
    benchmark_emulate_8080_op,
    benchmark_16_666_cycles,
    emulator_benchmark,
);
criterion_main!(benches);
