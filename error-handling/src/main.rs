fn main() {
  let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

  for &index in [0, 2, 99].iter() {
      match fruits.get(index) {
          Some(&"coconut") => println!("Coconut are awesome!"),
          Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
          None => println!("There is no fruit! :("),
      }
  }

  let a_number = Some(7);
  match a_number {
    Some(7) => println!("This is a lucky number")
  }
}
