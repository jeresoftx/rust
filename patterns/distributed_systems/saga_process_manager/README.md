# Saga / Process Manager

## Intención

Saga / Process Manager coordina un proceso de negocio que cruza varios servicios o pasos locales. En vez de depender de una transacción distribuida, guarda el estado del proceso y ejecuta pasos o compensaciones.

## Problema cotidiano

Un checkout real puede reservar inventario, cobrar pago y preparar envío. Si el pago falla después de reservar inventario, hay que liberar la reserva. Si el envío falla, quizá hay que devolver pago o marcar revisión manual.

Saga hace explícito el avance del proceso y sus acciones de compensación.

## Partes

- **Proceso:** flujo de negocio completo.
- **Paso:** acción concreta como reservar, cobrar o enviar.
- **Estado:** etapa persistida del proceso.
- **Compensación:** acción que deshace o mitiga un paso ya ejecutado.
- **Manager:** decide el siguiente paso según eventos o resultados.

## Cuándo usarlo

Úsalo en checkouts, reservas, onboarding, aprovisionamiento y procesos que cruzan servicios con fallas parciales.

## Cuándo evitarlo

Evítalo si todo ocurre en una transacción local simple, o si no existe una compensación razonable para los pasos críticos.

## Ejemplos

- [x] Reserva, pago y envío coordinados por pasos.
- [x] Compensación cuando falla un paso intermedio.
- [ ] Estado persistente del proceso.

### Reserva, pago y envío coordinados por pasos

El primer ejemplo ejecuta un flujo feliz: reserva inventario, captura pago y programa envío.

El módulo `checkout_flow` coordina los pasos de un checkout y evita repetirlos si la saga ya quedó completada.

### Compensación cuando falla un paso intermedio

El segundo ejemplo falla durante pago y libera la reserva previa.

El módulo `compensation` registra la reserva de inventario, detecta falla de pago y ejecuta la compensación para liberar la reserva.

### Estado persistente del proceso

El tercer ejemplo guarda el estado de la saga para continuar después de una pausa o reinicio.

## Cómo ejecutar

```bash
cargo test saga_process_manager
```
