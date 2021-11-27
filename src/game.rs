use crate::card;
use crate::deck::Deck;
use crate::hand::Hand;

#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub min_bet: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameResult {
    Win,
    Lose,
    Tie,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameMove {
    Hit,
    Stand,
    Split,
}

impl Game {
    pub fn new(deck_size: u8, min_bet: u32) -> Game {
        Game {
            deck: Deck::new(deck_size),
            min_bet,
        }
    }

    fn can_afford(&self, total: &u32) -> bool {
        return *total >= self.min_bet;
    }

    fn draw_card(&mut self, face_up: bool) -> Result<card::Card, &'static str> {
        if let Some(mut c) = self.deck.draw() {
            if face_up {
                c.flip();
            }
            Ok(c)
        } else {
            Err("No enough cards")
        }
    }

    pub fn play(&mut self, total: &mut u32) -> Result<GameResult, &'static str> {
        let mut bet_multiplier = 1;

        if !self.can_afford(total) {
            return Err("Cannot afford keep playing");
        }

        let mut house_hand = Hand::new();
        let mut player_hand = Hand::new();

        {// First draw
            match self.draw_card(true) {
                Ok(card) => player_hand.add_card(card),
                Err(message) => return Err(message),
            }

            match self.draw_card(true) {
                Ok(card) => house_hand.add_card(card),
                Err(message) => return Err(message),
            }
        }

        {// Second draw
            match self.draw_card(true) {
                Ok(card) => player_hand.add_card(card),
                Err(message) => return Err(message),
            }

            match self.draw_card(true) {
                Ok(card) => house_hand.add_card(card),
                Err(message) => return Err(message),
            }
        }

        println!("\t Casino => {:?}\tPlayer => {:?}", house_hand.sum(), player_hand.sum());

        let house_sum = house_hand.sum();
        let player_sum = player_hand.sum();

        if player_sum > 21 {
            *total -= self.min_bet;
            Ok(GameResult::Lose)
        } else if player_sum == 21 {
            *total += self.min_bet * bet_multiplier + (self.min_bet / 2);
            Ok(GameResult::Win)
        } else if house_sum > 21 {
            *total += self.min_bet * bet_multiplier;
            Ok(GameResult::Win)
        } else if house_sum < player_sum {
            *total += self.min_bet * bet_multiplier;
            Ok(GameResult::Win)
        } else if house_sum > player_sum {
            *total -= self.min_bet;
            Ok(GameResult::Lose)
        } else {
            Ok(GameResult::Tie)
        }
    }
}