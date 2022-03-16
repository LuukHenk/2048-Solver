""" This file contains a Board class which manages a 2048 board """

from copy import deepcopy

class Board:
    """ Generates and controls a 2048 board """
    def __init__(self, size=6, default_value=0):
        self.__size = size
        self.default_value = default_value
        self.board = self.generate()
    
    @property
    def size(self) -> int:
        """The board size"""
        return deepcopy(self.__size)
        
    def generate(self):
        """ Generates a two dimentional board of a given size """
        size = range(self.__size)
        return [[self.default_value for __ in size] for _ in size]

    def render(self):
        """ Draw the board in te terminal """
        for row in self.board:
            print('|'.join(list(map(lambda n: str(' ' if n == 0 else n).center(10, ' '), row))))

    def set(self, value, y_pos, x_pos):
        """ Set a given value on a given position on the board """
        self.board[y_pos][x_pos] = value

    def replace(self, board):
        """ Replace the board with another board """
        self.board = board

    def rotate(self):
        """ Rotates the board """
        # Generate a new board
        out = self.generate()

        # Fill the new board with the switched x and y from the current board
        for y_pos, row in enumerate(self.board):
            for x_pos, value in enumerate(row):
                out[x_pos][y_pos] = value
        self.board = out

    def empty_tiles(self, board):
        """ Returns the position of empty tiles on a board """
        empty_tiles = []

        #find empty tiles on the board
        for y_pos, row in enumerate(board):
            for x_pos, value in enumerate(row):
                if value == 0:
                    empty_tiles.append({'y': y_pos, 'x': x_pos})
        return empty_tiles

    def full(self, board):
        """ Checks if the board still contains empty tiles """
        return len(self.empty_tiles(board)) == 0

    def duplicate(self):
        """ Make a copy of the board"""
        return deepcopy(self)
