# Iterator Adapters

Iterator Adapters permite construir pipelines de transformación sobre iteradores.

## Idea central

Un iterador produce valores y los adaptadores crean nuevos iteradores con reglas adicionales. El trabajo suele ser perezoso hasta que aparece un consumidor final.

En Rust suele aparecer como:

- `filter` para descartar elementos.
- `map` y `filter_map` para transformar datos.
- `flat_map` para expandir resultados.
- `fold` para agregar sin colecciones intermedias.
- `collect` para materializar el resultado final.

## Ejemplos del repositorio

- Pipeline de filtrado y transformación de órdenes: `src/patterns/rust_idiomatic/iterator_adapters/order_pipeline.rs`
- Procesamiento por lotes de registros: `src/patterns/rust_idiomatic/iterator_adapters/batch_records.rs`
- Agregaciones de reportes sin estructuras intermedias: `src/patterns/rust_idiomatic/iterator_adapters/report_aggregations.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/iterator_adapters/README.md`
