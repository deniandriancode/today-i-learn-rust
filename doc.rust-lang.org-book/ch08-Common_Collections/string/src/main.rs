fn main() {
    let s1 = String::from("incomplete");
    let s2 = String::from(" module");
    let s = s1 + &s2;
    println!("{}", s);

    let s1 = String::from("macro");
    let s2 = String::from("cplusplus");
    let s3 = String::from("backward_compatibility");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{}", s);

    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    }
}
