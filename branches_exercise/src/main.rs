fn main() {
    celcius_to_farenheit(25);
}

fn celcius_to_farenheit(value: i32) {
    let farenheit_value = (value * 2) + 30;
    println!("{} Celcius equals {} farenheit", value, farenheit_value);
}


