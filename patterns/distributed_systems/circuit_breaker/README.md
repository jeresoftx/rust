# Circuit Breaker

## Intención

Circuit Breaker protege a un sistema cuando una dependencia remota empieza a fallar. En vez de seguir llamándola indefinidamente, el circuito se abre, rechaza llamadas por un tiempo y luego permite una prueba controlada de recuperación.

## Problema cotidiano

En un servicio real, una dependencia puede degradarse por saturación, errores internos o latencia excesiva. Si todos los clientes siguen intentando cada request, el problema crece:

- Se acumulan conexiones y threads ocupados.
- Aumenta la latencia de operaciones sanas.
- Los reintentos presionan más a la dependencia dañada.
- El usuario recibe respuestas lentas en vez de fallas rápidas y controladas.

Circuit Breaker cambia la respuesta: cuando el sistema detecta demasiadas fallas, deja de llamar temporalmente a la dependencia y falla rápido.

## Partes

- **Closed:** el circuito deja pasar llamadas normales.
- **Open:** el circuito rechaza llamadas sin tocar la dependencia.
- **Half-open:** el circuito permite una llamada de prueba para validar si la dependencia se recuperó.
- **Umbral de fallas:** número de fallas consecutivas necesarias para abrir el circuito.
- **Métricas:** conteo de éxitos, fallas y rechazos por circuito abierto.

## Cuándo usarlo

Úsalo en llamadas HTTP, proveedores externos, bases de datos secundarias, colas o servicios donde una dependencia fallando puede afectar al resto del sistema.

## Cuándo evitarlo

Evítalo cuando la operación es local y barata, cuando no existe una estrategia clara para responder al rechazo, o cuando el estado compartido del circuito introduciría más complejidad que beneficio.

## Ejemplos

- [x] Abrir circuito después de fallas consecutivas.
- [x] Estado half-open para probar recuperación.
- [x] Métricas de rechazos por circuito abierto.

### Abrir circuito después de fallas consecutivas

El primer ejemplo muestra un circuito cerrado que cuenta fallas consecutivas. Al alcanzar el umbral, pasa a estado abierto y deja de ejecutar la operación protegida.

El módulo `consecutive_failures` usa una dependencia simulada con respuestas predecibles. La prueba confirma que el circuito se abre al alcanzar el umbral, que un éxito reinicia el contador y que un circuito abierto rechaza sin invocar la dependencia.

### Estado half-open para probar recuperación

El segundo ejemplo agrega una transición controlada: después del periodo de enfriamiento, el circuito permite una llamada de prueba. Si funciona, cierra; si falla, vuelve a abrir.

El módulo `half_open_recovery` usa un reloj lógico en ticks. Así se puede probar el periodo de enfriamiento sin `sleep`, validando que el circuito rechaza antes del tiempo permitido, entra a half-open al cumplirse el enfriamiento y decide cerrar o reabrir según la llamada de prueba.

### Métricas de rechazos por circuito abierto

El tercer ejemplo registra cuántas llamadas fueron rechazadas sin tocar la dependencia. Esto ayuda a observabilidad, alertas y decisiones operativas.

El módulo `open_rejection_metrics` separa métricas de llamadas reales, éxitos, fallas y rechazos. Así un servicio puede alertar por circuitos abiertos sin confundir rechazos locales con errores de la dependencia.

## Cómo ejecutar

```bash
cargo test circuit_breaker
```
