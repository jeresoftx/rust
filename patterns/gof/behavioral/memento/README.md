# Memento

## Intención

Memento captura el estado interno de un objeto para poder restaurarlo después sin exponer sus detalles.

## Problema cotidiano

En sistemas reales muchas operaciones necesitan deshacer o volver a un punto anterior:

- Configuraciones que se prueban y deben revertirse si fallan.
- Documentos que guardan historial de cambios.
- Carritos que recuperan su estado después de una sesión interrumpida.

Si cada consumidor conoce todos los campos internos para restaurar un objeto, el encapsulamiento se pierde. Memento separa el objeto que cambia, el snapshot de su estado y el componente que conserva esos snapshots.

## Cómo se ve en Rust

En Rust, Memento suele representarse con structs clonables que contienen una copia segura del estado. El objeto principal expone métodos como `save` y `restore`, mientras que el snapshot no necesita permitir mutaciones externas.

Para estados pequeños, `Clone` suele ser suficiente. Para estados grandes, conviene guardar solo la parte necesaria, usar eventos o snapshots incrementales.

## Cuándo usarlo

- Cuando necesitas rollback, undo o recuperación de estado.
- Cuando el estado interno no debería exponerse para que otros lo reconstruyan.
- Cuando quieres conservar puntos de restauración explícitos y testeables.

## Cuándo evitarlo

- Si el estado es muy grande y cada snapshot cuesta demasiado.
- Si un historial de eventos describe mejor lo que ocurrió.
- Si una transacción o rollback de base de datos ya resuelve el problema.

## Diferencia con Command

Command encapsula una acción y puede saber cómo deshacerla. Memento guarda un estado anterior para restaurarlo sin repetir la acción inversa.

## Ejemplos

- [x] Snapshots de configuración para rollback.
- [ ] Historial de cambios de documento.
- [ ] Restaurar estado de carrito.

### Snapshots de configuración

El módulo `config_snapshots` modela un editor de configuración que puede guardar y restaurar snapshots.

El ejemplo muestra cómo probar cambios riesgosos y volver a un estado anterior sin que el consumidor reconstruya todos los campos manualmente.

## Comandos

```bash
cargo test memento
```
