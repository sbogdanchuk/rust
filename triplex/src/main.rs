use rand::prelude::*;
use std::io::{self, BufRead};

fn print_introduction(difficulty: u16) {
    // Print welcome message to the terminal
    print!(
        "\n\nYou are a secret agent breaking into a level {}",
        difficulty
    );
    print!(" secret weapons lab...\n You need to enter the correct codes to continue...\n\n");
}

fn play_game(difficulty: u16) -> bool {
    print_introduction(difficulty);
    // Declare 3 number code
    let mut rng = thread_rng();
    let code_a: u16 = rng.gen_range(0..difficulty) + difficulty;
    let code_b: u16 = rng.gen_range(0..difficulty) + difficulty;
    let code_c: u16 = rng.gen_range(0..difficulty) + difficulty;

    // println!("{} {} {}", code_a, code_b, code_c);
    let code_sum = code_a + code_b + code_c;
    let code_product = code_a * code_b * code_c;

    println!("\n+ There are 3 numbers in the code");
    println!("+ The codes add-up to: {}", code_sum);
    println!("+ The codes multiply to give: {}", code_product);

    // Store player guess

    let guesses: Vec<u16> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| s.chars().all(char::is_numeric))
        .map(|s| s.parse().unwrap())
        .collect();
    let guess_sum: u16 = guesses.iter().sum();
    let guess_product: u16 = guesses.iter().fold(1, |a, b| a * b);

    // Check if the players guess is correct
    if guess_sum == code_sum && guess_product == code_product {
        println!("\n*** Well done agent! You have extracted a weapon! Keep going! ***");
        true
    } else {
        println!("\n*** You entered the wrong code! Careful agent! Try again! ***");
        false
    }
}

fn main() {
    let mut level_difficulty: u16 = 1;
    let max_difficulty: u16 = 5;

    while level_difficulty <= max_difficulty {
        // Loop game until all levels completed
        let level_complete = play_game(level_difficulty);

        if level_complete {
            level_difficulty += 1;
        }
    }
    println!("\nGrean work agent! You have got all the weapons!, Now get out of there! ***");
}
