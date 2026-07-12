# Rate Limiting

Rate Limiting controla cuántas solicitudes u operaciones puede realizar un consumidor dentro de un periodo. El patrón protege capacidad compartida y permite responder rápido cuando alguien excede su cuota.

## Qué problema resuelve

- Protege endpoints costosos frente a ráfagas.
- Separa cuota por usuario, API key o tenant.
- Evita que un consumidor degrade el servicio para otros.
- Permite devolver respuestas claras cuando se excede el límite.
- Ayuda a operar APIs con presupuesto de uso explícito.

## Ejemplos del repositorio

- [x] Token bucket determinista.
- [x] Límite por usuario o API key.
- [x] Respuesta con tiempo sugerido para reintento.

## Código

- Documentación local: `patterns/distributed_systems/rate_limiting/README.md`
- Módulo Rust: `src/patterns/distributed_systems/rate_limiting.rs`
- Ejemplo de token bucket: `src/patterns/distributed_systems/rate_limiting/token_bucket.rs`
- Ejemplo de límite por API key: `src/patterns/distributed_systems/rate_limiting/api_key_limit.rs`
- Ejemplo de respuesta retry-after: `src/patterns/distributed_systems/rate_limiting/retry_after_response.rs`
