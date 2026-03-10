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
- Organiza las conversiones en funciones separadas

Ejemplos
cargo run -- 100 km mi       # 100 kilómetros a millas
cargo run -- 37 c f          # 37 grados Celsius a Fahrenheit
cargo run -- 5 kg lb         # 5 kilogramos a libras
*/

use std::env;

fn main() {

    // Argumentos
    let args: Vec<String> = env::args().collect();

    let num = &args[1];

    let inicio = &args[2];

    let fin = &args[3];
    

    // Salida
    println!("Unidad Inicio {}", num);
    println!("Unidad Inicio {}", inicio);
    println!("Unidad Final {}", fin);
}
