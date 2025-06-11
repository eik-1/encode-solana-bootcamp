fn main() {
    let x:i32 = 5; // Created on stack
    let y:i32 = x; // This is a copy

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    let s1: String = String::from("Hello"); // Created on heap
    let s2: String = s1; // Move takes place instead of copying

    // println!("String s1 is {}", s1); // This will now produce an error since s1 doesn't exist
    println!("String s2 is {}", s2);

    let s3: String = String::from("Hello again");
    let s4: String = s3.clone(); // Cloning works

    println!("String s3 is {}", s3); 
    println!("String s4 is {}", s4); 

    takes_ownership(s3); // Passing the argument will transfer the ownership to the parameter

    // println!("String s3 is {}", s4); // This will now produce an error since s3 doesn't exist anymore

    let s5: string = gives_ownership(); // Gets the ownership from the variable returned by the function
    println!("We took the ownership: {}", s5);
}

fn takes_ownership(some_string: String) {
    println!("We took the string: {}", some_string); // It takes the ownership of the argument passed
} // scope of some_string ends

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");

    some_string // Returning some_string and transfering its ownership
}