use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the number");
    let secret_num = rand::thread_rng().gen_range(1..100);
    loop{
       println!("Please input your guess.");
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess : i32= match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    match guess.cmp(&secret_num){
        Ordering::Equal => {
            println!("{}","You win".green().bold());
            break;
        },
        Ordering::Greater => println!("{}", "Too big".red()),
        Ordering::Less => println!("{}", "Too small".red()),
    } 
    }
    
}
