use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use super::game::Game;

pub fn play_games(total_games: usize, threads: usize) -> Vec<Game> {
    let games_per_worker: usize = total_games / threads;
    let total_splitable_games: usize = games_per_worker * threads;
    let remaining_games: usize = total_games - total_splitable_games;

    let (sender, receiver): (Sender<Game>, Receiver<Game>) = mpsc::channel();
    for _ in 0 .. threads {
        let sender_clone = sender.clone(); 
        thread::spawn(
            move || for _ in 0 .. games_per_worker {
                sender_clone.send(Game::play()).unwrap();
            }
        );
    }

    let mut results: Vec<Game> = Vec::with_capacity(total_games);
    for _ in 0 .. total_splitable_games { 
        print!("Playing games... ({:.2}%)\r", results.len() as f32 /total_games as f32 *100.0);              
        results.push(receiver.recv().unwrap()) 
    }
    print!("                                      \r");

    for _ in 0 .. remaining_games {
        results.push(Game::play())
    }
    
    results
} 