# Plugin Architecture

Plugin Architecture define contratos estables para agregar capacidades al sistema sin modificar el núcleo de la aplicación.

## Qué problema resuelve

- Evita que el núcleo crezca con ramas por cada variante.
- Permite registrar extensiones concretas detrás de un contrato común.
- Facilita activar capacidades por configuración o contexto.
- Hace más fácil probar cada plugin de forma aislada.

## Estructura típica

- Contrato del plugin.
- Plugins concretos.
- Registro de plugins.
- Núcleo que consume el contrato.
- Configuración que decide qué extensión usar.

## Ejemplos del repositorio

- [ ] Exportadores JSON, CSV y texto.
- [ ] Estrategias cargadas por configuración.
- [ ] Extensiones internas con traits y trait objects.

## Código

- Documentación local: `patterns/architecture/plugin_architecture/README.md`
- Módulo Rust: `src/patterns/architecture/plugin_architecture.rs`
