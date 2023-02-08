

use std::collections::HashMap;
use rand::Rng;
use rand::rngs::ThreadRng;

use super::Board;
use super::Direction;

#[derive(Debug)]
pub struct RandomWeightsAlgorithm {
    thread_rng: ThreadRng,
    weights: HashMap<Direction, u8>
}

static EXPECTED_TOTAL_DIRECTIONS: &'static usize = &4;

impl RandomWeightsAlgorithm {
    pub fn new() -> RandomWeightsAlgorithm {
        let mut thread_rng: ThreadRng = rand::thread_rng();
        let mut weights: HashMap<Direction, u8> = HashMap::with_capacity(*EXPECTED_TOTAL_DIRECTIONS);
        weights.insert(Direction::Left, thread_rng.gen());
        weights.insert(Direction::Right, thread_rng.gen());
        weights.insert(Direction::Down, thread_rng.gen());
        weights.insert(Direction::Up, thread_rng.gen());
        RandomWeightsAlgorithm{thread_rng, weights}
    }

    pub fn determine_next_movement(&mut self, mut board: Board) -> Direction {
        let possible_directions: Vec<Direction> = board.get_possible_movements();
        let mut best_score: u32 = 0;
        let mut best_movement: Direction = Direction::None;
        for direction in possible_directions {
            let random_input_u8: u8 = self.thread_rng.gen();
            let random_input_u32: u32 = random_input_u8.into();
            let weight_u8: u8 = *self.weights.get(&direction).unwrap();
            let weight_u32: u32 = weight_u8.into();
            let score: u32 = random_input_u32 * weight_u32;
            if score >= best_score {
                best_movement = direction;
                best_score = score
            }
        }
        best_movement
    }

    pub fn copy(&self) -> RandomWeightsAlgorithm {
        let mut weights_copy: HashMap<Direction, u8> = HashMap::with_capacity(*EXPECTED_TOTAL_DIRECTIONS);
        for (key, value) in self.weights.iter() {
            weights_copy.insert(*key, *value);
        }
        RandomWeightsAlgorithm {thread_rng: rand::thread_rng(), weights: weights_copy}
    }

    pub fn display_weights(&self) {
        for (direction, weight) in self.weights.iter() {
            println!("Weight {:#?}:\t{:#?}/255", direction, weight);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_a_new_algorithm() {
        let algo: RandomWeightsAlgorithm = RandomWeightsAlgorithm::new();
        assert_ne!(algo.weights.get(&Direction::Left), None);
        assert_ne!(algo.weights.get(&Direction::Right), None);
        assert_ne!(algo.weights.get(&Direction::Down), None);
        assert_ne!(algo.weights.get(&Direction::Up), None);
        assert_eq!(algo.weights.get(&Direction::None), None);
    }

    #[test]
    fn test_determine_best_score_without_weight() {
        let mut algo: RandomWeightsAlgorithm = RandomWeightsAlgorithm::new();
        let mut weights: HashMap<Direction, u8> = HashMap::with_capacity(*EXPECTED_TOTAL_DIRECTIONS);
        weights.insert(Direction::Left, 0);
        weights.insert(Direction::Right, 0);
        weights.insert(Direction::Down, 0);
        weights.insert(Direction::Up, 0);
        algo.weights = weights;

        let next_movement = algo.determine_next_movement(Board::new());

        let mut direction_found: bool = false; 
        for direction in vec![Direction::Left, Direction::Right, Direction::Down, Direction::Up].iter() {
            if next_movement == *direction {
                direction_found = true;
            }
        }
        assert!(direction_found);
    }

    #[test]
    fn test_determine_best_score() {
        let mut algo: RandomWeightsAlgorithm = RandomWeightsAlgorithm::new();
        let mut weights: HashMap<Direction, u8> = HashMap::with_capacity(4);
        weights.insert(Direction::Left, 10);
        weights.insert(Direction::Right, 0);
        weights.insert(Direction::Down, 0);
        weights.insert(Direction::Up, 0);
        algo.weights = weights;

        let next_movement = algo.determine_next_movement(Board::new());

        assert_eq!(next_movement, Direction::Left);
    }

    #[test]
    fn test_copy() {
        let algo: RandomWeightsAlgorithm = RandomWeightsAlgorithm::new();
        let algo_copy: RandomWeightsAlgorithm = algo.copy();
        assert_eq!(algo_copy.weights, algo.weights)
    }
}