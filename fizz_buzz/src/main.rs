fn main() {
    let mut i :uint = 1;
    while i <= 100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        }
        else {
            println!("{}", i.to_string());
        }
        i += 1;
    }
}