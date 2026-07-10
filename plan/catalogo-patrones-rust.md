# Catalogo de Patrones de Diseno en Rust

Este documento es el tablero de progreso del proyecto. La primera meta es construir un catalogo practico de los 23 patrones GoF usando Rust, con documentacion clara, ejemplos ejecutables y commits pequenos.

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
    ├── creational/
    ├── structural/
    └── behavioral/
```

Cada patron seguira esta forma:

```text
patterns/<categoria>/<patron>/
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
- [ ] Crear crate Rust base con `Cargo.toml`.
- [ ] Crear README principal del repositorio.
- [ ] Crear modulos base por categoria: creacionales, estructurales y comportamiento.
- [ ] Definir convenciones de nombres, pruebas y comandos.
- [ ] Verificar `cargo fmt`, `cargo check` y `cargo test`.
- [ ] Commit de la base del proyecto.

## Fase 1: Patrones creacionales

### 1. Abstract Factory

- [ ] Documentacion del patron.
- [ ] Ejemplo: familia de conectores para bases de datos SQL y NoSQL.
- [ ] Ejemplo: componentes de UI para consola y web.
- [ ] Ejemplo: clientes de proveedor de pagos para Stripe-like y PayPal-like.

### 2. Builder

- [ ] Documentacion del patron.
- [ ] Ejemplo: construccion de configuracion de servidor HTTP.
- [ ] Ejemplo: construccion de consultas de busqueda con filtros opcionales.
- [ ] Ejemplo: construccion de payload de email transaccional.

### 3. Factory Method

- [ ] Documentacion del patron.
- [ ] Ejemplo: creacion de procesadores de archivos CSV, JSON y XML.
- [ ] Ejemplo: creacion de metodos de pago segun canal.
- [ ] Ejemplo: creacion de clientes HTTP por ambiente.

### 4. Prototype

- [ ] Documentacion del patron.
- [ ] Ejemplo: clonar plantillas de reportes con ajustes por cliente.
- [ ] Ejemplo: duplicar configuraciones base de despliegue.
- [ ] Ejemplo: crear ordenes desde una plantilla preconfigurada.

### 5. Singleton

- [ ] Documentacion del patron.
- [ ] Ejemplo: configuracion global de aplicacion con `OnceLock`.
- [ ] Ejemplo: registro centralizado de metricas.
- [ ] Ejemplo: pool compartido simulado para recursos costosos.

## Fase 2: Patrones estructurales

### 6. Adapter

- [ ] Documentacion del patron.
- [ ] Ejemplo: adaptar un cliente externo de pagos a una interfaz interna.
- [ ] Ejemplo: adaptar formatos legacy de usuario a un modelo nuevo.
- [ ] Ejemplo: adaptar logger de terceros a un trait propio.

### 7. Bridge

- [ ] Documentacion del patron.
- [ ] Ejemplo: notificaciones desacopladas de canales email, SMS y push.
- [ ] Ejemplo: reportes desacoplados de renderizadores PDF, HTML y texto.
- [ ] Ejemplo: almacenamiento desacoplado de proveedores local y nube.

### 8. Composite

- [ ] Documentacion del patron.
- [ ] Ejemplo: arbol de permisos por modulo y accion.
- [ ] Ejemplo: estructura de menu con submenus.
- [ ] Ejemplo: carpeta con archivos y subcarpetas para calcular tamano.

### 9. Decorator

- [ ] Documentacion del patron.
- [ ] Ejemplo: cliente HTTP con retry, timeout y logging.
- [ ] Ejemplo: repositorio con cache encima de almacenamiento base.
- [ ] Ejemplo: pipeline de validaciones sobre una orden.

### 10. Facade

- [ ] Documentacion del patron.
- [ ] Ejemplo: servicio de checkout que coordina carrito, pago e inventario.
- [ ] Ejemplo: API simple para enviar notificaciones multicanal.
- [ ] Ejemplo: generador de reporte que oculta carga, calculo y render.

### 11. Flyweight

- [ ] Documentacion del patron.
- [ ] Ejemplo: catalogo compartido de monedas o paises.
- [ ] Ejemplo: cache de metadatos de productos repetidos.
- [ ] Ejemplo: reutilizacion de estilos en render de documentos.

### 12. Proxy

