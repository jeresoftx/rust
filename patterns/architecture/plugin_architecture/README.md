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

- [ ] Plugins de exportación JSON, CSV y texto.
- [ ] Registro de estrategias cargadas por configuración.
- [ ] Extensiones internas mediante traits y trait objects.

## Cómo ejecutar

```bash
cargo test plugin_architecture
```
