// Shadowing
fn main() {
    let mut _x: i32 = 1;


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!"; 

    println!("Success!");
}