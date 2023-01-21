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
        for _ in 0 .. amount {
            self.games.push(Game::play());
        }
    }

    pub fn drain_games(&mut self, amount: usize) {
        let total_games: usize = self.games.len();
        if amount > total_games {
            self.games.clear();
            return
        }
        self.games.drain(total_games - amount..);

    }

    pub fn sort_games_on_score(&mut self) {
        let sorted_game_scores: Vec<u64> = self.__sort_game_scores();
        
        for score_index in 0..sorted_game_scores.len(){
            let score: u64 = sorted_game_scores[score_index];
            for game_index in 0..self.games.len() {
                if self.games[game_index].get_final_score() != score {continue}
                let game: Game = self.games.remove(game_index);
                self.games.insert(score_index, game);
            }
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



#[cfg(test)]
mod tests {
    use crate::game::{board::Board};

    use super::*;

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
    fn test_sort_scores() {
        let mut games: Vec<Game> = Vec::new();
        
        
        games.push(__create_game(2));
        games.push(__create_game(3));
        games.push(__create_game(2));
        games.push(__create_game(1));
        games.push(__create_game(3));
        games.push(__create_game(1));
        let total_games = games.len();

        let mut game_handler: GameHandler = GameHandler {games};
        game_handler.sort_games_on_score();
        assert_eq!(game_handler.games.len(), total_games);
        let mut latest_score = game_handler.games[0].get_final_score();
        for game in game_handler.games.iter() {
            assert!(latest_score >= game.get_final_score());
            latest_score = game.get_final_score();
        }
    }

    fn __create_game(final_score: u64) -> Game {
        let mut board: Board = Board::new();
        board.score = final_score; 
        let mut game: Game = Game::play();
        game.boards = vec![board];
        game
    }
}