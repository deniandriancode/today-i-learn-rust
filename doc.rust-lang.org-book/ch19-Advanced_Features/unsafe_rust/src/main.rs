static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    extern "C" {
	fn abs(input: i32) -> i32;
    }

    unsafe {
	println!("Aboslute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);
}

unsafe fn dangerous() {
    
}

fn raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
	println!("r1 is {}", *r1);
	println!("r2 is {}", *r2);
    }
}
