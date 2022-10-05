use rand::Rng;

use super::direction::Direction;


pub static EMPTY_BOARD: u64 = 0x0;
pub static TILE_MASK: u64 = 0xF;
pub static TILES_IN_BOARD: usize = 16;
pub static ROW_MASK: u64 = 0xFFFF;

pub struct Board{board: u64}

impl Board {

    pub fn new() -> Board {
        let mut board: Board = Board{board: EMPTY_BOARD};
        Self::spawn_tile(&mut board);
        Self::spawn_tile(&mut board);
        board
    }

    pub fn perform_movement(&mut self, direction: &Direction) {
        self.board = Self::execute(self.board, direction);
        Self::spawn_tile(self)
    }

    pub fn print_board(&self) {
        // This function will not be tested
        println!("{:#02X}", self.board);
    }

    fn spawn_tile(&mut self) {
        let empty_tiles = Self::get_empty_tiles(self);
        let random_position = rand::thread_rng().gen_range(0..empty_tiles.len());
        let selected_empty_tile: usize = empty_tiles[random_position];
        self.board = self.board | Self::generate_new_tile() << selected_empty_tile * 4;
    }

    fn get_empty_tiles(&self) -> Vec<usize> {
        let mut empty_tiles: Vec<usize> = Vec::with_capacity(TILES_IN_BOARD);
        for i in 0..TILES_IN_BOARD {
            if Self::get_tile(self, i) == 0 { empty_tiles.push(i); } 
        }
        empty_tiles
    }

    fn execute(board: u64, direction: &Direction) -> u64 {
        match direction {
            Direction::Right => Self::right_move(board),
            Direction::Left => Self::left_move(board),
            Direction::Down => Self::down_move(board),
            Direction::Up => Self::up_move(board),
        }
    }

