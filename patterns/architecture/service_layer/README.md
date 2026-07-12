# Service Layer

## Intención

Service Layer concentra los casos de uso de aplicación en servicios explícitos. El servicio recibe comandos o consultas, valida reglas de aplicación, coordina repositorios, políticas y gateways, y devuelve una respuesta pensada para la capa que lo invoca.

La idea central es que un controlador HTTP, un job o una interfaz de consola no decidan cómo se registra un usuario, cómo se confirma un checkout o cómo se arma un reporte. Esas capas solo traducen entrada y salida; la coordinación vive en el servicio.

## Problema cotidiano

En sistemas reales es común encontrar:

- Controladores que validan, persisten, envían correos y deciden reglas.
- Jobs que duplican la misma lógica que un endpoint.
- Repositorios usados directamente desde muchas capas sin una intención de negocio clara.
- Flujos como checkout, registro o generación de reportes repartidos en varios lugares.

Service Layer pone un punto de entrada estable para esos casos de uso.

## Partes

- **Comando o consulta:** DTO de entrada con los datos necesarios para ejecutar el caso de uso.
- **Servicio de aplicación:** coordina el flujo y expresa la intención del caso de uso.
- **Repositorios y gateways:** dependencias que leen, persisten o se comunican con otros sistemas.
- **Políticas:** reglas intercambiables que ayudan a decidir precios, descuentos, permisos o validaciones.
- **Respuesta:** DTO de salida con lo que necesita el consumidor.

## Cuándo usarlo

Úsalo cuando un caso de uso coordina varias dependencias, necesita transacciones, aplica reglas de aplicación o se reutiliza desde más de una interfaz.

Evítalo si solo estás envolviendo un CRUD trivial sin agregar intención. Un servicio que únicamente llama `repository.save` sin reglas ni coordinación suele añadir ruido.

## Ejemplos

- [ ] Servicio de aplicación para registrar usuario.
- [ ] Servicio de checkout que coordina repositorios y políticas.
- [ ] Servicio de reportes para consultas complejas.

## Cómo ejecutar

```bash
cargo test service_layer
```
