fn main() {
    println!("Hello, world!");

    another_function(5, 6.0);
}

fn another_function(_x: i32, _y: f64) {
    println!("The value of x is: {}, y is: {}", _x, _y);
}
