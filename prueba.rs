use std::io::{self, Write};

// Función que lee la entrada del usuario y devuelve un mensaje procesado
fn leer_entrada(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de leer

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Usamos trim para eliminar espacios en blanco y newlines al principio y final
    let input = input.trim().to_string();

    // Si la entrada está vacía, devolvemos None, de lo contrario, Some(input)
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

fn main() {
    // Llamamos a la función leer_entrada y guardamos el mensaje procesado
    let user_input: Option<String> = leer_entrada("Por favor, ingresa algo: ");

    // // Verificamos si el usuario ingresó algo
    // match &user_input {
    //     Some(input) => println!("El usuario ingresó: {}", input),
    //     None => println!("No se ingresó ningún valor."),
    // }
    println!("{}",Some(input));

    // Usamos el valor ingresado más adelante en el programa
    if let Some(input) = user_input {
        println!("Usando el valor ingresado más adelante: {}", input);
        // Aquí puedes hacer lo que necesites con el valor de `input`
    } else {
        println!("No hay valor para usar más adelante.");
    }
}