pub trait PaymentAuthorizer {
    fn gateway(&self) -> &'static str;
    fn authorize(&self, order_id: &str, amount_cents: u32) -> String;
}

pub trait ReceiptPublisher {
    fn publish_receipt(&self, order_id: &str) -> String;
}

pub trait PaymentProviderFactory {
    fn create_authorizer(&self) -> Box<dyn PaymentAuthorizer>;
    fn create_receipt_publisher(&self) -> Box<dyn ReceiptPublisher>;
}

pub struct StripeLikeFactory;

impl PaymentProviderFactory for StripeLikeFactory {
    fn create_authorizer(&self) -> Box<dyn PaymentAuthorizer> {
        Box::new(StripeLikeAuthorizer)
    }

    fn create_receipt_publisher(&self) -> Box<dyn ReceiptPublisher> {
        Box::new(StripeLikeReceiptPublisher)
    }
}

pub struct PaypalLikeFactory;

impl PaymentProviderFactory for PaypalLikeFactory {
    fn create_authorizer(&self) -> Box<dyn PaymentAuthorizer> {
        Box::new(PaypalLikeAuthorizer)
    }

    fn create_receipt_publisher(&self) -> Box<dyn ReceiptPublisher> {
        Box::new(PaypalLikeReceiptPublisher)
    }
}

struct StripeLikeAuthorizer;

impl PaymentAuthorizer for StripeLikeAuthorizer {
    fn gateway(&self) -> &'static str {
        "stripe"
    }

    fn authorize(&self, order_id: &str, amount_cents: u32) -> String {
        format!("pi_{order_id}_{amount_cents}")
    }
}

struct StripeLikeReceiptPublisher;

impl ReceiptPublisher for StripeLikeReceiptPublisher {
    fn publish_receipt(&self, order_id: &str) -> String {
        format!("receipt.stripe/{order_id}")
    }
}

struct PaypalLikeAuthorizer;

impl PaymentAuthorizer for PaypalLikeAuthorizer {
    fn gateway(&self) -> &'static str {
        "paypal"
    }

    fn authorize(&self, order_id: &str, amount_cents: u32) -> String {
        format!("paypal-{order_id}-{amount_cents}")
    }
}

struct PaypalLikeReceiptPublisher;

impl ReceiptPublisher for PaypalLikeReceiptPublisher {
    fn publish_receipt(&self, order_id: &str) -> String {
        format!("paypal.me/receipts/{order_id}")
    }
}

pub fn checkout(factory: &dyn PaymentProviderFactory, order_id: &str, amount_cents: u32) -> String {
    let authorizer = factory.create_authorizer();
    let receipt_publisher = factory.create_receipt_publisher();

    format!(
        "gateway={} | authorization={} | receipt={}",
        authorizer.gateway(),
        authorizer.authorize(order_id, amount_cents),
        receipt_publisher.publish_receipt(order_id)
    )
}
