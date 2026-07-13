# Checklist: alinear el repo con RFC-0001

El catálogo de patrones (52 patrones, ~135 ejemplos) está funcionalmente
completo (ver [catalogo-patrones-rust.md](./catalogo-patrones-rust.md)), pero
no cumple todavía el estándar publicable del manual (RFC-0001 §13, §15, §20).
Este documento es el tablero de esa brecha específica.

Nada aquí tiene fecha límite (RFC-0001 §1); es una lista de qué falta, no un
sprint.

## 1. Gobernanza del repositorio (§15 — plantilla obligatoria)

- Registro §2: la gobernanza convierte el catálogo en un repositorio legible como
  libro de ingeniería; la alternativa era dejar convenciones implícitas, pero
  RFC-0001 §15 exige archivos canónicos para que humanos e IA trabajen igual.

- [x] Crear `AGENTS.md` (obligatorio en todo repositorio, §20). Instanciar el
      canónico con `{colección}` = complementario (patrones) y `{tema}` =
      catálogo de patrones de diseño en Rust.
- [x] Crear `ROADMAP.md` en la raíz. No duplicar `plan/catalogo-patrones-rust.md`
      — puede ser un puntero corto a él, igual que los `.md` históricos de la
      raíz del ecosistema apuntan a capítulos del manual.
- [x] Doble licencia (§15): agregar `LICENSE-APACHE` y una licencia de
      contenido `LICENSE-CC-BY-SA-4.0.md` para `wiki/` y los README de
      `patterns/`; actualizar `Cargo.toml` (`license = "MIT OR Apache-2.0"`) y
      dejar `LICENSE.md` como índice de ambas, en vez de un único `LICENSE` MIT.
- [x] Crear `diagrams/` — al menos un diagrama Mermaid/SVG por familia (GoF,
      idiomáticos, arquitectura, distribuidos) que muestre la relación entre
      sus piezas; o, si se decide que no aplica para alguno, declararlo por
      escrito (no dejarlo mudo, §14).
- [x] CI en `.github/workflows/`: `cargo fmt --check`, `cargo clippy` sin
      advertencias, `cargo test` en cada push (§7).

## 2. Código (§13 — estándares)

- Registro §2: primero se limpia lo verificable por herramientas porque un
  catálogo que enseña Rust debe mostrar código sin ruido de linter; la
  alternativa era justificar advertencias globalmente, pero solo se conserva la
  excepción local de RAII donde el `return` temprano es parte de la lección.

- [x] Limpiar las 28 advertencias actuales de `cargo clippy --all-targets`
      (`needless_return`, `default_constructed_unit_structs`,
      `should_implement_trait`, `type_complexity`, `useless_vec`,
      `collapsible_if`) o justificar por escrito cada excepción que se
      mantenga.
- [ ] Agregar doc-comments (`///`) a la API pública de cada patrón: qué hace,
      un ejemplo ejecutable (doctest) y notas de complejidad cuando aplique.
      Hoy prácticamente ningún `pub fn`/`pub struct` los tiene.
- [ ] Decidir y documentar, patrón por patrón (o por familia), si los
      benchmarks aplican. Donde no apliquen, declararlo explícitamente en el
      README del patrón en vez de omitirlo en silencio (§14). Donde sí
      apliquen (p. ej. Cache Aside, Flyweight, Object Pool-like en Singleton),
      agregar `criterion` y una carpeta `benches/`.
- [ ] Misma decisión y documentación para property testing (`proptest`): dónde
      aporta (p. ej. Newtype, Interpreter, Rate Limiting) y dónde se declara
      que no aplica.

## 3. Consistencia con lo ya construido

- [ ] Una vez agregado `AGENTS.md`/`ROADMAP.md`, revisar que el README
      principal siga apuntando a las mismas fuentes de verdad sin quedar
      desactualizado (mismo hábito de auditoría que ya usa
      `plan/catalogo-patrones-rust.md`).
- [ ] Confirmar que `Cargo.toml` (`license`) y el nuevo `LICENSE.md` no se
      contradigan.

## Fuera de este checklist

Contenido nuevo (patrones 53+, una Fase 5) no es parte de esta brecha — es
expansión de alcance, no cumplimiento de estándar, y se decide aparte.
