use std::collections::HashMap;
use std::sync::Arc;

pub struct DocumentStyleRegistry {
    styles: HashMap<String, Arc<DocumentStyle>>,
}

impl DocumentStyleRegistry {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
        }
    }

    pub fn style(&mut self, name: &str, font: &str, size: u8, bold: bool) -> Arc<DocumentStyle> {
        self.styles
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(DocumentStyle::new(name, font, size, bold)))
            .clone()
    }
}

impl Default for DocumentStyleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DocumentStyle {
    name: String,
    font: String,
    size: u8,
    bold: bool,
}

impl DocumentStyle {
    fn new(name: &str, font: &str, size: u8, bold: bool) -> Self {
        Self {
            name: name.to_string(),
            font: font.to_string(),
            size,
            bold,
        }
    }

    fn weight(&self) -> &'static str {
        if self.bold { "bold" } else { "regular" }
    }
}

pub struct TextRun {
    text: String,
    style: Arc<DocumentStyle>,
}

impl TextRun {
    pub fn new(text: impl Into<String>, style: Arc<DocumentStyle>) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    pub fn shares_style_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.style, &other.style)
    }

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
