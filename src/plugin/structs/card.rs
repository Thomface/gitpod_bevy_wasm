
#[derive(PartialEq)]
enum Type {
    Attack,
    Defend,
    Spell
}

#[derive(PartialEq)]
enum Target {
    All,
    SingleEnemy,
    AllEnemy,
    Everybody,
    Random
}

pub struct Card {
    type: Type,
    target: Target,
    value: u8
}

impl Card {
    pub fn new(type: Type, target: Target, value: u8) -> Self {
        type: type,
        target: target,
        value: value,
    }
}