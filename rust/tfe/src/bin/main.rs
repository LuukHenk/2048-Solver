use tfe::Game;
use tfe::Statistics;
use tfe::GameData;
use stopwatch::{Stopwatch};

fn main() {
    // Arrange
    let stopwatch = Stopwatch::start_new();
    let total_games: u32 = 10;
    let workers: u32 = 1;

    // Play games
    let played_games: Vec<GameData> = Game::play_games(total_games, workers);
    
    // Display result
    println!("Total games: {:?}\nTime: {:?}", played_games.len(), stopwatch.elapsed());
    println!("------------------------");
    println!("Game_data: {:?}", played_games);
    // Statistics::print_highest_tiles_statistics(highest_tiles);
}