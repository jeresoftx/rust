#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    cents: i32,
}

impl Money {
    pub fn usd(cents: i32) -> Self {
        Self { cents }
    }

    fn add(self, other: Self) -> Self {
        Self {
            cents: self.cents + other.cents,
        }
    }

    fn subtract(self, other: Self) -> Self {
        Self {
            cents: self.cents - other.cents,
        }
    }

    fn is_negative(self) -> bool {
        self.cents < 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountState {
    Closed,
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountEvent {
    Opened { account_id: String },
    Deposited { cents: i32 },
    Withdrawn { cents: i32 },
}

impl AccountEvent {
    pub fn opened(account_id: impl Into<String>) -> Self {
        Self::Opened {
            account_id: account_id.into(),
        }
    }

    pub fn deposited(cents: i32) -> Self {
        Self::Deposited { cents }
    }

    pub fn withdrawn(cents: i32) -> Self {
        Self::Withdrawn { cents }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    id: AccountId,
    state: AccountState,
    balance: Money,
    pending_events: Vec<AccountEvent>,
}

impl Account {
    pub fn rehydrate(id: AccountId, events: &[AccountEvent]) -> Result<Self, AccountError> {
        let mut account = Self {
            id,
            state: AccountState::Closed,
            balance: Money::usd(0),
            pending_events: Vec::new(),
        };

        for event in events {
            account.apply(event)?;
        }

        Ok(account)
    }

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn state(&self) -> AccountState {
        self.state
    }

    pub fn balance(&self) -> Money {
        self.balance
    }

    pub fn withdraw(&mut self, amount: Money) -> Result<(), AccountError> {
        if self.balance.subtract(amount).is_negative() {
            return Err(AccountError::InsufficientFunds);
        }

        let event = AccountEvent::withdrawn(amount.cents);
        self.apply(&event)?;
        self.pending_events.push(event);
        Ok(())
    }

    pub fn pending_events(&self) -> Vec<AccountEvent> {
        self.pending_events.clone()
    }

    fn apply(&mut self, event: &AccountEvent) -> Result<(), AccountError> {
        match event {
            AccountEvent::Opened { account_id } => {
                if account_id != self.id.as_str() {
                    return Err(AccountError::WrongAccountStream);
                }
                self.state = AccountState::Open;
            }
            AccountEvent::Deposited { cents } => {
                self.balance = self.balance.add(Money::usd(*cents));
            }
            AccountEvent::Withdrawn { cents } => {
                self.balance = self.balance.subtract(Money::usd(*cents));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountError {
    InsufficientFunds,
    WrongAccountStream,
}
