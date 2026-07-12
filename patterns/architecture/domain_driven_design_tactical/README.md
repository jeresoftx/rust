# Domain-Driven Design Táctico

## Intención

Domain-Driven Design táctico ayuda a modelar el dominio con piezas pequeñas y expresivas: entidades, value objects, agregados, servicios de dominio y eventos de dominio. Su objetivo no es crear capas por costumbre, sino proteger reglas de negocio importantes dentro del lenguaje del negocio.

## Problema cotidiano

En sistemas reales, la lógica de negocio suele dispersarse en controladores, scripts, servicios de aplicación o consultas a base de datos:

- Una orden permite estados inválidos porque sus campos son públicos.
- El dinero se representa como enteros sueltos sin moneda.
- Las políticas de descuento se duplican en varios casos de uso.
- Los cambios importantes no dejan eventos para integraciones internas.

DDD táctico concentra esas reglas en modelos ricos y explícitos.

## Partes principales

- Value objects: valores inmutables con validación e igualdad por valor.
- Entidades: objetos con identidad estable.
- Agregados: límites de consistencia que protegen invariantes.
- Servicios de dominio: reglas que no pertenecen naturalmente a una sola entidad.
- Eventos de dominio: hechos relevantes que ya ocurrieron dentro del dominio.

## Cómo se ve en Rust

Rust favorece modelos explícitos con constructores, enums y métodos que mantienen invariantes:

```rust
struct Money {
    cents: u32,
    currency: Currency,
}
```

Un agregado puede exponer métodos de intención, como `add_item`, `confirm` o `record_payment`, en vez de permitir modificar campos internos directamente.

## Cuándo usarlo

- Cuando el dominio tiene reglas importantes y vocabulario propio.
- Cuando hay invariantes que deben protegerse en memoria antes de persistir.
- Cuando varias aplicaciones comparten las mismas reglas de negocio.
- Cuando los eventos del dominio ayudan a coordinar módulos internos.

## Cuándo evitarlo

- Si el sistema es CRUD simple con reglas mínimas.
- Si solo estás renombrando DTOs como entidades sin comportamiento.
- Si los agregados crecen hasta controlar demasiados conceptos.
- Si los eventos se usan antes de que exista una necesidad clara.

## Ejemplos

- [x] Agregados y value objects para órdenes.
- [x] Servicios de dominio para políticas de descuento.
- [x] Eventos de dominio para integración interna.

### Agregados y value objects para órdenes

El módulo `order_aggregate` modela una orden como agregado. `OrderId`, `Sku` y `Money` son value objects que evitan usar strings y enteros sin significado.

El agregado `Order` protege invariantes: no permite líneas con cantidad cero, no permite confirmar órdenes vacías y bloquea cambios después de confirmar. La intención del dominio aparece en métodos como `add_item` y `confirm`.

### Servicios de dominio para políticas de descuento

El módulo `discount_service` coloca una regla que cruza varios conceptos en un servicio de dominio. La decisión de descuento usa el segmento del cliente, el subtotal del carrito y un cupón opcional.

El servicio elige la mejor política sin acumular descuentos accidentalmente. `Coupon` valida porcentajes inválidos y `DiscountBreakdown` explica el resultado con subtotal, descuento y motivo.

### Eventos de dominio para integración interna

El módulo `domain_events` registra eventos cuando el agregado cambia de estado. Al confirmar una orden, el agregado agrega un `OrderConfirmed` a su buffer interno de eventos.

El método `pull_events` entrega y limpia esos eventos para que un handler interno pueda reservar inventario o notificar facturación sin acoplar esas integraciones al agregado.

## Comandos

```bash
cargo test domain_driven_design_tactical
```
