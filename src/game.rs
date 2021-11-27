use crate::card::Card;use crate::deck::Deck;
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
pub enum GameTermination {
    NoCards,
    NoFunds,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameMove {
    Hit,
    Stand,
    Split,
}

impl Game {
    pub fn new(deck_size: u16, min_bet: u32) -> Game {
        Game {
            deck: Deck::new(deck_size),
            min_bet,
        }
    }

    fn can_afford_play(&self, total: &u32) -> bool {
        return *total >= self.min_bet;
    }

    fn draw_card(&mut self, face_up: bool) -> Result<Card, GameTermination> {
        if let Some(mut c) = self.deck.draw() {
            if face_up {
                c.flip();
            }
            Ok(c)
        } else {
            Err(GameTermination::NoCards)
        }
    }

    fn assess_winner(&self, total: &mut u32, bet_multiplier: u32, house_hand: &Hand, player_hand: &Hand) -> GameResult {
        let house_sum = house_hand.sum();
        let player_sum = player_hand.sum();

        if player_sum > 21 {
            *total -= self.min_bet;
            GameResult::Lose
        } else if player_hand.is_black_jack() {
            *total += self.min_bet * bet_multiplier + (self.min_bet / 2);
            GameResult::Win
        } else if house_sum > 21 {
            *total += self.min_bet * bet_multiplier;
            GameResult::Win
        } else if house_sum < player_sum {
            *total += self.min_bet * bet_multiplier;
            GameResult::Win
        } else if house_sum > player_sum {
            *total -= self.min_bet;
            GameResult::Lose
        } else {
            GameResult::Tie
        }
    }

    pub fn play(&mut self, total: &mut u32) -> Result<GameResult, GameTermination> {
        let mut bet_multiplier: u32 = 1;

        if !self.can_afford_play(total) {
            return Err(GameTermination::NoFunds);
        }

        let mut house_hand = Hand::new();
        let mut player_hand = Hand::new();
        let mut split_player_hand = Hand::new();

        {// First draw
            match self.draw_card(true) {
                Ok(card) => player_hand.add_card(card),
                Err(term) => return Err(term),
            }

            match self.draw_card(true) {
                Ok(card) => house_hand.add_card(card),
                Err(term) => return Err(term),
            }
        }

        {// Second draw
            match self.draw_card(true) {
                Ok(card) => player_hand.add_card(card),
                Err(term) => return Err(term),
            }

            match self.draw_card(true) {
                Ok(card) => house_hand.add_card(card),
                Err(term) => return Err(term),
            }
        }

        let result = self.assess_winner(total, bet_multiplier, &house_hand, &player_hand);

        println!("-------------");
        println!("[{:?}]\t\t\tCasino => {:?}\t\t\tPlayer => {:?}", result, house_hand.sum(), player_hand.sum());

        Ok(result)
    }
}