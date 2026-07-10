# Builder

## Intención

Builder construye objetos paso a paso cuando tienen muchas opciones, valores por defecto o reglas de validación antes de crear el valor final.

## Problema cotidiano

En sistemas reales abundan objetos con campos opcionales:

- Configuración de servidores HTTP.
- Consultas de búsqueda con filtros.
- Payloads de email transaccional.

Pasar todos los parámetros a un constructor único vuelve el código frágil y difícil de leer. Builder permite expresar la intención de cada opción y validar al final.

## Cómo se ve en Rust

En Rust, Builder suele implementarse con una struct mutable o consumible que acumula configuración y termina con `build() -> Result<T, Error>` cuando hay reglas que validar.

## Cuándo usarlo

- Cuando un tipo tiene muchas opciones.
- Cuando hay valores por defecto claros.
- Cuando la construcción puede fallar por datos incompletos.
- Cuando quieres una API legible y encadenable.

## Cuándo evitarlo

- Si el tipo tiene pocos campos obligatorios.
- Si una struct literal es más clara.
- Si el builder solo copia campos sin aportar validación o legibilidad.

## Diferencia con el patrón clásico

La versión clásica suele separar builder y director. En Rust muchas veces basta con un builder asociado al tipo final. Agregamos un director solo si hay recetas reutilizables que lo justifiquen.

## Ejemplos

- [x] Construcción de configuración de servidor HTTP.
- [x] Construcción de consultas de búsqueda con filtros opcionales.
- [x] Construcción de payload de email transaccional.

### Configuración de servidor HTTP

El módulo `http_server_config` muestra un builder encadenable para una configuración con defaults claros: host local, puerto `3000`, TLS apagado y cuatro workers.

El método `build()` valida que la cantidad de workers sea positiva y devuelve `Result<HttpServerConfig, ConfigError>`.

### Consultas de búsqueda

El módulo `search_query` construye una query con término obligatorio y filtros opcionales: categoría, score mínimo, tags y ordenamiento.

El método `build()` valida que el término no esté vacío y `to_query_string()` muestra cómo quedaría una representación simple para una API.

### Payload de email transaccional

El módulo `transactional_email` muestra un builder para correos de sistema con destinatario, asunto, cuerpo, copias y headers opcionales.

El método `build()` valida los campos obligatorios y devuelve errores específicos para que el llamador pueda responder con mensajes claros.

## Comandos

```bash
cargo test builder
```
