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

- [x] Creación de procesadores de archivos CSV, JSON y XML.
- [x] Creación de métodos de pago según canal.
- [x] Creación de clientes HTTP por ambiente.

### Procesadores de archivos

El módulo `file_processors` decide qué procesador crear a partir de la extensión del archivo. El cliente llama `process_file` y recibe el resultado sin conocer `CsvProcessor`, `JsonProcessor` ni `XmlProcessor`.

Este ejemplo mantiene la lógica de creación en `processor_for`, que funciona como Factory Method.

### Métodos de pago por canal

El módulo `payment_methods` recibe un `PaymentChannel` y crea el método de pago concreto para tarjeta, transferencia bancaria o wallet.

El cliente llama `charge` y no conoce las structs concretas `CardPayment`, `BankTransferPayment` ni `WalletPayment`.

### Clientes HTTP por ambiente

El módulo `http_clients` crea un cliente distinto para local, staging o producción. Cada cliente encapsula base URL, timeout y reintentos.

El cliente llama `request_summary` con un `Environment` y no conoce los tipos concretos de cliente HTTP.

## Comandos

```bash
cargo test factory_method
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
