#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CounterSnapshot` usado por el ejemplo para expresar el dominio del patron.
pub struct CounterSnapshot {
    version: u64,
    value: i32,
}

impl CounterSnapshot {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(version: u64, value: i32) -> Self {
        Self { version, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CounterEvent` dentro del ejemplo.
pub enum CounterEvent {
    /// Variante `Incremented` del estado o error del ejemplo.
    Incremented {
        /// Valor publico `version` asociado a la variante `Incremented`.
        version: u64,
        /// Valor publico `amount` asociado a la variante `Incremented`.
        amount: i32,
    },
    /// Variante `Decremented` del estado o error del ejemplo.
    Decremented {
        /// Valor publico `version` asociado a la variante `Decremented`.
        version: u64,
        /// Valor publico `amount` asociado a la variante `Decremented`.
        amount: i32,
    },
}

impl CounterEvent {
    /// Modela la operacion `incremented` dentro del ejemplo del patron.
    pub fn incremented(version: u64, amount: i32) -> Self {
        Self::Incremented { version, amount }
    }

    /// Modela la operacion `decremented` dentro del ejemplo del patron.
    pub fn decremented(version: u64, amount: i32) -> Self {
        Self::Decremented { version, amount }
    }

    /// Operacion `version` definida por la abstraccion del ejemplo.
    fn version(&self) -> u64 {
        match self {
            Self::Incremented { version, .. } | Self::Decremented { version, .. } => *version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Counter` usado por el ejemplo para expresar el dominio del patron.
pub struct Counter {
    version: u64,
    value: i32,
}

impl Counter {
    /// Modela la operacion `restored` dentro del ejemplo del patron.
    pub fn restored(version: u64, value: i32) -> Self {
        Self { version, value }
    }

    /// Modela la operacion `rebuild from snapshot` dentro del ejemplo del patron.
    pub fn rebuild_from_snapshot(
        snapshot: CounterSnapshot,
        events: &[CounterEvent],
    ) -> Result<RebuildReport, SnapshotError> {
        let mut counter = Self::restored(snapshot.version, snapshot.value);
        let mut replayed = 0;

        for event in events
            .iter()
            .filter(|event| event.version() > snapshot.version)
        {
            counter.apply(event)?;
            replayed += 1;
        }

        Ok(RebuildReport::new(counter, replayed))
    }

    /// Modela la operacion `version` dentro del ejemplo del patron.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Modela la operacion `value` dentro del ejemplo del patron.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&mut self, event: &CounterEvent) -> Result<(), SnapshotError> {
        if event.version() != self.version + 1 {
            return Err(SnapshotError::VersionGap {
                expected: self.version + 1,
                actual: event.version(),
            });
        }

        match event {
            CounterEvent::Incremented { version, amount } => {
                self.version = *version;
                self.value += amount;
            }
            CounterEvent::Decremented { version, amount } => {
                self.version = *version;
                self.value -= amount;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RebuildReport` usado por el ejemplo para expresar el dominio del patron.
pub struct RebuildReport {
    counter: Counter,
    events_replayed: usize,
}

impl RebuildReport {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(counter: Counter, events_replayed: usize) -> Self {
        Self {
            counter,
            events_replayed,
        }
    }

    /// Modela la operacion `counter` dentro del ejemplo del patron.
    pub fn counter(&self) -> &Counter {
        &self.counter
    }

    /// Modela la operacion `events replayed` dentro del ejemplo del patron.
    pub fn events_replayed(&self) -> usize {
        self.events_replayed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `SnapshotError` dentro del ejemplo.
pub enum SnapshotError {
    /// Variante `VersionGap` del estado o error del ejemplo.
    VersionGap {
        /// Valor publico `expected` asociado a la variante `VersionGap`.
        expected: u64,
        /// Valor publico `actual` asociado a la variante `VersionGap`.
        actual: u64,
    },
}
