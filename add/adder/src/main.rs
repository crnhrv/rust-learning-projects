use add_one;
use add_two;
use rand:: { thread_rng, Rng}; 

fn main() {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(1..100);
    println!(
        "Hello, world! {} plus one is {}!",
        rand_num,
        add_one::add_one(rand_num)
    );

    println!(
        "And {} plus two is {}!",
        rand_num,
        add_two::add_two(rand_num)
    );
}