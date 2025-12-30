// Rust typically enforces that mutable variables are only accessible through mutable references. However, there are some advanced scenarios where you might want a type to be mutated through an immutable reference. Interior mutability allows you to achieve this pattern. The most common types that provide interior mutability are:

// Cell<T>

// RefCell<T>

// These types allow you to mutate data inside them even when they are behind an immutable reference, but the borrow checker ensures that this is done safely.


use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    *x.borrow_mut() = 10;  // Mutable borrow through RefCell
    println!("{}", x.borrow());  // Prints: 10
}
