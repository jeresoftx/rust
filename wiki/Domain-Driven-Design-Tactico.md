# Domain-Driven Design Táctico

Domain-Driven Design táctico modela reglas del negocio con tipos explícitos, agregados y eventos de dominio.

## Idea central

El dominio debe hablar con el vocabulario del negocio. Las invariantes viven cerca de los datos que protegen, y los casos importantes se expresan con métodos de intención.

En Rust suele aparecer como:

- Value objects con constructores que validan datos.
- Entidades con identidad estable.
- Agregados que protegen cambios consistentes.
- Servicios de dominio para reglas que cruzan varias entidades.
- Eventos de dominio para comunicar hechos relevantes.

## Ejemplos del repositorio

- Agregados y value objects para órdenes: pendiente.
- Servicios de dominio para políticas de descuento: pendiente.
- Eventos de dominio para integración interna: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/domain_driven_design_tactical/README.md`
