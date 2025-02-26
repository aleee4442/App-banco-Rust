# Aplicación de Banco en Rust (ficticia)

>[!NOTE]
>Esto es una prueba de proyecto inventado por mi mismo mientras aprendía rust
>Es una gestión bancaria sencilla y eficiente en Rust

## Descripción

Este proyecto es una aplicación de banco desarrollada en Rust que permite la gestión de usuarios con funcionalidades como:

Creación de cuentas.

Consulta de saldo e información personal.

Gestión de saldo (añadir o retirar dinero).

## Características

-Crear nuevos usuarios con:
  -Nombre
  -Teléfono (opcional)
  -Número de tarjeta (generado aleatoriamente si no se proporciona)
  -Edad
  -Saldo inicial
-Consultar información del usuario.
-Ver saldo disponible.
-Añadir o retirar saldo.
-Interfaz de línea de comandos intuitiva.

## Requisitos
Para ejecutar este proyecto, necesitarás:
-Rust instalado en tu sistema.
-Cargo para compilar y ejecutar el proyecto.

## Instalación
### Clonar el repositorio
git clone https://github.com/tu-usuario/tu-repo.git
cd tu-repo

### Compilar y ejecutar el programa
cargo run

## Uso

Al ejecutar la aplicación, se mostrará un menú con las siguientes opciones:
-Agregar usuario
-Ver saldo de un usuario
-Ver información del usuario
-Añadir/Quitar saldo
-Salir

Introducir la opción deseada y seguir las instrucciones en pantalla.

>[!TIP]
> Puedes probar con diferentes usuarios y manejar múltiples cuentas simultáneamente.

## Librerías utilizadas
-std::io para entrada y salida de datos.
-rand para generación de números aleatorios.
-std::thread y std::time::Duration para pausas en la ejecución.

Este proyecto me ha servido para comprender más a fondo como funciona rust teniendo que investigar bastante por mi cuenta
