use aoc2021::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark_day(c: &mut Criterion) {
    for day in 1..=11 {
        let input_path = format!("input/day_{:02}.txt", day);
        let runner = get_runner(day, input_path).unwrap();

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

criterion_group!(benches, criterion_benchmark_day);
criterion_main!(benches);
