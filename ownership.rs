fn main{
    // ownership in Rust is a set of rules that governs how memory is managed.
    // The main principles of ownership are: 
    // 1. Each value in Rust has a variable that is called its owner.
    // 2. There can only be one owner at a time.      
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        let s1 = String::from("hello"); // s1 is the owner of the string "hello"
        let s2 = s1; // ownership of the string is moved from s1 to s2

        // println!("{}", s1); // this line would cause a compile-time error because s1 is no longer valid
        println!("{}", s2); // this works because s2 is now the owner
    } // s2 goes out of scope here and the memory is freed

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // deep copy of the string, both s1 and s2 are valid

        println!("s1 = {}, s2 = {}", s1, s2); // both s1 and s2 can be used
    } // both s1 and s2 go out of scope here and their memory is freed

    {
        let x = 5;
        let y = x; // for types like integers, a copy is made

        println!("x = {}, y = {}", x, y); // both x and y can be used
    } // x and y go out of scope here 

    {
        let s = String::from("hello");

        takes_ownership(s); // ownership of s is moved to the function

        // println!("{}", s); // this line would cause a compile-time error because s is no longer valid
    } // s goes out of scope here, but it has already been dropped in the function

    {
        let x = 5;

        makes_copy(x); // x is copied into the function

        println!("{}", x); // x can still be used because it was copied
    } // x goes out of scope here

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    } // some_string goes out of scope here and the memory is freed

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    } // some_integer goes out of scope here

    
}