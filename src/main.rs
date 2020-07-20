

mod guess_the_number;
mod ownership;
mod split_string;
mod structs;

use guess_the_number::GuessingGame;
use guess_the_number::Clue;

fn main() {
    let mut guessingGame = GuessingGame {
        correct: false,
        latest_guess: 0,
        guess_higher: Clue::Correct,
    };

    while !guessingGame.correct {
        guessingGame.guess();
    }

    println!("You won the guessing game!");
    
    println!("\n==========================\n");
    ownership::ownership_test();

    let my_word = String::from("Hello World");
    let first_word = split_string::split(&my_word);

    println!("First word of {} is {}, ", my_word, first_word);

    structs::test_users();

}
