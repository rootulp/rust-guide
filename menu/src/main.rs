// Implements http://rosettacode.org/wiki/Menu

fn print_both(menu: [&str, ..4], prompt: &str) {
    println!("{}", prompt);

    for i in range(0, 4u) {
        println!("{}. {}", i, menu[i]);
    }
}

fn next_line() -> Option<int> {
    use std::io;

    let mut stdin = io::stdin();

    let err_line: io::IoResult<String> = stdin.read_line();
    let line: String = err_line.unwrap();

    let line_no_extra_whitespace: &str = line.as_slice().trim();
    let possible_number: Option<int> = from_str(line_no_extra_whitespace);
    return possible_number;
}

fn menu(menu: [&str, ..4], prompt: &str) -> String {

    let mut done = false;
    while !done {
        print_both(menu, prompt);
        let possible_number: Option<int> = next_line(); 

        match possible_number {
            Some(n) => match n {
                0...3 => return menu[n as uint].to_string(),
                _=> print_both(menu, prompt)
            },
            None => print_both(menu, prompt)
        }
    }
    //return menu[n.parse::<uint>()].to_string()
    return "goodbye".to_string();
}

fn main() {
    let items = ["fee fie", "huff and puff", "mirror mirror", "tick tock"];
    let prompt = "Choose one.";
    let mut result = menu(items, prompt);
    println!("{}", result);
}
