# Catálogo de Patrones de Diseño en Rust

Este documento es el tablero de progreso del proyecto. La meta es construir un catálogo práctico de patrones usando Rust en tres niveles: patrones GoF, patrones idiomáticos de Rust y patrones de arquitectura para sistemas reales. Cada nivel debe tener documentación clara, ejemplos ejecutables y commits pequeños.

## Reglas de trabajo

- [x] Mantener este archivo actualizado en cada avance.
- [x] Crear un README principal con índice, categorías y guía de ejecución.
- [x] Usar Rust idiomático antes que traducciones literales de OOP.
- [x] Incluir ejemplos cercanos al día a día de sistemas: configuración, pagos, notificaciones, caché, logs, APIs, colas, reportes, permisos y procesamiento de datos.
- [x] Cada patrón debe tener su propia documentación.
- [x] Cada ejemplo debe compilar y tener pruebas o asserts ejecutables.
- [x] Cada ejemplo práctico debe ir en un commit individual.
- [x] Cuando un patrón tenga varios ejemplos, cada ejemplo se commitea por separado.
- [x] Los commits deben ser descriptivos, por ejemplo: `feat: add factory method payment example`.

## Estructura del proyecto

```text
.
├── Cargo.toml
├── README.md
├── plan/
│   └── catalogo-patrones-rust.md
├── patterns/
│   ├── gof/
│   ├── rust_idiomatic/
│   └── architecture/
├── src/
│   └── patterns/
├── tests/
└── wiki/
```

Cada patrón seguirá esta forma:

```text
patterns/<familia>/<categoria-opcional>/<patron>/README.md
src/patterns/<familia>/<categoria-opcional>/<patron>.rs
src/patterns/<familia>/<categoria-opcional>/<patron>/<ejemplo>.rs
tests/<patron>_<ejemplo>_test.rs
```

El objetivo se mantiene: cada patrón queda aislado, documentado y fácil de ejecutar.

## Formato de documentación por patrón

- [x] Intención del patrón.
- [x] Problema cotidiano que resuelve.
- [x] Cuándo usarlo en Rust.
- [x] Cuándo evitarlo en Rust.
- [x] Lista de ejemplos incluidos.
- [x] Comando para ejecutar pruebas o ejemplo.
- [x] Diferencias o notas idiomáticas cuando el patrón clásico cambia al llevarlo a Rust.
- [x] Diagrama simple o explicación estructural cuando ayuda a entender la relación entre piezas.

## Fase 0: Base del proyecto

- [x] Crear carpeta `plan/` y documento de progreso.
- [x] Crear crate Rust base con `Cargo.toml`.
- [x] Crear README principal del repositorio.
- [x] Crear wiki local versionada en `wiki/`.
- [x] Crear módulos base por familia: GoF, Rust idiomático y arquitectura.
- [x] Definir convenciones de nombres, pruebas y comandos.
- [x] Verificar `cargo fmt`, `cargo check` y `cargo test`.
- [x] Commit de la base del proyecto.

## Fase 1: Patrones GoF

Primero construiremos los 23 patrones clásicos GoF, explicando cómo traducirlos a Rust sin forzar herencia o jerarquías innecesarias.

### Subfase 1.1: Patrones creacionales

#### 1. Abstract Factory

- [x] Documentación del patrón.
- [x] Ejemplo: familia de conectores para bases de datos SQL y NoSQL.
- [x] Ejemplo: componentes de UI para consola y web.
- [x] Ejemplo: clientes de proveedor de pagos para Stripe-like y PayPal-like.

#### 2. Builder

- [x] Documentación del patrón.
- [x] Ejemplo: construcción de configuración de servidor HTTP.
- [x] Ejemplo: construcción de consultas de búsqueda con filtros opcionales.
- [x] Ejemplo: construcción de payload de email transaccional.

#### 3. Factory Method

- [x] Documentación del patrón.
- [x] Ejemplo: creación de procesadores de archivos CSV, JSON y XML.
- [x] Ejemplo: creación de métodos de pago según canal.
- [x] Ejemplo: creación de clientes HTTP por ambiente.

#### 4. Prototype

- [x] Documentación del patrón.
- [x] Ejemplo: clonar plantillas de reportes con ajustes por cliente.
- [x] Ejemplo: duplicar configuraciones base de despliegue.
- [x] Ejemplo: crear órdenes desde una plantilla preconfigurada.

#### 5. Singleton

