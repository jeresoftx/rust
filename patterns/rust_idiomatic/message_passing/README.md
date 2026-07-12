# Message Passing

## Intención

Message Passing coordina trabajo enviando mensajes entre productores y consumidores. En Rust es una forma natural de mover ownership entre hilos sin compartir memoria mutable directamente.

## Problema cotidiano

En sistemas reales, muchos flujos funcionan mejor cuando cada parte procesa mensajes claros:

- Un worker consume jobs enviados por una cola.
- Un evento de dominio se distribuye a varios consumidores.
- Un pipeline transforma datos en etapas independientes.

Si todas las partes comparten estructuras mutables, la coordinación se vuelve frágil. Con canales, cada etapa recibe datos propios y comunica resultados de forma explícita.

## Cómo se ve en Rust

La biblioteca estándar incluye canales en `std::sync::mpsc`:

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();
sender.send("job").expect("message should be sent");

let job = receiver.recv().expect("message should arrive");
assert_eq!(job, "job");
```

Los mensajes se mueven entre hilos. Eso reduce la necesidad de locks cuando el diseño puede expresarse como flujo de datos.

## Cuándo usarlo

- Cuando un productor y un consumidor pueden comunicarse por mensajes.
- Cuando quieres aislar etapas de procesamiento.
- Cuando mover ownership es más simple que compartir estado mutable.
- Cuando necesitas modelar colas, eventos o pipelines internos.

## Cuándo evitarlo

- Si una llamada directa expresa mejor el flujo.
- Si el canal oculta demasiada lógica de negocio.
- Si no tienes una política clara para cerrar el canal.
- Si necesitas broadcast real y estás usando `mpsc` sin fan-out explícito.

## Ejemplos

- [x] Worker que procesa jobs desde un canal.
- [x] Fan-out de eventos a consumidores.
- [ ] Pipeline de etapas con canales.

### Worker que procesa jobs desde un canal

El módulo `job_worker` usa `std::sync::mpsc` para separar productores de un worker consumidor.

Los productores envían `Job` por el canal. El worker vive en otro hilo, consume hasta que todos los `Sender` se cierran y devuelve un `JobReport` con los jobs procesados. Es un patrón común para colas internas pequeñas, trabajos en segundo plano y coordinación sin memoria mutable compartida.

### Fan-out de eventos a consumidores

El módulo `event_fanout` muestra que `mpsc` no es broadcast automático: para entregar un evento a varios consumidores, el publicador conserva un `Sender` por consumidor y clona el evento.

Este estilo funciona bien para eventos pequeños de dominio, auditoría, métricas o notificaciones internas donde cada consumidor procesa su propia copia.

## Comandos

```bash
cargo test message_passing
```
