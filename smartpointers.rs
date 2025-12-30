// Rust provides smart pointers to manage memory and ownership in more complex scenarios.

// Box<T>: A smart pointer that allocates data on the heap and ensures the data is deallocated when it goes out of scope.

// Rc<T> (Reference Counted): A smart pointer for shared ownership of data. It keeps track of how many references exist to the data, and when no more references exist, the data is freed.

// Arc<T> (Atomic Reference Counted): Like Rc, but designed for thread-safe shared ownership in concurrent programs.


fn main() {
    let b = Box::new(5);  // Allocates memory on the heap
    println!("{}", b);  // Prints: 5
}

// Key Takeaway: Smart pointers are essential when you need more control over memory allocation or want to share data safely between multiple parts of your program.