# Facade

## Intención

Facade ofrece una interfaz simple para usar un subsistema compuesto por varias piezas.

## Problema cotidiano

En sistemas reales una operación de alto nivel suele coordinar varios componentes:

- Un checkout usa carrito, pagos e inventario.
- Una notificación puede enviarse por varios canales.
- Un reporte necesita cargar datos, calcularlos y renderizarlos.

Si el código cliente conoce todos esos pasos, termina acoplado al detalle interno del subsistema. Facade concentra la coordinación detrás de una API pequeña.

## Cómo se ve en Rust

En Rust, Facade suele modelarse como una struct de servicio que posee o recibe los componentes internos y expone métodos orientados al caso de uso. La fachada no debe esconder errores importantes; debe convertirlos en resultados claros para el cliente.

## Cuándo usarlo

- Cuando un flujo usa varios componentes internos.
- Cuando quieres simplificar la API pública de un módulo.
- Cuando necesitas proteger al cliente de cambios internos del subsistema.

## Cuándo evitarlo

- Si la fachada se vuelve un objeto gigante con demasiadas responsabilidades.
- Si solo delega un método sin simplificar nada.
- Si oculta errores o decisiones que el cliente sí necesita conocer.

## Diferencia con Adapter

Adapter convierte una interfaz incompatible. Facade simplifica el acceso a un subsistema que puede tener muchas interfaces internas.

## Ejemplos

- [x] Servicio de checkout que coordina carrito, pago e inventario.
- [ ] API simple para enviar notificaciones multicanal.
- [ ] Generador de reporte que oculta carga, cálculo y render.

### Checkout

El módulo `checkout` expone `CheckoutFacade` para coordinar carrito, inventario y pagos.

El ejemplo muestra cómo el cliente ejecuta un checkout con un solo método y recibe errores de negocio sin conocer los subsistemas internos.

## Comandos

```bash
cargo test facade
```
