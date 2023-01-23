fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = number + 6;
    if number % 4 == 0 {
        println!("number was divisible by 4");
    } else if number % 3 == 0 {
        println!("number was divisible by 3");
    } else if number % 2 == 0 {
        println!("number was divisible by 2");
    } else {
        println!("number was prime");
    }

    let mut counter = 1;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };

    println!("the counter value is {result}");

    for n in (1..4).rev() {
        println!("{n}");
    }

    let a = ['a', 's', 'd', 'f', 'g'];
    for c in a {
        println!("{c}");
    }
}
