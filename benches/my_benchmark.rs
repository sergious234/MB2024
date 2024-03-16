use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use practica_one::*;


fn criterion_benchmark(c: &mut Criterion) {


    println!("{:?}", std::env::current_dir());
    let cities = read_palets("data/destinos_palets_168.txt");
    let distances = read_distances("data/matriz_distancias_50.txt");

    let mut b = c.benchmark_group("Basic");
    b.measurement_time(Duration::from_secs(10));

    b.bench_function("SA", |b| b.iter(|| {
        let sa = SimulatedAnnealing::new(black_box(&distances), black_box(cities.clone()));
        sa.run();
    }));
}



criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
