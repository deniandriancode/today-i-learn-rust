fn main() {
    let v = vec![1, 2, 3];

    let mut ve = Vec::new();

    ve.push(2);
    ve.push(5);
    ve.push(1);
    ve.push(8);

    let third: &i32 = &ve[2];
    println!("The third element is {third}");

    let third: Option<&i32> = ve.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let v = vec![100, 44, 66];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 44, 66];
    for i in &mut v {
        println!("{i}");
    }
}
