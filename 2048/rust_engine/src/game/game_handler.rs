use multimap::MultiMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use super::game::Game;

pub fn play_games(total_games: usize, threads: usize) -> MultiMap<u64, Game> {
    let games_per_worker: usize = total_games / threads;
    let total_splitable_games: usize = games_per_worker * threads;
    let remaining_games: usize = total_games - total_splitable_games;

    let (sender, receiver): (Sender<Game>, Receiver<Game>) = mpsc::channel();
    for _ in 0..threads {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            for _ in 0..games_per_worker {
                sender_clone.send(Game::play()).unwrap();
            }
        });
    }

    let mut results: MultiMap<u64, Game> = MultiMap::with_capacity(total_games);
    for i in 0..total_splitable_games {
        print!(
            "Playing games... ({:.2}%)\r",
            i as f32 / total_splitable_games as f32 * 100.0
        );
        let game = receiver.recv().unwrap();
        results.insert(extract_final_score(&game), game);
    }
    print!("                                      \r");

    for _ in 0..remaining_games {
        let game = Game::play();
        results.insert(extract_final_score(&game), game);
    }

    results
}

fn extract_final_score(game: &Game) -> u64 {
    game.scores[game.scores.len() - 1]
}
