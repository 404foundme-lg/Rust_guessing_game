// _x is called a leading underscore variable in Rust
// It is used to indicate that the variable is intentionally unused
// This helps to avoid compiler warnings about unused variables

fn main(){
    let x= 6;
    let _xy =7;

    println!("The value of x is: {}", x);
}

// The variable _xy is not used in the program, but because it starts with an underscore,
// the Rust compiler will not generate a warning about it being unused. 
// This is useful when you want to declare a variable for future use or for clarity,
// but you don't need to use it immediately.
// Leading underscore variables are a common convention in Rust to indicate that a variable is intentionally unused.
// This helps to keep the code clean and free of unnecessary warnings.// For example, in a function where you need to accept a parameter but don't use it,
// you can name it with a leading underscore to avoid warnings.
fn example_function(_unused_param: i32) {
    println!("This function does not use its parameter.");
}

// Calling the example function
// example_function(10);    
// The parameter _unused_param is not used in the function body,
// but because it starts with an underscore, the compiler will not generate a warning.  

// Leading underscore variables can also be useful in pattern matching,
// where you want to ignore certain values.
fn pattern_matching_example() {
    let tuple = (1, 2, 3);

    match tuple {
        (x, _, z) => {
            println!("x: {}, z: {}", x, z); // ignores the second value
        }
    }
}

// Calling the pattern matching example function
// pattern_matching_example();          
// In this example, the second value of the tuple is ignored using a leading underscore in the pattern match.   



// unutilized function paramenters 
// by default, Rust will warn you about unused variables or parameters
fn process_value(value: i32, _unused_param: i32) {
    println!("Processing value: {}", value);
}

// The correct version would be 

fn main(){
    process_value(42.34, 0); // rust enforcess arity and type correctness strictly
}

fn process_value(x:f64, _y:i32){
    println!("Processing value: {}", x)
}