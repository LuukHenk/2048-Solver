use rand::seq::SliceRandom; // 0.7.2
use super::direction::Direction;
use super::board::Board;

pub struct Game{}

impl Game {
    pub fn play_games(total_games: u32) {
        let mut games_played: u32 = 0;
        let mut highest_tiles: Vec<u64> = Vec::new();
        while total_games >= games_played {
            highest_tiles.push(Self::play());
            games_played += 1;
            println!("{:?}",games_played)     
        }
        // 100_000 games in 2 min and 30 sec
        // println!("{:?}",highest_tiles) 
    }

    fn play() -> u64 {
        let mut board = Board::new();
        while Board::board_full(board) == false {
            board = Self::perform_movement(board);
            
        }
        Board::get_highest_tile(board)
    }

    fn perform_movement(mut board: u64) -> u64{
        let possible_movements: Vec<Direction> = Board::get_possible_movements(board);
        let mut rng = rand::thread_rng();
        let direction: &Direction = possible_movements.choose(&mut rng).unwrap();
        board = Board::perform_movement(board, direction);
        // println!("{:#02X} - {:?}", board, direction);
        board
           
    }
}