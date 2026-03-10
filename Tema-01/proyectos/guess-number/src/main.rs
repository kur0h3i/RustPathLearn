/*
Objetivo: practicar bucles, stdin, parseo y lógica condicional.

Clásico juego de adivinar un número aleatorio.

Requisitos:

- Usa la crate rand para generar el número (añádela a Cargo.toml)
- Muestra cuántos intentos ha necesitado el jugador
- Si el input no es un número válido, vuelve a preguntar sin contar 
    el intento
- Limita a 10 intentos; si se acaban, revela el número
- Al final, pregunta si quiere jugar otra vez (s/n)

*/


use std::io;
use std::io::Write;
use rand::Rng;  

fn main() {
    // Crea numero random
    let mut rng = rand::thread_rng();
    let mut random: i32 = rng.gen_range(0..=100); // Numero aleatorio de 0 a 100
    let mut num: i32 = -1;
    let mut opcion_jugar = String::from("s");
    let mut intentos: i32 = 0;
    let max_intentos: i32 = 10;
    
    // Bucle volver a jugar
    while opcion_jugar.to_lowercase() != "n" {
        intentos = 0;
        num = -1;
        println!("Nueva partida");

        // Bucle Juego
        while num != random && intentos < max_intentos {
            // Funcion Juego => Return num 
            match juego() {
                Some(valor) => {
                    num = valor;
                    intentos += 1;
                }
                None => {
                    println!("No es un numero valido");
                    continue;
                }
            }

            // Si juego acabado -> Volver Jugar -> new rand 
            if num == random {
                println!("Enhorabuena has ganado en {} intentos!", intentos);
            } else {
                println!("Ese no es el numero");
            }
        }

        if num != random {
            println!("Se acabaron los intentos. El numero era {}", random);
        }

        opcion_jugar = opcion();
        if opcion_jugar.to_lowercase() != "n" {
            random = rng.gen_range(0..=100);
        }
    }

    println!("Gracias por jugar");
}

fn juego() -> Option<i32> {
    
    
    // pedir numero 
    print!("Introudce tu numero : ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let num: i32 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            return None;
        }
    };

    // devolver numero pedido por el usuario
    Some(num)
}

fn opcion() -> String {
    // Pedir opcion de si o no
    print!("Quieres jugar otra vez (S/n)");
    io::stdout().flush().unwrap();
    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion).unwrap();
    let opcion = opcion.trim().to_string();

    opcion
}