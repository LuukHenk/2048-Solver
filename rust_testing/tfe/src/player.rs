use super::game::game::Game;

pub struct Player {}

impl Player {
    pub fn new() -> Player {
        Player {}
    }

    pub fn train(self) {
        let mut game = Game::new();
        let result = game.play();
        println!("{:?}", result);
    }
}
