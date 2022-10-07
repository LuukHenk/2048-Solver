use multimap::MultiMap;

use super::export_to_file;
use super::game::game::Game;
use super::game::game_handler;
use super::json_conversion;

pub struct Player {
    top_games: MultiMap<u64, Game>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            top_games: MultiMap::new(),
        }
    }

    pub fn train(
        &mut self,
        trainings_set_size: usize,
        percentage_top_games: f32,
        thread_capacity: usize,
        trainings_rounds: usize,
    ) {
        if percentage_top_games > 1_f32 {
            panic!("percentage_top_games should be defined between 0 and 1")
        };
        let mut max_top_games: usize = (trainings_set_size as f32 * percentage_top_games) as usize;
        if max_top_games < 1 {
            max_top_games = 1;
        }
        self.redetermine_top_games(trainings_set_size, max_top_games, 0x0_u64, thread_capacity);
        for _ in 0..trainings_rounds {
            self.redetermine_top_games(trainings_set_size, max_top_games, 0x0_u64, thread_capacity);
        }
    }

    pub fn export_games(&self, number_of_games_to_export: usize, saving_path: &String) {
        let game_data_json_format: String = json_conversion::convert_games_data_to_json(
            self.select_games_for_export(number_of_games_to_export),
        );
        export_to_file::write(game_data_json_format, saving_path);
    }

    fn select_games_for_export(&self, number_of_games_to_export: usize) -> Vec<&Game> {
        let mut games_to_export: Vec<&Game> = Vec::with_capacity(number_of_games_to_export);
        let top_scores = self.order_score();
        for score in top_scores.iter() {
            for game in self.top_games.get_vec(score).unwrap().iter() {
                games_to_export.push(game);
                if games_to_export.len() == number_of_games_to_export {
                    return games_to_export;
                }
            }
        }

        panic!(
            "Expected games to export '{:?}' is higher than the amount of saved games (top_games) '{:?}'",
            number_of_games_to_export,
            games_to_export.len()
        );
    }

    fn redetermine_top_games(
        &mut self,
        trainings_set_size: usize,
        max_top_games: usize,
        _starting_board: u64,
        thread_capacity: usize,
    ) {
        for game_set in game_handler::play_games(trainings_set_size, thread_capacity).iter_all() {
            self.top_games
                .insert_many_from_slice(*game_set.0, game_set.1);
        }

        let mut top_scores = self.order_score();
        let other_scores: Vec<u64> = top_scores.drain(max_top_games..).collect();
        for score in other_scores.iter() {
            self.top_games.remove(score);
        }
        let mut number_of_games: usize = 0;
        for score in top_scores.iter() {
            if max_top_games >= number_of_games {
                let games_with_score = self.top_games.get_vec(score).unwrap();
                number_of_games += games_with_score.len();
            } else {
                self.top_games.remove(score);
            }
        }
    }

    fn order_score(&self) -> Vec<u64> {
        let mut map_keys: Vec<u64> = self.top_games.keys().cloned().collect();
        map_keys.sort_by(|a, b| b.cmp(a));
        map_keys
    }

    fn _get_games(&mut self) -> Vec<Game> {
        let mut games_out: Vec<Game> = Vec::new();
        for (_, top_games_values) in self.top_games.iter_all_mut() {
            for game in top_games_values.iter() {
                games_out.push(game.clone());
            }
        }
        games_out
    }
}
