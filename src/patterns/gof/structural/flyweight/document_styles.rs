use std::collections::HashMap;
use std::sync::Arc;

/// Tipo publico `DocumentStyleRegistry` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentStyleRegistry {
    styles: HashMap<String, Arc<DocumentStyle>>,
}

impl DocumentStyleRegistry {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
        }
    }

    /// Modela la operacion `style` dentro del ejemplo del patron.
    pub fn style(&mut self, name: &str, font: &str, size: u8, bold: bool) -> Arc<DocumentStyle> {
        self.styles
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(DocumentStyle::new(name, font, size, bold)))
            .clone()
    }
}

impl Default for DocumentStyleRegistry {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}

/// Tipo publico `DocumentStyle` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentStyle {
    name: String,
    font: String,
    size: u8,
    bold: bool,
}

impl DocumentStyle {
    /// Operacion `new` definida por la abstraccion del ejemplo.
    fn new(name: &str, font: &str, size: u8, bold: bool) -> Self {
        Self {
            name: name.to_string(),
            font: font.to_string(),
            size,
            bold,
        }
    }

    /// Operacion `weight` definida por la abstraccion del ejemplo.
    fn weight(&self) -> &'static str {
        if self.bold { "bold" } else { "regular" }
    }
}

/// Tipo publico `TextRun` usado por el ejemplo para expresar el dominio del patron.
pub struct TextRun {
    text: String,
    style: Arc<DocumentStyle>,
}

impl TextRun {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(text: impl Into<String>, style: Arc<DocumentStyle>) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    /// Modela la operacion `shares style with` dentro del ejemplo del patron.
    pub fn shares_style_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.style, &other.style)
    }

    /// Modela la operacion `render` dentro del ejemplo del patron.
    pub fn render(&self) -> String {
        format!(
            "[{} {} {} {}] {}",
            self.style.name,
            self.style.font,
            self.style.size,
            self.style.weight(),
            self.text
        )
    }
}
