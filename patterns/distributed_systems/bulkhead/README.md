# Bulkhead

## Intención

Bulkhead divide la capacidad de un sistema en compartimentos aislados. Si una dependencia, tipo de trabajo o flujo de negocio se satura, no debería consumir los recursos que necesita el resto del sistema.

## Problema cotidiano

En servicios con muchas integraciones es común compartir el mismo pool de workers, conexiones o slots entre operaciones diferentes. Cuando una parte se degrada, puede arrastrar a las demás:

- Un proveedor externo lento ocupa todos los workers.
- Una operación no crítica consume capacidad que necesitan pagos o login.
- Un lote pesado bloquea requests interactivos.
- Una cola secundaria impide procesar eventos importantes.

Bulkhead limita el daño reservando capacidad por compartimento.

## Partes

- **Compartimento:** grupo de capacidad separado por proveedor, flujo o prioridad.
- **Límite:** número máximo de operaciones activas por compartimento.
- **Admisión:** decisión de aceptar o rechazar una operación según capacidad disponible.
- **Liberación:** devolución del slot al terminar una operación.
- **Aislamiento:** garantía de que saturar un compartimento no bloquea otro.

## Cuándo usarlo

Úsalo cuando un servicio atiende operaciones con prioridades distintas o dependencias externas independientes. Es especialmente útil en clientes HTTP, workers, colas, jobs batch y límites por tenant.

## Cuándo evitarlo

Evítalo cuando el sistema es pequeño y un único límite global basta, o cuando separar capacidad reduciría demasiado el aprovechamiento de recursos sin mejorar la resiliencia.

## Ejemplos

- [x] Pools separados para proveedores externos.
- [x] Límite de concurrencia simulado por recurso.
- [x] Aislamiento de fallas entre operaciones críticas y no críticas.

### Pools separados para proveedores externos

El primer ejemplo asigna capacidad independiente a dos proveedores. Si un proveedor agota su pool, el otro puede seguir aceptando trabajo.

El módulo `provider_pools` separa capacidad para proveedores de pagos y envíos. Las pruebas validan que saturar un proveedor no bloquea al otro, que liberar un slot afecta solo a su proveedor y que el sistema reporta capacidad restante por compartimento.

### Límite de concurrencia simulado por recurso

El segundo ejemplo modela adquisición y liberación de slots sin threads reales. Así se puede probar el límite de concurrencia de forma determinista.

El módulo `resource_concurrency_limit` representa recursos como base de datos e índice de búsqueda. Cada recurso tiene su propio límite, su propio contador de actividad y su propio contador de rechazos.

### Aislamiento de fallas entre operaciones críticas y no críticas

El tercer ejemplo separa operaciones críticas de operaciones no críticas para mostrar que la saturación de tareas secundarias no debe bloquear el flujo principal.

El módulo `critical_isolation` separa capacidad y métricas de fallas por clase de operación. Así el trabajo de reportes, sincronizaciones o tareas secundarias puede fallar sin consumir la capacidad reservada para pagos, login u otros flujos principales.

## Cómo ejecutar

```bash
cargo test bulkhead
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
