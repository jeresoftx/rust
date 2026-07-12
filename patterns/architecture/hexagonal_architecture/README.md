# Hexagonal Architecture

## Intención

Hexagonal Architecture separa el núcleo de la aplicación de los detalles externos mediante puertos y adaptadores. El dominio y los casos de uso definen lo que necesitan; la infraestructura implementa cómo conectarse con bases de datos, APIs, colas o notificaciones.

## Problema cotidiano

En sistemas reales, una regla de negocio puede quedar acoplada a detalles técnicos:

- Un checkout depende directamente de una base de datos concreta.
- Un caso de uso llama a un cliente HTTP específico.
- Las pruebas necesitan levantar infraestructura innecesaria.
- Cambiar un proveedor de notificaciones obliga a tocar lógica de negocio.

Con puertos y adaptadores, el núcleo queda protegido. Los detalles técnicos se enchufan desde afuera.

## Partes principales

- Dominio: reglas y entidades del negocio.
- Aplicación: casos de uso que orquestan puertos.
- Puertos de entrada: API que ofrece el núcleo.
- Puertos de salida: traits que el núcleo necesita.
- Adaptadores: implementaciones concretas de esos puertos.

## Cómo se ve en Rust

Los puertos suelen expresarse con traits:

```rust
trait OrderRepository {
    fn save(&mut self, order_id: &str);
}
```

Los casos de uso reciben implementaciones genéricas o trait objects. Así se puede cambiar un adaptador sin cambiar el núcleo.

## Cuándo usarlo

- Cuando el dominio debe sobrevivir a cambios de infraestructura.
- Cuando necesitas probar casos de uso sin bases de datos reales.
- Cuando hay varios adaptadores posibles para el mismo puerto.
- Cuando quieres que los casos de uso definan sus dependencias.

## Cuándo evitarlo

- Si la aplicación es demasiado pequeña para justificar los puertos.
- Si los traits duplican una sola llamada sin aportar flexibilidad.
- Si los adaptadores empiezan a filtrar detalles técnicos al dominio.
- Si el equipo no mantiene reglas claras de dependencia.

## Ejemplos

- [x] Caso de uso de checkout con puertos y adaptadores.
- [ ] Repositorio en memoria y repositorio simulado externo.
- [ ] Adaptador de notificaciones intercambiable.

### Checkout con puertos y adaptadores

El módulo `checkout_ports_adapters` coloca el caso de uso de checkout en el centro. El núcleo solo conoce puertos de inventario y pagos; los adaptadores concretos viven en `adapters`.

Esto permite cambiar el inventario o el gateway de pagos sin modificar el caso de uso.

## Comandos

```bash
cargo test hexagonal_architecture
```
