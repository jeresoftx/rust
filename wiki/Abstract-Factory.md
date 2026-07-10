# Abstract Factory

Abstract Factory permite construir familias de objetos compatibles detrás de una interfaz común.

## Idea central

El cliente pide una familia de productos. La factory decide que implementaciones concretas entregar.

En Rust, la implementación suele apoyarse en:

- Traits para definir contratos.
- Structs concretos para cada familia.
- `Box<dyn Trait>` cuando se necesita polimorfismo dinámico.
- Generics cuando la familia puede resolverse en tiempo de compilación.

## Ejemplos del repositorio

- Conectores SQL y NoSQL: `src/patterns/gof/creational/abstract_factory/database_connectors.rs`
- Componentes de UI para consola y web: `src/patterns/gof/creational/abstract_factory/ui_components.rs`
- Proveedores de pago Stripe-like y PayPal-like: `src/patterns/gof/creational/abstract_factory/payment_providers.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/creational/abstract_factory/README.md`
