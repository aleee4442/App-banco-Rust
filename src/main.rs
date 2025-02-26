// Librerias utilizadas en el programa

use std::io::{self, Write}; // Libreria para leer los inputs del usuario
use rand::Rng; // Libreria para generar números aleatorios
use std::thread; // Libreria para pausar el programa
use std::time::Duration; // Libreria para pausar el programa

// Estructura de la persona con su nombre, telefono, tarjeta, edad y saldo
struct Persona{
    nombre: String,
    telefono: Option<String>, // Como es opcional es un Option<String>
    tarjeta: String, 
    edad: String, // como solo pido la edad para tener datos lo hago un string y no lo paso a numerico
    saldo:f64 // posibilidad de que tengas un saldo decimal
}

// Impl con las funciones relacionadas con Persona
impl Persona {
    // Funcion para imprimir la informacion del usuario
    fn print_user(&self) {
        println!("Nombre: {}", self.nombre); // Imprimir nombre
        match &self.telefono { // Comprobar si el telefono es opcional
            Some(tel) => println!("Telefono: {}", tel), // Si tiene telefono se imprime
            None => println!("Telefono: No tiene teléfono"), // Si no tiene telefono se imprime lo siguiente
        }
        println!("Tarjeta: {:?}", self.tarjeta); // Imprimir tarjeta
        println!("Edad: {}", self.edad); // Imprimir edad
        println!("Saldo: {:.2}€", self.saldo); // Imprimir saldo
    }
    // Funcion para aniadir tantos usuarios como quieras
    fn new_user()-> Persona{ 

        // Variables para almacenar los datos del usuario
        let mut nombre = String::new();
        let telefono:Option<String>;
        let mut tarjeta = String::new();
        let mut edad = String::new();
        let mut prueba = String::new();
        
        // Introducir el nombre del usuario
        println!("Introduce tu nombre");
        io::stdin().read_line(&mut nombre)
        .expect("Error al leer la entrada");

        // Introducir el telefono del usuario
        telefono = leer_entrada("Introduce tu telefono (opcional)");

        // Introducir si tiene tarjeta o no
        println!("Tienes alguna tarjeta ya creada(Y/N)");
        io::stdin().read_line(&mut prueba)        
        .expect("Failed to read line");
        if prueba == "Y" || prueba == "y"{
            io::stdin().read_line(&mut nombre) // Si tiene tarjeta leerla y guardarla
            .expect("Error al leer la entrada");
        }else{
            println!("Generando Tarjeta"); // Si no tiene tarjeta generar una
            tarjeta = crear_tarjeta();
        }        
        
        // Introducir la edad del usuario
        println!("Introduce tu edad");
        io::stdin().read_line(&mut edad)        
        .expect("Failed to read line");
        
        // Introducir el saldo inicial del usuario para pruebas en el codigo
        let mut saldo_string = String::new();
        println!("Introduce el saldo inicial");
        io::stdin().read_line(&mut saldo_string)        
        .expect("Failed to read line");
        let saldo = saldo_string.trim().parse::<f64>().expect("Error, numero no introducido");

        Persona{nombre,telefono, tarjeta, edad, saldo}
    }

    // Función para mostrar el saldo de un usuario
    fn mostrar_saldo(&self) {
        println!("Saldo de {}: {:.2}€", self.nombre, self.saldo);
    }
}

// Función para crear una tarjeta aleatoria
fn crear_tarjeta()-> String{
    let mut rng = rand::rng(); // Generador de números aleatorios

    //  Crear un vector vacío para almacenar los 16 números aleatorios
    let mut digits: Vec<u32> = Vec::new();

    // Generar 16 números aleatorios y los aniado al vector
    for _ in 0..16{
        let digit:u32 = rng.random_range(0..10);
        digits.push(digit);
    }
    
    // Uno todo lo del vector a un solo String
    let number: String = digits.iter().map(|&digit| digit.to_string()).collect();
    number // Devuelvo el string
}

// Función para leer la entrada del usuario
fn leer_entrada(mensaje: &str) -> Option<String> {
    print!("{}", mensaje);
    io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de leer

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Elimino espacios en blanco y saltos de línea al principio y final
    let input = input.trim().to_string();

    if input.is_empty() {
        None  // Si la entrada esta vacia devuelvo None
    } else {
        Some(input) // Si hay algo devuelvo el input
    }
}

