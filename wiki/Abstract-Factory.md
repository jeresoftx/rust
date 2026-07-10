# Abstract Factory

Abstract Factory permite construir familias de objetos compatibles detras de una interfaz comun.

## Idea central

El cliente pide una familia de productos. La factory decide que implementaciones concretas entregar.

En Rust, la implementacion suele apoyarse en:

- Traits para definir contratos.
- Structs concretos para cada familia.
- `Box<dyn Trait>` cuando se necesita polimorfismo dinamico.
- Generics cuando la familia puede resolverse en tiempo de compilacion.

## Ejemplos del repositorio

- Conectores SQL y NoSQL: `src/patterns/gof/creational/abstract_factory/database_connectors.rs`
- Componentes de UI para consola y web: `src/patterns/gof/creational/abstract_factory/ui_components.rs`
- Proveedores de pago Stripe-like y PayPal-like.

## Guia tecnica

La guia cercana al codigo vive en:

`patterns/gof/creational/abstract_factory/README.md`
