use tfe::Game;
use tfe::Statistics;
use stopwatch::{Stopwatch};

fn main() {
    // Arrange
    let stopwatch = Stopwatch::start_new();
    let total_games: u32 = 100;
    let workers: u32 = 3;

    // Play games
    let highest_tiles: Vec<u64> = Game::play_games(total_games, workers);
    
    // Display result
    println!("Total games: {:?}\nTime: {:?}", highest_tiles.len(), stopwatch.elapsed());
    println!("------------------------");

    Statistics::print_highest_tiles_statistics(highest_tiles);
}