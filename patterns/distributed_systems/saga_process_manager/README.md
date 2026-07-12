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

- [ ] Reserva, pago y envío coordinados por pasos.
- [ ] Compensación cuando falla un paso intermedio.
- [ ] Estado persistente del proceso.

### Reserva, pago y envío coordinados por pasos

El primer ejemplo ejecuta un flujo feliz: reserva inventario, captura pago y programa envío.

### Compensación cuando falla un paso intermedio

El segundo ejemplo falla durante pago y libera la reserva previa.

### Estado persistente del proceso

El tercer ejemplo guarda el estado de la saga para continuar después de una pausa o reinicio.

## Cómo ejecutar

```bash
cargo test saga_process_manager
```
