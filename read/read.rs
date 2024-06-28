use std::io::{self};

fn main() {
  let stdin = io::stdin();
  
  let mut input = String::new();
  let _ = stdin.read_line(&mut input);

  println!("You entered: {}", input);

}
