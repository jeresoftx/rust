# Proxy

Proxy controla el acceso a un recurso real sin obligar al cliente a conocer todos sus detalles.

## Idea central

El cliente habla con el proxy como si hablara con el recurso real. El proxy decide si delega, cachea, carga tarde, registra o rechaza la operación.

En Rust suele aparecer como:

- Traits para describir la interfaz esperada.
- Structs envoltorio que poseen el servicio real.
- `Result` para expresar accesos rechazados o fallas del recurso.

## Ejemplos del repositorio

- Proxy con caché para API externa.
- Proxy con autorización para repositorio.
- Proxy lazy para cargar archivos grandes.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/proxy/README.md`
