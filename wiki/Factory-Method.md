# Factory Method

Factory Method concentra la decisión de qué implementación crear y permite que el cliente trabaje con un contrato común.

## Idea central

El cliente pide un producto para un caso específico. La factory decide qué tipo concreto entregar.

En Rust suele aparecer como:

- Una función que devuelve `Box<dyn Trait>`.
- Una función asociada como `ProcessorFactory::from_extension(...)`.
- Un `enum` cuando todas las variantes son conocidas y no hace falta despacho dinámico.

## Ejemplos del repositorio

- Procesadores de archivos CSV, JSON y XML.
- Métodos de pago según canal.
- Clientes HTTP por ambiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/creational/factory_method/README.md`
