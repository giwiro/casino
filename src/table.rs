use crate::game::{Game, GameTermination};
use crate::GameResult;

#[derive(PartialEq, Eq, Debug)]
pub enum TableOutcome {
    Win,
    Lose,
}

pub struct Table {
    game: Game,
    pub total: u32,
    max_total_to_win: u32,
    hands_played: u32,
    hands_won: u32,
    hands_lost: u32,
}

impl Table {
    pub fn new(total: u32, max_total_to_win: u32, deck_size: u16, min_bet: u32) -> Table {
        let game = Game::new(deck_size, min_bet);

        Table {
            game,
            total,
            max_total_to_win,
            hands_played: 0,
            hands_won: 0,
            hands_lost: 0,
        }
    }

    pub fn start_game(&mut self) -> TableOutcome {
        loop {
            match self.game.play(&mut self.total) {
                Ok(result) => {
                    self.hands_played += 1;
                    if self.game.deck.should_reshuffle() {
                        self.game.deck.reset_cards();
                    }
                    match result {
                        GameResult::Win => self.hands_won += 1,
                        GameResult::Lose => self.hands_lost += 1,
                        GameResult::Tie => (),
                    }
                    if self.total >= self.max_total_to_win {
                        return TableOutcome::Win;
                    }
                },
                Err(term) => {
                    eprintln!("{:?}", term);
                    println!("Hands played = {:}", self.hands_played);
                    println!("Hands won = {:}", self.hands_won);
                    if term == GameTermination::NoFunds {
                        return TableOutcome::Lose;
                    }
                }
            }
        }
    }
}
