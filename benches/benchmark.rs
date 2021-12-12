use aoc2021::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    for day in 1..=25 {
        let input_path = format!("input/day_{:02}.txt", day);
        let runner = match get_runner(day, input_path) {
            Ok(r) => r,
            Err(_) => continue,
        };

        c.bench_function(&format!("day {:02} part 1", day), |b| {
            b.iter(|| {
                runner.part_1();
            })
        });
        c.bench_function(&format!("day {:02} part 2", day), |b| {
            b.iter(|| {
                runner.part_2();
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
