//! Criterion benchmarks for phenotype-sentinel resilience primitives.
//!
//! Covers the three hot paths identified in the L6 audit finding:
//! circuit-breaker state transitions, token-bucket acquisition, and
//! bulkhead partition acquire/release round-trip.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use phenotype_sentinel::{CircuitBreaker, CircuitState, LeakyBucket, TokenBucket};
use std::time::Duration;

// ---------------------------------------------------------------------------
// Token-bucket
// ---------------------------------------------------------------------------

fn bench_token_bucket_try_acquire(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_bucket");
    for capacity in [10usize, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("try_acquire", capacity),
            &capacity,
            |b, &cap| {
                b.iter(|| {
                    let mut bucket = TokenBucket::new(cap, cap);
                    black_box(bucket.try_acquire())
                });
            },
        );
    }
    group.finish();
}

fn bench_leaky_bucket_try_add(c: &mut Criterion) {
    c.bench_function("leaky_bucket/try_add", |b| {
        b.iter(|| {
            let mut bucket = LeakyBucket::new(1_000, 1_000);
            black_box(bucket.try_add())
        });
    });
}

// ---------------------------------------------------------------------------
// Circuit-breaker
// ---------------------------------------------------------------------------

fn bench_circuit_breaker_is_allowed_closed(c: &mut Criterion) {
    c.bench_function("circuit_breaker/is_allowed_closed", |b| {
        let cb = CircuitBreaker::new(5, Duration::from_secs(60));
        b.iter(|| black_box(cb.is_allowed()));
    });
}

fn bench_circuit_breaker_record_failure_to_open(c: &mut Criterion) {
    c.bench_function("circuit_breaker/record_failure_until_open", |b| {
        b.iter(|| {
            let mut cb = CircuitBreaker::new(5, Duration::from_secs(60));
            for _ in 0..5 {
                cb.record_failure();
            }
            black_box(cb.state() == CircuitState::Open)
        });
    });
}

fn bench_circuit_breaker_success_reset(c: &mut Criterion) {
    c.bench_function("circuit_breaker/success_resets_failure_count", |b| {
        b.iter(|| {
            let mut cb = CircuitBreaker::new(5, Duration::from_secs(60));
            cb.record_failure();
            cb.record_success();
            black_box(cb.state() == CircuitState::Closed)
        });
    });
}

// ---------------------------------------------------------------------------
// Bulkhead  (async — single-threaded tokio runtime)
// ---------------------------------------------------------------------------

fn bench_bulkhead_acquire_release(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");

    let mut group = c.benchmark_group("bulkhead");
    for partitions in [2usize, 8, 32] {
        group.bench_with_input(
            BenchmarkId::new("acquire_release", partitions),
            &partitions,
            |b, &p| {
                b.to_async(&rt).iter(|| async move {
                    let bh = phenotype_sentinel::bulkhead::Bulkhead::new(p, 100);
                    let guard = bh.try_acquire(0).await.expect("acquire");
                    black_box(guard);
                    // guard's Drop spawns an async release; yield to let it run
                    tokio::task::yield_now().await;
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_token_bucket_try_acquire,
    bench_leaky_bucket_try_add,
    bench_circuit_breaker_is_allowed_closed,
    bench_circuit_breaker_record_failure_to_open,
    bench_circuit_breaker_success_reset,
    bench_bulkhead_acquire_release,
);
criterion_main!(benches);
