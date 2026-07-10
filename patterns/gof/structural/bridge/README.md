# Bridge

## Intención

Bridge separa una abstracción de su implementación para que ambas puedan variar de forma independiente.

## Problema cotidiano

En sistemas reales una idea de negocio suele combinarse con distintas formas de ejecutarse:

- Notificaciones que pueden enviarse por email, SMS o push.
- Reportes que pueden renderizarse como PDF, HTML o texto.
- Almacenamiento que puede usar proveedor local o nube.

Si cada combinación se modela como un tipo distinto, el número de variantes crece rápido. Bridge mantiene una jerarquía para la abstracción y otra para la implementación.

## Cómo se ve en Rust

En Rust, Bridge suele modelarse con traits para la implementación y structs para la abstracción que reciben esa implementación como dependencia. Puede usarse genéricos para despacho estático o trait objects para elegir la implementación en tiempo de ejecución.

## Cuándo usarlo

- Cuando una abstracción y su mecanismo de ejecución cambian por razones distintas.
- Cuando quieres evitar una explosión de tipos por combinaciones.
- Cuando necesitas cambiar implementaciones sin tocar el flujo de alto nivel.

## Cuándo evitarlo

- Si solo hay una implementación real.
- Si un `enum` cerrado expresa mejor todas las variantes.
- Si la separación introduce indirección sin reducir complejidad.

## Diferencia con Adapter

Adapter traduce una interfaz incompatible. Bridge diseña dos dimensiones que evolucionan por separado desde el inicio.

## Ejemplos

- [x] Notificaciones desacopladas de canales email, SMS y push.
- [x] Reportes desacoplados de renderizadores PDF, HTML y texto.
- [x] Almacenamiento desacoplado de proveedores local y nube.

### Notificaciones y canales

El módulo `notifications` separa la abstracción `AlertNotification` de los canales `EmailChannel`, `SmsChannel` y `PushChannel`.

El ejemplo muestra cómo una misma notificación puede enviarse por canales distintos sin crear una struct por cada combinación.

### Reportes y renderizadores

El módulo `reports` separa la lógica de `SalesReport` de los renderizadores `PdfRenderer`, `HtmlRenderer` y `TextRenderer`.

El ejemplo muestra cómo cambiar la salida del reporte sin modificar el cálculo ni el formato lógico del reporte.

### Almacenamiento y proveedores

El módulo `storage` separa `DocumentStore` de los proveedores `LocalStorageProvider` y `CloudStorageProvider`.

El ejemplo muestra cómo guardar el mismo documento en destinos distintos sin cambiar el flujo de alto nivel.

## Comandos

```bash
cargo test bridge
```
