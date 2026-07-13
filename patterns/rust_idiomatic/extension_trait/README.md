# Extension Trait

## Intención

Extension Trait agrega métodos de conveniencia a tipos existentes sin modificar su definición original. Es una forma idiomática de hacer que operaciones frecuentes se lean como métodos propios del dominio.

## Problema cotidiano

En sistemas reales repetimos transformaciones pequeñas sobre tipos que no controlamos:

- Normalizar strings de entrada antes de validar o guardar.
- Convertir errores genéricos en errores de dominio.
- Paginar o partir colecciones sin repetir cálculos de índices.

Sin Extension Trait, estas operaciones suelen quedar como funciones sueltas y se vuelven menos descubribles en el código.

## Cómo se ve en Rust

En Rust se define un trait propio y se implementa para un tipo existente:

```rust
pub trait StringNormalizationExt {
    fn normalized_slug(&self) -> String;
}

impl StringNormalizationExt for str {
    fn normalized_slug(&self) -> String {
        self.trim().to_lowercase().replace(' ', "-")
    }
}
```

Quien importa el trait puede llamar el método como si fuera parte del tipo:

```rust
use crate::StringNormalizationExt;

let slug = "  Orden Nueva  ".normalized_slug();
```

## Cuándo usarlo

- Cuando una operación es común, pequeña y cercana a un tipo existente.
- Cuando quieres mejorar legibilidad y descubrimiento sin crear wrappers.
- Cuando el trait vive cerca del dominio que da sentido a los métodos.

## Cuándo evitarlo

- Si el método sorprende o cambia expectativas del tipo original.
- Si el trait mezcla demasiadas responsabilidades.
- Si un newtype expresa mejor invariantes o validación fuerte.
- Si el método solo se usa una vez y una función simple sería más clara.

## Relación con Newtype

Newtype crea un tipo nuevo para reforzar significado e invariantes. Extension Trait no cambia el tipo; solo agrega métodos cuando el trait está en scope.

## Ejemplos

- [x] Helpers de strings para normalizar entradas.
- [x] Helpers de `Result` para mapear errores de dominio.
- [x] Helpers de colecciones para paginar resultados.

### Normalización de strings

El módulo `string_normalization` define `StringNormalizationExt` para agregar métodos de limpieza a `str`.

El ejemplo muestra cómo normalizar espacios, emails y llaves tipo slug sin crear un wrapper nuevo para cada entrada.

### Mapeo de errores con `Result`

El módulo `result_mapping` define `ResultDomainExt` para convertir errores técnicos en `DomainError`.

El ejemplo muestra cómo conservar valores exitosos y agregar contexto de dominio cuando una operación falla.

### Paginación de colecciones

El módulo `collection_pagination` define `CollectionPaginationExt` para paginar slices y vectores.

El ejemplo muestra cómo calcular elementos, totales y páginas siguientes sin repetir lógica de índices en cada caso de uso.

## Comandos

```bash
cargo test extension_trait
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
