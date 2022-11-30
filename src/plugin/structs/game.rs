use bevy::prelude::Component;

use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(Copy, Clone)]
pub enum Action {
    Attack,
    Defend,
    Spell
}

#[derive(Copy, Clone)]
pub enum Target {
    Player,
    All,
    SingleEnemy,
    AllEnemy,
    Everybody,
    Random
}

#[derive(Clone,Component,Copy)]
pub struct Card {
    action: Action,
    target: Target,
    value: u8
}

impl Card {
    pub fn new(action: Action, target: Target, value: u8) -> Self {
        Card {
            action: action,
            target: target,
            value: value,
        }
    }
}

pub struct Game {
    pub draw: Vec<Card>,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            draw: Vec::new(),
            hand: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn end_turn(&mut self) {
        loop {
            match self.hand.pop() {
                Some(card) => {
                    self.discard.push(card);
                }
                _ => break
            }
        }
    }

    pub fn draw(&mut self) {
        match self.draw.pop() {
            Some(card) => {
                self.hand.push(card);
            }
            _ => {
                if self.refill_draw() > 0 {
                    self.draw();
                }
            }
        }
    }

    pub fn refill_draw(&mut self) -> i32 {
        let mut nb = 0;
        loop {
            match self.discard.pop() {
                Some(card) => {
                    self.draw.push(card);
                    nb += 1;
                }
                _ => break
            }
        }
        return nb;
    }

    pub fn shuffle_draw(&mut self) {
        self.draw.shuffle(&mut thread_rng());
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut game = Game::new();

        game.draw.push(Card::new(Action::Attack, Target::SingleEnemy, 6));
        game.draw.push(Card::new(Action::Attack, Target::SingleEnemy, 6));
        game.draw.push(Card::new(Action::Defend, Target::Player, 5));
        game.draw.push(Card::new(Action::Defend, Target::Player, 5));
        return game;
    }
}
