# Health Checks y Readiness

Health Checks y Readiness reportan si un servicio está vivo, listo para recibir tráfico y si sus dependencias críticas están disponibles.

## Qué problema resuelve

- Evita enviar tráfico a procesos no listos.
- Diferencia liveness de readiness.
- Expone fallas de dependencias críticas.
- Da reportes simples para orquestadores.
- Ayuda a operar despliegues y reinicios.

## Ejemplos del repositorio

- [x] Chequeos de dependencias críticas.
- [x] Readiness separado de liveness.
- [ ] Reporte agregado para orquestadores.

## Código

- Documentación local: `patterns/distributed_systems/health_checks_readiness/README.md`
- Módulo Rust: `src/patterns/distributed_systems/health_checks_readiness.rs`
- Ejemplo de dependencias críticas: `src/patterns/distributed_systems/health_checks_readiness/critical_dependencies.rs`
- Ejemplo de readiness y liveness: `src/patterns/distributed_systems/health_checks_readiness/readiness_liveness.rs`
