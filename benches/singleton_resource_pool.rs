use criterion::{Criterion, black_box, criterion_group, criterion_main};
use design_patterns_rust::patterns::gof::creational::singleton::resource_pool::{
    acquire_connection, release_connection,
};

fn acquire_and_release_shared_connection(c: &mut Criterion) {
    c.bench_function("singleton_resource_pool_acquire_release", |b| {
        b.iter(|| {
            if let Some(connection) = acquire_connection() {
                black_box(connection.summary());
                release_connection(connection);
            }
        });
    });
}

criterion_group!(benches, acquire_and_release_shared_connection);
criterion_main!(benches);
