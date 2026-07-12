# CQRS

CQRS separa el modelo que cambia estado del modelo que responde consultas.

## Idea central

Los comandos representan intención y modifican el lado de escritura. Las consultas no cambian estado y pueden usar modelos de lectura preparados para la pantalla, API o reporte que los consume.

En Rust suele aparecer como:

- Tipos distintos para comandos y consultas.
- Handlers separados para escritura y lectura.
- Modelos de lectura específicos para casos de uso.
- Proyecciones que sincronizan datos desde escritura hacia lectura.

## Ejemplos del repositorio

- Comandos separados de consultas para inventario: `inventory_commands_queries`.
- Modelo de lectura optimizado para dashboard: pendiente.
- Sincronización simple entre escritura y lectura: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/cqrs/README.md`
