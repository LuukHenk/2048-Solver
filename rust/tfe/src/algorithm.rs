use std::collections::HashMap;
use rand::Rng;

use super::direction::Direction;
use super::board::Board;

pub struct Algorithm;
pub static CORNER_MASK: u64 = 0xF00F_0000_0000_F00F;
pub static TOP_MASK: u64 = 0xFFFF_0000_0000_0000;
pub static BOTTOM_MASK: u64 = 0x0000_0000_0000_FFFF;
pub static LEFT_MASK: u64 = 0xF000_F000_F000_F000;
pub static RIGHT_MASK: u64 = 0x000F_000F_000F_000F;

impl Algorithm {

    pub fn get_direction(board: u64, possible_movements: HashMap<Direction, u64>, highest_tile: u64) -> Direction {
        let best_movements: Vec<Direction> = Self::determine_best_movements(board, possible_movements, highest_tile);
        let mut rng = rand::thread_rng();
        let selected_direction_index: usize = rng.gen_range(0..best_movements.len());
        best_movements[selected_direction_index]
    }   

    fn determine_best_movements(current_board: u64, possible_movements: HashMap<Direction, u64>, highest_tile: u64) -> Vec<Direction> {
        // Arrange  - Create movement weight hashmap
        let mut best_movements: Vec<Direction> = Vec::new();
        let mut highest_weight: i128 = i128::MIN;

        
        for (direction, board_after_move) in possible_movements.iter() {
            let mut weight: i128 = 0;
            // Act 1    - Determine if highest tike is in corner
            weight += Self::set_weight_for_highest_tile_in_corner(highest_tile, current_board, *board_after_move);
            // Act 2    - Determine the difference between sides of the baord
            weight += Self::set_weight_for_rows_on_the_side_of_the_board(current_board, *board_after_move, *direction);
            // Act 3    - Determine score difference
            weight += Self::set_weight_for_get_score_differences(current_board, *board_after_move);
            // Assert   - Determine highest weight determined by acts and choose best movement
            if weight > highest_weight {
                highest_weight = weight;
                best_movements = vec![*direction]
            } else if weight == highest_weight {
                best_movements.push(*direction)
            }
        }

        // let directions: Vec<Direction> = possible_movements.into_keys().collect();
        // println!("Board: {:#02x}\nBest movements: {:?}\nPossible directions: {:?}\n", current_board, best_movements, directions);
        // 
        best_movements

    }

    fn set_weight_for_highest_tile_in_corner(highest_tile: u64, current_board: u64, board_after_move: u64) -> i128 {
        let found_in_current_board: bool = Self::find_value_in_board(highest_tile, current_board & CORNER_MASK);
        let found_in_board_after_move: bool = Self::find_value_in_board(highest_tile, board_after_move & CORNER_MASK);
        if found_in_current_board == found_in_board_after_move {
            return 0_i128
        } else {
            let weight: i128 = highest_tile.into();
            if found_in_current_board && !found_in_board_after_move {
                return -weight
            } else {
                return weight
            }
        }
    }

    fn find_value_in_board(value: u64, board: u64) -> bool {
        for i in 0..16 { if Board::get_tile(board, i) ==  value { return true } }
        return false
    }

    fn set_weight_for_rows_on_the_side_of_the_board(
        current_board: u64,
        board_after_move: u64,
        movement: Direction
    ) -> i128 {
        if movement == Direction::Up || movement == Direction::Down {
            return Self::calculate_score_differences(
                Board::get_score(current_board & TOP_MASK).into(),
                Board::get_score(current_board & BOTTOM_MASK).into(),
                Board::get_score(board_after_move & TOP_MASK).into(),
                Board::get_score(board_after_move & BOTTOM_MASK).into()
            )
        } else {
            return Self::calculate_score_differences(
                Board::get_score(current_board & LEFT_MASK).into(),
                Board::get_score(current_board & RIGHT_MASK).into(),
                Board::get_score(board_after_move & LEFT_MASK).into(),
                Board::get_score(board_after_move & RIGHT_MASK).into()
            )
        }
    }

