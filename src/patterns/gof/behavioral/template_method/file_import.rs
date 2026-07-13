#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ImportReport` usado por el ejemplo para expresar el dominio del patron.
pub struct ImportReport {
    steps: Vec<String>,
    saved_records: usize,
}

impl ImportReport {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(steps: Vec<String>, saved_records: usize) -> Self {
        Self {
            steps,
            saved_records,
        }
    }

    /// Modela la operacion `steps` dentro del ejemplo del patron.
    pub fn steps(&self) -> Vec<&str> {
        self.steps.iter().map(String::as_str).collect()
    }

    /// Modela la operacion `saved records` dentro del ejemplo del patron.
    pub fn saved_records(&self) -> usize {
        self.saved_records
    }
}

/// Contrato publico `FileImporter` que desacopla las piezas del ejemplo.
pub trait FileImporter {
    /// Operacion `source name` definida por la abstraccion del ejemplo.
    fn source_name(&self) -> &'static str;
    /// Operacion `destination` definida por la abstraccion del ejemplo.
    fn destination(&self) -> &'static str;
    /// Operacion `parse records` definida por la abstraccion del ejemplo.
    fn parse_records(&self, input: &str) -> Vec<Vec<String>>;

    /// Operacion `normalize records` definida por la abstraccion del ejemplo.
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

    /// Operacion `validate records` definida por la abstraccion del ejemplo.
    fn validate_records(&self, records: &[Vec<String>]) -> bool {
        records
            .iter()
            .all(|row| row.len() >= 2 && row.iter().all(|field| !field.is_empty()))
    }

    /// Operacion `import` definida por la abstraccion del ejemplo.
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
/// Tipo publico `CsvLeadImporter` usado por el ejemplo para expresar el dominio del patron.
pub struct CsvLeadImporter;

impl FileImporter for CsvLeadImporter {
    /// Operacion `source name` definida por la abstraccion del ejemplo.
    fn source_name(&self) -> &'static str {
        "csv_leads"
    }

    /// Operacion `destination` definida por la abstraccion del ejemplo.
    fn destination(&self) -> &'static str {
        "leads"
    }

    /// Operacion `parse records` definida por la abstraccion del ejemplo.
    fn parse_records(&self, input: &str) -> Vec<Vec<String>> {
        input
            .lines()
            .map(|line| line.split(',').map(str::to_string).collect())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `PipeOrderImporter` usado por el ejemplo para expresar el dominio del patron.
pub struct PipeOrderImporter;

impl FileImporter for PipeOrderImporter {
    /// Operacion `source name` definida por la abstraccion del ejemplo.
    fn source_name(&self) -> &'static str {
        "pipe_orders"
    }

    /// Operacion `destination` definida por la abstraccion del ejemplo.
    fn destination(&self) -> &'static str {
        "orders"
    }

    /// Operacion `parse records` definida por la abstraccion del ejemplo.
    fn parse_records(&self, input: &str) -> Vec<Vec<String>> {
        input
            .lines()
            .map(|line| line.split('|').map(str::to_string).collect())
            .collect()
    }
}
