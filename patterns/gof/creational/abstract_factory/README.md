# Abstract Factory

## Intencion

Abstract Factory crea familias de objetos relacionados sin acoplar el codigo cliente a sus implementaciones concretas.

## Problema cotidiano

En sistemas reales suele existir mas de una familia de implementaciones compatibles:

- Conectores SQL y NoSQL.
- Componentes de UI para consola y web.
- Proveedores de pago con APIs distintas.

El codigo de negocio no deberia conocer los detalles de construccion de cada familia. Solo necesita pedir objetos que cumplan un contrato comun.

## Como se ve en Rust

En Rust se suele modelar con traits para los productos y un trait o struct factory para construirlos. Cuando las implementaciones concretas son conocidas y cerradas, un `enum` tambien puede ser una alternativa mas simple.

## Cuando usarlo

- Cuando necesitas crear familias completas de objetos compatibles.
- Cuando el cliente debe depender de abstracciones, no de constructores concretos.
- Cuando quieres intercambiar una familia de implementaciones en tests, ambientes o proveedores.

## Cuando evitarlo

- Si solo creas un tipo de objeto; probablemente Factory Method o una funcion constructora basta.
- Si las variantes son pocas y cerradas; un `enum` puede ser mas directo.
- Si el patron obliga a usar `Box<dyn Trait>` sin una necesidad real de polimorfismo dinamico.

## Diferencia con el patron clasico

La version clasica suele depender de jerarquias de clases. En Rust preferimos traits, generics, enums y composicion. El objetivo no es imitar clases abstractas, sino separar la decision de familia del codigo que consume los productos.

## Ejemplos planeados

- [ ] Familia de conectores para bases de datos SQL y NoSQL.
- [ ] Componentes de UI para consola y web.
- [ ] Clientes de proveedor de pagos para Stripe-like y PayPal-like.

## Comandos

```bash
cargo test abstract_factory
```
