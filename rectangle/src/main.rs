use std::io;

fn main() {
    println!("Calculates the area of a rectangle");

    println!("Please enter the width of the rectangle");
    let width: u32 = get_integer_input();

    println!("Now enter its height");
    let height: u32 = get_integer_input();

    println!("The area is: {}", width * height);
}

fn get_integer_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        match input.trim().parse::<u32>() {
            Ok(val) => {
                break val;
            }
            Err(_) => {
                if input.trim().to_lowercase() == String::from("x") {
                    std::process::exit(0);
                }
                println!("Please enter a number (or X to quit)");
            }
        }
    }
}
