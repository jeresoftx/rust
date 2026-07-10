pub enum MenuItem {
    Link { label: String, url: String },
    Section(MenuSection),
}

impl MenuItem {
    pub fn link(label: impl Into<String>, url: impl Into<String>) -> Self {
        Self::Link {
            label: label.into(),
            url: url.into(),
        }
    }

    fn render_into(&self, depth: usize, lines: &mut Vec<String>) {
        match self {
            Self::Link { label, url } => {
                lines.push(format!("{}{} -> {}", "  ".repeat(depth), label, url));
            }
            Self::Section(section) => section.render_into(depth, lines),
        }
    }

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
    fn from(section: MenuSection) -> Self {
        Self::Section(section)
    }
}

pub struct MenuSection {
    label: String,
    children: Vec<MenuItem>,
}

impl MenuSection {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
        }
    }

    pub fn with(mut self, item: impl Into<MenuItem>) -> Self {
        self.children.push(item.into());
        self
    }

    pub fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        self.render_into(0, &mut lines);
        lines
    }

    pub fn find_url(&self, label: &str) -> Option<String> {
        self.children.iter().find_map(|child| child.find_url(label))
    }

    fn render_into(&self, depth: usize, lines: &mut Vec<String>) {
        lines.push(format!("{}{}", "  ".repeat(depth), self.label));

        for child in &self.children {
            child.render_into(depth + 1, lines);
        }
    }
}
