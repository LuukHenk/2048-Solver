use tfe::{GameHandler, Player};

fn main() {
    let game_handler: GameHandler = GameHandler::new();
    let mut player: Player = Player::new(game_handler);
    player.train(10, 5000);
}
