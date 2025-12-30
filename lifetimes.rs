// Lifetimes are a way to ensure that references in Rust are always valid. Rust’s borrow checker uses lifetimes to make sure that references do not outlive the data they point to, preventing dangling references (a common issue in languages like C and C++).

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
