use tfe::{Player, Trainer, Exporter};

fn main() {
    let player: Player = Player::new();
    let mut trainer: Trainer = Trainer::new(player);
    trainer.train();

}
