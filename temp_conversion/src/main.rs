use std::io;

fn main() {
    println!("Do you want to convert to Celsius or Fahrenheit?");
    println!("Press 1 for Celsius, 2 for Fahrenheit");

    let mut choice = String::new();

    let choice: u32 = loop {
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input");
        match choice.trim().parse() {
            Ok(value) => {
                if value != 1 && value != 2 {
                    println!("Please enter either 1 or 2");
                } else {
                    break value;
                }
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
    };

    println!("What temperature would you like to convert?");

    let mut temperature = String::new();

    let temperature: u32 = loop {
        io::stdin()
            .read_line(&mut temperature)
            .expect("Error reading input");
        match temperature.trim().parse() {
            Ok(value) => break value,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
    };

    if choice == 1 {
        println!(
            "{}F is {}C",
            temperature,
            fahrenheit_to_celsius(temperature)
        );
    } else {
        println!(
            "{}C is {}F",
            temperature,
            celsius_to_fahrenheit(temperature)
        );
    }
}

fn celsius_to_fahrenheit(value: u32) -> u32 {
    return (value * 9 / 5) + 32;
}

fn fahrenheit_to_celsius(value: u32) -> u32 {
    return (value - 32) * 5 / 9;
}
