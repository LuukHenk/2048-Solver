use tfe::Player;

fn main() {
    let trainins_set_size = 10_000_usize;
    let percentage_used = 0.01_f32;
    let threads = 7_usize;
    let trainings_rounds: usize = 10_usize;

    let mut player: Player = Player::new();
    player.train(
        trainins_set_size,
        percentage_used,
        threads,
        trainings_rounds,
    );
}
