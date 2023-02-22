

use super::player::Player;
use super::game::Game;

pub struct Trainer{
    players: Vec<Player>,
    top_games: usize,
    games_per_trainings_round: usize
}

impl Trainer{
    pub fn new() -> Trainer {
        let total_players: usize = 5;
        let players: Vec<Player> = Trainer::__create_players(total_players);
        Trainer {
            players, 
            top_games: 10,
            games_per_trainings_round: 10000
        }
    }

    pub fn train(&mut self) -> Vec<Game> {
        self.__player_selection();
        let best_games_after_training: Vec<Game> = self.players[0].copy_games();
        best_games_after_training
    }

    fn __player_selection(&mut self) {
        self.__play_games();
        let player_id_filter: usize = self.__get_player_id_of_player_with_best_average_score();
        self.__filter_players(player_id_filter);

    }

    fn __play_games(&mut self) {
        let retries_per_game: usize = self.games_per_trainings_round / self.top_games;
        for player_id in 0..self.players.len() - 1 {
            println!("Player {}", player_id+1);
            let player: &mut Player = &mut self.players[player_id]; 
            player.play_games(self.games_per_trainings_round);
            player.sort_games_on_score();
            player.resize_total_games(self.top_games);
            player.improve_games(retries_per_game);
            println!("Average score:\t{:#?}", player.get_average_score());
            println!("Top score:\t{:#?}\n=========\n", player.get_top_score());
        }
    }

    fn __filter_players(&mut self, player_id_filter: usize) {
        self.players.drain(player_id_filter+1..);
        self.players.drain(..player_id_filter);
    }

    fn __get_player_id_of_player_with_best_average_score(&self) -> usize {
        let mut player_id_with_best_average_score: usize = 0;
        let mut best_average_player_score: u64 = 0;
        
        for player_id in 0 .. self.players.len() - 1 {
            let player: &Player = &self.players[player_id];
            let average_score: u64 = player.get_average_score();
            if average_score > best_average_player_score {
                best_average_player_score = average_score;
                player_id_with_best_average_score = player_id;
            }
        }
        player_id_with_best_average_score    
    }

    fn __create_players(total_players: usize) -> Vec<Player>{
        let mut players: Vec<Player> = Vec::with_capacity(total_players);
        for _ in 0 .. total_players {
            players.push(Player::new());
        }
        players
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    // Fixme: test more when we have a mocker

    #[test]
    fn test_creating_a_new_trainer() {
        let trainer: Trainer = Trainer::new();
        assert!(trainer.games_per_trainings_round > 0);
        assert!(trainer.top_games <= trainer.games_per_trainings_round);
        assert_eq!(trainer.players.len(), 10);
    }
    #[test] 
    fn test_player_selection() {
        // Fixme: test more when we have a mocker
        let mut trainer: Trainer = Trainer::new();
        trainer.games_per_trainings_round = 1;
        trainer.top_games = 1;
        trainer.__player_selection();
        assert!(trainer.players.len() == 1);
    }
    #[test]
    fn test_playing_games() {
        // Fixme: test game sorting when we have a mocker

        let mut trainer: Trainer = Trainer::new();
        trainer.games_per_trainings_round = 1;
        trainer.top_games = 1;
        trainer.__play_games();

        for player in trainer.players.iter() {
            assert!(player.get_average_score() > 0);
            assert!(player.games.len() == trainer.top_games )
        }

    }

    #[test]
    fn test_filter_on_player_id() {
        let player_id_filter: usize = 4;
        let mut trainer: Trainer = Trainer::new();
        trainer.games_per_trainings_round = 1;
        trainer.top_games = 1;
        trainer.__play_games();
        let score_of_filtered_player: u64 = trainer.players[player_id_filter].get_average_score(); 

        trainer.__filter_players(player_id_filter);

        assert_eq!(trainer.players.len(), 1);
        assert_eq!(trainer.players[0].get_average_score(), score_of_filtered_player)
    }

    #[test]
    fn test_get_player_id_of_player_with_best_average_score() {
        // Fixme: test when we can mock player.get_average_score()
    }   

    #[test]
    fn test_get_player_id_of_player_with_best_average_score_when_players_have_no_score() {
        let trainer: Trainer = Trainer::new();
        let player_id: usize = trainer.__get_player_id_of_player_with_best_average_score();
        assert_eq!(player_id, 0)
    }   

    #[test]
    fn test_create_players() {
        let total_players: usize = 10;
        let players: Vec<Player> = Trainer::__create_players(total_players);
        assert_eq!(players.len(), total_players);
    }
}