# Repository and Unit of Work

## Intención

Repository encapsula el acceso a colecciones de entidades y Unit of Work coordina varios cambios para confirmarlos como una sola operación lógica. Juntos ayudan a que los casos de uso no dependan de detalles de persistencia ni confirmen cambios parciales por accidente.

## Problema cotidiano

En sistemas reales, la persistencia suele mezclarse con reglas de aplicación:

- Un servicio guarda entidades directamente en varias tablas o colecciones.
- Las pruebas necesitan una base de datos real para validar reglas simples.
- Una operación actualiza usuario, orden e inventario, pero falla a media ejecución.
- El rollback queda disperso en varios lugares.

Repository y Unit of Work separan la colección de entidades de la decisión de confirmar o descartar cambios.

## Partes principales

- Entidad: objeto del dominio que se guarda y recupera.
- Repository: interfaz para agregar, buscar o listar entidades.
- Implementación en memoria: repositorio rápido para pruebas y ejemplos.
- Unit of Work: acumula cambios y decide `commit` o `rollback`.
- Transacción: frontera lógica que evita persistir cambios incompletos.

## Cómo se ve en Rust

Rust permite expresar repositorios con traits y unidades de trabajo con estructuras que poseen cambios pendientes:

```rust
trait CustomerRepository {
    fn find(&self, id: &str) -> Option<Customer>;
    fn save(&mut self, customer: Customer);
}
```

La unidad de trabajo puede mantener repositorios pendientes y aplicar cambios al almacenamiento real solo en `commit`.

## Cuándo usarlo

- Cuando quieres probar casos de uso sin infraestructura real.
- Cuando una operación toca varias colecciones.
- Cuando necesitas evitar commits parciales.
- Cuando la persistencia debe quedar detrás de una interfaz estable.

## Cuándo evitarlo

- Si el sistema es CRUD simple y el framework ya resuelve transacciones.
- Si el Unit of Work duplica una transacción real sin aportar claridad.
- Si los repositorios exponen demasiados detalles de la base de datos.
- Si el modelo en memoria se aleja de las reglas reales de persistencia.

## Ejemplos

- [x] Repositorio en memoria para pruebas.
- [x] Unidad de trabajo para confirmar varios cambios.
- [x] Transacción simulada con rollback.

### Repositorio en memoria para pruebas

El módulo `in_memory_repository` define el trait `CustomerRepository` y una implementación `InMemoryCustomerRepository`. Un servicio de aplicación puede registrar clientes y consultar emails sin depender de una base de datos real.

Este ejemplo muestra cómo un repositorio en memoria permite pruebas rápidas, deterministas y enfocadas en reglas de aplicación.

### Unidad de trabajo para confirmar varios cambios

El módulo `commit_changes` mantiene repositorios pendientes para clientes y órdenes. El `UnitOfWork` permite preparar varios cambios y publicarlos juntos con `commit`.

Antes del commit, el almacenamiento real no ve los cambios. Si se llama `rollback`, los cambios pendientes se descartan.

### Transacción simulada con rollback

El módulo `transaction_rollback` simula una frontera transaccional. La unidad de trabajo toma un snapshot del almacenamiento antes de ejecutar la operación.

Si todas las operaciones pasan, los cambios quedan aplicados. Si una operación falla, el store vuelve al snapshot original y no quedan cambios parciales.

## Comandos

```bash
cargo test repository_unit_of_work
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
