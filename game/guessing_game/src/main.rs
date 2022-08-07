use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    println!("\n>>>>>>>Guess the number!<<<<<<<<<<\n\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);println!("是多少不告诉你");

    loop
    {
        println!("你猜是多少? 1-100");

        let mut guess = String::new();io::stdin().read_line(&mut guess).expect("err");//println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse(){Ok(num) => num,Err(_) => continue};//println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!(" ^ 小了!"),Ordering::Greater => println!(" ^ 大了!"),Ordering::Equal => { println!(" ^a^ ====> You win!"); break;}
        }
    }
}