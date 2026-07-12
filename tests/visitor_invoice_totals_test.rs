use design_patterns_rust::patterns::gof::behavioral::visitor::invoice_totals::{
    Invoice, TotalsVisitor,
};

fn sample_invoice() -> Invoice {
    Invoice::new("INV-100")
        .with_product("Subscription", 10_000, 2)
        .with_product("Extra seats", 2_500, 3)
        .with_discount("Launch coupon", 1_500)
        .with_tax("VAT", 3_900)
}

#[test]
fn visitor_calculates_invoice_totals() {
    let invoice = sample_invoice();
    let mut visitor = TotalsVisitor::default();

    invoice.accept(&mut visitor);

    assert_eq!(visitor.subtotal_cents(), 27_500);
    assert_eq!(visitor.discount_cents(), 1_500);
    assert_eq!(visitor.tax_cents(), 3_900);
    assert_eq!(visitor.total_cents(), 29_900);
}

#[test]
fn visitor_records_invoice_lines_while_calculating_totals() {
    let invoice = sample_invoice();
    let mut visitor = TotalsVisitor::default();

    invoice.accept(&mut visitor);

    assert_eq!(
        visitor.events(),
        [
            "invoice:INV-100",
            "product:Subscription:20000",
            "product:Extra seats:7500",
            "discount:Launch coupon:1500",
            "tax:VAT:3900"
        ]
    );
}

#[test]
fn visitor_handles_invoice_without_discounts_or_taxes() {
    let invoice = Invoice::new("INV-101").with_product("Support", 5_000, 1);
    let mut visitor = TotalsVisitor::default();

    invoice.accept(&mut visitor);

    assert_eq!(visitor.subtotal_cents(), 5_000);
    assert_eq!(visitor.total_cents(), 5_000);
}
