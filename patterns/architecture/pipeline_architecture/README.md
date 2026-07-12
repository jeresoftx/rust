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
- [ ] Pipeline de procesamiento de imágenes simulado.

### Pipeline ETL para importar CSV

El módulo `csv_etl` divide una importación en pasos pequeños: validar encabezado, parsear filas, normalizar campos y validar datos requeridos.

El resultado es un `ImportReport` con clientes importados y errores por fila. En un sistema real este estilo ayuda a mostrar al usuario qué filas debe corregir sin detener toda la importación.

### Pipeline de validación y enriquecimiento de eventos

El módulo `event_enrichment` procesa eventos crudos en varias etapas: valida actores permitidos, filtra ruido de baja prioridad, normaliza el tipo del evento y agrega una categoría.

Este ejemplo es común en integraciones, auditoría, analítica o consumidores de colas donde cada evento debe pasar por reglas pequeñas antes de ser publicado a otro sistema.

## Cómo ejecutar

```bash
cargo test pipeline_architecture
```
