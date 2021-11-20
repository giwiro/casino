use crate::card::Card;
use crate::Shape;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn create_single_deck() -> Vec<Card> {
        let mut cards = vec!();
        for n in 1..14 {
            cards.push(Card::create_card(Shape::Clubs, n));
            cards.push(Card::create_card(Shape::Diamonds, n));
            cards.push(Card::create_card(Shape::Hearts, n));
            cards.push(Card::create_card(Shape::Spades, n));
        }
        cards
    }

    pub fn create_deck(size: u8) -> Deck {
        let mut deck = Deck {
            cards: vec![],
        };
        for _ in 0..size {
            deck.cards.extend(Deck::create_single_deck())
        }
        deck
    }

    pub fn shuffle_deck(&mut self) {
        self.cards[0] = Card {
            shape: Shape::Clubs,
            number: 3,
        }
    }
}