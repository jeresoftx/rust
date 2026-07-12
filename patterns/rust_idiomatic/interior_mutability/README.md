# Interior Mutability

## Intención

Interior Mutability permite cambiar estado interno a través de una referencia compartida. En Rust esto se usa cuando la API pública debe verse inmutable, pero una parte interna necesita actualizar cachés, métricas o coordinación concurrente.

## Problema cotidiano

En sistemas reales es común necesitar mutación controlada sin exponer `&mut self` en cada llamada:

- Un servicio lee datos y guarda resultados calculados en una caché.
- Un contador de métricas se comparte entre hilos.
- Una configuración se lee muchas veces y se actualiza ocasionalmente.

Sin este patrón, la API puede volverse incómoda o terminar propagando mutabilidad más allá de la frontera donde realmente se necesita.

## Cómo se ve en Rust

Rust ofrece tipos que mueven la validación de préstamos o sincronización al lugar correcto:

```rust
use std::cell::RefCell;

struct Cache {
    values: RefCell<Vec<String>>,
}

impl Cache {
    fn remember(&self, value: String) {
        self.values.borrow_mut().push(value);
    }
}
```

Para concurrencia se usan primitivas seguras como `Mutex` y `RwLock`.

## Cuándo usarlo

- Cuando necesitas actualizar estado interno sin cambiar la firma pública a `&mut self`.
- Cuando el estado mutable está encapsulado y tiene una razón clara.
- Cuando varios hilos necesitan compartir y modificar datos con sincronización explícita.
- Cuando una caché, métrica o registro interno no debe dominar el diseño externo.

## Cuándo evitarlo

- Si puedes pasar `&mut self` de forma clara y simple.
- Si el estado mutable empieza a esconder reglas de negocio importantes.
- Si estás usando `RefCell` para esquivar el borrow checker sin entender el modelo de ownership.
- Si una sección crítica con `Mutex` o `RwLock` crece demasiado.

## Ejemplos

- [x] Caché en memoria con `RefCell`.
- [x] Contador compartido con `Mutex`.
- [ ] Lectura concurrente con `RwLock`.

### Caché en memoria con `RefCell`

El módulo `refcell_cache` modela un catálogo de productos que expone `find_product(&self)`, pero internamente actualiza una caché.

`RefCell` permite que la mutación se mantenga encapsulada en tiempo de ejecución. Desde fuera, el servicio se usa como una dependencia de solo lectura; por dentro, evita lecturas repetidas al origen de datos.

### Contador compartido con `Mutex`

El módulo `mutex_counter` muestra un contador de métricas que puede compartirse entre hilos con `Arc`.

`Mutex` protege el `HashMap` interno para que varias tareas puedan incrementar métricas sin condiciones de carrera. La API sigue usando `&self`, pero cada operación toma el candado solo durante la sección crítica necesaria.

## Comandos

```bash
cargo test interior_mutability
```
