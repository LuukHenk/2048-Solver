
use crate::Game;

use super::player::Player;

pub struct Trainer{
    player: Player,
    top_games: usize
}

impl Trainer{
    pub fn new(player: Player, top_games: usize) -> Trainer {
        Trainer {player, top_games}
    }


    pub fn train(&mut self) {
        let initial_games: usize = 100;
        let total_trainings_rounds: usize = 5;

        println!("Playing initial games");
        self.__play_initial_games(initial_games);

        for trainings_round_index in 0.. total_trainings_rounds {
            println!("Trainings round {}", trainings_round_index+1);
            self.__trainings_round(initial_games)
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

    fn __trainings_round(&mut self, initial_games: usize) {
        let retries: usize = initial_games/self.top_games;
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