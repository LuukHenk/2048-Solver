use std::collections::HashMap;
use rand::Rng;

use super::direction::Direction;

pub struct Algorithm;

impl Algorithm {

    pub fn get_direction(board: u64, possible_movements: HashMap<Direction, u64>, highest_tile: u64) -> Direction {
        best_movements: Vec<Direction> = Self::determine_best_movements(board, possible_movements, highest);
        let mut rng = rand::thread_rng();
        let selected_direction_index: usize = rng.gen_range(0..best_movements.len());
        directions[selected_direction_index]
    }   

    fn determine_best_movements(board: u64, possible_movements: HashMap<Direction, u64>, highest_tile: u64) -> Vec<Direction> {
        // Arrange  - Create movement weight hashmap
        let mut best_movements: Vec<Direction> = Vec::new();
        let mut highest_weight: u32 = 0;
        for direction in possible_movements.into_keys().collect().iter() {
            let mut weight: u32 = 0;
            // Act 1    - Determine if highest value is in corner

            // Act 2    - Determine the difference between sides of the baord
            // Act 3    - Determine score difference
            // Assert   - Determine highest weight determined by acts and choose best movement
            if weight > highest_weight {
                highest_weight = weight;
                best_movements = [direction]
            } else if weight == highest_weight {
                best_movements.append(direction)
            }
        }

        best_movements

    }
    // pub fn determine_best_movements(board: u64, mut possible_movements: Vec<Direction>) -> Vec<Direction> {
    //     possible_movements = Self::remove_up_from_movements(possible_movements);
    //     

    //     possible_movements
    // }

    // fn remove_up_from_movements(mut possible_movements: Vec<Direction>) -> Vec<Direction>{
    //     if possible_movements.len() > 1 {
    //         let index = Self::get_index_of_direction(&possible_movements, Direction::Up);
    //         if index != None {
    //             possible_movements.remove(index.unwrap());
    //         }
    //     }
    //     possible_movements
    // }

    // fn get_index_of_direction(direction_vec: &Vec<Direction>, direction_to_index: Direction) -> Option<usize> {
    //     for (i, direction) in direction_vec.iter().enumerate() {
    //         if direction == &direction_to_index {
    //             return Some(i);
    //         }
    //     }
    //     None
    // }

    // fn highest_in_corner(board: u64, movement_weights: HashMap<&Direction, u32>) {

    // }
}