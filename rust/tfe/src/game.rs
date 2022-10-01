
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use super::direction::Direction;
use super::board::Board;
use super::algorithm::Algorithm;
use super::game_data_model::GameData;

pub struct Game{}

impl Game {
    pub fn play_games(total_games: u32, workers: u32) -> Vec<GameData> {
        let games_per_worker: u32 = total_games / workers;
        let total_splitable_games: u32 = games_per_worker * workers;
        let remaining_games: u32 = total_games - total_splitable_games;

        let (sender, receiver): (Sender<GameData>, Receiver<GameData>) = mpsc::channel();
        for _ in 0 .. workers {
            let sender_clone = sender.clone(); 
            thread::spawn(
                move || for _ in 0 .. games_per_worker {
                    sender_clone.send(Game::play_game()).unwrap();
                }
            );
        }

        let mut results: Vec<GameData> = Vec::new();
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
    
    fn play_game() -> GameData {
        let mut board: u64 = Board::new();
        let mut possible_movements: Vec<Direction> = Board::get_possible_movements(board);
        let mut direction: Direction;
        let mut boards: Vec<u64> = Vec::new();
        boards.push(board);

        while possible_movements.len() > 0 {
            direction = Algorithm::determine_best_movement(possible_movements);
            board = Board::perform_movement(board, &direction);
            boards.push(board);
            possible_movements = Board::get_possible_movements(board);
        }

        GameData {
            boards: boards,
            final_score: Board::get_score(board)
        }
    }
}