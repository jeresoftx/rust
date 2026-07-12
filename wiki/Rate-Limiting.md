# Rate Limiting

Rate Limiting controla cuántas solicitudes u operaciones puede realizar un consumidor dentro de un periodo. El patrón protege capacidad compartida y permite responder rápido cuando alguien excede su cuota.

## Qué problema resuelve

- Protege endpoints costosos frente a ráfagas.
- Separa cuota por usuario, API key o tenant.
- Evita que un consumidor degrade el servicio para otros.
- Permite devolver respuestas claras cuando se excede el límite.
- Ayuda a operar APIs con presupuesto de uso explícito.

## Ejemplos del repositorio

- [ ] Token bucket determinista.
- [ ] Límite por usuario o API key.
- [ ] Respuesta con tiempo sugerido para reintento.

## Código

- Documentación local: `patterns/distributed_systems/rate_limiting/README.md`
- Módulo Rust: `src/patterns/distributed_systems/rate_limiting.rs`
