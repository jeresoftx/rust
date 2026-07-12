#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Database,
    SearchIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkheadError {
    ResourceFull(Resource),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permit {
    resource: Resource,
}

#[derive(Debug)]
pub struct ResourceBulkhead {
    database_limit: usize,
    search_index_limit: usize,
    active_database: usize,
    active_search_index: usize,
    rejected_database: usize,
    rejected_search_index: usize,
}

impl ResourceBulkhead {
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

    pub fn active(&self, resource: Resource) -> usize {
        match resource {
            Resource::Database => self.active_database,
            Resource::SearchIndex => self.active_search_index,
        }
    }

    pub fn rejected(&self, resource: Resource) -> usize {
        match resource {
            Resource::Database => self.rejected_database,
            Resource::SearchIndex => self.rejected_search_index,
        }
    }
}
