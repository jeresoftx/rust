# Layered Architecture

## Intención

Layered Architecture organiza una aplicación en capas con responsabilidades claras. Cada capa depende de la capa más estable hacia adentro y evita mezclar detalles de transporte, casos de uso, dominio y persistencia.

## Problema cotidiano

En sistemas de negocio es común que el código crezca mezclando:

- Validación HTTP con reglas de dominio.
- DTOs externos con entidades internas.
- Acceso a datos con lógica de aplicación.
- Pruebas que necesitan levantar demasiada infraestructura.

Cuando todo vive junto, los cambios se vuelven frágiles. Con capas explícitas, cada módulo puede evolucionar y probarse con menor fricción.

## Capas típicas

- Presentación: recibe requests, traduce DTOs y produce responses.
- Aplicación: coordina casos de uso y dependencias.
- Dominio: contiene entidades, reglas e invariantes.
- Infraestructura: implementa persistencia, servicios externos y detalles técnicos.

## Cómo se ve en Rust

Una estructura pequeña puede usar módulos anidados:

```rust
pub mod presentation;
pub mod application;
pub mod domain;
pub mod infrastructure;
```

Las pruebas pueden apuntar a cada frontera: dominio sin infraestructura, aplicación con repositorios reemplazables y presentación validando traducción de DTOs.

## Cuándo usarlo

- Cuando una aplicación tiene varios casos de uso.
- Cuando necesitas separar API externa de modelo de dominio.
- Cuando quieres pruebas por capa.
- Cuando habrá más de una infraestructura posible.

## Cuándo evitarlo

- Si el programa es un script pequeño.
- Si las capas solo agregan carpetas sin reglas claras.
- Si cada cambio trivial necesita tocar demasiados archivos.
- Si el dominio todavía no tiene comportamiento propio.

## Ejemplos

- [x] API de usuarios con capas de presentación, aplicación y dominio.
- [x] Separación entre DTOs, entidades y repositorios.
- [ ] Pruebas por capa con dependencias reemplazables.

### API de usuarios con capas

El módulo `user_api` separa una operación de registro de usuarios en cuatro capas:

- `presentation`: traduce requests y responses.
- `application`: coordina el caso de uso.
- `domain`: valida reglas del usuario.
- `infrastructure`: guarda usuarios en memoria.

La presentación no conoce cómo se persiste el usuario y el dominio no conoce detalles HTTP. Cada frontera queda visible en el tipo que intercambia datos con la siguiente capa.

### Separación entre DTOs, entidades y repositorios

El módulo `dto_entity_repository` muestra un flujo de órdenes donde:

- Los DTOs viven en presentación.
- Las entidades y reglas viven en dominio.
- El caso de uso vive en aplicación.
- El repositorio en memoria vive en infraestructura.

El repositorio almacena entidades de dominio, no DTOs de entrada ni DTOs de respuesta.

## Comandos

```bash
cargo test layered_architecture
```
