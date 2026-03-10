# 🦀 rust-mastery-roadmap

> Hoja de ruta completa para dominar Rust desde cero hasta nivel avanzado.  
> Un proyecto final por cada módulo para consolidar lo aprendido.

[![Rust](https://img.shields.io/badge/Rust-1.78+-CE412B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Módulos](https://img.shields.io/badge/Módulos-14-green?style=flat-square)]()
[![Proyectos](https://img.shields.io/badge/Proyectos-14-orange?style=flat-square)]()

---

## 📋 Descripción del repositorio

**Nombre:** `rust-mastery-roadmap`

**Descripción GitHub:**
> 🦀 Complete Rust learning roadmap — from fundamentals to async runtimes, unsafe code, macros and WebAssembly. One real project per module.

---

## 🗺️ Índice de módulos

| # | Módulo | Duración estimada |
|---|--------|-------------------|
| 01 | [Fundamentos del lenguaje](#01---fundamentos-del-lenguaje) | 1–2 semanas |
| 02 | [Ownership, Borrowing y Lifetimes](#02---ownership-borrowing-y-lifetimes) | 2–3 semanas |
| 03 | [Structs, Enums y Pattern Matching](#03---structs-enums-y-pattern-matching) | 1–2 semanas |
| 04 | [Traits y Genéricos](#04---traits-y-genéricos) | 2 semanas |
| 05 | [Colecciones e Iteradores](#05---colecciones-e-iteradores) | 1–2 semanas |
| 06 | [Manejo de Errores](#06---manejo-de-errores) | 1 semana |
| 07 | [Módulos, Crates y Cargo avanzado](#07---módulos-crates-y-cargo-avanzado) | 1 semana |
| 08 | [Smart Pointers](#08---smart-pointers) | 1–2 semanas |
| 09 | [Concurrencia](#09---concurrencia) | 2 semanas |
| 10 | [Async / Await](#10---async--await) | 2–3 semanas |
| 11 | [Macros](#11---macros) | 1–2 semanas |
| 12 | [Unsafe y FFI](#12---unsafe-y-ffi) | 1–2 semanas |
| 13 | [Testing y Benchmarking](#13---testing-y-benchmarking) | 1 semana |
| 14 | [WebAssembly](#14---webassembly) | 1–2 semanas |

> **Total estimado:** 5–7 meses a ritmo constante.

---

## 📌 Cómo usar este roadmap

1. Sigue los módulos **en orden**, cada uno construye sobre el anterior.
2. No avances hasta haber completado el **proyecto final** del módulo.
3. Usa `cargo clippy` y `cargo fmt` desde el primer día.
4. Ante la duda, consulta [The Rust Book](https://doc.rust-lang.org/book/) y [Rust by Example](https://doc.rust-lang.org/rust-by-example/).

---

## 01 — Fundamentos del lenguaje

### Temas

- **Cargo:** `cargo new`, `cargo run`, `cargo build`, `cargo check`, `cargo doc`. Estructura de `Cargo.toml`.
- **Variables y mutabilidad:** `let` (inmutable por defecto), `let mut`, `const`, `static`, shadowing.
- **Tipos escalares:** enteros `i8`–`i128` / `u8`–`u128`, `f32`/`f64`, `bool`, `char` (Unicode completo).
- **Tipos compuestos:** tuplas `(i32, f64, bool)`, arrays `[T; N]`. Diferencia stack vs heap.
- **Control de flujo:** `if`/`else` como expresión, `loop`, `while`, `for` con rangos e iteradores.
- **Funciones:** parámetros tipados, retorno explícito vs expresión final, funciones anidadas.
- **Strings:** `String` (heap, owned) vs `&str` (borrowed slice). `format!`, métodos de string.
- **I/O básica:** `println!`, `eprintln!`, `stdin().read_line()`.

### 🎯 Proyecto final: Calculadora REPL

Construye una calculadora interactiva en consola que:
- Lea expresiones desde stdin (`3 + 4 * 2 - 1`)
- Evalúe respetando precedencia de operadores
- Soporte `+`, `-`, `*`, `/`, `%` y paréntesis
- Continúe en bucle hasta que el usuario escriba `exit`
- Muestre errores descriptivos (división entre cero, expresión inválida)

```
> 3 + 4 * 2
= 11
> (3 + 4) * 2
= 14
> exit
Bye!
```

---

## 02 — Ownership, Borrowing y Lifetimes

> ⚠️ **El módulo más importante del lenguaje.** Dedica todo el tiempo que necesites.

### Temas

- **Ownership:** cada valor tiene un único dueño. Cuando sale de scope, se destruye (`drop`).
- **Move semantics:** asignar o pasar como argumento transfiere la propiedad. El original queda invalidado.
- **Copy vs Clone:** tipos `Copy` (escalares, `bool`, `char`, `&T`) se copian implícitamente. `Clone` es explícito.
- **Referencias:** `&T` referencia inmutable, `&mut T` mutable. Regla: N inmutables **o** 1 mutable, nunca ambas.
- **Slices:** `&str`, `&[T]`. Vistas de secuencias contiguas sin tomar ownership.
- **El Borrow Checker:** cómo razona el compilador. Aprender a leer sus mensajes de error.
- **Lifetimes básicos:** anotaciones `'a`. Por qué existen, cuándo el compilador las infiere (elision rules).
- **Lifetimes en structs y funciones:** `struct Important<'a> { content: &'a str }`.
- **`'static`:** qué significa realmente y cuándo usarlo.

### 🎯 Proyecto final: Analizador de logs en memoria

Construye un sistema que:
- Cargue un archivo de logs como `String`
- Exponga vistas (`&str`) sobre las líneas sin copiar datos
- Permita filtrar por nivel (`ERROR`, `WARN`, `INFO`) devolviendo slices del original
- Implemente una función que tome referencias a dos colecciones de logs y devuelva las entradas comunes
- El compilador debe validar que ninguna referencia sobreviva al buffer original

---

## 03 — Structs, Enums y Pattern Matching

### Temas

- **Structs:** tuple structs, unit structs, structs con campos nombrados.
- **Métodos:** bloque `impl`, `self`, `&self`, `&mut self`. Métodos asociados (constructores `::new()`).
- **Enums:** variantes con y sin datos. Enums como suma de tipos (`Shape::Circle(f64)`, `Shape::Rect(f64, f64)`).
- **`Option<T>`:** representación de ausencia de valor. `Some(T)`, `None`. Métodos: `unwrap`, `unwrap_or`, `map`, `and_then`, `is_some`, `is_none`.
- **`Result<T, E>`:** manejo de operaciones fallibles. `Ok(T)`, `Err(E)`.
- **`match`:** exhaustivo, destructuring de tuplas/structs/enums, guards `if`, bindings `@`.
- **`if let` y `while let`:** azúcar sintáctico para matchear una variante.
- **Destructuring:** en `let`, en parámetros de función, en `for`.

### 🎯 Proyecto final: Intérprete de un lenguaje de comandos

Define mediante enums y structs un pequeño lenguaje de comandos:

```
SET nombre "Alice"
GET nombre
DELETE nombre
LIST
```

- Parsea cada línea a un enum `Command { Set(String, String), Get(String), Delete(String), List }`
- Ejecuta los comandos sobre un `HashMap` en memoria
- Devuelve `Result<String, CommandError>` con errores tipados
- Usa `match` exhaustivo para despachar comandos

---

## 04 — Traits y Genéricos

### Temas

- **Traits:** definición, implementación, traits como interfaces. `Display`, `Debug`, `Clone`, `PartialEq`, `PartialOrd`.
- **Trait bounds:** `fn mayor<T: PartialOrd>(a: T, b: T) -> T`. Sintaxis `where`.
- **Genéricos en structs y enums:** `struct Stack<T>`, `impl<T> Stack<T>`.
- **Trait objects:** `&dyn Trait`, `Box<dyn Trait>`. Dispatch dinámico vs estático.
- **`impl Trait`:** en posición de parámetro y de retorno.
- **Blanket implementations:** `impl<T: Display> ToString for T`.
- **Traits estándar importantes:** `From`/`Into`, `AsRef`, `Deref`, `Iterator`, `Default`, `Drop`.
- **Newtype pattern:** wrapping de tipos para implementar traits externos.
- **Operador overloading:** `std::ops::Add`, `Mul`, etc.

### 🎯 Proyecto final: Librería de estructuras de datos genéricas

Implementa desde cero:
- `Stack<T>`: push, pop, peek con genéricos
- `Queue<T>`: enqueue, dequeue
- `BinaryTree<T: Ord>`: insert, search, in-order traversal
- Implementa `Display` para cada estructura
- Implementa `Iterator` para poder usar `for` sobre ellas
- Escribe un trait `Container<T>` que abstraiga operaciones comunes

---

## 05 — Colecciones e Iteradores

### Temas

- **`Vec<T>`:** creación, push/pop, indexing, slicing, retención, deduplicación, ordenación.
- **`HashMap<K, V>`:** insert, get, entry API (`or_insert`, `or_insert_with`). `BTreeMap` y sus garantías de orden.
- **`HashSet<T>` y `BTreeSet<T>`:** unión, intersección, diferencia.
- **`VecDeque<T>`:** cola de doble extremo.
- **`String` como colección:** bytes vs chars, iteración, slicing seguro.
- **Iterator trait:** `next()`, adaptadores lazy: `map`, `filter`, `flat_map`, `take`, `skip`, `zip`, `enumerate`, `chain`.
- **Consumidores:** `collect`, `fold`, `sum`, `product`, `count`, `any`, `all`, `find`, `position`, `max`, `min`.
- **Closures:** captura de entorno (`move`), `Fn`, `FnMut`, `FnOnce`.
- **`impl Iterator`:** implementar el trait para tipos propios.

### 🎯 Proyecto final: Procesador de CSV

Construye un procesador de archivos CSV que:
- Lea un CSV de N columnas desde un archivo
- Permita filtrar filas con predicados encadenables (`.filter_by("edad", |v| v > 18)`)
- Permita seleccionar columnas (`.select(&["nombre", "email"])`)
- Permita ordenar por columna (`.sort_by("apellido")`)
- Agrupe y cuente por un campo (`.group_count("ciudad")`)
- Escriba el resultado a un nuevo CSV
- Todo construido sobre iteradores, sin cargar el archivo entero en memoria de una vez

---

## 06 — Manejo de Errores

### Temas

- **`Result<T, E>`** en profundidad: composición, transformación, propagación.
- **Operador `?`:** cómo funciona internamente (convierte via `From`). Usar en `main`.
- **`From` y `Into`** para conversión de errores entre capas.
- **Errores personalizados:** implementar `std::error::Error` manualmente.
- **`thiserror`:** crate para derivar errores tipados con ergonomía.
- **`anyhow`:** crate para aplicaciones donde el tipo de error exacto no importa.
- **Cuándo usar `unwrap` y `expect`:** en tests y prototipos, nunca en producción.
- **`panic!` vs `Result`:** qué situaciones justifican cada uno.

### 🎯 Proyecto final: CLI de descarga de archivos con reintentos

Construye una herramienta de línea de comandos que:
- Acepte una URL como argumento
- Descargue el archivo usando `ureq` o `reqwest` (bloqueante)
- Defina una jerarquía de errores tipados: `NetworkError`, `IoError`, `ParseError`
- Implemente reintentos automáticos (hasta 3 veces) con backoff exponencial
- Muestre errores descriptivos diferenciando causas (timeout, 404, error de escritura)
- Retorne exit code apropiado (`0` éxito, `1` error)

---

## 07 — Módulos, Crates y Cargo avanzado

### Temas

- **Sistema de módulos:** `mod`, `pub`, `pub(crate)`, `pub(super)`, `use`, `as`.
- **Archivos y directorios:** `mod.rs` vs archivos con nombre de módulo. Árbol de módulos.
- **Crates:** binarios vs librerías. `lib.rs`, `main.rs`, múltiples binarios.
- **`Cargo.toml` avanzado:** features condicionales, `[features]`, `cfg!()`, `#[cfg(feature = "...")]`.
- **Workspaces:** monorepo con múltiples crates relacionados.
- **`crates.io`:** publicar una librería, `cargo publish`, semver, documentación con `///`.
- **`cargo` tools útiles:** `cargo expand`, `cargo audit`, `cargo udeps`, `cargo tree`.
- **Documentación:** `///` doc comments, ejemplos ejecutables en docs, `cargo doc --open`.

### 🎯 Proyecto final: Librería publicable en crates.io

Diseña y escribe una librería de utilidades (ej: parseo de configuración, formateo de fechas, validación de datos) que:
- Esté organizada en submódulos coherentes
- Tenga API pública bien definida con `pub` selectivo
- Incluya features opcionales (`serde` support activable con `features = ["serde"]`)
- Tenga documentación completa con ejemplos que compilen (`cargo test --doc`)
- Esté estructurada como workspace con un crate de librería y otro de CLI que la usa

---

## 08 — Smart Pointers

### Temas

- **`Box<T>`:** allocación en heap, tipos recursivos (`Box<Node>`), dispatch con `Box<dyn Trait>`.
- **`Rc<T>`:** reference counting para múltiples dueños en single-thread. `Rc::clone`, `Rc::strong_count`.
- **`Arc<T>`:** versión atómica de `Rc` para uso en múltiples threads.
- **`RefCell<T>`:** interior mutability, borrow checking en tiempo de ejecución. `borrow()`, `borrow_mut()`.
- **`Cell<T>`:** interior mutability para tipos `Copy`.
- **`Weak<T>`:** referencias débiles para romper ciclos de referencia.
- **`Cow<'a, T>`:** Clone on Write, evitar copias innecesarias.
- **`Deref` y `DerefCoerce`:** cómo `Box<String>` se comporta como `&str`.
- **`Drop` trait:** lógica de destrucción personalizada. `std::mem::drop`.

### 🎯 Proyecto final: Árbol DOM simplificado

Implementa un árbol de nodos estilo DOM donde:
- Cada nodo puede tener múltiples hijos y referencia a su padre
- Usa `Rc<RefCell<Node>>` para co-ownership y mutabilidad interior
- Usa `Weak<RefCell<Node>>` para la referencia al padre (evitar ciclos)
- Implementa: `append_child`, `remove_child`, `find_by_id`, traversal
- Serializa el árbol a una cadena de texto con indentación

---

## 09 — Concurrencia

### Temas

- **Threads:** `std::thread::spawn`, `JoinHandle`, `thread::sleep`. El modelo de concurrencia de Rust.
- **`Arc<Mutex<T>>`:** compartir estado mutable entre threads de forma segura.
- **`Arc<RwLock<T>>`:** múltiples lectores o un escritor.
- **Channels:** `std::sync::mpsc` — `Sender`, `Receiver`, `send`, `recv`. Múltiples productores.
- **`Mutex` y deadlocks:** cómo evitarlos, `try_lock`, timeouts.
- **`Send` y `Sync`:** los traits que garantizan thread safety en tiempo de compilación.
- **Rayon:** paralelismo de datos con `par_iter()`, `par_sort()`. Fork-join automático.
- **`Condvar`:** variables de condición para sincronización avanzada.
- **Atomic types:** `AtomicBool`, `AtomicUsize`, `AtomicPtr`. Ordering (Relaxed, SeqCst, etc.).

### 🎯 Proyecto final: Pool de workers con cola de tareas

Implementa un `ThreadPool` desde cero:
- Número configurable de worker threads
- Cola de tareas protegida con `Arc<Mutex<VecDeque<Job>>>`
- Los workers duermen (condvar) cuando no hay trabajo y se despiertan al añadir tareas
- Shutdown graceful: espera a que terminen las tareas en curso
- Métricas: tareas completadas, tiempo promedio, workers activos
- Úsalo para procesar en paralelo una carpeta de archivos de texto (contar palabras)

---

## 10 — Async / Await

### Temas

- **El problema que resuelve async:** I/O concurrente sin threads por conexión.
- **`Future` trait:** cómo funciona `poll`, `Waker`, el estado de una máquina de estados generada.
- **`async fn` y `.await`:** sintaxis, cómo el compilador transforma el código.
- **Runtimes:** por qué Rust no incluye uno. `Tokio` (el estándar de facto).
- **`tokio::spawn`:** tareas ligeras, `JoinHandle`.
- **`tokio::select!`:** esperar en múltiples futures simultáneamente.
- **Channels async:** `tokio::sync::mpsc`, `broadcast`, `oneshot`, `watch`.
- **`Stream`:** versión async de `Iterator`. `tokio_stream`, `StreamExt`.
- **Axum / Actix-web:** construir un servidor HTTP async. Routing, middleware, extractors.
- **`reqwest`:** HTTP client async. `sqlx`: base de datos async.
- **Errores comunes:** `Send` bounds en futures, async en traits (`async_trait`).

### 🎯 Proyecto final: API REST async

Construye una API REST con Axum que:
- Gestione una entidad (ej: tareas, productos, usuarios)
- CRUD completo: `GET /items`, `GET /items/:id`, `POST /items`, `PUT /items/:id`, `DELETE /items/:id`
- Persistencia con `sqlx` + SQLite o PostgreSQL
- Autenticación con JWT (middleware de Axum)
- Manejo de errores con tipos propios que se serialicen a JSON
- Tests de integración con `axum::http::StatusCode`

---

## 11 — Macros

### Temas

- **`macro_rules!`:** declarativas. Patrones, repetición `$(...)*`, `$(...)+`, fragmentos (`expr`, `ident`, `ty`, `block`, `stmt`, `pat`).
- **Macros del lenguaje:** `vec!`, `println!`, `assert!`, `todo!`, `unimplemented!`, `dbg!` — cómo están implementadas.
- **Macros procedurales:** diferencias con `macro_rules!`. Necesitan su propio crate.
- **Derive macros:** `#[derive(MyTrait)]`. Uso de `syn` y `quote` para parsear y generar código.
- **Attribute macros:** `#[route("GET", "/")]`. Transforman el item al que se aplican.
- **Function-like macros:** `sql!(SELECT * FROM users)`.
- **Casos de uso reales:** reducir boilerplate, DSLs internos, generación de código.

### 🎯 Proyecto final: Derive macro personalizada

Crea una derive macro `#[derive(Builder)]` que genere automáticamente el patrón Builder para cualquier struct:

```rust
#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    timeout: Option<u64>,
}

// Auto-genera ConfigBuilder con métodos encadenables:
let config = Config::builder()
    .host("localhost")
    .port(8080)
    .build()?;
```

---

## 12 — Unsafe y FFI

### Temas

- **Por qué existe `unsafe`:** invariantes que el compilador no puede verificar.
- **Las 5 superpoderes de unsafe:** raw pointers, llamar funciones unsafe, implementar traits unsafe, acceder a `static mut`, acceder a campos de unions.
- **Raw pointers:** `*const T` y `*mut T`. Creación, desreferencia, aritmética.
- **`unsafe fn` y `unsafe trait`:** cuándo y cómo usarlos correctamente.
- **FFI con C:** `extern "C"`, `#[no_mangle]`, `#[repr(C)]`, tipos compatibles.
- **`bindgen`:** generar bindings de C automáticamente.
- **Llamar a Rust desde C:** exponer funciones Rust como librería C.
- **`std::mem`:** `transmute`, `size_of`, `align_of`, `forget`, `ManuallyDrop`.
- **Reglas de oro:** unsafe mínimo, documentar invariantes, envolver en API safe.

### 🎯 Proyecto final: Binding a una librería C

Escribe un binding seguro para una librería C real (por ejemplo, `zlib` para compresión o `sqlite3`):
- Usa `bindgen` para generar los bindings crudos
- Crea una capa de API safe en Rust que encapsule todo el unsafe
- Implementa `Drop` para gestionar recursos C (evitar leaks)
- Escribe tests que comprueben que la API safe es sound

---

## 13 — Testing y Benchmarking

### Temas

- **Unit tests:** `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`, `#[should_panic]`, `#[ignore]`.
- **Tests en la librería vs en `tests/`:** inline vs integration tests. Módulo `#[cfg(test)]`.
- **Organización:** un test por comportamiento, nombres descriptivos, setup/teardown con funciones helper.
- **Mocking:** `mockall` crate. Mocking de traits con `#[automock]`.
- **`proptest` y `quickcheck`:** property-based testing. Generar casos aleatorios y buscar contraejemplos.
- **Benchmarking:** `criterion` crate. Cómo medir correctamente, evitar optimizaciones del compilador.
- **`cargo llvm-cov`:** cobertura de código.
- **Miri:** detectar undefined behavior en código unsafe.
- **Linting:** `cargo clippy` (lint avanzado), `cargo fmt` (formateo), `cargo audit` (vulnerabilidades en deps).

### 🎯 Proyecto final: Suite de tests para el proyecto de ownership (módulo 02)

Toma el analizador de logs del módulo 02 y escríbele:
- Tests unitarios con cobertura >90%
- Tests de integración que lean archivos de fixtures reales
- Property tests con `proptest` para el parser (cualquier string válido debe parsear sin panic)
- Benchmarks con `criterion` comparando dos implementaciones del filtrado
- Un workflow de CI (GitHub Actions `.yml`) que ejecute `clippy`, `fmt`, `test` y `audit`

---

## 14 — WebAssembly

### Temas

- **Qué es WebAssembly:** formato binario, ejecución en browser y en servidor (WASI).
- **`wasm-pack`:** compilar Rust a WASM listo para npm. `pkg/` output.
- **`wasm-bindgen`:** puente entre Rust y JavaScript. Exponer funciones, tipos, callbacks.
- **`web-sys` y `js-sys`:** bindings a las Web APIs (DOM, fetch, canvas, WebGL).
- **Interop JS ↔ Rust:** pasar strings, arrays, objetos. Gestión de memoria en el límite.
- **WASI:** WebAssembly fuera del browser. `wasmtime`, `wasmer`. Casos de uso en edge computing.
- **Leptos:** framework frontend en Rust puro compilado a WASM. Reactividad, componentes.
- **Tauri:** apps de escritorio con Rust (backend) + cualquier frontend web.

### 🎯 Proyecto final: Aplicación web con lógica en Rust/WASM

Construye una aplicación en el browser donde la lógica pesada corre en Rust compilado a WASM:
- Frontend en HTML/JS mínimo que carga el módulo WASM
- La lógica (ej: compresión de texto, cálculo numérico, procesado de imágenes en canvas) vive en Rust
- Expón funciones via `wasm-bindgen`
- Mide y documenta la diferencia de rendimiento vs la misma lógica en JS puro
- Publica el resultado en GitHub Pages

---

## 📚 Recursos esenciales

| Recurso | Descripción | Gratis |
|---------|-------------|--------|
| [The Rust Book](https://doc.rust-lang.org/book/) | Referencia oficial, empieza aquí | ✅ |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | Aprende con ejemplos ejecutables | ✅ |
| [Rustlings](https://github.com/rust-lang/rustlings) | Ejercicios interactivos en terminal | ✅ |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | Unsafe Rust en profundidad | ✅ |
| [Async Rust Book](https://rust-lang.github.io/async-book/) | Referencia oficial de async | ✅ |
| [Tokio Tutorial](https://tokio.rs/tokio/tutorial) | Tutorial oficial de Tokio | ✅ |
| [docs.rs](https://docs.rs/) | Documentación de cualquier crate | ✅ |
| [Exercism - Rust track](https://exercism.org/tracks/rust) | 100+ ejercicios con mentores | ✅ |
| [Zero To Production In Rust](https://www.zero2prod.com/) | Construir una API de producción | 💰 |
| [Rust for Rustaceans](https://nostarch.com/rust-rustaceans) | Nivel avanzado, altamente recomendado | 💰 |

---

## 🛠️ Crates esenciales que conocerás

```
serde / serde_json   →  serialización/deserialización
tokio                →  runtime async
axum / actix-web     →  servidores web
reqwest              →  HTTP client
sqlx                 →  base de datos async
clap                 →  CLI argument parsing
thiserror / anyhow   →  manejo de errores
rayon                →  paralelismo de datos
criterion            →  benchmarking
tracing              →  logging estructurado
uuid / chrono        →  tipos de utilidad
```

---

## 🔖 Progreso

Puedes usar esta checklist para trackear tu avance:

- [ ] 01 — Fundamentos + Calculadora REPL
- [ ] 02 — Ownership, Borrowing, Lifetimes + Analizador de logs
- [ ] 03 — Structs, Enums, Pattern Matching + Intérprete de comandos
- [ ] 04 — Traits y Genéricos + Estructuras de datos genéricas
- [ ] 05 — Colecciones e Iteradores + Procesador CSV
- [ ] 06 — Manejo de Errores + CLI de descarga con reintentos
- [ ] 07 — Módulos, Crates, Cargo + Librería publicable
- [ ] 08 — Smart Pointers + Árbol DOM
- [ ] 09 — Concurrencia + Thread pool
- [ ] 10 — Async / Await + API REST
- [ ] 11 — Macros + Derive macro Builder
- [ ] 12 — Unsafe y FFI + Binding a librería C
- [ ] 13 — Testing y Benchmarking + Suite de CI
- [ ] 14 — WebAssembly + App web con WASM

---

<p align="center">
  Hecho con 🦀 y mucho <code>cargo check</code>
</p>