- [x] Documentación del patrón.
- [x] Ejemplo: configuración global de aplicación con `OnceLock`.
- [x] Ejemplo: registro centralizado de métricas.
- [x] Ejemplo: pool compartido simulado para recursos costosos.

### Subfase 1.2: Patrones estructurales

#### 6. Adapter

- [x] Documentación del patrón.
- [x] Ejemplo: adaptar un cliente externo de pagos a una interfaz interna.
- [x] Ejemplo: adaptar formatos legacy de usuario a un modelo nuevo.
- [x] Ejemplo: adaptar logger de terceros a un trait propio.

#### 7. Bridge

- [x] Documentación del patrón.
- [x] Ejemplo: notificaciones desacopladas de canales email, SMS y push.
- [x] Ejemplo: reportes desacoplados de renderizadores PDF, HTML y texto.
- [x] Ejemplo: almacenamiento desacoplado de proveedores local y nube.

#### 8. Composite

- [x] Documentación del patrón.
- [x] Ejemplo: árbol de permisos por módulo y acción.
- [x] Ejemplo: estructura de menú con submenús.
- [x] Ejemplo: carpeta con archivos y subcarpetas para calcular tamaño.

#### 9. Decorator

- [x] Documentación del patrón.
- [x] Ejemplo: cliente HTTP con retry, timeout y logging.
- [x] Ejemplo: repositorio con caché encima de almacenamiento base.
- [x] Ejemplo: pipeline de validaciones sobre una orden.

#### 10. Facade

- [x] Documentación del patrón.
- [x] Ejemplo: servicio de checkout que coordina carrito, pago e inventario.
- [x] Ejemplo: API simple para enviar notificaciones multicanal.
- [x] Ejemplo: generador de reporte que oculta carga, cálculo y render.

#### 11. Flyweight

- [x] Documentación del patrón.
- [x] Ejemplo: catálogo compartido de monedas o países.
- [x] Ejemplo: caché de metadatos de productos repetidos.
- [x] Ejemplo: reutilización de estilos en render de documentos.

#### 12. Proxy

- [x] Documentación del patrón.
- [x] Ejemplo: proxy con caché para API externa.
- [x] Ejemplo: proxy con autorización para repositorio.
- [x] Ejemplo: proxy lazy para cargar archivos grandes.

### Subfase 1.3: Patrones de comportamiento

#### 13. Chain of Responsibility

- [x] Documentación del patrón.
- [x] Ejemplo: pipeline de validación de requests.
- [x] Ejemplo: resolución de soporte por niveles.
- [x] Ejemplo: filtros de moderación de contenido.

#### 14. Command

- [x] Documentación del patrón.
- [x] Ejemplo: comandos de CLI para crear, actualizar y borrar usuarios.
- [x] Ejemplo: acciones reversibles para editar una orden.
- [x] Ejemplo: cola de trabajos con comandos serializables.

#### 15. Interpreter

- [x] Documentación del patrón.
- [x] Ejemplo: filtros simples de búsqueda `campo operador valor`.
- [x] Ejemplo: reglas de descuentos expresadas como árbol.
- [x] Ejemplo: mini lenguaje para permisos.

#### 16. Iterator

- [x] Documentación del patrón.
- [x] Ejemplo: paginación sobre resultados de API.
- [x] Ejemplo: iterador de lotes para procesamiento de registros.
- [x] Ejemplo: recorrido de árbol de categorías.

#### 17. Mediator

- [x] Documentación del patrón.
- [x] Ejemplo: coordinador de eventos entre formulario, validación y guardado.
- [x] Ejemplo: sala de chat que media usuarios.
- [x] Ejemplo: orquestador de módulos de checkout.

#### 18. Memento

- [x] Documentación del patrón.
- [x] Ejemplo: snapshots de configuración para rollback.
- [x] Ejemplo: historial de cambios de documento.
- [x] Ejemplo: restaurar estado de carrito.

#### 19. Observer

- [x] Documentación del patrón.
- [x] Ejemplo: eventos de dominio para orden creada.
- [x] Ejemplo: suscriptores de métricas y logs.
- [x] Ejemplo: notificaciones al cambiar estado de inventario.

#### 20. State

- [x] Documentación del patrón.
- [x] Ejemplo: flujo de orden pendiente, pagada, enviada y cancelada.
- [x] Ejemplo: máquina de estados de autenticación.
- [x] Ejemplo: ciclo de vida de ticket de soporte.

#### 21. Strategy

