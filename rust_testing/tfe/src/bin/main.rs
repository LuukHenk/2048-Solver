use tfe::GameHandler;

fn main() {
    let mut game_handler: GameHandler = GameHandler::new();
    game_handler.play_games(100)
}
