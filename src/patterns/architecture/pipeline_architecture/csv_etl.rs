#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ImportedCustomer` usado por el ejemplo para expresar el dominio del patron.
pub struct ImportedCustomer {
    id: String,
    name: String,
    email: String,
}

impl ImportedCustomer {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ImportError` usado por el ejemplo para expresar el dominio del patron.
pub struct ImportError {
    row: usize,
    message: String,
}

impl ImportError {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(row: usize, message: impl Into<String>) -> Self {
        Self {
            row,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ImportReport` usado por el ejemplo para expresar el dominio del patron.
pub struct ImportReport {
    imported: Vec<ImportedCustomer>,
    errors: Vec<ImportError>,
}

impl ImportReport {
    /// Modela la operacion `imported` dentro del ejemplo del patron.
    pub fn imported(&self) -> Vec<ImportedCustomer> {
        self.imported.clone()
    }

    /// Modela la operacion `errors` dentro del ejemplo del patron.
    pub fn errors(&self) -> Vec<ImportError> {
        self.errors.clone()
    }
}

#[derive(Debug, Default)]
/// Tipo publico `CsvImportPipeline` usado por el ejemplo para expresar el dominio del patron.
pub struct CsvImportPipeline;

impl CsvImportPipeline {
    /// Modela la operacion `import` dentro del ejemplo del patron.
    pub fn import(&self, input: &str) -> ImportReport {
        let mut lines = input.lines();
        let header = lines.next().unwrap_or_default();

        if header != "id,name,email" {
            return ImportReport {
                imported: Vec::new(),
                errors: vec![ImportError::new(1, "encabezado esperado: id,name,email")],
            };
        }

        let mut imported = Vec::new();
        let mut errors = Vec::new();

        for (index, line) in lines.enumerate() {
            let row = index + 2;
            let fields = parse_row(line);

            let Some(fields) = fields else {
                errors.push(ImportError::new(row, "fila inválida"));
                continue;
            };

            match normalize_and_validate(row, fields) {
                Ok(customer) => imported.push(customer),
                Err(error) => errors.push(error),
            }
        }

        ImportReport { imported, errors }
    }
}

/// Operacion `parse row` definida por la abstraccion del ejemplo.
fn parse_row(line: &str) -> Option<[String; 3]> {
    let mut fields = line.split(',').map(str::trim);
    let id = fields.next()?.to_string();
    let name = fields.next()?.to_string();
    let email = fields.next()?.to_string();

    if fields.next().is_some() {
        return None;
    }

    Some([id, name, email])
}

/// Operacion `normalize and validate` definida por la abstraccion del ejemplo.
fn normalize_and_validate(
    row: usize,
    fields: [String; 3],
) -> Result<ImportedCustomer, ImportError> {
    let [id, name, email] = fields;
    let email = email.to_lowercase();

    if id.is_empty() {
        return Err(ImportError::new(row, "id requerido"));
    }

    if !email.contains('@') {
        return Err(ImportError::new(row, "email inválido"));
    }

    Ok(ImportedCustomer::new(id, name, email))
}
