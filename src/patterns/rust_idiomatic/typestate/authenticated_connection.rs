use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Connected` usado por el ejemplo para expresar el dominio del patron.
pub struct Connected;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Authenticated` usado por el ejemplo para expresar el dominio del patron.
pub struct Authenticated;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Disconnected` usado por el ejemplo para expresar el dominio del patron.
pub struct Disconnected;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Connection` usado por el ejemplo para expresar el dominio del patron.
pub struct Connection<State> {
    database: String,
    user: Option<String>,
    executed_queries: Vec<String>,
    state: PhantomData<State>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `QueryResult` usado por el ejemplo para expresar el dominio del patron.
pub struct QueryResult {
    database: String,
    user: String,
    sql: String,
}

impl Connection<Connected> {
    /// Modela la operacion `connect` dentro del ejemplo del patron.
    pub fn connect(database: impl Into<String>) -> Self {
        Self {
            database: database.into(),
            user: None,
            executed_queries: Vec::new(),
            state: PhantomData,
        }
    }

    /// Modela la operacion `authenticate` dentro del ejemplo del patron.
    pub fn authenticate(
        self,
        user: impl Into<String>,
        password: impl AsRef<str>,
    ) -> Result<Connection<Authenticated>, String> {
        let user = user.into();

        if user.trim().is_empty() {
            return Err("user is required".to_string());
        }

        if password.as_ref().is_empty() {
            return Err("password is required".to_string());
        }

        Ok(Connection {
            database: self.database,
            user: Some(user),
            executed_queries: self.executed_queries,
            state: PhantomData,
        })
    }
}

impl Connection<Authenticated> {
    /// Modela la operacion `query` dentro del ejemplo del patron.
    pub fn query(&mut self, sql: impl Into<String>) -> QueryResult {
        let sql = sql.into();
        self.executed_queries.push(sql.clone());

        QueryResult {
            database: self.database.clone(),
            user: self
                .user
                .clone()
                .expect("Authenticated siempre contiene user"),
            sql,
        }
    }

    /// Modela la operacion `executed queries` dentro del ejemplo del patron.
    pub fn executed_queries(&self) -> Vec<&str> {
        self.executed_queries.iter().map(String::as_str).collect()
    }

    /// Modela la operacion `disconnect` dentro del ejemplo del patron.
    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection {
            database: self.database,
            user: None,
            executed_queries: self.executed_queries,
            state: PhantomData,
        }
    }
}

impl Connection<Disconnected> {
    /// Modela la operacion `database` dentro del ejemplo del patron.
    pub fn database(&self) -> &str {
        &self.database
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> &str {
        "disconnected"
    }
}

impl QueryResult {
    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!("{}:{} -> {}", self.database, self.user, self.sql)
    }
}
