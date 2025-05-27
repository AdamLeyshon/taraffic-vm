use criterion::{Criterion, black_box, criterion_group, criterion_main};
use tls::shared::Register;
use tls::rgal::parse_program;
use tls::tpu::create_basic_tpu_config;

fn add_benchmark(c: &mut Criterion) {
    // Benchmark adding two constants
    let add_program = "LDR X, 5\nLDR Y,3\nADD X, Y\nJMP 2";
    let parsed_program = parse_program(add_program).unwrap();
    let mut tpu = create_basic_tpu_config(parsed_program);
    c.bench_function("add_constants", |b| {
        b.iter(|| {
            tpu.step();
            black_box(tpu.read_register(Register::A))
        })
    });

    let blink_program = r#"
        LDR A, 10
        LDR X, 0x5555
        PUSH A
        DPWW X
        ROL X, X, 1
        POP A
        DECA
        BEZ 9, A
        JMP 2
        LDR A, 255"#;
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
