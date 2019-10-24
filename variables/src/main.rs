fn main() {
    let tup: (i32, u64, f32) = (500, 1, 2.0);

    let (x, y, z) = tup;
    println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];
    println!("The value of element is: {}", element);
}
