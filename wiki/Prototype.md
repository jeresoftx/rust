# Prototype

Prototype crea nuevos valores copiando una instancia base y ajustando solo las diferencias.

## Idea central

En lugar de reconstruir todo desde cero, se parte de una plantilla confiable. El nuevo objeto conserva la estructura base y cambia los campos específicos del caso.

En Rust suele aparecer como:

- `#[derive(Clone)]` para valores simples.
- Métodos como `for_customer(...)` o `for_environment(...)`.
- Separación entre datos compartidos y datos específicos.

## Ejemplos del repositorio

- Plantillas de reportes con ajustes por cliente.
- Configuraciones base de despliegue.
- Órdenes creadas desde una plantilla preconfigurada.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/creational/prototype/README.md`
