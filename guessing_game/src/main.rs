use std::io::{self, Write};
use rand::Rng;

fn main() {
    let mut choice = String::new();
    let mut next: bool = true;
    let mut num: u16 = rand::thread_rng().gen_range(1..10);
    let mut guess = String::new();

    while next {
        println!("\n===================================");
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        guess.clear();
        choice.clear();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline.");

        if &num == &guess.trim().parse::<u16>().unwrap() {
            println!("\nYou alright! number is {}, your guess is {}", &num, &guess);
            next = false;
        } else {
            println!("\nYou is wrong! number is {}, your guess is {}", &num, &guess);

            print!("Play again? Y/N: ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to readline.");
            
            if &choice.trim().to_ascii_uppercase() == "Y" {
                next = true;
                num = rand::thread_rng().gen_range(1..10);
            } else {
                next = false;
                println!("\nBye..");
                println!("===================================\n");
            }
        }

    }
}
