// dangling references are references that point to data that has already been deallocated. 
// Rust's ownership system and borrow checker are designed to prevent dangling references at compile time, ensuring memory safety.
// In Rust, a dangling reference can occur if you try to return a reference to a local variable that goes out of scope. The Rust compiler will catch this and prevent the code from compiling.
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // This line would cause a compile-time error because s goes out of scope
}      // and the reference would be dangling


