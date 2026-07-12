# Health Checks y Readiness

Health Checks y Readiness reportan si un servicio está vivo, listo para recibir tráfico y si sus dependencias críticas están disponibles.

## Qué problema resuelve

- Evita enviar tráfico a procesos no listos.
- Diferencia liveness de readiness.
- Expone fallas de dependencias críticas.
- Da reportes simples para orquestadores.
- Ayuda a operar despliegues y reinicios.

## Ejemplos del repositorio

- [ ] Chequeos de dependencias críticas.
- [ ] Readiness separado de liveness.
- [ ] Reporte agregado para orquestadores.

## Código

- Documentación local: `patterns/distributed_systems/health_checks_readiness/README.md`
- Módulo Rust: `src/patterns/distributed_systems/health_checks_readiness.rs`
