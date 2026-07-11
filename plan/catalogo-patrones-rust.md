# Catálogo de Patrones de Diseño en Rust

Este documento es el tablero de progreso del proyecto. La meta es construir un catálogo práctico de patrones usando Rust en tres niveles: patrones GoF, patrones idiomáticos de Rust y patrones de arquitectura para sistemas reales. Cada nivel debe tener documentación clara, ejemplos ejecutables y commits pequeños.

## Reglas de trabajo

- [ ] Mantener este archivo actualizado en cada avance.
- [ ] Crear un README principal con índice, categorías y guía de ejecución.
- [ ] Usar Rust idiomático antes que traducciones literales de OOP.
- [ ] Incluir ejemplos cercanos al día a día de sistemas: configuración, pagos, notificaciones, caché, logs, APIs, colas, reportes, permisos y procesamiento de datos.
- [ ] Cada patrón debe tener su propia documentación.
- [ ] Cada ejemplo debe compilar y tener pruebas o asserts ejecutables.
- [ ] Cada ejemplo práctico debe ir en un commit individual.
- [ ] Cuando un patrón tenga varios ejemplos, cada ejemplo se commitea por separado.
- [ ] Los commits deben ser descriptivos, por ejemplo: `feat: add factory method payment example`.

## Estructura propuesta

```text
.
├── Cargo.toml
├── README.md
├── plan/
│   └── catalogo-patrones-rust.md
└── patterns/
    ├── gof/
    │   ├── creational/
    │   ├── structural/
    │   └── behavioral/
    ├── rust_idiomatic/
    └── architecture/
```

Cada patrón seguirá esta forma:

```text
patterns/<familia>/<categoria>/<patron>/
├── README.md
├── mod.rs
└── examples/
    ├── <ejemplo_1>.rs
    └── <ejemplo_2>.rs
```

La estructura exacta puede ajustarse cuando creemos el crate, pero el objetivo se mantiene: cada patrón queda aislado, documentado y fácil de ejecutar.

## Formato de documentación por patrón

- [ ] Intención del patrón.
- [ ] Problema cotidiano que resuelve.
- [ ] Cuándo usarlo en Rust.
- [ ] Cuándo evitarlo en Rust.
- [ ] Diferencias entre el patrón clásico y una versión rústica.
- [ ] Diagrama simple si ayuda a entender la relación entre piezas.
- [ ] Lista de ejemplos incluidos.
- [ ] Comando para ejecutar pruebas o ejemplo.

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
- [ ] Ejemplo: lifecycle de ticket de soporte.

#### 21. Strategy

- [ ] Documentación del patrón.
- [ ] Ejemplo: estrategias de descuento.
- [ ] Ejemplo: estrategias de ordenamiento de resultados.
- [ ] Ejemplo: estrategias de cálculo de envío.

#### 22. Template Method

- [ ] Documentación del patrón.
- [ ] Ejemplo: flujo común para importar archivos con pasos variables.
- [ ] Ejemplo: generación de reportes con secciones personalizadas.
- [ ] Ejemplo: proceso de onboarding con hooks por tipo de cuenta.

#### 23. Visitor

- [ ] Documentación del patrón.
- [ ] Ejemplo: exportar un árbol de expresiones a texto y JSON.
- [ ] Ejemplo: calcular totales recorriendo elementos de factura.
- [ ] Ejemplo: validar nodos de un workflow.

## Fase 2: Patrones idiomáticos de Rust

Esta fase se trabajará después de completar la base GoF. No reemplaza los patrones clásicos; sirve para explicar soluciones que aparecen mucho en Rust real y que muchas veces son mejores que portar un patrón OOP de forma literal.

### 24. Newtype

- [ ] Documentación del patrón.
- [ ] Ejemplo: IDs tipados para usuario, orden y producto.
- [ ] Ejemplo: dinero y moneda sin mezclar unidades.
- [ ] Ejemplo: tipos seguros para email y token.

