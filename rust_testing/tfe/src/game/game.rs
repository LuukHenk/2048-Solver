use rand::Rng;
use rand::rngs::ThreadRng;

use super::board::Board;
use super::direction::Direction;

#[derive(Clone, Debug)]
pub struct Game {
    pub boards: Vec<Board> // Fixme: Made the score public for testing purposes in Game handler
}

impl Game {
    pub fn play() -> Game {
        let mut game: Game = Game{boards: Vec::new()};
        game.__game_loop(Board::new());
        game
    }

    pub fn get_final_score(&self) -> u64 {
        self.boards[self.boards.len()-1].get_score()
    }

    pub fn get_index_of_highest_score_increasement(&self) -> usize {
        // Get the latest highest increasement
        let mut previous_board_score: u64 = 0;
        let mut highest_score_increasement: u64 = 0;
        let mut highest_score_index: usize = 0;

        for board_index in 0 ..self.boards.len()-1 {
            let board_score: u64 = self.boards[board_index].get_score();
            let score_increasement: u64 = board_score - previous_board_score;
            if score_increasement >= highest_score_increasement {
                highest_score_increasement = score_increasement;
                highest_score_index = board_index;
            }
            previous_board_score = board_score;
        }

        highest_score_index
    }

    pub fn rewind(&mut self, index: usize) {
        // Rewind the game to a specific index
        self.boards.drain(index+1..);
    }

    pub fn copy(&self) -> Game {
        let mut boards_copy: Vec<Board> = Vec::with_capacity(self.boards.len()-1); 
        for board in self.boards.iter() {
            boards_copy.push(board.copy());
        }

        Game {boards:boards_copy}
    }

    fn __game_loop(&mut self, mut board: Board) {
        let mut rng:ThreadRng = rand::thread_rng();
        let mut possible_movements: Vec<Direction> = board.get_possible_movements();
        let mut direction: Direction;

        self.boards.push(board.copy());
        while possible_movements.len() > 0 {
            let selected_direction_index: usize = rng.gen_range(0..possible_movements.len());
            direction = possible_movements[selected_direction_index];
            board.perform_movement(&direction);
            self.boards.push(board.copy());
            possible_movements = board.get_possible_movements();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_game_is_over_after_play() {
        let mut game: Game = Game::play();
        let final_board: Option<Board> = game.boards.pop();
        assert_eq!(final_board.unwrap().get_possible_movements(), Vec::new());
    }

    #[test]
    fn test_get_final_score() {
        let mut game: Game = Game::play();
        let final_score: u64 = game.get_final_score();
        let final_board: Option<Board> =game.boards.pop(); 
        assert_eq!(
            final_score,
            final_board.unwrap().get_score(),
        );
    }

    #[test]
    fn test_get_index_of_highest_score_increasement() {
        let high_score_increasement: u64 = 1000;
        let low_score_increasement: u64 = 1;

        let start_board: Board = Board::new();
        let mut board_1: Board = Board::new();
        let mut board_2: Board = Board::new();
        let mut board_3: Board = Board::new();
        let mut board_4: Board = Board::new();
        let mut board_5: Board = Board::new();

        board_1.score = start_board.score + low_score_increasement;
        board_2.score = board_1.score + high_score_increasement;
        board_3.score = board_2.score + low_score_increasement;
        board_4.score = board_3.score + high_score_increasement;
        board_5.score = board_4.score + low_score_increasement;
        
        let mut game: Game = Game::play();
        game.boards = Vec::with_capacity(5);
        game.boards.push(start_board);
        game.boards.push(board_1);
        game.boards.push(board_2);
        game.boards.push(board_3);
        game.boards.push(board_4);
        game.boards.push(board_5);

        assert_eq!(game.get_index_of_highest_score_increasement(), 4)


        
    }

    #[test]
    fn test_rewind() {
        let mut game: Game = Game::play();
        let final_index: usize = 2;
        let expected_final_board: Board = game.boards[final_index].clone();
        game.rewind(final_index);
        let final_board: Option<Board> = game.boards.pop();
        assert_eq!(final_board.unwrap(), expected_final_board);
    }

    #[test]
    fn test_copy() {
        let game: Game = Game::play();
        let game_copy: Game = game.copy();
        
        for index in 0 .. game.boards.len() -1 {
            assert_eq!(game.boards[index], game_copy.boards[index])
        }
    }
}