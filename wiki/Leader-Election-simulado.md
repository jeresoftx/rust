# Leader Election simulado

Leader Election simulado elige un nodo coordinador entre varios candidatos usando reglas deterministas. Sirve para estudiar prioridad, failover y protección contra dos líderes activos.

## Qué problema resuelve

- Coordina trabajos que solo debe ejecutar una instancia.
- Permite failover si el líder cae.
- Reduce duplicación de tareas programadas.
- Usa rondas o leases para evitar dos líderes activos.
- Hace explícita la regla de elección.

## Ejemplos del repositorio

- [x] Elegir líder por prioridad.
- [ ] Failover cuando el líder deja de responder.
- [ ] Evitar dos líderes activos en la misma ronda.

## Código

- Documentación local: `patterns/distributed_systems/leader_election/README.md`
- Módulo Rust: `src/patterns/distributed_systems/leader_election.rs`
- Ejemplo de prioridad: `src/patterns/distributed_systems/leader_election/priority_election.rs`
