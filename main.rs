use std::io;

fn main() {
    println!("What's your name?");

    let input = io::stdin()
                        .read_line()
                        .ok()
                        .expect("Failed to read line");

    println!("Hello, {}", input);
}