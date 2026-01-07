// Lifetimes are a way to ensure that references in Rust are always valid. 
// Rust’s borrow checker uses lifetimes to make sure that references do not outlive the data they point to, preventing dangling references (a common issue in languages like C and C++).

// When you borrow data, you must specify lifetimes to tell the Rust compiler how long references are valid. Lifetimes are mostly implicit, but sometimes, you need to be explicit, especially with functions that take references as parameters.

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let str1 = String::from("long string");
    let str2 = "short";
    let result = longest(str1.as_str(), str2);
    println!("The longest string is: {}", result);
}

// Example of dangling reference

// remember: Lifetime is not about how long a value exists in memory
// It is about how long a reference to that value may be used safely

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