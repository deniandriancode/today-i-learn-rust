fn main() {
    let s1 = String::from("hello");

    takes_ownership(s1);

    let x = 4;
    makes_copy(x);

    let s1 = String::from("hi");
    let l = calculate_length(&s1);
    println!("{}", l);

    let mut s2 = String::from("ferum");
    change(&mut s2);
}

fn change(s: &mut String) {
    s.push_str(", iron");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(s: String) {
    println!("ownership: {s}");
}

fn makes_copy(n: u32) {
    println!("copy: {n}");
}
