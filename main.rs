//Guessing game in rust
use std::io;
fn main() {
  println!("Guess the number!");
  println!("Please input your gress: ");
  let mut guess= String::new();
  io::stdin()
     .read_line(&mut guess)
     .expect("Failed to read line");
     println!("You guessed the number:\n{}", guess);
}