use tfe::{GameHandler, Trainer};

fn main() {
    let game_handler: GameHandler = GameHandler::new();
    let mut trainer: Trainer = Trainer::new(game_handler);
    trainer.train(10, 5000);
}
