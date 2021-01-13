use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Mutable. To enable adding to the string after reading the input
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Pass in a mutable reference to 'guess'
    // match io::stdin().read_line(&mut guess) {
    //     Ok(n) => {
    //         println!("{} bytes read", n);
    //         println!("You guessed: {}", guess);
    //     }
    //     Err(error) => println!("error: {}", error),
    // }
}
