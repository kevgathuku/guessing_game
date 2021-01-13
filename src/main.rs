use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // Mutable. To enable adding to the string after reading the input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // go to the next iteration of the loop and ask for another guess
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // Pass in a mutable reference to 'guess'
    // match io::stdin().read_line(&mut guess) {
    //     Ok(n) => {
    //         println!("{} bytes read", n);
    //         println!("You guessed: {}", guess);
    //     }
    //     Err(error) => println!("error: {}", error),
    // }
}
