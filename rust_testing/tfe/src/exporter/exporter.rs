use super::games_to_json_converter::construct_json_player_object;
use super::export_to_file::write;
use super::player::Player;

pub struct Exporter{
    player: Player
}



impl Exporter{
    // FIXME add tests for class when whe know how to make mock objects
    pub fn new(player: Player) -> Exporter {
        Exporter { player }
    }

    pub fn export_games_to_json_file(self, file_path: String) {
        let player_json_object: String = construct_json_player_object(self.player);
        write(player_json_object, &file_path)
    }
}