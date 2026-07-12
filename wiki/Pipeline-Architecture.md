# Pipeline Architecture

Pipeline Architecture divide un flujo de trabajo en etapas pequeñas y ordenadas. Cada etapa recibe una entrada, la transforma o valida, y entrega el resultado a la siguiente etapa.

## Qué problema resuelve

- Evita funciones largas que mezclan parseo, validación, enriquecimiento y salida.
- Facilita probar cada flujo con datos de entrada y salida claros.
- Permite reordenar, reemplazar o reutilizar etapas.
- Hace más expresivos procesos como importaciones, eventos y procesamiento de archivos.

## Estructura típica

- Entrada cruda.
- Etapas de transformación.
- Validaciones o filtros.
- Contexto opcional.
- Salida final.

## Ejemplos del repositorio

- [x] ETL CSV.
- [x] Validación y enriquecimiento de eventos.
- [x] Procesamiento de imágenes simulado.

## Código

- Documentación local: `patterns/architecture/pipeline_architecture/README.md`
- Módulo Rust: `src/patterns/architecture/pipeline_architecture.rs`
- Ejemplo ETL CSV: `src/patterns/architecture/pipeline_architecture/csv_etl.rs`
- Ejemplo de eventos: `src/patterns/architecture/pipeline_architecture/event_enrichment.rs`
- Ejemplo de imágenes: `src/patterns/architecture/pipeline_architecture/image_processing.rs`
