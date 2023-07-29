use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_lang::{interpreter::Interpreter, Compile};

fn interpret(line: &str) -> f32 {
    Interpreter::from_source(line).unwrap().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Interpreter bench", |b| {
        b.iter(|| {
            interpret(black_box(
                "540 - 28 * 14 ^ sin(42) * ctg(11) \\ 2 + log(100, 3.5) % 1.2",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
