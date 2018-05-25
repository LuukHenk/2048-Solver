import copy

class Board:
    def __init__(self, size = 4, defaultValue = 0):
        self.size = size
        self.defaultValue = defaultValue
        self.board = self.generate()

    def generate(self):
        s = range(self.size)
        return [[self.defaultValue for __ in s] for _ in s]

    def set(self, value, y, x):
        self.board[y][x] = value

    def render(self):
        for row in self.board:
            print('|'.join(list(map(lambda n: str(' ' if n == 0 else n).center(2, ' '), row))))

    def replace(self, board):
        self.board = board

    def rotate(self):
        out = self.generate()
        for y, row in enumerate(self.board):
            for x, value in enumerate(row):
                out[x][y] = value
        self.board = out

    def emptyTiles(self):
        emptyTiles = []
        for y, row in enumerate(self.board):
            for x, value in enumerate(row):
                if (value == 0):
                    emptyTiles.append({'y': y, 'x': x})
        return emptyTiles

    def full(self, board):
        if len(self.emptyTiles()) == 0:
            return True
        return False

    def duplicate(self):
        return copy.deepcopy(self)
