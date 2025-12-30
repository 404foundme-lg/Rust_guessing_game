// shadowing in Rust allows you to declare a new variable with the same name as a previous variable.
// The new variable shadows the previous one, meaning the previous variable is no longer accessible.
fn main() {
    let x = 5;
    println!("The value of x is: {}", x); // prints 5

    let x = x + 1; // shadows the previous x
    println!("The value of x is: {}", x); // prints 6

    {
        let x = x * 2; // shadows the previous x within this block
        println!("The value of x in the inner scope is: {}", x); // prints 12
    }

    println!("The value of x is still: {}", x); // prints 6, the outer scope 


    //shadowing with different types
    let spaces = "   ";
    let spaces = spaces.len(); // shadows the previous spaces variable with a new type (usize)
    println!("The number of spaces is: {}", spaces); // prints 3

    // example2: Variable shadowing with different types 
    let guess = 24;
    let guess = guess as f64;
    println!("The value of guess as f64 is: {}", guess); // prints 24.0


    let _ = 6;  // anonymous binding, value is immediately dropped 
}

