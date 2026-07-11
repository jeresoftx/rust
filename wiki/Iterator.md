# Iterator

Iterator recorre elementos sin exponer los detalles internos de la colección, fuente de datos o estructura.

## Idea central

El consumidor pide el siguiente elemento. El iterador decide cómo obtenerlo: desde una página remota, desde un lote local, desde una pila interna para recorrer un árbol o desde cualquier otra estructura.

En Rust suele aparecer como:

- Tipos que implementan el trait `Iterator`.
- Métodos de dominio como `depth_first` cuando el orden del recorrido importa.
- Composición con adaptadores como `map`, `filter`, `take` o `collect`.

## Ejemplos del repositorio

- Paginación sobre resultados de API: `src/patterns/gof/behavioral/iterator/paginated_api.rs`
- Iterador de lotes para procesamiento de registros: `src/patterns/gof/behavioral/iterator/record_batches.rs`
- Recorrido de árbol de categorías: `src/patterns/gof/behavioral/iterator/category_tree.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/iterator/README.md`
