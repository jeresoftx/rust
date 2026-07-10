# Composite

Composite permite trabajar con elementos individuales y grupos usando una misma interfaz.

## Idea central

La estructura se representa como un árbol. Las hojas contienen el dato final y los nodos compuestos agrupan otros nodos. El cliente ejecuta una operación sobre la raíz y la operación se propaga por el árbol.

En Rust suele aparecer como:

- `enum` recursivos para árboles con variantes conocidas.
- Structs con `Vec` de hijos.
- Traits cuando se necesita polimorfismo abierto.

## Ejemplos del repositorio

- Árbol de permisos por módulo y acción: `src/patterns/gof/structural/composite/permissions.rs`
- Estructura de menú con submenús.
- Carpeta con archivos y subcarpetas para calcular tamaño.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/composite/README.md`
