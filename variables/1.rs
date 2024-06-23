fn main() {
    // Declare a variable
    let a_number: i32;
        
    // Declare a second variable and bind the value
    let a_word = "Ten";
        
    // Bind a value to the first variable
    // Remove the following comment tag to resolve the compile error
    a_number = 10;
    
    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);
    
    // Change the value of an immutable variable
    // Remove the following comment tag to see the compile error
    // a_number = 15;
}
