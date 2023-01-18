use rand::Rng;

use super::board::Board;
use super::direction::Direction;

#[derive(Debug, Clone)]
pub struct Game {
    pub boards: Vec<u64>,
    pub scores: Vec<u64>,
    pub movements: Vec<Direction>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            boards: Vec::new(),
            scores: Vec::new(),
            movements: Vec::new(),
        }
    }

    pub fn play(&mut self) -> (u64, Game) {
        let mut rng = rand::thread_rng();
        let mut board: Board = Board::new();
        let mut possible_movements: Vec<Direction> = board.get_possible_movements();
        let mut direction: Direction;
        self.update_game_data(&board, Direction::None);

        while possible_movements.len() > 0 {
            let selected_direction_index: usize = rng.gen_range(0..possible_movements.len());
            direction = possible_movements[selected_direction_index];
            board.perform_movement(&direction);
            self.update_game_data(&board, direction);
            possible_movements = board.get_possible_movements()
        }

        (self.extract_final_score(), self.clone())
    }

    fn update_game_data(&mut self, board: &Board, direciton: Direction) {
        self.boards.push(board.get_board());
        self.scores.push(board.get_score());
        self.movements.push(direciton);
    }

    fn extract_final_score(&self) -> u64 {
        self.scores[self.scores.len() - 1]
    }
}
