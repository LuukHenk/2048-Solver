use tfe::{Trainer, export_games_to_json_file};

fn main() {
    let saving_path: &str = "../../data/results.json";
    let games_per_trainings_round: usize = 10000;
    let total_trainings_rounds: usize = 20;
    let top_games: usize = 10;
    let mut trainer: Trainer = Trainer::new(top_games);

    trainer.train(games_per_trainings_round, total_trainings_rounds);

    export_games_to_json_file(trainer.copy_top_games(), saving_path.to_string());
}