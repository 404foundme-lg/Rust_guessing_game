// Rust’s slices and references allow you to work with parts of collections (like arrays or strings) without taking ownership of the entire collection. This is useful when you want to work with a portion of data.

// Example: Working with Slices

// A slice is a reference to a contiguous sequence of elements in a collection.

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];  // A slice of the array (index 1, 2, 3)
    println!("{:?}", slice);  // Prints: [2, 3, 4]
}

// Key Takeaway: Slices allow you to borrow part of an array or string without taking ownership of the entire data structure.

