use std::collections::VecDeque;

use super::pow_unsafe;
use super::game::Game;
use super::player::Player;

pub struct Exporter{
    player: Player
}

static JSON_DATA_SEPERATOR: &'static str = ", ";
static JSON_DICT_PAIR_SEPERATOR: &'static str = ": ";

impl Exporter{
    pub fn new(player: Player) -> Exporter {
        Exporter { player }
    }
//     pub fn export_all_games(&self)  {
//         let games: Vec<Game> = self.player.copy_games();
//         Self::__convert_games_to_json(games);
//     }

//     fn __convert_games_to_json(games: Vec<Game>) {
//         let mut json_data: Vec<String> = Vec::with_capacity(games.len());
//         let mut games_deque: VecDeque<Game> = games.into();
//         for _ in 0..games_deque.len() {
//             let first_game: Game = games_deque.pop_front().unwrap();
//             json_data.push(Self::__convert_game_to_json(first_game))
//         }
//     }

//     fn __convert_game_to_json(game: Game) -> String {
//         let game

//     }

//     fn __create_json_dictionary_item(key: String, value: String) -> String {
//         let mut dict: String = String::new();
//         dict.push_str(&key);
//         dict.push_str(": ");
//         dict.push_str(&value);
//         dict
//     }
    fn board_to_json(board: u64) -> String {
        let mut rows: Vec<String> = Vec::with_capacity(4);
        for row_index in 0..4 {
            let row_value: u64 = board >> row_index * 16 & 0xFFFF;
            rows.insert(0, Self::row_to_json(row_value));
        }
        rows.join(JSON_DATA_SEPERATOR)
    }

    fn row_to_json(row: u64) -> String {
        let mut tiles: Vec<String> = Vec::with_capacity(4);
        for tile_index in 0..4 {
            let tile_value: u64 = row >> tile_index * 4 & 0xF;
            tiles.insert(0, pow_unsafe(tile_value).to_string());
        }
        Self::construct_json_list(tiles)
    }

    fn construct_json_list(list_items: Vec<String>) -> String {
        vec![String::from("["), list_items.join(JSON_DATA_SEPERATOR), String::from("]")].join("")
    }

    fn construct_json_dict(dict_items: Vec<String>) -> String {
        vec![String::from("{"), dict_items.join(JSON_DATA_SEPERATOR), String::from("}")].join("")
    }

    fn construct_json_dict_pair(key: String, value: String) -> String {
        vec![key, value].join(JSON_DICT_PAIR_SEPERATOR)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board_to_json() {
        let board: u64 = 0x1234_5678_4321_8765;
        let expected_result: String = String::from(
            "[2, 4, 8, 16], [32, 64, 128, 256], [16, 8, 4, 2], [256, 128, 64, 32]"
        );
        assert_eq!(Exporter::board_to_json(board), expected_result);
    }
    #[test]
    fn test_row_to_json() {
        let board_row: u64 = 0x1234;
        let expected_result: String = String::from("[2, 4, 8, 16]");
        assert_eq!(Exporter::row_to_json(board_row), expected_result);
    }

    #[test]
    fn test_construct_json_dict_pair() {
        let key: String = String::from("hello");
        let value: String = String::from("goodbye");
        assert_eq!(Exporter::construct_json_dict_pair(key, value), String::from("hello: goodbye"));
    }

    #[test]
    fn test_constructing_json_list() {
        let data: &str = "hello";
        let object_data: Vec<String> = vec![String::from(data), String::from(data)];
        let expected_output: String = String::from("[hello, hello]");
        assert_eq!(Exporter::construct_json_list(object_data), expected_output);
    }

    #[test]
    fn test_constructing_json_dict() {
        let data: &str = "hello: goodbye";
        let object_data: Vec<String> = vec![String::from(data), String::from(data)];
        let expected_output: String = String::from("{hello: goodbye, hello: goodbye}");
        assert_eq!(Exporter::construct_json_dict(object_data), expected_output);
    }

}