# Command

## Intención

Command encapsula una acción como un objeto o valor que puede ejecutarse, guardarse, deshacerse o encolarse.

## Problema cotidiano

En sistemas reales muchas operaciones necesitan viajar como datos:

- Un CLI recibe acciones como crear, actualizar o borrar usuarios.
- Una orden necesita acciones reversibles para aplicar cambios y deshacerlos.
- Una cola de trabajos necesita guardar comandos que se ejecutarán después.

Si cada flujo invoca lógica directamente, es difícil registrar, reintentar, auditar o componer acciones. Command separa la intención de la ejecución.

## Cómo se ve en Rust

En Rust, Command puede modelarse con enums, traits, closures o structs pequeñas. Para comandos cerrados, un enum con `match` suele ser más claro. Para comandos extensibles, un trait como `Command` permite almacenar acciones heterogéneas detrás de `Box<dyn Command>`.

## Cuándo usarlo

- Cuando necesitas representar una acción como dato.
- Cuando quieres encolar, auditar, reintentar o deshacer operaciones.
- Cuando el invocador no debe conocer los detalles de cada acción.

## Cuándo evitarlo

- Si solo hay una llamada directa y no necesitas desacoplarla.
- Si el comando solo envuelve una función sin aportar semántica.
- Si una lista de funciones simples comunica mejor la intención.

## Diferencia con Strategy

Strategy cambia el algoritmo usado para resolver una tarea. Command representa una acción concreta que puede ejecutarse ahora, después o en sentido inverso.

## Ejemplos

- [x] Comandos de CLI para crear, actualizar y borrar usuarios.
- [x] Acciones reversibles para editar una orden.
- [ ] Cola de trabajos con comandos serializables.

### CLI de usuarios

El módulo `user_cli` usa un enum `UserCommand` para representar acciones de crear, actualizar y borrar usuarios.

El ejemplo muestra cómo un invocador ejecuta comandos y mantiene una bitácora sin exponer los detalles de cada acción.

### Acciones reversibles de orden

El módulo `reversible_order` usa comandos con `execute` y `undo` para aplicar descuentos y notas sobre una orden.

El ejemplo muestra cómo un historial puede deshacer la última acción sin que el cliente conozca los detalles internos de cada comando.

## Comandos

```bash
cargo test command
```
