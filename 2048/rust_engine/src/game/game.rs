
use super::board::Board;
use super::direction::Direction;
use rand::Rng;

pub struct Game{pub boards: Vec<u64>}

impl Game {
    pub fn play() -> Game {
        let mut rng = rand::thread_rng();
        let mut game = Game{boards: Vec::new()};
        let mut board: Board = Board::new();
        let mut possible_movements: Vec<Direction> = board.get_possible_movements();
        let mut direction: Direction;
        game.boards.push(board.get_board());

        while possible_movements.len() > 0 {
            let selected_direction_index: usize = rng.gen_range(0..possible_movements.len());
            direction = possible_movements[selected_direction_index];
            board.perform_movement(&direction);
            game.boards.push(board.get_board());
            possible_movements = board.get_possible_movements();
        }
        
        game
    }
}