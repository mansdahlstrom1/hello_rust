extern crate rand;
use rand::Rng;

use std::io;
use std::cmp::Ordering;

pub enum Clue {
  GuessLower = -1,
  Correct = 0,
  GuessHigher = 1,
}

pub struct GuessingGame {
  pub correct: bool,
  pub latest_guess: u32,
  pub guess_higher: Clue,
}

impl GuessingGame {
  pub fn guess(&mut self) {
    println!("Please input the magic number!");
  
    let mut guess = String::new();
  
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
  
    println!("you guessed {}", guess);
  
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Something went wrong");

    self.latest_guess = guess;
  
    let secret_number = rand::thread_rng().gen_range(1, 10);
  
    println!("The secret number is {}", secret_number);
  
    match guess.cmp(&secret_number) {
        Ordering::Less => {
          println!("Guess higher!");
          self.guess_higher = Clue::GuessHigher;
          self.correct = false;
        },
        Ordering::Greater => {
          println!("Guess lower");
          self.guess_higher = Clue::GuessLower;
          self.correct = false;
        },
        Ordering::Equal => {
          println!("Correct");
          self.guess_higher = Clue::Correct;
          self.correct = true;
        },
    }
  }
} 
