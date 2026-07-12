# Idempotency Key

## Intención

Idempotency Key permite repetir una solicitud sin duplicar su efecto. El cliente envía una llave estable para una operación; si la misma operación llega de nuevo, el servidor devuelve el resultado ya registrado en vez de ejecutar otra vez el cambio.

## Problema cotidiano

En sistemas distribuidos una petición puede repetirse por timeouts, retries, doble clic o pérdida de respuesta:

- Un pago se envía dos veces porque el cliente no recibió respuesta.
- Un job reintenta después de una desconexión.
- Una app móvil reenvía una orden al recuperar red.
- Un balanceador reintenta una llamada que sí alcanzó al servicio.

Sin idempotencia, esas repeticiones pueden duplicar cobros, órdenes, emails o movimientos de inventario.

## Partes

- **Llave de idempotencia:** identificador único generado por el cliente para una operación lógica.
- **Huella de payload:** representación estable de los datos de la solicitud.
- **Resultado almacenado:** respuesta que se devuelve si la misma llave llega de nuevo.
- **Detector de conflicto:** rechaza usar la misma llave con un payload diferente.
- **Operación protegida:** acción que solo debe ejecutarse una vez por llave válida.

## Cuándo usarlo

Úsalo en pagos, creación de órdenes, reservas, envíos de mensajes, APIs con retries y operaciones que modifican estado.

## Cuándo evitarlo

Evítalo para lecturas simples o cuando no puedes almacenar de forma confiable el resultado asociado a la llave. También evita aceptar la misma llave con payloads distintos.

## Ejemplos

- [x] Pagos protegidos contra doble envío.
- [x] Caché de respuestas por llave de idempotencia.
- [ ] Conflicto cuando la misma llave trae payload distinto.

### Pagos protegidos contra doble envío

El primer ejemplo registra pagos por llave. Si llega el mismo pago dos veces con la misma llave, devuelve el cargo original y no vuelve a ejecutar el cobro.

El módulo `payment_deduplication` guarda la huella del pago y el resultado del cargo por llave. Las pruebas validan que los reintentos equivalentes regresan el mismo cargo y que llaves distintas generan cargos distintos.

### Caché de respuestas por llave de idempotencia

El segundo ejemplo almacena una respuesta de API y la reutiliza en reintentos equivalentes.

El módulo `cached_responses` simula la creación de órdenes. La primera solicitud ejecuta la operación y guarda la respuesta; los reintentos con la misma llave y payload reciben exactamente la misma respuesta.

### Conflicto cuando la misma llave trae payload distinto

El tercer ejemplo detecta el caso peligroso: una misma llave usada con datos diferentes. En vez de devolver una respuesta incorrecta, responde con conflicto.

## Cómo ejecutar

```bash
cargo test idempotency_key
```
