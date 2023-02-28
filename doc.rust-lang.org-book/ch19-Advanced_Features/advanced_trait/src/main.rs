use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
	Point {
	    x: self.x + other.x,
	    y: self.y + other.y,
	}
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
	println!("This is your captain speaking!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
	println!("Hehehe!");
    }
}

impl Human {
    fn fly(&self) {
	println!("*woooooz!*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    let dog = Dog;
    println!("The dog baby name is {}", <Dog as Animal>::baby_name());
}
