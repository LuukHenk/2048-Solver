use tfe::{Player, Trainer};

fn main() {
    let player: Player = Player::new();
    let mut trainer: Trainer = Trainer::new(player);
    trainer.train(10, 5000);
}
