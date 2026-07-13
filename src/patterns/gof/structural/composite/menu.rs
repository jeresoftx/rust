/// Conjunto de estados o errores publicos de `MenuItem` dentro del ejemplo.
pub enum MenuItem {
    /// Variante `Link` del estado o error del ejemplo.
    Link {
        /// Valor publico `label` asociado a la variante `Link`.
        label: String,
        /// Valor publico `url` asociado a la variante `Link`.
        url: String,
    },
    /// Variante `Section` del estado o error del ejemplo.
    Section(MenuSection),
}

impl MenuItem {
    /// Modela la operacion `link` dentro del ejemplo del patron.
    pub fn link(label: impl Into<String>, url: impl Into<String>) -> Self {
        Self::Link {
            label: label.into(),
            url: url.into(),
        }
    }

    /// Operacion `render into` definida por la abstraccion del ejemplo.
    fn render_into(&self, depth: usize, lines: &mut Vec<String>) {
        match self {
            Self::Link { label, url } => {
                lines.push(format!("{}{} -> {}", "  ".repeat(depth), label, url));
            }
            Self::Section(section) => section.render_into(depth, lines),
        }
    }

    /// Operacion `find url` definida por la abstraccion del ejemplo.
    fn find_url(&self, label: &str) -> Option<String> {
        match self {
            Self::Link {
                label: item_label,
                url,
            } if item_label == label => Some(url.clone()),
            Self::Link { .. } => None,
            Self::Section(section) => section.find_url(label),
        }
    }
}

impl From<MenuSection> for MenuItem {
    /// Operacion `from` definida por la abstraccion del ejemplo.
    fn from(section: MenuSection) -> Self {
        Self::Section(section)
    }
}

/// Tipo publico `MenuSection` usado por el ejemplo para expresar el dominio del patron.
pub struct MenuSection {
    label: String,
    children: Vec<MenuItem>,
}

impl MenuSection {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
        }
    }

    /// Modela la operacion `with` dentro del ejemplo del patron.
    pub fn with(mut self, item: impl Into<MenuItem>) -> Self {
        self.children.push(item.into());
        self
    }

    /// Modela la operacion `render` dentro del ejemplo del patron.
    pub fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        self.render_into(0, &mut lines);
        lines
    }

    /// Modela la operacion `find url` dentro del ejemplo del patron.
    pub fn find_url(&self, label: &str) -> Option<String> {
        self.children.iter().find_map(|child| child.find_url(label))
    }

    /// Operacion `render into` definida por la abstraccion del ejemplo.
    fn render_into(&self, depth: usize, lines: &mut Vec<String>) {
        lines.push(format!("{}{}", "  ".repeat(depth), self.label));

        for child in &self.children {
            child.render_into(depth + 1, lines);
        }
    }
}
