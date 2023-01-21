
use super::game_handler::GameHandler;

pub struct Player{
    game_handler: GameHandler
}

impl Player{
    pub fn new(game_handler: GameHandler) -> Player {
        Player {game_handler}
    }

    pub fn train(&mut self, trainings_rounds: usize, trainings_round_size: usize) {
        println!("Playing initial games");
        self.game_handler.play_games(trainings_round_size);
        for trainings_round in 0..trainings_rounds {
            println!("\nTrainings round: {:#?}/{:#?}", trainings_round + 1, trainings_rounds);
            self.game_handler.play_games(trainings_round_size);
            self.game_handler.drain_games(trainings_round_size);
        }
    }
}