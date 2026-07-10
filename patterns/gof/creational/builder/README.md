# Builder

## Intencion

Builder construye objetos paso a paso cuando tienen muchas opciones, valores por defecto o reglas de validacion antes de crear el valor final.

## Problema cotidiano

En sistemas reales abundan objetos con campos opcionales:

- Configuracion de servidores HTTP.
- Consultas de busqueda con filtros.
- Payloads de email transaccional.

Pasar todos los parametros a un constructor unico vuelve el codigo fragil y dificil de leer. Builder permite expresar la intencion de cada opcion y validar al final.

## Como se ve en Rust

En Rust, Builder suele implementarse con una struct mutable o consumible que acumula configuracion y termina con `build() -> Result<T, Error>` cuando hay reglas que validar.

## Cuando usarlo

- Cuando un tipo tiene muchas opciones.
- Cuando hay valores por defecto claros.
- Cuando la construccion puede fallar por datos incompletos.
- Cuando quieres una API legible y encadenable.

## Cuando evitarlo

- Si el tipo tiene pocos campos obligatorios.
- Si una struct literal es mas clara.
- Si el builder solo copia campos sin aportar validacion o legibilidad.

## Diferencia con el patron clasico

La version clasica suele separar builder y director. En Rust muchas veces basta con un builder asociado al tipo final. Agregamos un director solo si hay recetas reutilizables que lo justifiquen.

## Ejemplos

- [x] Construccion de configuracion de servidor HTTP.
- [x] Construccion de consultas de busqueda con filtros opcionales.
- [ ] Construccion de payload de email transaccional.

### Configuracion de servidor HTTP

El modulo `http_server_config` muestra un builder encadenable para una configuracion con defaults claros: host local, puerto `3000`, TLS apagado y cuatro workers.

El metodo `build()` valida que la cantidad de workers sea positiva y devuelve `Result<HttpServerConfig, ConfigError>`.

### Consultas de busqueda

El modulo `search_query` construye una query con termino obligatorio y filtros opcionales: categoria, score minimo, tags y ordenamiento.

El metodo `build()` valida que el termino no este vacio y `to_query_string()` muestra como quedaria una representacion simple para una API.

## Comandos

```bash
cargo test builder
```
