/// Contrato publico `InvoiceVisitor` que desacopla las piezas del ejemplo.
pub trait InvoiceVisitor {
    /// Operacion `visit invoice` definida por la abstraccion del ejemplo.
    fn visit_invoice(&mut self, invoice: &Invoice);
    /// Operacion `visit product` definida por la abstraccion del ejemplo.
    fn visit_product(&mut self, line: &ProductLine);
    /// Operacion `visit discount` definida por la abstraccion del ejemplo.
    fn visit_discount(&mut self, line: &DiscountLine);
    /// Operacion `visit tax` definida por la abstraccion del ejemplo.
    fn visit_tax(&mut self, line: &TaxLine);
}

/// Contrato publico `VisitableInvoiceItem` que desacopla las piezas del ejemplo.
pub trait VisitableInvoiceItem {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn InvoiceVisitor);
}

/// Tipo publico `Invoice` usado por el ejemplo para expresar el dominio del patron.
pub struct Invoice {
    id: String,
    items: Vec<Box<dyn VisitableInvoiceItem>>,
}

impl Invoice {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
        }
    }

    /// Modela la operacion `id` dentro del ejemplo del patron.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Modela la operacion `with product` dentro del ejemplo del patron.
    pub fn with_product(mut self, name: impl Into<String>, unit_cents: u32, quantity: u32) -> Self {
        self.items
            .push(Box::new(ProductLine::new(name, unit_cents, quantity)));
        self
    }

    /// Modela la operacion `with discount` dentro del ejemplo del patron.
    pub fn with_discount(mut self, name: impl Into<String>, amount_cents: u32) -> Self {
        self.items
            .push(Box::new(DiscountLine::new(name, amount_cents)));
        self
    }

    /// Modela la operacion `with tax` dentro del ejemplo del patron.
    pub fn with_tax(mut self, name: impl Into<String>, amount_cents: u32) -> Self {
        self.items.push(Box::new(TaxLine::new(name, amount_cents)));
        self
    }

    /// Modela la operacion `accept` dentro del ejemplo del patron.
    pub fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_invoice(self);
        for item in &self.items {
            item.accept(visitor);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ProductLine` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductLine {
    name: String,
    unit_cents: u32,
    quantity: u32,
}

impl ProductLine {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, unit_cents: u32, quantity: u32) -> Self {
        Self {
            name: name.into(),
            unit_cents,
            quantity,
        }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `line total cents` dentro del ejemplo del patron.
    pub fn line_total_cents(&self) -> u32 {
        self.unit_cents * self.quantity
    }
}

impl VisitableInvoiceItem for ProductLine {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_product(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DiscountLine` usado por el ejemplo para expresar el dominio del patron.
pub struct DiscountLine {
    name: String,
    amount_cents: u32,
}

impl DiscountLine {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, amount_cents: u32) -> Self {
        Self {
            name: name.into(),
            amount_cents,
        }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `amount cents` dentro del ejemplo del patron.
    pub fn amount_cents(&self) -> u32 {
        self.amount_cents
    }
}

impl VisitableInvoiceItem for DiscountLine {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_discount(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `TaxLine` usado por el ejemplo para expresar el dominio del patron.
pub struct TaxLine {
    name: String,
    amount_cents: u32,
}

impl TaxLine {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, amount_cents: u32) -> Self {
        Self {
            name: name.into(),
            amount_cents,
        }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `amount cents` dentro del ejemplo del patron.
    pub fn amount_cents(&self) -> u32 {
        self.amount_cents
    }
}

impl VisitableInvoiceItem for TaxLine {
    /// Operacion `accept` definida por la abstraccion del ejemplo.
    fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_tax(self);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `TotalsVisitor` usado por el ejemplo para expresar el dominio del patron.
pub struct TotalsVisitor {
    subtotal_cents: u32,
    discount_cents: u32,
    tax_cents: u32,
    events: Vec<String>,
}

impl TotalsVisitor {
    /// Modela la operacion `subtotal cents` dentro del ejemplo del patron.
    pub fn subtotal_cents(&self) -> u32 {
        self.subtotal_cents
    }

    /// Modela la operacion `discount cents` dentro del ejemplo del patron.
    pub fn discount_cents(&self) -> u32 {
        self.discount_cents
    }

    /// Modela la operacion `tax cents` dentro del ejemplo del patron.
    pub fn tax_cents(&self) -> u32 {
        self.tax_cents
    }

    /// Modela la operacion `total cents` dentro del ejemplo del patron.
    pub fn total_cents(&self) -> u32 {
        self.subtotal_cents + self.tax_cents - self.discount_cents
    }

    /// Modela la operacion `events` dentro del ejemplo del patron.
    pub fn events(&self) -> Vec<&str> {
        self.events.iter().map(String::as_str).collect()
    }
}

impl InvoiceVisitor for TotalsVisitor {
    /// Operacion `visit invoice` definida por la abstraccion del ejemplo.
    fn visit_invoice(&mut self, invoice: &Invoice) {
        self.events.push(format!("invoice:{}", invoice.id()));
    }

    /// Operacion `visit product` definida por la abstraccion del ejemplo.
    fn visit_product(&mut self, line: &ProductLine) {
        let amount = line.line_total_cents();
        self.subtotal_cents += amount;
        self.events
            .push(format!("product:{}:{amount}", line.name()));
    }

    /// Operacion `visit discount` definida por la abstraccion del ejemplo.
    fn visit_discount(&mut self, line: &DiscountLine) {
        self.discount_cents += line.amount_cents();
        self.events
            .push(format!("discount:{}:{}", line.name(), line.amount_cents()));
    }

    /// Operacion `visit tax` definida por la abstraccion del ejemplo.
    fn visit_tax(&mut self, line: &TaxLine) {
        self.tax_cents += line.amount_cents();
        self.events
            .push(format!("tax:{}:{}", line.name(), line.amount_cents()));
    }
}
