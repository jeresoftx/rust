# Template Method

Template Method define un flujo común y permite personalizar pasos específicos.

## Idea central

El método plantilla mantiene el orden del algoritmo. Las implementaciones concretas completan los pasos variables mediante hooks o métodos requeridos.

En Rust suele aparecer como:

- Traits con métodos por defecto.
- Un método principal que llama pasos pequeños.
- Hooks opcionales con implementación vacía o estándar.
- Tipos concretos que implementan solo lo que cambia.

## Ejemplos del repositorio

- Flujo común para importar archivos con pasos variables: `src/patterns/gof/behavioral/template_method/file_import.rs`
- Generación de reportes con secciones personalizadas: pendiente.
- Proceso de onboarding con hooks por tipo de cuenta: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/template_method/README.md`
