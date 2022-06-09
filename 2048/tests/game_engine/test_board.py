""" Testing the 2048 game board"""

from unittest import main as unittest_main, TestCase
from typing import List

from src.game_engine.board import Board


class TestBoard(TestCase):
    """Tests the 2048 game board"""

    DEFAULT_BOARD_SIZE = 4

    def set_values_on_board(self, board: List[List[int]]):
        """Fill the 2048 game board with values"""
        output_board = Board(self.DEFAULT_BOARD_SIZE)
        for y_position, row in enumerate(board):
            for x_position, value in enumerate(row):
                output_board.set_value(value, x_position, y_position)
        return output_board

    def test_default_board_generation(self):
        """Test the initiation of the board"""
        # Arrange
        default_board_size = self.DEFAULT_BOARD_SIZE
        board = Board()
        # Act & Assert
        self.assertEqual(board.size, default_board_size)
        self.assertEqual(len(board.board), default_board_size)
        for row in board.board:
            self.assertEqual(row, [0 for _ in range(default_board_size)])

    def test_board_generation_altered_size(self):
        """Test the initiation of the board with an altered board size"""
        # Arrange
        altered_board_size = 6
        board = Board(altered_board_size)
        # Act & Assert
        self.assertEqual(board.size, altered_board_size)
        self.assertEqual(len(board.board), altered_board_size)
        for row in board.board:
            self.assertEqual(row, [0 for _ in range(altered_board_size)])

    def test_board_generation_invalid_size(self):
        """Invalid board size should raise a ValueError"""
        # Arrange, Act & Assert
        self.assertRaises(ValueError, Board, -1)

    def test_board_setter(self):
        """Test setting a board"""
        # Arrange
        board = Board(4)
        output = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]]
        # Act
        board.board = output
        # Assert
        self.assertEqual(board.board, output)

    def test_board_setter_invalid_board(self):
        """Test setting a board with an invalid board size"""
        board = Board(4)
        invalid_board_1 = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        invalid_board_2 = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        with self.assertRaises(IndexError):
            board.board = invalid_board_1
        with self.assertRaises(IndexError):
            board.board = invalid_board_2

    def test_set_value(self):
        """Test setting a value on the board"""
        # Arrange
        board = Board(4)
        expected_output = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]]
        # Act
        board.set_value(1, 0, 0)
        board.set_value(2, 3, 3)
        # Assert
        self.assertEqual(board.board, expected_output)

    def test_set_value_invalid_position(self):
        """Test setting an invalid value on the board"""
        # Arrange
        board_size = self.DEFAULT_BOARD_SIZE
        board = Board(board_size)
        max_negative = (board_size + 1) * -1
        # Act & Assert
        self.assertRaises(IndexError, board.set_value, 1, max_negative, 0)
        self.assertRaises(IndexError, board.set_value, 1, 0, max_negative)
        self.assertRaises(IndexError, board.set_value, 1, board_size, 0)
        self.assertRaises(IndexError, board.set_value, 1, 0, board_size)

    def test_rotate(self):
        """Test the rotation of the board"""
        # Arrange
        inp_board = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 0, 1], [2, 3, 4, 5]]
        board = self.set_values_on_board(inp_board)
        expected_output_board = [[0, 4, 8, 2], [1, 5, 9, 3], [2, 6, 0, 4], [3, 7, 1, 5]]
        # Act
        board.rotate()
        # Assert
        self.assertEqual(board.board, expected_output_board)

    def test_empty_tile_positions(self):
        """Test finding the position of empty tiles"""
        # Arrange
        inp_board = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 0, 1], [2, 3, 4, 5]]
        board = self.set_values_on_board(inp_board)
        expected_output = [{"y": 0, "x": 0}, {"y": 2, "x": 2}]
        # Act & Assert
        self.assertEqual(board.empty_tile_positions(), expected_output)

    def test_full_board(self):
        """Tests the full board check"""
        # Arrange
        inp_board = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 1, 1], [2, 3, 4, 5]]
        board = self.set_values_on_board(inp_board)
        # Act
        not_full = board.full()
        board.set_value(1, 0, 0)
        full = board.full()
        # Assert
        self.assertEqual(not_full, False)
        self.assertEqual(full, True)


if __name__ == "__main__":
    unittest_main()
