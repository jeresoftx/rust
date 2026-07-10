# Singleton

## Intención

Singleton garantiza que exista una sola instancia compartida de un recurso y ofrece un punto de acceso controlado.

## Problema cotidiano

En sistemas reales algunos recursos deben inicializarse una sola vez y reutilizarse en varias partes de la aplicación:

- Configuración global cargada al iniciar.
- Registro centralizado de métricas.
- Pools compartidos para recursos costosos.

Crear instancias sueltas en cada módulo puede duplicar estado, ocultar efectos secundarios y complicar pruebas.

## Cómo se ve en Rust

En Rust conviene evitar singletons mutables globales por defecto. Cuando hacen sentido, suelen expresarse con `std::sync::OnceLock`, tipos inmutables compartidos o sincronización explícita como `Mutex`.

La versión rústica debe dejar claro qué se inicializa, cuándo ocurre y cómo se protege el acceso concurrente.

## Cuándo usarlo

- Cuando un recurso es caro de construir y debe compartirse.
- Cuando la aplicación requiere una configuración única y estable.
- Cuando el acceso global simplifica un recurso transversal sin esconder reglas importantes.

## Cuándo evitarlo

- Si una dependencia puede inyectarse de forma explícita.
- Si el estado global dificulta pruebas aisladas.
- Si la inicialización depende de orden implícito o de datos cambiantes.

## Diferencia con estado global

Singleton no significa "usar variables globales sin control". En Rust, el patrón debe encapsular inicialización, sincronización y reglas de acceso.

## Ejemplos

- [x] Configuración global de aplicación con `OnceLock`.
- [x] Registro centralizado de métricas.
- [ ] Pool compartido simulado para recursos costosos.

### Configuración global de aplicación

El módulo `app_config` usa `OnceLock` para inicializar una sola configuración de aplicación.

El ejemplo muestra cómo consultar la misma instancia compartida sin reconstruir la configuración en cada módulo.

### Registro centralizado de métricas

El módulo `metrics_registry` usa un `OnceLock` con `Mutex` para compartir contadores de métricas.

El ejemplo muestra cómo incrementar y leer métricas desde un único registro global protegido.

## Comandos

```bash
cargo test singleton
```
