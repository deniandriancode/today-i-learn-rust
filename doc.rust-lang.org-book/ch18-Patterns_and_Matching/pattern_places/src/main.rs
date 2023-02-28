fn main() {
    let point = (32, 44);
    func_param(&point);
}

fn func_param(&(x, y): &(i32, i32)) {
    println!("Current coordinate is {x} {y}");
}

fn let_statement() {
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    println!("{x} {y} {z}");
}

fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
	println!("{} is at index {}", value, index);
    }
}

fn while_let_loop() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
	println!("{}", top);
    }
}

fn if_let_arm() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
	println!("Using your favorite color, {}", color);
    } else if is_tuesday {
	println!("Tuesday is green day!");
    } else if let Ok(a) = age {
	if a > 30 {
	    println!("More than 30!");
	} else {
	    println!("Less than or equal 30!");
	}
    } else {
	println!("Using blue as background color!");
    }
}

fn match_arm() {
    // match VALUE {
    // 	PATTERN => EXPRESSION,
    // 	PATTERN => EXPRESSION,
    // 	PATTERN => EXPRESSION,
    // }
    let v = Some(8);
    let v: Option<i32> = None;

    match v {
	None => println!("Got none"),
	Some(i) => println!("Wow, a number: {i}"),
    }
}
