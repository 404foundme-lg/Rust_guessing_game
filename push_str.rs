//The push_str method in Rust is used to append a string slice (&str) to an existing String object. It allows you to add more text to a String after it has been created.

//How it works:
// push_str takes a &str as an argument (which is a reference to a string slice) and appends that text to the String it's called on.
// t does not return a new string; it modifies the original String in place.

let mut s = String::from("Hello");
s.push_str(" there");
println!("{}", s); // Prints: "Hello there"

s.push_str("! How are you?");
println!("{}", s); // Prints: "Hello there! How are you?"


// Note: Since push_str modifies the original String, the String must be mutable (declared with mut).

// Important Points:
// Ownership: The push_str method does not take ownership of the argument. It only borrows the &str reference.
// Efficiency: push_str is more efficient than using + or format! for string concatenation, especially in a loop, because it avoids creating intermediate string values.
// Usage: push_str is commonly used when building strings dynamically, such as in loops or when constructing messages.
