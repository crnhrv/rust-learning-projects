use std::io;

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return self.height > rect.height && self.width > rect.width;
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Calculates the area of a rectangle");

    println!("Please enter the width of the rectangle");
    let width: u32 = get_integer_input();

    println!("Now enter its height");
    let height: u32 = get_integer_input();
    let rectangle = Rectangle {
        width,
        height
    };

    println!("The area is: {}", &rectangle.area());

    can_hold(&rectangle);
}

fn can_hold(rect1: &Rectangle) {
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
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
