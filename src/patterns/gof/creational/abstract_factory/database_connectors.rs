/// Contrato publico `DatabaseConnection` que desacopla las piezas del ejemplo.
pub trait DatabaseConnection {
    /// Operacion `connection string` definida por la abstraccion del ejemplo.
    fn connection_string(&self) -> &'static str;
}

/// Contrato publico `QueryBuilder` que desacopla las piezas del ejemplo.
pub trait QueryBuilder {
    /// Operacion `select all` definida por la abstraccion del ejemplo.
    fn select_all(&self, collection: &str) -> String;
}

/// Contrato publico `DatabaseFactory` que desacopla las piezas del ejemplo.
pub trait DatabaseFactory {
    /// Operacion `create connection` definida por la abstraccion del ejemplo.
    fn create_connection(&self) -> Box<dyn DatabaseConnection>;
    /// Operacion `create query builder` definida por la abstraccion del ejemplo.
    fn create_query_builder(&self) -> Box<dyn QueryBuilder>;
}

/// Tipo publico `SqlDatabaseFactory` usado por el ejemplo para expresar el dominio del patron.
pub struct SqlDatabaseFactory;

impl DatabaseFactory for SqlDatabaseFactory {
    /// Operacion `create connection` definida por la abstraccion del ejemplo.
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(PostgresConnection)
    }

    /// Operacion `create query builder` definida por la abstraccion del ejemplo.
    fn create_query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(SqlQueryBuilder)
    }
}

/// Tipo publico `NoSqlDatabaseFactory` usado por el ejemplo para expresar el dominio del patron.
pub struct NoSqlDatabaseFactory;

impl DatabaseFactory for NoSqlDatabaseFactory {
    /// Operacion `create connection` definida por la abstraccion del ejemplo.
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(MongoConnection)
    }

    /// Operacion `create query builder` definida por la abstraccion del ejemplo.
    fn create_query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(DocumentQueryBuilder)
    }
}

struct PostgresConnection;

impl DatabaseConnection for PostgresConnection {
    /// Operacion `connection string` definida por la abstraccion del ejemplo.
    fn connection_string(&self) -> &'static str {
        "postgres://primary"
    }
}

struct MongoConnection;

impl DatabaseConnection for MongoConnection {
    /// Operacion `connection string` definida por la abstraccion del ejemplo.
    fn connection_string(&self) -> &'static str {
        "mongodb://cluster"
    }
}

struct SqlQueryBuilder;

impl QueryBuilder for SqlQueryBuilder {
    /// Operacion `select all` definida por la abstraccion del ejemplo.
    fn select_all(&self, collection: &str) -> String {
        format!("SELECT * FROM {collection};")
    }
}

struct DocumentQueryBuilder;

impl QueryBuilder for DocumentQueryBuilder {
    /// Operacion `select all` definida por la abstraccion del ejemplo.
    fn select_all(&self, collection: &str) -> String {
        format!("db.{collection}.find({{}})")
    }
}

/// Modela la operacion `describe database stack` dentro del ejemplo del patron.
pub fn describe_database_stack(factory: &dyn DatabaseFactory, collection: &str) -> String {
    let connection = factory.create_connection();
    let query_builder = factory.create_query_builder();

    format!(
        "connection={} | query={}",
        connection.connection_string(),
        query_builder.select_all(collection)
    )
}
