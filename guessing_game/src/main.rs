use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess number");
    let select_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("please input the number first");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("you guess number: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&select_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        
       }

    }
}
