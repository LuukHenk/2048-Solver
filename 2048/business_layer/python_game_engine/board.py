""" This file contains board data and functions for the 2048 game """
from typing import List, Dict
from copy import deepcopy

class Board:
    """ Keeps track of the board data for the 2048 game """
    def __init__(self, size: int = 4) -> None:
        """
        Args:
            size (int). The size of the 2D square board (default = 4)
        """
        self.__set_board_size(size)
        self.__board = self.__generate()

    def __set_board_size(self, size: int) -> None:
        """Set the size of the board. Board size must be at least one
        Args:
            size (int): The size of the board
        """
        if size <= 0:
            raise ValueError("Minimum board size is 1")
        self.__size = size

    def __generate(self) -> List[List[int]]:
        """ Generates a two dimentional board of a given size """
        board_size_range = range(self.__size)
        return [[0 for __ in board_size_range] for _ in board_size_range]

    @property
    def size(self) -> int:
        """ Returns the size of the current board """
        return self.__size

    @property
    def board(self) -> List[List[int]]:
        """ Returns the current 2D 2048 board"""
        return self.__board

    def set_value(self, value: int, x_position: int, y_position: int) -> None:
        """Set a given value on a given position on the board
        Args:
            value (int): The value to be set
            x_position / y_position (int): the position of the board where the
                value is set
        """
        self.__board[y_position][x_position] = value

    def rotate(self) -> None:
        """ Rotates the board by switching the x and y axis"""
        rotated_board = self.__generate()
        for y_position, row in enumerate(self.__board):
            for x_position, value in enumerate(row):
                rotated_board[x_position][y_position] = value
        self.__board = rotated_board

    def empty_tile_positions(self) -> List[Dict[str, int]]:
        """Returns the x and y coordinates of empty tiles on the current board
        """
        empty_tiles = []
        for y_pos, row in enumerate(self.__board):
            for x_pos, value in enumerate(row):
                if value == 0:
                    empty_tiles.append({'y': y_pos, 'x': x_pos})
        return empty_tiles

    def full(self) -> bool:
        """ Checks if the board still contains empty tiles
        returns True if the board is full
        """
        return self.__total_empty_tiles() == 0

    def __total_empty_tiles(self) -> int:
        """ Get the amount of empty tiles left """
        return len([val for row in self.__board for val in row if val == 0])
