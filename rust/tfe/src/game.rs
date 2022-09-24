use rand::seq::SliceRandom; // 0.7.2

use super::direction::Direction;
use super::board::Board;

pub struct Game{}

impl Game {
    pub fn play_games(total_games: u32) -> Vec<u64>{
        let mut games_played: u32 = 0;
        let mut highest_tiles: Vec<u64> = Vec::new();
        while total_games >= games_played {
            highest_tiles.push(Self::play());
            games_played += 1;
            print!("Playing games... ({:.2}%)\r", games_played as f32 /total_games as f32 *100.0)  
        }
        highest_tiles
    }

    fn play() -> u64 {
        let mut board = Board::new();
        while Board::board_full(board) == false {
            board = Self::perform_movement(board);
            
        }
        Board::get_highest_tile(board)
    }

    fn perform_movement(mut board: u64) -> u64{
        let mut possible_movements: Vec<Direction> = Board::get_possible_movements(board);
        possible_movements = Self::determine_best_movements(possible_movements);
        let mut rng = rand::thread_rng();
        let direction: &Direction = possible_movements.choose(&mut rng).unwrap();
        board = Board::perform_movement(board, direction);
        board
           
    }


    fn determine_best_movements(mut possible_movements: Vec<Direction>) -> Vec<Direction> {
        possible_movements = Self::remove_up_from_movements(possible_movements);
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
}