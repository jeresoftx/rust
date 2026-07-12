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

- [x] Servicio de aplicación para registrar usuario.
- [x] Servicio de checkout que coordina repositorios y políticas.
- [x] Servicio de reportes para consultas complejas.

### Servicio de aplicación para registrar usuario

El módulo `user_registration` muestra un servicio que recibe un `RegistrationRequest`, valida reglas de aplicación, consulta un repositorio en memoria, persiste el usuario y coordina el envío de un correo de bienvenida mediante un gateway falso.

Este ejemplo deja los controladores fuera de la ecuación: cualquier interfaz puede invocar `RegistrationService::register` y reutilizar la misma orquestación.

### Servicio de checkout que coordina repositorios y políticas

El módulo `checkout_service` modela un checkout con inventario, repositorio de órdenes, gateway de pago y política de descuento. El servicio calcula el subtotal, aplica descuentos, cobra y solo después reserva inventario y guarda la orden.

La prueba de pago rechazado muestra una regla importante de aplicación: si el pago falla, no se debe modificar inventario ni persistir la orden.

### Servicio de reportes para consultas complejas

El módulo `reporting_service` muestra un caso de uso de lectura. El servicio recibe una consulta, combina clientes y órdenes desde un repositorio de reportes, filtra ventas pagadas o todas las órdenes y produce un `SalesReport`.

Este estilo es útil cuando el endpoint necesita un DTO específico para tablero, auditoría o análisis, pero no queremos que la capa HTTP conozca los detalles de agregación.

## Cómo ejecutar

```bash
cargo test service_layer
```
