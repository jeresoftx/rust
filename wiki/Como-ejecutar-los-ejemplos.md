# Cómo ejecutar los ejemplos

Todos los ejemplos deben compilar con Cargo y tener pruebas o asserts ejecutables.

## Verificar todo el proyecto

```bash
cargo fmt --check
cargo check
cargo test --quiet
```

## Ejecutar una prueba específica

```bash
cargo test exposes_the_three_learning_families
```

## Convención de los ejemplos

Cada patrón mantiene una documentación local con:

- Intención del patrón.
- Problema cotidiano que resuelve.
- Cuándo usarlo en Rust.
- Cuándo evitarlo.
- Ejemplos incluidos.
- Comando para ejecutar sus pruebas.

Cada ejemplo práctico debe entrar en un commit individual.