- [x] Documentación del patrón.
- [x] Ejemplo: estrategias de descuento.
- [x] Ejemplo: estrategias de ordenamiento de resultados.
- [x] Ejemplo: estrategias de cálculo de envío.

#### 22. Template Method

- [x] Documentación del patrón.
- [x] Ejemplo: flujo común para importar archivos con pasos variables.
- [x] Ejemplo: generación de reportes con secciones personalizadas.
- [x] Ejemplo: proceso de onboarding con hooks por tipo de cuenta.

#### 23. Visitor

- [x] Documentación del patrón.
- [x] Ejemplo: exportar un árbol de expresiones a texto y JSON.
- [x] Ejemplo: calcular totales recorriendo elementos de factura.
- [x] Ejemplo: validar nodos de un workflow.

## Fase 2: Patrones idiomáticos de Rust

Esta fase complementa la base GoF. No reemplaza los patrones clásicos; sirve para explicar soluciones que aparecen mucho en Rust real y que muchas veces son mejores que portar un patrón OOP de forma literal.

### 24. Newtype

- [x] Documentación del patrón.
- [x] Ejemplo: IDs tipados para usuario, orden y producto.
- [x] Ejemplo: dinero y moneda sin mezclar unidades.
- [x] Ejemplo: tipos seguros para email y token.

### 25. Typestate

- [x] Documentación del patrón.
- [x] Ejemplo: request builder que no permite enviar sin URL.
- [x] Ejemplo: orden que solo puede enviarse después de pagarse.
- [x] Ejemplo: conexión que solo ejecuta consultas después de autenticarse.

### 26. RAII

- [x] Documentación del patrón.
- [x] Ejemplo: lock guard para secciones críticas.
- [x] Ejemplo: transacción que hace rollback si no se confirma.
- [x] Ejemplo: archivo temporal que se limpia al salir de scope.

### 27. Extension Trait

- [x] Documentación del patrón.
- [x] Ejemplo: helpers de strings para normalizar entradas.
- [x] Ejemplo: helpers de `Result` para mapear errores de dominio.
- [x] Ejemplo: helpers de colecciones para paginar resultados.

### 28. Iterator Adapters

- [x] Documentación del patrón.
- [x] Ejemplo: pipeline de filtrado y transformación de órdenes.
- [x] Ejemplo: procesamiento por lotes de registros.
- [x] Ejemplo: agregaciones de reportes sin estructuras intermedias.

### 29. Error Handling con Result

- [x] Documentación del patrón.
- [x] Ejemplo: errores de dominio para checkout.
- [x] Ejemplo: conversión de errores de infraestructura a errores de aplicación.
- [x] Ejemplo: validación acumulada y validación fail-fast.

### 30. Interior Mutability

- [x] Documentación del patrón.
- [x] Ejemplo: caché en memoria con `RefCell`.
- [x] Ejemplo: contador compartido con `Mutex`.
- [x] Ejemplo: lectura concurrente con `RwLock`.

### 31. Message Passing

- [x] Documentación del patrón.
- [x] Ejemplo: worker que procesa jobs desde un canal.
- [x] Ejemplo: fan-out de eventos a consumidores.
- [x] Ejemplo: pipeline de etapas con canales.

### 32. Actor-like Workers

- [x] Documentación del patrón.
- [x] Ejemplo: actor de email que recibe comandos.
- [x] Ejemplo: actor de inventario que serializa cambios.
- [x] Ejemplo: actor de métricas que agrega eventos.

## Fase 3: Patrones de arquitectura

Esta fase conectará los patrones anteriores con diseño de sistemas. El objetivo es mostrar estructuras de aplicación completas, no solo ejemplos aislados.

### 33. Layered Architecture

- [x] Documentación del patrón.
- [x] Ejemplo: API de usuarios con capas de presentación, aplicación y dominio.
- [x] Ejemplo: separación entre DTOs, entidades y repositorios.
- [x] Ejemplo: pruebas por capa con dependencias reemplazables.

### 34. Hexagonal Architecture

- [x] Documentación del patrón.
- [x] Ejemplo: caso de uso de checkout con puertos y adaptadores.
- [x] Ejemplo: repositorio en memoria y repositorio simulado externo.
- [x] Ejemplo: adaptador de notificaciones intercambiable.

### 35. Clean Architecture

- [x] Documentación del patrón.
- [x] Ejemplo: entidades, casos de uso, gateways y controladores.
- [x] Ejemplo: reglas de negocio independientes de framework.
- [x] Ejemplo: presenter para respuesta HTTP y respuesta CLI.

