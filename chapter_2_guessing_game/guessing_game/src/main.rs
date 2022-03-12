use std::cmp::Ordering;
use std::io;
use rand::Rng;
// Prelude: items in std lib brought into every program by default

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // let: indicates variable definition
        // mut: declares variable as mutable (variables immutable by default)

        io::stdin()
            .read_line(&mut guess)
            // & indicates a reference to a variable instead of copying the data
            // references are immutable by default
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");


        // TODO: Consider Invalid Input

        // Note: guess exists prior to this u32 definition
        // shadowing allows for reusing the name
        // shadowing is usually used for type casting/conversion
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
