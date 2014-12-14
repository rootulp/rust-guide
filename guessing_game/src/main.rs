use std::io;
use std::rand;

fn main() {
    println!("Guess a number between 1 and 10.");

    let secret_number = (rand::random::<uint>() % 10u) + 1u;

    loop {

        let input = io::stdin()
                            .read_line()
                            .ok()
                            .expect("Failed to read line");
        let input_num: Option<uint> = from_str(input.as_slice().trim());

        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("Please input a number");
                continue;
            }
        };

        match cmp(num, secret_number) {
            Less    => println!("Too small! Keep guessing."),
            Greater => println!("Too big! Keep guessing."),
            Equal   => {
                println!("You win!");
                return;
            },
        }
    }
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}