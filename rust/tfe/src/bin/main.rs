use tfe::Game;
use tfe::Statistics;
use tfe::Player;
use stopwatch::{Stopwatch};

fn main() {
    // Arrange
    let stopwatch = Stopwatch::start_new();
    let total_games: u32 = 10;
    let workers: u32 = 1;

    // Play games
    Player::train(total_games, workers, 0_u8);
    
    // Display result
    // println!("Total games: {:?}\nTime: {:?}", played_games.len(), stopwatch.elapsed());
    // println!("------------------------");
    // println!("Game_data: {:?}", played_games);
    // Statistics::print_highest_tiles_statistics(highest_tiles);
}