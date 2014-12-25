// Implements http://rosettacode.org/wiki/Menu

use std::io;

// Print the menu followed by the prompt
fn print_both(menu: [&str, ..4], prompt: &str) {

    // Iterate through array and print index then period then menu item
    for i in range(0, 4u) { // Hard coded number 4 here
        println!("{}. {}", i, menu[i]);
    }

    // Print the prompt
    println!("{}", prompt);

}

// Grab the next line of input
fn next_input() -> int {

    let mut stdin = io::stdin();
    let line: io::IoResult<String> = stdin.read_line();
    let string: String = line.unwrap();
    let trimmed: &str = string.as_slice().trim();
    let pos_num: Option<int> = from_str(trimmed);

    match pos_num {
        Some(num) => 
            match num {
                0...3 => return num,
                 _=> return -1
            },
        None => return -1
    }
}



// Couldn't figure out how to make select return an element of menu
// Got a 'missing lifetime specifier' error when trying to return &str
fn select(menu: [&str, ..4], prompt: &str) -> String {

    // Loop until user inputs a valid number
    loop {
        print_both(menu, prompt);
        let numero: int = next_input();
        match numero{
            0 => return menu[0 as uint].to_string(),
            1 => return menu[1 as uint].to_string(),
            2 => return menu[2 as uint].to_string(),
            3 => return menu[3 as uint].to_string(),
            _=> print_both(menu, prompt),
        }
    }
    return "No Solution".to_string();
}

fn main() {
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let prompt = "Choose one.";
    println!("{}", select(items, prompt));
}
