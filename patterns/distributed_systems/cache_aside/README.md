# Cache Aside

## Intención

Cache Aside mantiene el cache como una capa externa al repositorio principal. La aplicación primero consulta el cache; si no encuentra el dato, carga desde el repositorio y guarda el resultado para siguientes lecturas.

## Problema cotidiano

Muchas consultas son repetidas y costosas. Leer siempre del repositorio puede aumentar latencia y carga. Pero cachear sin reglas de invalidación o expiración puede devolver datos obsoletos.

## Partes

- **Cache:** almacenamiento rápido con datos derivados.
- **Repositorio:** fuente de verdad.
- **Miss:** ausencia en cache que dispara carga desde repositorio.
- **Invalidación:** limpieza del dato cuando cambia.
- **TTL:** expiración temporal para reducir datos viejos.

## Cuándo usarlo

Úsalo en catálogos, perfiles, configuraciones, productos y consultas frecuentes con tolerancia controlada a datos antiguos.

## Cuándo evitarlo

Evítalo cuando la consistencia fuerte es obligatoria en cada lectura o cuando el costo de invalidar correctamente supera el beneficio.

## Ejemplos

- [x] Leer de caché o cargar del repositorio.
- [x] Invalidación al actualizar datos.
- [x] TTL simulado con reloj determinista.

### Leer de caché o cargar del repositorio

El primer ejemplo carga un producto desde repositorio solo en miss y luego lo atiende desde cache.

El módulo `read_through` consulta el cache antes del repositorio. En miss carga el producto, lo guarda en cache y evita lecturas repetidas.

### Invalidación al actualizar datos

El segundo ejemplo invalida el cache cuando se actualiza la fuente de verdad.

El módulo `invalidation` actualiza el repositorio y elimina solo la entrada afectada del cache para que la siguiente lectura recargue el valor vigente.

### TTL simulado con reloj determinista

El tercer ejemplo expira entradas con un reloj lógico para pruebas estables.

El módulo `ttl_cache` asocia cada entrada con un tick de expiración. Antes del vencimiento usa cache; al vencer recarga desde la fuente de verdad.

## Cómo ejecutar

```bash
cargo test cache_aside
```
