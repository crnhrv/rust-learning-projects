use std::io;

fn main() {
    println!("Generate the nth Fibonnaci Number");
    println!("Please pick a number");

    let mut input = String::new();

    let input: usize = loop {
        io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim().parse() {
            Ok(value) => {
                break value;
            },
            Err(_) => {
                println!("Please enter a valid number!");
                input = String::new();
                continue;
            }          
        }
    };
    
    let nth_fib = generate_fibonnaci_number(input);

    println!("{}th fibonnaci number: {}", input, nth_fib);
}

fn generate_fibonnaci_number(ordinal: usize) -> usize {

    let mut vec = vec![0, 1, 1];

    for elem in  2..ordinal { 
        let num = vec[elem] + vec[elem-1];
        vec.push(num);
    };

    return vec[ordinal];
}

