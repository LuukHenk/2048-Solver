

use rand::Rng;
use rand::rngs::ThreadRng;

use super::direction::Direction;


pub struct Algorithm {
    pub thread_rng: ThreadRng
}

impl Algorithm {
    pub fn new() -> Algorithm {
        let thread_rng: ThreadRng = rand::thread_rng();
        Algorithm{thread_rng}
    }
    pub fn determine_next_movement(&mut self, possible_movements: Vec<Direction>) -> Direction {
        let selected_direction_index: usize = self.thread_rng.gen_range(0..possible_movements.len());
        possible_movements[selected_direction_index]
    }
}