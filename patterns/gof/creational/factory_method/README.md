# Factory Method

## Intención

Factory Method delega la creación de un producto concreto a una función o método especializado, mientras el código cliente trabaja contra una abstracción común.

## Problema cotidiano

En sistemas reales a menudo se necesita elegir una implementación según un dato de entrada:

- El formato de un archivo que debe procesarse.
- El canal de pago que llega desde checkout.
- El ambiente donde se ejecuta un cliente HTTP.

El código que consume el producto no debería llenarse de ramas con detalles de construcción. La decisión de creación vive en un punto claro.

## Cómo se ve en Rust

En Rust puede modelarse con una función asociada, una función libre, un trait factory o un `match` que devuelve un trait object. Cuando las variantes son cerradas, un `enum` también puede ser más directo.

## Cuándo usarlo

- Cuando varias implementaciones cumplen el mismo contrato.
- Cuando la elección depende de configuración, input o ambiente.
- Cuando quieres aislar la creación para que el cliente use una abstracción.

## Cuándo evitarlo

- Si solo existe una implementación.
- Si un `match` local es más claro y no se repite.
- Si introducir trait objects complica una solución que podría ser un `enum`.

## Diferencia con Abstract Factory

Factory Method crea un producto concreto de una familia. Abstract Factory crea familias completas de productos relacionados.

## Ejemplos

- [ ] Creación de procesadores de archivos CSV, JSON y XML.
- [ ] Creación de métodos de pago según canal.
- [ ] Creación de clientes HTTP por ambiente.

## Comandos

```bash
cargo test factory_method
```
