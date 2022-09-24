use tfe::Game;
use tfe::Statistics;
use stopwatch::{Stopwatch};
// use std::thread;

fn main() {
    // Arrange
    let stopwatch = Stopwatch::start_new();
    let total_games = 1_000_000;

    // Play games
    let highest_tiles: Vec<u64> = Game::play_games(total_games);
    
    // Display result
    Statistics::print_highest_tiles_statistics(highest_tiles);
    println!("{:?}", stopwatch.elapsed());

}