use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tls::shared::{Operand, Register};
use tls::tpu::{TPU, TpuState, create_basic_tpu_config};
use strum::EnumCount;
use tls::shared::{AnalogPin, DigitalPin};
use tls::tps::parse_program;

fn add_benchmark(c: &mut Criterion) {
    // Benchmark adding two constants
    let add_program = "ADD 5, 3\nJMP 0";
    let parsed_program = parse_program(add_program).unwrap();
    let mut tpu = create_basic_tpu_config(parsed_program);
    c.bench_function("add_constants", |b| {
        b.iter(|| {
            tpu.step();
            black_box(tpu.read_register(Register::A))
        })
    });

    let blink_program = r#"LDA 10
        LDR X, 0x5555
        PUSH A
        DPWW X
        ROL X, X, 1
        POP A
        DECA
        BEZ 9, A
        JMP 2
        JMP 0"#;
    let parsed_program = parse_program(blink_program).unwrap();
    let mut tpu = create_basic_tpu_config(parsed_program);
    c.bench_function("blink_program", |b| {
        b.iter(|| {
            tpu.step();
            black_box(tpu.read_register(Register::A));
            black_box(tpu.get_digital_pins());
        })
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);