use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Default)]
/// Tipo publico `Ledger` usado por el ejemplo para expresar el dominio del patron.
pub struct Ledger {
    balances: RefCell<HashMap<String, i64>>,
    events: RefCell<Vec<String>>,
}

#[derive(Debug)]
/// Tipo publico `Transaction` usado por el ejemplo para expresar el dominio del patron.
pub struct Transaction<'a> {
    ledger: &'a Ledger,
    id: String,
    staged_entries: Vec<(String, i64)>,
    committed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CommitReceipt` usado por el ejemplo para expresar el dominio del patron.
pub struct CommitReceipt {
    transaction_id: String,
    entry_count: usize,
}

impl Ledger {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `begin transaction` dentro del ejemplo del patron.
    pub fn begin_transaction(&self, id: impl Into<String>) -> Transaction<'_> {
        Transaction {
            ledger: self,
            id: id.into(),
            staged_entries: Vec::new(),
            committed: false,
        }
    }

    /// Modela la operacion `balance` dentro del ejemplo del patron.
    pub fn balance(&self, account: &str) -> i64 {
        *self.balances.borrow().get(account).unwrap_or(&0)
    }

    /// Modela la operacion `events` dentro del ejemplo del patron.
    pub fn events(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}

impl Transaction<'_> {
    /// Modela la operacion `credit` dentro del ejemplo del patron.
    pub fn credit(&mut self, account: impl Into<String>, amount: i64) {
        self.staged_entries.push((account.into(), amount));
    }

    /// Modela la operacion `debit` dentro del ejemplo del patron.
    pub fn debit(&mut self, account: impl Into<String>, amount: i64) {
        self.staged_entries.push((account.into(), -amount));
    }

    /// Modela la operacion `commit` dentro del ejemplo del patron.
    pub fn commit(mut self) -> CommitReceipt {
        let entry_count = self.staged_entries.len();

        {
            let mut balances = self.ledger.balances.borrow_mut();

            for (account, amount) in &self.staged_entries {
                *balances.entry(account.clone()).or_insert(0) += amount;
            }
        }

        self.ledger
            .events
            .borrow_mut()
            .push(format!("{}:commit {} entries", self.id, entry_count));
        self.committed = true;

        CommitReceipt {
            transaction_id: self.id.clone(),
            entry_count,
        }
    }
}

impl Drop for Transaction<'_> {
    /// Operacion `drop` definida por la abstraccion del ejemplo.
    fn drop(&mut self) {
        if self.committed {
            return;
        }

        self.ledger.events.borrow_mut().push(format!(
            "{}:rollback {} staged entries",
            self.id,
            self.staged_entries.len()
        ));
    }
}

impl CommitReceipt {
    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} committed {} entries",
            self.transaction_id, self.entry_count
        )
    }
}
