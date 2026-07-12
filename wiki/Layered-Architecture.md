# Layered Architecture

Layered Architecture separa presentación, aplicación, dominio e infraestructura para que cada capa tenga una responsabilidad clara.

## Idea central

La capa de presentación traduce entradas y salidas. La capa de aplicación coordina casos de uso. El dominio contiene reglas de negocio. La infraestructura implementa detalles técnicos como repositorios o clientes externos.

En Rust suele aparecer como:

- Módulos por capa.
- DTOs separados de entidades.
- Traits para reemplazar dependencias en pruebas.
- Casos de uso que coordinan dominio e infraestructura.

## Ejemplos del repositorio

- API de usuarios con capas de presentación, aplicación y dominio: `user_api`.
- Separación entre DTOs, entidades y repositorios: pendiente.
- Pruebas por capa con dependencias reemplazables: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/layered_architecture/README.md`
