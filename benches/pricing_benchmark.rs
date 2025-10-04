use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_options_pricing::{BlackScholes, MonteCarloSimulator, OptionType};

fn black_scholes_benchmark(c: &mut Criterion) {
    c.bench_function("black_scholes_price", |b| {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
        b.iter(|| black_box(bs.price()));
    });

    c.bench_function("black_scholes_greeks", |b| {
        let bs = BlackScholes::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call);
        b.iter(|| black_box(bs.greeks()));
    });

    c.bench_function("implied_volatility", |b| {
        b.iter(|| {
            BlackScholes::implied_volatility(
                black_box(100.0),
                black_box(100.0),
                black_box(1.0),
                black_box(0.05),
                black_box(12.0),
                OptionType::Call,
            )
        });
    });
}

fn monte_carlo_benchmark(c: &mut Criterion) {
    c.bench_function("monte_carlo_10k", |b| {
        let mc = MonteCarloSimulator::new(100.0, 100.0, 1.0, 0.05, 0.2, 10000, OptionType::Call);
        b.iter(|| black_box(mc.price()));
    });

    c.bench_function("monte_carlo_100k", |b| {
        let mc = MonteCarloSimulator::new(100.0, 100.0, 1.0, 0.05, 0.2, 100000, OptionType::Call);
        b.iter(|| black_box(mc.price()));
    });
}

criterion_group!(benches, black_scholes_benchmark, monte_carlo_benchmark);
criterion_main!(benches);
