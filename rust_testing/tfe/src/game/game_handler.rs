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

    pub fn drain_games(&mut self, amount: usize) {
        

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
        let games_to_play: usize = 100;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        assert_eq!(game_handler.games.len(), games_to_play);
    }

    #[test]
    fn test_drain_more_games_then_played() {
        let games_to_play: usize = 100;
        let games_to_drain: usize = 200;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        game_handler.drain_games(games_to_drain);
        assert_eq!(game_handler.games.len(), 0)
    }

    #[test]
    fn test_drain_games() {
        let games_to_play: usize = 100;
        let games_to_drain: usize = 50;
        let mut game_handler: GameHandler = GameHandler::new();
        game_handler.play_games(games_to_play);
        game_handler.drain_games(games_to_drain);
        assert_eq!(game_handler.games.len(), 50)
    }
}