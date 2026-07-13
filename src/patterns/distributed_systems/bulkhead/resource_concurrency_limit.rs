#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `Resource` dentro del ejemplo.
pub enum Resource {
    /// Variante `Database` del estado o error del ejemplo.
    Database,
    /// Variante `SearchIndex` del estado o error del ejemplo.
    SearchIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `BulkheadError` dentro del ejemplo.
pub enum BulkheadError {
    /// Variante `ResourceFull` del estado o error del ejemplo.
    ResourceFull(Resource),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Permit` usado por el ejemplo para expresar el dominio del patron.
pub struct Permit {
    resource: Resource,
}

#[derive(Debug)]
/// Tipo publico `ResourceBulkhead` usado por el ejemplo para expresar el dominio del patron.
pub struct ResourceBulkhead {
    database_limit: usize,
    search_index_limit: usize,
    active_database: usize,
    active_search_index: usize,
    rejected_database: usize,
    rejected_search_index: usize,
}

impl ResourceBulkhead {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(database_limit: usize, search_index_limit: usize) -> Self {
        Self {
            database_limit,
            search_index_limit,
            active_database: 0,
            active_search_index: 0,
            rejected_database: 0,
            rejected_search_index: 0,
        }
    }

    /// Modela la operacion `acquire` dentro del ejemplo del patron.
    pub fn acquire(&mut self, resource: Resource) -> Result<Permit, BulkheadError> {
        match resource {
            Resource::Database if self.active_database < self.database_limit => {
                self.active_database += 1;
                Ok(Permit { resource })
            }
            Resource::SearchIndex if self.active_search_index < self.search_index_limit => {
                self.active_search_index += 1;
                Ok(Permit { resource })
            }
            Resource::Database => {
                self.rejected_database += 1;
                Err(BulkheadError::ResourceFull(resource))
            }
            Resource::SearchIndex => {
                self.rejected_search_index += 1;
                Err(BulkheadError::ResourceFull(resource))
            }
        }
    }

    /// Modela la operacion `release` dentro del ejemplo del patron.
    pub fn release(&mut self, permit: Permit) {
        match permit.resource {
            Resource::Database => {
                self.active_database = self.active_database.saturating_sub(1);
            }
            Resource::SearchIndex => {
                self.active_search_index = self.active_search_index.saturating_sub(1);
            }
        }
    }

    /// Modela la operacion `active` dentro del ejemplo del patron.
    pub fn active(&self, resource: Resource) -> usize {
        match resource {
            Resource::Database => self.active_database,
            Resource::SearchIndex => self.active_search_index,
        }
    }

    /// Modela la operacion `rejected` dentro del ejemplo del patron.
    pub fn rejected(&self, resource: Resource) -> usize {
        match resource {
            Resource::Database => self.rejected_database,
            Resource::SearchIndex => self.rejected_search_index,
        }
    }
}
