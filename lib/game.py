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

    def left_possible(self):
        """
        Check if a movement to the left is possible by duplicating the board,
        perfoming a movement to the left, and checking if the board has changed after the movement.
        """

        # Make a duplication of the original board
        duplicate_board = self.board.duplicate()

        # Perform a movement to the left
        new_board = []
        for row in duplicate_board.board:
            new_board.append(self.comparing(row, False))
        duplicate_board.replace(new_board)

        # Compare the moved board with the original board
        return not duplicate_board.board == self.board.board

    def right_possible(self):
        """
        Check if a movement to the right is possible by duplicating the board,
        perfoming a movement to the right, and checking if the board has changed after the movement.
        """

        # Make a duplication of the original board
        duplicate_board = self.board.duplicate()

        # Perform a movement to the right
        new_board = []
        for row in duplicate_board.board:
            new_board.append(list(reversed(self.comparing(list(reversed(row)), False))))
        duplicate_board.replace(new_board)

        # Compare the moved board with the original board
        return not duplicate_board.board == self.board.board

    def down_possible(self):
        """
        Check if a downward movement is possible by duplicating the board,
        perfoming a downward movement, and checking if the board has changed after the movement.
        """

        # Make a duplication of the original board
        duplicate_board = self.board.duplicate()

        # Perform a downward movement
        duplicate_board.rotate()
        new_board = []
        for row in duplicate_board.board:
            new_board.append(list(reversed(self.comparing(list(reversed(row)), False))))
        duplicate_board.replace(new_board)
        duplicate_board.rotate()

        # Compare the duplicated board with the original
        return not duplicate_board.board == self.board.board

    def up_possible(self):
        """
        Check if an upward movement is possible by duplicating the board,
        perfoming an upward movement, and checking if the board has changed after the movement.
        """

        # Make a duplication of the original board
        duplicate_board = self.board.duplicate()

        # Perform a movement to the left
        duplicate_board.rotate()
        new_board = []
        for row in duplicate_board.board:
            new_board.append(self.comparing(row, False))
        duplicate_board.replace(new_board)
        duplicate_board.rotate()

        # Compare the moved board with the original board
        return not duplicate_board.board == self.board.board

    def move_possible(self):
        move_possible = True
        left = self.left_possible()
        right = self.right_possible()
        down = self.down_possible()
        up = self.up_possible()
        if (left == False and right == False and down == False and up == False):
            move_possible = False
        return move_possible

    def game_over(self):
        game_over = False
        if (self.board.full(self.board.board) == True and self.move_possible() == False):
            game_over = True
        return game_over

    def left_move(self):
        self.board.replace([self.comparing(row) for row in self.board.board])
        self.insert_number()

    def right_move(self):
        m = [list(reversed(self.comparing(list(reversed(row))))) for row in self.board.board]
        self.board.replace(m)
        self.insert_number()

    def down_move(self):
        self.board.rotate()
        self.right_move()
        self.board.rotate()

    def up_move(self):
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
