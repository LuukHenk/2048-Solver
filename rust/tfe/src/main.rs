use rand::Rng;

fn main() {
    Game::play()
}

pub static EMPTY_ROW: u64 = 0x0000;
pub static ROW_MASK: u64 = 0x0000_0000_0000_FFFF;

pub struct Game{pub board: u64}

impl Game {
    pub fn new() -> Self {
        return Game {board: 0x1234_5678_1234_0000_u64};
    }

    pub fn play() {  
        // init
        let game = Self::new();
        let mut board = game.board;
        board = Self::add_number_to_empty_position(board);
        board = Self::add_number_to_empty_position(board);
        println!("{:#02X}", board);

        // play loop
        board = Self::right_move(board);
        board = Self::add_number_to_empty_position(board);
        println!("{:#02X}", board);
    }

    fn add_number_to_empty_position(board: u64) -> u64 {
        let empty_tiles = Self::get_empty_tiles(board);
        let random_position = rand::thread_rng().gen_range(0..empty_tiles.len());
        let selected_empty_tile: u8 = empty_tiles[random_position];
        board | Self::get_number() << selected_empty_tile * 4
    }

    fn get_number() -> u64 {
        if rand::thread_rng().gen_range(0..10) == 10 { 2 } else { 1 }
    }

    fn get_empty_tiles(board: u64) -> Vec<u8> {
        let mut empty_tiles: Vec<u8> = Vec::new();
        for i in 0u8..16 {
            let tile = board >> i * 4 & 0xF;
            if tile == 0 { empty_tiles.push(i); } 
        }
        empty_tiles
    }
    
    fn right_move(board: u64) -> u64 {
        let mut new_board: u64 = 0x0000_0000_0000_0000;
        for i in 0..4 {
            let row = i * 16;
            new_board ^= Self::merge_row_to_the_right(board >> row & ROW_MASK) << row;
        }
        new_board
    }

    fn merge_row_to_the_right(row: u64) -> u64 {
        let mut tmp_row: u64 = row;
        let mut new_row: u64 = EMPTY_ROW;
        let mut tile_to_add: u64;
        let mut current_position_on_new_row: u8 = 0;
        let mut first_tile: u64 = 0x000F & tmp_row;
        let mut second_tile: u64 = EMPTY_ROW;
        tmp_row >>= 4;
        
        for _ in 0..3 {
            second_tile = 0x000F & tmp_row;
            tmp_row >>= 4;
            if first_tile == 0 {
                first_tile = second_tile;
            } else if second_tile != 0{
                if first_tile == second_tile {
                    tile_to_add = first_tile + 1;
                    first_tile = EMPTY_ROW;
                    second_tile = EMPTY_ROW;
                } else {
                    tile_to_add = first_tile;
                    first_tile = second_tile;
                }
                new_row += tile_to_add << current_position_on_new_row;
                current_position_on_new_row += 4;
            }
    
        }
        if second_tile != 0 {
            new_row += second_tile << current_position_on_new_row;
        }
    
        new_row
    }


}