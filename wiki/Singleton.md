# Singleton

Singleton ofrece una instancia compartida y controlada de un recurso que debe existir una sola vez.

## Idea central

El valor se inicializa una vez y después se consulta desde varios puntos del sistema. En Rust se debe modelar con inicialización explícita y protección de concurrencia cuando hay mutabilidad.

En Rust suele aparecer como:

- `OnceLock<T>` para inicialización única.
- `Mutex<T>` o tipos atómicos cuando el recurso compartido cambia.
- Funciones pequeñas que ocultan los detalles de acceso global.

## Ejemplos del repositorio

- Configuración global de aplicación con `OnceLock`.
- Registro centralizado de métricas.
- Pool compartido simulado para recursos costosos.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/creational/singleton/README.md`
