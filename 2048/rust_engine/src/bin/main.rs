use stopwatch::{Stopwatch};

use tfe::game_handler;
use tfe::Game;

fn main() {
    // Arrange
    let total_games: usize = 1_000_000;
    let threads: usize = 1;

    // Act
    let stopwatch = Stopwatch::start_new();
    let played_games: Vec<Game> = game_handler::play_games(total_games, threads);

    // Result
    println!("Total games: {:?}\nTime: {:?}", played_games.len(), stopwatch.elapsed());    
    println!("------------------------");
}
