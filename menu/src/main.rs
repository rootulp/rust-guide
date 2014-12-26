// Implements http://rosettacode.org/wiki/Menu

use std::io;

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
fn select<'a>(menu: &'a [&str], prompt: &str) -> &'a str {
    // Check if menu is empty
    if menu.len() == 0 {
        return "";
    }

    // Loop until user inputs a valid menu index
    loop {

        print_both(menu, prompt);

        let menu_index: int = next_input();

        if let 0...3 = menu_index {
            return menu[menu_index as uint];
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