fn main() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("changed x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("max points are sent in stone as {}", MAX_POINTS);

    // shadowing variables
    let y = 6;
    let y = y + 1;
    let y = y * 3;
    println!("after shadowing y twice, its value is now {}", y);

    // shadowing => type change
    let spaces = "  ";
    let spaces = spaces.len();

    println!("spaces was a string, now it's {}", spaces);
}
