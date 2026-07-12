pub trait InvoiceVisitor {
    fn visit_invoice(&mut self, invoice: &Invoice);
    fn visit_product(&mut self, line: &ProductLine);
    fn visit_discount(&mut self, line: &DiscountLine);
    fn visit_tax(&mut self, line: &TaxLine);
}

pub trait VisitableInvoiceItem {
    fn accept(&self, visitor: &mut dyn InvoiceVisitor);
}

pub struct Invoice {
    id: String,
    items: Vec<Box<dyn VisitableInvoiceItem>>,
}

impl Invoice {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn with_product(mut self, name: impl Into<String>, unit_cents: u32, quantity: u32) -> Self {
        self.items
            .push(Box::new(ProductLine::new(name, unit_cents, quantity)));
        self
    }

    pub fn with_discount(mut self, name: impl Into<String>, amount_cents: u32) -> Self {
        self.items
            .push(Box::new(DiscountLine::new(name, amount_cents)));
        self
    }

    pub fn with_tax(mut self, name: impl Into<String>, amount_cents: u32) -> Self {
        self.items.push(Box::new(TaxLine::new(name, amount_cents)));
        self
    }

    pub fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_invoice(self);
        for item in &self.items {
            item.accept(visitor);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductLine {
    name: String,
    unit_cents: u32,
    quantity: u32,
}

impl ProductLine {
    pub fn new(name: impl Into<String>, unit_cents: u32, quantity: u32) -> Self {
        Self {
            name: name.into(),
            unit_cents,
            quantity,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn line_total_cents(&self) -> u32 {
        self.unit_cents * self.quantity
    }
}

impl VisitableInvoiceItem for ProductLine {
    fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_product(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscountLine {
    name: String,
    amount_cents: u32,
}

impl DiscountLine {
    pub fn new(name: impl Into<String>, amount_cents: u32) -> Self {
        Self {
            name: name.into(),
            amount_cents,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn amount_cents(&self) -> u32 {
        self.amount_cents
    }
}

impl VisitableInvoiceItem for DiscountLine {
    fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_discount(self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaxLine {
    name: String,
    amount_cents: u32,
}

impl TaxLine {
    pub fn new(name: impl Into<String>, amount_cents: u32) -> Self {
        Self {
            name: name.into(),
            amount_cents,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn amount_cents(&self) -> u32 {
        self.amount_cents
    }
}

impl VisitableInvoiceItem for TaxLine {
    fn accept(&self, visitor: &mut dyn InvoiceVisitor) {
        visitor.visit_tax(self);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TotalsVisitor {
    subtotal_cents: u32,
    discount_cents: u32,
    tax_cents: u32,
    events: Vec<String>,
}

impl TotalsVisitor {
    pub fn subtotal_cents(&self) -> u32 {
        self.subtotal_cents
    }

    pub fn discount_cents(&self) -> u32 {
        self.discount_cents
    }

    pub fn tax_cents(&self) -> u32 {
        self.tax_cents
    }

    pub fn total_cents(&self) -> u32 {
        self.subtotal_cents + self.tax_cents - self.discount_cents
    }

    pub fn events(&self) -> Vec<&str> {
        self.events.iter().map(String::as_str).collect()
    }
}

impl InvoiceVisitor for TotalsVisitor {
    fn visit_invoice(&mut self, invoice: &Invoice) {
        self.events.push(format!("invoice:{}", invoice.id()));
    }

    fn visit_product(&mut self, line: &ProductLine) {
        let amount = line.line_total_cents();
        self.subtotal_cents += amount;
        self.events
            .push(format!("product:{}:{amount}", line.name()));
    }

    fn visit_discount(&mut self, line: &DiscountLine) {
        self.discount_cents += line.amount_cents();
        self.events
            .push(format!("discount:{}:{}", line.name(), line.amount_cents()));
    }

    fn visit_tax(&mut self, line: &TaxLine) {
        self.tax_cents += line.amount_cents();
        self.events
            .push(format!("tax:{}:{}", line.name(), line.amount_cents()));
    }
}
