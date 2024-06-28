use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input = String::new();

  let _ = stdin.read_line(&mut input);

  // Parse the string into an integer
  let num: i32 = input.trim().parse().unwrap(); // replace i32 with your desired integer type

  println!("You entered: {}", num);
}
