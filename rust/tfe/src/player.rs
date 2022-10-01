pub struct Player;

use super::game_data_model::GameData;
use super::game::Game;

impl Player {
    pub fn train(games_per_training: u32, threads: u32, minmax_depth_percentage: u8) {


        let played_games: Vec<GameData> = Game::play_games(games_per_training, threads);
    }

}