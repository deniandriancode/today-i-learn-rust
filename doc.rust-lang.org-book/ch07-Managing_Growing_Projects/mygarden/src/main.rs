use garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, world!");
    garden::display_info();
    let asp = Asparagus { greet: String::from("Asparagus number 1"), };
    println!("{}", asp.greet);
}
