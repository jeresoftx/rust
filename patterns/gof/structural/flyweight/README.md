# Flyweight

## Intención

Flyweight reduce el uso de memoria compartiendo datos repetidos entre muchos objetos.

## Problema cotidiano

En sistemas reales hay datos que se repiten una y otra vez:

- Monedas, países o catálogos de referencia.
- Metadatos de productos usados por muchas variantes.
- Estilos reutilizados al renderizar documentos.

Si cada objeto duplica esos datos, aumenta el uso de memoria y se dificulta mantener una fuente única de verdad. Flyweight separa el estado compartido del estado específico de cada instancia.

## Cómo se ve en Rust

En Rust, Flyweight suele apoyarse en `Arc`, `Rc`, cachés, interning o registros que devuelven referencias compartidas. La clave es distinguir entre estado intrínseco compartido y estado extrínseco que pertenece a cada uso concreto.

## Cuándo usarlo

- Cuando muchos objetos repiten los mismos datos inmutables.
- Cuando puedes centralizar un catálogo o registro compartido.
- Cuando compartir estado reduce memoria sin complicar demasiado el diseño.

## Cuándo evitarlo

- Si los objetos no se repiten lo suficiente.
- Si compartir referencias vuelve más difícil entender la propiedad de los datos.
- Si el costo de la caché supera el ahorro.

## Diferencia con Singleton

Singleton busca una única instancia global de un recurso. Flyweight comparte muchas instancias pequeñas y reutilizables por clave.

## Ejemplos

- [x] Catálogo compartido de monedas o países.
- [ ] Caché de metadatos de productos repetidos.
- [ ] Reutilización de estilos en render de documentos.

### Catálogo de monedas

El módulo `currency_catalog` usa `Arc<Currency>` para compartir definiciones de moneda entre muchos montos.

El ejemplo muestra cómo mantener el estado variable en `MoneyAmount` y reutilizar la moneda como estado intrínseco.

## Comandos

```bash
cargo test flyweight
```
