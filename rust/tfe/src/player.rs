use stopwatch::{Stopwatch};

pub struct Player;

use super::game_data_model::GameData;
use super::game::Game;

impl Player {
    pub fn train(games_per_training: u32, threads: u32, _minmax_depth_percentage: u8) {


        let stopwatch = Stopwatch::start_new();
        let played_games: Vec<GameData> = Game::play_games(games_per_training, threads);
        println!("Total games: {:?}\nTime: {:?}", played_games.len(), stopwatch.elapsed());    
        println!("------------------------");
        // println!("Game_data: {:?}", played_games);
    }

}