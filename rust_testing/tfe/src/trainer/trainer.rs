
use super::player::Player;

pub struct Trainer{
    player: Player
}

impl Trainer{
    pub fn new(player: Player) -> Trainer {
        Trainer {player}
    }

    pub fn train(&mut self, trainings_rounds: usize, trainings_round_size: usize) {
        println!("Playing initial games");
        self.player.play_games(trainings_round_size);
        for trainings_round in 0..trainings_rounds {
            println!("\nTrainings round: {:#?}/{:#?}", trainings_round + 1, trainings_rounds);
            self.player.play_games(trainings_round_size);
            self.player.drain_games(trainings_round_size);
        }
        println!("{:#?}", self.player.get_top_scores(10));
    }
}