# Template Method

## Intención

Template Method define el esqueleto de un algoritmo en una operación común y deja que algunos pasos sean personalizados por implementaciones concretas.

## Problema cotidiano

Muchos flujos de negocio comparten pasos obligatorios, pero tienen variaciones pequeñas:

- Importar archivos siempre valida, transforma y guarda, aunque cada formato parsea distinto.
- Generar reportes siempre arma portada, cuerpo y cierre, aunque cada tipo incluye secciones diferentes.
- Crear una cuenta siempre valida datos y activa servicios base, aunque cada plan tiene beneficios propios.

Si cada variante copia el flujo completo, los cambios de seguridad, auditoría o validación se duplican. Template Method centraliza el orden del proceso y deja hooks para lo variable.

## Cómo se ve en Rust

En Rust suele modelarse con traits que tienen métodos con implementación por defecto. El método principal define el flujo completo y llama métodos requeridos u opcionales que cada tipo implementa.

Esta forma funciona bien cuando el orden del algoritmo debe ser estable, pero algunos pasos cambian según el caso.

## Cuándo usarlo

- Cuando varias variantes comparten el mismo flujo general.
- Cuando el orden de los pasos es una regla de negocio.
- Cuando quieres ofrecer hooks claros sin duplicar el algoritmo completo.

## Cuándo evitarlo

- Si las variantes no comparten una secuencia real.
- Si una composición con funciones pequeñas sería más simple.
- Si los hooks empiezan a formar demasiadas combinaciones difíciles de razonar.

## Diferencia con Strategy

Strategy intercambia algoritmos completos. Template Method fija el algoritmo principal y permite variar pasos específicos.

## Ejemplos

- [x] Flujo común para importar archivos con pasos variables.
- [x] Generación de reportes con secciones personalizadas.
- [x] Proceso de onboarding con hooks por tipo de cuenta.

### Flujo común para importar archivos

El módulo `file_import` modela un pipeline fijo para importar datos: leer, parsear, normalizar, validar y guardar.

El ejemplo muestra cómo cada importador cambia el parser y el destino, mientras el orden del proceso se mantiene en un solo método plantilla.

### Generación de reportes

El módulo `report_generation` modela reportes con portada, resumen, secciones y pie.

El ejemplo muestra cómo cada reporte personaliza sus secciones y puede usar un hook para cambiar el pie sin duplicar el flujo de generación.

### Proceso de onboarding

El módulo `onboarding` modela un flujo común para validar email, crear cuenta, aprovisionar servicios base, ejecutar hooks y cerrar el alta.

El ejemplo muestra cómo cuentas starter y enterprise comparten el flujo principal, pero cambian hooks como correo de bienvenida, asignación de success manager o SSO.

## Comandos

```bash
cargo test template_method
```
