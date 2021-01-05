fn main() {
    let some_u8_value = Some(0u8);
    // verbose using match
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => ()
    // }

    // simpler using if let

    if let Some(3) = some_u8_value {
        println!("three");
    }
}
