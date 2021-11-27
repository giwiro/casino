use crate::card::Shape;
use crate::game::GameResult;

mod card;
mod deck;
mod hand;
mod game;

fn main() {
    let mut total = 100;
    let mut g = game::Game::new(4, 10);

    loop {
        match g.play(&mut total) {
            Ok(result) => {
                println!("{:?} => {:}", result, total);
            }
            Err(message) => {
                eprintln!("{:?}", message);
                break;
            }
        }
    }
}
