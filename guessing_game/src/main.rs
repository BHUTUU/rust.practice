use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
  loop {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret numver is: {secret_number}");
    println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");
      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
      };
    println!("you guessed: {guess}");
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!\n"),
      Ordering::Greater => println!("Too big!\n"),
      Ordering::Equal => {
        println!("You win!\n");
        break;
      }
    }
  }
}
