use crate::card::Card;

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
        }
    }

    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].number == self.cards[1].number
    }

    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn sum(&self) -> u8 {
        self.cards.iter().filter(|c| c.face_up).fold(0, |a, b| a + b.number)
    }
}