// Función para mostrar el menu y obtener la opción del usuario
fn menu(usuarios: &mut Vec<Persona>) -> u32 {
    println!("----- Menu de la aplicación del banco -----");
    println!("1: Agregar usuario");               
    thread::sleep(Duration::from_millis(100)); // Pausa de 100 milisegundos          
    println!("2: Ver saldo de un usuario");                
    thread::sleep(Duration::from_millis(100)); // Pausa de 100 milisegundos
    println!("3: Ver informacion del usuario");
    thread::sleep(Duration::from_millis(100)); // Pausa de 100 milisegundos
    println!("4: Añadir/Quitar saldo");
    thread::sleep(Duration::from_millis(100)); // Pausa de 100 milisegundos
    println!("5: Salir");                                  
    println!("------------------------------------------");

    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion).expect("Failed to read line");

    match opcion.trim().parse::<u32>() {
        Ok(1) => {
            let usuario = Persona::new_user();
            usuarios.push(usuario);
            println!("Usuario creado exitosamente.");
            0 // Devuelve 0 para indicar que no se ha salido del programa
        }
        Ok(2) => {
            if usuarios.is_empty() {
                println!("No hay usuarios creados.");
            } else {
                println!("Selecciona un usuario:");
                for (i, usuario) in usuarios.iter().enumerate() {
                    println!("{}. {}", i + 1, usuario.nombre);
                }

                let mut seleccion = String::new();
                io::stdin().read_line(&mut seleccion).expect("Failed to read line");
                let seleccion: usize = seleccion.trim().parse().unwrap_or(0);

                if seleccion > 0 && seleccion <= usuarios.len() {
                    usuarios[seleccion - 1].mostrar_saldo();
                } else {
                    println!("Selección no válida.");
                }
            }
            0 // Devuelve 0 para indicar que no se ha salido del programa
        }
        Ok(3) => {
            if usuarios.is_empty() {
                println!("No hay usuarios creados.");
            } else {
                println!("Selecciona un usuario:");
                for (i, usuario) in usuarios.iter().enumerate() {
                    println!("{}. {}", i + 1, usuario.nombre);
                }

                let mut seleccion = String::new();
                io::stdin().read_line(&mut seleccion).expect("Failed to read line");
                let seleccion: usize = seleccion.trim().parse().unwrap_or(0);

                if seleccion > 0 && seleccion <= usuarios.len() {
                    usuarios[seleccion - 1].print_user();
                } else {
                    println!("Selección no válida.");
                }
            }
            0 // Devuelve 0 para indicar que no se ha salido del programa
        }
        Ok(4) => {
            if usuarios.is_empty() {
                println!("No hay usuarios creados.");
            } else {
                println!("Selecciona un usuario:");
                for (i, usuario) in usuarios.iter().enumerate() {
                    println!("{}. {}", i + 1, usuario.nombre);
                }

                let mut seleccion = String::new();
                io::stdin().read_line(&mut seleccion).expect("Failed to read line");

                let seleccion: usize = seleccion.trim().parse().unwrap_or(0);

                if seleccion > 0 && seleccion <= usuarios.len() {
                    println!("Quieres añadir (1) o quitar saldo(2)?");
                    let mut seleccion_saldo = String::new();
                    io::stdin().read_line(&mut seleccion_saldo).expect("Failed to read line");
                    if seleccion_saldo == "1" {
                        println!("Introduce el importe a añadir");
                        let mut importe = String::new();
                        io::stdin().read_line(&mut importe).expect("Failed to read line");
                        usuarios[seleccion - 1].saldo += importe.trim().parse::<f64>().expect("Error, numero no introducido");
                        usuarios[seleccion - 1].mostrar_saldo();
                    } else if seleccion_saldo == "2" {
                        println!("Introduce el importe a quitar");
                        let mut importe = String::new();
                        io::stdin().read_line(&mut importe).expect("Failed to read line");
                        usuarios[seleccion - 1].saldo -= importe.trim().parse::<f64>().expect("Error, numero no introducido");
                        usuarios[seleccion - 1].mostrar_saldo();
                    } else {
                        println!("Selección no válida.");
                    }
                }
            }
            0 // Devuelve 0 para indicar que no se ha salido del programa
        }
        Ok(5) => {
            println!("Saliendo del programa...");
            5 // Devuelve 5 para indicar que se debe salir del programa
        }
        Ok(_) => {
            println!("Opción no válida.");
            0 // Devuelve 0 para indicar que no se ha salido del programa
        }
        Err(_) => {
            println!("Opción no válida.");
            0 // Devuelve 0 para indicar que no se ha salido del programa
        }
    }
}

fn main() {
    // Vector para almacenar a todos los usuarios
    let mut usuarios: Vec<Persona> = Vec::new();

    println!("¡Bienvenido a la aplicación del banco!");

    loop {
        // Llamada a la función menu con una referencia mutable
        let solucion = menu(&mut usuarios);
        if solucion == 5 { break; } // Salir del bucle si se devuelve 5
    }
}