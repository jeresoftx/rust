# Bridge

Bridge separa una abstracción de su implementación para que ambas puedan evolucionar sin crear una clase o tipo por cada combinación.

## Idea central

La abstracción representa el flujo de negocio. La implementación representa el mecanismo concreto. La abstracción delega en la implementación mediante un trait.

En Rust suele aparecer como:

- Traits para canales, renderizadores o proveedores.
- Structs de alto nivel que reciben una implementación.
- Genéricos o `Box<dyn Trait>` según se necesite despacho estático o dinámico.

## Ejemplos del repositorio

- Notificaciones desacopladas de canales email, SMS y push: `src/patterns/gof/structural/bridge/notifications.rs`
- Reportes desacoplados de renderizadores PDF, HTML y texto: `src/patterns/gof/structural/bridge/reports.rs`
- Almacenamiento desacoplado de proveedores local y nube: `src/patterns/gof/structural/bridge/storage.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/bridge/README.md`
