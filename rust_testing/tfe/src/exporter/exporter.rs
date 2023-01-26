use super::games_to_json_converter::construct_json_games_object;
use super::export_to_file::write;
use super::game::Game;

pub fn export_games_to_json_file(games: Vec<Game>, file_path: String) {
    let player_json_object: String = construct_json_games_object(games);
    write(player_json_object, &file_path)
}