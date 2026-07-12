# Actor-like Workers

## Intención

Actor-like Workers organizan estado y comportamiento dentro de un hilo o tarea que recibe comandos por un canal. El resto del sistema no toca el estado directamente; envía mensajes y espera respuestas cuando las necesita.

## Problema cotidiano

En sistemas reales aparecen componentes que deben serializar decisiones:

- Un worker de email recibe comandos para enviar mensajes y registrar resultados.
- Un actor de inventario aplica reservas y liberaciones en orden.
- Un agregador de métricas recibe eventos desde varias partes del sistema.

Si todos modifican el mismo estado compartido, aparecen locks largos y reglas dispersas. Con un actor, el estado vive en un solo lugar y los cambios pasan por comandos explícitos.

## Cómo se ve en Rust

Rust no obliga a usar un framework de actores para este estilo. Con `std::sync::mpsc`, un hilo y comandos tipados se puede modelar un actor pequeño:

```rust
enum Command {
    Increment,
    Snapshot(std::sync::mpsc::Sender<u64>),
    Shutdown,
}
```

El actor procesa comandos en orden y conserva ownership de su estado interno.

## Cuándo usarlo

- Cuando un recurso debe serializar cambios.
- Cuando quieres evitar compartir estado mutable entre varios llamadores.
- Cuando los comandos del componente son finitos y se pueden tipar.
- Cuando necesitas respuestas puntuales por canales de retorno.

## Cuándo evitarlo

- Si una función directa o un `Mutex` simple expresa mejor el caso.
- Si el actor termina siendo un objeto enorme con demasiadas responsabilidades.
- Si no tienes una política clara para apagar el worker.
- Si cada comando necesita bloquear esperando muchas dependencias externas.

## Ejemplos

- [ ] Actor de email que recibe comandos.
- [ ] Actor de inventario que serializa cambios.
- [ ] Actor de métricas que agrega eventos.

## Comandos

```bash
cargo test actor_like
```
