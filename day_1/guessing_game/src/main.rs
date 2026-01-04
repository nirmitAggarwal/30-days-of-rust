//guessing
// random range
//loop till they guess right

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let secret: u32 = rand::thread_rng().gen_range(0, 100);
        println!("Secret is {secret}");
        println!("Input a number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error in input");
        println!("You entered {}", guess.trim());
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","THAT IS NOT A NUMBER!!!".truecolor(255, 165, 0));
                continue;
            }
        };

        // if guess > secret {
        //     println!("Your Guess is bigger");
        // }else if secret > guess {
        //     println!("Your Guess is smaller");
        // }else{
        //     println!("Bingo ");
        //     break;
        // }

        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("{}", "Bingo!".green());
                break;
            }
            Ordering::Less => println!("{}", "TOO Low!".red()),
            Ordering::Greater => println!("{}", "TOO High!".red()),
        }
    }
}
