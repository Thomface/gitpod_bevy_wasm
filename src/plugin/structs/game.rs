use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(PartialEq)]
pub enum Action {
    Attack,
    Defend,
    Spell
}

#[derive(PartialEq)]
pub enum Target {
    All,
    SingleEnemy,
    AllEnemy,
    Everybody,
    Random
}

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

    pub fn endTurn(&mut self) {
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
                self.refillDraw();
                self.draw();
            }
        }
    }

    pub fn refillDraw(&mut self) {
        loop {
            match self.discard.pop() {
                Some(card) => {
                    self.draw.push(card);
                }
                _ => break
            }
        }
    }

    pub fn shuffleDraw(&mut self) {
        self.draw.shuffle(&mut thread_rng());
    }
}