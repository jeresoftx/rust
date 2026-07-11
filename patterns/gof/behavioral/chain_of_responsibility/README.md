# Chain of Responsibility

## Intención

Chain of Responsibility permite pasar una solicitud por una cadena de manejadores hasta que alguno la procesa o todos la rechazan.

## Problema cotidiano

En sistemas reales muchas decisiones se resuelven por etapas:

- Un request necesita pasar por validaciones de autenticación, permisos y datos obligatorios.
- Un ticket de soporte debe escalarse según prioridad o tema.
- Un mensaje de usuario puede pasar por filtros de moderación antes de publicarse.

Si todo vive en un bloque grande de `if` y `match`, el flujo se vuelve difícil de extender. La cadena separa cada regla en un manejador pequeño y ordenado.

## Cómo se ve en Rust

En Rust puede modelarse con traits, enums, closures o una lista de funciones que reciben contexto y devuelven una decisión. No es necesario imitar jerarquías OOP; lo importante es que cada paso decida si procesa, rechaza o deja continuar.

## Cuándo usarlo

- Cuando hay reglas ordenadas que pueden detener el flujo.
- Cuando quieres agregar o quitar pasos sin reescribir un método central.
- Cuando cada manejador tiene una responsabilidad clara.

## Cuándo evitarlo

- Si el orden de reglas no importa y una composición simple basta.
- Si la cadena oculta demasiado qué paso produjo el resultado.
- Si los manejadores comparten demasiado estado mutable.

## Diferencia con Pipeline

Un pipeline normalmente transforma datos en cada paso. Chain of Responsibility decide si una solicitud se maneja, se rechaza o avanza al siguiente manejador.

## Ejemplos

- [x] Pipeline de validación de requests.
- [x] Resolución de soporte por niveles.
- [x] Filtros de moderación de contenido.

### Validación de requests

El módulo `request_validation` usa una cadena de validadores para revisar autenticación, permisos y payload.

El ejemplo muestra cómo detener el flujo en el primer error y devolver una respuesta clara sin ejecutar validaciones posteriores.

### Soporte por niveles

El módulo `support_levels` modela una cadena de soporte donde cada nivel decide si puede resolver un ticket.

El ejemplo muestra cómo un caso simple se resuelve en nivel 1 y un incidente de infraestructura escala hasta nivel 3.

### Moderación de contenido

El módulo `content_moderation` revisa un mensaje con filtros de spam, palabras prohibidas y longitud.

El ejemplo muestra cómo un mensaje limpio atraviesa toda la cadena y cómo un rechazo temprano evita ejecutar reglas posteriores.

## Comandos

```bash
cargo test chain_of_responsibility
```
