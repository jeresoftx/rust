# Adapter

## Intención

Adapter convierte la interfaz de un componente existente en la interfaz que espera el código cliente.

## Problema cotidiano

En sistemas reales no siempre controlamos las formas de los datos ni las APIs que consumimos:

- Un proveedor externo de pagos usa nombres y tipos distintos a los del dominio interno.
- Un sistema legacy entrega usuarios en un formato viejo.
- Una librería de logging expone métodos que no coinciden con nuestro trait.

El código de negocio no debería quedar lleno de conversiones ni detalles del proveedor. Adapter concentra esa traducción en una pieza pequeña.

## Cómo se ve en Rust

En Rust, Adapter suele modelarse con structs envoltorio, funciones de conversión, implementación de traits propios sobre tipos externos o `From`/`TryFrom` cuando la conversión es natural.

La versión rústica favorece interfaces explícitas y evita ocultar errores de conversión.

## Cuándo usarlo

- Cuando necesitas integrar una API externa sin contaminar el dominio.
- Cuando migras de un modelo legacy a uno nuevo.
- Cuando quieres que el código cliente dependa de un trait propio y estable.

## Cuándo evitarlo

- Si una conversión directa con `From` o `TryFrom` basta y es clara.
- Si el adapter solo renombra métodos sin aportar aislamiento real.
- Si puedes cambiar la interfaz original porque también la controlas.

## Diferencia con Facade

Adapter cambia una interfaz para que encaje con otra. Facade ofrece una entrada simple sobre un subsistema más grande.

## Ejemplos

- [x] Adaptar un cliente externo de pagos a una interfaz interna.
- [x] Adaptar formatos legacy de usuario a un modelo nuevo.
- [ ] Adaptar logger de terceros a un trait propio.

### Cliente externo de pagos

El módulo `payment_gateway` envuelve un cliente externo con nombres ajenos al dominio y lo expone como `PaymentGateway`.

El ejemplo muestra cómo el código cliente cobra usando un trait interno sin depender de los métodos del proveedor.

### Usuario legacy a modelo nuevo

El módulo `legacy_user` convierte registros antiguos hacia un `UserProfile` moderno.

El ejemplo muestra una adaptación fallible: si el correo legacy no cumple la regla mínima, la conversión devuelve un error explícito.

## Comandos

```bash
cargo test adapter
```
