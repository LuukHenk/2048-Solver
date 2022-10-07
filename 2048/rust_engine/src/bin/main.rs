use tfe::Player;

fn main() {
    let max_games_in_set = 10_usize;
    let threads = 1_usize;
    let selection_size = 20_usize;

    let mut player: Player = Player::new(max_games_in_set);
    player.train(threads, selection_size);
}
