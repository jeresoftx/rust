# AGENTS.md

Este repositorio es parte de la colección **complementario (patrones)** de
Jeresoft Academy y se rige por la RFC-0001 (manual fundacional).

## Objetivo

Crear el mejor recurso educativo posible sobre **el catálogo de patrones de
diseño en Rust**.

Todo cambio debe mejorar simultáneamente:

- calidad técnica
- claridad
- documentación
- mantenibilidad

## Antes de escribir código

Siempre, en este orden (RFC-0001 §2, §13):

1. Explicar el concepto.
2. Explicar el problema.
3. Comparar alternativas.
4. Justificar la implementación.

## Código

Conforme a RFC-0001 §13:

- Rust idiomático.
- `cargo fmt --check` sin diffs.
- `cargo clippy --all-targets` sin advertencias.
- `cargo test` limpio.
- Sin `unsafe` salvo justificación documentada con comentario `SAFETY`.
- Comentarios solo donde aporten valor educativo o aclaren invariantes.

## Documentación

Toda nueva funcionalidad incluye:

- README actualizado si cambia la estructura, el alcance o cómo correr el repo.
- Documentación del patrón en `patterns/`.
- Diagramas Mermaid cuando ayuden a entender relaciones entre piezas.
- Ejemplos ejecutables.
- Tests.
- Benchmarks cuando apliquen; cuando no apliquen, se declara.

## Nunca

- Agregar dependencias innecesarias.
- Optimizar prematuramente.
- Duplicar código.
- Omitir documentación.
- Publicar capítulos o patrones parciales.

## Filosofía

Este repositorio debe poder utilizarse como un libro de ingeniería. Nunca
sacrificar claridad por ingenio. Explicar el porqué, no solo el cómo. Es un
proyecto de legado sin prisa: la calidad siempre gana sobre la velocidad
(RFC-0001 §1).
