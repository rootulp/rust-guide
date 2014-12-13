fn next_two(x: int) -> (int, int) { 
  (x + 1i, x + 2i) 
}

fn main() {
    let (x, y) = next_two(5i);
    println!("The numbers after 5 are: {}, {}", x, y);
}