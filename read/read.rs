use std::io;

fn main() {
  // declare a mutable variable
  let mut input = String::new();
  // read a  line
  let _ = io::stdin().read_line(&mut input);

  println!("You entered: {}", input.trim());
}
