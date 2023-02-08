use std::env;

use tfe::{Trainer, export_games_to_json_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    // let saving_path: &str = &args[1];

    let mut trainer: Trainer = Trainer::new();

    trainer.train();
}
