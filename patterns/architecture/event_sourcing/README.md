# Event Sourcing

## Intención

Event Sourcing guarda los cambios importantes como una secuencia de eventos, no solo como el estado actual. El estado se reconstruye aplicando esos eventos en orden, lo que permite auditoría, reproducción de casos y modelos derivados.

## Problema cotidiano

En sistemas reales, guardar únicamente el estado final puede ocultar información valiosa:

- Una cuenta bancaria muestra saldo actual, pero no cómo llegó ahí.
- Una orden cambia de estado varias veces sin rastro de quién la modificó.
- Un bug en una proyección obliga a recalcular vistas sin tener los hechos originales.
- Una auditoría necesita explicar la historia completa, no solo el último valor.

Event Sourcing conserva los hechos del dominio para reconstruir y explicar el sistema.

## Partes principales

- Eventos: hechos inmutables que ya ocurrieron.
- Stream de eventos: secuencia ordenada por agregado o entidad.
- Apply: función que actualiza estado a partir de un evento.
- Rehidratación: reconstrucción de estado desde eventos.
- Snapshots: puntos de partida guardados para evitar reproducir streams largos completos.

## Cómo se ve en Rust

Rust encaja bien con eventos modelados como enums:

```rust
enum AccountEvent {
    MoneyDeposited { cents: i32 },
    MoneyWithdrawn { cents: i32 },
}
```

El agregado puede reconstruirse con `fold` o aplicando cada evento a un estado inicial.

## Cuándo usarlo

- Cuando la historia de cambios es tan importante como el estado actual.
- Cuando necesitas auditoría fuerte o trazabilidad.
- Cuando varios modelos de lectura se derivan de los mismos eventos.
- Cuando puedes tolerar reconstrucción o proyecciones eventuales.

## Cuándo evitarlo

- Si solo necesitas CRUD simple.
- Si la auditoría básica de cambios es suficiente.
- Si el equipo no está listo para versionar eventos.
- Si la consistencia eventual complica más de lo que ayuda.

## Ejemplos

- [x] Cuenta bancaria reconstruida desde eventos.
- [ ] Auditoría de cambios de orden.
- [ ] Snapshots para acelerar reconstrucción.

### Cuenta bancaria reconstruida desde eventos

El módulo `bank_account` reconstruye una cuenta aplicando eventos `Opened`, `Deposited` y `Withdrawn`. El saldo actual no se guarda como fuente primaria: se deriva del stream.

Cuando se ejecuta un comando nuevo, como retirar dinero, el agregado valida la regla y registra un evento pendiente. Si la operación falla, no se agrega ningún evento.

## Comandos

```bash
cargo test event_sourcing
```
