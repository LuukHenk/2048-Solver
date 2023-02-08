

use super::player::Player;

pub struct Trainer{
    players: Vec<Player>,
    top_games: usize,
    games_per_trainings_round: usize,
    trainings_rounds: usize
}

impl Trainer{
    pub fn new() -> Trainer {
        let total_players: usize = 10;
        let players = Trainer::__create_players(total_players);
        Trainer {
            players, 
            top_games: 100,
            games_per_trainings_round: 10000,
            trainings_rounds: 10
        }
    }

    pub fn train(&mut self) {
        for player in self.players.iter_mut() {
            player.play_games(self.games_per_trainings_round);
            println!("{:#?}", player.get_average_score());
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
        assert!(trainer.trainings_rounds > 0);
        assert!(trainer.players.len() > 0);
    }
}