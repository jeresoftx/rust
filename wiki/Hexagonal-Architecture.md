# Hexagonal Architecture

Hexagonal Architecture protege el núcleo de la aplicación usando puertos y adaptadores.

## Idea central

Los casos de uso definen los puertos que necesitan. Los adaptadores externos implementan esos puertos para bases de datos, APIs, notificaciones o interfaces de usuario.

En Rust suele aparecer como:

- Traits como puertos de salida.
- Casos de uso genéricos sobre esos traits.
- Adaptadores en memoria para pruebas.
- Adaptadores externos simulados o reales.
- Notificadores o gateways intercambiables.

## Ejemplos del repositorio

- Caso de uso de checkout con puertos y adaptadores: `checkout_ports_adapters`.
- Repositorio en memoria y repositorio simulado externo: `repository_adapters`.
- Adaptador de notificaciones intercambiable: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/hexagonal_architecture/README.md`
