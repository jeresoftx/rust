# Adapter

Adapter permite usar un componente existente a través de la interfaz que el sistema actual espera.

## Idea central

El adapter recibe llamadas en el lenguaje del dominio interno y las traduce hacia una API, formato o librería externa. También puede convertir la respuesta de vuelta a tipos propios.

En Rust suele aparecer como:

- Structs envoltorio alrededor de clientes externos.
- Implementaciones de traits propios para adaptadores.
- Conversiones explícitas con `From` o `TryFrom` cuando puede fallar.

## Ejemplos del repositorio

- Cliente externo de pagos adaptado a una interfaz interna.
- Formato legacy de usuario adaptado a un modelo nuevo.
- Logger de terceros adaptado a un trait propio.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/adapter/README.md`
