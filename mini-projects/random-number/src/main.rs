use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter a number between 1 and 100");

    let rnum = rand::thread_rng().gen_range(1..=100);

    loop {
        // take user input
        let mut user_line = String::new();
        io::stdin()
            .read_line(&mut user_line)
            .expect("Cannot read line!");

        // convert to integer
        let user_num: u8 = match user_line.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a valid integer!");
                continue
            }
        };

        // compare
        match user_num.cmp(&rnum) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("correct!");
                break
            }
        }
    }
}
