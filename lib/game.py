"""
This file keeps track of the board and score per game using the Game class.
Also imports the board file for the 2048 board and the functions needed for the board.
"""

import random
from .board import Board

class Game:
    """
    Keeps track of the board and score per game.
    Contains a function to insert numbers,
    a function to check if a movement is possible,
    a function to check if you are game over,
    a function for left, right, down and up movement,
    and a function to move and combine the numbers on the board
    """

    def __init__(self):
        self.score = 0 # Set score to 0
        self.moves = 0
        self.board = Board() # Get an empty board
        self.insert_number() # Add a number to the board (2x)
        self.insert_number()

    def insert_number(self):
        """
        Inserts a 2 or a 4 on a random location on the board
        """

        value = 0
        if not self.game_over():
            value = 2

            # Find an empty position on the board
            position = random.choice(self.board.empty_tiles(self.board.board))

            # 1/10 chance to get a 4 instead of a 2 on the board
            if random.randint(1, 10) == 10:
                value = 4

        # Set the value to the random position on the board
        return self.board.set(value, position['y'], position['x'])

    def movement_possible(self, direction):
        """
        Check if a movement is possible by duplicating the board,
        perfoming a movement in the desired direction using the duplicated board,
        and checking if the duplicated board differences from the original board

        Returns true if the board is different, false if it is the same
        """
        if direction in ("left", "down", "right", "up"):
            # Make a duplication of the original board
            duplicate_board = self.board.duplicate()

            # Rotate board if up or down, because then you make a left/right movement
            if direction in ("down", "up"):
                duplicate_board.rotate()

            # Perform a movement in the desired direction with the duplicated board
            # And add the result into new_board
            new_board = []
            if direction in ("left", "up"):
                for row in duplicate_board.board:
                    new_board.append(self.comparing(row, False))
            elif direction in ("right", "up"):
                for row in duplicate_board.board:
                    new_board.append(list(reversed(self.comparing(list(reversed(row)), False))))

            # Compare the moved board with the original board
            return duplicate_board.board != new_board
        return False

    def game_over(self):
        """
        Checks if there are still empty tiles on the gameboard
        If not, and there are no movements possible, the game is over.

        Returns if game_over is true or false
        """

        # If there are no empty tiles on the board
        if self.board.full(self.board.board):

            # And of no movements are possible
            for direction in ("left", "right", "down", "up"):
                if not self.movement_possible(direction):
                    # Game is over :(
                    return True

        return False

    def left_move(self):
        """ Performs a movement to the left and inserts a new random number to the board """

        self.board.replace([self.comparing(row) for row in self.board.board])

    def right_move(self):
        """ Performs a movement to the right and inserts a new random number to the board """

        new_board = []
        for row in self.board.board:
            new_board.append(list(reversed(self.comparing(list(reversed(row)), False))))
        self.board.replace(new_board)

    def down_move(self):
        """ Performs a downwards movement and inserts a new random number to the board """

        self.board.rotate()
        self.right_move()
        self.board.rotate()

    def up_move(self):
        """ Performs a upwards movement and inserts a new random number to the board """

        self.board.rotate()
        self.left_move()
        self.board.rotate()

    def perform_movement(self, direction):
        """
        Checks if a movement is possible.
        If possible, it performs the movement and inserts a new random number on the board
        """

        movement_performed = False

        if self.movement_possible(direction):
            movement_performed = True
            if direction == "left":
                self.left_move()
            elif direction == "right":
                self.right_move()
            elif direction == "up":
                self.up_move()
            elif direction == "down":
                self.down_move()

            self.moves += 1
            self.insert_number()

        return movement_performed

    def comparing(self, sample, write_score=True):
        """
        Performs a 'left movement' on a row in the 2048 board.
        First all the 0's are removed.
        Then we sum adjecent values if they are the same (from left to right).
        If not, we just add them in the same oder at the most left side of the row.
        When we have combined all values, we add back all the 0's left next to the other values
        """

        # Remove all 0's from the input row (e.g. [0, 2, 0, 2] -> [2, 2])
        inp = list(filter(lambda value: value != 0, sample))
        out = []

        #while the row isn't empty yet, pick the first two items.
        while len(inp) > 0:
            first_value = inp.pop(0)
            if len(inp) >= 1:
                second_value = inp.pop(0)
            else:
                second_value = None

            # If the first two items are the same, combine them and add them back to the input
            # at the most left position (e.g. 2 and 2 -> [4])
            if first_value == second_value:
                new_value = first_value + second_value
                inp.insert(0, new_value)
                if write_score:
                    self.score += new_value

            # If they are not the same, add the first item to the output and
            # If there was a second item, add it back into the input (position 0)
            # If there is no second item, break the while loop and start adding zeros
            else:
                out.append(first_value)
                if second_value is None:
                    break
                inp.insert(0, second_value)

        # Add the remaining 0's on the right of the numbers in the output
        zeros_to_add = len(sample) - len(out)
        while zeros_to_add > 0:
            zeros_to_add -= 1
            out.append(0)

        return out
