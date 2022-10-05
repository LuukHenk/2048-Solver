use rand::Rng;

use super::direction::Direction;

pub struct Algorithm{}

impl Algorithm {

    pub fn determine_best_movement(mut possible_movements: Vec<Direction>) -> Direction {
        possible_movements = Self::remove_up_from_movements(possible_movements);
        let mut rng = rand::thread_rng();
        let selected_direction_index: usize = rng.gen_range(0..possible_movements.len());
        possible_movements[selected_direction_index]
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
}


