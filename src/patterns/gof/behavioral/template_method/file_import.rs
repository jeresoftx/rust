#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReport {
    steps: Vec<String>,
    saved_records: usize,
}

impl ImportReport {
    pub fn new(steps: Vec<String>, saved_records: usize) -> Self {
        Self {
            steps,
            saved_records,
        }
    }

    pub fn steps(&self) -> Vec<&str> {
        self.steps.iter().map(String::as_str).collect()
    }

    pub fn saved_records(&self) -> usize {
        self.saved_records
    }
}

pub trait FileImporter {
    fn source_name(&self) -> &'static str;
    fn destination(&self) -> &'static str;
    fn parse_records(&self, input: &str) -> Vec<Vec<String>>;

    fn normalize_records(&self, records: Vec<Vec<String>>) -> Vec<Vec<String>> {
        records
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|field| field.trim().to_string())
                    .collect()
            })
            .collect()
    }

    fn validate_records(&self, records: &[Vec<String>]) -> bool {
        records
            .iter()
            .all(|row| row.len() >= 2 && row.iter().all(|field| !field.is_empty()))
    }

    fn import(&self, input: &str) -> ImportReport {
        let mut steps = vec![format!("read:{}", self.source_name())];
        let parsed = self.parse_records(input);
        steps.push(format!("parse:{}", parsed.len()));

        let normalized = self.normalize_records(parsed);
        steps.push(format!("normalize:{}", normalized.len()));

        if !self.validate_records(&normalized) {
            steps.push("validate:failed".to_string());
            return ImportReport::new(steps, 0);
        }

        steps.push(format!("validate:{}", normalized.len()));
        steps.push(format!("save:{}:{}", self.destination(), normalized.len()));

        ImportReport::new(steps, normalized.len())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CsvLeadImporter;

impl FileImporter for CsvLeadImporter {
    fn source_name(&self) -> &'static str {
        "csv_leads"
    }

    fn destination(&self) -> &'static str {
        "leads"
    }

    fn parse_records(&self, input: &str) -> Vec<Vec<String>> {
        input
            .lines()
            .map(|line| line.split(',').map(str::to_string).collect())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PipeOrderImporter;

impl FileImporter for PipeOrderImporter {
    fn source_name(&self) -> &'static str {
        "pipe_orders"
    }

    fn destination(&self) -> &'static str {
        "orders"
    }

    fn parse_records(&self, input: &str) -> Vec<Vec<String>> {
        input
            .lines()
            .map(|line| line.split('|').map(str::to_string).collect())
            .collect()
    }
}
