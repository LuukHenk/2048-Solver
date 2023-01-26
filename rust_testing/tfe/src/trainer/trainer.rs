
use crate::Game;

use super::player::Player;

pub struct Trainer{
    player: Player,
    top_games: usize
}

impl Trainer{
    pub fn new(top_games: usize) -> Trainer {
        let player: Player = Player::new();
        Trainer {player, top_games}
    }


    pub fn train(&mut self, games_per_trainings_round: usize, total_trainings_rounds: usize) {

        println!("Playing initial games");
        self.__play_initial_games(games_per_trainings_round);

        for trainings_round_index in 0.. total_trainings_rounds {
            println!("Trainings round {}", trainings_round_index+1);
            self.__trainings_round(games_per_trainings_round)
        }
    }
    
    pub fn copy_top_games(&mut self) -> Vec<Game> {
        self.__cleanup();
        self.player.copy_games()
    }

    fn __play_initial_games(&mut self, initial_games: usize) {
        self.player.play_games(initial_games);
        self.__cleanup();
    }

    fn __trainings_round(&mut self, games_per_trainings_round: usize) {
        let retries: usize = games_per_trainings_round/self.top_games;
        for game_index in 0 .. self.top_games {
            self.player.retry_game(game_index, retries);
        }
        self.__cleanup();
    }

    fn __cleanup(&mut self) {
        println!("");
        self.player.sort_games_on_score();
        println!("");
        self.player.resize_total_games(self.top_games);
        self.player.print_final_scores();
    }


}