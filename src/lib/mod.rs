mod board;
pub mod game;
mod player;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Square {
    SimpleSquare(u8),
    Snake(u8, u8),
    Ladder(u8, u8),
}

impl Square {
    fn position(&self) -> u8 {
        match self {
            Square::SimpleSquare(position) => position,
            Square::Snake(head, _) => head,
            Square::Ladder(bottom, _) => bottom,
        }
        .clone()
    }
    fn next(&self) -> Square {
        match self {
            Square::SimpleSquare(_) => self.clone(),
            Square::Snake(_, tail) => Square::SimpleSquare(tail.clone()),
            Square::Ladder(_, top) => Square::SimpleSquare(top.clone()),
        }
    }
}

trait Dice {
    fn roll(&self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1, 7)
    }
}
