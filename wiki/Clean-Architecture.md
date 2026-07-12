# Clean Architecture

Clean Architecture coloca las reglas de negocio en el centro y empuja los detalles técnicos hacia los bordes.

## Idea central

Las entidades y los casos de uso no dependen de frameworks, bases de datos, HTTP ni CLI. Los adaptadores externos traducen datos hacia el núcleo y presentan respuestas hacia afuera.

En Rust suele aparecer como:

- Módulos para separar entidades, casos de uso, gateways, controladores y presenters.
- Traits para describir dependencias externas.
- Tipos de entrada y salida propios del núcleo.
- Adaptadores HTTP, CLI, memoria o infraestructura que implementan los límites.

## Ejemplos del repositorio

- Entidades, casos de uso, gateways y controladores: `registration_flow`.
- Reglas de negocio independientes de framework: `framework_independent_rules`.
- Presenter para respuesta HTTP y respuesta CLI: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/clean_architecture/README.md`
