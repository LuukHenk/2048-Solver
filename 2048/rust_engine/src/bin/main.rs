use tfe::Board;
use tfe::Direction;

fn main() {
    let mut board = Board::new();
    board.perform_movement(&Direction::Right);
}
