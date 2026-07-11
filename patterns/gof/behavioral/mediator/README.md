# Mediator

## Intención

Mediator centraliza la comunicación entre varios objetos para evitar que se conozcan y coordinen directamente entre sí.

## Problema cotidiano

En sistemas reales es común que varias piezas tengan que reaccionar al mismo flujo:

- Un formulario necesita validar datos, guardar cambios y mostrar mensajes.
- Una sala de chat debe distribuir mensajes sin que cada usuario conozca a todos los demás.
- Un checkout coordina inventario, pago, envío y notificaciones.

Si cada componente llama a todos los demás, el acoplamiento crece rápido. Mediator mueve esa coordinación a un objeto explícito que conoce el flujo y reduce las dependencias entre participantes.

## Cómo se ve en Rust

En Rust, Mediator suele aparecer como una estructura que recibe eventos o comandos y coordina servicios internos. También puede tomar forma de enums de eventos, traits para participantes o funciones de orquestación cuando el flujo es pequeño.

La clave no es crear una clase gigante, sino concentrar una conversación específica del dominio en un punto claro y testeable.

## Cuándo usarlo

- Cuando varios componentes se coordinan en el mismo caso de uso.
- Cuando quieres que los participantes no se llamen entre sí directamente.
- Cuando el orden de pasos, validaciones o efectos del flujo importa.

## Cuándo evitarlo

- Si una función directa expresa mejor el caso.
- Si el mediador empieza a concentrar reglas de dominios no relacionados.
- Si solo estás escondiendo dependencias sin simplificar la conversación.

## Diferencia con Facade

Facade ofrece una entrada simple a un subsistema. Mediator coordina participantes que podrían conversar entre sí y decide cómo fluye esa interacción.

## Ejemplos

- [x] Coordinador de eventos entre formulario, validación y guardado.
- [x] Sala de chat que media usuarios.
- [x] Orquestador de módulos de checkout.

### Coordinador de formulario

El módulo `form_workflow` modela un mediador que recibe un formulario de registro y coordina validación, persistencia y notificación.

El ejemplo muestra cómo los participantes no necesitan llamarse entre sí: el mediador decide si guardar, qué evento emitir y cuándo cortar el flujo.

### Sala de chat

El módulo `chat_room` representa una sala que media mensajes entre usuarios.

El ejemplo muestra broadcast, mensajes privados y fallas de entrega sin que un usuario tenga referencias directas a los demás.

### Orquestador de checkout

El módulo `checkout_orchestrator` coordina inventario, pago, envío y notificación.

El ejemplo muestra un flujo exitoso, un rechazo por inventario y una compensación cuando el pago falla después de reservar stock.

## Comandos

```bash
cargo test mediator
```
