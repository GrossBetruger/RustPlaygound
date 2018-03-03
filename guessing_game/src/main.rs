extern crate regex;
extern crate rand;

use std::io;
use std::cmp::Ordering;
use regex::Regex;
use rand::Rng;

fn guessing_game() -> String{
    let max_rand = 10;

    println!("guess a number...");

    let mut guess = String::new();
    let random_num = rand::thread_rng().gen_range(0, max_rand);
    let num_re = Regex::new(r"^\d+\n$").unwrap();


    io::stdin().read_line(&mut guess)
        .expect("failed to read your input");

    assert!(num_re.is_match(&guess), "your guess is not a number, we can't play...");

    let numeric_guess = guess.trim().parse::<i32>().unwrap();
    assert!(numeric_guess <= max_rand,
        format!("your guess is bigger than {}, go smaller...", max_rand));
    println!("your guess: {}", guess);

    match numeric_guess.cmp(&random_num){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("EXACTLY! you win")
    }

    println!("play again?");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("failed to read answer");
    return answer;

}

fn main() {

    loop {
        let play_again = guessing_game();
        if let "yes\n" = play_again.as_ref() {
            continue;
        }
        else { break }
    }

    println!("Bye Bye!")
}


