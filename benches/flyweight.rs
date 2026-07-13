use criterion::{Criterion, black_box, criterion_group, criterion_main};
use design_patterns_rust::patterns::gof::structural::flyweight::currency_catalog::{
    CurrencyCatalog, MoneyAmount,
};

fn shared_currency_lookup(c: &mut Criterion) {
    let catalog = CurrencyCatalog::default();

    c.bench_function("flyweight_shared_currency_lookup", |b| {
        b.iter(|| {
            let currency = catalog.currency(black_box("USD")).expect("USD exists");
            let amount = MoneyAmount::new(black_box(12_345), currency);
            black_box(amount.format())
        });
    });
}

criterion_group!(benches, shared_currency_lookup);
criterion_main!(benches);
