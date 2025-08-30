use criterion::{Criterion, criterion_group, criterion_main};
use java_hashcode_chosen_prefix::{find_collision, hash_update, hashcode};

fn bench_collision(c: &mut Criterion) {
    let mut counter = 100_000_000u64;
    let mut group = c.benchmark_group("collision");
    group.throughput(criterion::Throughput::Elements(u32::MAX as u64 / 2));
    group.bench_function("meet_in_the_middle", |b| {
        b.iter_batched(
            || {
                let prefix = br#"{"deduct":"#;
                let mut hash = hashcode(prefix);
                let mut digs = [0u8; 8];
                let mut t = counter;
                for i in 0..8 {
                    digs[i] = (t % 10) as u8 + b'0';
                    t /= 10;
                }
                counter += 1;
                hash_update(&mut hash, &digs);
                hash
            },
            |midstate| {
                core::hint::black_box(find_collision(midstate, b"0", b"99999").msg());
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn bench_collision_parallel(c: &mut Criterion) {
    let mut counter = 100_000_000u64;
    let mut group = c.benchmark_group("collision_parallel");

    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    group.bench_function("meet_in_the_middle", |b| {
        b.iter_custom(|iters| {
            use rayon::iter::{IntoParallelIterator, ParallelIterator};

            let prefix = br#"{"deduct":"#;

            let mut midstates = vec![0u32; iters as usize * 512];

            for m in midstates.iter_mut() {
                let mut hash = hashcode(prefix);
                let mut digs = [0u8; 8];
                let mut t = counter;
                for i in 0..8 {
                    digs[i] = (t % 10) as u8 + b'0';
                    t /= 10;
                }
                counter += 1;
                hash_update(&mut hash, &digs);
                *m = hash;
            }

            pool.scope(|_| {
                let start = std::time::Instant::now();
                midstates.into_par_iter().for_each(|midstate| {
                    core::hint::black_box(find_collision(midstate, b"0", b"99999").msg());
                });

                start.elapsed() / 512
            })
        })
    });
}

criterion_group!(benches, bench_collision, bench_collision_parallel);
criterion_main!(benches);
