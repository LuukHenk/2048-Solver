
use super::game_handler::GameHandler;

pub struct Trainer{
    game_handler: GameHandler
}

impl Trainer{
    pub fn new(game_handler: GameHandler) -> Trainer {
        Trainer {game_handler}
    }

    pub fn train(&mut self, trainings_rounds: usize, trainings_round_size: usize) {
        println!("Playing initial games");
        self.game_handler.play_games(trainings_round_size);
        for trainings_round in 0..trainings_rounds {
            println!("\nTrainings round: {:#?}/{:#?}", trainings_round + 1, trainings_rounds);
            self.game_handler.play_games(trainings_round_size);
            self.game_handler.drain_games(trainings_round_size);
        }
        println!("{:#?}", self.game_handler.get_top_scores(10));
    }
}