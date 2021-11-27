use std::collections::VecDeque;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::Shape;

#[derive(Debug)]
pub struct Deck {
    size: u16,
    cards: VecDeque<Card>,
    cut_from_end: u16,
}

impl Deck {
    pub fn new(size: u16) -> Deck {
        Deck {
            size,
            cards: VecDeque::from_iter(Deck::provided_shuffled_cards(size)),
            cut_from_end: if size < 4 { 8 } else { 78 }
        }
    }

    pub fn reset_cards(&mut self) {
        self.cards = VecDeque::from_iter(Deck::provided_shuffled_cards(self.size));
    }

    pub fn should_reshuffle(&self) -> bool {
        self.cards.len() <= self.cut_from_end as usize
    }

    fn provided_shuffled_cards(size: u16) -> Vec<Card> {
        let mut cards = vec![];
        for _ in 0..size {
            cards.extend(Deck::create_single_deck())
        }
        cards.shuffle(&mut rand::thread_rng());
        cards
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
        self.cards.pop_front()
    }
}