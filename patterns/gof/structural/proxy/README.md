# Proxy

## Intención

Proxy controla el acceso a otro objeto para agregar una política alrededor de su uso.

## Problema cotidiano

En sistemas reales no siempre queremos llamar directamente a un recurso:

- Una API externa puede ser lenta o costosa y conviene cachear respuestas.
- Un repositorio puede requerir autorización antes de leer o escribir datos.
- Un archivo grande puede cargarse solo cuando realmente se necesita.

Si el cliente conoce esas reglas, la lógica se repite en muchos lugares. Proxy conserva una interfaz similar al recurso real y agrega control de acceso, caché, carga diferida, logging o validaciones.

## Cómo se ve en Rust

En Rust, Proxy suele modelarse con traits, structs envoltorio y composición. El proxy recibe o posee el servicio real y expone métodos parecidos, devolviendo `Result` cuando la política puede rechazar la operación.

## Cuándo usarlo

- Cuando necesitas agregar control de acceso sin cambiar el recurso real.
- Cuando quieres diferir una carga costosa hasta que sea necesaria.
- Cuando quieres cachear, auditar o limitar llamadas a un servicio externo.

## Cuándo evitarlo

- Si el proxy solo delega sin agregar una política clara.
- Si oculta errores importantes del recurso real.
- Si introduce estado compartido difícil de razonar.

## Diferencia con Decorator

Decorator agrega responsabilidades manteniendo una cadena de envoltorios. Proxy se enfoca en controlar el acceso a un recurso real: puede negar, demorar, cachear o representar algo remoto.

## Ejemplos

- [x] Proxy con caché para API externa.
- [ ] Proxy con autorización para repositorio.
- [ ] Proxy lazy para cargar archivos grandes.

### Caché para API externa

El módulo `cached_api` expone `CachedApiProxy`, que conserva respuestas por id de producto antes de volver a consultar la API real.

El ejemplo muestra cómo proteger al cliente de llamadas repetidas a un recurso externo lento, caro o sujeto a límites.

## Comandos

```bash
cargo test proxy
```
