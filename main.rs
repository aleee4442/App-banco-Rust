// Libreria utilziada para leer los imputs del usuario
use std::io;

// Estructura de la persona con su nombre y telefono
struct Persona{
    nombre: String,
    telefono: Option<String>,
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
    }
    // Funcion para aniadir tantos usuarios como quieras
    fn new_user()-> Persona{ 
        let mut nombre = String::new();
        let mut telefono = String::new();

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

        Persona{nombre,telefono}
    }

}

fn main() {
    let persona1 = Persona::new_user();
    let persona2 = Persona::new_user();

    persona1.print_user();
    persona2.print_user();
}
