use design_patterns_rust::patterns::gof::creational::prototype::order_templates::{
    OrderTemplate, create_order_from_template,
};

#[test]
fn prototype_creates_order_from_preconfigured_template() {
    let template = OrderTemplate::weekly_office_supplies();

    let order = create_order_from_template(&template, "customer-77", 3);

    assert_eq!(
        order.summary(),
        "customer=customer-77 sku=PAPER-A4 quantity=3 cadence=weekly shipping=standard"
    );
}

#[test]
fn prototype_keeps_order_template_unchanged() {
    let template = OrderTemplate::weekly_office_supplies();

    let _order = create_order_from_template(&template, "customer-77", 3);

    assert_eq!(
        template.summary(),
        "customer=<customer> sku=PAPER-A4 quantity=1 cadence=weekly shipping=standard"
    );
}
