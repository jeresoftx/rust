# Pipeline Architecture

## Intención

Pipeline Architecture organiza un proceso como una cadena de etapas. Cada etapa hace una cosa bien definida: parsear, validar, enriquecer, transformar, filtrar o producir una salida.

El patrón ayuda cuando un flujo tiene varios pasos repetibles y queremos que cada paso sea fácil de probar, reordenar o reemplazar.

## Problema cotidiano

En sistemas reales aparecen flujos como:

- Importar un CSV, limpiar campos, validar datos y generar registros finales.
- Recibir eventos, descartar los inválidos, enriquecerlos y decidir su prioridad.
- Procesar imágenes pasando por redimensionado, marcas de agua y optimización.

Sin pipeline, estos pasos suelen terminar como una función larga con muchas ramas y efectos secundarios mezclados.

## Partes

- **Entrada:** datos crudos o parcialmente procesados.
- **Etapas:** funciones, structs o traits que transforman la entrada.
- **Contexto:** datos compartidos o configuración necesaria para las etapas.
- **Salida:** resultado final del flujo.
- **Errores:** fallos detenidos temprano o acumulados según el caso de uso.

## Cuándo usarlo

Úsalo cuando el proceso se puede explicar como una secuencia clara de transformaciones. Es especialmente útil en importaciones, integración de eventos, procesamiento de archivos, validación progresiva y flujos de datos.

Evítalo cuando el flujo necesita decisiones altamente ramificadas o interacción compleja entre etapas. En esos casos puede ser mejor un orquestador explícito o una máquina de estados.

## Ejemplos

- [x] Pipeline ETL para importar CSV.
- [x] Pipeline de validación y enriquecimiento de eventos.
- [x] Pipeline de procesamiento de imágenes simulado.

### Pipeline ETL para importar CSV

El módulo `csv_etl` divide una importación en pasos pequeños: validar encabezado, parsear filas, normalizar campos y validar datos requeridos.

El resultado es un `ImportReport` con clientes importados y errores por fila. En un sistema real este estilo ayuda a mostrar al usuario qué filas debe corregir sin detener toda la importación.

### Pipeline de validación y enriquecimiento de eventos

El módulo `event_enrichment` procesa eventos crudos en varias etapas: valida actores permitidos, filtra ruido de baja prioridad, normaliza el tipo del evento y agrega una categoría.

Este ejemplo es común en integraciones, auditoría, analítica o consumidores de colas donde cada evento debe pasar por reglas pequeñas antes de ser publicado a otro sistema.

### Pipeline de procesamiento de imágenes simulado

El módulo `image_processing` no depende de librerías externas; trabaja con metadatos de una imagen para mostrar el patrón. El flujo valida dimensiones, redimensiona a un ancho máximo, aplica una marca de agua y optimiza el tamaño simulado.

El punto didáctico es que cada etapa recibe una imagen procesada parcial y devuelve la siguiente versión, dejando una lista de operaciones para auditar el flujo.

## Cómo ejecutar

```bash
cargo test pipeline_architecture
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
