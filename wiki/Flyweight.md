# Flyweight

Flyweight comparte datos repetidos para evitar duplicarlos en muchos objetos.

## Idea central

El estado compartido vive en un catálogo, caché o registro. Cada objeto concreto conserva solo el estado que cambia y referencia los datos comunes.

En Rust suele aparecer como:

- `Arc<T>` o `Rc<T>` para compartir datos inmutables.
- Registros por clave que devuelven referencias compartidas.
- Separación explícita entre estado compartido y estado específico.

## Ejemplos del repositorio

- Catálogo compartido de monedas o países: `src/patterns/gof/structural/flyweight/currency_catalog.rs`
- Caché de metadatos de productos repetidos: `src/patterns/gof/structural/flyweight/product_metadata.rs`
- Reutilización de estilos en render de documentos: `src/patterns/gof/structural/flyweight/document_styles.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/flyweight/README.md`