### 25. Typestate

- [ ] Documentación del patrón.
- [ ] Ejemplo: request builder que no permite enviar sin URL.
- [ ] Ejemplo: orden que solo puede enviarse después de pagarse.
- [ ] Ejemplo: conexión que solo ejecuta consultas después de autenticarse.

### 26. RAII

- [ ] Documentación del patrón.
- [ ] Ejemplo: lock guard para secciones críticas.
- [ ] Ejemplo: transacción que hace rollback si no se confirma.
- [ ] Ejemplo: archivo temporal que se limpia al salir de scope.

### 27. Extension Trait

- [ ] Documentación del patrón.
- [ ] Ejemplo: helpers de strings para normalizar entradas.
- [ ] Ejemplo: helpers de `Result` para mapear errores de dominio.
- [ ] Ejemplo: helpers de colecciones para paginar resultados.

### 28. Iterator Adapters

- [ ] Documentación del patrón.
- [ ] Ejemplo: pipeline de filtrado y transformación de órdenes.
- [ ] Ejemplo: procesamiento por lotes de registros.
- [ ] Ejemplo: agregaciones de reportes sin estructuras intermedias.

### 29. Error Handling con Result

- [ ] Documentación del patrón.
- [ ] Ejemplo: errores de dominio para checkout.
- [ ] Ejemplo: conversión de errores de infraestructura a errores de aplicación.
- [ ] Ejemplo: validación acumulada y validación fail-fast.

### 30. Interior Mutability

- [ ] Documentación del patrón.
- [ ] Ejemplo: caché en memoria con `RefCell`.
- [ ] Ejemplo: contador compartido con `Mutex`.
- [ ] Ejemplo: lectura concurrente con `RwLock`.

### 31. Message Passing

- [ ] Documentación del patrón.
- [ ] Ejemplo: worker que procesa jobs desde un canal.
- [ ] Ejemplo: fan-out de eventos a consumidores.
- [ ] Ejemplo: pipeline de etapas con canales.

### 32. Actor-like Workers

- [ ] Documentación del patrón.
- [ ] Ejemplo: actor de email que recibe comandos.
- [ ] Ejemplo: actor de inventario que serializa cambios.
- [ ] Ejemplo: actor de métricas que agrega eventos.

## Fase 3: Patrones de arquitectura

Esta fase conectará los patrones anteriores con diseño de sistemas. El objetivo es mostrar estructuras de aplicación completas, no solo ejemplos aislados.

### 33. Layered Architecture

- [ ] Documentación del patrón.
- [ ] Ejemplo: API de usuarios con capas de presentación, aplicación y dominio.
- [ ] Ejemplo: separación entre DTOs, entidades y repositorios.
- [ ] Ejemplo: pruebas por capa con dependencias reemplazables.

### 34. Hexagonal Architecture

- [ ] Documentación del patrón.
- [ ] Ejemplo: caso de uso de checkout con puertos y adaptadores.
- [ ] Ejemplo: repositorio en memoria y repositorio simulado externo.
- [ ] Ejemplo: adaptador de notificaciones intercambiable.

### 35. Clean Architecture

- [ ] Documentación del patrón.
- [ ] Ejemplo: entidades, casos de uso, gateways y controladores.
- [ ] Ejemplo: reglas de negocio independientes de framework.
- [ ] Ejemplo: presenter para respuesta HTTP y respuesta CLI.

### 36. Domain-Driven Design Táctico

- [ ] Documentación del patrón.
- [ ] Ejemplo: agregados y value objects para órdenes.
- [ ] Ejemplo: servicios de dominio para políticas de descuento.
- [ ] Ejemplo: eventos de dominio para integración interna.

### 37. CQRS

- [ ] Documentación del patrón.
- [ ] Ejemplo: comandos separados de consultas para inventario.
- [ ] Ejemplo: modelo de lectura optimizado para dashboard.
- [ ] Ejemplo: sincronización simple entre escritura y lectura.

