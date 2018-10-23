extern crate rand;

use rand::Rng;

fn main() {
    println!("****************************");
    println!("      Guess the number!     ");
    println!("****************************");

    let secret_number: u32 = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number! Try again...");
                continue;
            }
        };   

        println!("You guessed: {}", guess);
       
        match guess.cmp(&secret_number){
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
}
