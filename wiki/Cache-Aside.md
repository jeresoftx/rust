# Cache Aside

Cache Aside consulta primero el cache y, si no encuentra el dato, carga desde la fuente de verdad y guarda el resultado para próximas lecturas.

## Qué problema resuelve

- Reduce lecturas repetidas al repositorio.
- Mantiene el cache fuera de la fuente de verdad.
- Permite invalidar al actualizar datos.
- Agrega TTL para limitar datos obsoletos.
- Mejora latencia en consultas frecuentes.

## Ejemplos del repositorio

- [x] Leer de caché o cargar del repositorio.
- [ ] Invalidación al actualizar datos.
- [ ] TTL simulado con reloj determinista.

## Código

- Documentación local: `patterns/distributed_systems/cache_aside/README.md`
- Módulo Rust: `src/patterns/distributed_systems/cache_aside.rs`
- Ejemplo de lectura cache-aside: `src/patterns/distributed_systems/cache_aside/read_through.rs`
