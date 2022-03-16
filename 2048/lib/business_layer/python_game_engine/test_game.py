"""Test the game class"""
from unittest import main, TestCase
from dataclasses import fields
from game import Game, Directions

class TestGame(TestCase):
    """Test the game class"""
    EXPECTED_DIRECTIONS = ["left", "right", "down", "up"]
    def test_directions_dataclass(self):
        """Test if the expected directions are in the directions dataclass """
        # Act
        directions = [direction.name for direction in fields(Directions())]
        # Assert
        self.assertEqual(directions, self.EXPECTED_DIRECTIONS)

    def test_directions_property(self):
        """Test if the game its directions property returns the correct directions"""
        # Arrange
        game = Game()
        # Assert
        self.assertEqual(game.directions, self.EXPECTED_DIRECTIONS)

    def test_default_board_size(self):
        """ Test the expected default board size """
        # Arrange
        game = Game()
        default_size = 4
        # Assert
        self.assertEqual(len(game.board), default_size)
        for row in game.board:
            self.assertEqual(len(row), default_size)

    def test_alternative_board_size(self):
        """ Test the altered board size """
        # Arrange
        altered_size = 6
        # Act
        game = Game(board_size=altered_size)
        # Assert
        self.assertEqual(len(game.board), altered_size)
        for row in game.board:
            self.assertEqual(len(row), altered_size)


    def test_perform_move_left(self):
        """Test perform movement function"""
        # Arrange
        expected_output_board = [
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        game = Game()
        game.board = [
            [2, 0, 0, 0],
            [2, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0],
        ]
        # Act
        game.perform_movement("left", insert_number=False)
        # Assert
        self.assertEqual(game.board, expected_output_board)

    def test_perform_move_right(self):
        """Test perform movement function"""
        # Arrange
        expected_output_board = [
            [0, 0, 0, 2],
            [0, 0, 0, 4],
            [0, 0, 0, 2],
            [0, 0, 0, 0],
        ]
        game = Game()
        game.board = [
            [2, 0, 0, 0],
            [2, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0],
        ]
        # Act
        game.perform_movement("right", insert_number=False)
        # Assert
        self.assertEqual(game.board, expected_output_board)


    def test_perform_move_up(self):
        """Test perform movement function"""
        # Arrange
        expected_output_board = [
            [4, 2, 2, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]

        game = Game()
        game.board = [
            [2, 0, 0, 0],
            [2, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0],
        ]
        # Act
        game.perform_movement("up", insert_number=False)
        # Assert
        self.assertEqual(game.board, expected_output_board)

    def test_perform_move_down(self):
        """Test perform movement function"""
        # Arrange
        expected_output_board = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [4, 2, 2, 0],
        ]

        game = Game()
        game.board = [
            [2, 0, 0, 0],
            [2, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0],
        ]
        # Act
        game.perform_movement("down", insert_number=False)
        # Assert
        self.assertEqual(game.board, expected_output_board)

    def test_all_movement_not_possible(self):
        """Test the possible_movements function"""
        # Arrange
        game = Game()
        game.board = [
            [0, 0, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        # Act & Assert
        self.assertEqual(game.possible_movements(), ["left", "right", "down", "up"])

    def test_left_movement_not_possible(self):
        """Test the possible_movements function"""
        # Arrange
        game = Game()
        game.board = [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        # Act & Assert
        self.assertEqual(game.possible_movements(), ["right", "down", "up"])

    def test_right_movement_not_possible(self):
        """Test the possible_movements function"""
        # Arrange
        game = Game()
        game.board = [
            [0, 0, 0, 0],
            [0, 0, 0, 2],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        # Act & Assert
        self.assertEqual(game.possible_movements(), ["left", "down", "up"])

    def test_up_movement_not_possible(self):
        """Test the possible_movements function"""
        # Arrange
        game = Game()
        game.board = [
            [0, 2, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        # Act & Assert
        self.assertEqual(game.possible_movements(), ["left", "right", "down"])

    def test_down_movement_not_possible(self):
        """Test the possible_movements function"""
        # Arrange
        game = Game()
        game.board = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 2, 0, 0],
        ]
        # Act & Assert
        self.assertEqual(game.possible_movements(), ["left", "right", "up"])


if __name__ == "__main__":
    main()
