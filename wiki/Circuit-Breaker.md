# Circuit Breaker

Circuit Breaker protege un servicio cuando una dependencia empieza a fallar. El patrón abre un circuito después de cierto número de fallas, rechaza llamadas temporalmente y luego permite una prueba controlada para decidir si puede volver a cerrar.

## Qué problema resuelve

- Evita llamadas repetidas a dependencias degradadas.
- Falla rápido cuando una integración está dañada.
- Reduce presión sobre servicios saturados.
- Permite modelar recuperación con estado half-open.
- Expone métricas de rechazos y fallas para observabilidad.

## Ejemplos del repositorio

- [x] Abrir circuito después de fallas consecutivas.
- [x] Estado half-open para probar recuperación.
- [x] Métricas de rechazos por circuito abierto.

## Código

- Documentación local: `patterns/distributed_systems/circuit_breaker/README.md`
- Módulo Rust: `src/patterns/distributed_systems/circuit_breaker.rs`
- Ejemplo de fallas consecutivas: `src/patterns/distributed_systems/circuit_breaker/consecutive_failures.rs`
- Ejemplo de recuperación half-open: `src/patterns/distributed_systems/circuit_breaker/half_open_recovery.rs`
- Ejemplo de métricas de rechazos: `src/patterns/distributed_systems/circuit_breaker/open_rejection_metrics.rs`
