# Strategy

## Intención

Strategy permite definir una familia de algoritmos, encapsular cada variante y elegir cuál usar sin cambiar el código que la ejecuta.

## Problema cotidiano

En sistemas reales es común que una misma operación tenga varias reglas posibles:

- Un checkout puede aplicar descuentos por porcentaje, monto fijo o volumen.
- Una búsqueda puede ordenar resultados por relevancia, precio o fecha.
- Un cálculo de envío puede cambiar por distancia, peso o proveedor.

Si todas las reglas viven en un `match` enorme dentro del caso de uso, cada variante nueva obliga a tocar el flujo principal. Strategy separa la decisión del algoritmo de su ejecución.

## Cómo se ve en Rust

En Rust, Strategy suele representarse con traits, closures o enums con métodos. La opción más directa para ejemplos extensibles es un trait que describe la operación y varias estructuras que implementan ese contrato.

También puedes usar generics cuando quieres dispatch estático, o `Box<dyn Trait>` cuando necesitas elegir estrategias en tiempo de ejecución.

## Cuándo usarlo

- Cuando tienes varias formas intercambiables de resolver una operación.
- Cuando el cliente debe elegir el algoritmo.
- Cuando quieres probar cada variante de negocio por separado.

## Cuándo evitarlo

- Si solo hay una regla y no se espera variación real.
- Si un enum pequeño expresa mejor todas las opciones cerradas.
- Si la estrategia necesita demasiados datos internos del contexto.

## Diferencia con State

State cambia el comportamiento por una transición interna del objeto. Strategy cambia el algoritmo porque alguien eligió una política o variante.

## Ejemplos

- [x] Estrategias de descuento.
- [x] Estrategias de ordenamiento de resultados.
- [ ] Estrategias de cálculo de envío.

### Estrategias de descuento

El módulo `discounts` modela un checkout que puede calcular el total con descuento porcentual, descuento fijo o descuento por volumen.

El ejemplo muestra cómo el flujo principal no cambia cuando se agrega una nueva regla de descuento: solo se intercambia la estrategia.

### Estrategias de ordenamiento de resultados

El módulo `result_sorting` modela resultados de búsqueda que pueden ordenarse por relevancia, menor precio o fecha más reciente.

El ejemplo muestra cómo una API puede cambiar el criterio de ordenamiento sin mezclar esas reglas con la presentación de resultados.

## Comandos

```bash
cargo test strategy
```
