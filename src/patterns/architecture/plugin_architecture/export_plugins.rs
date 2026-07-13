use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ExportRecord` usado por el ejemplo para expresar el dominio del patron.
pub struct ExportRecord {
    id: String,
    amount_cents: u32,
}

impl ExportRecord {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, amount_cents: u32) -> Self {
        Self {
            id: id.into(),
            amount_cents,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ExportError` dentro del ejemplo.
pub enum ExportError {
    /// Variante `PluginNotFound` del estado o error del ejemplo.
    PluginNotFound(String),
}

trait ExportPlugin {
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str;
    /// Operacion `export` definida por la abstraccion del ejemplo.
    fn export(&self, records: &[ExportRecord]) -> String;
}

#[derive(Default)]
/// Tipo publico `ExportRegistry` usado por el ejemplo para expresar el dominio del patron.
pub struct ExportRegistry {
    plugins: HashMap<String, Box<dyn ExportPlugin>>,
}

impl ExportRegistry {
    /// Modela la operacion `with default plugins` dentro del ejemplo del patron.
    pub fn with_default_plugins() -> Self {
        let mut registry = Self::default();
        registry.register(JsonExportPlugin);
        registry.register(CsvExportPlugin);
        registry.register(TextExportPlugin);
        registry
    }

    /// Operacion `register` definida por la abstraccion del ejemplo.
    fn register(&mut self, plugin: impl ExportPlugin + 'static) {
        self.plugins
            .insert(plugin.key().to_string(), Box::new(plugin));
    }

    /// Modela la operacion `export` dentro del ejemplo del patron.
    pub fn export(&self, key: &str, records: &[ExportRecord]) -> Result<String, ExportError> {
        self.plugins
            .get(key)
            .map(|plugin| plugin.export(records))
            .ok_or_else(|| ExportError::PluginNotFound(key.to_string()))
    }
}

struct JsonExportPlugin;

impl ExportPlugin for JsonExportPlugin {
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str {
        "json"
    }

    /// Operacion `export` definida por la abstraccion del ejemplo.
    fn export(&self, records: &[ExportRecord]) -> String {
        let items = records
            .iter()
            .map(|record| {
                format!(
                    r#"{{"id":"{}","amount_cents":{}}}"#,
                    record.id, record.amount_cents
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        format!("[{items}]")
    }
}

struct CsvExportPlugin;

impl ExportPlugin for CsvExportPlugin {
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str {
        "csv"
    }

    /// Operacion `export` definida por la abstraccion del ejemplo.
    fn export(&self, records: &[ExportRecord]) -> String {
        let rows = records
            .iter()
            .map(|record| format!("{},{}", record.id, record.amount_cents))
            .collect::<Vec<_>>()
            .join("\n");

        format!("id,amount_cents\n{rows}")
    }
}

struct TextExportPlugin;

impl ExportPlugin for TextExportPlugin {
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str {
        "text"
    }

    /// Operacion `export` definida por la abstraccion del ejemplo.
    fn export(&self, records: &[ExportRecord]) -> String {
        records
            .iter()
            .map(|record| {
                format!(
                    "{}: ${}.{:02}",
                    record.id,
                    record.amount_cents / 100,
                    record.amount_cents % 100
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
