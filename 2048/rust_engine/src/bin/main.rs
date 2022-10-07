use tfe::Player;

fn main() {
    let trainins_set_size = 100_usize;
    let percentage_used = 0.01_f32;
    let threads = 1_usize;
    let trainings_rounds: usize = 1_usize;
    let saving_file_path: String = String::from("../../data/results.json");

    let mut player: Player = Player::new();
    player.train(
        trainins_set_size,
        percentage_used,
        threads,
        trainings_rounds,
    );
    player.export_games(1, &saving_file_path);
}
