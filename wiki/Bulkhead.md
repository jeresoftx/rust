# Bulkhead

Bulkhead divide recursos en compartimentos aislados. Si una integración o flujo se satura, el resto del sistema conserva capacidad propia para seguir atendiendo operaciones.

## Qué problema resuelve

- Evita que un proveedor externo consuma toda la capacidad.
- Reserva slots para operaciones críticas.
- Separa límites por recurso, tenant, prioridad o tipo de trabajo.
- Permite rechazar rápido cuando un compartimento está lleno.
- Reduce el radio de impacto de fallas parciales.

## Ejemplos del repositorio

- [x] Pools separados para proveedores externos.
- [x] Límite de concurrencia simulado por recurso.
- [x] Aislamiento de fallas entre operaciones críticas y no críticas.

## Código

- Documentación local: `patterns/distributed_systems/bulkhead/README.md`
- Módulo Rust: `src/patterns/distributed_systems/bulkhead.rs`
- Ejemplo de pools por proveedor: `src/patterns/distributed_systems/bulkhead/provider_pools.rs`
- Ejemplo de límite por recurso: `src/patterns/distributed_systems/bulkhead/resource_concurrency_limit.rs`
- Ejemplo de aislamiento crítico/no crítico: `src/patterns/distributed_systems/bulkhead/critical_isolation.rs`
