# Facade

Facade ofrece una entrada simple para un subsistema con varias piezas internas.

## Idea central

La fachada expone métodos orientados al caso de uso. Por dentro coordina componentes especializados, pero el cliente no necesita conocer el orden ni los detalles de esa coordinación.

En Rust suele aparecer como:

- Structs de servicio con métodos de caso de uso.
- Dependencias internas pequeñas y explícitas.
- Resultados claros cuando un paso interno falla.

## Ejemplos del repositorio

- Servicio de checkout que coordina carrito, pago e inventario: `src/patterns/gof/structural/facade/checkout.rs`
- API simple para enviar notificaciones multicanal.
- Generador de reporte que oculta carga, cálculo y render.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/facade/README.md`
