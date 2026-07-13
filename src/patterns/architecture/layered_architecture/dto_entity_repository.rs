//! Dto Entity Repository.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::layered_architecture::dto_entity_repository as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `domain` dentro del catalogo de patrones.
pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
    pub struct Order {
        id: String,
        lines: Vec<OrderLine>,
        status: OrderStatus,
    }

    impl Order {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(id: impl Into<String>, lines: Vec<OrderLine>) -> Result<Self, OrderError> {
            if lines.is_empty() {
                return Err(OrderError::EmptyOrder);
            }

            if lines.iter().any(|line| line.quantity == 0) {
                return Err(OrderError::InvalidQuantity);
            }

            Ok(Self {
                id: id.into(),
                lines,
                status: OrderStatus::Accepted,
            })
        }

        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `total units` dentro del ejemplo del patron.
        pub fn total_units(&self) -> u32 {
            self.lines.iter().map(|line| line.quantity).sum()
        }

        /// Modela la operacion `status` dentro del ejemplo del patron.
        pub fn status(&self) -> OrderStatus {
            self.status
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `OrderLine` usado por el ejemplo para expresar el dominio del patron.
    pub struct OrderLine {
        sku: String,
        quantity: u32,
    }

    impl OrderLine {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `OrderStatus` dentro del ejemplo.
    pub enum OrderStatus {
        /// Variante `Accepted` del estado o error del ejemplo.
        Accepted,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `OrderError` dentro del ejemplo.
    pub enum OrderError {
        /// Variante `EmptyOrder` del estado o error del ejemplo.
        EmptyOrder,
        /// Variante `InvalidQuantity` del estado o error del ejemplo.
        InvalidQuantity,
    }

    impl OrderError {
        /// Modela la operacion `message` dentro del ejemplo del patron.
        pub fn message(&self) -> &'static str {
            match self {
                Self::EmptyOrder => "order must have at least one line",
                Self::InvalidQuantity => "order line quantity must be greater than zero",
            }
        }
    }
}

/// Modulo del ejemplo `application` dentro del catalogo de patrones.
pub mod application {
    use super::domain::{Order, OrderError, OrderLine};

    /// Contrato publico `OrderRepository` que desacopla las piezas del ejemplo.
    pub trait OrderRepository {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, order: Order);
        /// Operacion `all` definida por la abstraccion del ejemplo.
        fn all(&self) -> Vec<Order>;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CreateOrderCommand` usado por el ejemplo para expresar el dominio del patron.
    pub struct CreateOrderCommand {
        id: String,
        lines: Vec<OrderLine>,
    }

    impl CreateOrderCommand {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(id: impl Into<String>, lines: Vec<OrderLine>) -> Self {
            Self {
                id: id.into(),
                lines,
            }
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `CreateOrder` usado por el ejemplo para expresar el dominio del patron.
    pub struct CreateOrder<R> {
        repository: R,
    }

    impl<R> CreateOrder<R>
    where
        R: OrderRepository,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
        pub fn execute(&mut self, command: CreateOrderCommand) -> Result<Order, OrderError> {
            let order = Order::new(command.id, command.lines)?;
            self.repository.save(order.clone());
            Ok(order)
        }

        /// Modela la operacion `stored orders` dentro del ejemplo del patron.
        pub fn stored_orders(&self) -> Vec<Order> {
            self.repository.all()
        }
    }
}

/// Modulo del ejemplo `infrastructure` dentro del catalogo de patrones.
pub mod infrastructure {
    use super::application::OrderRepository;
    use super::domain::Order;

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `InMemoryOrderRepository` usado por el ejemplo para expresar el dominio del patron.
    pub struct InMemoryOrderRepository {
        orders: Vec<Order>,
    }

    impl OrderRepository for InMemoryOrderRepository {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, order: Order) {
            self.orders.push(order);
        }

        /// Operacion `all` definida por la abstraccion del ejemplo.
        fn all(&self) -> Vec<Order> {
            self.orders.clone()
        }
    }
}

/// Modulo del ejemplo `presentation` dentro del catalogo de patrones.
pub mod presentation {
    use super::application::{CreateOrder, CreateOrderCommand, OrderRepository};
    use super::domain::{Order, OrderLine};

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CreateOrderDto` usado por el ejemplo para expresar el dominio del patron.
    pub struct CreateOrderDto {
        id: String,
        lines: Vec<OrderLineDto>,
    }

    impl CreateOrderDto {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(id: impl Into<String>, lines: Vec<OrderLineDto>) -> Self {
            Self {
                id: id.into(),
                lines,
            }
        }

        /// Operacion `into command` definida por la abstraccion del ejemplo.
        fn into_command(self) -> CreateOrderCommand {
            CreateOrderCommand::new(
                self.id,
                self.lines
                    .into_iter()
                    .map(|line| OrderLine::new(line.sku, line.quantity))
                    .collect(),
            )
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `OrderLineDto` usado por el ejemplo para expresar el dominio del patron.
    pub struct OrderLineDto {
        sku: String,
        quantity: u32,
    }

    impl OrderLineDto {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
            }
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `OrderController` usado por el ejemplo para expresar el dominio del patron.
    pub struct OrderController<R> {
        create_order: CreateOrder<R>,
    }

    impl<R> OrderController<R>
    where
        R: OrderRepository,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(create_order: CreateOrder<R>) -> Self {
            Self { create_order }
        }

        /// Modela la operacion `create order` dentro del ejemplo del patron.
        pub fn create_order(&mut self, dto: CreateOrderDto) -> CreateOrderResponse {
            match self.create_order.execute(dto.into_command()) {
                Ok(order) => CreateOrderResponse::created(OrderResponseDto::from(order)),
                Err(error) => CreateOrderResponse::bad_request(error.message()),
            }
        }

        /// Modela la operacion `stored orders` dentro del ejemplo del patron.
        pub fn stored_orders(&self) -> Vec<Order> {
            self.create_order.stored_orders()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CreateOrderResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct CreateOrderResponse {
        status_code: u16,
        body: Option<OrderResponseDto>,
        error: Option<String>,
    }

    impl CreateOrderResponse {
        /// Operacion `created` definida por la abstraccion del ejemplo.
        fn created(body: OrderResponseDto) -> Self {
            Self {
                status_code: 201,
                body: Some(body),
                error: None,
            }
        }

        /// Operacion `bad request` definida por la abstraccion del ejemplo.
        fn bad_request(message: &str) -> Self {
            Self {
                status_code: 400,
                body: None,
                error: Some(message.to_string()),
            }
        }

        /// Modela la operacion `status code` dentro del ejemplo del patron.
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        /// Modela la operacion `body` dentro del ejemplo del patron.
        pub fn body(&self) -> &OrderResponseDto {
            self.body.as_ref().expect("response should contain a body")
        }

        /// Modela la operacion `error` dentro del ejemplo del patron.
        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `OrderResponseDto` usado por el ejemplo para expresar el dominio del patron.
    pub struct OrderResponseDto {
        order_id: String,
        total_units: u32,
        status: String,
    }

    impl OrderResponseDto {
        /// Modela la operacion `order id` dentro del ejemplo del patron.
        pub fn order_id(&self) -> &str {
            &self.order_id
        }

        /// Devuelve un resumen legible del estado actual.
        pub fn summary(&self) -> String {
            format!(
                "{} {} with {} units",
                self.order_id, self.status, self.total_units
            )
        }
    }

    impl From<Order> for OrderResponseDto {
        /// Operacion `from` definida por la abstraccion del ejemplo.
        fn from(order: Order) -> Self {
            Self {
                order_id: order.id().to_string(),
                total_units: order.total_units(),
                status: "accepted".to_string(),
            }
        }
    }
}
