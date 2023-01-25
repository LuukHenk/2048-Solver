use super::game::Game;

static PLAYING_MESSAGE: &'static str = "Playing games: ";
static SORTING_MESSAGE: &'static str = "Sorting games: ";

#[derive(Debug)]
pub struct Player {
    games: Vec<Game>
}

impl Player {

    pub fn new() -> Player {
        Player{
            games: Vec::new()
        }
    }

    pub fn play_games(&mut self, amount: usize) {
        for game_index in 0 .. amount {
            __display_status(PLAYING_MESSAGE, game_index + 1, amount);
            self.games.push(Game::new());
        }
    }

    pub fn retry_game(&mut self, index: usize, times: usize) {
        let mut game_to_retry = self.games[index].copy();
        game_to_retry.rewind(game_to_retry.get_index_of_highest_score_increasement());

        for game_index in 0 .. times {
            __display_status(PLAYING_MESSAGE, game_index + 1, times);
            let mut game = game_to_retry.copy();
            game.resume();
            self.games.push(game);
        }
    }

    pub fn resize_total_games(&mut self, maximum_games: usize) {
        let total_games: usize = self.games.len();
        if maximum_games >= total_games {
            return
        }
        self.games.drain(maximum_games..);
    }

    pub fn get_top_scores(&self, mut amount: usize) -> Vec<u64> {
        if amount > self.games.len() {
            amount = self.games.len()
        }
        let mut game_scores: Vec<u64> = self.__sort_game_scores();
        game_scores.drain(amount..);
        game_scores
    }

    pub fn sort_games_on_score(&mut self) {
        let sorted_game_scores: Vec<u64> = self.__sort_game_scores();
        
        for score_index in 0..sorted_game_scores.len(){
            __display_status(SORTING_MESSAGE, score_index + 1, sorted_game_scores.len());
            let score: u64 = sorted_game_scores[score_index];
            for game_index in 0..self.games.len() {
                if self.games[game_index].get_final_score() != score {continue}
                self.games.swap(game_index, score_index);
            }
        }
    }

    pub fn print_final_scores(&mut self) {
        self.sort_games_on_score();
        for game in self.games.iter() {
            println!("{:#?}", game.get_final_score());
        }
    }

    fn __sort_game_scores(&self) -> Vec<u64> {
        let mut game_scores: Vec<u64> = self.games.iter().map(
            |game| game.get_final_score()
        ).collect();
        game_scores.sort_by(|a, b| b.cmp(a));
        game_scores
    }

}


fn __display_status(message: &str, current_status: usize, goal: usize) {
    print!("                                        \r");
    print!("{}{:#?}/{:#?}", message, current_status, goal);
}

#[cfg(test)]
mod tests {
    use crate::game::board::Board;

    use super::*;
    const TOP_SCORE: u64 = 9999999999;

    #[test]
    fn test_defining_player() {
        let player: Player = Player::new();
        assert_eq!(player.games.len(), 0);
    }

    #[test]
    fn test_playing_games() {
        let games_to_play: usize = 100;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        assert_eq!(player.games.len(), games_to_play);
    }

    #[test]
    #[should_panic(expected="index out of bounds: the len is 0 but the index is 1")]
    fn test_retry_game_index_out_of_bound() {
        let mut player: Player = Player::new();
        player.retry_game(1, 1);
    }

    #[test]
    fn test_retry_game() {
        let games_to_play: usize = 1;
        let retries: usize = 20;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        let latest_played_game_before_retry = player.games[games_to_play-1].copy();

        player.retry_game(games_to_play-1, retries);

        assert_eq!(player.games[games_to_play-1], latest_played_game_before_retry);
        assert_eq!(player.games.len(), games_to_play + retries);
    }

    #[test]
    fn test_resize_total_games_with_maximum_size_higher_than_games_played() {
        let games_to_play: usize = 50;
        let maximum_size: usize = 100;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        player.resize_total_games(maximum_size);

        assert_eq!(player.games.len(), games_to_play)
    }

    #[test]
    fn test_resize_total_games_with_maximum_size_equal_to_games_played() {
        let games_to_play: usize = 50;
        let maximum_size: usize = 50;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        player.resize_total_games(maximum_size);

        assert_eq!(player.games.len(), games_to_play)
    }

    #[test]
    fn test_resize_total_games_with_maximum_size_lower_than_games_played() {
        let games_to_play: usize = 100;
        let maximum_size: usize = 50;
        let mut player: Player = Player::new();
        player.play_games(games_to_play);
        player.resize_total_games(maximum_size);

        assert_eq!(player.games.len(), maximum_size)
    }

    #[test]
    fn test_get_more_top_scores_than_games_played() {
        let games: Vec<Game> = vec![__create_game(10)];       
        let player: Player = Player {games};
        assert_eq!(player.get_top_scores(100).len(), 1);
    }

    #[test]
    fn test_get_top_scores() {
        let games: Vec<Game> = __create_games();
        let player: Player = Player {games};
        assert_eq!(player.get_top_scores(1), vec![TOP_SCORE]);
    }

    #[test]
    fn test_sort_scores() {
        let games: Vec<Game> = __create_games();

        let total_games = games.len();

        let mut player: Player = Player {games};
        player.sort_games_on_score();
        assert_eq!(player.games.len(), total_games);
        let mut latest_score = player.games[0].get_final_score();
        for game in player.games.iter() {
            assert!(latest_score >= game.get_final_score());
            latest_score = game.get_final_score();
        }
    }

    fn __create_games() -> Vec<Game> {
        // Creates a simple game set
        let mut games: Vec<Game> = Vec::new();
        games.push(__create_game(2));
        games.push(__create_game(TOP_SCORE));
        games.push(__create_game(2));
        games.push(__create_game(1));
        games.push(__create_game(TOP_SCORE));
        games.push(__create_game(1));
        games
    }

    fn __create_game(final_score: u64) -> Game {
        let mut board: Board = Board::new();
        board.score = final_score; 
        let mut game: Game = Game::new();
        game.boards = vec![board];
        game
    }
}