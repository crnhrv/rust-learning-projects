use add_one;

pub fn add_two(x: u32) -> u32 {
    let y = add_one::add_one(x);
    add_one::add_one(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}