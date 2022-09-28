
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;

use super::direction::Direction;
use super::board::Board;
use super::algorithm::Algorithm;

pub struct Game{}

impl Game {
    pub fn play_games(total_games: u32, workers: u32) -> Vec<u64> {
        let games_per_worker: u32 = total_games / workers;
        let total_splitable_games: u32 = games_per_worker * workers;
        let remaining_games: u32 = total_games - total_splitable_games;

        let (sender, receiver): (Sender<u64>, Receiver<u64>) = mpsc::channel();
        for _ in 0 .. workers {
            let sender_clone = sender.clone(); 
            thread::spawn(
                move || for _ in 0 .. games_per_worker {
                    sender_clone.send(Game::play_game()).unwrap();
                }
            );
        }

        let mut results: Vec<u64> = Vec::new();
        for _ in 0 .. total_splitable_games { 
            print!("Playing games... ({:.2}%)\r", results.len() as f32 /total_games as f32 *100.0);              
            results.push(receiver.recv().unwrap()) 
        }
        print!("                                      \r");

        for _ in 0 .. remaining_games {
            results.push(Self::play_game())
        }
        
        results
    } 
    
    fn play_game() -> u64 {
        let mut board = Board::new();
        while Board::board_full(board) == false {
            board = Self::perform_movement(board);
            
        }
        let highest_tile = Board::get_highest_tile(board);
        highest_tile
    }

    fn perform_movement(mut board: u64) -> u64{
        let possible_movements: HashMap<Direction, u64> = Board::get_possible_movements(board);
        let direction: Direction = Algorithm::determine_best_movements(board, possible_movements);
        board = Board::perform_movement(board, &direction);
        board
    }
}