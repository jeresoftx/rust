# Catalogo de Patrones de Diseno en Rust

Este documento es el tablero de progreso del proyecto. La meta es construir un catalogo practico de patrones usando Rust en tres niveles: patrones GoF, patrones idiomaticos de Rust y patrones de arquitectura para sistemas reales. Cada nivel debe tener documentacion clara, ejemplos ejecutables y commits pequenos.

## Reglas de trabajo

- [ ] Mantener este archivo actualizado en cada avance.
- [ ] Crear un README principal con indice, categorias y guia de ejecucion.
- [ ] Usar Rust idiomatico antes que traducciones literales de OOP.
- [ ] Incluir ejemplos cercanos al dia a dia de sistemas: configuracion, pagos, notificaciones, cache, logs, APIs, colas, reportes, permisos y procesamiento de datos.
- [ ] Cada patron debe tener su propia documentacion.
- [ ] Cada ejemplo debe compilar y tener pruebas o asserts ejecutables.
- [ ] Cada ejemplo practico debe ir en un commit individual.
- [ ] Cuando un patron tenga varios ejemplos, cada ejemplo se commitea por separado.
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

Cada patron seguira esta forma:

```text
patterns/<familia>/<categoria>/<patron>/
├── README.md
├── mod.rs
└── examples/
    ├── <ejemplo_1>.rs
    └── <ejemplo_2>.rs
```

La estructura exacta puede ajustarse cuando creemos el crate, pero el objetivo se mantiene: cada patron queda aislado, documentado y facil de ejecutar.

## Formato de documentacion por patron

- [ ] Intencion del patron.
- [ ] Problema cotidiano que resuelve.
- [ ] Cuando usarlo en Rust.
- [ ] Cuando evitarlo en Rust.
- [ ] Diferencias entre el patron clasico y una version rustica.
- [ ] Diagrama simple si ayuda a entender la relacion entre piezas.
- [ ] Lista de ejemplos incluidos.
- [ ] Comando para ejecutar pruebas o ejemplo.

## Fase 0: Base del proyecto

- [x] Crear carpeta `plan/` y documento de progreso.
- [x] Crear crate Rust base con `Cargo.toml`.
- [x] Crear README principal del repositorio.
- [x] Crear wiki local versionada en `wiki/`.
- [x] Crear modulos base por familia: GoF, Rust idiomatico y arquitectura.
- [x] Definir convenciones de nombres, pruebas y comandos.
- [x] Verificar `cargo fmt`, `cargo check` y `cargo test`.
- [x] Commit de la base del proyecto.

## Fase 1: Patrones GoF

Primero construiremos los 23 patrones clasicos GoF, explicando como traducirlos a Rust sin forzar herencia o jerarquias innecesarias.

### Subfase 1.1: Patrones creacionales

#### 1. Abstract Factory

- [x] Documentacion del patron.
- [x] Ejemplo: familia de conectores para bases de datos SQL y NoSQL.
- [x] Ejemplo: componentes de UI para consola y web.
- [x] Ejemplo: clientes de proveedor de pagos para Stripe-like y PayPal-like.

#### 2. Builder

- [x] Documentacion del patron.
- [x] Ejemplo: construccion de configuracion de servidor HTTP.
- [x] Ejemplo: construccion de consultas de busqueda con filtros opcionales.
- [ ] Ejemplo: construccion de payload de email transaccional.

#### 3. Factory Method

- [ ] Documentacion del patron.
- [ ] Ejemplo: creacion de procesadores de archivos CSV, JSON y XML.
- [ ] Ejemplo: creacion de metodos de pago segun canal.
- [ ] Ejemplo: creacion de clientes HTTP por ambiente.

#### 4. Prototype

- [ ] Documentacion del patron.
- [ ] Ejemplo: clonar plantillas de reportes con ajustes por cliente.
- [ ] Ejemplo: duplicar configuraciones base de despliegue.
- [ ] Ejemplo: crear ordenes desde una plantilla preconfigurada.

#### 5. Singleton

- [ ] Documentacion del patron.
- [ ] Ejemplo: configuracion global de aplicacion con `OnceLock`.
- [ ] Ejemplo: registro centralizado de metricas.
- [ ] Ejemplo: pool compartido simulado para recursos costosos.

