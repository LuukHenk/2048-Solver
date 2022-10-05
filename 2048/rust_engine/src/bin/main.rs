use tfe::Game;

fn main() {
    let game_results: Game = Game::play();
    println!("{:?}", game_results.boards);
}
