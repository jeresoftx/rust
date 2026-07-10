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

- [ ] Notificaciones desacopladas de canales email, SMS y push.
- [ ] Reportes desacoplados de renderizadores PDF, HTML y texto.
- [ ] Almacenamiento desacoplado de proveedores local y nube.

## Comandos

```bash
cargo test bridge
```
