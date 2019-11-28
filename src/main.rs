use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

pub fn gen(x : &mut u32) -> bool {
    let secret_number = rand::thread_rng().gen_range(0, 2);
    //println!("secrect num: {}", secret_number);
    *x += 1;
    return secret_number == 0;
}

fn main(){
    let mut n = 0;
    for _ in 0..10000 {
        let mut x = 0;
        while gen(&mut x) {
            //println!("this gen: {}", x);
        }
        n += x;
    } 

    println!("get total {}, avg: {}", n, n / 10000);
}