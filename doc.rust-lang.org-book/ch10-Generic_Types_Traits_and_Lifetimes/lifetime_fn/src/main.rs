fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    invalid_lifetime();
}

fn valid_lifetime() {
    let s1 = String::from("asdf");

    {
        let s2 = String::from("wywiueytiwuety");
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest: {}", result);
    }
}

fn invalid_lifetime() {
    let s1 = String::from("asdf");
    let result;

    {
        let s2 = String::from("a");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("The longest: {}", result);
}
