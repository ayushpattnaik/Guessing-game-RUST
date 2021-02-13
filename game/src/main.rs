//Guessing game in rust
use std::io;
use rand::Rng;
fn main() {
  println!("Guess the number!");
  println!("Please input your gress: ");
  let sec_num=rand::thread_rng().gen_range(1,101);
  let mut guess= String::new();
  io::stdin()
     .read_line(&mut guess)
     .expect("Failed to read line");
  print!("You guessed the number:{}", guess);
  println!("The secret number is {}",sec_num);
}