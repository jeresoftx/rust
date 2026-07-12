pub mod entities {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CartLine {
        sku: String,
        quantity: u32,
        unit_price_cents: u32,
    }

    impl CartLine {
        pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
                unit_price_cents,
            }
        }

        pub fn subtotal_cents(&self) -> u32 {
            self.quantity * self.unit_price_cents
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Cart {
        lines: Vec<CartLine>,
    }

    impl Cart {
        pub fn new(lines: Vec<CartLine>) -> Self {
            Self { lines }
        }

        pub fn subtotal_cents(&self) -> u32 {
            self.lines.iter().map(CartLine::subtotal_cents).sum()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DiscountPolicy {
        threshold_cents: u32,
        percent: u32,
    }

    impl DiscountPolicy {
        pub fn new(threshold_cents: u32, percent: u32) -> Self {
            Self {
                threshold_cents,
                percent,
            }
        }

        pub fn discount_for(&self, subtotal_cents: u32) -> u32 {
            if subtotal_cents >= self.threshold_cents {
                subtotal_cents * self.percent / 100
            } else {
                0
            }
        }
    }
}

pub mod use_cases {
    use super::entities::{Cart, DiscountPolicy};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DiscountRequest {
        cart: Cart,
    }

    impl DiscountRequest {
        pub fn new(cart: Cart) -> Self {
            Self { cart }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DiscountResult {
        subtotal_cents: u32,
        discount_cents: u32,
        total_cents: u32,
    }

    impl DiscountResult {
        pub fn subtotal_cents(&self) -> u32 {
            self.subtotal_cents
        }

        pub fn discount_cents(&self) -> u32 {
            self.discount_cents
        }

        pub fn total_cents(&self) -> u32 {
            self.total_cents
        }
    }

    #[derive(Debug, Clone)]
    pub struct CalculateDiscount {
        policy: DiscountPolicy,
    }

    impl CalculateDiscount {
        pub fn new(policy: DiscountPolicy) -> Self {
            Self { policy }
        }

        pub fn execute(&self, request: DiscountRequest) -> DiscountResult {
            let subtotal_cents = request.cart.subtotal_cents();
            let discount_cents = self.policy.discount_for(subtotal_cents);

            DiscountResult {
                subtotal_cents,
                discount_cents,
                total_cents: subtotal_cents - discount_cents,
            }
        }
    }
}

pub mod adapters {
    use super::entities::{Cart, CartLine};
    use super::use_cases::{CalculateDiscount, DiscountRequest};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HttpCartLineDto {
        sku: String,
        quantity: u32,
        unit_price_cents: u32,
    }

    impl HttpCartLineDto {
        pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
                unit_price_cents,
            }
        }

        fn into_entity(self) -> CartLine {
            CartLine::new(self.sku, self.quantity, self.unit_price_cents)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HttpDiscountRequest {
        lines: Vec<HttpCartLineDto>,
    }

    impl HttpDiscountRequest {
        pub fn new(lines: Vec<HttpCartLineDto>) -> Self {
            Self { lines }
        }

        fn into_use_case_request(self) -> DiscountRequest {
            let lines = self
                .lines
                .into_iter()
                .map(HttpCartLineDto::into_entity)
                .collect();
            DiscountRequest::new(Cart::new(lines))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HttpDiscountResponse {
        status_code: u16,
        body: String,
    }

    impl HttpDiscountResponse {
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> &str {
            &self.body
        }
    }

    #[derive(Debug, Clone)]
    pub struct HttpDiscountController {
        use_case: CalculateDiscount,
    }

    impl HttpDiscountController {
        pub fn new(use_case: CalculateDiscount) -> Self {
            Self { use_case }
        }

        pub fn handle(&self, request: HttpDiscountRequest) -> HttpDiscountResponse {
            let result = self.use_case.execute(request.into_use_case_request());
            HttpDiscountResponse {
                status_code: 200,
                body: format!(
                    "subtotal={} discount={} total={}",
                    result.subtotal_cents(),
                    result.discount_cents(),
                    result.total_cents()
                ),
            }
        }
    }
}
