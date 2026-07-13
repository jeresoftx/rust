# Plugin Architecture

## Intención

Plugin Architecture permite extender una aplicación mediante contratos estables. El núcleo define interfaces, registros y puntos de extensión; los plugins implementan capacidades concretas sin cambiar el flujo principal.

En Rust suele modelarse con `trait`, `Box<dyn Trait>`, enums de configuración o registros explícitos. No hace falta cargar librerías dinámicas para enseñar el patrón: muchas arquitecturas internas usan plugins compilados junto al binario.

## Problema cotidiano

En sistemas reales aparecen necesidades como:

- Exportar datos en varios formatos sin llenar el servicio principal de `match` gigantes.
- Activar estrategias por configuración para clientes, países o ambientes.
- Permitir extensiones internas en un motor de reglas, un procesador de comandos o una tubería de trabajo.

Sin puntos de extensión claros, cada nueva capacidad empuja cambios sobre el núcleo y aumenta el riesgo de regresiones.

## Partes

- **Contrato del plugin:** trait o interfaz que define qué debe implementar cada extensión.
- **Plugins concretos:** implementaciones para casos específicos.
- **Registro:** estructura que guarda plugins disponibles y permite resolverlos por nombre, configuración o contexto.
- **Núcleo:** código que usa el contrato sin conocer los detalles concretos.
- **Configuración:** datos que seleccionan qué plugin o estrategia usar.

## Cuándo usarlo

Úsalo cuando el sistema debe crecer con variantes frecuentes, integraciones opcionales, formatos de salida, reglas por cliente o extensiones internas bien acotadas.

Evítalo si solo tienes dos ramas fijas que no van a crecer. Un registro de plugins prematuro puede ocultar una solución simple.

## Ejemplos

- [x] Plugins de exportación JSON, CSV y texto.
- [x] Registro de estrategias cargadas por configuración.
- [x] Extensiones internas mediante traits y trait objects.

### Plugins de exportación JSON, CSV y texto

El módulo `export_plugins` define un contrato `ExportPlugin` y un `ExportRegistry` que resuelve exportadores por clave. El núcleo solo pide `export("json", records)` o `export("csv", records)`; no conoce los detalles de cada formato.

Este estilo evita que un servicio principal acumule ramas por cada nuevo formato de salida.

### Registro de estrategias cargadas por configuración

El módulo `configured_strategies` muestra un motor de precios que selecciona una estrategia a partir de `PricingConfig`. El registro conoce las estrategias disponibles y el núcleo solo pide el cálculo.

Este enfoque aparece mucho en sistemas con reglas por cliente, ambiente, país o campaña.

### Extensiones internas mediante traits y trait objects

El módulo `internal_extensions` modela un procesador de solicitudes con extensiones internas. Cada extensión implementa `RequestExtension` y el `RequestProcessor` las ejecuta como `Box<dyn RequestExtension>`.

Es una forma sencilla de abrir puntos de extensión dentro del mismo binario sin exponer el núcleo a detalles concretos.

## Cómo ejecutar

```bash
cargo test plugin_architecture
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
