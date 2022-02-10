use std::io;

fn main() {
    println!("Enter a sentence to return the first word");
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");

    println!("First word: {}", get_first_word(&input));
}

fn get_first_word(input: &String) -> String {
    let splits = input.split(" ");

    for elem in splits {
        return String::from(elem);
    }

    return String::from(input);
}
