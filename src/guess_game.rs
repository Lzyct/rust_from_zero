use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub(crate) fn guess_game() {
    let mut secret_number = random_number();
    let mut attempts = 5;
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing guess variable to convert it from string to u32
        // and handle when user input is not a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e.to_string());
                println!("Please input a number!");
                continue;
            }
        };

        // decrement attempts
        attempts = &attempts - 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if attempts != 0 {
                    println!("⛔️ Too small!\n⚠️ You have {} attempts left", &attempts);
                    println!("-------------------------------------------------");
                } else {
                    println!("--------------------");
                    println!("You lose! 😭😭😭");
                    println!("---------------------\n");
                }
            }
            Ordering::Equal => {
                println!("------------------");
                println!("You win! 🎉");
                println!("------------------\n");
                attempts = 0;
            }
            Ordering::Greater => {
                if attempts != 0 {
                    println!("⛔️ Too big!\n⚠️ You have {} attempts left", &attempts);
                    println!("-------------------------------------------------");
                } else {
                    println!("--------------------");
                    println!("You lose! 😭😭😭");
                    println!("---------------------\n");
                }
            }
        }

        if attempts == 0 {

            // ask user if they want to play again
            println!("--------------------------------");
            println!("Do you want to play again? (y/n)");
            println!("--------------------------------");
            let mut play_again = String::new();
            io::stdin()
                .read_line(&mut play_again)
                .expect("Failed to read line");

            match play_again.trim().to_lowercase().as_str() {
                "y" => {
                    println!("\n\n---------------------");
                    println!("Let's play again! 💃🏻");
                    println!("---------------------");
                    secret_number = random_number();
                    attempts = 5;
                    continue;
                }
                "n" => {
                    println!("Bye! 👋");
                    break;
                }
                _ => {
                    println!("Invalid input!");
                    break;
                }
            }
        }
    }
}

fn random_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.(integer between 1 and 100)");
    println!("---------------------------------------------------");

    return secret_number;
}