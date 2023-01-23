const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    /*
     * multiline comment
     */
    let mut x = 5;
    println!("the value of x: {x}");
    x = 10;
    println!("the value of x: {x}");
    println!("three hours in seconds is {THREE_HOURS_IN_SECONDS}");
    let y = 5i8;
    let y = add_two(y);
    println!("result: {y}");
}

fn add_two(x: i8) -> i8 {
    x + 2
}