### 38. Event Sourcing

- [ ] Documentación del patrón.
- [ ] Ejemplo: cuenta bancaria reconstruida desde eventos.
- [ ] Ejemplo: auditoría de cambios de orden.
- [ ] Ejemplo: snapshots para acelerar reconstrucción.

### 39. Repository and Unit of Work

- [ ] Documentación del patrón.
- [ ] Ejemplo: repositorio en memoria para pruebas.
- [ ] Ejemplo: unidad de trabajo para confirmar varios cambios.
- [ ] Ejemplo: transacción simulada con rollback.

### 40. Service Layer

- [ ] Documentación del patrón.
- [ ] Ejemplo: servicio de aplicación para registrar usuario.
- [ ] Ejemplo: servicio de checkout que coordina repositorios y políticas.
- [ ] Ejemplo: servicio de reportes para consultas complejas.

### 41. Pipeline Architecture

- [ ] Documentación del patrón.
- [ ] Ejemplo: pipeline ETL para importar CSV.
- [ ] Ejemplo: pipeline de validación y enriquecimiento de eventos.
- [ ] Ejemplo: pipeline de procesamiento de imágenes simulado.

### 42. Plugin Architecture

- [ ] Documentación del patrón.
- [ ] Ejemplo: plugins de exportación JSON, CSV y texto.
- [ ] Ejemplo: registro de estrategias cargadas por configuración.
- [ ] Ejemplo: extensiones internas mediante traits y trait objects.

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
- [ ] Commit individual por cada ejemplo de Flyweight.
- [ ] Commit individual por cada ejemplo de Proxy.
- [ ] Commit individual por cada ejemplo de Chain of Responsibility.
- [ ] Commit individual por cada ejemplo de Command.
- [ ] Commit individual por cada ejemplo de Interpreter.
- [ ] Commit individual por cada ejemplo de Iterator.
- [ ] Commit individual por cada ejemplo de Mediator.
- [ ] Commit individual por cada ejemplo de Memento.
- [ ] Commit individual por cada ejemplo de Observer.
- [ ] Commit individual por cada ejemplo de State.
- [ ] Commit individual por cada ejemplo de Strategy.
- [ ] Commit individual por cada ejemplo de Template Method.
- [ ] Commit individual por cada ejemplo de Visitor.
- [ ] Commit individual por cada ejemplo de Newtype.
- [ ] Commit individual por cada ejemplo de Typestate.
- [ ] Commit individual por cada ejemplo de RAII.
- [ ] Commit individual por cada ejemplo de Extension Trait.
- [ ] Commit individual por cada ejemplo de Iterator Adapters.
- [ ] Commit individual por cada ejemplo de Error Handling con Result.
- [ ] Commit individual por cada ejemplo de Interior Mutability.
- [ ] Commit individual por cada ejemplo de Message Passing.
- [ ] Commit individual por cada ejemplo de Actor-like Workers.
- [ ] Commit individual por cada ejemplo de Layered Architecture.
- [ ] Commit individual por cada ejemplo de Hexagonal Architecture.
- [ ] Commit individual por cada ejemplo de Clean Architecture.
- [ ] Commit individual por cada ejemplo de Domain-Driven Design Táctico.
- [ ] Commit individual por cada ejemplo de CQRS.
- [ ] Commit individual por cada ejemplo de Event Sourcing.
- [ ] Commit individual por cada ejemplo de Repository and Unit of Work.
- [ ] Commit individual por cada ejemplo de Service Layer.
- [ ] Commit individual por cada ejemplo de Pipeline Architecture.
- [ ] Commit individual por cada ejemplo de Plugin Architecture.

## Siguiente paso inmediato

Continuar con la Fase 1 de patrones GoF, siguiendo con Flyweight y manteniendo el ritmo de documentación, pruebas y commits individuales por ejemplo.
