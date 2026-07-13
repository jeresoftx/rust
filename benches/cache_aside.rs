use criterion::{Criterion, black_box, criterion_group, criterion_main};
use design_patterns_rust::patterns::distributed_systems::cache_aside::read_through::{
    CacheAsideService, Product, ProductRepository,
};

fn hot_cache_hit(c: &mut Criterion) {
    let repository = ProductRepository::with_products([
        Product::new("sku-1", "Keyboard"),
        Product::new("sku-2", "Mouse"),
    ]);
    let mut service = CacheAsideService::new(repository);
    service.get_product("sku-1");

    c.bench_function("cache_aside_hot_hit", |b| {
        b.iter(|| black_box(service.get_product(black_box("sku-1"))));
    });
}

criterion_group!(benches, hot_cache_hit);
criterion_main!(benches);
