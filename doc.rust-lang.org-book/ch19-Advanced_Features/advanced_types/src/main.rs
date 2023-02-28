fn main() {
    // type synonym just like haskell
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    never_return();
}

fn never_return() -> ! {
    panic!("NEVER RETURN, JUST CRASH");
}
