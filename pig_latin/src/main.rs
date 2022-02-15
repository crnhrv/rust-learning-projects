use std::io;

fn main() {
    println!("Enter a sentence to pig-latinify!");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
    
        if !input.is_ascii() {
            println!("Please use only the latin alphabet!");
            continue;    
        }

        println!("{}", pig_latinify(&input.trim()));
    }
}

fn pig_latinify(string: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut latinified_word = vec![];

    let words = string.split_ascii_whitespace();
    for word in words {
        let mut char_to_replace = '\0';
        let mut formatted_word = String::new();

        if let Some(val) = word.chars().last() {
            if !val.is_alphabetic() {
                char_to_replace = val;
                formatted_word = word.replace(char_to_replace, "");
            } else {
                formatted_word = word.to_string();
            }
        }

        let mut new_word = String::new();
        if let Some(val) = formatted_word.chars().nth(0) {
            if vowels.contains(&val) {
                new_word = format!("{}-{}", formatted_word, "hay");
            } else {
                new_word = format!("{}-{}{}", &formatted_word[1..], val, "ay");
            }
        }

        if char_to_replace != '\0' {
            new_word += &char_to_replace.to_string();
        }

        latinified_word.push(new_word);
    }

    return latinified_word.join(" ");
}
