use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // Mutable. To enable adding to the string after reading the input
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
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
