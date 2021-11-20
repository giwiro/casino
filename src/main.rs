use crate::card::Shape;

mod card;
mod deck;

fn main() {
    let d = deck::Deck::create_deck(2);
    println!("Hello, world! => {:#?}", d);
}
