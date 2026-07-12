# Idempotency Key

Idempotency Key evita duplicar efectos cuando una petición se repite. El cliente envía una llave estable para una operación lógica; si la misma solicitud llega de nuevo, el servidor devuelve el resultado guardado.

## Qué problema resuelve

- Protege pagos contra doble cobro.
- Evita crear órdenes duplicadas por retries.
- Permite reintentar después de timeouts sin repetir efectos.
- Cachea respuestas para solicitudes equivalentes.
- Rechaza conflictos cuando una llave se reutiliza con payload distinto.

## Ejemplos del repositorio

- [x] Pagos protegidos contra doble envío.
- [x] Caché de respuestas por llave de idempotencia.
- [ ] Conflicto cuando la misma llave trae payload distinto.

## Código

- Documentación local: `patterns/distributed_systems/idempotency_key/README.md`
- Módulo Rust: `src/patterns/distributed_systems/idempotency_key.rs`
- Ejemplo de pagos idempotentes: `src/patterns/distributed_systems/idempotency_key/payment_deduplication.rs`
- Ejemplo de caché de respuestas: `src/patterns/distributed_systems/idempotency_key/cached_responses.rs`
