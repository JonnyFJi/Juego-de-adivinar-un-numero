use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main() {
    println!("ADIVINA EL NÚMERO del 1 al 1000!");

    let secret = rand::thread_rng().gen_range(1..=1000);

//    println!("El número secreto es: {secret}");

    loop {
        println!("Por favor, ingresa un número.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo leyendo el número");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

//        println!("Lo adivinaste?: {guess}?");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Oh, muy pequeño!"),
            Ordering::Greater => println!("Oh, muy grande!"),
            Ordering::Equal => {
                println!("Felicidades, ganaste!");
                break;
            }
        }
    }
}
// Jonny Ji 2025