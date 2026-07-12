# Service Layer

Service Layer concentra los casos de uso de aplicación en servicios explícitos. En lugar de dejar que controladores, jobs o comandos de consola coordinen validaciones, repositorios, políticas y gateways, el servicio se convierte en el punto de entrada del flujo.

## Qué problema resuelve

- Evita controladores cargados de lógica de negocio.
- Reduce duplicación entre HTTP, CLI, workers y tareas programadas.
- Da nombres claros a casos de uso como registrar usuario, confirmar checkout o construir reportes.
- Facilita pruebas enfocadas sobre la orquestación de aplicación.

## Estructura típica

- Comando o consulta de entrada.
- Servicio de aplicación.
- Repositorios y gateways.
- Políticas de aplicación.
- Respuesta o DTO de salida.

## Ejemplos del repositorio

- [x] Registro de usuario.
- [ ] Checkout coordinado.
- [ ] Reportes complejos.

## Código

- Documentación local: `patterns/architecture/service_layer/README.md`
- Módulo Rust: `src/patterns/architecture/service_layer.rs`
- Ejemplo de registro: `src/patterns/architecture/service_layer/user_registration.rs`
