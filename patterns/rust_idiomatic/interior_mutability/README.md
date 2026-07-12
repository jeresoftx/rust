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

- [ ] Caché en memoria con `RefCell`.
- [ ] Contador compartido con `Mutex`.
- [ ] Lectura concurrente con `RwLock`.

## Comandos

```bash
cargo test interior_mutability
```
