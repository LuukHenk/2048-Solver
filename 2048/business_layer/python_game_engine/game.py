"""
The game class keeps track of a single game
Also imports the board file for the 2048 board and the functions needed for the board.
"""

from typing import List
import random
from copy import deepcopy
from dataclasses import dataclass, fields

from board import Board

@dataclass
class Directions():
    left: str = "left"
    right: str = "right"
    down: str = "down"
    up: str = "up"

class Game:
    """
    Keeps track of the board and score per game
    Optional args:
        board_size (int): the diagonal size of the 2048 board. Default is 4
    """

    def __init__(self, board_size: int = 4) -> None:
        """A default game starts with a board with two numbers"""
        self.__score = 0
        self.__moves = 0
        self.__directions = Directions()
        self.__board = Board(size=board_size)
        self.__insert_number()
        self.__insert_number()

    @property
    def board(self) -> List[List[int]]:
        """returns the game board as a 2D list"""
        return deepcopy(self.__board.board)

    @property
    def current_score(self) -> int:
        """Returns the current game score as an integer"""
        return deepcopy(self.__score)

    @property
    def total_moves_performed(self) -> int:
        """Returns the total moves performed so far"""
        return deepcopy(self.__moves)

    @property
    def directions(self) -> List[str]:
        """
        Returns the possible directions that can be used to perform a movement
        """
        return [direction.name for direction in fields(self.__directions)]

    def perform_movement(self, direction: Directions) -> None:
        """
        Performs a move if possible
        Args:
            direction (Directions): The direction in which the move is done
        """
        if direction == self.__directions.left:
            self.__left_move()
        elif direction == self.__directions.right:
            self.__right_move()
        elif direction == self.__directions.down:
            self.__down_move()
        elif direction == self.__directions.up:
            self.__up_move()
        self.__moves += 1
        self.__insert_number()

    def possible_movements(self) -> List[str]:
        """
        Determines which moves are possible by performing a move in each
        direction
        Returns (List[str]): The movemements that are possible
        """
        out = []
        for move in fields(self.__directions):
            if self.__mergeable(deepcopy(self.__board.board), move.name):
                out.append(move.name)
        return out

    ###########################################################################

    def __insert_number(self) -> None:
        """ Inserts a 2 or a 4 on a random location on the board """
        value = 4 if random.randint(1, 10) == 1 else 2
        position = random.choice(self.__board.empty_tiles(self.__board.board))
        self.__board.set(value, position['y'], position['x'])

    def __left_move(self):
        """ Performs a movement to the left and inserts a new random number to the board """
        self.__board.replace([self.__merge_values(row) for row in self.__board.board])

    def __right_move(self):
        """ Performs a movement to the right and inserts a new random number to the board """
        new_board = []
        for row in self.__board.board:
            new_board.append(list(reversed(self.__merge_values(list(reversed(row))))))
        self.__board.replace(new_board)

    def __down_move(self):
        """ Performs a downwards movement and inserts a new random number to the board """
        self.__board.rotate()
        self.__right_move()
        self.__board.rotate()

    def __up_move(self):
        """ Performs a upwards movement and inserts a new random number to the board """
        self.__board.rotate()
        self.__left_move()
        self.__board.rotate()

    def __rotate(self, board: List[List[int]]) -> List[List[int]]:
        """
        Rotates the board by switching the x and y positions
        Args:
            board (List[List[int]]): The 2d board to be rotated
        Returns (List[List[int]]): The rotated 2d board
        """
        board_range = range(len(board))
        out = [[0 for _ in board_range] for _ in board_range]
        for y_pos, row in enumerate(board):
            for x_pos, value in enumerate(row):
                out[x_pos][y_pos] = value
        return out

    def __mergeable(self, board: List[List[int]], move: Directions) -> bool:
        """
        Tries to perform a movement and returns if that movement is possible
        Args:
            board (List[List[int]]): The game board
            move (Directions): The move to be tested
        Returns (bool): True if the move is possible
        """
        mergeable = False
        if move == self.__directions.left:
            mergeable = any([self.__left_move_possible(row) for row in board])
        if move == self.__directions.right:
            mergeable = any([self.__left_move_possible(list(reversed(row))) for row in board])
        if move == self.__directions.down:
            board = self.__rotate(board)
            mergeable = any([self.__left_move_possible(list(reversed(row))) for row in board])
        if move == self.__directions.up:
            board = self.__rotate(board)
            mergeable = any([self.__left_move_possible(row) for row in board])
        return mergeable

    def __left_move_possible(self, row: List[int]) -> bool:
        """
        Determines if a left movement is possible
        Args:
            row (List[int]): The row to be checked for possible movement
        Returns (bool): True if the left movement is possible
        """
        possible = False
        previous = row.pop(0)
        for value in row:
            if value != 0:
                if previous in (value, 0):
                    possible = True
            previous = value
        return possible


    def __merge_values(self, input_row: List[int], write_score: bool = True) -> List[int]:
        """
        Performs a 'left movement' on a row in the 2048 board.
        First all the 0's are removed.
        Then we sum adjecent values if they are the same (from left to right).
        If not, we just add them in the same oder at the most left side of the row.
        When we have combined all values, we add back all the 0's left next to the other values
        """

        row_values = self.__filter_list(input_row)
        output_row = []
        while len(row_values) > 0:
            first_value = row_values.pop(0)
            if len(row_values) > 0:
                second_value = row_values.pop(0)
            else:
                second_value = None
            # If the first two items are the same, combine them and add them to the output
            # at the most left position (e.g. 2 and 2 -> [4])
            if first_value == second_value:
                new_value = first_value + second_value
                output_row.append(new_value)
                if write_score:
                    self.__score += new_value
            # If they are not the same, add the first item to the output and
            # If there was a second item, add it back into the input (position 0)
            # If there is no second item, break the while loop and start adding zeros
            else:
                output_row.append(first_value)
                if second_value is None:
                    break
                row_values.insert(0, second_value)
        return self.__fill_list_up_to_size(output_row, len(input_row))

    @staticmethod
    def __filter_list(lst: list, filtered_value: any = 0) -> List[any]:
        """Filter value from a list"""
        return list(filter(lambda value: value != filtered_value, lst))

    @staticmethod
    def __fill_list_up_to_size(lst: list, size: int, value: any = 0) -> List[any]:
        """ Fill a list with value until it reaches the expected size"""
        while len(lst) < size: lst.append(value)
        return lst


# Run locally for testing purposes
if __name__ == "__main__":
    GAME = Game()
    for move in GAME.directions:
        GAME.perform_movement(move)
        GAME.possible_movements(GAME.board)
        GAME.board
        GAME.total_moves_performed
        GAME.current_score
        GAME.directions
