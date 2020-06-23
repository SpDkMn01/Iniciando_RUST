use std::io;

fn main() {
    println!("Adivina el numero!");

    println!("Por favor introduce un numero:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al leer la linea");

    println!("Tu numero: {}", guess);
}
