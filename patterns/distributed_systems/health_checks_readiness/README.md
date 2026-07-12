# Health Checks y Readiness

## Intención

Health Checks y Readiness permiten reportar si un servicio está vivo, listo para recibir tráfico y si sus dependencias críticas funcionan.

## Problema cotidiano

Un proceso puede estar encendido pero no listo: quizá no cargó configuración, no conecta a base de datos o perdió una dependencia crítica. Si el balanceador envía tráfico de todos modos, los usuarios reciben errores.

## Partes

- **Liveness:** indica si el proceso sigue vivo.
- **Readiness:** indica si puede recibir tráfico.
- **Dependencias críticas:** recursos necesarios para operar.
- **Reporte agregado:** salida resumida para monitoreo u orquestadores.

## Cuándo usarlo

Úsalo en servicios HTTP, workers, APIs y procesos desplegados en orquestadores.

## Cuándo evitarlo

Evítalo como reemplazo de métricas profundas. Un health check debe ser rápido, claro y barato.

## Ejemplos

- [x] Chequeos de dependencias críticas.
- [ ] Readiness separado de liveness.
- [ ] Reporte agregado para orquestadores.

### Chequeos de dependencias críticas

El primer ejemplo evalúa base de datos y broker como dependencias críticas.

El módulo `critical_dependencies` distingue dependencias críticas y opcionales. Solo una dependencia crítica caída vuelve `Unhealthy` el reporte.

### Readiness separado de liveness

El segundo ejemplo muestra que un proceso puede estar vivo pero no listo.

### Reporte agregado para orquestadores

El tercer ejemplo produce un resumen con estado general y detalles por componente.

## Cómo ejecutar

```bash
cargo test health_checks_readiness
```
