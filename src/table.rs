use crate::game::{Game, GameTermination};
use crate::GameResult;

pub struct Table {
    game: Game,
    total: u32,
    hands_played: u32,
    hands_won: u32,
    hands_lost: u32,
}

impl Table {
    pub fn new(total: u32, deck_size: u16, min_bet: u32) -> Table {
        let game = Game::new(deck_size, min_bet);

        Table {
            game,
            total,
            hands_played: 0,
            hands_won: 0,
            hands_lost: 0,
        }
    }

    pub fn start_game(&mut self) {
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
                },
                Err(term) => {
                    eprintln!("{:?}", term);
                    println!("Hands played = {:}", self.hands_played);
                    println!("Hands won = {:}", self.hands_won);
                    break;
                }
            }
        }
    }
}
