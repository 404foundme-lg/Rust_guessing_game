
// ## 1. `String` — the **growable, heap-allocated string**

// * **Owns its data** (stored on the heap)
// * You can **change it**, like push, pop, or append
// * Stored as `String` object in memory with **pointer + length + capacity**

// ```rust
let mut s = String::from("Hello");
s.push_str(", world!"); // You can modify it
println!("{}", s);      // "Hello, world!"
// ```

// ✅ Key points:

// * Mutable
// * Allocated on the heap
// * Can grow or shrink

// ---

// ## 2. `str` — the **fixed string slice**

// * A **view of string data**, usually seen as `&str`
// * **Does NOT own the data**
// * **Cannot be changed**
// * Typically comes from **string literals** or slices of `String`

// ```rust
let s: &str = "Hello";
// s.push_str("!");  <-- ❌ ERROR, cannot modify a &str
// ```

// ✅ Key points:

// * Immutable
// * Borrowed reference to some string
// * Often just a pointer + length

// ---

// ## 3. Analogy

// Think of **`String`** like a **resizable text box**,
// and **`&str`** like a **window looking at part of that text box**.

// * You can modify the text box (`String`)
// * You can read through the window (`&str`)

// ---

// ## 4. Common pattern

// * Use `String` when you **own or modify** the text
// * Use `&str` when you just **read or borrow** text


fn greet(name: &str) {
    println!("Hello, {}!", name);
}

let my_name = String::from("Alice");
greet(&my_name);  // works because &String can become &str
greet("Bob");      // works too



// another example
fn print(s: &str) {
    println!("{}", s);
}

let owned = String::from("hello");

print("literal");
print(&owned);
