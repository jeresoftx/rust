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

- [x] Clonar plantillas de reportes con ajustes por cliente.
- [x] Duplicar configuraciones base de despliegue.
- [x] Crear órdenes desde una plantilla preconfigurada.

### Plantillas de reportes

El módulo `report_templates` usa `Clone` para copiar una plantilla financiera estándar y personalizar cliente y período.

El ejemplo demuestra que la plantilla original no se modifica al crear un reporte derivado.

### Configuraciones de despliegue

El módulo `deployment_configs` clona una configuración base de servicio web para crear variantes de staging y producción.

Cada variante cambia ambiente, réplicas y dominio sin modificar la configuración base.

### Órdenes desde plantilla

El módulo `order_templates` clona una orden recurrente de insumos de oficina y ajusta cliente y cantidad.

El ejemplo muestra cómo crear órdenes repetibles sin mutar la plantilla original.

## Comandos

```bash
cargo test prototype
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
