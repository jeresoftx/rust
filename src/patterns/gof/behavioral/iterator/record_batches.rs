#[derive(Debug, Clone)]
pub struct RecordBatcher {
    records: Vec<u32>,
    batch_size: usize,
    index: usize,
}

impl RecordBatcher {
    pub fn new(records: Vec<u32>, batch_size: usize) -> Self {
        Self {
            records,
            batch_size,
            index: 0,
        }
    }

    pub fn try_new(records: Vec<u32>, batch_size: usize) -> Result<Self, String> {
        if batch_size == 0 {
            return Err("batch size must be greater than zero".to_string());
        }

        Ok(Self::new(records, batch_size))
    }
}

impl Iterator for RecordBatcher {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.records.len() {
            return None;
        }

        let end = (self.index + self.batch_size).min(self.records.len());
        let batch = self.records[self.index..end].to_vec();
        self.index = end;
        Some(batch)
    }
}
