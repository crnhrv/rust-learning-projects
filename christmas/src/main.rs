fn main() {
    let christmas_gifts: [&str; 12] = [
        "partridge in a pair tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let days: [&str; 12] = [
        "First",
        "Second",
        "Third",
        "Fourth",
        "Fifth",
        "Sixth",
        "Seventh",
        "Eighth",
        "Ninth",
        "Tenth",
        "Eleventh",
        "Twelth",
    ];

    let ordinals: [&str; 12] = [
        "A",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", days[i]);
        for j in (0..i+1).rev() {           
            if j == 1 {
                println!("{} {}, and", ordinals[j], christmas_gifts[j]);
            } else {
                println!("{} {}", ordinals[j], christmas_gifts[j]);
            }
        }
        println!("-----------------------------------------")
    }
}
