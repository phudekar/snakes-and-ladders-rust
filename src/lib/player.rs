use super::Dice;
use super::Square;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    square: Square,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name: name,
            square: Square::SimpleSquare(0),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn move_to(&mut self, square: Square) {
        self.square = square;
    }

    pub fn square(&self) -> Square {
        self.square
    }

    pub fn position(&self) -> u8 {
        self.square().position()
    }
}

impl Display for Player {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{} - {}", self.name, self.position())
    }
}

impl Dice for Player {}
