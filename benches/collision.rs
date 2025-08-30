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

criterion_group!(benches, bench_collision);
criterion_main!(benches);
