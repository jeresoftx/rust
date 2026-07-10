# Como ejecutar los ejemplos

Todos los ejemplos deben compilar con Cargo y tener pruebas o asserts ejecutables.

## Verificar todo el proyecto

```bash
cargo fmt --check
cargo check
cargo test
```

## Ejecutar una prueba especifica

```bash
cargo test exposes_the_three_learning_families
```

## Convencion para futuros ejemplos

Cada patron agregara una documentacion local con:

- Intencion del patron.
- Problema cotidiano que resuelve.
- Cuando usarlo en Rust.
- Cuando evitarlo.
- Ejemplos incluidos.
- Comando para ejecutar sus pruebas.

Cada ejemplo practico debe entrar en un commit individual.
