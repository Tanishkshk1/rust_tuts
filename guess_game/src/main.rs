use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Hello, world!");
        let secret_nuber = rand::thread_rng().gen_range(1..=10);
        // println!("The Random number is {secret_nuber}");
        // building a new string using io
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Here i am using cmo to campare to things
        match guess.cmp(&secret_nuber) {
            Ordering::Less => println!("Too Less"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
