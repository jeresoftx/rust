# Cómo ejecutar los ejemplos

Todos los ejemplos deben compilar con Cargo y tener pruebas o asserts ejecutables.

## Verificar todo el proyecto

```bash
cargo fmt --check
cargo check
cargo test
```

## Ejecutar una prueba específica

```bash
cargo test exposes_the_three_learning_families
```

## Convención para futuros ejemplos

Cada patrón agregará una documentación local con:

- Intención del patrón.
- Problema cotidiano que resuelve.
- Cuándo usarlo en Rust.
- Cuándo evitarlo.
- Ejemplos incluidos.
- Comando para ejecutar sus pruebas.

Cada ejemplo práctico debe entrar en un commit individual.
