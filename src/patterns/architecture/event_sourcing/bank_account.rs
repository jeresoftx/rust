#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `AccountId` usado por el ejemplo para expresar el dominio del patron.
pub struct AccountId(String);

impl AccountId {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Money` usado por el ejemplo para expresar el dominio del patron.
pub struct Money {
    cents: i32,
}

impl Money {
    /// Modela la operacion `usd` dentro del ejemplo del patron.
    pub fn usd(cents: i32) -> Self {
        Self { cents }
    }

    /// Operacion `add` definida por la abstraccion del ejemplo.
    fn add(self, other: Self) -> Self {
        Self {
            cents: self.cents + other.cents,
        }
    }

    /// Operacion `subtract` definida por la abstraccion del ejemplo.
    fn subtract(self, other: Self) -> Self {
        Self {
            cents: self.cents - other.cents,
        }
    }

    /// Operacion `is negative` definida por la abstraccion del ejemplo.
    fn is_negative(self) -> bool {
        self.cents < 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `AccountState` dentro del ejemplo.
pub enum AccountState {
    /// Variante `Closed` del estado o error del ejemplo.
    Closed,
    /// Variante `Open` del estado o error del ejemplo.
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `AccountEvent` dentro del ejemplo.
pub enum AccountEvent {
    /// Variante `Opened` del estado o error del ejemplo.
    Opened {
        /// Valor publico `account_id` asociado a la variante `Opened`.
        account_id: String,
    },
    /// Variante `Deposited` del estado o error del ejemplo.
    Deposited {
        /// Valor publico `cents` asociado a la variante `Deposited`.
        cents: i32,
    },
    /// Variante `Withdrawn` del estado o error del ejemplo.
    Withdrawn {
        /// Valor publico `cents` asociado a la variante `Withdrawn`.
        cents: i32,
    },
}

impl AccountEvent {
    /// Modela la operacion `opened` dentro del ejemplo del patron.
    pub fn opened(account_id: impl Into<String>) -> Self {
        Self::Opened {
            account_id: account_id.into(),
        }
    }

    /// Modela la operacion `deposited` dentro del ejemplo del patron.
    pub fn deposited(cents: i32) -> Self {
        Self::Deposited { cents }
    }

    /// Modela la operacion `withdrawn` dentro del ejemplo del patron.
    pub fn withdrawn(cents: i32) -> Self {
        Self::Withdrawn { cents }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Account` usado por el ejemplo para expresar el dominio del patron.
pub struct Account {
    id: AccountId,
    state: AccountState,
    balance: Money,
    pending_events: Vec<AccountEvent>,
}

impl Account {
    /// Modela la operacion `rehydrate` dentro del ejemplo del patron.
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

    /// Modela la operacion `id` dentro del ejemplo del patron.
    pub fn id(&self) -> &AccountId {
        &self.id
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> AccountState {
        self.state
    }

    /// Modela la operacion `balance` dentro del ejemplo del patron.
    pub fn balance(&self) -> Money {
        self.balance
    }

    /// Modela la operacion `withdraw` dentro del ejemplo del patron.
    pub fn withdraw(&mut self, amount: Money) -> Result<(), AccountError> {
        if self.balance.subtract(amount).is_negative() {
            return Err(AccountError::InsufficientFunds);
        }

        let event = AccountEvent::withdrawn(amount.cents);
        self.apply(&event)?;
        self.pending_events.push(event);
        Ok(())
    }

    /// Modela la operacion `pending events` dentro del ejemplo del patron.
    pub fn pending_events(&self) -> Vec<AccountEvent> {
        self.pending_events.clone()
    }

    /// Operacion `apply` definida por la abstraccion del ejemplo.
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
/// Conjunto de estados o errores publicos de `AccountError` dentro del ejemplo.
pub enum AccountError {
    /// Variante `InsufficientFunds` del estado o error del ejemplo.
    InsufficientFunds,
    /// Variante `WrongAccountStream` del estado o error del ejemplo.
    WrongAccountStream,
}
