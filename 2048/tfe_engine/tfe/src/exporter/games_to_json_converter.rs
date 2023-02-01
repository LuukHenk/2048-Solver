


use super::pow_unsafe;
use super::direction::Direction;
use super::board::Board;
use super::game::Game;

static SCORE_OBJECT_KEY: &'static str = "\"score\"";
static PERFORMED_MOVE_OBJECT_KEY: &'static str = "\"performed move\"";
static BOARD_OBJECT_KEY: &'static str = "\"board\"";
static JSON_DATA_SEPERATOR: &'static str = ", ";
static JSON_DICT_PAIR_SEPERATOR: &'static str = ": ";

pub fn construct_json_games_object(games: Vec<Game>) -> String {
    // FIXME add a test for this funcion when whe know how to make mock objects
    let mut json_game_objects: Vec<String> = Vec::with_capacity(games.len());
    for game in games {
        json_game_objects.push(__construct_json_game_object(game));
    }
    __construct_json_list(json_game_objects)
}


fn __construct_json_game_object(game: Game) -> String {
    // FIXME add a test for this funcion when whe know how to make mock objects
    let game_boards: Vec<Board> = game.copy_boards();
    let mut json_board_objects: Vec<String> = Vec::with_capacity(game_boards.len());
    for board in game_boards {
        json_board_objects.push(__construct_json_board_object(board));
    }
    __construct_json_list(json_board_objects)
}

fn __construct_json_board_object(board: Board) -> String{
    let mut board_sub_objects: Vec<String> = Vec::with_capacity(3);
    board_sub_objects.push(
        __format_board_dict_object(board.get_board())
    );
    board_sub_objects.push(
        __format_performed_move_dict_object(board.get_latest_movement())
    );
    board_sub_objects.push(
        __format_score_dict_object(board.get_score())
    );
    __construct_json_dict(board_sub_objects)


}

fn __format_score_dict_object(score: u64) -> String {
    __construct_json_dict_pair(
        SCORE_OBJECT_KEY.to_string(),
        score.to_string()    
    )
}

fn __format_performed_move_dict_object(performed_move: Direction) -> String {
    __construct_json_dict_pair(
        PERFORMED_MOVE_OBJECT_KEY.to_string(),
        performed_move.to_string()    
    )
}

fn __format_board_dict_object(board: u64) -> String {
    __construct_json_dict_pair(
        BOARD_OBJECT_KEY.to_string(),
        __board_u64_to_json(board)
    )
}

fn __board_u64_to_json(board: u64) -> String {
    let mut rows: Vec<String> = Vec::with_capacity(4);
    for row_index in 0..4 {
        let row_value: u64 = board >> row_index * 16 & 0xFFFF;
        rows.insert(0, __row_to_json(row_value));
    }
    rows.join(JSON_DATA_SEPERATOR)
}

fn __row_to_json(row: u64) -> String {
    let mut tiles: Vec<String> = Vec::with_capacity(4);
    for tile_index in 0..4 {
        let tile_value: u64 = row >> tile_index * 4 & 0xF;
        tiles.insert(0, pow_unsafe(tile_value).to_string());
    }
    __construct_json_list(tiles)
}

fn __construct_json_list(list_items: Vec<String>) -> String {
    vec!["[".to_string(), list_items.join(JSON_DATA_SEPERATOR), "]".to_string()].join("")
}

fn __construct_json_dict(dict_items: Vec<String>) -> String {
    vec!["{".to_string(), dict_items.join(JSON_DATA_SEPERATOR), "}".to_string()].join("")
}

fn __construct_json_dict_pair(key: String, value: String) -> String {
    vec![key, value].join(JSON_DICT_PAIR_SEPERATOR)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_json_board_object() {
        let board_str: String = "\"board\": [2, 4, 8, 16], [32, 64, 128, 256], [16, 8, 4, 2], [256, 128, 64, 32]".to_string();
        let latest_movement_str: String = "\"performed move\": Down".to_string();
        let score_str: String = "\"score\": 15".to_string();
        let expected_result: String = __construct_json_dict(vec![board_str, latest_movement_str, score_str]);
        let score: u64 = 15;
        let latest_movement: Direction = Direction::Down;
        let board_u64: u64 = 0x1234_5678_4321_8765;
        let mut board: Board = Board::new();
        board.score = score;
        board.lastest_movement = latest_movement;
        board.board = board_u64;

        assert_eq!(__construct_json_board_object(board), expected_result);

    }


    #[test]
    fn test_format_score_dict_object() {
        let score: u64 = 10;
        let expected_result: String = String::from(
            "\"score\": 10"
        );
        assert_eq!(__format_score_dict_object(score), expected_result);
    }    

    #[test]
    fn test_format_performed_move_dict_object() {
        let performed_move: Direction = Direction::Left;
        let expected_result: String = String::from(
            "\"performed move\": Left"
        );
        assert_eq!(__format_performed_move_dict_object(performed_move), expected_result);
    }    

    #[test]
    fn test_format_board_dict_object() {
        let board: u64 = 0x1234_5678_4321_8765;
        let expected_result: String = String::from(
            "\"board\": [2, 4, 8, 16], [32, 64, 128, 256], [16, 8, 4, 2], [256, 128, 64, 32]"
        );
        assert_eq!(__format_board_dict_object(board), expected_result);
    }    
    #[test]
    fn test_board_to_json() {
        let board: u64 = 0x1234_5678_4321_8765;
        let expected_result: String = String::from(
            "[2, 4, 8, 16], [32, 64, 128, 256], [16, 8, 4, 2], [256, 128, 64, 32]"
        );
        assert_eq!(__board_u64_to_json(board), expected_result);
    }
    #[test]
    fn test_row_to_json() {
        let board_row: u64 = 0x1234;
        let expected_result: String = String::from("[2, 4, 8, 16]");
        assert_eq!(__row_to_json(board_row), expected_result);
    }

    #[test]
    fn test_construct_json_dict_pair() {
        let key: String = String::from("hello");
        let value: String = String::from("goodbye");
        assert_eq!(__construct_json_dict_pair(key, value), String::from("hello: goodbye"));
    }

    #[test]
    fn test_constructing_json_list() {
        let data: &str = "hello";
        let object_data: Vec<String> = vec![String::from(data), String::from(data)];
        let expected_output: String = String::from("[hello, hello]");
        assert_eq!(__construct_json_list(object_data), expected_output);
    }

    #[test]
    fn test_constructing_json_dict() {
        let data: &str = "hello: goodbye";
        let object_data: Vec<String> = vec![String::from(data), String::from(data)];
        let expected_output: String = String::from("{hello: goodbye, hello: goodbye}");
        assert_eq!(__construct_json_dict(object_data), expected_output);
    }

}