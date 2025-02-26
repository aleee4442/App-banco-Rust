// Libreria utilziada para leer los imputs del usuario
// use std::io;
use std::io::{self, Write};
use rand::Rng;

// Estructura de la persona con su nombre y telefono
struct Persona{
    nombre: String,
    telefono: Option<String>,
    tarjeta: Option<String>,
    edad: u32
    
}

// Impl con las funciones relacionadas con Persona
impl Persona {
    // Funcion para imprimir la informacion del usuario
    fn print_user(&self) {
        println!("Nombre: {}", self.nombre);
        match &self.telefono {
            Some(tel) => println!("Telefono: {}", tel),
            None => println!("Telefono: No tiene teléfono"),
        }
        println!("Tarjeta: {:?}", self.tarjeta);
        println!("Edad: {}", self.edad);
    }
    // Funcion para aniadir tantos usuarios como quieras
    fn new_user()-> Persona{ 
        let mut nombre = String::new();
        let mut telefono:Option<String> = None;
        let mut tarjeta = None;
        let mut edad = String::new();
        let mut prueba = String::new();
        
        println!("Introduce tu nombre");
        io::stdin().read_line(&mut nombre)
        .expect("Error al leer la entrada");
        
        println!("Introduce tu telefono (opcional)");
        io::stdin().read_line(&mut telefono)        
        .expect("Failed to read line");
        
        // Borro los elementos en blanco por si el usuario no introduce nada salga None
        let telefono = telefono.trim().to_string();

        // Compruebo si el usuario ha anidido algo, sino lo dejo vacio
        let telefono = if telefono.is_empty(){
            None
        }else{
            Some(telefono)
        };

        println!("Tienes alguna tarjeta ya creada(Y/N)");
        io::stdin().read_line(&mut prueba)        
        .expect("Failed to read line");
        if prueba == "Y" || prueba == "y"{
            tarjeta = leer_entrada("Introduce la tarjeta")
        }else{
            println!("Generando Tarjeta");
            tarjeta = crear_tarjeta()
        }        

        println!("Introduce tu edad");
        io::stdin().read_line(&mut edad)        
        .expect("Failed to read line");

        // pasar de string a numero
        let edad = edad.trim().parse::<u32>().expect("Error, numero no introducido");

        Persona{nombre,telefono, tarjeta, edad}
    }

    fn crear_tarjeta()-> Option<String>{
        let mut rng = rand::rng(); // Generador de números aleatorios

        // Crear un vector vacío para almacenar los 16 números aleatorios
        let mut digits: Vec<u32> = Vec::new();

        for _ in 0..16{
            let digit:u32 = rng.random_range(0..10);
            digits.push(digit);
        }
        
        // Convertir los dígitos a un solo número concatenado
        let number: String = digits.iter().map(|&digit| digit.to_string()).collect();
        number
    }

}

fn leer_entrada(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de leer

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Si la entrada no está vacía, la devolvemos como Some
    if !input.trim().is_empty() {
        Some(input.trim().to_string())
    } else {
        None
    }
}

fn main() {
    let persona1 = Persona::new_user();
    let persona2 = Persona::new_user();

    persona1.print_user();
    persona2.print_user();
}