### 36. Domain-Driven Design Táctico

- [x] Documentación del patrón.
- [x] Ejemplo: agregados y value objects para órdenes.
- [x] Ejemplo: servicios de dominio para políticas de descuento.
- [x] Ejemplo: eventos de dominio para integración interna.

### 37. CQRS

- [x] Documentación del patrón.
- [x] Ejemplo: comandos separados de consultas para inventario.
- [x] Ejemplo: modelo de lectura optimizado para dashboard.
- [x] Ejemplo: sincronización simple entre escritura y lectura.

### 38. Event Sourcing

- [x] Documentación del patrón.
- [x] Ejemplo: cuenta bancaria reconstruida desde eventos.
- [x] Ejemplo: auditoría de cambios de orden.
- [x] Ejemplo: snapshots para acelerar reconstrucción.

### 39. Repository and Unit of Work

- [x] Documentación del patrón.
- [x] Ejemplo: repositorio en memoria para pruebas.
- [x] Ejemplo: unidad de trabajo para confirmar varios cambios.
- [x] Ejemplo: transacción simulada con rollback.

### 40. Service Layer

- [x] Documentación del patrón.
- [x] Ejemplo: servicio de aplicación para registrar usuario.
- [x] Ejemplo: servicio de checkout que coordina repositorios y políticas.
- [x] Ejemplo: servicio de reportes para consultas complejas.

### 41. Pipeline Architecture

- [x] Documentación del patrón.
- [x] Ejemplo: pipeline ETL para importar CSV.
- [x] Ejemplo: pipeline de validación y enriquecimiento de eventos.
- [x] Ejemplo: pipeline de procesamiento de imágenes simulado.

### 42. Plugin Architecture

- [x] Documentación del patrón.
- [x] Ejemplo: plugins de exportación JSON, CSV y texto.
- [x] Ejemplo: registro de estrategias cargadas por configuración.
- [x] Ejemplo: extensiones internas mediante traits y trait objects.

## Checklist de commits

- [x] Commit: plan inicial del catálogo.
- [x] Commit: reorganización del catálogo en fases GoF, Rust idiomático y arquitectura.
- [x] Commit: base del crate Rust.
- [x] Commit: README principal.
- [x] Commit individual por cada ejemplo de Abstract Factory.
- [x] Commit individual por cada ejemplo de Builder.
- [x] Commit individual por cada ejemplo de Factory Method.
- [x] Commit individual por cada ejemplo de Prototype.
- [x] Commit individual por cada ejemplo de Singleton.
- [x] Commit individual por cada ejemplo de Adapter.
- [x] Commit individual por cada ejemplo de Bridge.
- [x] Commit individual por cada ejemplo de Composite.
- [x] Commit individual por cada ejemplo de Decorator.
- [x] Commit individual por cada ejemplo de Facade.
- [x] Commit individual por cada ejemplo de Flyweight.
- [x] Commit individual por cada ejemplo de Proxy.
- [x] Commit individual por cada ejemplo de Chain of Responsibility.
- [x] Commit individual por cada ejemplo de Command.
- [x] Commit individual por cada ejemplo de Interpreter.
- [x] Commit individual por cada ejemplo de Iterator.
- [x] Commit individual por cada ejemplo de Mediator.
- [x] Commit individual por cada ejemplo de Memento.
- [x] Commit individual por cada ejemplo de Observer.
- [x] Commit individual por cada ejemplo de State.
- [x] Commit individual por cada ejemplo de Strategy.
- [x] Commit individual por cada ejemplo de Template Method.
- [x] Commit individual por cada ejemplo de Visitor.
- [x] Commit individual por cada ejemplo de Newtype.
- [x] Commit individual por cada ejemplo de Typestate.
- [x] Commit individual por cada ejemplo de RAII.
- [x] Commit individual por cada ejemplo de Extension Trait.
- [x] Commit individual por cada ejemplo de Iterator Adapters.
- [x] Commit individual por cada ejemplo de Error Handling con Result.
- [x] Commit individual por cada ejemplo de Interior Mutability.
- [x] Commit individual por cada ejemplo de Message Passing.
- [x] Commit individual por cada ejemplo de Actor-like Workers.
- [x] Commit individual por cada ejemplo de Layered Architecture.
- [x] Commit individual por cada ejemplo de Hexagonal Architecture.
- [x] Commit individual por cada ejemplo de Clean Architecture.
- [x] Commit individual por cada ejemplo de Domain-Driven Design Táctico.
- [x] Commit individual por cada ejemplo de CQRS.
- [x] Commit individual por cada ejemplo de Event Sourcing.
- [x] Commit individual por cada ejemplo de Repository and Unit of Work.
- [x] Commit individual por cada ejemplo de Service Layer.
- [x] Commit individual por cada ejemplo de Pipeline Architecture.
- [x] Commit individual por cada ejemplo de Plugin Architecture.

