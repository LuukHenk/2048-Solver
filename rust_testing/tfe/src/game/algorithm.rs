

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::Board;

use super::direction::Direction;


pub struct Algorithm {
    pub thread_rng: ThreadRng
}

impl Algorithm {
    pub fn new() -> Algorithm {
        let thread_rng: ThreadRng = rand::thread_rng();
        Algorithm{thread_rng}
    }
    pub fn determine_next_movement(&mut self, mut board: Board) -> Direction {
        //FIXME write tests
        let mut possible_movements: Vec<Direction> = board.get_possible_movements();

        if possible_movements.contains(&Direction::Up) && possible_movements.len() > 1 {
            if possible_movements.contains(&Direction::Right) && possible_movements.len() > 2 {
                let position_of_right: usize = possible_movements.iter().position(|&r| r == Direction::Right).unwrap();
                possible_movements.remove(position_of_right);
            }
            let position_of_up: usize = possible_movements.iter().position(|&r| r == Direction::Up).unwrap();
            possible_movements.remove(position_of_up);
        }

        let selected_direction_index: usize = self.thread_rng.gen_range(0..possible_movements.len());
        possible_movements[selected_direction_index]
    }
}