use crate::card::Shape;
use crate::game::GameResult;
use crate::table::Table;

mod card;
mod deck;
mod hand;
mod game;
mod table;

fn main() {
    let mut table = Table::new(100, 8, 10);
    table.start_game();
}
