use std::collections::HashMap;
use rand::Rng;

use super::direction::Direction;

pub struct Algorithm;

impl Algorithm {

    pub fn determine_best_movements(_board: u64, possible_movements: HashMap<Direction, u64>) -> Direction {
        // Arrange  - Create movement weight hashmap
        let directions: Vec<Direction> = possible_movements.into_keys().collect();
        let mut movement_weights: HashMap<&Direction, u32> = HashMap::new();
        for movement in directions.iter() {movement_weights.insert(movement, 0);}

        // Act 1    - Determine if highest value is in corner
        // Act 2    - Determine the difference between sides of the baord
        // Act 3    - Determine score difference
        // Assert   - Determine highest weight determined by acts and choose best movement

        // TMP output
        let mut rng = rand::thread_rng();
        
        let selected_direction_index: usize = rng.gen_range(0..directions.len());
        directions[selected_direction_index]
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