# Rust idiomático vs patrones GoF

Los patrones GoF nacieron en un contexto orientado a objetos clásico. Rust permite expresar muchas de esas ideas, pero sus herramientas principales son distintas.

## Traducciones comunes

- Interfaces: traits.
- Herencia: composición, traits y enums.
- Polimorfismo dinámico: `Box<dyn Trait>`, `&dyn Trait` o `Arc<dyn Trait>`.
- Polimorfismo estático: generics y trait bounds.
- Estados válidos: enums o Typestate.
- Recursos con ciclo de vida: RAII y `Drop`.

## Regla práctica

Primero buscamos el modelo Rust más simple. Si un patrón clásico ayuda a nombrar la intención, lo usamos; si complica el código, explicamos la alternativa idiomática.
