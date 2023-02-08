use std::env;

use tfe::{Trainer, export_games_to_json_file, Game};

fn main() {
    let args: Vec<String> = env::args().collect();
    let saving_path: &str = &args[1];

    let mut trainer: Trainer = Trainer::new();

    let best_games_after_trainng: Vec<Game> = trainer.train();


    export_games_to_json_file(best_games_after_trainng, saving_path.to_string());
}
