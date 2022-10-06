use stopwatch::{Stopwatch};
use multimap::MultiMap;

use super::game::game_handler;
use super::game::game::Game;
use super::json_conversion;

pub struct Player{
    games_per_training: usize,
}

impl Player {
    pub fn new(games_per_training: usize) -> Player {
        let player: Player = Player{
            games_per_training: games_per_training
        };
        player
    }


    pub fn train(&mut self, threads: usize, selection_percentage: usize) {
        let stopwatch = Stopwatch::start_new();
        let mut top_selection_size: usize = self.games_per_training * selection_percentage / 100;
        if top_selection_size < 1 {top_selection_size = 1;} 
        let current_trainings_set = game_handler::play_games(
            self.games_per_training, threads
        );
        
        // WOrk in prgress
        let top_games = Player::select_top_games(current_trainings_set, top_selection_size);
        json_conversion::convert_games_data_to_json(top_games);



        println!("{:?}", top_games);
        println!("Total games: {:?}\nTime: {:?}", self.games_per_training, stopwatch.elapsed());    
        println!("------------------------");
    }

    fn select_top_games(mut trainings_set: MultiMap<u64, Game>, top_selection_size: usize) -> Vec<Game> {
        let scores = Player::order_map_keys(&trainings_set, true);
        let mut top_games: Vec<Game> = Vec::with_capacity(top_selection_size);

        for i in 0 .. scores.len() {
            let mut games: Vec<Game> = trainings_set.remove(&scores[i]).unwrap();
            for j in 0 .. games.len() {
                top_games.push(games.swap_remove(j));
                if top_games.len() == top_selection_size {
                    break
                }
            }
            if top_games.len() == top_selection_size {
                break
            }
        }
        top_games
    }

    fn order_map_keys(map: &MultiMap<u64, Game>, descending: bool) -> Vec<u64> {
        let mut map_keys: Vec<u64> = map.keys().cloned().collect();
        if descending {
            map_keys.sort_by(|a, b| b.cmp(a))
        } else {
            map_keys.sort()
        }
        map_keys  
    }
}