    fn right_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        for i in 0..4 {
            let row: u8 = i * 16;
            new_board ^= Self::merge_row_to_the_right(&((board >> row) & ROW_MASK)) << row;
        }
        new_board
    }
    fn left_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        for i in 0..4 {
            let row: u8 = i * 16;
            let mut reversed_row = Self::reverse_row(board >> row & ROW_MASK);
            reversed_row = Self::merge_row_to_the_right(&reversed_row);
            new_board ^= Self::reverse_row(reversed_row) << row;
        }
        new_board
    }

    fn down_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        let tmp_board: u64 = Self::transpose(&board);
        for i in 0..4 {
            let row: u8 = i * 16;
            new_board ^= Self::merge_row_to_the_right(
                &((tmp_board >> row) & ROW_MASK)
            ) << row;
        }
        Self::transpose(&new_board)
    }

    fn up_move(board: u64) -> u64 {
        let mut new_board: u64 = EMPTY_BOARD;
        let tmp_board: u64 = Self::transpose(&board);
        for i in 0..4 {
            let row: u8 = i * 16;
            let mut tmp_row = Self::reverse_row(tmp_board >> row & ROW_MASK);
            tmp_row = Self::merge_row_to_the_right(&tmp_row);
            new_board ^= Self::reverse_row(tmp_row) << row;
        }
        Self::transpose(&new_board)
    }

    fn get_tile(&self, index: usize) -> u64 {
        self.board >> index * 4 & TILE_MASK
    }

    fn generate_new_tile() -> u64 {
        if rand::thread_rng().gen_range(0..10) == 10 { 2 } else { 1 }
    }

    fn transpose(board: &u64) -> u64 {
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;
        let a  = a1 | (a2 << 12) | (a3 >> 12);
        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;
        b1 | (b2 >> 24) | (b3 << 24)
    }

    fn reverse_row(row: u64) -> u64 {
        (row&0xF000)>>12|(row&0x0F00)>>4|(row&0x00F0)<<4|(row&0x000F)<<12
    }

    fn merge_row_to_the_right(&row: &u64) -> u64 {
        let mut tmp_row: u64 = row;
        let mut new_row: u64 = EMPTY_BOARD;
        let mut tile_to_add: u64;
        let mut current_position_on_new_row: u8 = 0;
        let mut first_tile: u64 = TILE_MASK & tmp_row;
        tmp_row >>= 4;
        
        for _ in 0..3 {
            let second_tile = TILE_MASK & tmp_row;
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
    fn test_creating_new_board() {
        let mut board: Board = Board::new();
        let mut total_set_values: u8 = 0;
        let tile_with_value_1: u64 = 0x0000_0000_0000_0001;
        let tile_with_value_2: u64 = 0x0000_0000_0000_0002;
        for _i in 0..16 {
            if board.board & tile_with_value_1 != EMPTY_BOARD || board.board & tile_with_value_2 != EMPTY_BOARD {
                total_set_values = total_set_values + 1;
            }
            board.board >>= 4;
        }
        assert_eq!(total_set_values, 2);
    }
    
    #[test]
    fn test_spawn_tile_during_perform_movement() {
        let mut board: Board = Board::new();
        board.board = 0x1234_5678_9ABC_DEF0;
        board.perform_movement(&Direction::Right);
        board.print_board();
        assert_eq!(board.get_empty_tiles(), Vec::new());
    }

    #[test]
    fn test_get_tile() {
        let mut board: Board = Board::new();
        board.board = 0x1234_5678_9ABC_DEF0;
        assert_eq!(board.get_tile(2_usize), 0x0000_0000_0000_000E); 
    }

    #[test]
    fn test_get_empty_tiles() {
        let mut board: Board = Board::new();
        board.board = 0x1234_5678_9ABC_DEF0;
        assert_eq!(board.get_empty_tiles(), vec![0_usize]); 
        board.board = 0x1234_5670_9ABC_DEF0;
        assert_eq!(board.get_empty_tiles(), vec![0_usize, 8_usize]); 
        board.board = 0x1234_5678_9ABC_DEF1;
        assert_eq!(board.get_empty_tiles(), Vec::new()); 
    }

    #[test]
    fn test_spawn_tile() {
        let mut board: Board = Board::new();
        board.board = 0x1234_5678_9ABC_DEF0;
        board.spawn_tile();
        assert_eq!(board.get_empty_tiles(), Vec::new());
    }

    #[test] 
    fn test_execute_right() {
        assert_eq!(Board::execute(0x1122_1123_3448_0058, &Direction::Right), 0x0023_0223_0358_0058)
    }

    #[test] 
    fn test_execute_left() {
        assert_eq!(Board::execute(0x8500_8443_3211_2211, &Direction::Left), 0x8500_8530_3220_3200)
    }
    #[test] 
    fn test_execute_down() {
        assert_eq!(Board::execute(0x0311_0411_5422_8832, &Direction::Down), 0x0000_0320_5522_8833)
    }
    #[test] 
    fn test_execute_up() {
        assert_eq!(Board::execute(0x8832_5422_0411_0311, &Direction::Up), 0x8833_5522_0320_0000)
    }

    #[test]
    fn test_right_move_complex() {
        assert_eq!(Board::right_move(0x1122_1123_3448_0058), 0x0023_0223_0358_0058);
    }
    #[test]
    fn test_right_move_no_tiles() {
        assert_eq!(Board::right_move(EMPTY_BOARD), EMPTY_BOARD);
    }
    #[test]
    fn test_right_move_simple() {
        assert_eq!(Board::right_move(0x0011_0000_2200_0330), 0x0002_0000_0003_0004);
    }
    #[test]
    fn test_left_move_complex() {
        assert_eq!(Board::left_move(0x8500_8443_3211_2211), 0x8500_8530_3220_3200);
    }
    #[test]
    fn test_left_move_no_tiles() {
        assert_eq!(Board::left_move(EMPTY_BOARD), EMPTY_BOARD);
    }
    #[test]
    fn test_left_move_simple() {
        assert_eq!(Board::left_move(0x1100_0000_0022_0330), 0x2000_0000_3000_4000);
    }
    
    #[test]
    fn test_up_move_complex() {
        assert_eq!(Board::up_move(0x8832_5422_0411_0311), 0x8833_5522_0320_0000);
    }
    #[test]
    fn test_up_move_no_tiles() {
        assert_eq!(Board::up_move(EMPTY_BOARD), EMPTY_BOARD);
    }
    #[test]
    fn test_up_move_simple() {
        assert_eq!(Board::up_move(0x1000_1003_0023_0020), 0x2034_0000_0000_0000);
    }
    #[test]
    fn test_down_move_complex() {
        assert_eq!(Board::down_move(0x0311_0411_5422_8832), 0x0000_0320_5522_8833);
    }
    #[test]
    fn test_down_move_no_tiles() {
        assert_eq!(Board::down_move(EMPTY_BOARD), EMPTY_BOARD);
    }
    #[test]
    fn test_down_move_simple() {
        assert_eq!(Board::down_move(0x1000_1003_0023_0020), 0x0000_0000_0000_2034);
    }
    

    #[test]
    fn test_reverse_row() {
        assert_eq!(Board::reverse_row(0x1234), 0x4321);
    }
    
    #[test]
    fn test_transpose() {
        assert_eq!(Board::transpose(&0x0123_4567_89AB_CDEF), 0x048C_159D_26AE_37BF_u64);
    }

    #[test]
    fn test_merge_row_to_the_right_with_two_equal_values_after_merge() {
        assert_eq!(Board::merge_row_to_the_right(&0x2101), 0x0022);
    }

    #[test]
    fn test_merge_row_to_the_right_with_single_value() {
        assert_eq!(Board::merge_row_to_the_right(&0x0010), 0x0001);
    }

    #[test]
    fn test_merge_row_to_the_right_with_two_equal_values_before_merge() {
        assert_eq!(Board::merge_row_to_the_right(&0x1100), 0x0002);
    }

    #[test]
    fn test_merge_row_to_the_right_with_three_equal_values_before_merge() {
        assert_eq!(Board::merge_row_to_the_right(&0x1101), 0x0012);
    }

    #[test]
    fn test_merge_row_to_the_right_with_four_equal_values_before_merge() {
        assert_eq!(Board::merge_row_to_the_right(&0x1111), 0x0022);
    }

    #[test]
    fn test_merge_row_to_the_right_with_no_values() {
        assert_eq!(Board::merge_row_to_the_right(&0x0000), 0x0000);
    }

    #[test]
    fn test_merge_row_to_the_right_complex() {
        assert_eq!(Board::merge_row_to_the_right(&0x1211), 0x0122);
    }

    #[test]
    fn test_merge_row_to_the_right_complex_with_higher_values() {
        assert_eq!(Board::merge_row_to_the_right(&0x1221), 0x0131);
    }
}