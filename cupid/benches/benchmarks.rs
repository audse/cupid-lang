use criterion::{criterion_group, criterion_main, Criterion};
use cupid::RFileHandler;

fn run() {
	let mut file_handler = RFileHandler::new("src/tests/main.cupid");
	file_handler.run();
}

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("main.cupid", |b| b.iter(|| run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);