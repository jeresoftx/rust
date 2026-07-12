# Leader Election simulado

## Intención

Leader Election elige un nodo coordinador entre varios candidatos. En sistemas reales se apoya en consenso, leases o almacenamiento compartido; aquí lo simulamos de forma determinista para estudiar las reglas.

## Problema cotidiano

Algunas tareas deben ejecutarse por una sola instancia: agendar jobs, coordinar migraciones, compactar datos o publicar lotes. Si dos nodos creen ser líderes al mismo tiempo, pueden duplicar trabajo o corromper estado.

## Partes

- **Nodo:** instancia candidata.
- **Prioridad:** regla determinista para elegir líder.
- **Heartbeat:** señal de vida del líder.
- **Ronda:** periodo de elección.
- **Lease:** permiso temporal que evita dos líderes activos.

## Cuándo usarlo

Úsalo cuando varias instancias compiten por coordinar un trabajo que debe tener un solo dueño activo.

## Cuándo evitarlo

Evítalo si puedes delegar coordinación a infraestructura ya probada, como una cola, scheduler externo o base de datos con locks confiables.

## Ejemplos

- [x] Elegir líder por prioridad.
- [x] Failover cuando el líder deja de responder.
- [ ] Evitar dos líderes activos en la misma ronda.

### Elegir líder por prioridad

El primer ejemplo selecciona el nodo vivo con mayor prioridad.

El módulo `priority_election` elige el nodo vivo con mayor prioridad y no elige nada si todos están caídos.

### Failover cuando el líder deja de responder

El segundo ejemplo promueve otro nodo si el líder actual deja de enviar heartbeat.

El módulo `failover` marca nodos sin heartbeat como no responsivos y promueve al siguiente candidato disponible por prioridad.

### Evitar dos líderes activos en la misma ronda

El tercer ejemplo usa una ronda y lease para rechazar una segunda promoción simultánea.

## Cómo ejecutar

```bash
cargo test leader_election
```
