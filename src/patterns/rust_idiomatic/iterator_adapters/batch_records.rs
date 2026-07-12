#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawRecord {
    line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessedRecord {
    entity_id: String,
    event: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchSummary {
    batch_number: usize,
    records: Vec<ProcessedRecord>,
}

pub fn process_record_batches(records: &[RawRecord], batch_size: usize) -> Vec<BatchSummary> {
    if batch_size == 0 {
        return Vec::new();
    }

    records
        .chunks(batch_size)
        .enumerate()
        .map(|(index, batch)| BatchSummary {
            batch_number: index + 1,
            records: batch.iter().filter_map(RawRecord::parse).collect(),
        })
        .filter(|summary| !summary.records.is_empty())
        .collect()
}

impl RawRecord {
    pub fn new(line: impl Into<String>) -> Self {
        Self { line: line.into() }
    }

    fn parse(&self) -> Option<ProcessedRecord> {
        let (entity_id, event) = self.line.split_once(',')?;
        let entity_id = entity_id.trim();
        let event = event.trim();

        if entity_id.is_empty() || event.is_empty() {
            return None;
        }

        Some(ProcessedRecord {
            entity_id: entity_id.to_string(),
            event: event.to_string(),
        })
    }
}

impl BatchSummary {
    pub fn batch_number(&self) -> usize {
        self.batch_number
    }

    pub fn events(&self) -> Vec<String> {
        self.records
            .iter()
            .map(|record| format!("{}:{}", record.entity_id, record.event))
            .collect()
    }

    pub fn summary(&self) -> String {
        format!(
            "batch {}: {} valid records",
            self.batch_number,
            self.records.len()
        )
    }
}
