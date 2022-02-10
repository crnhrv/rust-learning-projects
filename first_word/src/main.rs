use std::io;

fn main() {
    println!("Enter a sentence to return the first word");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");

    println!("First word: {}", get_first_word(&input));
}

fn get_first_word(input: &str) -> &str {
    let bytes = input.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &input[0..i];
        }
    }

    return &input[..];
}
