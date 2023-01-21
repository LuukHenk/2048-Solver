use super::game::Game;

pub struct GameHandler {
    games: Vec<Game>
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler{
            games: Vec::new()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defining_game_handler() {
        let game_handler = GameHandler::new();
        assert_eq!(game_handler.games, Vec::new())
    }
}