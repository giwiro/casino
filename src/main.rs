use crate::card::Shape;
use crate::game::GameResult;
use crate::table::{Table, TableOutcome};

mod card;
mod deck;
mod hand;
mod game;
mod table;

fn main() {
    let mut table = Table::new(100, 200,8, 10);
    let outcome = table.start_game();

    match outcome {
        TableOutcome::Win => {
            println!("Won {:}", table.total)
        },
        TableOutcome::Lose => {
            println!("Lost")
        },
    }
}
