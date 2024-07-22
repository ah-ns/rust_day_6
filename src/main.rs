/*
Ownership in Rust:
    - Gives control over memory
    - Error free
    - Faster runtime than Garbage Collection
    - Small program size
    - Slower write time and learning curve

Dynamic size variables get stored on the heap.
Static size variables get stored on the stack.

Ownership rules
    1. Each value in Rust has a variable called its owner
    2. There can't be more than one owner at a time
    3. When the owner goes out of scope, the value will be dropped

Reference rules
    1. In any scope, you can have either one mutable, or any number of immutable references
    2. References must always be valid (They can't be from a function that is dropped)
*/

fn main() {
    let x = 5;
    let y = x; // Simple types (int, bool, char) can be copied simply like this
    println!("x: {x}, y: {y}");
    
    makes_copy(x);
    println!("x: {x}"); // In the same way, we can still print x after passing it into a function



    let s1 = String::from("hello");
    let s2 = s1; // Moves s1 to s2, if s1 is called, we will get an error
    let s3 = s2.clone(); // This actually clones s1 and adds s3 to the heap (more expensive)
    println!("s1: error, s2: {s2}, s3: {s3}");

    take_ownership(s3);
    println!("s3: error"); // In the same way, we can't print after it is passed into a function

    let s4 = give_ownership();
    println!("s4: {}", s4); // Functions can also give away ownership

    let s5 = takes_and_gives_back(s4); // Takes away ownership while in function
    println!("s4: {}", s5); // Gives back because the value is returned



    reference(&s5);
    println!("s5: {}", s5);

    let mut my_mut_string = String::from("Hello world");
    mutate_reference(&mut my_mut_string);
    println!("{}", my_mut_string); // You can only have one mutable reference to a value within a scope, prevents race conditions

    let my_string_literal = "hello world";
    //let hello = &my_mut_string[..=5];
    //let world = &my_mut_string[6..];
    let word = slicing(&my_string_literal);
    println!("{}", word);
}

// Makes copy of a simple data type
fn makes_copy(simple: i32) {
    println!("{}", simple);
}

// Takes owndership of a complex data type
fn take_ownership(complex: String) {
    println!("{}", complex);
}

// Gives ownership of return value to the place where the function is called
fn give_ownership() -> String{
    let my_string = String::from("world");
    my_string
}

// Takes ownership, then gives back
fn takes_and_gives_back(complex: String) -> String{
    complex
}

// Uses reference to the value, so can use without taking ownership
fn reference(my_string: &String) {
    println!("{}", my_string)
}

// References are immutable by default, but it is possible to make them mutable
fn mutate_reference(my_mut_string: &mut String) {
    my_mut_string.push_str("(added) world!");
}

// Slices can be used on collections such as strings, vectors, arrays, and hash maps
fn slicing(my_string: &str) -> &str {
    let bytes = my_string.as_bytes(); // Converts the string to array of bytes

    for (i, &item) in bytes.iter().enumerate() { // Iterate over the array of bytes, and enumerate tuple of (index, element reference)
        if item == b'r' { // Stops at the index where 'r' is
            return &my_string[0..i]; // Returns the slice up until where 'r' is
        }
    }

    &my_string[..] // If the character isn't found, return the whole string
}