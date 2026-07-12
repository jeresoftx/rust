use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Ledger {
    balances: RefCell<HashMap<String, i64>>,
    events: RefCell<Vec<String>>,
}

#[derive(Debug)]
pub struct Transaction<'a> {
    ledger: &'a Ledger,
    id: String,
    staged_entries: Vec<(String, i64)>,
    committed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitReceipt {
    transaction_id: String,
    entry_count: usize,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_transaction(&self, id: impl Into<String>) -> Transaction<'_> {
        Transaction {
            ledger: self,
            id: id.into(),
            staged_entries: Vec::new(),
            committed: false,
        }
    }

    pub fn balance(&self, account: &str) -> i64 {
        *self.balances.borrow().get(account).unwrap_or(&0)
    }

    pub fn events(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}

impl Transaction<'_> {
    pub fn credit(&mut self, account: impl Into<String>, amount: i64) {
        self.staged_entries.push((account.into(), amount));
    }

    pub fn debit(&mut self, account: impl Into<String>, amount: i64) {
        self.staged_entries.push((account.into(), -amount));
    }

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
    pub fn summary(&self) -> String {
        format!(
            "{} committed {} entries",
            self.transaction_id, self.entry_count
        )
    }
}
