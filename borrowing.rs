
// Rather than transferring ownership, you can also borrow data, which allows you to access it without taking ownership. There are two types of borrowing in Rust:
// Immutable Borrowing (&): You can borrow data immutably, meaning you can read it but not change it.
// Mutable Borrowing (&mut): You can borrow data mutably, meaning you can change it.

// Immutable Borrowing:
//With immutable borrowing, multiple parts of your code can read from the same variable at once. However, no one can modify the variable while it’s being borrowed.

fn main() {
    let s = String::from("Hello, Rust!");
    let s_ref1 = &s;  // Immutable borrow
    let s_ref2 = &s;  // Another immutable borrow
    println!("{}", s_ref1);
    println!("{}", s_ref2);
}

// Mutable Borrowing:
//With mutable borrowing, only one part of your code can have a mutable reference to a variable at a time. This ensures that no other part of your code can read or modify the variable while it’s being changed.


// This prevents "DATA RACE" conditions at compile time.
fn main() {
    let mut s = String::from("Hello");
    let s_mut_ref = &mut s;  // Mutable borrow
    s_mut_ref.push_str(", Rust!");
    println!("{}", s_mut_ref);
}

// If you try to mix immutable and mutable borrowing at the same time, Rust will give you a compile-time error:
fn main() {
    let mut s = String::from("Hello");
    let s_ref1 = &mut s;  // Mutable borrow
    let s_ref2 = &s;      // Immutable borrow (This will cause an error)
}
// To fix this, ensure that you only have either mutable or immutable references at any given time.


// Key Takeaway: Borrowing ensures that you can either have multiple read-only references or a single writeable reference, but never both at the same time.