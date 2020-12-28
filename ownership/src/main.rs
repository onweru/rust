fn main() {
    
    let s1 = String::from("hello");
    let (s4, len) = calculate_length(s1);

    println!("The length of '{}` is {}.", s4, len);

    let _s1 = gives_ownership(); // gives_ownership movers its return value into _s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which will also move its return value into _s3

} // here, _s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. _s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some string is returned and moves out to the calling function 
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
  
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // lens() returns the length of a string
    (s, length)
}
// fn main() {

//     let s = String::from("hello");

//     takes_ownershop(s);

//     // println!("s value, {}, was moved and dropped in scope", s);

//     let x = 5;

//     makes_copy(x);

//     println!("x value, { }, is still usable, through copy", x)

//     // let _s1  = String::from("hello");
//     // let s2 = _s1.clone();
    
//     // println!("_s1 = {}, s2 = {}", _s1, s2);
//     // let _s1 = String::from("hello");
//     // let s2 = _s1;

//     // println!("{}, world!", s2);

//     // let _s = String::from("hello");

//     // let mut s = String::from("hello");

//     // s.push_str(", world!");

//     // println!("{}", s);
// }

// fn takes_ownershop(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }