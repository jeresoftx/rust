pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Order {
        id: String,
        lines: Vec<OrderLine>,
        status: OrderStatus,
    }

    impl Order {
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

        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn total_units(&self) -> u32 {
            self.lines.iter().map(|line| line.quantity).sum()
        }

        pub fn status(&self) -> OrderStatus {
            self.status
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OrderLine {
        sku: String,
        quantity: u32,
    }

    impl OrderLine {
        pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OrderStatus {
        Accepted,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum OrderError {
        EmptyOrder,
        InvalidQuantity,
    }

    impl OrderError {
        pub fn message(&self) -> &'static str {
            match self {
                Self::EmptyOrder => "order must have at least one line",
                Self::InvalidQuantity => "order line quantity must be greater than zero",
            }
        }
    }
}

pub mod application {
    use super::domain::{Order, OrderError, OrderLine};

    pub trait OrderRepository {
        fn save(&mut self, order: Order);
        fn all(&self) -> Vec<Order>;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CreateOrderCommand {
        id: String,
        lines: Vec<OrderLine>,
    }

    impl CreateOrderCommand {
        pub fn new(id: impl Into<String>, lines: Vec<OrderLine>) -> Self {
            Self {
                id: id.into(),
                lines,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CreateOrder<R> {
        repository: R,
    }

    impl<R> CreateOrder<R>
    where
        R: OrderRepository,
    {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        pub fn execute(&mut self, command: CreateOrderCommand) -> Result<Order, OrderError> {
            let order = Order::new(command.id, command.lines)?;
            self.repository.save(order.clone());
            Ok(order)
        }

        pub fn stored_orders(&self) -> Vec<Order> {
            self.repository.all()
        }
    }
}

pub mod infrastructure {
    use super::application::OrderRepository;
    use super::domain::Order;

    #[derive(Debug, Clone, Default)]
    pub struct InMemoryOrderRepository {
        orders: Vec<Order>,
    }

    impl OrderRepository for InMemoryOrderRepository {
        fn save(&mut self, order: Order) {
            self.orders.push(order);
        }

        fn all(&self) -> Vec<Order> {
            self.orders.clone()
        }
    }
}

pub mod presentation {
    use super::application::{CreateOrder, CreateOrderCommand, OrderRepository};
    use super::domain::{Order, OrderLine};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CreateOrderDto {
        id: String,
        lines: Vec<OrderLineDto>,
    }

    impl CreateOrderDto {
        pub fn new(id: impl Into<String>, lines: Vec<OrderLineDto>) -> Self {
            Self {
                id: id.into(),
                lines,
            }
        }

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
    pub struct OrderLineDto {
        sku: String,
        quantity: u32,
    }

    impl OrderLineDto {
        pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct OrderController<R> {
        create_order: CreateOrder<R>,
    }

    impl<R> OrderController<R>
    where
        R: OrderRepository,
    {
        pub fn new(create_order: CreateOrder<R>) -> Self {
            Self { create_order }
        }

        pub fn create_order(&mut self, dto: CreateOrderDto) -> CreateOrderResponse {
            match self.create_order.execute(dto.into_command()) {
                Ok(order) => CreateOrderResponse::created(OrderResponseDto::from(order)),
                Err(error) => CreateOrderResponse::bad_request(error.message()),
            }
        }

        pub fn stored_orders(&self) -> Vec<Order> {
            self.create_order.stored_orders()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CreateOrderResponse {
        status_code: u16,
        body: Option<OrderResponseDto>,
        error: Option<String>,
    }

    impl CreateOrderResponse {
        fn created(body: OrderResponseDto) -> Self {
            Self {
                status_code: 201,
                body: Some(body),
                error: None,
            }
        }

        fn bad_request(message: &str) -> Self {
            Self {
                status_code: 400,
                body: None,
                error: Some(message.to_string()),
            }
        }

        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> &OrderResponseDto {
            self.body.as_ref().expect("response should contain a body")
        }

        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OrderResponseDto {
        order_id: String,
        total_units: u32,
        status: String,
    }

    impl OrderResponseDto {
        pub fn order_id(&self) -> &str {
            &self.order_id
        }

        pub fn summary(&self) -> String {
            format!(
                "{} {} with {} units",
                self.order_id, self.status, self.total_units
            )
        }
    }

    impl From<Order> for OrderResponseDto {
        fn from(order: Order) -> Self {
            Self {
                order_id: order.id().to_string(),
                total_units: order.total_units(),
                status: "accepted".to_string(),
            }
        }
    }
}
