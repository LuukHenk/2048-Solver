use std::env;

use tfe::{export_games_to_json_file, Trainer};

fn main() {
    let args: Vec<String> = env::args().collect();
    let saving_path: &str = if args.len() > 1 {
        &args[1]
    } else {
        "results.json"
    };

    let games_per_trainings_round: usize = 10000;
    let total_trainings_rounds: usize = 10;
    let top_games: usize = 10;
    let mut trainer: Trainer = Trainer::new(top_games);

    trainer.train(games_per_trainings_round, total_trainings_rounds);

    export_games_to_json_file(trainer.copy_top_games(), saving_path.to_string());
}