### Subfase 1.2: Patrones estructurales

#### 6. Adapter

- [ ] Documentacion del patron.
- [ ] Ejemplo: adaptar un cliente externo de pagos a una interfaz interna.
- [ ] Ejemplo: adaptar formatos legacy de usuario a un modelo nuevo.
- [ ] Ejemplo: adaptar logger de terceros a un trait propio.

#### 7. Bridge

- [ ] Documentacion del patron.
- [ ] Ejemplo: notificaciones desacopladas de canales email, SMS y push.
- [ ] Ejemplo: reportes desacoplados de renderizadores PDF, HTML y texto.
- [ ] Ejemplo: almacenamiento desacoplado de proveedores local y nube.

#### 8. Composite

- [ ] Documentacion del patron.
- [ ] Ejemplo: arbol de permisos por modulo y accion.
- [ ] Ejemplo: estructura de menu con submenus.
- [ ] Ejemplo: carpeta con archivos y subcarpetas para calcular tamano.

#### 9. Decorator

- [ ] Documentacion del patron.
- [ ] Ejemplo: cliente HTTP con retry, timeout y logging.
- [ ] Ejemplo: repositorio con cache encima de almacenamiento base.
- [ ] Ejemplo: pipeline de validaciones sobre una orden.

#### 10. Facade

- [ ] Documentacion del patron.
- [ ] Ejemplo: servicio de checkout que coordina carrito, pago e inventario.
- [ ] Ejemplo: API simple para enviar notificaciones multicanal.
- [ ] Ejemplo: generador de reporte que oculta carga, calculo y render.

#### 11. Flyweight

- [ ] Documentacion del patron.
- [ ] Ejemplo: catalogo compartido de monedas o paises.
- [ ] Ejemplo: cache de metadatos de productos repetidos.
- [ ] Ejemplo: reutilizacion de estilos en render de documentos.

#### 12. Proxy

- [ ] Documentacion del patron.
- [ ] Ejemplo: proxy con cache para API externa.
- [ ] Ejemplo: proxy con autorizacion para repositorio.
- [ ] Ejemplo: proxy lazy para cargar archivos grandes.

### Subfase 1.3: Patrones de comportamiento

#### 13. Chain of Responsibility

- [ ] Documentacion del patron.
- [ ] Ejemplo: pipeline de validacion de requests.
- [ ] Ejemplo: resolucion de soporte por niveles.
- [ ] Ejemplo: filtros de moderacion de contenido.

#### 14. Command

- [ ] Documentacion del patron.
- [ ] Ejemplo: comandos de CLI para crear, actualizar y borrar usuarios.
- [ ] Ejemplo: acciones reversibles para editar una orden.
- [ ] Ejemplo: cola de trabajos con comandos serializables.

#### 15. Interpreter

- [ ] Documentacion del patron.
- [ ] Ejemplo: filtros simples de busqueda `campo operador valor`.
- [ ] Ejemplo: reglas de descuentos expresadas como arbol.
- [ ] Ejemplo: mini lenguaje para permisos.

#### 16. Iterator

- [ ] Documentacion del patron.
- [ ] Ejemplo: paginacion sobre resultados de API.
- [ ] Ejemplo: iterador de lotes para procesamiento de registros.
- [ ] Ejemplo: recorrido de arbol de categorias.

#### 17. Mediator

- [ ] Documentacion del patron.
- [ ] Ejemplo: coordinador de eventos entre formulario, validacion y guardado.
- [ ] Ejemplo: sala de chat que media usuarios.
- [ ] Ejemplo: orquestador de modulos de checkout.

#### 18. Memento

- [ ] Documentacion del patron.
- [ ] Ejemplo: snapshots de configuracion para rollback.
- [ ] Ejemplo: historial de cambios de documento.
- [ ] Ejemplo: restaurar estado de carrito.

#### 19. Observer

- [ ] Documentacion del patron.
- [ ] Ejemplo: eventos de dominio para orden creada.
- [ ] Ejemplo: suscriptores de metricas y logs.
- [ ] Ejemplo: notificaciones al cambiar estado de inventario.

#### 20. State

- [ ] Documentacion del patron.
- [ ] Ejemplo: flujo de orden pendiente, pagada, enviada y cancelada.
- [ ] Ejemplo: maquina de estados de autenticacion.
- [ ] Ejemplo: lifecycle de ticket de soporte.

