# Interpreter

Interpreter representa reglas o expresiones de un lenguaje pequeño y las evalúa contra un contexto.

## Idea central

Una expresión se modela como datos. Luego un evaluador interpreta esos datos para responder si una regla se cumple, cuánto descuento aplicar o si un permiso existe.

En Rust suele aparecer como:

- Enums recursivos para árboles de expresión.
- Traits cuando las expresiones deben ser extensibles.
- Parsers pequeños cuando el lenguaje entra como texto.

## Ejemplos del repositorio

- Filtros simples de búsqueda `campo operador valor`: `src/patterns/gof/behavioral/interpreter/search_filter.rs`
- Reglas de descuentos expresadas como árbol: `src/patterns/gof/behavioral/interpreter/discount_rules.rs`
- Mini lenguaje para permisos: `src/patterns/gof/behavioral/interpreter/permission_language.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/interpreter/README.md`
