use rand::Rng;
use strum::IntoEnumIterator;
use super::direction::Direction;
pub static EMPTY_BOARD: u64 = 0x0;
pub static ROW_MASK: u64 = 0xFFFF;

pub struct Game{pub board: u64}


impl Game {    
    pub fn play() {  
        // init
        let game = Self::new();
        let mut board = game.board;
        // println!("{:#02X} - new board", board);

        // play loop
        board = Self::perform_movement(board, &Direction::Right);
        // println!("{:#02X} - right move", board);
        
        board = Self::perform_movement(board, &Direction::Left);
        // println!("{:#02X} - left move", board);
        
        board = Self::perform_movement(board, &Direction::Down);
        // println!("{:#02X} - down move", board);
        
        board = Self::perform_movement(board, &Direction::Up);
        // println!("{:#02X} - up move", board);

        board = 0xFFFF_0000_0000_0000;
        for movement in Self::get_possible_movements(board) {
            println!("{:#?}", movement);
        };
        
    }

    pub fn perform_movement(mut board: u64, direction: &Direction) -> u64 {
        board = Self::execute(board, direction);
        Self::add_number_to_empty_position(board)
    }

    fn new() -> Self {
        let mut new_board: u64 = EMPTY_BOARD;
        new_board = Self::add_number_to_empty_position(new_board);
        new_board = Self::add_number_to_empty_position(new_board);
        return Game {board: new_board};
    }

    fn get_possible_movements(board: u64) -> Vec<Direction>{
        let mut possible_movements: Vec<Direction> = Vec::new();
        for direction in Direction::iter() {
            let mut tmp_board = board;
            tmp_board = Self::execute(tmp_board, &direction);

            if tmp_board != board { possible_movements.push(direction) }
        };
        possible_movements
        
    }

    fn execute(mut board: u64, direction: &Direction) -> u64 {
        board = match direction {
            Direction::Right => Self::right_move(board),
            Direction::Left => Self::left_move(board),
            Direction::Down => Self::down_move(board),
            Direction::Up => Self::up_move(board),
        };
        board
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
        let mut new_board: u64 = EMPTY_BOARD;
        for i in 0..4 {
            let row: u8 = i * 16;
            new_board ^= Self::merge_row_to_the_right((board >> row) & ROW_MASK) << row;
        }
        new_board
    }

    fn left_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        for i in 0..4 {
            let row: u8 = i * 16;
            let mut reversed_row = Self::reverse_row(board >> row & ROW_MASK);
            reversed_row = Self::merge_row_to_the_right(reversed_row);
            new_board ^= Self::reverse_row(reversed_row) << row;
        }
        new_board
    }

    fn down_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        let tmp_board: u64 = Self::transpose(board);
        for i in 0..4 {
            let row: u8 = i * 16;
            new_board ^= Self::merge_row_to_the_right(
                (tmp_board >> row) & ROW_MASK
            ) << row;
        }
        Self::transpose(new_board)
    }

    fn up_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        let tmp_board: u64 = Self::transpose(board);
        for i in 0..4 {
            let row: u8 = i * 16;
            let mut tmp_row = Self::reverse_row(tmp_board >> row & ROW_MASK);
            tmp_row = Self::merge_row_to_the_right(tmp_row);
            new_board ^= Self::reverse_row(tmp_row) << row;
        }
        Self::transpose(new_board)
    }

    fn reverse_row(row: u64) -> u64 {
        (row&0xF000)>>12|(row&0x0F00)>>4|(row&0x00F0)<<4|(row&0x000F)<<12
    }
    
    fn transpose(board: u64) -> u64 {
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;
        let a  = a1 | (a2 << 12) | (a3 >> 12);
        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;
        b1 | (b2 >> 24) | (b3 << 24)
    }

    fn merge_row_to_the_right(row: u64) -> u64 {
        let mut tmp_row: u64 = row;
        let mut new_row: u64 = EMPTY_BOARD;
        let mut tile_to_add: u64;
        let mut current_position_on_new_row: u8 = 0;
        let mut first_tile: u64 = 0x000F & tmp_row;
        tmp_row >>= 4;
        
        for _ in 0..3 {
            let second_tile = 0x000F & tmp_row;
            tmp_row >>= 4;
            if first_tile == 0 {
                first_tile = second_tile;
            } else if second_tile != 0 {
                if first_tile == second_tile {
                    tile_to_add = first_tile + 1;
                    first_tile = EMPTY_BOARD;
                } else  {
                    tile_to_add = first_tile;
                    first_tile = second_tile;
                }
                new_row += tile_to_add << current_position_on_new_row;
                current_position_on_new_row += 4;                
            }
    
        }
        if first_tile != 0 {
            new_row += first_tile << current_position_on_new_row;
        }
        new_row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_row_to_the_right() {
        let mut board: u64 = 0x0000_0000_0000_2101;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0022);

        let mut board: u64 = 0x0000_0000_0000_0010;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0001);

        let mut board: u64 = 0x0000_0000_0000_1100;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0002);

        let mut board: u64 = 0x0000_0000_0000_1110;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0012);

        let mut board: u64 = 0x0000_0000_0000_1111;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0022);

        let mut board: u64 = 0x0000_0000_0000_0000;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0000);

        let mut board: u64 = 0x0000_0000_0000_1211;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0122);

        let mut board: u64 = 0x0000_0000_0000_1221;
        board = Game::merge_row_to_the_right(board);
        assert_eq!(board, 0x0000_0000_0000_0131);
    }
}