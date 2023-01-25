
use super::player::Player;

pub struct Trainer{
    player: Player
}

impl Trainer{
    pub fn new(player: Player) -> Trainer {
        Trainer {player}
    }


    pub fn train(&mut self) {
        let initial_games: usize = 50000;
        let top_games: usize = 50;
        let total_trainings_rounds: usize = 10;

        println!("Playing initial games");
        self.__play_initial_games(initial_games, top_games);

        for trainings_round_index in 0.. total_trainings_rounds {
            println!("Trainings round {}", trainings_round_index);
            self.__trainings_round(initial_games, top_games)
        }
    }
    
    fn __play_initial_games(&mut self, initial_games: usize, top_games: usize) {
        self.player.play_games(initial_games);
        self.__cleanup(top_games);
    }

    fn __trainings_round(&mut self, initial_games: usize, top_games: usize) {
        let retries: usize = initial_games/top_games;
        for game_index in 0 .. top_games {
            self.player.retry_game(game_index, retries);
        }
        self.__cleanup(top_games);
    }

    fn __cleanup(&mut self, top_games: usize) {
        println!("");
        self.player.sort_games_on_score();
        println!("");
        self.player.resize_total_games(top_games);
        self.__print_top_scores(top_games);
    }

    fn __print_top_scores(&self, top_games:usize) {
        println!("Top scores:\n{:#?}", self.player.get_top_scores(top_games));
    }
}