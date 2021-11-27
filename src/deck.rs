use std::collections::VecDeque;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::Shape;

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new(size: u8) -> Deck {
        let mut cards = vec![];
        for _ in 0..size {
            cards.extend(Deck::create_single_deck())
        }
        cards.shuffle(&mut rand::thread_rng());
        Deck {
            cards: VecDeque::from_iter(cards),
        }
    }

    fn create_single_deck() -> Vec<Card> {
        let mut cards = vec!();
        for n in 1..14 {
            cards.push(Card::new(Shape::Clubs, n, false));
            cards.push(Card::new(Shape::Diamonds, n, false));
            cards.push(Card::new(Shape::Hearts, n, false));
            cards.push(Card::new(Shape::Spades, n, false));
        }
        cards
    }

    pub fn draw(&mut self) -> Option<Card> {
        return self.cards.pop_front();
    }
}