/*
Objetivo: practicar funciones, tipos, match y formateo.

Crea un programa que convierta entre unidades. 
Debe aceptar tres argumentos por línea de comandos: valor, origen, destino.

Requisitos:

- Conversiones a implementar: km ↔ mi | m ↔ ft | kg ↔ lb | c ↔ f | l ↔ gal |
- Si las unidades no son válidas, muestra un error 
    descriptivo y sal con código 1
- Formatea el resultado con 4 decimales
- Usa match para despachar la conversión correcta

Ejemplos
cargo run -- 100 km mi       # 100 kilómetros a millas
cargo run -- 37 c f          # 37 grados Celsius a Fahrenheit
cargo run -- 5 kg lb         # 5 kilogramos a libras
*/

use std::env;

fn main() {

    // Argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Uso: {} <num> <unidad inicio> <unidad final>", args[0]);
        std::process::exit(1);
    }

    let mut num : f64 = args[1].parse().unwrap();
    let num_original = num;

    let conversion = (args[2].as_str(), args[3].as_str());

    // Match
    match conversion {
        ("km", "mi") => num *= 0.621371,
        ("mi", "km") => num *= 1.60934,
        ("m", "ft") => num *= 3.28084,
        ("ft", "m") => num *= 0.3048,
        ("kg", "lb") => num *= 2.2046,
        ("lb", "kg") => num *= 0.453592,
        ("c", "f") => num = (num * 9.0/5.0) + 32.0,
        ("f", "c") => num = (num - 32.0) * 5.0/9.0,
        ("l", "gal") => num *= 0.264172,
        ("gal", "l") => num *= 3.785,
        _ => {
            eprintln!("Tipos incorrectos");
            std::process::exit(1);
        }
    }

    // Salida
    println!("{} {}", num_original, args[2].as_str());
    println!("{:.4} {}", num, args[3].as_str());
}