## Siguiente paso inmediato

Continuar con la Fase 4 de sistemas distribuidos y resiliencia, siguiendo con Bulkhead y manteniendo el ritmo de documentación, pruebas y commits individuales por ejemplo.

## Fase 4: Sistemas distribuidos y resiliencia

Esta fase agrega patrones operativos para servicios reales. El objetivo es mostrar cómo responder a fallas parciales, latencia, duplicados, límites de capacidad y coordinación entre procesos con ejemplos deterministas en Rust.

### 43. Retry with Backoff

- [x] Documentación del patrón.
- [x] Ejemplo: cliente HTTP simulado con backoff exponencial.
- [x] Ejemplo: reintentos solo para errores transitorios.
- [x] Ejemplo: jitter determinista para evitar reintentos sincronizados.

### 44. Circuit Breaker

- [x] Documentación del patrón.
- [x] Ejemplo: abrir circuito después de fallas consecutivas.
- [x] Ejemplo: estado half-open para probar recuperación.
- [x] Ejemplo: métricas de rechazos por circuito abierto.

### 45. Bulkhead

- [x] Documentación del patrón.
- [x] Ejemplo: pools separados para proveedores externos.
- [x] Ejemplo: límite de concurrencia simulado por recurso.
- [ ] Ejemplo: aislamiento de fallas entre operaciones críticas y no críticas.

### 46. Rate Limiting

- [ ] Documentación del patrón.
- [ ] Ejemplo: token bucket determinista.
- [ ] Ejemplo: límite por usuario o API key.
- [ ] Ejemplo: respuesta con tiempo sugerido para reintento.

### 47. Idempotency Key

- [ ] Documentación del patrón.
- [ ] Ejemplo: pagos protegidos contra doble envío.
- [ ] Ejemplo: caché de respuestas por llave de idempotencia.
- [ ] Ejemplo: conflicto cuando la misma llave trae payload distinto.

### 48. Outbox Pattern

- [ ] Documentación del patrón.
- [ ] Ejemplo: guardar entidad y evento en la misma unidad de trabajo.
- [ ] Ejemplo: publicador que marca mensajes como enviados.
- [ ] Ejemplo: reintento de mensajes pendientes.

### 49. Saga / Process Manager

- [ ] Documentación del patrón.
- [ ] Ejemplo: reserva, pago y envío coordinados por pasos.
- [ ] Ejemplo: compensación cuando falla un paso intermedio.
- [ ] Ejemplo: estado persistente del proceso.

### 50. Health Checks y Readiness

- [ ] Documentación del patrón.
- [ ] Ejemplo: chequeos de dependencias críticas.
- [ ] Ejemplo: readiness separado de liveness.
- [ ] Ejemplo: reporte agregado para orquestadores.

### 51. Cache Aside

- [ ] Documentación del patrón.
- [ ] Ejemplo: leer de caché o cargar del repositorio.
- [ ] Ejemplo: invalidación al actualizar datos.
- [ ] Ejemplo: TTL simulado con reloj determinista.

### 52. Leader Election simulado

- [ ] Documentación del patrón.
- [ ] Ejemplo: elegir líder por prioridad.
- [ ] Ejemplo: failover cuando el líder deja de responder.
- [ ] Ejemplo: evitar dos líderes activos en la misma ronda.

## Checklist de commits de Fase 4

- [x] Commit: base documental de sistemas distribuidos y resiliencia.
- [x] Commit individual por cada ejemplo de Retry with Backoff.
- [x] Commit individual por cada ejemplo de Circuit Breaker.
- [ ] Commit individual por cada ejemplo de Bulkhead.
- [ ] Commit individual por cada ejemplo de Rate Limiting.
- [ ] Commit individual por cada ejemplo de Idempotency Key.
- [ ] Commit individual por cada ejemplo de Outbox Pattern.
- [ ] Commit individual por cada ejemplo de Saga / Process Manager.
- [ ] Commit individual por cada ejemplo de Health Checks y Readiness.
- [ ] Commit individual por cada ejemplo de Cache Aside.
- [ ] Commit individual por cada ejemplo de Leader Election simulado.
