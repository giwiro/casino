use std::cmp::min;
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

    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].number == self.cards[1].number
    }

    pub fn is_black_jack(&self) -> bool {
        self.cards.len() == 2 && self.sum() == 21
    }

    pub fn split(&mut self) -> Result<Card, &'static str> {
        if !self.is_pair() {
            return Err("Cannot split hand");
        }

        if let Some(c) = self.cards.pop() {
            Ok(c)
        } else {
            Err("Could not split")
        }
    }

    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn sum(&self) -> u8 {
        let has_as = false;


        self.cards.iter().filter(|c| c.face_up).fold(0, |a, b| a + min(b.number, 10))
    }
}