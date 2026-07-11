# Strategy

Strategy modela familias de algoritmos intercambiables.

## Idea central

El código cliente conserva una interfaz común para ejecutar una operación, mientras cada estrategia contiene una variante concreta del algoritmo.

En Rust suele aparecer como:

- Traits implementados por varias estrategias.
- Generics cuando la estrategia se decide en compilación.
- `Box<dyn Trait>` cuando la estrategia se decide en tiempo de ejecución.
- Closures para casos pequeños y muy locales.

## Ejemplos del repositorio

- Estrategias de descuento: `src/patterns/gof/behavioral/strategy/discounts.rs`
- Estrategias de ordenamiento de resultados: `src/patterns/gof/behavioral/strategy/result_sorting.rs`
- Estrategias de cálculo de envío: `src/patterns/gof/behavioral/strategy/shipping.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/strategy/README.md`
