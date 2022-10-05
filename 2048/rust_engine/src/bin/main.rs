use tfe::Game;

fn main() {
    let game_results: Game = Game::play();
    for i in 0..game_results.boards.len() {
        println!("{:#02X}\n{:?}\t{:?}\n", game_results.boards[i], game_results.scores[i], game_results.moves[i]);
    }
    
}