    fn calculate_score_differences(
        board_side_1: i128,
        board_side_2: i128,
        board_after_move_side_1: i128, 
        board_after_move_side_2: i128
    ) -> i128 {
        (board_after_move_side_1 - board_after_move_side_2).abs() - (board_side_1 - board_side_2).abs()
    }

    fn set_weight_for_get_score_differences(board: u64, board_after_move: u64) -> i128 {
        let score_after_move: i128 = Board::get_score(board_after_move).into();
        let score_current_board: i128 = Board::get_score(board).into();
        score_after_move - score_current_board
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

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_find_value_in_board() {

        let value: u64 = 1;
        let board: u64 = 0x2222_0000_6666_0000; 
        assert_eq!(Algorithm::find_value_in_board(value, board), false);
    
        let value: u64 = 2;
        let board: u64 = 0x2222_0000_6666_0000; 
        assert_eq!(Algorithm::find_value_in_board(value, board), true);
    }

    #[test]
    fn test_set_weight_for_highest_tile_in_corner() {
        let highest_tile: u64 = 9;
        let board_with_highest_tile_in_corner: u64 = 0x9000_3333_2222_1111;
        let board_without_highest_tile_in_corner: u64 = 0x0900_3333_2222_1111;

        // Not at start, not at end
        let result: i128 = Algorithm::set_weight_for_highest_tile_in_corner(
            highest_tile,
            board_without_highest_tile_in_corner,
            board_without_highest_tile_in_corner
        );
        assert_eq!(result, 0_i128);

        // Not at start, but at end
        let result: i128 = Algorithm::set_weight_for_highest_tile_in_corner(
            highest_tile,
            board_without_highest_tile_in_corner,
            board_with_highest_tile_in_corner
        );
        assert_eq!(result, 9_i128);

        // At start, but not at end
        let result: i128 = Algorithm::set_weight_for_highest_tile_in_corner(
            highest_tile,
            board_with_highest_tile_in_corner,
            board_without_highest_tile_in_corner
        );
        assert_eq!(result, -9_i128);

        // At start, and at end
        let result: i128 = Algorithm::set_weight_for_highest_tile_in_corner(
            highest_tile,
            board_with_highest_tile_in_corner,
            board_with_highest_tile_in_corner
        );
        assert_eq!(result, 0_i128);

    }

    #[test]
    fn test_set_weight_for_rows_on_the_side_of_the_board() {
        let current_board: u64 = 0x0802_1801_4015_0010;
        // Note that these movements are not perse legit

        // move up
        let board_after_move: u64 = 0x1922_4001_0005_0000;
        let expected_weight: i128 = 19; 
        let movement: Direction = Direction::Up;
        let weight: i128 = Algorithm::set_weight_for_rows_on_the_side_of_the_board(
            current_board, board_after_move, movement
        );
        assert_eq!(weight, expected_weight);

        // move down
        let board_after_move: u64 = 0x0333_0000_0000_3333;
        let expected_weight: i128 = -6;
        let movement: Direction = Direction::Down;
        let weight: i128 = Algorithm::set_weight_for_rows_on_the_side_of_the_board(
            current_board, board_after_move, movement
        );
        assert_eq!(weight, expected_weight);

        // move left
        let board_after_move: u64 = 0x8200_1810_4150_1000;
        let expected_weight: i128 = 19;
        let movement: Direction = Direction::Left;
        let weight: i128 = Algorithm::set_weight_for_rows_on_the_side_of_the_board(
            current_board, board_after_move, movement
        );
        assert_eq!(weight, expected_weight);

        // move right
        let board_after_move: u64 = 0x3003_3003_3003_3000;
        let expected_weight: i128 = -6;
        let movement: Direction = Direction::Right;
        let weight: i128 = Algorithm::set_weight_for_rows_on_the_side_of_the_board(
            current_board, board_after_move, movement
        );
        assert_eq!(weight, expected_weight);
    }
}