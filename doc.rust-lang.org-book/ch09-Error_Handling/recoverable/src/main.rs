use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening file {:?}", other_error);
    //         }
    //     }
    // };
    let greeting_file = File::open("hi.txt").expect("hi.txt should be included in this project");
}
