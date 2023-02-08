

use super::player::Player;

pub struct Trainer{
    players: Vec<Player>,
    top_games: usize,
    games_per_trainings_round: usize
}

impl Trainer{
    pub fn new() -> Trainer {
        let total_players: usize = 10;
        let players: Vec<Player> = Trainer::__create_players(total_players);
        Trainer {
            players, 
            top_games: 10,
            games_per_trainings_round: 10000
        }
    }

    pub fn train(&mut self) {
        self.__player_selection()
        // Use the best player to select the best game (previous method, you know)
    }

    fn __player_selection(&mut self) {
        // Select the best player FIXME
        for player in self.players.iter_mut() {
            player.play_games(self.games_per_trainings_round);
            player.sort_games_on_score();
            player.resize_total_games(self.top_games);
            println!("Average score:\t{:#?}\n=========\n", player.get_average_score());
        }
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

    #[test]
    fn test_creating_a_new_trainer() {
        let trainer: Trainer = Trainer::new();
        assert!(trainer.games_per_trainings_round > 0);
        assert!(trainer.top_games <= trainer.games_per_trainings_round);
        assert!(trainer.players.len() > 0);
    }
}