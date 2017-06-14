import random
import time
from lib import board

class Game:
    def __init__(self):
        self.board = board.Board()
        self.score = 0
        self.insert_number()
        self.insert_number()
        self.dir = ['left', 'right', 'down', 'up']
        self.dir_map = {'left': 0, 'right': 1, 'down': 2, 'up': 3}

    def play_game(self, game_number, mc, move_callback):
        while (self.game_over() == False):
            ml = sum(map(lambda k: [k for _ in range(round(mc[k]))], mc), [])
            m = random.choice(ml)
            if (m == self.dir[0] and self.left_possible()  == True): self.left_move()
            if (m == self.dir[1] and self.right_possible() == True): self.right_move()
            if (m == self.dir[2] and self.down_possible()  == True): self.down_move()
            if (m == self.dir[3] and self.up_possible()    == True): self.up_move()
            move_callback(m)

    def insert_number(self):
        if (self.game_over() == False):
            position = random.choice(self.board.empty_tiles(self.board.board))
            value = 2
            if(random.randint(1, 10) == 10):
                value = 4
            return self.board.set(value, position['y'], position['x'])

    def left_possible(self):
        left = True
        duplicate_board = self.board.duplicate()
        duplicate_board.replace([self.comparing(row, False) for row in duplicate_board.board])
        if duplicate_board.board == self.board.board:
            left = False
        return left

    def right_possible(self):
        right = True
        duplicate_board = self.board.duplicate()
        duplicate_board.replace([list(reversed(self.comparing(list(reversed(row)), False))) for row in duplicate_board.board])
        if duplicate_board.board == self.board.board:
            right = False
        return right

    def down_possible(self):
        down = True
        duplicate_board = self.board.duplicate()
        duplicate_board.rotate()
        duplicate_board.replace([list(reversed(self.comparing(list(reversed(row)), False))) for row in duplicate_board.board])
        duplicate_board.rotate()
        if duplicate_board.board == self.board.board:
            down = False
        return down

    def up_possible(self):
        up = True
        duplicate_board = self.board.duplicate()
        duplicate_board.rotate()
        duplicate_board.replace([self.comparing(row, False) for row in duplicate_board.board])
        duplicate_board.rotate()
        if duplicate_board.board == self.board.board:
            up = False
        return up

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
