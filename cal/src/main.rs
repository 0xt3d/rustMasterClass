use std::io;

fn main() {
  println!("Enter an integer:");

  let mut input = String::new();

  io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

  let input: i32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => {
          println!("Invalid integer!");
          return;
      }
  };

  println!("You entered: {}", input);
}
Added a line