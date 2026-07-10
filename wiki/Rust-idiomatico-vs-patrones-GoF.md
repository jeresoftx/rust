# Rust idiomatico vs patrones GoF

Los patrones GoF nacieron en un contexto orientado a objetos clasico. Rust permite expresar muchas de esas ideas, pero sus herramientas principales son distintas.

## Traducciones comunes

- Interfaces: traits.
- Herencia: composicion, traits y enums.
- Polimorfismo dinamico: `Box<dyn Trait>`, `&dyn Trait` o `Arc<dyn Trait>`.
- Polimorfismo estatico: generics y trait bounds.
- Estados validos: enums o Typestate.
- Recursos con ciclo de vida: RAII y `Drop`.

## Regla practica

Primero buscamos el modelo Rust mas simple. Si un patron clasico ayuda a nombrar la intencion, lo usamos; si complica el codigo, explicamos la alternativa idiomatica.
