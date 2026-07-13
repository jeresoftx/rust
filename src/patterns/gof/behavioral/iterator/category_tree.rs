#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Category` usado por el ejemplo para expresar el dominio del patron.
pub struct Category {
    name: String,
    children: Vec<Category>,
}

impl Category {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    /// Modela la operacion `with child` dentro del ejemplo del patron.
    pub fn with_child(mut self, child: Category) -> Self {
        self.children.push(child);
        self
    }

    /// Modela la operacion `depth first` dentro del ejemplo del patron.
    pub fn depth_first(&self) -> DepthFirstCategories<'_> {
        DepthFirstCategories { stack: vec![self] }
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `DepthFirstCategories` usado por el ejemplo para expresar el dominio del patron.
pub struct DepthFirstCategories<'a> {
    stack: Vec<&'a Category>,
}

impl<'a> Iterator for DepthFirstCategories<'a> {
    /// Tipo asociado `Item` producido por la abstraccion del ejemplo.
    type Item = &'a str;

    /// Operacion `next` definida por la abstraccion del ejemplo.
    fn next(&mut self) -> Option<Self::Item> {
        let category = self.stack.pop()?;

        for child in category.children.iter().rev() {
            self.stack.push(child);
        }

        Some(&category.name)
    }
}
