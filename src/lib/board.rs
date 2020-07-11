use super::Square;
use std::collections::HashMap;

pub struct Board {
    squares: HashMap<u8, Square>,
}

impl Board {
    pub fn new<'a>(snakes: Vec<Square>, ladders: Vec<Square>) -> Board {
        let mut squares: HashMap<u8, Square> = HashMap::new();
        (1..101).for_each(|position| {
            if let None = squares.insert(position, Square::SimpleSquare(position)) {}
        });

        snakes.iter().for_each(|snake| {
            if let Square::Snake(head, _) = snake {
                squares.insert(head.clone(), snake.clone()).unwrap();
            }
        });

        ladders.iter().for_each(|ladder| {
            if let Square::Ladder(bottom, _) = ladder {
                squares.insert(bottom.clone(), ladder.clone()).unwrap();
            }
        });
        Board { squares }
    }

    pub fn get_square(&self, position: u8) -> Option<&Square> {
        self.squares.get(&position)
    }
}
