enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let ip = IpAddr::V4(String::from("127.0.0.1"));
    println!("{:?}", ip);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
