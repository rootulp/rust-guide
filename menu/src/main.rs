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
fn next_input() -> Option<int> {

    let mut stdin = io::stdin();
    let err_line: io::IoResult<String> = stdin.read_line();
    let line: String = err_line.unwrap();
    let line_no_extra_whitespace: &str = line.as_slice().trim();

    // Return potential number
    return from_str(line_no_extra_whitespace);

}

// Couldn't figure out how to make select return an element of menu
// Got a 'missing lifetime specifier' error when trying to return &str
fn select(menu: [&str, ..4], prompt: &str) -> String {

    // Loop until user inputs a valid number
    loop {
        print_both(menu, prompt);
        let user_input: Option<int> = next_input(); 

        match user_input {
            Some(num) => match num {
                0...3 => return menu[num as uint].to_string(),
                _=> print_both(menu, prompt)
            },
            None => print_both(menu, prompt)
        }
    }
    return "No Solution".to_string();
}

fn main() {
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let prompt = "Choose one.";
    println!("{}", select(items, prompt));
}
