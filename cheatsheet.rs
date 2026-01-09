// ## Rust Daily-Use Keyword Cheat Sheet
//
// ### Variable Binding and Mutability
//
// ```rust
let x = 5;
let mut y = 10;
const MAX: u32 = 100;
static GLOBAL: i32 = 42;
// ```
//
// **Keywords:** `let`, `mut`, `const`, `static`
//
// ---
//
// ### Functions
//
// ```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// ```
//
// **Keywords:** `fn`, `return` (implicit return used above)
//
// ---
//
// ### Control Flow
//
// ```rust
if x > 0 {
    println!("positive");
} else {
    println!("non-positive");
}

for i in 0..3 {
    println!("{}", i);
}

while x > 0 {
    break;
}

loop {
    continue;
}
// ```
//
// **Keywords:** `if`, `else`, `for`, `while`, `loop`, `break`, `continue`
//
// ---
//
// ### Pattern Matching
//
// ```rust
match value {
    0 => println!("zero"),
    _ => println!("other"),
}
// ```
//
// **Keywords:** `match`
//
// ---
//
// ### Ownership and Borrowing
//
// ```rust
let s = String::from("hello");
let r = &s;
let m = &mut String::from("world");
// ```
//
// **Keywords:** `move`, `ref`, `mut`
//
// ---
//
// ### Structs and Enums
//
// ```rust
struct User {
    name: String,
    active: bool,
}

enum Status {
    Ok,
    Error,
}
// ```
//
// **Keywords:** `struct`, `enum`
//
// ---
//
// ### Implementations and Traits
//
// ```rust
trait Speak {
    fn speak(&self);
}

impl Speak for User {
    fn speak(&self) {
        println!("{}", self.name);
    }
}
// ```
//
// **Keywords:** `trait`, `impl`, `self`, `Self`
//
// ---
//
// ### Modules and Imports
//
// ```rust
mod utils;
use crate::utils::helper;
// ```
//
// **Keywords:** `mod`, `use`, `crate`, `super`
//
// ---
//
// ### Error Handling
//
// ```rust
fn risky() -> Result<i32, String> {
    Ok(42)
}
// ```
//
// **Keywords:** `Result`, `panic`
//
// ---
//
// ### Generics and Type System
//
// ```rust
fn print<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val);
}
// ```
//
// **Keywords:** `where`, `as`, `dyn`
//
// ---
//
// ### Lifetimes (Occasional Use)
//
// ```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}
// ```
//
// **Keywords:** lifetime syntax `'a`, `'static`
//
// ---
//
// ### Async Basics
//
// ```rust
async fn fetch() -> i32 {
    5
}

let result = fetch().await;
// ```
//
// **Keywords:** `async`, `await`
//
// ---
//
// ## Mental Model for Daily Rust
// * **You write**: `let`, `fn`, `if`, `match`, `struct`, `impl`
// * **You reason about**: ownership, borrowing, lifetimes
// * **You organize with**: `mod`, `use`
// * **You scale with**: traits, generics, async
