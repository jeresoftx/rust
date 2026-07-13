//! Framework Independent Rules.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::clean_architecture::framework_independent_rules as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `entities` dentro del catalogo de patrones.
pub mod entities {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CartLine` usado por el ejemplo para expresar el dominio del patron.
    pub struct CartLine {
        sku: String,
        quantity: u32,
        unit_price_cents: u32,
    }

    impl CartLine {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
                unit_price_cents,
            }
        }

        /// Modela la operacion `subtotal cents` dentro del ejemplo del patron.
        pub fn subtotal_cents(&self) -> u32 {
            self.quantity * self.unit_price_cents
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `Cart` usado por el ejemplo para expresar el dominio del patron.
    pub struct Cart {
        lines: Vec<CartLine>,
    }

    impl Cart {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(lines: Vec<CartLine>) -> Self {
            Self { lines }
        }

        /// Modela la operacion `subtotal cents` dentro del ejemplo del patron.
        pub fn subtotal_cents(&self) -> u32 {
            self.lines.iter().map(CartLine::subtotal_cents).sum()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Tipo publico `DiscountPolicy` usado por el ejemplo para expresar el dominio del patron.
    pub struct DiscountPolicy {
        threshold_cents: u32,
        percent: u32,
    }

    impl DiscountPolicy {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(threshold_cents: u32, percent: u32) -> Self {
            Self {
                threshold_cents,
                percent,
            }
        }

        /// Modela la operacion `discount for` dentro del ejemplo del patron.
        pub fn discount_for(&self, subtotal_cents: u32) -> u32 {
            if subtotal_cents >= self.threshold_cents {
                subtotal_cents * self.percent / 100
            } else {
                0
            }
        }
    }
}

/// Modulo del ejemplo `use_cases` dentro del catalogo de patrones.
pub mod use_cases {
    use super::entities::{Cart, DiscountPolicy};

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `DiscountRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct DiscountRequest {
        cart: Cart,
    }

    impl DiscountRequest {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(cart: Cart) -> Self {
            Self { cart }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `DiscountResult` usado por el ejemplo para expresar el dominio del patron.
    pub struct DiscountResult {
        subtotal_cents: u32,
        discount_cents: u32,
        total_cents: u32,
    }

    impl DiscountResult {
        /// Modela la operacion `subtotal cents` dentro del ejemplo del patron.
        pub fn subtotal_cents(&self) -> u32 {
            self.subtotal_cents
        }

        /// Modela la operacion `discount cents` dentro del ejemplo del patron.
        pub fn discount_cents(&self) -> u32 {
            self.discount_cents
        }

        /// Modela la operacion `total cents` dentro del ejemplo del patron.
        pub fn total_cents(&self) -> u32 {
            self.total_cents
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `CalculateDiscount` usado por el ejemplo para expresar el dominio del patron.
    pub struct CalculateDiscount {
        policy: DiscountPolicy,
    }

    impl CalculateDiscount {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(policy: DiscountPolicy) -> Self {
            Self { policy }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
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

/// Modulo del ejemplo `adapters` dentro del catalogo de patrones.
pub mod adapters {
    use super::entities::{Cart, CartLine};
    use super::use_cases::{CalculateDiscount, DiscountRequest};

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `HttpCartLineDto` usado por el ejemplo para expresar el dominio del patron.
    pub struct HttpCartLineDto {
        sku: String,
        quantity: u32,
        unit_price_cents: u32,
    }

    impl HttpCartLineDto {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
                unit_price_cents,
            }
        }

        /// Operacion `into entity` definida por la abstraccion del ejemplo.
        fn into_entity(self) -> CartLine {
            CartLine::new(self.sku, self.quantity, self.unit_price_cents)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `HttpDiscountRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct HttpDiscountRequest {
        lines: Vec<HttpCartLineDto>,
    }

    impl HttpDiscountRequest {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(lines: Vec<HttpCartLineDto>) -> Self {
            Self { lines }
        }

        /// Operacion `into use case request` definida por la abstraccion del ejemplo.
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
    /// Tipo publico `HttpDiscountResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct HttpDiscountResponse {
        status_code: u16,
        body: String,
    }

    impl HttpDiscountResponse {
        /// Modela la operacion `status code` dentro del ejemplo del patron.
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        /// Modela la operacion `body` dentro del ejemplo del patron.
        pub fn body(&self) -> &str {
            &self.body
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `HttpDiscountController` usado por el ejemplo para expresar el dominio del patron.
    pub struct HttpDiscountController {
        use_case: CalculateDiscount,
    }

    impl HttpDiscountController {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(use_case: CalculateDiscount) -> Self {
            Self { use_case }
        }

        /// Procesa la entrada publica del ejemplo.
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
