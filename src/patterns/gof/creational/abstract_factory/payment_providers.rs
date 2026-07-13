/// Contrato publico `PaymentAuthorizer` que desacopla las piezas del ejemplo.
pub trait PaymentAuthorizer {
    /// Operacion `gateway` definida por la abstraccion del ejemplo.
    fn gateway(&self) -> &'static str;
    /// Operacion `authorize` definida por la abstraccion del ejemplo.
    fn authorize(&self, order_id: &str, amount_cents: u32) -> String;
}

/// Contrato publico `ReceiptPublisher` que desacopla las piezas del ejemplo.
pub trait ReceiptPublisher {
    /// Operacion `publish receipt` definida por la abstraccion del ejemplo.
    fn publish_receipt(&self, order_id: &str) -> String;
}

/// Contrato publico `PaymentProviderFactory` que desacopla las piezas del ejemplo.
pub trait PaymentProviderFactory {
    /// Operacion `create authorizer` definida por la abstraccion del ejemplo.
    fn create_authorizer(&self) -> Box<dyn PaymentAuthorizer>;
    /// Operacion `create receipt publisher` definida por la abstraccion del ejemplo.
    fn create_receipt_publisher(&self) -> Box<dyn ReceiptPublisher>;
}

/// Tipo publico `StripeLikeFactory` usado por el ejemplo para expresar el dominio del patron.
pub struct StripeLikeFactory;

impl PaymentProviderFactory for StripeLikeFactory {
    /// Operacion `create authorizer` definida por la abstraccion del ejemplo.
    fn create_authorizer(&self) -> Box<dyn PaymentAuthorizer> {
        Box::new(StripeLikeAuthorizer)
    }

    /// Operacion `create receipt publisher` definida por la abstraccion del ejemplo.
    fn create_receipt_publisher(&self) -> Box<dyn ReceiptPublisher> {
        Box::new(StripeLikeReceiptPublisher)
    }
}

/// Tipo publico `PaypalLikeFactory` usado por el ejemplo para expresar el dominio del patron.
pub struct PaypalLikeFactory;

impl PaymentProviderFactory for PaypalLikeFactory {
    /// Operacion `create authorizer` definida por la abstraccion del ejemplo.
    fn create_authorizer(&self) -> Box<dyn PaymentAuthorizer> {
        Box::new(PaypalLikeAuthorizer)
    }

    /// Operacion `create receipt publisher` definida por la abstraccion del ejemplo.
    fn create_receipt_publisher(&self) -> Box<dyn ReceiptPublisher> {
        Box::new(PaypalLikeReceiptPublisher)
    }
}

struct StripeLikeAuthorizer;

impl PaymentAuthorizer for StripeLikeAuthorizer {
    /// Operacion `gateway` definida por la abstraccion del ejemplo.
    fn gateway(&self) -> &'static str {
        "stripe"
    }

    /// Operacion `authorize` definida por la abstraccion del ejemplo.
    fn authorize(&self, order_id: &str, amount_cents: u32) -> String {
        format!("pi_{order_id}_{amount_cents}")
    }
}

struct StripeLikeReceiptPublisher;

impl ReceiptPublisher for StripeLikeReceiptPublisher {
    /// Operacion `publish receipt` definida por la abstraccion del ejemplo.
    fn publish_receipt(&self, order_id: &str) -> String {
        format!("receipt.stripe/{order_id}")
    }
}

struct PaypalLikeAuthorizer;

impl PaymentAuthorizer for PaypalLikeAuthorizer {
    /// Operacion `gateway` definida por la abstraccion del ejemplo.
    fn gateway(&self) -> &'static str {
        "paypal"
    }

    /// Operacion `authorize` definida por la abstraccion del ejemplo.
    fn authorize(&self, order_id: &str, amount_cents: u32) -> String {
        format!("paypal-{order_id}-{amount_cents}")
    }
}

struct PaypalLikeReceiptPublisher;

impl ReceiptPublisher for PaypalLikeReceiptPublisher {
    /// Operacion `publish receipt` definida por la abstraccion del ejemplo.
    fn publish_receipt(&self, order_id: &str) -> String {
        format!("paypal.me/receipts/{order_id}")
    }
}

/// Modela la operacion `checkout` dentro del ejemplo del patron.
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
