
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

