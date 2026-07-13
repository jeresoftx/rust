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

- [x] Cliente HTTP simulado con backoff exponencial.
- [x] Reintentos solo para errores transitorios.
- [x] Jitter determinista para evitar reintentos sincronizados.

### Cliente HTTP simulado con backoff exponencial

El módulo `http_client` usa un cliente HTTP simulado para mostrar la mecánica central: intentar una operación, registrar una espera creciente y detenerse al obtener éxito o agotar intentos.

Las esperas se guardan como números en vez de dormir realmente; así el ejemplo es rápido, determinista y fácil de probar.

### Reintentos solo para errores transitorios

El módulo `transient_errors` separa errores transitorios de errores permanentes. El executor reintenta timeouts o saturación temporal, pero falla rápido cuando el problema es un payload inválido o una regla de negocio permanente.

Esta distinción evita gastar presupuesto de reintentos en errores que no se van a corregir esperando.

### Jitter determinista para evitar reintentos sincronizados

El módulo `deterministic_jitter` calcula offsets reproducibles a partir del identificador de un nodo o worker. Así varias instancias pueden usar la misma política base sin reintentar exactamente al mismo tiempo.

El ejemplo evita aleatoriedad real para que las pruebas sean estables.

## Cómo ejecutar

```bash
cargo test retry_with_backoff
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
