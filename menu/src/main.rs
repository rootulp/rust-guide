// Implements http://rosettacode.org/wiki/Menu

use std::io;
use std::str;

// Print the menu followed by the prompt
fn print_both(menu: &[&str], prompt: &str) {

    // Iterate through array and print index, period, and menu item
    for (i, item) in menu.iter().enumerate() {
        println!("{}. {}", i, item);
    }

    // Print the prompt
    println!("{}", prompt);
}


// Grab the next line of input
fn next_input() -> Option<uint> {

    // Convert it to a possible int
    // let mut stdin = io::stdin();
    // let line = stdin.read_line().unwrap();
    // let trimmed = line.trim();
    let input = io::stdin().read_line()
                           .ok()
                           .expect("Failed to read line");
    let user_input: Option<uint> = input.trim().parse();

    return user_input;
}

fn select<'a>(menu: &'a [&str], prompt: &str) -> &'a str {
    // Check if menu is empty
    if menu.len() == 0 {
        return "";
    }

    // Loop until user inputs a valid menu index
    loop {

        print_both(menu, prompt);

        let user_input = next_input();

        let menu_index = match user_input {
            Some(menu_index) => menu_index,
            None      => continue
        };

        if let 0...3 = menu_index {
            return menu[menu_index];
        }
    }
}

#[cfg(not(test))]
fn main() {
    let prompt = "Choose one.";
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let menu = items.slice(0,4);
    println!("{}", select(menu, prompt));
}

// Need to add tests but having trouble simulating std io