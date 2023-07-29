use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_lang::{
    vm::{bytecode::Bytecode, StackVM, VM},
    Compile,
};

fn exec(bytecode: &[u8]) -> f32 {
    let mut vm = VM::new(bytecode);
    while let Ok(flag) = vm.exec_next() {
        if !flag {
            break;
        }
    }
    vm.pop()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("VM bench", |b| {
        let bytecode =
        Bytecode::from_source("540 - 28 * 14 ^ sin(42) * ctg(11) \\ 2 + log(100, 3.5) % 1.2")
            .unwrap()
            .compile();
        b.iter(|| exec(black_box(&bytecode)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);