# Observer

## Intención

Observer permite que varios suscriptores reaccionen automáticamente cuando un objeto publica un cambio o evento.

## Problema cotidiano

En sistemas reales una sola acción suele disparar muchas reacciones:

- Una orden creada debe alimentar auditoría, fulfillment y proyecciones.
- Un proceso debe reportar métricas y logs sin acoplarse a cada salida.
- Un cambio de inventario debe notificar canales interesados.

Si el objeto principal llama directamente a cada consumidor, queda acoplado a detalles que no le pertenecen. Observer separa al publicador de los suscriptores y permite agregar reacciones sin cambiar el origen del evento.

## Cómo se ve en Rust

En Rust, Observer puede implementarse con traits, closures, canales, listas de suscriptores o buses de eventos explícitos. Para ejemplos pequeños, una estructura que registra suscriptores y distribuye eventos suele ser suficiente y fácil de probar.

En sistemas concurrentes, `std::sync::mpsc`, `tokio::sync::broadcast` o un broker externo pueden cumplir el mismo rol a otra escala.

## Cuándo usarlo

- Cuando varios consumidores reaccionan al mismo evento.
- Cuando el publicador no debería conocer a cada consumidor concreto.
- Cuando quieres agregar o quitar reacciones sin tocar la lógica principal.

## Cuándo evitarlo

- Si solo hay un consumidor directo y estable.
- Si el flujo necesita una respuesta inmediata y ordenada de cada participante.
- Si los efectos secundarios ocultos dificultan razonar sobre el sistema.

## Diferencia con Mediator

Mediator coordina una conversación entre participantes. Observer distribuye eventos desde un publicador hacia suscriptores independientes.

## Ejemplos

- [x] Eventos de dominio para orden creada.
- [x] Suscriptores de métricas y logs.
- [x] Notificaciones al cambiar estado de inventario.

### Eventos de dominio para orden creada

El módulo `order_events` implementa un bus pequeño para publicar `OrderCreated`.

El ejemplo muestra cómo auditoría, fulfillment u otros consumidores pueden suscribirse al evento sin que la lógica de órdenes conozca a cada receptor.

### Suscriptores de métricas y logs

El módulo `metrics_logs` publica eventos de un proceso hacia observadores de métricas y logs.

El ejemplo muestra cómo cada suscriptor reacciona al mismo evento con una responsabilidad distinta.

### Notificaciones de inventario

El módulo `inventory_notifications` publica cambios de stock hacia observadores de reabastecimiento y ventas.

El ejemplo muestra cómo distintos equipos pueden reaccionar al mismo cambio sin acoplarse a la lógica que actualiza inventario.

## Comandos

```bash
cargo test observer
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
