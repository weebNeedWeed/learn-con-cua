#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: u8,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, size: u8) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_contain_1_shoes() {
        let shoes = vec![
            Shoe {
                size: 32,
                style: String::from("conheo"),
            },
            Shoe {
                size: 19,
                style: String::from("conmeo"),
            },
        ];

        let filtered = shoes_in_size(shoes, 32);

        assert_eq!(
            filtered,
            vec![Shoe {
                size: 32,
                style: String::from("conheo")
            },]
        );
    }
}
