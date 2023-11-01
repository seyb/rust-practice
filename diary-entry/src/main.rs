use std::fmt::Debug;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    match fs::write("diary.txt", "this is a content") {
        Err(e) => print!("{}", e.to_string()),
        Ok(_x) => print!("Completed !!")
    }

    let content = fs::read("diary.txt");

    match content {
        Ok(c) => print!("{:?}", String::from_utf8(c)),
        Err(e) => print!("{}", e.to_string()),
    }


    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // Crée le fichier s'il n'existe pas
        .open("journal.txt")
        .unwrap();

    let entry = "Voici une nouvelle entrée dans le journal.\n";
    file.write_all(entry.as_bytes()).unwrap();

}
