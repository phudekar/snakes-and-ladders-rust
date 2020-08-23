use super::board::Board;
use super::player::Player;
use super::{Dice, Square};
use std::collections::HashMap;

pub struct Game {
    players: HashMap<String, Player>,
    board: Board,
    current_player_index: usize,
}

impl Game {
    pub fn new(number_of_players: u8, snakes: Vec<Square>, ladders: Vec<Square>) -> Game {
        let mut players = HashMap::new();
        (1..=number_of_players).for_each(|n| {
            let name = format!("Player_{}", n);
            players.insert(name.clone(), Player::new(name));
        });
        Game {
            players,
            board: Board::new(snakes, ladders),
            current_player_index: usize::from(number_of_players),
        }
    }

    /*
        Execute turn of next player and return the Player option if Player reaches end position.
    */
    pub fn next_turn(&mut self, next_player: Player) -> Result<Option<Player>, String> {
        if let Some(player) = self.players.get_mut(&next_player.name()) {
            let steps = player.roll();
            let next_position = player.square().position() + steps;
            if next_position > 100 {
                println!("{} skipped", player.name());
                return Ok(None);
            }
            if let Some(square) = self.board.get_square(next_position) {
                player.move_to(square.next().clone());
                Self::print_progress(player, steps, square);
            }
            if player.position() == 100 {
                Ok(Some(player.clone()))
            } else {
                Ok(None)
            }
        } else {
            Err(format!("Could not find player {}", next_player))
        }
    }

    pub fn get_next_player(&mut self) -> Option<Player> {
        let mut next_index = 0;
        if self.current_player_index + 1 < self.players.len() {
            next_index = self.current_player_index + 1;
        }
        // println!("{} -> {}", self.current_player_index, next_index);
        self.current_player_index = next_index;
        let mut ps: Vec<Player> = self.players.values().cloned().collect();
        ps.sort_by_key(|p| p.name());
        ps.get(self.current_player_index).cloned()
    }

    fn print_progress(player: &Player, steps: u8, square: &Square) {
        match square {
            Square::Snake(head, _) => println!(
                "{} took {} steps and got eaten by Snake at {} and moved to {}",
                player.name(),
                steps,
                head,
                player.position()
            ),
            Square::Ladder(bottom, _) => println!(
                "{} took {} steps and found Ladder at {} and moved to {}",
                player.name(),
                steps,
                bottom,
                player.position()
            ),
            _ => println!(
                "{} took {} steps and moved to {}",
                player.name(),
                steps,
                player.position()
            ),
        }
    }
}
