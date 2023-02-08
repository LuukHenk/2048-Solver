use super::single_corner_strategy::SingleCornerStrategy;
use super::board::Board;
use super::direction::Direction;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub boards: Vec<Board> // Fixme: Made the score public for testing purposes in Game handler
}

impl Game {
    pub fn new(algorithm: SingleCornerStrategy) -> Game {
        let mut game: Game = Game{boards: Vec::new()};
        game.boards.push(Board::new());
        game.resume(algorithm);
        game
    }

    pub fn copy_boards(&self) -> Vec<Board> {
        let mut boards_copy: Vec<Board> = Vec::with_capacity(self.boards.len());
        for board in self.boards.iter() {
            boards_copy.push(board.copy());
        }
        boards_copy
    }

    pub fn resume(&mut self, mut algorithm: SingleCornerStrategy) {
        let mut board: Board = self.boards[self.__latest_board_index()].copy();
        let mut possible_movements: Vec<Direction> = board.get_possible_movements();

        while possible_movements.len() > 0 {
            let direction: Direction = algorithm.determine_next_movement(board.copy());
            board.perform_movement(&direction);
            self.boards.push(board.copy());
            possible_movements = board.get_possible_movements();
        }
    }

    pub fn get_final_score(&self) -> u64 {
        self.boards[self.__latest_board_index()].get_score()
    }

    pub fn get_index_of_highest_score_increasement(&self) -> usize {
        let mut previous_board_score: u64 = 0;
        let mut highest_score_increasement: u64 = 0;
        let mut highest_score_index: usize = 0;

        for board_index in 0 .. self.__latest_board_index() {
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
        self.boards.drain(index+1..);
    }

    pub fn copy(&self) -> Game {
        let mut boards_copy: Vec<Board> = Vec::with_capacity(self.__latest_board_index()); 
        for board in self.boards.iter() {
            boards_copy.push(board.copy());
        }

        Game {boards:boards_copy}
    }

    fn __latest_board_index(&self) -> usize {
        self.boards.len()-1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_game_is_over_after_play() {
        let mut game: Game = Game::new(SingleCornerStrategy::new());
        let final_board: Option<Board> = game.boards.pop();
        assert_eq!(final_board.unwrap().get_possible_movements(), Vec::new());
    }
    #[test]
    fn test_get_boards(){
        let game: Game = Game::new(SingleCornerStrategy::new());
        assert_eq!(game.boards, game.copy_boards());
    }

    #[test]
    fn test_resume_game_when_there_are_no_possible_movements() {
        let mut game: Game = Game::new(SingleCornerStrategy::new());
        let latest_move = game.boards[game.__latest_board_index()].copy();
        game.resume(SingleCornerStrategy::new());
        assert_eq!(latest_move, game.boards[game.__latest_board_index()]);
    }

    #[test]
    #[should_panic(expected="attempt to subtract with overflow")]
    fn test_resume_game_when_there_are_no_boards() {
        let mut game: Game = Game::new(SingleCornerStrategy::new());
        game.boards = Vec::new();
        game.resume(SingleCornerStrategy::new());
    }

    #[test]
    fn test_resume_game_when_there_are_movements_possible() {
        let mut game: Game = Game::new(SingleCornerStrategy::new());
        let latest_move =Board::new(); 
        game.boards.push(latest_move.copy());
        game.resume(SingleCornerStrategy::new());
        
        assert_ne!(game.boards[game.__latest_board_index()], latest_move);
    }

    #[test]
    fn test_get_final_score() {
        let mut game: Game = Game::new(SingleCornerStrategy::new());
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
        
        let mut game: Game = Game::new(SingleCornerStrategy::new());
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
        let mut game: Game = Game::new(SingleCornerStrategy::new());
        let final_index: usize = 2;
        let expected_final_board: Board = game.boards[final_index].clone();
        game.rewind(final_index);
        let final_board: Option<Board> = game.boards.pop();
        assert_eq!(final_board.unwrap(), expected_final_board);
    }

    #[test]
    fn test_copy() {
        let game: Game = Game::new(SingleCornerStrategy::new());
        let game_copy: Game = game.copy();
        
        for index in 0 .. game.boards.len() -1 {
            assert_eq!(game.boards[index], game_copy.boards[index])
        }
    }
}