# Fundamentos del Lenguaje

## Índice

1. [Cargo y estructura de un proyecto](#1-cargo-y-estructura-de-un-proyecto)
2. [Variables y mutabilidad](#2-variables-y-mutabilidad)
3. [Tipos de datos](#3-tipos-de-datos)
4. [Funciones](#4-funciones)
5. [Control de flujo](#5-control-de-flujo)
6. [Strings](#6-strings)
7. [I/O básica](#7-io-básica)
8. [Proyectos intermedios](#-proyectos-intermedios)
9. [Proyecto final](#-proyecto-final-calculadora-repl)

---

## 1. Cargo y estructura de un proyecto

Cargo es el gestor de paquetes y sistema de build de Rust. Es la herramienta central con la que trabajarás cada día.

### Crear un proyecto

```bash
cargo new mi_proyecto        # Crea un binario (ejecutable)
cargo new mi_lib --lib       # Crea una librería
cargo new mi_proyecto --vcs none  # Sin inicializar git
```

Esto genera la siguiente estructura:

```
mi_proyecto/
├── Cargo.toml       # Metadatos y dependencias del proyecto
├── Cargo.lock       # Versiones exactas resueltas (no tocar a mano)
└── src/
    └── main.rs      # Punto de entrada del programa
```

### Cargo.toml

```toml
[package]
name    = "mi_proyecto"
version = "0.1.0"
edition = "2021"        # La edición de Rust (usa siempre 2021)

[dependencies]
rand = "0.8"            # Añadir una dependencia externa
```

### Comandos esenciales

```bash
cargo build             # Compila en modo debug  (target/debug/)
cargo build --release   # Compila optimizado     (target/release/)
cargo run               # Compila y ejecuta
cargo run -- arg1 arg2  # Ejecuta con argumentos
cargo check             # Verifica errores SIN compilar (más rápido)
cargo test              # Ejecuta los tests
cargo doc --open        # Genera y abre la documentación
cargo fmt               # Formatea el código
cargo clippy            # Linter con sugerencias avanzadas
```

> **Regla de oro:** usa `cargo check` mientras escribes. Es mucho más rápido que `cargo build` y te da los errores del compilador al instante.

### Dependencias

Para añadir una dependencia puedes editar `Cargo.toml` a mano o usar:

```bash
cargo add rand          # Añade la última versión de 'rand'
cargo add serde --features derive
```

---

## 2. Variables y mutabilidad

### `let` — inmutable por defecto

En Rust, **las variables son inmutables por defecto**. Esto no es un accidente: obliga a ser explícito cuando algo va a cambiar.

```rust
fn main() {
    let x = 5;
    println!("x vale {}", x);

    // x = 6; // ❌ ERROR: cannot assign twice to immutable variable
}
```

### `let mut` — mutable

```rust
fn main() {
    let mut contador = 0;
    contador += 1;
    contador += 1;
    println!("contador = {}", contador); // 2
}
```

### Shadowing — redeclarar una variable

Shadowing permite redeclarar una variable con el mismo nombre. Es diferente a `mut`: crea una variable nueva y puede cambiar el tipo.

```rust
fn main() {
    let x = 5;
    let x = x + 1;       // Nueva variable 'x', shadow de la anterior
    let x = x * 2;
    println!("x = {}", x); // 12

    // Con shadowing puedes cambiar el tipo:
    let texto = "42";
    let texto = texto.parse::<i32>().unwrap(); // ahora texto es i32
    println!("texto = {}", texto); // 42
}
```

### `const` — constantes

Las constantes existen durante toda la vida del programa. Su tipo **siempre debe anotarse** y solo pueden ser expresiones evaluables en compilación.

```rust
const MAX_PUNTOS: u32 = 100_000;  // _ como separador visual, válido en Rust
const PI: f64 = 3.14159265358979;
```

### `static` — variable global

Similar a `const` pero tiene una dirección de memoria fija. Las mutables (`static mut`) requieren `unsafe`.

```rust
static SALUDO: &str = "Hola, mundo";

fn main() {
    println!("{}", SALUDO);
}
```

---

## 3. Tipos de datos

Rust es un lenguaje de **tipado estático**: todos los tipos se conocen en compilación. Sin embargo, el compilador puede inferirlos en la mayoría de los casos.

### Enteros

| Tamaño   | Con signo | Sin signo |
| -------- | --------- | --------- |
| 8 bits   | `i8`      | `u8`      |
| 16 bits  | `i16`     | `u16`     |
| 32 bits  | `i32`     | `u32`     |
| 64 bits  | `i64`     | `u64`     |
| 128 bits | `i128`    | `u128`    |
| Nativo   | `isize`   | `usize`   |

- El tipo por defecto cuando no se especifica es `i32`.
- `usize` e `isize` dependen de la arquitectura (64 bits en sistemas modernos). Se usan para índices y tamaños de colecciones.

```rust
fn main() {
    let a: i32  = -42;
    let b: u8   = 255;
    let c: i64  = 9_000_000_000;
    let d       = 100;       // inferido como i32

    // Literales en distintas bases:
    let hex  = 0xFF;         // 255
    let octal = 0o77;        // 63
    let bin  = 0b1111_0000;  // 240
    let byte: u8 = b'A';     // 65

    println!("{} {} {} {}", a, b, c, d);
}
```

**Overflow:** en debug, el overflow hace panic. En release, hace wrapping. Si necesitas wrapping explícito usa los métodos `wrapping_add`, `checked_add`, `saturating_add`.

### Flotantes

```rust
fn main() {
    let x = 2.0;          // f64 por defecto
    let y: f32 = 3.14;

    println!("{:.4}", x);  // 2.0000 — formateo con 4 decimales
    println!("{:e}", y);   // notación científica

    // Operaciones:
    let resultado = (2.0_f64).sqrt();     // 1.414...
    let potencia  = 2.0_f64.powi(10);    // 1024.0 (exp entera)
    let potencia2 = 2.0_f64.powf(0.5);   // 1.414... (exp flotante)
}
```

### Booleanos

```rust
fn main() {
    let verdadero: bool = true;
    let falso = false;

    println!("{}", verdadero && falso);  // false
    println!("{}", verdadero || falso);  // true
    println!("{}", !verdadero);          // false
}
```

### `char` — carácter Unicode

En Rust, `char` ocupa **4 bytes** y representa un scalar value Unicode. No es un byte como en C.

```rust
fn main() {
    let letra = 'a';
    let emoji: char = '🦀';
    let kanji = '漢';

    println!("{} {} {}", letra, emoji, kanji);
    println!("Tamaño de char: {} bytes", std::mem::size_of::<char>()); // 4
}
```

### Tuplas

Agrupan valores de **tipos distintos**. Tamaño fijo. Acceso por índice con `.0`, `.1`, etc.

```rust
fn main() {
    let persona: (&str, u8, f64) = ("Alice", 30, 1.75);

    // Acceso por índice:
    println!("Nombre: {}", persona.0);
    println!("Edad:   {}", persona.1);

    // Destructuring:
    let (nombre, edad, altura) = persona;
    println!("{} tiene {} años", nombre, edad);

    // Tupla unitaria () — el tipo "vacío" de Rust
    let vacia: () = ();
}
```

### Arrays

Tamaño **fijo en compilación**, todos los elementos del mismo tipo. Se almacenan en el stack.

```rust
fn main() {
    let nums = [1, 2, 3, 4, 5];       // [i32; 5]
    let ceros = [0; 10];               // 10 ceros — [0, 0, 0, ...]

    println!("Longitud: {}", nums.len());
    println!("Primer elemento: {}", nums[0]);

    // Iterar:
    for n in &nums {
        print!("{} ", n);
    }

    // Slices de arrays:
    let mitad = &nums[1..4];  // [2, 3, 4]
    println!("\nMitad: {:?}", mitad);
}
```

> Para colecciones de tamaño dinámico usa `Vec<T>` (lo veremos en el módulo 05).

### Inferencia de tipos

Rust puede inferir tipos en contextos donde hay suficiente información:

```rust
fn main() {
    let mut v = Vec::new();  // Rust aún no sabe el tipo
    v.push(1_i32);           // Ahora sabe que es Vec<i32>

    // También a través del uso posterior:
    let numeros = vec![1, 2, 3];      // Vec<i32>
    let suma: i64 = numeros.iter().map(|&x| x as i64).sum();
}
```

---

## 4. Funciones

### Declaración básica

```rust
fn saludar(nombre: &str) -> String {
    format!("Hola, {}!", nombre)   // Sin punto y coma = expresión de retorno
}

fn suma(a: i32, b: i32) -> i32 {
    a + b   // La última expresión sin ; es el valor de retorno
}

fn main() {
    let msg = saludar("Alice");
    println!("{}", msg);
    println!("2 + 3 = {}", suma(2, 3));
}
```

### Expresiones vs sentencias

Esta distinción es fundamental en Rust:

- **Sentencia (statement):** no devuelve valor. Termina con `;`.
- **Expresión (expression):** devuelve un valor. Sin `;`.

```rust
fn main() {
    // Esto es una sentencia:
    let x = 5;

    // Esto es un bloque-expresión que devuelve un valor:
    let y = {
        let a = 3;
        a * a + 1   // sin ; → este es el valor del bloque: 10
    };

    println!("y = {}", y); // 10
}
```

### Retorno anticipado con `return`

```rust
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return f64::NAN;  // retorno anticipado
    }
    a / b  // retorno normal por expresión final
}
```

### Múltiples valores de retorno con tuplas

Rust no tiene retorno múltiple, pero las tuplas lo simulan perfectamente:

```rust
fn min_max(v: &[i32]) -> (i32, i32) {
    let mut min = v[0];
    let mut max = v[0];
    for &n in v {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let (minimo, maximo) = min_max(&nums);
    println!("min={}, max={}", minimo, maximo);
}
```

### Funciones anidadas

```rust
fn procesar(datos: &[i32]) -> f64 {
    fn cuadrado(x: i32) -> i32 { x * x }  // función local

    let suma_cuadrados: i32 = datos.iter().map(|&x| cuadrado(x)).sum();
    suma_cuadrados as f64 / datos.len() as f64
}
```

---

## 5. Control de flujo

### `if` como expresión

En Rust, `if` devuelve un valor. Todas las ramas deben devolver el mismo tipo.

```rust
fn main() {
    let numero = 7;

    // if como sentencia:
    if numero % 2 == 0 {
        println!("par");
    } else if numero % 3 == 0 {
        println!("divisible por 3");
    } else {
        println!("impar");
    }

    // if como expresión (equivalente al ternario de otros lenguajes):
    let descripcion = if numero > 0 { "positivo" } else { "no positivo" };
    println!("{}", descripcion);

    // En una asignación:
    let absoluto = if numero >= 0 { numero } else { -numero };
}
```

### `loop` — bucle infinito

```rust
fn main() {
    let mut intentos = 0;

    let resultado = loop {
        intentos += 1;
        if intentos == 5 {
            break intentos * 2;  // loop también es una expresión
        }
    };

    println!("Resultado: {}", resultado); // 10
}
```

### Labels en bucles anidados

```rust
fn main() {
    'externo: for i in 0..5 {
        for j in 0..5 {
            if i + j == 6 {
                println!("Rompiendo externo en ({}, {})", i, j);
                break 'externo;  // sale del bucle externo
            }
        }
    }
}
```

### `while`

```rust
fn main() {
    let mut n = 1;

    while n < 100 {
        n *= 2;
    }

    println!("Primer potencia de 2 >= 100: {}", n); // 128
}
```

### `for` — la forma idiomática de iterar

```rust
fn main() {
    // Rango exclusivo:
    for i in 0..5 {
        print!("{} ", i);  // 0 1 2 3 4
    }

    // Rango inclusivo:
    for i in 1..=5 {
        print!("{} ", i);  // 1 2 3 4 5
    }

    // Iterar un array (con referencia para no mover):
    let frutas = ["manzana", "pera", "uva"];
    for fruta in &frutas {
        println!("{}", fruta);
    }

    // Con índice usando enumerate():
    for (i, fruta) in frutas.iter().enumerate() {
        println!("{}: {}", i, fruta);
    }

    // Iterar al revés:
    for i in (0..5).rev() {
        print!("{} ", i);  // 4 3 2 1 0
    }
}
```

### `match` — el control de flujo más poderoso de Rust

`match` es exhaustivo: el compilador obliga a cubrir todos los casos.

```rust
fn main() {
    let numero = 7;

    match numero {
        1         => println!("uno"),
        2 | 3     => println!("dos o tres"),
        4..=6     => println!("entre cuatro y seis"),
        n if n % 2 == 0 => println!("{} es par", n),  // guard
        n         => println!("{} es impar y mayor que 6", n),
    }
}
```

---

## 6. Strings

Esta es una de las partes donde Rust difiere más de otros lenguajes. Hay dos tipos principales:

### `&str` — string slice

- **Referencia prestada** a una secuencia de bytes UTF-8.
- Inmutable. Tamaño conocido en compilación (literals) o en runtime (slices de `String`).
- No tiene ownership de los datos.

```rust
fn main() {
    let literal: &str = "Hola, mundo";  // vive en el binario ('static)
    let longitud = literal.len();       // longitud en bytes, no en chars
    println!("{} tiene {} bytes", literal, longitud);
}
```

### `String` — string dinámico

- **Propiedad** de los datos. Almacenado en el heap.
- Mutable y de tamaño dinámico.

```rust
fn main() {
    // Crear:
    let mut s = String::new();
    let s2 = String::from("Hola");
    let s3 = "mundo".to_string();

    // Modificar:
    s.push_str("Hola");
    s.push(' ');           // push añade un char
    s.push_str("mundo");

    // Concatenar con + (consume el primer String):
    let s4 = s2 + ", " + &s3;  // s2 ya no es válido aquí

    // Forma idiomática sin consumir:
    let s5 = format!("{} {}", "Hola", "mundo");

    println!("{}", s);
    println!("{}", s5);
}
```

### Métodos útiles de String

```rust
fn main() {
    let texto = String::from("  Hola, Mundo!  ");

    println!("{}", texto.to_uppercase());        // "  HOLA, MUNDO!  "
    println!("{}", texto.to_lowercase());        // "  hola, mundo!  "
    println!("{}", texto.trim());                // "Hola, Mundo!"
    println!("{}", texto.contains("Mundo"));     // true
    println!("{}", texto.replace("Mundo", "Rust")); // "  Hola, Rust!  "
    println!("{}", texto.len());                 // longitud en bytes

    // Split e iteración:
    let csv = "uno,dos,tres,cuatro";
    for parte in csv.split(',') {
        println!("{}", parte);
    }

    // Parseo de tipos:
    let num: i32 = "42".parse().unwrap();
    let flotante: f64 = "3.14".parse().unwrap();
}
```

### Iterar sobre caracteres (no bytes)

```rust
fn main() {
    let texto = "Hola 🦀";

    // Iterar por chars (correcto para Unicode):
    for c in texto.chars() {
        print!("{} ", c);
    }

    println!("\nNúmero de chars: {}", texto.chars().count()); // 7
    println!("Número de bytes: {}", texto.len());             // 10
}
```

---

## 7. I/O básica

### Salida estándar

```rust
fn main() {
    let nombre = "Alice";
    let edad = 30;

    // println! — con salto de línea
    println!("Nombre: {}", nombre);
    println!("Edad: {:>5}", edad);    // alineado a la derecha en 5 chars
    println!("{:0>4}", 42);           // "0042" — relleno con ceros
    println!("{:.3}", 3.14159);       // "3.142" — 3 decimales
    println!("{:?}", [1, 2, 3]);      // debug format: [1, 2, 3]
    println!("{:#?}", (1, "hola"));   // pretty debug

    // print! — sin salto de línea
    print!("sin salto ");
    print!("de línea\n");

    // eprintln! — stderr (para logs de error)
    eprintln!("Error: algo salió mal");
}
```

### Entrada estándar

```rust
use std::io;
use std::io::Write;  // necesario para flush

fn main() {
    // Pedir input al usuario:
    print!("Introduce tu nombre: ");
    io::stdout().flush().unwrap();  // asegura que el prompt se muestra

    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre).unwrap();
    let nombre = nombre.trim();  // eliminar \n del final

    println!("Hola, {}!", nombre);

    // Leer un número:
    print!("Introduce un número: ");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    let numero: i32 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Eso no es un número válido");
            return;
        }
    };

    println!("El doble de {} es {}", numero, numero * 2);
}
```

### Argumentos de línea de comandos

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] es el nombre del programa
    if args.len() < 2 {
        eprintln!("Uso: {} <nombre>", args[0]);
        std::process::exit(1);
    }

    println!("Hola, {}!", args[1]);
}
```

---

## 🔨 Proyectos intermedios

Antes del proyecto final, completa estos ejercicios para afianzar cada concepto.

---

### 🔨 Intermedio 1 — Conversor de unidades

**Objetivo:** practicar funciones, tipos, match y formateo.

Crea un programa que convierta entre unidades. Debe aceptar tres argumentos por línea de comandos: `valor`, `origen`, `destino`.

```bash
cargo run -- 100 km mi       # 100 kilómetros a millas
cargo run -- 37 c f          # 37 grados Celsius a Fahrenheit
cargo run -- 5 kg lb         # 5 kilogramos a libras
```

**Requisitos:**

- Conversiones a implementar: `km↔mi`, `m↔ft`, `kg↔lb`, `c↔f`, `l↔gal`
- Si las unidades no son válidas, muestra un error descriptivo y sal con código 1
- Formatea el resultado con 4 decimales
- Usa `match` para despachar la conversión correcta
- Organiza las conversiones en funciones separadas

---

### 🔨 Intermedio 2 — Juego de adivinar el número

**Objetivo:** practicar bucles, stdin, parseo y lógica condicional.

Clásico juego de adivinar un número aleatorio.

```bash
🎯 Adivina el número (entre 1 y 100)
Intento 1: 50
📈 Más alto
Intento 2: 75
📉 Más bajo
Intento 3: 63
✅ ¡Correcto! Lo adivinaste en 3 intentos.
```

**Requisitos:**

- Usa la crate `rand` para generar el número (añádela a `Cargo.toml`)
- Muestra cuántos intentos ha necesitado el jugador
- Si el input no es un número válido, vuelve a preguntar sin contar el intento
- Limita a 10 intentos; si se acaban, revela el número
- Al final, pregunta si quiere jugar otra vez (`s/n`)

---

### 🔨 Intermedio 3 — Analizador de texto

**Objetivo:** practicar iteradores básicos, strings y colecciones simples.

Lee texto desde stdin línea a línea y cuando el usuario introduzca una línea vacía, muestra las estadísticas.

```
Escribe texto (línea vacía para analizar):
> Rust es un lenguaje de sistemas
> que garantiza seguridad de memoria
> sin necesitar un garbage collector
>
📊 Estadísticas:
   Líneas:           3
   Palabras:         14
   Caracteres:       88
   Palabra más larga: collector (9 letras)
   Línea más larga:  "que garantiza seguridad de memoria" (34 chars)
   Palabras únicas:  13
```

**Requisitos:**

- Cuenta líneas, palabras y caracteres (excluye espacios en la cuenta de chars)
- Encuentra la palabra más larga y la línea más larga
- Cuenta palabras únicas (ignora mayúsculas/minúsculas)
- No uses colecciones avanzadas como `HashMap` — solo `Vec`, comparaciones y loops

---

### 🔨 Intermedio 4 — Generador de tablas ASCII

**Objetivo:** practicar formateo de strings, bucles anidados y funciones.

Genera tablas formateadas en la consola.

```
cargo run -- multiplicacion 5
```

```
✖  │  1   2   3   4   5
───┼────────────────────
1  │  1   2   3   4   5
2  │  2   4   6   8  10
3  │  3   6   9  12  15
4  │  4   8  12  16  20
5  │  5  10  15  20  25
```

**Requisitos:**

- Modo `multiplicacion N`: tabla de multiplicar de NxN
- Modo `suma N`: tabla de suma de NxN
- Modo `fibonacci N`: primeros N números de Fibonacci en columnas de 5
- Alinea los números correctamente (padding dinámico según el número más grande)
- Si N > 15 para las tablas, muestra un warning (la tabla sería muy grande)

---

### 🔨 Intermedio 5 — Mini agenda de contactos

**Objetivo:** practicar structs simples, tuplas, arrays/vec y el ciclo completo de un programa interactivo.

> ⚠️ Aún no hemos visto `HashMap` ni `struct` formalmente. Usa `Vec<(String, String, String)>` para almacenar `(nombre, teléfono, email)`.

```
📒 Agenda de Contactos
======================
1. Añadir contacto
2. Buscar contacto
3. Listar todos
4. Eliminar contacto
5. Salir

Elige una opción: _
```

**Requisitos:**

- Almacena contactos como `Vec<(String, String, String)>` (nombre, tel, email)
- Búsqueda por nombre (parcial, ignora mayúsculas)
- Al eliminar, pide confirmación
- Muestra los contactos en una tabla formateada con columnas alineadas
- Valida que el email contenga `@` y que el teléfono solo tenga dígitos y `+`

---

## 🎯 Proyecto final: Calculadora REPL

Una calculadora interactiva completa en consola.

### Especificación

```
🦀 Calculadora Rust v1.0
Escribe una expresión o 'help' para ver comandos.

> 3 + 4 * 2
= 11

> (3 + 4) * 2
= 14

> 10 / 3
= 3.3333

> 2 ^ 10
= 1024

> sqrt(144)
= 12

> 15 % 4
= 3

> resultado = 3 + 4 * 2
Variable guardada: resultado = 11

> resultado * 2
= 22

> history
[1] 3 + 4 * 2 = 11
[2] (3 + 4) * 2 = 14
...

> clear
Historial borrado.

> exit
¡Hasta luego!
```

### Requisitos técnicos

- **Parser:** tokeniza la expresión en números, operadores y paréntesis
- **Evaluador:** respeta precedencia (`*`, `/`, `%` antes que `+`, `-`)
- **Paréntesis:** soporte completo
- **Operadores:** `+`, `-`, `*`, `/`, `%`, `^` (potencia)
- **Funciones:** `sqrt(x)`, `abs(x)`, `floor(x)`, `ceil(x)`
- **Variables:** permite guardar resultados con `nombre = expresión`
- **Historial:** comando `history` muestra las últimas 10 operaciones
- **Errores:** maneja división por cero, expresiones inválidas y paréntesis sin cerrar con mensajes claros
- **Comandos:** `help`, `history`, `clear`, `exit`

### Estructura sugerida

```
src/
├── main.rs          # REPL loop, lectura de stdin
├── lexer.rs         # Tokenización: "3 + 4 * 2" → [Num(3), Plus, Num(4), ...]
├── parser.rs        # Árbol de expresión respetando precedencia
└── evaluator.rs     # Evalúa el árbol, gestiona variables e historial
```

### Criterios de éxito

- [ ] Las 4 operaciones básicas funcionan
- [ ] La precedencia es correcta (`2 + 3 * 4 = 14`, no `20`)
- [ ] Los paréntesis funcionan a cualquier nivel de anidamiento
- [ ] Las variables persisten durante la sesión
- [ ] Los errores no rompen el REPL (el bucle continúa)
- [ ] El código está organizado en módulos separados
- [ ] `cargo clippy` no reporta warnings

---

<p align="center">
  ← <a href="../README.md">Volver al roadmap</a> · 
  <a href="../02-ownership-borrowing-lifetimes/">Módulo 02 →</a>
</p>
