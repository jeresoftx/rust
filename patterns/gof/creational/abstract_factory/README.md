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

## Ejemplos

- [x] Familia de conectores para bases de datos SQL y NoSQL.
- [x] Componentes de UI para consola y web.
- [x] Clientes de proveedor de pagos para Stripe-like y PayPal-like.

### Familia de conectores SQL y NoSQL

El modulo `database_connectors` define una factory abstracta que crea dos productos compatibles:

- Una conexion de base de datos.
- Un constructor de consultas.

`SqlDatabaseFactory` entrega una familia Postgres + SQL. `NoSqlDatabaseFactory` entrega una familia MongoDB + consulta documental. El codigo cliente usa `describe_database_stack` contra el trait `DatabaseFactory`, sin conocer los tipos concretos.

### Componentes de UI para consola y web

El modulo `ui_components` define una factory abstracta para crear botones y entradas de texto compatibles dentro de una misma familia visual.

`ConsoleUiFactory` renderiza un formulario de login para terminal. `WebUiFactory` renderiza los mismos conceptos como HTML. El cliente usa `render_login_form` contra el trait `UiFactory`.

### Clientes de proveedor de pagos

El modulo `payment_providers` crea una familia completa de piezas para checkout:

- Un autorizador de pago.
- Un publicador de recibos.

`StripeLikeFactory` y `PaypalLikeFactory` producen familias compatibles con el mismo flujo `checkout`, pero con formatos de autorizacion y recibo propios de cada proveedor.

## Comandos

```bash
cargo test abstract_factory
```
