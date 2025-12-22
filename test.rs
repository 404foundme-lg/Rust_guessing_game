use std::io;

fn main(){
    println!("This is hour guessing game");
    println!("Plaese guess a number");


    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("Your guess has been: {}", guess);
}