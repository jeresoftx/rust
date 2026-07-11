pub struct Order {
    id: String,
    total: i32,
    notes: Vec<String>,
}

impl Order {
    pub fn new(id: impl Into<String>, total: i32) -> Self {
        Self {
            id: id.into(),
            total,
            notes: Vec::new(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} total={} notes={}",
            self.id,
            self.total,
            self.notes.join(", ")
        )
    }
}

pub trait OrderCommand {
    fn execute(&mut self, order: &mut Order);
    fn undo(&mut self, order: &mut Order);
}

pub struct ApplyDiscountCommand {
    amount: i32,
}

impl ApplyDiscountCommand {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

impl OrderCommand for ApplyDiscountCommand {
    fn execute(&mut self, order: &mut Order) {
        order.total -= self.amount;
    }

    fn undo(&mut self, order: &mut Order) {
        order.total += self.amount;
    }
}

pub struct AddNoteCommand {
    note: String,
}

impl AddNoteCommand {
    pub fn new(note: impl Into<String>) -> Self {
        Self { note: note.into() }
    }
}

impl OrderCommand for AddNoteCommand {
    fn execute(&mut self, order: &mut Order) {
        order.notes.push(self.note.clone());
    }

    fn undo(&mut self, order: &mut Order) {
        if let Some(position) = order.notes.iter().rposition(|note| note == &self.note) {
            order.notes.remove(position);
        }
    }
}

pub struct OrderHistory {
    executed: Vec<Box<dyn OrderCommand>>,
}

impl OrderHistory {
    pub fn new() -> Self {
        Self {
            executed: Vec::new(),
        }
    }

    pub fn execute(&mut self, mut command: Box<dyn OrderCommand>, order: &mut Order) {
        command.execute(order);
        self.executed.push(command);
    }

    pub fn undo_last(&mut self, order: &mut Order) {
        if let Some(mut command) = self.executed.pop() {
            command.undo(order);
        }
    }
}

impl Default for OrderHistory {
    fn default() -> Self {
        Self::new()
    }
}
