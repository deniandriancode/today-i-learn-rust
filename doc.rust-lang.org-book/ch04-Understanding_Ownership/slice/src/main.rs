fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{word}");

    s.clear();

    let a = [1, 2, 3, 4];
    let anum = &a[0..2];
    for i in anum {
        println!("{i}");
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
