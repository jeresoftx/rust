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

- [ ] Pipeline ETL para importar CSV.
- [ ] Pipeline de validación y enriquecimiento de eventos.
- [ ] Pipeline de procesamiento de imágenes simulado.

## Cómo ejecutar

```bash
cargo test pipeline_architecture
```
