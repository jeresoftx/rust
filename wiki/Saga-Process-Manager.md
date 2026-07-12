# Saga / Process Manager

Saga / Process Manager coordina procesos largos con varios pasos y compensaciones. Guarda el estado del proceso para avanzar, reintentar o compensar sin depender de transacciones distribuidas.

## Qué problema resuelve

- Coordina reserva, pago y envío.
- Evita dejar pasos previos sin compensar.
- Hace visible el estado de procesos largos.
- Permite continuar después de pausas o reinicios.
- Modela fallas parciales entre servicios.

## Ejemplos del repositorio

- [x] Reserva, pago y envío coordinados por pasos.
- [x] Compensación cuando falla un paso intermedio.
- [x] Estado persistente del proceso.

## Código

- Documentación local: `patterns/distributed_systems/saga_process_manager/README.md`
- Módulo Rust: `src/patterns/distributed_systems/saga_process_manager.rs`
- Ejemplo de checkout coordinado: `src/patterns/distributed_systems/saga_process_manager/checkout_flow.rs`
- Ejemplo de compensación: `src/patterns/distributed_systems/saga_process_manager/compensation.rs`
- Ejemplo de estado persistente: `src/patterns/distributed_systems/saga_process_manager/persistent_state.rs`