#### 21. Strategy

- [ ] Documentacion del patron.
- [ ] Ejemplo: estrategias de descuento.
- [ ] Ejemplo: estrategias de ordenamiento de resultados.
- [ ] Ejemplo: estrategias de calculo de envio.

#### 22. Template Method

- [ ] Documentacion del patron.
- [ ] Ejemplo: flujo comun para importar archivos con pasos variables.
- [ ] Ejemplo: generacion de reportes con secciones personalizadas.
- [ ] Ejemplo: proceso de onboarding con hooks por tipo de cuenta.

#### 23. Visitor

- [ ] Documentacion del patron.
- [ ] Ejemplo: exportar un arbol de expresiones a texto y JSON.
- [ ] Ejemplo: calcular totales recorriendo elementos de factura.
- [ ] Ejemplo: validar nodos de un workflow.

## Fase 2: Patrones idiomaticos de Rust

Esta fase se trabajara despues de completar la base GoF. No reemplaza los patrones clasicos; sirve para explicar soluciones que aparecen mucho en Rust real y que muchas veces son mejores que portar un patron OOP de forma literal.

### 24. Newtype

- [ ] Documentacion del patron.
- [ ] Ejemplo: IDs tipados para usuario, orden y producto.
- [ ] Ejemplo: dinero y moneda sin mezclar unidades.
- [ ] Ejemplo: tipos seguros para email y token.

### 25. Typestate

- [ ] Documentacion del patron.
- [ ] Ejemplo: request builder que no permite enviar sin URL.
- [ ] Ejemplo: orden que solo puede enviarse despues de pagarse.
- [ ] Ejemplo: conexion que solo ejecuta consultas despues de autenticarse.

### 26. RAII

- [ ] Documentacion del patron.
- [ ] Ejemplo: lock guard para secciones criticas.
- [ ] Ejemplo: transaccion que hace rollback si no se confirma.
- [ ] Ejemplo: archivo temporal que se limpia al salir de scope.

### 27. Extension Trait

- [ ] Documentacion del patron.
- [ ] Ejemplo: helpers de strings para normalizar entradas.
- [ ] Ejemplo: helpers de `Result` para mapear errores de dominio.
- [ ] Ejemplo: helpers de colecciones para paginar resultados.

### 28. Iterator Adapters

- [ ] Documentacion del patron.
- [ ] Ejemplo: pipeline de filtrado y transformacion de ordenes.
- [ ] Ejemplo: procesamiento por lotes de registros.
- [ ] Ejemplo: agregaciones de reportes sin estructuras intermedias.

### 29. Error Handling con Result

- [ ] Documentacion del patron.
- [ ] Ejemplo: errores de dominio para checkout.
- [ ] Ejemplo: conversion de errores de infraestructura a errores de aplicacion.
- [ ] Ejemplo: validacion acumulada y validacion fail-fast.

### 30. Interior Mutability

- [ ] Documentacion del patron.
- [ ] Ejemplo: cache en memoria con `RefCell`.
- [ ] Ejemplo: contador compartido con `Mutex`.
- [ ] Ejemplo: lectura concurrente con `RwLock`.

### 31. Message Passing

- [ ] Documentacion del patron.
- [ ] Ejemplo: worker que procesa jobs desde un canal.
- [ ] Ejemplo: fan-out de eventos a consumidores.
- [ ] Ejemplo: pipeline de etapas con canales.

### 32. Actor-like Workers

- [ ] Documentacion del patron.
- [ ] Ejemplo: actor de email que recibe comandos.
- [ ] Ejemplo: actor de inventario que serializa cambios.
- [ ] Ejemplo: actor de metricas que agrega eventos.

## Fase 3: Patrones de arquitectura

Esta fase conectara los patrones anteriores con diseno de sistemas. El objetivo es mostrar estructuras de aplicacion completas, no solo ejemplos aislados.

### 33. Layered Architecture

- [ ] Documentacion del patron.
- [ ] Ejemplo: API de usuarios con capas de presentacion, aplicacion y dominio.
- [ ] Ejemplo: separacion entre DTOs, entidades y repositorios.
- [ ] Ejemplo: pruebas por capa con dependencias reemplazables.

