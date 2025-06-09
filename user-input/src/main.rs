use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim(); // Remove newline

    println!("Hi, {name}! It's very nice to meet you.");
}
