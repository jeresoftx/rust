# Rate Limiting

## Intención

Rate Limiting limita cuántas solicitudes u operaciones puede aceptar un sistema dentro de un periodo. El objetivo es proteger capacidad, repartir uso de forma justa y responder rápido cuando un cliente excede su cuota.

## Problema cotidiano

En APIs, workers y servicios internos, un cliente puede enviar demasiadas solicitudes en poco tiempo:

- Un usuario refresca una pantalla repetidamente.
- Una API key queda dentro de un job con un loop defectuoso.
- Un bot consume capacidad compartida.
- Un endpoint costoso recibe tráfico mayor al esperado.

Sin límite, el sistema puede degradarse para todos. Con Rate Limiting, cada consumidor tiene un presupuesto claro.

## Partes

- **Identidad:** usuario, API key, tenant o cliente que consume cuota.
- **Política:** capacidad máxima y ritmo de recarga o ventana.
- **Decisión:** aceptar o rechazar una operación.
- **Estado:** tokens disponibles, contador por ventana o historial mínimo.
- **Respuesta:** información útil como tokens restantes o tiempo sugerido para reintentar.

## Cuándo usarlo

Úsalo en APIs públicas, endpoints caros, integraciones con cuotas, workers multi-tenant y operaciones donde un consumidor puede afectar a otros.

## Cuándo evitarlo

Evítalo cuando el tráfico es interno y ya está controlado por otro mecanismo, o cuando rechazar solicitudes sin una experiencia clara empeoraría el flujo de usuario.

## Ejemplos

- [x] Token bucket determinista.
- [x] Límite por usuario o API key.
- [ ] Respuesta con tiempo sugerido para reintento.

### Token bucket determinista

El primer ejemplo modela un bucket con capacidad máxima y recarga por ticks lógicos. No usa tiempo real; las pruebas controlan exactamente cuándo aparecen nuevos tokens.

El módulo `token_bucket` muestra una política simple: cada request consume un token, el bucket se recarga por ticks lógicos y nunca supera su capacidad máxima. Cuando no hay tokens, devuelve rechazo con un tiempo de reintento determinista.

### Límite por usuario o API key

El segundo ejemplo mantiene estado separado por consumidor para que una API key saturada no consuma la cuota de otra.

El módulo `api_key_limit` guarda un bucket por API key. Las pruebas muestran que una llave agotada no afecta a otra, que la recarga se aplica al consumidor consultado y que se puede reportar cuota restante por llave.

### Respuesta con tiempo sugerido para reintento

El tercer ejemplo muestra cómo devolver una respuesta útil cuando se excede el límite, incluyendo cuántos ticks faltan para volver a intentar.

## Cómo ejecutar

```bash
cargo test rate_limiting
```
