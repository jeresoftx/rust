# Visitor

Visitor separa operaciones de la estructura de objetos que esas operaciones recorren.

## Idea central

Los nodos aceptan un visitante. El visitante sabe qué hacer con cada tipo de nodo y puede acumular resultados durante el recorrido.

En Rust suele aparecer como:

- Traits para visitantes.
- Métodos `accept` en los nodos.
- Estructuras que acumulan texto, totales, errores o métricas.
- Enums con `match` cuando se busca una versión más idiomática y cerrada.

## Ejemplos del repositorio

- Exportar un árbol de expresiones a texto y JSON: pendiente.
- Calcular totales recorriendo elementos de factura: pendiente.
- Validar nodos de un workflow: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/visitor/README.md`
