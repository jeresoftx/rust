# Retry with Backoff

Retry with Backoff reintenta operaciones fallidas con esperas crecientes entre intentos. El patrón ayuda a tolerar fallas temporales sin presionar más a una dependencia que ya está degradada.

## Qué problema resuelve

- Reduce reintentos inmediatos contra servicios saturados.
- Diferencia errores transitorios de errores permanentes.
- Permite limitar intentos máximos.
- Puede agregar jitter para evitar que muchas instancias reintenten al mismo tiempo.

## Ejemplos del repositorio

- [x] Cliente HTTP simulado con backoff exponencial.
- [x] Reintentos solo para errores transitorios.
- [ ] Jitter determinista para evitar reintentos sincronizados.

## Código

- Documentación local: `patterns/distributed_systems/retry_with_backoff/README.md`
- Módulo Rust: `src/patterns/distributed_systems/retry_with_backoff.rs`
- Ejemplo HTTP: `src/patterns/distributed_systems/retry_with_backoff/http_client.rs`
- Ejemplo de errores transitorios: `src/patterns/distributed_systems/retry_with_backoff/transient_errors.rs`
