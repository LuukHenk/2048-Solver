use tfe::{Player, Trainer, export_games_to_json_file};

fn main() {
    let saving_path: String = "../../data/results.json".to_string();
    let player: Player = Player::new();
    let mut trainer: Trainer = Trainer::new(player, 1);
    trainer.train();
    export_games_to_json_file(trainer.copy_top_games(), saving_path);

}
