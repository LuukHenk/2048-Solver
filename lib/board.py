import copy

class Board:
    def __init__(self, size = 4, default_value = 0):
        self.size = size
        self.default_value = default_value
        self.board = self.generate()

    def generate(self):
        s = range(self.size)
        return [[self.default_value for __ in s] for _ in s]

    def render(self):
        for row in self.board:
            #gekke complexe print statement :D
            print('|'.join(list(map(lambda n: str(' ' if n == 0 else n).center(10, ' '), row))))

    def set(self, value, y, x):
        self.board[y][x] = value

    def replace(self, board):
        self.board = board

    def rotate(self):
        out = self.generate()
        for y, row in enumerate(self.board):
            for x, value in enumerate(row):
                out[x][y] = value
        self.board = out

    def empty_tiles(self, board):
        empty_tiles = []
        for y, row in enumerate(board):
            for x, value in enumerate(row):
                if (value == 0):
                    empty_tiles.append({'y': y, 'x': x})
        return empty_tiles

    def full(self, board):
        full = False
        if len(self.empty_tiles(board)) == 0:
            full = True
        return full

    def duplicate(self):
        return copy.deepcopy(self)
