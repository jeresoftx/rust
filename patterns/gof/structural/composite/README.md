# Composite

## Intención

Composite permite tratar objetos individuales y grupos de objetos de forma uniforme dentro de una estructura de árbol.

## Problema cotidiano

En sistemas reales muchas estructuras son jerárquicas:

- Permisos agrupados por módulo y acción.
- Menús con submenús y opciones finales.
- Carpetas con archivos y subcarpetas.

El código cliente no debería tener que distinguir constantemente entre una hoja y un grupo. Composite define una interfaz común para recorrer o calcular sobre toda la estructura.

## Cómo se ve en Rust

En Rust, Composite puede modelarse con `enum` recursivos, structs con `Vec` de hijos, trait objects o combinaciones de estos enfoques. Cuando las variantes son conocidas, un `enum` suele ser más simple y seguro.

## Cuándo usarlo

- Cuando necesitas recorrer o calcular sobre árboles.
- Cuando hojas y grupos deben responder a la misma operación.
- Cuando quieres evitar lógica repetida para cada nivel de la jerarquía.

## Cuándo evitarlo

- Si la estructura no es realmente jerárquica.
- Si las hojas y los grupos tienen comportamientos demasiado distintos.
- Si una lista plana con filtros expresa mejor el problema.

## Diferencia con Decorator

Composite organiza objetos en árboles. Decorator envuelve un objeto para añadir comportamiento.

## Ejemplos

- [ ] Árbol de permisos por módulo y acción.
- [ ] Estructura de menú con submenús.
- [ ] Carpeta con archivos y subcarpetas para calcular tamaño.

## Comandos

```bash
cargo test composite
```
