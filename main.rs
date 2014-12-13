struct Point {
  x : int,
  y : int,
}

fn main() {
    let point1 = Point {x : 3, y : 2};
    println!("Point1 is at {}, {}", point1.x, point1.y);
}