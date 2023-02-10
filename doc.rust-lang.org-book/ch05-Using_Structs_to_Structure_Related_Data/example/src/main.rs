#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementing methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30),
        height: 50,
    };
    let sqr1 = Rectangle::square(9);

    if rect1.can_hold(&sqr1) {
        println!("ok");
    }

    println!("Rectangle is {:?}", rect1);
    println!("Rectangle is {:#?}", rect1);
    println!("The area is {}", area(&rect1));

    println!("");
    println!("The area method result is {}", rect1.area());
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
