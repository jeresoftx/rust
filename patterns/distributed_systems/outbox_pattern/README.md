# Outbox Pattern

## Intención

Outbox Pattern evita perder eventos cuando una operación guarda datos y además necesita publicar un mensaje. La idea es persistir el cambio de dominio y el mensaje pendiente en la misma unidad de trabajo; otro proceso publica después ese mensaje.

## Problema cotidiano

Un servicio puede crear una orden y luego publicar `OrderCreated`. Si la base de datos confirma pero el broker falla, la orden existe pero el evento se pierde. Si se publica primero y luego falla la base, otros servicios reaccionan a algo que no existe.

Outbox reduce esa ventana: primero se guarda todo junto, después se publica de forma confiable desde la bandeja de salida.

## Partes

- **Entidad de dominio:** dato principal que cambia.
- **Mensaje outbox:** evento pendiente de publicar.
- **Unidad de trabajo:** guarda entidad y mensaje juntos.
- **Publicador:** lee pendientes, publica y marca enviados.
- **Reintento:** vuelve a procesar mensajes que siguen pendientes.

## Cuándo usarlo

Úsalo cuando un cambio de base de datos debe generar eventos, notificaciones, jobs o integraciones externas.

## Cuándo evitarlo

Evítalo si la operación no publica efectos externos, o si ya usas una infraestructura transaccional que cubre de forma confiable datos y mensajes.

## Ejemplos

- [x] Guardar entidad y evento en la misma unidad de trabajo.
- [x] Publicador que marca mensajes como enviados.
- [ ] Reintento de mensajes pendientes.

### Guardar entidad y evento en la misma unidad de trabajo

El primer ejemplo guarda una orden y su evento outbox de forma conjunta para que no queden separados.

El módulo `unit_of_work` simula una transacción en memoria: una orden y su mensaje outbox se registran juntos, y un fallo aborta ambos cambios.

### Publicador que marca mensajes como enviados

El segundo ejemplo toma mensajes pendientes, los publica y actualiza su estado.

El módulo `publisher_marks_sent` publica solo mensajes pendientes y marca cada uno como enviado para evitar publicaciones duplicadas en ejecuciones posteriores.

### Reintento de mensajes pendientes

El tercer ejemplo muestra que un fallo de publicación deja el mensaje pendiente para un siguiente intento.

## Cómo ejecutar

```bash
cargo test outbox_pattern
```
