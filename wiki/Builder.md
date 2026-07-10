# Builder

Builder ayuda a construir valores con muchas opciones sin llenar el codigo de constructores largos o parametros ambiguos.

## Idea central

El builder acumula decisiones con metodos claros y termina en `build()`, que devuelve el valor final o un error de validacion.

En Rust aparece mucho como:

- `Type::builder()`
- `Builder::default()`
- Metodos encadenables que consumen o mutan el builder.
- `build() -> Result<T, Error>` cuando faltan datos obligatorios.

## Ejemplos del repositorio

- Configuracion de servidor HTTP: `src/patterns/gof/creational/builder/http_server_config.rs`
- Consultas de busqueda con filtros opcionales: `src/patterns/gof/creational/builder/search_query.rs`
- Payload de email transaccional.

## Guia tecnica

La guia cercana al codigo vive en:

`patterns/gof/creational/builder/README.md`
