use std::io;

fn temperature_converter() {
    println!("Please input the temperature to be convert in degree C eg 32.7");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input");

    let temperature: f32 = temperature.trim().parse().expect("Not and integer");
    println!("The output is {}F", temperature * 9.0 / 5.0 + 32.0);
}

fn main() {
    println!("Hello, world!");
    temperature_converter()
}