- [ ] Documentacion del patron.
- [ ] Ejemplo: proxy con cache para API externa.
- [ ] Ejemplo: proxy con autorizacion para repositorio.
- [ ] Ejemplo: proxy lazy para cargar archivos grandes.

## Fase 3: Patrones de comportamiento

### 13. Chain of Responsibility

- [ ] Documentacion del patron.
- [ ] Ejemplo: pipeline de validacion de requests.
- [ ] Ejemplo: resolucion de soporte por niveles.
- [ ] Ejemplo: filtros de moderacion de contenido.

### 14. Command

- [ ] Documentacion del patron.
- [ ] Ejemplo: comandos de CLI para crear, actualizar y borrar usuarios.
- [ ] Ejemplo: acciones reversibles para editar una orden.
- [ ] Ejemplo: cola de trabajos con comandos serializables.

### 15. Interpreter

- [ ] Documentacion del patron.
- [ ] Ejemplo: filtros simples de busqueda `campo operador valor`.
- [ ] Ejemplo: reglas de descuentos expresadas como arbol.
- [ ] Ejemplo: mini lenguaje para permisos.

### 16. Iterator

- [ ] Documentacion del patron.
- [ ] Ejemplo: paginacion sobre resultados de API.
- [ ] Ejemplo: iterador de lotes para procesamiento de registros.
- [ ] Ejemplo: recorrido de arbol de categorias.

### 17. Mediator

- [ ] Documentacion del patron.
- [ ] Ejemplo: coordinador de eventos entre formulario, validacion y guardado.
- [ ] Ejemplo: sala de chat que media usuarios.
- [ ] Ejemplo: orquestador de modulos de checkout.

### 18. Memento

- [ ] Documentacion del patron.
- [ ] Ejemplo: snapshots de configuracion para rollback.
- [ ] Ejemplo: historial de cambios de documento.
- [ ] Ejemplo: restaurar estado de carrito.

### 19. Observer

- [ ] Documentacion del patron.
- [ ] Ejemplo: eventos de dominio para orden creada.
- [ ] Ejemplo: suscriptores de metricas y logs.
- [ ] Ejemplo: notificaciones al cambiar estado de inventario.

### 20. State

- [ ] Documentacion del patron.
- [ ] Ejemplo: flujo de orden pendiente, pagada, enviada y cancelada.
- [ ] Ejemplo: maquina de estados de autenticacion.
- [ ] Ejemplo: lifecycle de ticket de soporte.

### 21. Strategy

- [ ] Documentacion del patron.
- [ ] Ejemplo: estrategias de descuento.
- [ ] Ejemplo: estrategias de ordenamiento de resultados.
- [ ] Ejemplo: estrategias de calculo de envio.

### 22. Template Method

- [ ] Documentacion del patron.
- [ ] Ejemplo: flujo comun para importar archivos con pasos variables.
- [ ] Ejemplo: generacion de reportes con secciones personalizadas.
- [ ] Ejemplo: proceso de onboarding con hooks por tipo de cuenta.

### 23. Visitor

- [ ] Documentacion del patron.
- [ ] Ejemplo: exportar un arbol de expresiones a texto y JSON.
- [ ] Ejemplo: calcular totales recorriendo elementos de factura.
- [ ] Ejemplo: validar nodos de un workflow.

## Fase 4: Patrones idiomaticos de Rust

Esta fase se trabajara despues de completar la base GoF. No reemplaza los patrones clasicos; sirve para explicar soluciones que aparecen mucho en Rust real.

- [ ] Newtype.
- [ ] Typestate.
- [ ] RAII.
- [ ] Extension Trait.
- [ ] Iterator adapters.
- [ ] Error handling con `Result` y errores de dominio.
- [ ] Interior mutability con `Cell`, `RefCell`, `Mutex` y `RwLock`.
- [ ] Message passing con canales.
- [ ] Actor-like workers.

## Checklist de commits

- [x] Commit: plan inicial del catalogo.
- [ ] Commit: base del crate Rust.
- [ ] Commit: README principal.
- [ ] Commit individual por cada ejemplo de Abstract Factory.
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

## Siguiente paso inmediato

Crear la base del crate Rust y el README principal. Despues de eso, comenzaremos con patrones creacionales, uno por uno, iniciando con Abstract Factory.
