# Prototype

## Intención

Prototype crea nuevos objetos copiando una instancia base y ajustando solo lo que cambia.

## Problema cotidiano

En sistemas reales solemos tener configuraciones, plantillas o entidades base que se repiten con pequeñas variaciones:

- Reportes con estructura común y datos por cliente.
- Configuraciones de despliegue por ambiente.
- Órdenes creadas desde una plantilla recurrente.

Crear todo desde cero duplica información y aumenta el riesgo de inconsistencias.

## Cómo se ve en Rust

En Rust, Prototype suele apoyarse en `Clone` y en métodos explícitos que clonan una plantilla y aplican cambios. Cuando el objeto es grande, conviene separar datos compartidos de datos variables.

## Cuándo usarlo

- Cuando crear desde cero es repetitivo o costoso.
- Cuando existe una plantilla clara con valores seguros por defecto.
- Cuando quieres preservar una base y derivar variantes controladas.

## Cuándo evitarlo

- Si clonar oculta cambios importantes.
- Si el objeto contiene recursos que no deben duplicarse sin cuidado.
- Si una función constructora expresa mejor la intención.

## Diferencia con Builder

Builder construye paso a paso. Prototype parte de una instancia existente y la copia para producir variantes.

## Ejemplos

- [ ] Clonar plantillas de reportes con ajustes por cliente.
- [ ] Duplicar configuraciones base de despliegue.
- [ ] Crear órdenes desde una plantilla preconfigurada.

## Comandos

```bash
cargo test prototype
```
