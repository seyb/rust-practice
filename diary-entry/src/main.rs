use std::fmt::Debug;
use std::fs;

fn main() {
    println!("Hello, world!");
    match fs::write("diary.txt", "this is a content") {
        Err(e) => print!("{}", e.to_string()),
        Ok(x) => print!("Completed !!")
    }

    let content = fs::read("diary.txt");


    match content {
        Ok(c) => print!("{:?}", String::from_utf8(c)),
        Err(e) => print!("{}", e.to_string()),
    }
}
