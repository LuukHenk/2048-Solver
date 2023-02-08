use multimap::MultiMap;
use itertools::Itertools; 

use super::game::Game;
use super::random_weights_algorithm::RandomWeightsAlgorithm;


#[derive(Debug)]
pub struct Player {
    pub games: Vec<Game>,
    algorithm: RandomWeightsAlgorithm
}

impl Player {

    pub fn new() -> Player {
        Player{
            games: Vec::new(),
            algorithm: RandomWeightsAlgorithm::new()
        }
    }

    pub fn play_games(&mut self, amount: usize) {
        self.algorithm.display_weights();
        for _game_index in 0 .. amount {
            self.games.push(Game::new(self.algorithm.copy()));
        }
    }

    pub fn copy_games(&self) -> Vec<Game> {
        let mut games_copy: Vec<Game> = Vec::with_capacity(self.games.len());
        for game in self.games.iter() {
            games_copy.push(game.copy());
        }
        games_copy
    }

    pub fn resize_total_games(&mut self, maximum_games: usize) {
        let total_games: usize = self.games.len();
        if maximum_games >= total_games {
            return
        }
        self.games.drain(maximum_games..);
    }

    pub fn get_top_score(&mut self) -> u64 {
        self.sort_games_on_score();
        self.games[0].get_final_score()
    }

    pub fn get_average_score(&self) -> u64{
        let mut average_score: u64 = 0;
        if self.games.len() < 1 {
            return average_score;
        } 
        for game in self.games.iter(){
            average_score += game.get_final_score();
        }
        let total_games: u64 = self.games.len().try_into().unwrap();
        average_score / total_games
    }

    pub fn sort_games_on_score(&mut self) {
        let mut game_multimap: MultiMap<u64, Game> = self.__move_games_to_multimap(); 
        let sorted_scores: Vec<u64> = game_multimap.keys().sorted().cloned().collect();
        for score in sorted_scores {
            let games: &mut Vec<Game> = game_multimap.get_vec_mut(&score).unwrap();
            for _game_index in 0..games.len() {
                let game: Game = games.pop().unwrap();
                self.games.insert(0, game);
            }
        }
    }

    pub fn improve_games(&mut self, retries_per_game: usize) {
        let games_before_retrying: usize = self.games.len();
        for game_index in 0 .. self.games.len() -1 {
            self.__improve_game(game_index, retries_per_game);
        }
        self.sort_games_on_score();
        self.resize_total_games(games_before_retrying);
    }


    fn __improve_game(&mut self, index: usize, times: usize) {
        let mut game_to_retry = self.games[index].copy();
        game_to_retry.rewind(game_to_retry.get_index_of_highest_score_increasement());

        for _game_index in 0 .. times {
            let mut game = game_to_retry.copy();
            game.resume(self.algorithm.copy());
            self.games.push(game);
        }
    }

    fn __move_games_to_multimap(&mut self) -> MultiMap<u64, Game> {
        let mut game_multimap: MultiMap<u64, Game> = MultiMap::new();
        for _game_index in 0..self.games.len() {
            let game: Game = self.games.pop().unwrap();
            game_multimap.insert(game.get_final_score(), game);
        }
        game_multimap
    }

}




#[cfg(test)]
mod tests {
    use crate::game::board::Board;

    use super::*;
    const TOP_SCORE: u64 = 9999999999;

    #[test]
    fn test_defining_player() {
        let player: Player = Player::new();
        assert_eq!(player.games.len(), 0);
    }

    #[test]
    fn test_playing_games() {
        let games_to_play: usize = 100;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        assert_eq!(player.games.len(), games_to_play);
    }

    #[test]
    fn test_copy_games() {
        let mut player: Player = Player::new();
        let played_game: Game = Game::new(RandomWeightsAlgorithm::new());
        player.games.push(played_game.copy());

        assert_eq!(player.copy_games()[0], played_game);
    }

    #[test]
    fn test_get_average_score_with_no_games() {
        let player: Player = Player::new();
        let average_score: u64 = player.get_average_score();
        assert_eq!(average_score, 0);
    }
    #[test]
    fn test_get_average_score() {
        let mut games: Vec<Game> = Vec::new();
        games.push(__create_game(10));
        games.push(__create_game(20));
        games.push(__create_game(30));
        let mut player: Player = Player::new();
        player.games = games;
        let average_score: u64 = player.get_average_score();
        assert_eq!(average_score, 20);
    }
    
    #[test]
    fn test_resize_total_games_with_maximum_size_higher_than_games_played() {
        let games_to_play: usize = 50;
        let maximum_size: usize = 100;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        player.resize_total_games(maximum_size);

        assert_eq!(player.games.len(), games_to_play)
    }

    #[test]
    fn test_resize_total_games_with_maximum_size_equal_to_games_played() {
        let games_to_play: usize = 50;
        let maximum_size: usize = 50;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        player.resize_total_games(maximum_size);

        assert_eq!(player.games.len(), games_to_play)
    }

    #[test]
    fn test_resize_total_games_with_maximum_size_lower_than_games_played() {
        let games_to_play: usize = 100;
        let maximum_size: usize = 50;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        player.resize_total_games(maximum_size);

        assert_eq!(player.games.len(), maximum_size)
    }

    #[test]
    fn test_sort_scores() {
        let games: Vec<Game> = __create_games();

        let total_games = games.len();

        let mut player: Player = Player::new();
        player.games = games;
        player.sort_games_on_score();
        assert_eq!(player.games.len(), total_games);
        let mut latest_score = player.games[0].get_final_score();
        for game in player.games.iter() {
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
        let mut game: Game = Game::new(RandomWeightsAlgorithm::new());
        game.boards = vec![board];
        game
    }
}