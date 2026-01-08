// structs in Rust
// Structs are custom data types that let you package related data together.
// They are similar to classes in other programming languages but do not have methods by default.
// Structs can have named fields, tuple-like fields, or be unit-like (without fields).
// Defining a struct with named fields
struct Person {
    name: String,
    age: u8,
}       
// Defining a tuple struct
struct Color(u8, u8, u8);
// Defining a unit-like struct
struct Marker;
fn main() {
    // Creating an instance of a struct with named fields
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // Accessing fields of the struct
    println!("Name: {}, Age: {}", person.name, person.age);
    
    // Creating an instance of a tuple struct
    let black = Color(0, 0, 0);
    println!("Color - R: {}, G: {}, B: {}", black.0, black.1, black.2);
    
    // Creating an instance of a unit-like struct
    let _marker = Marker;
}   

// Note: Structs can also implement methods and traits, but that is covered in other sections.
// Example of a function that takes multiple references and returns a reference with lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// The above function requires lifetime annotations because it takes multiple references and returns a reference.
// This ensures that the returned reference is valid as long as both input references are valid.


// In summary, structs are a fundamental way to create complex data types in Rust, allowing for better organization and management of related data.






//Example 2 

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let user1 = build_user(
        String::from("user1@example.com"),
        String::from("user1"),
    );
    println!("Username: {}, Email: {}", user1.username, user1.email);
}

// Example 3: Struct Update Syntax
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 15, ..p1 }; // Using struct update syntax
    println!("p2 - x: {}, y: {}", p2.x, p2.y); // y will be 10 from p1
}

// Example 4: Tuple Structs
struct Dimensions(i32, i32, i32);
fn main() {
    let dim = Dimensions(1920, 1080, 32);
    println!("Width: {}, Height: {}, Depth: {}", dim.0, dim.1, dim.2);
}
// Example 5: Unit-Like Structs
struct Singleton;
fn main() {
    let _s = Singleton; // Creating an instance of a unit-like struct
    println!("Singleton instance created.");
}
// Example 6: Implementing Methods for Structs
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect.area());
}
// Example 7: Using Structs with Lifetimes
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}
fn main() {
    let book = Book {
        title: "The Rust Programming Language",
        author: "Steve Klabnik and Carol Nichols",
    };

    println!("Book: {}, Author: {}", book.title, book.author);
}
// Note: In the Book struct, the lifetime 'a ensures that the references to title and author are valid as long as the Book instance is valid.

// Example 8: Nested Structs
struct Address {
    street: String,
    city: String,
    country: String,
}
struct Employee {
    name: String,
    address: Address,
}
fn main() {
    let emp_address = Address {
        street: String::from("123 Main St"),
        city: String::from("Metropolis"),
        country: String::from("Fictionland"),
    };

    let employee = Employee {
        name: String::from("John Doe"),
        address: emp_address,
    };

    println!("Employee: {}, Address: {}, {}, {}", 
             employee.name, 
             employee.address.street, 
             employee.address.city, 
             employee.address.country);
}
// Example 9: Structs with Generic Types
struct PointGeneric<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = PointGeneric { x: 5, y: 10 };
    let float_point = PointGeneric { x: 1.0, y: 4.0 };

    println!("Integer Point - x: {}, y: {}", int_point.x, int_point.y);
    println!("Float Point - x: {}, y: {}", float_point.x, float_point.y);
}
// Example 10: Mutable Structs
struct Counter {
    count: u32,
}
fn main() {
    let mut counter = Counter { count: 0 };
    counter.count += 1;
    println!("Counter: {}", counter.count);
}
// Note: Structs in Rust are versatile and can be used in various ways to model complex data and behaviors in your programs.

// Example of a function that takes multiple references and returns a reference with lifetimes
fn longest_ref<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// The above function requires lifetime annotations because it takes multiple references and returns a reference.
// This ensures that the returned reference is valid as long as both input references are valid.
// Example usage of the longest_ref function
fn main() {
    let string1 = String::from("Hello, world!");
    let string2 = "Hi!";
    
    let result = longest_ref(string1.as_str(), string2);
    println!("The longest string is: {}", result);
}
// Note: In this example, the lifetime 'a ensures that the returned reference from longest_ref is valid as long as both input references are valid.

// Example of dangling reference

fn main(){
    let r;
    
    {
        let x = 5;
        r = &x;
    } // x is dropped here
    
     // - `x` dropped here while still borrowed
    
    println!("{}", r); // invalid reference `x` does not live long enough
}
// Rust rejects this because r would outlive x.
// Example of a function that requires lifetimes
fn longest_no_lifetimes(x: &str, y: &str) -> &str
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The above function will not compile because the compiler cannot determine the lifetimes of the references.
// To fix this, we need to add lifetime annotations as shown in the previous examples.
// Example usage of the longest_no_lifetimes function
fn main() {


    let string1 = String::from("Hello, world!");
    let string2 = "Hi!";
    
    let result = longest_no_lifetimes(string1.as_str(), string2);
    println!("The longest string is: {}", result);
}

// Note: In this example, the function longest_no_lifetimes will not compile without lifetime annotations because the compiler cannot infer the lifetimes of the references involved.
// Rust rejects this because r would outlive x.

//lifetimes become reqired when:
// -A function takes multiple references

// - The return value is a reference

fn longest(x: &str, y:&str) -> &str {
    if x.len() > y.len(){
        x
        } else {
            y
            }
        }
    }
}


// Struct Update Syntax
// Assume you have an existing struct instance and want to create a new one that is mostly the same, with only a few fields changed.
// Without struct update syntax, you would need to repeat every field.

let user2 = User {
    username: user1.username,
    email: String::from("new@example.com"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
//This is verbose and error prone, especially as structs grow.

// What Struct Update Syntax Is
//-Struct update syntax allows you to:
//-Create a new struct instance
//-Reuse the remaining fields from another instance
//-Syntax form:

StructName {
    changed_field: new_value,
    ..existing_instance
}
//The .. means:
//“For all fields I did not explicitly set, take their values from this instance.”

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("alice@newdomain.com"),
    ..user1
};

// email is replaced
// All other fields are taken from user1



// After update syntax:
// Ownership of username moves to user2
// user1.username is no longer accessible
// println!("{}", user1.username); // This would cause a compile-time error
// Because String does not implement the Copy trait
// However, user1.email, user1.active, and user1.sign_in_count are still valid
// because they implement the Copy trait or are of types that do.
println!("{}", user1.email); // This is valid
println!("{}", user1.active); // This is valid
println!("{}", user1.sign_in_count); // This is valid
println!("{}", user2.username); // This is valid
println!("{}", user2.email); // This is valid   
println!("{}", user2.active); // This is valid
println!("{}", user2.sign_in_count); // This is valid
// Note: Struct update syntax is a convenient way to create new instances of structs while reusing existing data, reducing redundancy and potential errors.



// Using Struct Update With References
// If your struct uses references, values are copied as pointers.
struct User<'a> {
    username: &'a str,
    email: &'a str,
}

let user1 = User {
    username: "alice",
    email: "alice@example.com",
};

let user2 = User {
    email: "alice@newdomain.com",
    ..user1
};

// No ownership is transferred
// Both structs reference the same data


// When Struct Update Syntax Is Appropriate

// Use it when:
    // You want immutability
    // You want a new instance, not mutation
    // The original value is no longer needed
    // You want concise code

// Avoid it when:
    // You still need the original instance
   // The struct contains many owned fields you want to preserve