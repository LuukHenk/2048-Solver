use std::collections::HashMap;

use super::direction::Direction;

pub struct Algorithm;

impl Algorithm {

    pub fn determine_best_movements(board: u64, mut possible_movements: Vec<Direction>) -> Vec<Direction> {
        possible_movements = Self::remove_up_from_movements(possible_movements);
        let mut movement_weights: HashMap<&Direction, u32> = HashMap::new();
        for movement in possible_movements.iter() {
            movement_weights.insert(movement, 0);
        } 
        possible_movements
    }

    fn remove_up_from_movements(mut possible_movements: Vec<Direction>) -> Vec<Direction>{
        if possible_movements.len() > 1 {
            let index = Self::get_index_of_direction(&possible_movements, Direction::Up);
            if index != None {
                possible_movements.remove(index.unwrap());
            }
        }
        possible_movements
    }

    fn get_index_of_direction(direction_vec: &Vec<Direction>, direction_to_index: Direction) -> Option<usize> {
        for (i, direction) in direction_vec.iter().enumerate() {
            if direction == &direction_to_index {
                return Some(i);
            }
        }
        None
    }

    fn highest_in_corner(board: u64, movement_weights: HashMap<&Direction, u32>) {

    }
}