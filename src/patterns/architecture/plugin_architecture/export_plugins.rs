use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportRecord {
    id: String,
    amount_cents: u32,
}

impl ExportRecord {
    pub fn new(id: impl Into<String>, amount_cents: u32) -> Self {
        Self {
            id: id.into(),
            amount_cents,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    PluginNotFound(String),
}

trait ExportPlugin {
    fn key(&self) -> &'static str;
    fn export(&self, records: &[ExportRecord]) -> String;
}

#[derive(Default)]
pub struct ExportRegistry {
    plugins: HashMap<String, Box<dyn ExportPlugin>>,
}

impl ExportRegistry {
    pub fn with_default_plugins() -> Self {
        let mut registry = Self::default();
        registry.register(JsonExportPlugin);
        registry.register(CsvExportPlugin);
        registry.register(TextExportPlugin);
        registry
    }

    fn register(&mut self, plugin: impl ExportPlugin + 'static) {
        self.plugins
            .insert(plugin.key().to_string(), Box::new(plugin));
    }

    pub fn export(&self, key: &str, records: &[ExportRecord]) -> Result<String, ExportError> {
        self.plugins
            .get(key)
            .map(|plugin| plugin.export(records))
            .ok_or_else(|| ExportError::PluginNotFound(key.to_string()))
    }
}

struct JsonExportPlugin;

impl ExportPlugin for JsonExportPlugin {
    fn key(&self) -> &'static str {
        "json"
    }

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
    fn key(&self) -> &'static str {
        "csv"
    }

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
    fn key(&self) -> &'static str {
        "text"
    }

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
