# Interpreter

## Intención

Interpreter define una forma de representar y evaluar expresiones de un lenguaje pequeño.

## Problema cotidiano

En sistemas reales aparecen lenguajes simples en muchos lugares:

- Filtros de búsqueda como `status = paid` o `total > 100`.
- Reglas de descuentos que combinan condiciones de carrito y cliente.
- Permisos expresados como `role:admin AND scope:orders.write`.

Si estas reglas se codifican como `if` dispersos, se vuelven difíciles de reutilizar o explicar. Interpreter modela la regla como un árbol o estructura que puede evaluarse contra un contexto.

## Cómo se ve en Rust

En Rust, Interpreter suele encajar bien con enums recursivos, traits para expresiones o parsers pequeños. Para lenguajes cerrados, un enum con variantes como `And`, `Or`, `Equals` o `GreaterThan` suele ser claro y seguro.

## Cuándo usarlo

- Cuando tienes un lenguaje pequeño y estable.
- Cuando quieres evaluar reglas contra distintos contextos.
- Cuando la estructura de la expresión importa tanto como su resultado.

## Cuándo evitarlo

- Si una función directa comunica mejor la regla.
- Si el lenguaje crecerá hasta necesitar un parser formal.
- Si las expresiones vendrán de usuarios sin validación ni límites.

## Diferencia con Strategy

Strategy selecciona un algoritmo intercambiable. Interpreter representa una expresión compuesta que se evalúa contra un contexto.

## Ejemplos

- [x] Filtros simples de búsqueda `campo operador valor`.
- [x] Reglas de descuentos expresadas como árbol.
- [x] Mini lenguaje para permisos.

### Filtros de búsqueda

El módulo `search_filter` interpreta expresiones como `status = paid` y `total > 100`.

El ejemplo muestra cómo convertir texto simple en una expresión evaluable contra registros de búsqueda.

### Reglas de descuentos

El módulo `discount_rules` representa reglas de promoción como un árbol con condiciones `And` y `Or`.

El ejemplo muestra cómo evaluar una regla compuesta contra un carrito sin esconder la estructura de la expresión.

### Mini lenguaje de permisos

El módulo `permission_language` interpreta reglas como `role:admin AND scope:orders.write`.

El ejemplo muestra cómo convertir una regla de autorización compacta en una expresión evaluable contra roles y scopes.

## Comandos

```bash
cargo test interpreter
```
