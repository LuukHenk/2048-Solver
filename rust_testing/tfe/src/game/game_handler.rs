use super::game::Game;

#[derive(Debug)]
pub struct GameHandler {
    games: Vec<Game>
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler{
            games: Vec::new()
        }
    }

    pub fn play_games(&mut self, amount: usize) {
        for _ in 0 .. amount {
            self.games.push(Game::play());
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defining_game_handler() {
        let game_handler: GameHandler = GameHandler::new();
        assert_eq!(game_handler.games.len(), 0);
    }

    #[test]
    fn test_playing_games() {
        let games_to_play = 100;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        assert_eq!(game_handler.games.len(), games_to_play);
    }
}