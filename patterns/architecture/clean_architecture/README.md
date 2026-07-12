# Clean Architecture

## Intención

Clean Architecture organiza el sistema alrededor de reglas de negocio independientes. Las entidades y los casos de uso no conocen frameworks, bases de datos, HTTP, CLI ni detalles de infraestructura; esos detalles viven en adaptadores externos.

## Problema cotidiano

En sistemas reales es común que la lógica importante quede atrapada en controladores, handlers HTTP, ORMs o SDKs externos:

- Validar una cuenta depende de un framework web.
- Un caso de uso recibe objetos propios de una librería HTTP.
- Las reglas de negocio no se pueden probar sin levantar infraestructura.
- Cambiar la salida de HTTP a CLI obliga a tocar el núcleo.

Clean Architecture invierte esas dependencias. El núcleo define lo que necesita y los adaptadores traducen hacia adentro o hacia afuera.

## Partes principales

- Entidades: reglas de negocio esenciales y estructuras del dominio.
- Casos de uso: flujos de aplicación que coordinan entidades y puertos.
- Gateways: traits que representan persistencia, servicios externos o recursos.
- Controladores: adaptadores de entrada que traducen una petición externa a un caso de uso.
- Presenters: adaptadores de salida que convierten resultados del caso de uso en formatos concretos.

## Cómo se ve en Rust

Rust encaja bien con esta arquitectura porque permite expresar límites con módulos, traits y tipos explícitos:

```rust
trait UserGateway {
    fn save(&mut self, user: NewUser) -> RegisteredUser;
}
```

El caso de uso depende del trait, no de una base de datos concreta. El controlador y el presenter se quedan en la orilla externa.

## Cuándo usarlo

- Cuando el dominio debe probarse sin infraestructura.
- Cuando una misma regla debe exponerse por HTTP, CLI, jobs o mensajes.
- Cuando quieres que los casos de uso sean el centro del diseño.
- Cuando el sistema crecerá con varios adaptadores.

## Cuándo evitarlo

- Si solo hay una operación pequeña y la separación agrega ceremonia.
- Si los límites se vuelven carpetas vacías sin reglas reales.
- Si el equipo no mantiene la regla de dependencias hacia el núcleo.
- Si cada cambio simple exige navegar demasiadas capas.

## Ejemplos

- [x] Entidades, casos de uso, gateways y controladores.
- [x] Reglas de negocio independientes de framework.
- [ ] Presenter para respuesta HTTP y respuesta CLI.

### Entidades, casos de uso, gateways y controladores

El módulo `registration_flow` modela un registro de usuario. La entidad valida el nombre y el email, el caso de uso coordina la creación, el gateway asigna identidad y persiste en memoria, y el controlador traduce una petición externa a una respuesta con código de estado.

La dependencia apunta hacia el núcleo: el controlador conoce al caso de uso, el caso de uso conoce el trait del gateway y la entidad no depende de ningún adaptador.

### Reglas de negocio independientes de framework

El módulo `framework_independent_rules` calcula descuentos de carrito usando entidades y casos de uso propios del núcleo. La regla de descuento recibe un `Cart`, una `DiscountPolicy` y devuelve un resultado sin usar tipos HTTP.

El controlador HTTP del ejemplo solo transforma DTOs externos en tipos del caso de uso y formatea la respuesta. Si mañana el mismo caso de uso se expone por CLI, job o cola de mensajes, la regla no cambia.

## Comandos

```bash
cargo test clean_architecture
```
