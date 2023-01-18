use multimap::MultiMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use super::game::Game;

pub fn play_games(
    total_games: usize,
    threads: usize,
    existing_game: Option<Game>,
) -> MultiMap<u64, Game> {
    let games_per_worker: usize = total_games / threads;
    let total_splitable_games: usize = games_per_worker * threads;
    let remaining_games: usize = total_games - total_splitable_games;

    let (sender, receiver): (Sender<(u64, Game)>, Receiver<(u64, Game)>) = mpsc::channel();
    for _ in 0..threads {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            for _ in 0..games_per_worker {
                sender_clone.send(play_game(existing_game.clone())).unwrap();
            }
        });
    }

    let mut results: MultiMap<u64, Game> = MultiMap::with_capacity(total_games);
    for i in 0..total_splitable_games {
        print!(
            "Playing games... ({:.2}%)\r",
            i as f32 / total_splitable_games as f32 * 100.0
        );
        let game_result = receiver.recv().unwrap();
        results.insert(game_result.0, game_result.1);
    }
    print!("                                      \r");

    for _ in 0..remaining_games {
        let game_result = play_game(existing_game.clone());
        results.insert(game_result.0, game_result.1);
    }

    results
}

fn play_game(existing_game: Option<Game>) -> (u64, Game) {
    let game = match existing_game {
        Some(game) => game,
        None => Game::new(),
    };
    game.play()
}
