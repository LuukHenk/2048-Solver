use rand::Rng;
use rand::rngs::ThreadRng;

use super::board::Board;
use super::direction::Direction;

pub struct Game {
    boards: Vec<Board>
}

impl Game {
    pub fn play() -> Game {
        let mut game: Game = Game{boards: Vec::new()};

        let mut rng:ThreadRng = rand::thread_rng();
        let mut board: Board = Board::new();
        let mut possible_movements: Vec<Direction> = board.get_possible_movements();
        let mut direction: Direction;

        game.boards.push(board.copy());
        while possible_movements.len() > 0 {
            let selected_direction_index: usize = rng.gen_range(0..possible_movements.len());
            direction = possible_movements[selected_direction_index];
            board.perform_movement(&direction);
            game.boards.push(board.copy());
            possible_movements = board.get_possible_movements();
        }

        game
    }

    fn get_final_score(&self) -> u64 {
        self.boards[self.boards.len()-1].get_score()
    }

    fn get_index_of_highest_score_increasement(&self) -> usize {
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

}