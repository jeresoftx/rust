# Retry with Backoff

## Intención

Retry with Backoff reintenta una operación que puede fallar de forma temporal, aumentando la espera entre intentos. El objetivo es dar tiempo a que una dependencia se recupere sin saturarla con reintentos inmediatos.

## Problema cotidiano

En sistemas distribuidos es normal que una llamada falle por causas transitorias:

- Un servicio HTTP responde `503`.
- Una cola no acepta un job por saturación temporal.
- Una base de datos corta una conexión momentáneamente.
- Muchas instancias reintentan al mismo tiempo y empeoran la carga.

Un retry simple puede ayudar, pero sin backoff puede convertir una falla pequeña en una avalancha.

## Partes

- **Operación:** función o cliente que puede fallar.
- **Política de reintentos:** define intentos máximos y qué errores se reintentan.
- **Backoff:** secuencia de esperas entre intentos.
- **Jitter:** variación opcional para que muchas instancias no reintenten al mismo tiempo.
- **Resultado final:** éxito, error permanente o agotamiento de intentos.

## Cuándo usarlo

Úsalo en llamadas remotas, trabajos en cola o integraciones donde el error puede ser temporal y repetir la operación es seguro.

## Cuándo evitarlo

Evítalo cuando la operación no es idempotente, cuando el error es permanente, o cuando el reintento puede duplicar pagos, envíos o cambios de estado sin protección adicional.

## Ejemplos

- [ ] Cliente HTTP simulado con backoff exponencial.
- [ ] Reintentos solo para errores transitorios.
- [ ] Jitter determinista para evitar reintentos sincronizados.

## Cómo ejecutar

```bash
cargo test retry_with_backoff
```
