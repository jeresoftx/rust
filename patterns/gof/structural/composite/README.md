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

- [x] Árbol de permisos por módulo y acción.
- [x] Estructura de menú con submenús.
- [x] Carpeta con archivos y subcarpetas para calcular tamaño.

### Árbol de permisos

El módulo `permissions` modela permisos individuales y grupos anidados con una misma estructura compuesta.

El ejemplo muestra cómo consultar permisos y listar rutas de autorización sin distinguir manualmente entre hojas y grupos.

### Menú con submenús

El módulo `menu` modela enlaces y secciones anidadas como elementos de un mismo árbol.

El ejemplo muestra cómo renderizar un menú completo y buscar una URL dentro de submenús sin lógica especial por nivel.

### Carpetas y archivos

El módulo `file_system` modela archivos y carpetas como entradas de un árbol común.

El ejemplo muestra cómo calcular tamaño total y listar rutas completas recorriendo la estructura compuesta.

## Comandos

```bash
cargo test composite
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
