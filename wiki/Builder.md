# Builder

Builder ayuda a construir valores con muchas opciones sin llenar el código de constructores largos o parámetros ambiguos.

## Idea central

El builder acumula decisiones con métodos claros y termina en `build()`, que devuelve el valor final o un error de validación.

En Rust aparece mucho como:

- `Type::builder()`
- `Builder::default()`
- Métodos encadenables que consumen o mutan el builder.
- `build() -> Result<T, Error>` cuando faltan datos obligatorios.

## Ejemplos del repositorio

- Configuración de servidor HTTP: `src/patterns/gof/creational/builder/http_server_config.rs`
- Consultas de búsqueda con filtros opcionales: `src/patterns/gof/creational/builder/search_query.rs`
- Payload de email transaccional: `src/patterns/gof/creational/builder/transactional_email.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/creational/builder/README.md`
