# Decorator

## Intención

Decorator añade responsabilidades a un objeto envolviéndolo con otros objetos que mantienen la misma interfaz.

## Problema cotidiano

En sistemas reales una capacidad base suele necesitar capas opcionales:

- Un cliente HTTP con logging, timeout y reintentos.
- Un repositorio con caché encima del almacenamiento real.
- Un flujo de validación que agrega reglas sobre una orden.

Crear una variante por cada combinación termina duplicando código. Decorator permite componer capas pequeñas alrededor de una implementación base.

## Cómo se ve en Rust

En Rust, Decorator suele modelarse con traits, structs genéricas que envuelven otro implementador del trait y composición explícita. Cuando las capas son cerradas y simples, un pipeline de funciones también puede ser más directo.

## Cuándo usarlo

- Cuando necesitas añadir comportamiento sin modificar el componente base.
- Cuando quieres combinar capas de forma flexible.
- Cuando varias responsabilidades transversales deben mantenerse aisladas.

## Cuándo evitarlo

- Si la composición de capas vuelve difícil seguir el flujo.
- Si una función simple o un middleware existente expresa mejor la intención.
- Si las capas necesitan conocer demasiados detalles internos del componente base.

## Diferencia con Composite

Composite organiza objetos en árboles. Decorator envuelve un objeto para añadir comportamiento manteniendo la misma interfaz.

## Ejemplos

- [x] Cliente HTTP con retry, timeout y logging.
- [x] Repositorio con caché encima de almacenamiento base.
- [ ] Pipeline de validaciones sobre una orden.

### Cliente HTTP con capas

El módulo `http_client` define un cliente base y decoradores para timeout, retry y logging.

El ejemplo muestra cómo componer capas alrededor de la misma interfaz `HttpClient` sin cambiar el cliente base.

### Repositorio con caché

El módulo `cached_repository` envuelve un repositorio en memoria con una capa de caché.

El ejemplo muestra cómo reducir lecturas al almacenamiento base manteniendo la misma interfaz `ProductRepository`.

## Comandos

```bash
cargo test decorator
```