### 34. Hexagonal Architecture

- [ ] Documentacion del patron.
- [ ] Ejemplo: caso de uso de checkout con puertos y adaptadores.
- [ ] Ejemplo: repositorio en memoria y repositorio simulado externo.
- [ ] Ejemplo: adaptador de notificaciones intercambiable.

### 35. Clean Architecture

- [ ] Documentacion del patron.
- [ ] Ejemplo: entidades, casos de uso, gateways y controladores.
- [ ] Ejemplo: reglas de negocio independientes de framework.
- [ ] Ejemplo: presenter para respuesta HTTP y respuesta CLI.

### 36. Domain-Driven Design Tactico

- [ ] Documentacion del patron.
- [ ] Ejemplo: agregados y value objects para ordenes.
- [ ] Ejemplo: servicios de dominio para politicas de descuento.
- [ ] Ejemplo: eventos de dominio para integracion interna.

### 37. CQRS

- [ ] Documentacion del patron.
- [ ] Ejemplo: comandos separados de consultas para inventario.
- [ ] Ejemplo: modelo de lectura optimizado para dashboard.
- [ ] Ejemplo: sincronizacion simple entre escritura y lectura.

### 38. Event Sourcing

- [ ] Documentacion del patron.
- [ ] Ejemplo: cuenta bancaria reconstruida desde eventos.
- [ ] Ejemplo: auditoria de cambios de orden.
- [ ] Ejemplo: snapshots para acelerar reconstruccion.

### 39. Repository and Unit of Work

- [ ] Documentacion del patron.
- [ ] Ejemplo: repositorio en memoria para pruebas.
- [ ] Ejemplo: unidad de trabajo para confirmar varios cambios.
- [ ] Ejemplo: transaccion simulada con rollback.

### 40. Service Layer

- [ ] Documentacion del patron.
- [ ] Ejemplo: servicio de aplicacion para registrar usuario.
- [ ] Ejemplo: servicio de checkout que coordina repositorios y politicas.
- [ ] Ejemplo: servicio de reportes para consultas complejas.

### 41. Pipeline Architecture

- [ ] Documentacion del patron.
- [ ] Ejemplo: pipeline ETL para importar CSV.
- [ ] Ejemplo: pipeline de validacion y enriquecimiento de eventos.
- [ ] Ejemplo: pipeline de procesamiento de imagenes simulado.

### 42. Plugin Architecture

- [ ] Documentacion del patron.
- [ ] Ejemplo: plugins de exportacion JSON, CSV y texto.
- [ ] Ejemplo: registro de estrategias cargadas por configuracion.
- [ ] Ejemplo: extensiones internas mediante traits y trait objects.

## Checklist de commits

- [x] Commit: plan inicial del catalogo.
- [x] Commit: reorganizacion del catalogo en fases GoF, Rust idiomatico y arquitectura.
- [x] Commit: base del crate Rust.
- [x] Commit: README principal.
- [x] Commit individual por cada ejemplo de Abstract Factory.
- [ ] Commit individual por cada ejemplo de Builder.
- [ ] Commit individual por cada ejemplo de Factory Method.
- [ ] Commit individual por cada ejemplo de Prototype.
- [ ] Commit individual por cada ejemplo de Singleton.
- [ ] Commit individual por cada ejemplo de Adapter.
- [ ] Commit individual por cada ejemplo de Bridge.
- [ ] Commit individual por cada ejemplo de Composite.
- [ ] Commit individual por cada ejemplo de Decorator.
- [ ] Commit individual por cada ejemplo de Facade.
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
- [ ] Commit individual por cada ejemplo de Domain-Driven Design Tactico.
- [ ] Commit individual por cada ejemplo de CQRS.
- [ ] Commit individual por cada ejemplo de Event Sourcing.
- [ ] Commit individual por cada ejemplo de Repository and Unit of Work.
- [ ] Commit individual por cada ejemplo de Service Layer.
- [ ] Commit individual por cada ejemplo de Pipeline Architecture.
- [ ] Commit individual por cada ejemplo de Plugin Architecture.

## Siguiente paso inmediato

Crear la base del crate Rust y el README principal. Despues de eso, comenzaremos con la Fase 1 de patrones GoF, uno por uno, iniciando con Abstract Factory.
