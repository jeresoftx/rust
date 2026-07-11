# Iterator

## Intención

Iterator permite recorrer una colección o flujo de datos sin exponer su representación interna.

## Problema cotidiano

En sistemas reales no siempre recorremos un `Vec` completo en memoria. A menudo necesitamos avanzar sobre datos que llegan por partes o que tienen una estructura interna distinta:

- Resultados de una API paginada.
- Registros que se procesan en lotes para no saturar memoria o servicios externos.
- Árboles de categorías que deben recorrerse en un orden estable.

Si cada consumidor conoce los detalles internos de paginación, particionado o recorrido, el código se acopla a la estructura de almacenamiento. Iterator encapsula ese avance detrás de una interfaz uniforme.

## Cómo se ve en Rust

En Rust, Iterator es idiomático porque el lenguaje ya trae el trait `Iterator`. Un tipo solo necesita implementar `next` para integrarse con `for`, `collect`, `map`, `filter` y demás adaptadores.

También es común exponer métodos como `iter`, `iter_mut`, `into_iter` o constructores específicos cuando el recorrido tiene significado de dominio, por ejemplo `depth_first`.

## Cuándo usarlo

- Cuando quieres separar el recorrido de la estructura interna.
- Cuando los datos se consumen bajo demanda.
- Cuando varios consumidores necesitan recorrer la misma estructura de forma consistente.

## Cuándo evitarlo

- Si una colección estándar y un `for` directo comunican suficiente.
- Si el orden de recorrido no es parte importante del dominio.
- Si el iterador esconde efectos secundarios costosos que deberían ser explícitos.

## Diferencia con Composite

Composite modela objetos en forma de árbol. Iterator define una forma de recorrer una estructura, sea árbol, lista, página remota o flujo de registros.

## Ejemplos

- [x] Paginación sobre resultados de API.
- [x] Iterador de lotes para procesamiento de registros.
- [ ] Recorrido de árbol de categorías.

### Paginación sobre resultados de API

El módulo `paginated_api` simula una fuente que entrega páginas de resultados.

El ejemplo muestra cómo un iterador puede ocultar la mecánica de paginación y entregar elementos uno por uno con `next` o `collect`.

### Iterador de lotes para procesamiento de registros

El módulo `record_batches` recibe registros y los entrega en grupos de tamaño fijo.

El ejemplo representa tareas comunes como enviar datos a una API por bloques, procesar filas de una exportación o repartir trabajo sin exponer índices al consumidor.

## Comandos

```bash
cargo test iterator
```
