#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterSnapshot {
    version: u64,
    value: i32,
}

impl CounterSnapshot {
    pub fn new(version: u64, value: i32) -> Self {
        Self { version, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterEvent {
    Incremented { version: u64, amount: i32 },
    Decremented { version: u64, amount: i32 },
}

impl CounterEvent {
    pub fn incremented(version: u64, amount: i32) -> Self {
        Self::Incremented { version, amount }
    }

    pub fn decremented(version: u64, amount: i32) -> Self {
        Self::Decremented { version, amount }
    }

    fn version(&self) -> u64 {
        match self {
            Self::Incremented { version, .. } | Self::Decremented { version, .. } => *version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    version: u64,
    value: i32,
}

impl Counter {
    pub fn restored(version: u64, value: i32) -> Self {
        Self { version, value }
    }

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

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn value(&self) -> i32 {
        self.value
    }

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
pub struct RebuildReport {
    counter: Counter,
    events_replayed: usize,
}

impl RebuildReport {
    pub fn new(counter: Counter, events_replayed: usize) -> Self {
        Self {
            counter,
            events_replayed,
        }
    }

    pub fn counter(&self) -> &Counter {
        &self.counter
    }

    pub fn events_replayed(&self) -> usize {
        self.events_replayed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    VersionGap { expected: u64, actual: u64 },
}
