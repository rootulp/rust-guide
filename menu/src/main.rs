// Implements http://rosettacode.org/wiki/Menu

use std::io;

// Print the menu followed by the prompt
fn print_both(menu: [&str, ..4], prompt: &str) {

    // Iterate through array and print index, period, and menu item
    for (i, item) in menu.iter().enumerate() {
        println!("{}. {}", i, item);
    }

    // Print the prompt
    println!("{}", prompt);

}

// Grab the next line of input
fn next_input() -> int {

    // Convert it to a possible int
    let mut stdin = io::stdin();
    let line = stdin.read_line().unwrap();
    let trimmed: &str = line.trim();
    let possible: Option<int> = from_str(trimmed);

    // If possible num is between 0 and 3, return it. 
    // Otherwise return -1 <= I don't think this is good practice
    match possible {
        None => return -1,
        Some(possible) => match possible {
            0...3 => return possible,
            _=> return -1
        }
    }
}

// Couldn't figure out how to make select return an element of menu
// Got a 'missing lifetime specifier' error when trying to return &str
fn select(menu: [&str, ..4], prompt: &str) -> String {

    // Check if menu is empty
    if menu.len() == 0 {
        return "".to_string()
    }

    // Loop until user inputs a valid menu index
    loop {

        print_both(menu, prompt);

        let menu_index: int = next_input();

        if let 0...3 = menu_index {
            return menu[menu_index as uint].to_string();
        }
    }
}

#[cfg(not(test))]
fn main() {
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let prompt = "Choose one.";
    println!("{}", select(items, prompt));
}

// Need to add tests but having trouble simulating std io