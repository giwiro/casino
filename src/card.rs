#[derive(Debug)]
pub enum Shape {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    shape: Shape,
    number: u8
}

impl Card {
    pub fn create_card(shape: Shape, number: u8) -> Card {
        Card {
            shape,
            number,
        }
    }
}
