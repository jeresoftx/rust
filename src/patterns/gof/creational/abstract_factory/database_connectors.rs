pub trait DatabaseConnection {
    fn connection_string(&self) -> &'static str;
}

pub trait QueryBuilder {
    fn select_all(&self, collection: &str) -> String;
}

pub trait DatabaseFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection>;
    fn create_query_builder(&self) -> Box<dyn QueryBuilder>;
}

pub struct SqlDatabaseFactory;

impl DatabaseFactory for SqlDatabaseFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(PostgresConnection)
    }

    fn create_query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(SqlQueryBuilder)
    }
}

pub struct NoSqlDatabaseFactory;

impl DatabaseFactory for NoSqlDatabaseFactory {
    fn create_connection(&self) -> Box<dyn DatabaseConnection> {
        Box::new(MongoConnection)
    }

    fn create_query_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(DocumentQueryBuilder)
    }
}

struct PostgresConnection;

impl DatabaseConnection for PostgresConnection {
    fn connection_string(&self) -> &'static str {
        "postgres://primary"
    }
}

struct MongoConnection;

impl DatabaseConnection for MongoConnection {
    fn connection_string(&self) -> &'static str {
        "mongodb://cluster"
    }
}

struct SqlQueryBuilder;

impl QueryBuilder for SqlQueryBuilder {
    fn select_all(&self, collection: &str) -> String {
        format!("SELECT * FROM {collection};")
    }
}

struct DocumentQueryBuilder;

impl QueryBuilder for DocumentQueryBuilder {
    fn select_all(&self, collection: &str) -> String {
        format!("db.{collection}.find({{}})")
    }
}

pub fn describe_database_stack(factory: &dyn DatabaseFactory, collection: &str) -> String {
    let connection = factory.create_connection();
    let query_builder = factory.create_query_builder();

    format!(
        "connection={} | query={}",
        connection.connection_string(),
        query_builder.select_all(collection)
    )
}
