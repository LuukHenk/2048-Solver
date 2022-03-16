from re import A
from unittest import main, TestCase
from unittest.mock import MagicMock, patch
from dataclasses import fields

from defer import return_value
from pyrfc3339 import generate

import game

class TestGame(TestCase):

    EXPECTED_DIRECTIONS = ["left", "right", "down", "up"]
    def test_directions_dataclass(self):
        """Test if the expected directions are in the directions dataclass """
        directions = [direction.name for direction in fields(game.Directions())]
        self.assertEqual(directions, self.EXPECTED_DIRECTIONS)

    def test_directions_property(self):
        """Test if the game its directions property returns the correct directions"""
        current_game = game.Game()
        self.assertEqual(current_game.directions, self.EXPECTED_DIRECTIONS)

    def test_default_board_size(self):
        """ Test the expected default board size """
        current_game = game.Game()
        default_size = 4
        self.assertEqual(len(current_game.board), default_size)
        for row in current_game.board:
            self.assertEqual(len(row), default_size)

    def test_alternative_board_size(self):
        """ Test the altered board size """

        altered_size = 6
        current_game = game.Game(board_size=altered_size)
        self.assertEqual(len(current_game.board), altered_size)
        for row in current_game.board:
            self.assertEqual(len(row), altered_size)

    def test_perform_move_to_left(self):
        """
        Test performing a movement to the left
        This also tests the up movement (without the rotation part)
        """
        input_board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 2, 2],
        ]
        expected_output_board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
        ]
        
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=input_board), 
            set=MagicMock()
        ):
            current_game = game.Game()
            current_game.perform_movement("left")
            self.assertEqual(current_game.board, expected_output_board)
            self.assertEqual(current_game.current_score, 4)
            self.assertEqual(current_game.total_moves_performed, 1)
    
    def test_perform_move_to_right(self):
        """
        Test performing a movement to the right
        This also tests the down movement (without the rotation part)
        """
        input_board = [
            [0, 0, 0, 0],
            [2, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 2],
        ]
        expected_output_board = [
            [0, 0, 0, 0],
            [0, 0, 0, 4],
            [0, 0, 0, 2],
            [0, 0, 0, 2],
        ]
        
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=input_board), 
            set=MagicMock()
        ):
            current_game = game.Game()
            current_game.perform_movement("right")
            self.assertEqual(current_game.board, expected_output_board)
            self.assertEqual(current_game.current_score, 4)
            self.assertEqual(current_game.total_moves_performed, 1)

    def test_perform_up_movement(self):
        """
        Up should give the same result as the left movement
        but also call the board rotate twice
        """
        input_board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 2, 2],
        ]
        expected_output_board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
        ]
        
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=input_board),
            rotate=MagicMock(return_value=input_board),
            set=MagicMock()
        ):
            current_game = game.Game()
            current_game.perform_movement("up")
            self.assertEqual(current_game.board, expected_output_board)
            self.assertEqual(current_game._Game__board.rotate.call_count, 2)

    def test_perform_move_down(self):
        """
        Down should give the same result as the right movement
        but also call the board rotate twice
        """
        input_board = [
            [0, 0, 0, 0],
            [2, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 2],
        ]
        expected_output_board = [
            [0, 0, 0, 0],
            [0, 0, 0, 4],
            [0, 0, 0, 2],
            [0, 0, 0, 2],
        ]
        
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=input_board),
            rotate=MagicMock(return_value=input_board),
            set=MagicMock()
        ):
            current_game = game.Game()
            current_game.perform_movement("down")
            self.assertEqual(current_game.board, expected_output_board)
            self.assertEqual(current_game._Game__board.rotate.call_count, 2)
    
    def test_all_movements_possible(self):
        board_all_movements_possible = [
            [0, 0, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        rotated_board = [
            [0, 0, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        expected_directions = ["left", "right", "down", "up"]
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=board_all_movements_possible),
            rotate=MagicMock(return_value=rotated_board),
            set=MagicMock()
        ):
            current_game = game.Game()
            self.assertEqual(current_game.possible_movements(), expected_directions)

    def test_left_movement_not_possible(self):
        board_all_but_left_possible = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        expected_directions = ["right", "down", "up"]
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=board_all_but_left_possible),
            set=MagicMock()
        ):
            current_game = game.Game()
            self.assertEqual(current_game.possible_movements(), expected_directions)

    def test_right_movement_not_possible(self):
        board_all_but_right_possible = [
            [0, 0, 0, 0],
            [0, 0, 0, 2],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        expected_directions = ["left", "down", "up"]
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=board_all_but_right_possible),
            set=MagicMock()
        ):
            current_game = game.Game()
            self.assertEqual(current_game.possible_movements(), expected_directions)

    def test_up_movement_not_possible(self):
        board_all_but_up_possible = [
            [0, 2, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        rotated_board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        expected_directions = ["left", "right", "down"]
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=board_all_but_up_possible),
            rotate=MagicMock(return_value=rotated_board),
            set=MagicMock()
        ):
            current_game = game.Game()
            self.assertEqual(current_game.possible_movements(), expected_directions)
    
    def test_down_movement_not_possible(self):
        board_all_but_down_possible = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 2, 0, 0],
        ]
        rotated_board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        expected_directions = ["left", "right", "up"]
        with patch.multiple(
            game.Board,
            generate=MagicMock(return_value=board_all_but_down_possible),
            rotate=MagicMock(return_value=rotated_board),
            set=MagicMock()
        ):
            current_game = game.Game()
            self.assertEqual(current_game.possible_movements(), expected_directions)


if __name__ == "__main__":
    main()