# CQRS

## Intención

CQRS separa comandos y consultas para que las operaciones que cambian estado no compartan el mismo modelo mental que las operaciones de lectura. Los comandos expresan intención y validan invariantes; las consultas devuelven vistas preparadas para leer rápido y con claridad.

## Problema cotidiano

En sistemas reales, un mismo modelo suele intentar resolver demasiadas cosas:

- Un endpoint actualiza inventario y también arma una respuesta de dashboard.
- Las consultas de reportes arrastran reglas de escritura que no necesitan.
- Las escrituras se vuelven lentas por mantener vistas complejas en línea.
- Los DTOs de lectura terminan filtrándose al modelo que protege reglas.

CQRS permite diseñar el lado de escritura y el lado de lectura con necesidades distintas.

## Partes principales

- Comandos: solicitudes de cambio, como reservar inventario o ajustar stock.
- Handlers de comando: validan reglas y modifican el modelo de escritura.
- Consultas: solicitudes de lectura sin efectos secundarios.
- Handlers de consulta: devuelven modelos optimizados para pantalla, API o reportes.
- Proyecciones: datos derivados desde escrituras hacia modelos de lectura.

## Cómo se ve en Rust

Rust permite expresar la separación con tipos y módulos explícitos:

```rust
struct ReserveStockCommand {
    sku: String,
    quantity: u32,
}
```

El handler de comando puede devolver un resultado pequeño, mientras que el handler de consulta puede devolver una vista lista para presentar.

## Cuándo usarlo

- Cuando las lecturas y escrituras tienen necesidades diferentes.
- Cuando un dashboard necesita vistas prearmadas.
- Cuando quieres evitar que consultas complejas contaminen reglas de escritura.
- Cuando hay eventos o proyecciones que alimentan modelos de lectura.

## Cuándo evitarlo

- Si el CRUD simple es suficiente.
- Si separar modelos agrega duplicación sin beneficio.
- Si no existe una razón real para tener proyecciones.
- Si la consistencia eventual no es aceptable para el caso.

## Ejemplos

- [x] Comandos separados de consultas para inventario.
- [x] Modelo de lectura optimizado para dashboard.
- [x] Sincronización simple entre escritura y lectura.

### Comandos separados de consultas para inventario

El módulo `inventory_commands_queries` separa el lado de escritura y lectura para inventario. `ReceiveStockCommand` y `ReserveStockCommand` pasan por `InventoryCommandHandler`; `GetInventoryQuery` pasa por `InventoryQueryHandler`.

Las consultas no cambian el estado y los comandos no devuelven una vista de dashboard. Esa separación mantiene clara la intención de cada operación.

### Modelo de lectura optimizado para dashboard

El módulo `dashboard_read_model` usa snapshots de inventario para construir tarjetas listas para UI. La consulta puede pedir todos los productos o solo los que necesitan atención.

La read model ordena primero productos sin stock, luego stock bajo y al final productos saludables. Esta vista está diseñada para leer y presentar, no para proteger invariantes de escritura.

### Sincronización simple entre escritura y lectura

El módulo `write_read_sync` usa eventos `StockChanged` para mover cambios del write model al read model. El lado de escritura emite eventos al recibir o reservar stock; la proyección aplica esos eventos en orden.

El ejemplo también muestra cómo reconstruir un read model desde una lista de eventos, útil para explicar la relación entre escritura, proyección y lectura sin introducir infraestructura externa.

## Comandos

```bash
cargo test cqrs
```
