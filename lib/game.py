import random
from lib import board

class Game:
    def __init__(self):
        self.score = 0 # Set score to 0
        self.board = board.Board() # Get an empty board
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
        # Make a duplication of the original board
        duplicate_board = self.board.duplicate()

        if direction in ("down", "up"):
            duplicate_board.rotate()

        # Perform a movement in the desired direction with the duplicated board
        new_board = []

        if direction in ("left", "up"):
            for row in duplicate_board.board:
                new_board.append(self.comparing(row, False))
                duplicate_board.replace(new_board)

        elif direction in ("right", "down"):
            for row in duplicate_board.board:
                new_board.append(list(reversed(self.comparing(list(reversed(row)), False))))
                duplicate_board.replace(new_board)

        duplicate_board.replace(new_board)

        if direction in ("down", "up"):
            duplicate_board.rotate()

        # Compare the moved board with the original board
        return not duplicate_board.board == self.board.board

    def game_over(self):
        """
        Checks if there are still empty tiles on the gameboard
        If not, and there are no movements possible, the game is over.

        Returns game over boolean
        """
        game_over = False

        # If there are no empty tiles on the board
        if self.board.full(self.board.board):

            # And of no movements are possible
            for direction in ("left", "right", "down", "up"):
                if not self.movement_possible(direction):
                    # Game is over :(
                    game_over = True

        return game_over

    def left_move(self):
        """
        Performs a movement to the left and inserts a new random number to the board
        """
        self.board.replace([self.comparing(row) for row in self.board.board])
        self.insert_number()

    def right_move(self):
        """
        Performs a movement to the right and inserts a new random number to the board
        """
        new_board = []
        for row in self.board.board:
            new_board.append(list(reversed(self.comparing(list(reversed(row)), False))))

        self.board.replace(new_board)
        self.insert_number()

    def down_move(self):
        """
        Performs a downwards movement and inserts a new random number to the board
        """
        self.board.rotate()
        self.right_move()
        self.board.rotate()

    def up_move(self):
        """
        Performs a upwards movement and inserts a new random number to the board
        """
        self.board.rotate()
        self.left_move()
        self.board.rotate()

    def comparing(self, sample, write_score = True):
        inp = list(filter(lambda value: value != 0, sample))
        out = []

        while (len(inp) > 0):
            a = inp.pop(0)
            if (len(inp) >= 1):
                m = inp.pop(0)
            else:
                m = None

            if (a == m):
                inp.insert(0, a * 2)
                if (write_score == True):
                    self.score += a * 2
            else:
                out.append(a)
                if (m == None):
                    break
                inp.insert(0, m)

        zeros_to_add = len(sample) - len(out)
        while (zeros_to_add > 0):
            zeros_to_add -= 1
            out.append(0)
        return out
