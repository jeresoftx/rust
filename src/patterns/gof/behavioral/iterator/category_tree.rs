#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    name: String,
    children: Vec<Category>,
}

impl Category {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: Category) -> Self {
        self.children.push(child);
        self
    }

    pub fn depth_first(&self) -> DepthFirstCategories<'_> {
        DepthFirstCategories { stack: vec![self] }
    }
}

#[derive(Debug, Clone)]
pub struct DepthFirstCategories<'a> {
    stack: Vec<&'a Category>,
}

impl<'a> Iterator for DepthFirstCategories<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let category = self.stack.pop()?;

        for child in category.children.iter().rev() {
            self.stack.push(child);
        }

        Some(&category.name)
    }
}
