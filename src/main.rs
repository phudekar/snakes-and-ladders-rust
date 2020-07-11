mod lib;
use lib::game::Game;
use lib::Square;
use std::{thread, time};

fn main() {
    let snakes = vec![Square::Snake(25, 9), Square::Snake(78, 36)];
    let ladders = vec![Square::Ladder(2, 29), Square::Ladder(18, 43)];
    let mut game = Game::new(2, snakes, ladders);
    loop {
        let next_player = game.get_next_player().unwrap();
        match game.next_turn(next_player.clone()) {
            Ok(Some(winner)) => {
                println!("{} is the Winner!!", winner.name());
                break;
            }
            Err(error) => println!("ERROR {}", error),
            _ => (),
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
