# Iterator Adapters

## Intención

Iterator Adapters encadena transformaciones sobre iteradores para expresar procesamiento de datos como un pipeline claro, perezoso y componible.

## Problema cotidiano

En sistemas reales es común recorrer datos para filtrar, transformar, agrupar o agregar:

- Órdenes que deben filtrarse antes de generar líneas de facturación.
- Registros que llegan en lotes y deben normalizarse.
- Reportes que agregan métricas sin construir estructuras intermedias innecesarias.

Si todo se resuelve con ciclos manuales, la intención puede perderse entre variables temporales, mutaciones y acumuladores.

## Cómo se ve en Rust

Rust ofrece adaptadores como `map`, `filter`, `filter_map`, `flat_map`, `scan`, `take`, `skip`, `fold` y `collect`:

```rust
let active_names: Vec<String> = users
    .iter()
    .filter(|user| user.active)
    .map(|user| user.name.trim().to_string())
    .collect();
```

La mayoría de adaptadores son perezosos: no hacen trabajo hasta que un consumidor como `collect`, `fold` o `sum` pide los valores.

## Cuándo usarlo

- Cuando una transformación se puede expresar como pasos pequeños.
- Cuando quieres evitar colecciones temporales.
- Cuando el orden del procesamiento se entiende mejor como pipeline.
- Cuando `fold` o `try_fold` deja clara una agregación.

## Cuándo evitarlo

- Si la cadena se vuelve demasiado larga o difícil de depurar.
- Si un ciclo explícito comunica mejor reglas de negocio complejas.
- Si necesitas muchos efectos secundarios entre pasos.
- Si hay que nombrar estados intermedios para explicar mejor el dominio.

## Diferencia con Iterator clásico

Iterator clásico define cómo recorrer una estructura. Iterator Adapters componen operaciones sobre cualquier iterador existente para producir nuevos iteradores o resultados finales.

## Ejemplos

- [x] Pipeline de filtrado y transformación de órdenes.
- [ ] Procesamiento por lotes de registros.
- [ ] Agregaciones de reportes sin estructuras intermedias.

### Pipeline de órdenes facturables

El módulo `order_pipeline` filtra órdenes pagadas con total positivo y las transforma en líneas de factura.

El ejemplo muestra cómo `iter`, `filter`, `map` y `collect` expresan una transformación de negocio sin ciclos manuales.

## Comandos

```bash
cargo test iterator_adapters
```
