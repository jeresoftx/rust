# Visitor

## Intención

Visitor permite agregar operaciones nuevas sobre una estructura de objetos sin cambiar los tipos que forman esa estructura.

## Problema cotidiano

En sistemas reales hay modelos que se recorren de muchas formas:

- Un árbol de expresiones puede exportarse a texto, JSON o SQL.
- Una factura puede recorrerse para calcular subtotales, impuestos y descuentos.
- Un workflow puede recorrerse para validar reglas antes de activarlo.

Si cada operación vive dentro de los nodos, esos tipos crecen con responsabilidades que no siempre pertenecen al modelo. Visitor separa la estructura de las operaciones que se ejecutan sobre ella.

## Cómo se ve en Rust

En Rust muchas veces un `enum` con `match` es más simple que Visitor. Aun así, Visitor es útil para explicar recorridos donde quieres separar operaciones y mantener estable una jerarquía de nodos.

Puede modelarse con traits para visitantes, métodos `accept` en los nodos y estructuras concretas que acumulan resultados durante el recorrido.

## Cuándo usarlo

- Cuando tienes una estructura de nodos estable y muchas operaciones externas.
- Cuando quieres recorrer una estructura compleja sin mezclar exportación, cálculo o validación en los nodos.
- Cuando cada operación necesita mantener estado propio durante el recorrido.

## Cuándo evitarlo

- Si la estructura de nodos cambia constantemente.
- Si un `enum` y un `match` expresan mejor el dominio.
- Si solo existe una operación y no hay señales de que habrá más.

## Diferencia con Iterator

Iterator define cómo recorrer elementos. Visitor define qué operación ejecutar sobre cada tipo de nodo visitado.

## Ejemplos

- [ ] Exportar un árbol de expresiones a texto y JSON.
- [ ] Calcular totales recorriendo elementos de factura.
- [ ] Validar nodos de un workflow.

## Comandos

```bash
cargo test visitor
```
