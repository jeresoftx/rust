# RAII

## Intención

RAII significa "Resource Acquisition Is Initialization". La idea es que adquirir un recurso y poseerlo queden ligados a un valor. Cuando ese valor sale de scope, Rust ejecuta `Drop` y libera o finaliza el recurso automáticamente.

## Problema cotidiano

En sistemas reales usamos recursos que deben cerrarse, liberar locks o revertir cambios si algo falla:

- Un lock debe liberarse incluso si una función termina temprano.
- Una transacción debe hacer rollback si no se confirma.
- Un archivo temporal debe limpiarse al salir del scope.

Si estos pasos dependen de recordar llamar métodos manuales al final, es fácil dejar recursos abiertos o estados inconsistentes.

## Cómo se ve en Rust

En Rust, RAII aparece de forma natural por ownership y `Drop`:

```rust
pub struct Guard<'a> {
    locked: &'a mut bool,
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        *self.locked = false;
    }
}
```

El valor dueño del recurso actúa como guard. Mientras existe, representa un estado activo; cuando se destruye, ejecuta la limpieza.

## Cuándo usarlo

- Cuando un recurso debe cerrarse o liberarse siempre.
- Cuando hay que proteger secciones críticas con guards.
- Cuando una operación debe revertirse si no se confirma explícitamente.
- Cuando quieres que la limpieza sea automática, incluso con retornos tempranos.

## Cuándo evitarlo

- Si la limpieza puede fallar y necesitas reportar ese error de forma explícita.
- Si el comportamiento oculto en `Drop` sorprende más de lo que ayuda.
- Si el recurso no tiene una relación clara de ownership.

## Diferencia con `defer`

En algunos lenguajes se usa `defer` para programar limpieza al final de una función. En Rust, RAII ata la limpieza al tiempo de vida de un valor, por lo que funciona con scopes anidados, retornos tempranos y composición de tipos.

## Ejemplos

- [x] Lock guard para secciones críticas.
- [x] Transacción que hace rollback si no se confirma.
- [x] Archivo temporal que se limpia al salir de scope.

### Lock guard para secciones críticas

El módulo `lock_guard` modela una sección crítica que se bloquea al crear un guard y se libera automáticamente cuando el guard sale de scope.

El ejemplo muestra cómo RAII evita olvidar la liberación del lock, incluso con retornos tempranos.

### Transacción con rollback automático

El módulo `transaction_rollback` modela un ledger con transacciones que acumulan cambios pendientes.

El ejemplo muestra cómo `commit` aplica los cambios y cómo `Drop` registra rollback si la transacción sale de scope sin confirmarse.

### Archivo temporal con limpieza automática

El módulo `temporary_file_cleanup` modela un archivo temporal que existe mientras vive el valor `TemporaryFile`.

El ejemplo muestra cómo el archivo se elimina al salir de scope y cómo la limpieza se mantiene segura incluso si el archivo ya fue removido manualmente.

## Comandos

```bash
cargo test raii
```
