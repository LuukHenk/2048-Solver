use super::game::Game;

#[derive(Debug)]
pub struct GameHandler {
    games: Vec<Game>
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler{
            games: Vec::new()
        }
    }

    pub fn play_games(&mut self, amount: usize) {
        for game_index in 0 .. amount {
            __display_status(game_index + 1, amount);
            self.games.push(Game::play());
        }
        self.__sort_games_on_score()
    }

    pub fn drain_games(&mut self, amount: usize) {
        let total_games: usize = self.games.len();
        if amount > total_games {
            self.games.clear();
            return
        }
        self.games.drain(total_games - amount..);

    }

    pub fn get_top_scores(&self, mut amount: usize) -> Vec<u64> {
        if amount > self.games.len() {
            amount = self.games.len()
        }
        let mut game_scores: Vec<u64> = self.__sort_game_scores();
        game_scores.drain(amount..);
        game_scores
    }

    fn __sort_games_on_score(&mut self) {
        let sorted_game_scores: Vec<u64> = self.__sort_game_scores();
        
        for score_index in 0..sorted_game_scores.len(){
            __display_status(score_index + 1, sorted_game_scores.len());
            let score: u64 = sorted_game_scores[score_index];
            for game_index in 0..self.games.len() {
                if self.games[game_index].get_final_score() != score {continue}
                self.games.swap(game_index, score_index);
            }
        }
    }

    pub fn print_final_scores(&mut self) {
        self.__sort_games_on_score();
        for game in self.games.iter() {
            println!("{:#?}", game.get_final_score());
        }
    }

    fn __sort_game_scores(&self) -> Vec<u64> {
        let mut game_scores: Vec<u64> = self.games.iter().map(
            |game| game.get_final_score()
        ).collect();
        game_scores.sort_by(|a, b| b.cmp(a));
        game_scores
    }

}


fn __display_status(current_status: usize, goal: usize) {
    print!("                                        \r");
    print!("{:#?}/{:#?}", current_status, goal);
}

#[cfg(test)]
mod tests {
    use crate::game::{board::Board};

    use super::*;
    const TOP_SCORE: u64 = 9999999999;

    #[test]
    fn test_defining_game_handler() {
        let game_handler: GameHandler = GameHandler::new();
        assert_eq!(game_handler.games.len(), 0);
    }

    #[test]
    fn test_playing_games() {
        let games_to_play: usize = 100;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        assert_eq!(game_handler.games.len(), games_to_play);
    }

    #[test]
    fn test_drain_more_games_then_played() {
        let games_to_play: usize = 100;
        let games_to_drain: usize = 200;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        game_handler.drain_games(games_to_drain);
        assert_eq!(game_handler.games.len(), 0)
    }

    #[test]
    fn test_drain_games() {
        let games_to_play: usize = 100;
        let games_to_drain: usize = 50;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        game_handler.drain_games(games_to_drain);
        assert_eq!(game_handler.games.len(), 50)
    }

    #[test]
    fn test_get_more_top_scores_than_games_played() {
        let games: Vec<Game> = vec![__create_game(10)];       
        let game_handler: GameHandler = GameHandler {games};
        assert_eq!(game_handler.get_top_scores(100).len(), 1);
    }

    #[test]
    fn test_get_top_scores() {
        let games: Vec<Game> = __create_games();
        let game_handler: GameHandler = GameHandler {games};
        assert_eq!(game_handler.get_top_scores(1), vec![TOP_SCORE]);
    }

    #[test]
    fn test_sort_scores() {
        let games: Vec<Game> = __create_games();

        let total_games = games.len();

        let mut game_handler: GameHandler = GameHandler {games};
        game_handler.__sort_games_on_score();
        assert_eq!(game_handler.games.len(), total_games);
        let mut latest_score = game_handler.games[0].get_final_score();
        for game in game_handler.games.iter() {
            assert!(latest_score >= game.get_final_score());
            latest_score = game.get_final_score();
        }
    }

    fn __create_games() -> Vec<Game> {
        // Creates a simple game set
        let mut games: Vec<Game> = Vec::new();
        games.push(__create_game(2));
        games.push(__create_game(TOP_SCORE));
        games.push(__create_game(2));
        games.push(__create_game(1));
        games.push(__create_game(TOP_SCORE));
        games.push(__create_game(1));
        games
    }

    fn __create_game(final_score: u64) -> Game {
        let mut board: Board = Board::new();
        board.score = final_score; 
        let mut game: Game = Game::play();
        game.boards = vec![board];
        game
    }
}