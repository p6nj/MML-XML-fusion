mod structure;
use structure::{Item, Wrapper};

pub fn add(left: usize, right: usize) -> usize {
    let a = Wrapper::Song(String::from("hi"),vec![Wrapper::Singleton(Item::Note(0u8))]);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
