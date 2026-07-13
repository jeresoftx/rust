/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    total: i32,
    notes: Vec<String>,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, total: i32) -> Self {
        Self {
            id: id.into(),
            total,
            notes: Vec::new(),
        }
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} total={} notes={}",
            self.id,
            self.total,
            self.notes.join(", ")
        )
    }
}

/// Contrato publico `OrderCommand` que desacopla las piezas del ejemplo.
pub trait OrderCommand {
    /// Operacion `execute` definida por la abstraccion del ejemplo.
    fn execute(&mut self, order: &mut Order);
    /// Operacion `undo` definida por la abstraccion del ejemplo.
    fn undo(&mut self, order: &mut Order);
}

/// Tipo publico `ApplyDiscountCommand` usado por el ejemplo para expresar el dominio del patron.
pub struct ApplyDiscountCommand {
    amount: i32,
}

impl ApplyDiscountCommand {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

impl OrderCommand for ApplyDiscountCommand {
    /// Operacion `execute` definida por la abstraccion del ejemplo.
    fn execute(&mut self, order: &mut Order) {
        order.total -= self.amount;
    }

    /// Operacion `undo` definida por la abstraccion del ejemplo.
    fn undo(&mut self, order: &mut Order) {
        order.total += self.amount;
    }
}

/// Tipo publico `AddNoteCommand` usado por el ejemplo para expresar el dominio del patron.
pub struct AddNoteCommand {
    note: String,
}

impl AddNoteCommand {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(note: impl Into<String>) -> Self {
        Self { note: note.into() }
    }
}

impl OrderCommand for AddNoteCommand {
    /// Operacion `execute` definida por la abstraccion del ejemplo.
    fn execute(&mut self, order: &mut Order) {
        order.notes.push(self.note.clone());
    }

    /// Operacion `undo` definida por la abstraccion del ejemplo.
    fn undo(&mut self, order: &mut Order) {
        if let Some(position) = order.notes.iter().rposition(|note| note == &self.note) {
            order.notes.remove(position);
        }
    }
}

/// Tipo publico `OrderHistory` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderHistory {
    executed: Vec<Box<dyn OrderCommand>>,
}

impl OrderHistory {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            executed: Vec::new(),
        }
    }

    /// Ejecuta el caso de uso o comando del ejemplo.
    pub fn execute(&mut self, mut command: Box<dyn OrderCommand>, order: &mut Order) {
        command.execute(order);
        self.executed.push(command);
    }

    /// Modela la operacion `undo last` dentro del ejemplo del patron.
    pub fn undo_last(&mut self, order: &mut Order) {
        if let Some(mut command) = self.executed.pop() {
            command.undo(order);
        }
    }
}

impl Default for OrderHistory {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}
