use super::Game;

pub fn convert_games_data_to_json(games: Vec<Game>) -> String {
    let mut games_data_json: Vec<String> = Vec::with_capacity(games.len());
    for game_index in 0..games.len() {
        let game: &Game = &games[game_index];
        let mut game_states: Vec<String> = Vec::with_capacity(game.boards.len());
        for game_state_index in 0..game.boards.len() {
            let formatted_game_state = create_desired_game_state_format(game, game_state_index);
            game_states.push(formatted_game_state);
        }
        games_data_json.push(convert_dataset_to_json_data_format(
            game_states,
            JsonDataFormats::List,
        ));
    }
    convert_dataset_to_json_data_format(games_data_json, JsonDataFormats::List)
}

fn create_desired_game_state_format(game: &Game, game_state_index: usize) -> String {
    let mut dataset: Vec<String> = Vec::with_capacity(3);
    dataset.push(create_json_dictionary_item(
        "'board'".to_string(),
        convert_board_to_json_list_format(game.boards[game_state_index]),
    ));
    dataset.push(create_json_dictionary_item(
        "'performed_move'".to_string(),
        game.movements[game_state_index].to_string(),
    ));
    dataset.push(create_json_dictionary_item(
        "'score'".to_string(),
        game.scores[game_state_index].to_string(),
    ));
    convert_dataset_to_json_data_format(dataset, JsonDataFormats::Dict)
}

fn convert_board_to_json_list_format(board: u64) -> String {
    let mut dataset: Vec<String> = Vec::with_capacity(4);
    for i in 0..4 {
        let row: u64 = board >> i * 16 & 0xFFFF;
        dataset.push(convert_row_to_json_list_format(row));
    }
    convert_dataset_to_json_data_format(dataset, JsonDataFormats::List)
}

fn convert_row_to_json_list_format(row: u64) -> String {
    let mut dataset: Vec<String> = Vec::with_capacity(4);
    for i in 0..4 {
        dataset.push((row >> i * 4 & 0xF).to_string());
    }
    convert_dataset_to_json_data_format(dataset, JsonDataFormats::List)
}

fn create_json_dictionary_item(key: String, value: String) -> String {
    let mut dict = String::new();
    dict.push_str(&key);
    dict.push_str(": ");
    dict.push_str(&value);
    dict
}
fn convert_dataset_to_json_data_format(
    dataset: Vec<String>,
    data_format: JsonDataFormats,
) -> String {
    let data_format_symbols: (String, String) = get_data_format_symbols(data_format);
    let mut out: String = data_format_symbols.0;
    out.push_str(&dataset.join(", "));
    out.push_str(&data_format_symbols.1);
    out
}

fn get_data_format_symbols(data_format: JsonDataFormats) -> (String, String) {
    match data_format {
        JsonDataFormats::List => ("[".to_string(), "]".to_string()),
        JsonDataFormats::Dict => ("{".to_string(), "}".to_string()),
    }
}

enum JsonDataFormats {
    List,
    Dict,
}
