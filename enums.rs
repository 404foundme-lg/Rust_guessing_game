/* Enums (short for "enumerations"), type that can be one of several variants. 
Each variant can optionally hold data.

Here's a basic example of how to define and use enums in Rust:
*/

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East  => println!("Moving East"),
        Direction::West  => println!("Moving West"),
    }
}

/* Enums vs structs

Use a struct when:
- All instances have the same fields.
- The data shape never changes.

Use an enum when:
- A value can be in one of several distinct forms.
- Each form may carry different data

*/


// Methods on enums
// Enums can have methods using impl, just like structs.

enum Status {
    Success,
    Error(String),
}

impl Status {
    fn is_success(&self) -> bool {
        match self {
            Status::Success => true,
            Status::Error(_) => false,
        }
    }
}

// Usage
let s = Status::Error(String::from("Failed"));
println!("{}", s.is_success());

// The Option enum
// One of the most important enums in Rust is Option.

enum Option<T> {
    Some(T),
    None,
}

// The Result enum
// Another core enum is Result, used for error handling.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
// Example usage
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
