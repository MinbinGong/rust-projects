fn main() {
    // let s = String::from("hello");

    // // let s2 = s1;

    // // println!("{}", s1);

    // takes_ownership(s);

    // // println!("{}", s); // compile error: value borrowed here after move

    // let x = 5;

    // makes_copy(x);

    // let s1 = gives_ownership();
    // println!("{}", s1);

    // let s2 = String::from("hello");
    // println!("{}", s2);

    // let s3 = takes_and_gives_back(s2);
    // println!("{}", s3);

    // let s1 = String::from("s: hello");

    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    // let s2 = String::from("s: hello");
    // let length = calculate_length(&s2);
    // println!("The length of '{}' is {}.", s2, length);

    let mut s = String::from("s: &String");

    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str("string: &str");
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
    
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn  calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }